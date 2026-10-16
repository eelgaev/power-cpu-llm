//! Explicit GGUF bindings for Qwen3.5/3.6 MoE (`qwen35moe`, e.g.
//! Qwen3.6-35B-A3B): a hybrid of Gated DeltaNet linear-attention layers and
//! gated full-attention layers (one in `full_attention_interval`), each
//! followed by a softmax-routed MoE with a sigmoid-gated shared expert, plus
//! an optional MTP block. The equations follow llama.cpp
//! `src/models/qwen35moe.cpp` and `delta-net-base.cpp` (GGUF conventions:
//! norm weights include the `1 +`, except the linear-attention gated norm;
//! value heads are in tiled order, value head j uses key head j mod
//! key_heads; `A = -exp(A_log)`). Like [`crate::engine::model`], this checks an
//! execution contract, not numerical correctness.

use crate::engine::model::{Error, Metadata, Registry};
use crate::engine::nemotron::{Vocabulary, read_vocabulary};
use crate::gguf::{Inventory, TensorInfo};

pub mod forward;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerKind {
    /// Gated DeltaNet.
    Linear,
    /// Gated full attention.
    Attention,
}

#[derive(Debug, Clone)]
pub struct Specification {
    pub hidden: usize,
    pub vocabulary: usize,
    pub context: usize,
    /// Main layers (the MTP block is not one of them).
    pub layers: Vec<LayerKind>,
    pub rms_epsilon: f32,
    // Full attention.
    pub query_heads: usize,
    pub kv_heads: usize,
    pub head_dim: usize,
    /// Rotated leading dimensions of each head (split-half pairs).
    pub rope_dims: usize,
    pub rope_base: f32,
    // Gated DeltaNet.
    pub key_heads: usize,
    pub value_heads: usize,
    /// Width of each key and value head.
    pub state: usize,
    pub conv_kernel: usize,
    // Mixture of experts.
    pub expert_count: usize,
    pub top_k: usize,
    pub expert_ff: usize,
    pub shared_ff: usize,
    pub mtp_layers: usize,
}

impl Specification {
    /// Q and K channels of the linear-attention input projection.
    pub fn key_width(&self) -> usize {
        self.key_heads * self.state
    }
    pub fn value_width(&self) -> usize {
        self.value_heads * self.state
    }
    /// Channels of the causal convolution: Q, K, then V.
    pub fn conv_channels(&self) -> usize {
        2 * self.key_width() + self.value_width()
    }

    fn read(metadata: &Metadata<'_>, vocabulary: usize) -> Result<Self, Error> {
        let fail = |m: &str| Error(m.into());
        let count = metadata.positive("block_count")?;
        let mtp_layers = metadata.uint_or_zero("nextn_predict_layers")?;
        if mtp_layers > 1 || mtp_layers >= count {
            return Err(fail("unsupported MTP layer count"));
        }
        let main = count - mtp_layers;
        let interval = metadata.positive("full_attention_interval")?;
        let layers = (0..main)
            .map(|i| {
                if (i + 1).is_multiple_of(interval) {
                    LayerKind::Attention
                } else {
                    LayerKind::Linear
                }
            })
            .collect();
        let head_dim = metadata.positive("attention.key_length")?;
        if metadata.positive("attention.value_length")? != head_dim {
            return Err(fail("different K/V head widths are unsupported"));
        }
        let query_heads = metadata.positive("attention.head_count")?;
        let kv_heads = metadata.positive("attention.head_count_kv")?;
        let rope_dims = metadata.positive("rope.dimension_count")?;
        if !query_heads.is_multiple_of(kv_heads)
            || rope_dims > head_dim
            || !rope_dims.is_multiple_of(2)
        {
            return Err(fail("invalid attention/RoPE geometry"));
        }
        let state = metadata.positive("ssm.state_size")?;
        let key_heads = metadata.positive("ssm.group_count")?;
        let value_heads = metadata.positive("ssm.time_step_rank")?;
        if metadata.positive("ssm.inner_size")? != value_heads * state
            || !value_heads.is_multiple_of(key_heads)
        {
            return Err(fail("invalid DeltaNet head geometry"));
        }
        let expert_count = metadata.positive("expert_count")?;
        let top_k = metadata.positive("expert_used_count")?;
        if top_k > expert_count {
            return Err(fail("more experts used than exist"));
        }
        if vocabulary == 0 || vocabulary > u32::MAX as usize {
            return Err(fail("invalid vocabulary size"));
        }
        Ok(Self {
            hidden: metadata.positive("embedding_length")?,
            vocabulary,
            context: metadata.positive("context_length")?,
            layers,
            rms_epsilon: metadata.float("attention.layer_norm_rms_epsilon")?,
            query_heads,
            kv_heads,
            head_dim,
            rope_dims,
            rope_base: metadata.float("rope.freq_base")?,
            key_heads,
            value_heads,
            state,
            conv_kernel: metadata.positive("ssm.conv_kernel")?,
            expert_count,
            top_k,
            expert_ff: metadata.positive("expert_feed_forward_length")?,
            shared_ff: metadata.positive("expert_shared_feed_forward_length")?,
            mtp_layers,
        })
    }
}

