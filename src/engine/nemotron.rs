//! Explicit GGUF bindings for Nemotron 3.5 (`nemotron_h_moe`): a hybrid of
//! Mamba-2 mixers, causal attention without positional encoding, and MoE
//! feed-forward layers, each a pre-norm residual block, plus an optional
//! multi-token-prediction (MTP) block after the last layer. The equations follow
//! transformers `models/nemotron_h/modeling_nemotron_h.py` (see
//! `docs/models.md`). Like [`crate::engine::model`], this validates an
//! execution contract (every tensor bound with its exact shape), not
//! numerical correctness.

use crate::engine::model::{Error, Metadata, Registry};
use crate::gguf::{ArrayValues, Inventory, TensorInfo};

pub mod forward;
pub mod tokenizer;

/// The mixer of one residual block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerKind {
    Mamba,
    Attention,
    Moe,
}

#[derive(Debug, Clone)]
pub struct Specification {
    pub hidden: usize,
    pub vocabulary: usize,
    pub context: usize,
    /// Main layers (the MTP block is not one of them).
    pub layers: Vec<LayerKind>,
    pub rms_epsilon: f32,
    // Attention (no positional encoding).
    pub query_heads: usize,
    pub kv_heads: usize,
    pub head_dim: usize,
    // Mamba-2.
    /// Heads x head width: the width of the scan, gate and output projection.
    pub ssm_inner: usize,
    pub ssm_state: usize,
    /// B/C groups; head h uses group h / (ssm_heads / ssm_groups).
    pub ssm_groups: usize,
    pub ssm_heads: usize,
    pub ssm_conv_kernel: usize,
    // Mixture of experts.
    pub expert_count: usize,
    pub top_k: usize,
    pub expert_ff: usize,
    pub shared_ff: usize,
    /// Routing weights are normalized over the top k, then multiplied by this.
    pub routed_scale: f32,
    /// MTP blocks after the main layers (0 or 1).
    pub mtp_layers: usize,
}

impl Specification {
    pub fn ssm_head_dim(&self) -> usize {
        self.ssm_inner / self.ssm_heads
    }
    /// Channels of the causal convolution: x, B and C.
    pub fn ssm_conv_channels(&self) -> usize {
        self.ssm_inner + 2 * self.ssm_groups * self.ssm_state
    }
    /// Input projection width: gate z, then x, B, C, then dt (one per head).
    pub fn ssm_projection(&self) -> usize {
        self.ssm_inner + self.ssm_conv_channels() + self.ssm_heads
    }

    fn read(metadata: &Metadata<'_>, vocabulary: usize) -> Result<Self, Error> {
        let count = metadata.positive("block_count")?;
        let mtp_layers = metadata.uint_or_zero("nextn_predict_layers")?;
        if mtp_layers > 1 || mtp_layers >= count {
            return Err(Error("unsupported MTP layer count".into()));
        }
        let main = count - mtp_layers;
        let kv = metadata.per_layer("attention.head_count_kv", count)?;
        let ff = metadata.per_layer("feed_forward_length", count)?;
        // As in llama.cpp: no K/V heads and no FF width marks a Mamba layer,
        // K/V heads without FF width an attention layer, FF width a MoE layer.
        let layers = (0..main)
            .map(|i| match (kv[i], ff[i]) {
                (0, 0) => Ok(LayerKind::Mamba),
                (_, 0) => Ok(LayerKind::Attention),
                (0, _) => Ok(LayerKind::Moe),
                _ => Err(Error(format!("layer {i} has both attention and FF"))),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let hidden = metadata.positive("embedding_length")?;
        let query_heads = metadata.positive("attention.head_count")?;
        let head_dim = metadata.positive("attention.key_length")?;
        if head_dim != metadata.positive("attention.value_length")? {
            return Err(Error("different K/V head widths are unsupported".into()));
        }
        let kv_heads = layers
            .iter()
            .zip(&kv)
            .filter(|(k, _)| **k == LayerKind::Attention)
            .map(|(_, &n)| n)
            .max()
            .ok_or_else(|| Error("target must have attention layers".into()))?;
        if (0..main).any(|i| layers[i] == LayerKind::Attention && kv[i] != kv_heads)
            || !query_heads.is_multiple_of(kv_heads)
        {
            return Err(Error("invalid attention group geometry".into()));
        }
        let expert_ff = metadata.positive("expert_feed_forward_length")?;
        if (0..main).any(|i| layers[i] == LayerKind::Moe && ff[i] != expert_ff) {
            return Err(Error("per-layer expert widths are unsupported".into()));
        }
        let expert_count = metadata.positive("expert_count")?;
        let top_k = metadata.positive("expert_used_count")?;
        // Expert groups are a no-op with one group (transformers NemotronHTopkRouter).
        if top_k > expert_count
            || metadata.uint_or_zero("expert_group_count")? > 1
            || metadata.uint_or_zero("expert_shared_count")? != 1
            || !metadata.boolean(&format!("{}expert_weights_norm", metadata.prefix))?
        {
            return Err(Error("unsupported expert routing variant".into()));
        }
        let ssm_inner = metadata.positive("ssm.inner_size")?;
        let ssm_groups = metadata.positive("ssm.group_count")?;
        let ssm_heads = metadata.positive("ssm.time_step_rank")?;
        if !ssm_inner.is_multiple_of(ssm_heads)
            || !ssm_heads.is_multiple_of(ssm_groups)
            || !ssm_inner.is_multiple_of(ssm_groups)
        {
            return Err(Error("invalid Mamba head/group geometry".into()));
        }
        let context = metadata.positive("context_length")?;
        if vocabulary == 0 || vocabulary > u32::MAX as usize {
            return Err(Error("invalid vocabulary size".into()));
        }
        Ok(Self {
            hidden,
            vocabulary,
            context,
            layers,
            rms_epsilon: metadata.float("attention.layer_norm_rms_epsilon")?,
            query_heads,
            kv_heads,
            head_dim,
            ssm_inner,
            ssm_state: metadata.positive("ssm.state_size")?,
            ssm_groups,
            ssm_heads,
            ssm_conv_kernel: metadata.positive("ssm.conv_kernel")?,
            expert_count,
            top_k,
            expert_ff,
            shared_ff: metadata.positive("expert_shared_feed_forward_length")?,
            routed_scale: metadata.float("expert_weights_scale")?,
            mtp_layers,
        })
    }
}

/// Mamba-2 mixer: `out(gated_norm(scan(conv(x, B, C), dt), z))`.
#[derive(Debug)]
pub struct MambaBindings<'a> {
    /// `[hidden, z | x | B | C | dt]`.
    pub input: &'a TensorInfo,
    /// `[kernel, channels]`: tap k of channel c at `c * kernel + k`.
    pub conv_weight: &'a TensorInfo,
    pub conv_bias: &'a TensorInfo,
    pub dt_bias: &'a TensorInfo,
    /// `A = -exp(A_log)` per head, stored negated and exponentiated.
    pub a: &'a TensorInfo,
    /// Skip connection `D` per head.
    pub d: &'a TensorInfo,
    /// Gated RMS norm weight, normalized per group of `inner / groups`.
    pub norm: &'a TensorInfo,
    pub output: &'a TensorInfo,
}

/// Attention without positional encoding.
#[derive(Debug)]
pub struct AttentionBindings<'a> {
    pub query: &'a TensorInfo,
    pub key: &'a TensorInfo,
    pub value: &'a TensorInfo,
    pub output: &'a TensorInfo,
}

/// Sigmoid-routed experts `down(relu(up(x))^2)` plus a shared expert of the
/// same form.
#[derive(Debug)]
pub struct MoeBindings<'a> {
    /// `[hidden, experts]`.
    pub router: &'a TensorInfo,
    /// Added to the sigmoid scores for choosing experts, not for weighting.
    pub correction_bias: &'a TensorInfo,
    /// `[hidden, expert_ff, experts]`.
    pub expert_up: &'a TensorInfo,
    /// `[expert_ff, hidden, experts]`.
    pub expert_down: &'a TensorInfo,
    pub shared_up: &'a TensorInfo,
    pub shared_down: &'a TensorInfo,
}