/// Gated DeltaNet mixer.
#[derive(Debug)]
pub struct LinearBindings<'a> {
    /// `[hidden, Q | K | V]`.
    pub qkv: &'a TensorInfo,
    /// The output gate `z`, `[hidden, value width]`.
    pub gate: &'a TensorInfo,
    /// `[kernel, channels]`, no bias.
    pub conv_weight: &'a TensorInfo,
    pub dt_bias: &'a TensorInfo,
    /// `-exp(A_log)` per value head.
    pub a: &'a TensorInfo,
    /// F32 `[hidden, value heads]`: sigmoid -> write strength.
    pub beta: &'a TensorInfo,
    /// F32 `[hidden, value heads]`: + dt_bias, softplus, * a -> log decay.
    pub alpha: &'a TensorInfo,
    /// Gated RMS norm weight per value-head channel (no `1 +`).
    pub norm: &'a TensorInfo,
    pub output: &'a TensorInfo,
}

/// Gated full attention: Q and gate interleaved per head, Q/K norms, partial
/// RoPE, `out(attention * sigmoid(gate))`.
#[derive(Debug)]
pub struct AttentionBindings<'a> {
    /// `[hidden, heads * 2 * head_dim]`: per head `[q | gate]`.
    pub query: &'a TensorInfo,
    pub key: &'a TensorInfo,
    pub value: &'a TensorInfo,
    pub output: &'a TensorInfo,
    pub query_norm: &'a TensorInfo,
    pub key_norm: &'a TensorInfo,
}

/// Softmax top-k experts `down(silu(gate x) * up x)` (normalized weights)
/// plus a shared expert of the same form scaled by `sigmoid(w . x)`.
#[derive(Debug)]
pub struct MoeBindings<'a> {
    pub router: &'a TensorInfo,
    pub expert_gate: &'a TensorInfo,
    pub expert_up: &'a TensorInfo,
    pub expert_down: &'a TensorInfo,
    pub shared_gate_input: &'a TensorInfo,
    pub shared_gate: &'a TensorInfo,
    pub shared_up: &'a TensorInfo,
    pub shared_down: &'a TensorInfo,
}

#[derive(Debug)]
pub enum Mixer<'a> {
    Linear(LinearBindings<'a>),
    Attention(AttentionBindings<'a>),
}

#[derive(Debug)]
pub struct LayerBindings<'a> {
    pub norm: &'a TensorInfo,
    pub mixer: Mixer<'a>,
    /// Pre-FFN norm (`post_attention_norm`).
    pub ffn_norm: &'a TensorInfo,
    pub moe: MoeBindings<'a>,
}

/// The MTP block: bound and validated, not executed yet.
#[derive(Debug)]
pub struct MtpBindings<'a> {
    pub embedding_norm: &'a TensorInfo,
    pub hidden_norm: &'a TensorInfo,
    pub projection: &'a TensorInfo,
    pub head_norm: &'a TensorInfo,
    pub layer: LayerBindings<'a>,
}

pub struct ModelDefinition<'a> {
    pub spec: Specification,
    pub vocabulary: Vocabulary<'a>,
    pub embedding: &'a TensorInfo,
    pub output_norm: &'a TensorInfo,
    pub output: &'a TensorInfo,
    pub layers: Vec<LayerBindings<'a>>,
    pub mtp: Option<MtpBindings<'a>>,
}

pub const ARCHITECTURE: &str = "qwen35moe";

impl<'a> ModelDefinition<'a> {
    pub fn bind(inventory: &'a Inventory) -> Result<Self, Error> {
        if inventory.architecture() != Some(ARCHITECTURE) {
            return Err(Error("unsupported architecture".into()));
        }
        let embedding = inventory
            .tensors
            .iter()
            .find(|t| t.name == "token_embd.weight")
            .ok_or_else(|| Error("missing embedding".into()))?;
        let vocabulary_size =
            usize::try_from(embedding.shape[1]).map_err(|_| Error("vocabulary overflow".into()))?;
        let metadata = Metadata {
            map: &inventory.metadata,
            prefix: "qwen35moe.",
        };
        let spec = Specification::read(&metadata, vocabulary_size)?;
        let vocabulary = read_vocabulary(&metadata, vocabulary_size)?;
        let mut registry = Registry::new(inventory)?;
        let h = spec.hidden;
        let embedding = registry.bind("token_embd.weight", [h, spec.vocabulary, 1, 1], false)?;
        let output_norm = registry.vector("output_norm.weight", h)?;
        let output = registry.bind("output.weight", [h, spec.vocabulary, 1, 1], false)?;
        let mut layers = Vec::with_capacity(spec.layers.len());
        for (index, &kind) in spec.layers.iter().enumerate() {
            let prefix = format!("blk.{index}.");
            let mixer = match kind {
                LayerKind::Linear => Mixer::Linear(linear(&mut registry, &spec, &prefix)?),
                LayerKind::Attention => Mixer::Attention(attention(&mut registry, &spec, &prefix)?),
            };
            layers.push(layer(&mut registry, &spec, &prefix, mixer, true)?);
        }
        let mtp = (spec.mtp_layers == 1)
            .then(|| -> Result<_, Error> {
                let prefix = format!("blk.{}.", spec.layers.len());
                let r = &mut registry;
                let mixer = Mixer::Attention(attention(r, &spec, &prefix)?);
                Ok(MtpBindings {
                    embedding_norm: r.vector(&format!("{prefix}nextn.enorm.weight"), h)?,
                    hidden_norm: r.vector(&format!("{prefix}nextn.hnorm.weight"), h)?,
                    projection: r.bind(
                        &format!("{prefix}nextn.eh_proj.weight"),
                        [2 * h, h, 1, 1],
                        false,
                    )?,
                    head_norm: r.vector(&format!("{prefix}nextn.shared_head_norm.weight"), h)?,
                    // The MTP router and shared gate are BF16 in the target file.
                    layer: layer(r, &spec, &prefix, mixer, false)?,
                })
            })
            .transpose()?;
        registry.finish()?;
        Ok(Self {
            spec,
            vocabulary,
            embedding,
            output_norm,
            output,
            layers,
            mtp,
        })
    }
}