#[derive(Debug)]
pub enum Mixer<'a> {
    Mamba(MambaBindings<'a>),
    Attention(AttentionBindings<'a>),
    Moe(MoeBindings<'a>),
}

#[derive(Debug)]
pub struct LayerBindings<'a> {
    pub norm: &'a TensorInfo,
    pub mixer: Mixer<'a>,
}

/// The multi-token-prediction block (a draft head): bound and validated, not
/// executed yet.
#[derive(Debug)]
pub struct MtpBindings<'a> {
    pub embedding_norm: &'a TensorInfo,
    pub hidden_norm: &'a TensorInfo,
    /// `[2 * hidden, hidden]`.
    pub projection: &'a TensorInfo,
    pub attention_norm: &'a TensorInfo,
    pub attention: AttentionBindings<'a>,
    pub moe_norm: &'a TensorInfo,
    pub moe: MoeBindings<'a>,
    pub head_norm: &'a TensorInfo,
}

pub struct Vocabulary<'a> {
    pub tokens: &'a [String],
    pub token_types: &'a [i64],
    pub merges: &'a [String],
    /// `tokenizer.ggml.pre`, e.g. `pixtral`.
    pub pre_tokenizer: &'a str,
    pub chat_template: &'a str,
    pub bos: u32,
    pub eos: u32,
    pub padding: u32,
    /// Absent in some GGUFs (Qwen 3.5/3.6).
    pub unknown: Option<u32>,
    pub add_bos: bool,
}

pub struct ModelDefinition<'a> {
    pub spec: Specification,
    pub vocabulary: Vocabulary<'a>,
    pub embedding: &'a TensorInfo,
    pub output_norm: &'a TensorInfo,
    /// Untied output projection `[hidden, vocabulary]`.
    pub output: &'a TensorInfo,
    pub layers: Vec<LayerBindings<'a>>,
    pub mtp: Option<MtpBindings<'a>>,
}

pub const ARCHITECTURE: &str = "nemotron_h_moe";

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
            prefix: "nemotron_h_moe.",
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
            let norm = registry.vector(&format!("{prefix}attn_norm.weight"), h)?;
            let mixer = match kind {
                LayerKind::Mamba => Mixer::Mamba(mamba(&mut registry, &spec, &prefix)?),
                LayerKind::Attention => Mixer::Attention(attention(&mut registry, &spec, &prefix)?),
                LayerKind::Moe => Mixer::Moe(moe(&mut registry, &spec, &prefix, true)?),
            };
            layers.push(LayerBindings { norm, mixer });
        }
        let mtp = (spec.mtp_layers == 1)
            .then(|| -> Result<_, Error> {
                let prefix = format!("blk.{}.", spec.layers.len());
                let r = &mut registry;
                Ok(MtpBindings {
                    embedding_norm: r.vector(&format!("{prefix}nextn.enorm.weight"), h)?,
                    hidden_norm: r.vector(&format!("{prefix}nextn.hnorm.weight"), h)?,
                    projection: r.bind(
                        &format!("{prefix}nextn.eh_proj.weight"),
                        [2 * h, h, 1, 1],
                        false,
                    )?,
                    attention_norm: r.vector(&format!("{prefix}attn_norm.weight"), h)?,
                    attention: attention(r, &spec, &prefix)?,
                    moe_norm: r.vector(&format!("{prefix}post_attention_norm.weight"), h)?,
                    // The MTP router is stored as BF16 in the target file.
                    moe: moe(r, &spec, &prefix, false)?,
                    head_norm: r.vector(&format!("{prefix}nextn.shared_head_norm.weight"), h)?,
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

fn mamba<'a>(
    r: &mut Registry<'a>,
    spec: &Specification,
    prefix: &str,
) -> Result<MambaBindings<'a>, Error> {
    let (h, heads, channels) = (spec.hidden, spec.ssm_heads, spec.ssm_conv_channels());
    let mut bind = |suffix: &str, shape, f32| r.bind(&format!("{prefix}{suffix}"), shape, f32);
    Ok(MambaBindings {
        input: bind("ssm_in.weight", [h, spec.ssm_projection(), 1, 1], false)?,
        conv_weight: bind(
            "ssm_conv1d.weight",
            [spec.ssm_conv_kernel, channels, 1, 1],
            true,
        )?,
        conv_bias: bind("ssm_conv1d.bias", [channels, 1, 1, 1], true)?,
        dt_bias: bind("ssm_dt.bias", [heads, 1, 1, 1], true)?,
        a: bind("ssm_a", [1, heads, 1, 1], true)?,
        d: bind("ssm_d", [1, heads, 1, 1], true)?,
        norm: bind(
            "ssm_norm.weight",
            [spec.ssm_inner / spec.ssm_groups, spec.ssm_groups, 1, 1],
            true,
        )?,
        output: bind("ssm_out.weight", [spec.ssm_inner, h, 1, 1], false)?,
    })
}

fn attention<'a>(
    r: &mut Registry<'a>,
    spec: &Specification,
    prefix: &str,
) -> Result<AttentionBindings<'a>, Error> {
    let h = spec.hidden;
    let (q, kv) = (
        spec.query_heads * spec.head_dim,
        spec.kv_heads * spec.head_dim,
    );
    let mut bind = |suffix: &str, shape| r.bind(&format!("{prefix}{suffix}"), shape, false);
    Ok(AttentionBindings {
        query: bind("attn_q.weight", [h, q, 1, 1])?,
        key: bind("attn_k.weight", [h, kv, 1, 1])?,
        value: bind("attn_v.weight", [h, kv, 1, 1])?,
        output: bind("attn_output.weight", [q, h, 1, 1])?,
    })
}