fn layer<'a>(
    r: &mut Registry<'a>,
    spec: &Specification,
    prefix: &str,
    mixer: Mixer<'a>,
    f32_router: bool,
) -> Result<LayerBindings<'a>, Error> {
    let (h, e, ff, shared) = (
        spec.hidden,
        spec.expert_count,
        spec.expert_ff,
        spec.shared_ff,
    );
    let mut bind = |s: &str, shape, f32| r.bind(&format!("{prefix}{s}"), shape, f32);
    Ok(LayerBindings {
        norm: bind("attn_norm.weight", [h, 1, 1, 1], true)?,
        mixer,
        ffn_norm: bind("post_attention_norm.weight", [h, 1, 1, 1], true)?,
        moe: MoeBindings {
            router: bind("ffn_gate_inp.weight", [h, e, 1, 1], f32_router)?,
            expert_gate: bind("ffn_gate_exps.weight", [h, ff, e, 1], false)?,
            expert_up: bind("ffn_up_exps.weight", [h, ff, e, 1], false)?,
            expert_down: bind("ffn_down_exps.weight", [ff, h, e, 1], false)?,
            shared_gate_input: bind("ffn_gate_inp_shexp.weight", [h, 1, 1, 1], f32_router)?,
            shared_gate: bind("ffn_gate_shexp.weight", [h, shared, 1, 1], false)?,
            shared_up: bind("ffn_up_shexp.weight", [h, shared, 1, 1], false)?,
            shared_down: bind("ffn_down_shexp.weight", [shared, h, 1, 1], false)?,
        },
    })
}

fn linear<'a>(
    r: &mut Registry<'a>,
    spec: &Specification,
    prefix: &str,
) -> Result<LinearBindings<'a>, Error> {
    let (h, heads, value) = (spec.hidden, spec.value_heads, spec.value_width());
    let mut bind = |s: &str, shape, f32| r.bind(&format!("{prefix}{s}"), shape, f32);
    Ok(LinearBindings {
        qkv: bind("attn_qkv.weight", [h, spec.conv_channels(), 1, 1], false)?,
        gate: bind("attn_gate.weight", [h, value, 1, 1], false)?,
        conv_weight: bind(
            "ssm_conv1d.weight",
            [spec.conv_kernel, spec.conv_channels(), 1, 1],
            true,
        )?,
        dt_bias: bind("ssm_dt.bias", [heads, 1, 1, 1], true)?,
        a: bind("ssm_a", [heads, 1, 1, 1], true)?,
        beta: bind("ssm_beta.weight", [h, heads, 1, 1], true)?,
        alpha: bind("ssm_alpha.weight", [h, heads, 1, 1], true)?,
        norm: bind("ssm_norm.weight", [spec.state, 1, 1, 1], true)?,
        output: bind("ssm_out.weight", [value, h, 1, 1], false)?,
    })
}

fn attention<'a>(
    r: &mut Registry<'a>,
    spec: &Specification,
    prefix: &str,
) -> Result<AttentionBindings<'a>, Error> {
    let (h, d) = (spec.hidden, spec.head_dim);
    let mut bind = |s: &str, shape, f32| r.bind(&format!("{prefix}{s}"), shape, f32);
    Ok(AttentionBindings {
        query: bind("attn_q.weight", [h, 2 * spec.query_heads * d, 1, 1], false)?,
        key: bind("attn_k.weight", [h, spec.kv_heads * d, 1, 1], false)?,
        value: bind("attn_v.weight", [h, spec.kv_heads * d, 1, 1], false)?,
        output: bind("attn_output.weight", [spec.query_heads * d, h, 1, 1], false)?,
        query_norm: bind("attn_q_norm.weight", [d, 1, 1, 1], true)?,
        key_norm: bind("attn_k_norm.weight", [d, 1, 1, 1], true)?,
    })
}