fn moe<'a>(
    r: &mut Registry<'a>,
    spec: &Specification,
    prefix: &str,
    f32_router: bool,
) -> Result<MoeBindings<'a>, Error> {
    let (h, e, ff, shared) = (
        spec.hidden,
        spec.expert_count,
        spec.expert_ff,
        spec.shared_ff,
    );
    let mut bind = |suffix: &str, shape, f32| r.bind(&format!("{prefix}{suffix}"), shape, f32);
    Ok(MoeBindings {
        router: bind("ffn_gate_inp.weight", [h, e, 1, 1], f32_router)?,
        correction_bias: bind("exp_probs_b.bias", [e, 1, 1, 1], true)?,
        expert_up: bind("ffn_up_exps.weight", [h, ff, e, 1], false)?,
        expert_down: bind("ffn_down_exps.weight", [ff, h, e, 1], false)?,
        shared_up: bind("ffn_up_shexp.weight", [h, shared, 1, 1], false)?,
        shared_down: bind("ffn_down_shexp.weight", [shared, h, 1, 1], false)?,
    })
}

/// The `gpt2` (byte-level BPE) vocabulary of a GGUF.
pub(crate) fn read_vocabulary<'a>(
    metadata: &Metadata<'a>,
    count: usize,
) -> Result<Vocabulary<'a>, Error> {
    if metadata.string("tokenizer.ggml.model")? != "gpt2" {
        return Err(Error("unsupported tokenizer model".into()));
    }
    let tokens = metadata.strings("tokenizer.ggml.tokens")?;
    let token_types = match metadata.array("tokenizer.ggml.token_type")? {
        ArrayValues::Signed(v) => v.as_slice(),
        _ => return Err(Error("expected signed token types".into())),
    };
    if tokens.len() != count || token_types.len() != count {
        return Err(Error("tokenizer array length mismatch".into()));
    }
    if token_types.iter().any(|t| !(0..=6).contains(t)) {
        return Err(Error("invalid token type".into()));
    }
    let id = |key| -> Result<u32, Error> {
        let n = metadata.uint(key)?;
        if n >= count {
            return Err(Error(format!("special token out of range: {key}")));
        }
        Ok(n as u32)
    };
    Ok(Vocabulary {
        tokens,
        token_types,
        merges: metadata.strings("tokenizer.ggml.merges")?,
        pre_tokenizer: metadata.string("tokenizer.ggml.pre")?,
        chat_template: metadata.string("tokenizer.chat_template")?,
        bos: id("tokenizer.ggml.bos_token_id")?,
        eos: id("tokenizer.ggml.eos_token_id")?,
        padding: id("tokenizer.ggml.padding_token_id")?,
        unknown: metadata
            .map
            .contains_key("tokenizer.ggml.unknown_token_id")
            .then(|| id("tokenizer.ggml.unknown_token_id"))
            .transpose()?,
        add_bos: metadata.boolean("tokenizer.ggml.add_bos_token")?,
    })
}

#[cfg(test)]
pub(crate) mod tests;
