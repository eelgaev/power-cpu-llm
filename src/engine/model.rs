//! Explicit GGUF bindings for DiffusionGemma (`diffusion-gemma`) and the
//! autoregressive Gemma 4 family (`gemma4`): the 26B-A4B MoE (same layer
//! geometry without the diffusion decoder parts) and the dense E2B/E4B models
//! with per-layer embeddings and shared-K/V layers. This validates an execution contract, not
//! numerical forward-pass correctness. No graph executor or tensor guessing.

use crate::gguf::{ArrayValues, Inventory, MetadataValue, TensorInfo};
use crate::kernels::encoded::Encoding;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

/// Which model family a GGUF binds as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelKind {
    /// DiffusionGemma: causal encoder + bidirectional canvas decoder.
    Diffusion,
    /// Gemma 4: causal decoder-only language model (token-by-token generation).
    Autoregressive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(pub String);
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for Error {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttentionKind {
    Sliding,
    Global,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttentionGeometry {
    pub kind: AttentionKind,
    pub query_heads: usize,
    pub kv_heads: usize,
    pub head_dim: usize,
    pub rotary_dim: usize,
}
#[derive(Debug, Clone)]
pub struct Specification {
    pub kind: ModelKind,
    pub hidden: usize,
    pub vocabulary: usize,
    pub canvas: usize,
    pub context: usize,
    /// Dense/shared MLP width of layer 0 (the self-conditioning MLP width).
    pub shared_ff: usize,
    /// Dense/shared MLP width per layer (E2B doubles it in shared-K/V layers).
    pub layer_ff: Vec<usize>,
    /// 0 for dense models (no routed experts).
    pub expert_ff: usize,
    pub expert_count: usize,
    pub top_k: usize,
    /// Per-layer embedding width (E2B: 256); 0 if the model has none.
    pub per_layer_input: usize,
    /// First layer that reuses an earlier layer's K/V (`block_count` if none).
    pub first_shared_kv: usize,
    pub sliding_window: usize,
    pub rms_epsilon: f32,
    pub logit_softcap: f32,
    pub rope_base_global: f32,
    pub rope_base_sliding: f32,
    pub attention: Vec<AttentionGeometry>,
}

pub struct Vocabulary<'a> {
    pub tokens: &'a [String],
    pub scores: &'a [f64],
    pub token_types: &'a [i64],
    pub merges: &'a [String],
    pub chat_template: &'a str,
    pub bos: u32,
    pub eos: u32,
    pub padding: u32,
    pub unknown: u32,
    /// Retained if present; the diffusion sampler uses uniform noise, not masks.
    pub mask: Option<u32>,
    pub add_bos: bool,
    pub add_space_prefix: bool,
}

#[derive(Debug)]
pub struct MlpBindings<'a> {
    pub gate: &'a TensorInfo,
    pub up: &'a TensorInfo,
    pub down: &'a TensorInfo,
}
#[derive(Debug)]
pub struct RouterBindings<'a> {
    pub projection: &'a TensorInfo,
    pub hidden_scale: &'a TensorInfo,
    pub expert_scale: &'a TensorInfo,
}
/// Routed-expert branch of a MoE layer.
#[derive(Debug)]
pub struct MoeBindings<'a> {
    pub router: RouterBindings<'a>,
    pub expert_gate_up: &'a TensorInfo,
    pub expert_down: &'a TensorInfo,
    /// Post-norm of the shared MLP branch (`post_ffw_norm_1`).
    pub shared_post_norm: &'a TensorInfo,
    pub expert_pre_norm: &'a TensorInfo,
    pub expert_post_norm: &'a TensorInfo,
}
/// Per-layer-embedding block of one layer (Gemma4TextDecoderLayer, after the
/// feed-forward residual): `x + post_norm(projection(gelu(gate(x)) * ple))`.
#[derive(Debug)]
pub struct PerLayerBindings<'a> {
    pub gate: &'a TensorInfo,
    pub projection: &'a TensorInfo,
    pub post_norm: &'a TensorInfo,
}
/// Model-level per-layer-embedding tensors.
#[derive(Debug)]
pub struct PerLayerInputBindings<'a> {
    /// `[layers * width, vocabulary]`, scaled by sqrt(width) on lookup.
    pub token_embedding: &'a TensorInfo,
    /// `[hidden, layers * width]`, scaled by 1/sqrt(hidden).
    pub model_projection: &'a TensorInfo,
    pub projection_norm: &'a TensorInfo,
}
#[derive(Debug)]
pub struct LayerBindings<'a> {
    pub query: &'a TensorInfo,
    /// None for shared-K/V layers.
    pub key: Option<&'a TensorInfo>,
    /// None when V reuses raw K (26B global layers) or for shared-K/V layers.
    pub value: Option<&'a TensorInfo>,
    pub attention_output: &'a TensorInfo,
    pub query_norm: &'a TensorInfo,
    pub key_norm: Option<&'a TensorInfo>,
    pub attention_norm: &'a TensorInfo,
    pub post_attention_norm: &'a TensorInfo,
    pub shared_mlp: MlpBindings<'a>,
    pub shared_pre_norm: &'a TensorInfo,
    /// `post_ffw_norm`: on (MLP + experts) for MoE, on the MLP for dense.
    pub combined_ff_norm: &'a TensorInfo,
    pub moe: Option<MoeBindings<'a>>,
    pub per_layer: Option<PerLayerBindings<'a>>,
    /// Shared-K/V layers: the earlier layer (same attention kind) whose
    /// staged K/V of the same pass this layer attends over.
    pub kv_source: Option<usize>,
    pub encoder_scale: &'a TensorInfo,
    pub decoder_scale: &'a TensorInfo,
}

pub struct ModelDefinition<'a> {
    pub spec: Specification,
    pub vocabulary: Vocabulary<'a>,
    /// Also the output projection: this target has tied embedding/head weights.
    pub embedding: &'a TensorInfo,
    pub output_norm: &'a TensorInfo,
    pub rope_frequency_factors: &'a TensorInfo,
    /// Diffusion models only.
    pub self_conditioning_pre_norm: Option<&'a TensorInfo>,
    /// Diffusion models only.
    pub self_conditioning_mlp: Option<MlpBindings<'a>>,
    /// Models with per-layer embeddings only (E2B).
    pub per_layer_input: Option<PerLayerInputBindings<'a>>,
    pub layers: Vec<LayerBindings<'a>>,
}

/// Typed metadata reads (shared with [`crate::engine::nemotron`]).
pub(crate) struct Metadata<'a> {
    pub(crate) map: &'a BTreeMap<String, MetadataValue>,
    /// Architecture key prefix, e.g. `diffusion-gemma.` or `gemma4.`.
    pub(crate) prefix: &'static str,
}
impl<'a> Metadata<'a> {
    pub(crate) fn value(&self, key: &str) -> Result<&'a MetadataValue, Error> {
        self.map
            .get(key)
            .ok_or_else(|| Error(format!("missing metadata: {key}")))
    }
    pub(crate) fn uint(&self, key: &str) -> Result<usize, Error> {
        match self.value(key)? {
            MetadataValue::Unsigned(n) => {
                usize::try_from(*n).map_err(|_| Error(format!("integer overflow: {key}")))
            }
            _ => Err(Error(format!("expected unsigned metadata: {key}"))),
        }
    }
    /// Unsigned value, or 0 if the key is absent.
    pub(crate) fn uint_or_zero(&self, suffix: &str) -> Result<usize, Error> {
        let key = format!("{}{suffix}", self.prefix);
        if self.map.contains_key(&key) {
            self.uint(&key)
        } else {
            Ok(0)
        }
    }
    /// A per-layer count stored as one unsigned value or a signed array.
    pub(crate) fn per_layer(&self, suffix: &str, count: usize) -> Result<Vec<usize>, Error> {
        let key = format!("{}{suffix}", self.prefix);
        match self.value(&key)? {
            MetadataValue::Unsigned(_) => Ok(vec![self.uint(&key)?; count]),
            MetadataValue::Array {
                values: ArrayValues::Signed(v),
                ..
            } if v.len() == count => v
                .iter()
                .map(|&n| usize::try_from(n).map_err(|_| Error(format!("negative value: {key}"))))
                .collect(),
            _ => Err(Error(format!(
                "expected one value or {count} per layer: {key}"
            ))),
        }
    }
    pub(crate) fn positive(&self, suffix: &str) -> Result<usize, Error> {
        let key = format!("{}{suffix}", self.prefix);
        let n = self.uint(&key)?;
        if n == 0 {
            return Err(Error(format!("metadata must be positive: {key}")));
        }
        Ok(n)
    }
    pub(crate) fn float(&self, suffix: &str) -> Result<f32, Error> {
        let key = format!("{}{suffix}", self.prefix);
        match self.value(&key)? {
            MetadataValue::Float(n)
                if n.is_finite() && *n > 0.0 && (*n as f32).is_finite() && (*n as f32) > 0.0 =>
            {
                Ok(*n as f32)
            }
            _ => Err(Error(format!("expected positive finite float: {key}"))),
        }
    }
    pub(crate) fn boolean(&self, key: &str) -> Result<bool, Error> {
        match self.value(key)? {
            MetadataValue::Bool(b) => Ok(*b),
            _ => Err(Error(format!("expected boolean: {key}"))),
        }
    }
    pub(crate) fn string(&self, key: &str) -> Result<&'a str, Error> {
        match self.value(key)? {
            MetadataValue::String(s) => Ok(s),
            _ => Err(Error(format!("expected string: {key}"))),
        }
    }
    pub(crate) fn array(&self, key: &str) -> Result<&'a ArrayValues, Error> {
        match self.value(key)? {
            MetadataValue::Array { values, .. } => Ok(values),
            _ => Err(Error(format!("expected array: {key}"))),
        }
    }
    pub(crate) fn strings(&self, key: &str) -> Result<&'a [String], Error> {
        match self.array(key)? {
            ArrayValues::String(v) => Ok(v),
            _ => Err(Error(format!("expected string array: {key}"))),
        }
    }
}

impl Specification {
    fn read(metadata: &Metadata<'_>, kind: ModelKind, vocabulary: usize) -> Result<Self, Error> {
        let p = metadata.prefix;
        let count = metadata.positive("block_count")?;
        // DiffusionGemma marks its (decoder) attention non-causal; Gemma 4 has no key.
        let causal_flag = match kind {
            ModelKind::Diffusion => metadata.boolean(&format!("{p}attention.causal"))?,
            ModelKind::Autoregressive => false,
        };
        let shared_kv_layers = metadata.uint_or_zero("attention.shared_kv_layers")?;
        let per_layer_input = metadata.uint_or_zero("embedding_length_per_layer_input")?;
        if causal_flag
            || (kind == ModelKind::Diffusion && (shared_kv_layers != 0 || per_layer_input != 0))
            || shared_kv_layers >= count
        {
            return Err(Error(
                "unsupported causal/shared-KV/per-layer-input variant".into(),
            ));
        }
        let hidden = metadata.positive("embedding_length")?;
        let heads = metadata.positive("attention.head_count")?;
        let global_dim = metadata.positive("attention.key_length")?;
        let local_dim = metadata.positive("attention.key_length_swa")?;
        if global_dim != metadata.positive("attention.value_length")?
            || local_dim != metadata.positive("attention.value_length_swa")?
        {
            return Err(Error("different K/V head widths are unsupported".into()));
        }
        let global_rotary = metadata.positive("rope.dimension_count")?;
        let local_rotary = metadata.positive("rope.dimension_count_swa")?;
        let pattern = match metadata.array(&format!("{p}attention.sliding_window_pattern"))? {
            ArrayValues::Bool(v) => v,
            _ => return Err(Error("sliding pattern must be a bool array".into())),
        };
        let kv = metadata.per_layer("attention.head_count_kv", count)?;
        if pattern.len() != count {
            return Err(Error("per-layer array length mismatch".into()));
        }
        let mut attention = Vec::with_capacity(count);
        for (&local, kv) in pattern.iter().zip(kv) {
            let kv_heads = kv;
            let head_dim = if local { local_dim } else { global_dim };
            let rotary_dim = if local { local_rotary } else { global_rotary };
            if kv_heads == 0
                || heads % kv_heads != 0
                || head_dim % 2 != 0
                || rotary_dim % 2 != 0
                || rotary_dim > head_dim
            {
                return Err(Error("invalid attention group/rotary geometry".into()));
            }
            heads
                .checked_mul(head_dim)
                .ok_or_else(|| Error("attention width overflow".into()))?;
            attention.push(AttentionGeometry {
                kind: if local {
                    AttentionKind::Sliding
                } else {
                    AttentionKind::Global
                },
                query_heads: heads,
                kv_heads,
                head_dim,
                rotary_dim,
            });
        }
        if !pattern.iter().any(|local| !local) {
            return Err(Error("target must have global attention layers".into()));
        }
        // Dense models (E2B) have no expert keys.
        let expert_count = metadata.uint_or_zero("expert_count")?;
        let top_k = metadata.uint_or_zero("expert_used_count")?;
        let expert_ff = metadata.uint_or_zero("expert_feed_forward_length")?;
        if top_k > expert_count
            || (expert_count > 0 && (top_k == 0 || expert_ff == 0))
            || (expert_count == 0 && kind == ModelKind::Diffusion)
        {
            return Err(Error("invalid expert count/top-k/width".into()));
        }
        let layer_ff = metadata.per_layer("feed_forward_length", count)?;
        if layer_ff.contains(&0) {
            return Err(Error("feed-forward width must be positive".into()));
        }
        // The autoregressive model has no canvas; 256 keeps batch limits identical.
        let canvas = match kind {
            ModelKind::Diffusion => metadata.uint("diffusion.canvas_length")?,
            ModelKind::Autoregressive => 256,
        };
        let context = metadata.positive("context_length")?;
        if canvas == 0 || canvas > context || vocabulary == 0 || vocabulary > u32::MAX as usize {
            return Err(Error("invalid canvas/context/vocabulary size".into()));
        }
        Ok(Self {
            kind,
            hidden,
            vocabulary,
            canvas,
            context,
            shared_ff: layer_ff[0],
            layer_ff,
            expert_ff,
            expert_count,
            top_k,
            per_layer_input,
            first_shared_kv: count - shared_kv_layers,
            sliding_window: metadata.positive("attention.sliding_window")?,
            rms_epsilon: metadata.float("attention.layer_norm_rms_epsilon")?,
            logit_softcap: metadata.float("final_logit_softcapping")?,
            rope_base_global: metadata.float("rope.freq_base")?,
            rope_base_sliding: metadata.float("rope.freq_base_swa")?,
            attention,
        })
    }
}

/// Tensor bindings by exact name and shape; every tensor must be bound.
pub(crate) struct Registry<'a> {
    tensors: BTreeMap<&'a str, &'a TensorInfo>,
    used: BTreeSet<&'a str>,
}
impl<'a> Registry<'a> {
    pub(crate) fn new(inventory: &'a Inventory) -> Result<Self, Error> {
        let mut tensors = BTreeMap::new();
        for tensor in &inventory.tensors {
            if tensors.insert(tensor.name.as_str(), tensor).is_some() {
                return Err(Error("duplicate tensor name".into()));
            }
        }
        Ok(Self {
            tensors,
            used: BTreeSet::new(),
        })
    }
    pub(crate) fn bind(
        &mut self,
        name: &str,
        shape: [usize; 4],
        require_f32: bool,
    ) -> Result<&'a TensorInfo, Error> {
        let tensor = *self
            .tensors
            .get(name)
            .ok_or_else(|| Error(format!("missing tensor: {name}")))?;
        if tensor.shape != shape.map(|n| n as u64) {
            return Err(Error(format!(
                "wrong shape for {name}: {:?}, expected {shape:?}",
                tensor.shape
            )));
        }
        if require_f32 && tensor.type_id != 0 {
            return Err(Error(format!("{name} must be F32")));
        }
        let encoding =
            Encoding::try_from(tensor.type_id).map_err(|e| Error(format!("{name}: {e}")))?;
        let size = shape[1..].iter().try_fold(
            encoding
                .row_bytes(shape[0])
                .map_err(|e| Error(format!("{name}: {e}")))?,
            |n, &d| {
                n.checked_mul(d)
                    .ok_or_else(|| Error("tensor size overflow".into()))
            },
        )?;
        if size as u64 != tensor.size_bytes {
            return Err(Error(format!("wrong stored byte size for {name}")));
        }
        self.used.insert(tensor.name.as_str());
        Ok(tensor)
    }
    /// Fails if any tensor of the file was left unbound.
    pub(crate) fn finish(&self) -> Result<(), Error> {
        if self.used.len() != self.tensors.len() {
            let extra: Vec<_> = self
                .tensors
                .keys()
                .filter(|name| !self.used.contains(**name))
                .copied()
                .collect();
            return Err(Error(format!("unsupported/unbound tensors: {extra:?}")));
        }
        Ok(())
    }
    pub(crate) fn has(&self, name: &str) -> bool {
        self.tensors.contains_key(name)
    }
    pub(crate) fn vector(&mut self, name: &str, width: usize) -> Result<&'a TensorInfo, Error> {
        self.bind(name, [width, 1, 1, 1], true)
    }
    pub(crate) fn mlp(
        &mut self,
        prefix: &str,
        hidden: usize,
        ff: usize,
    ) -> Result<MlpBindings<'a>, Error> {
        Ok(MlpBindings {
            gate: self.bind(&format!("{prefix}gate.weight"), [hidden, ff, 1, 1], false)?,
            up: self.bind(&format!("{prefix}up.weight"), [hidden, ff, 1, 1], false)?,
            down: self.bind(&format!("{prefix}down.weight"), [ff, hidden, 1, 1], false)?,
        })
    }
}

impl<'a> ModelDefinition<'a> {
    pub fn bind(inventory: &'a Inventory) -> Result<Self, Error> {
        let (kind, prefix) = match inventory.architecture() {
            Some("diffusion-gemma") => (ModelKind::Diffusion, "diffusion-gemma."),
            Some("gemma4") => (ModelKind::Autoregressive, "gemma4."),
            _ => return Err(Error("unsupported architecture".into())),
        };
        let embedding = inventory
            .tensors
            .iter()
            .find(|t| t.name == "token_embd.weight")
            .ok_or_else(|| Error("missing embedding".into()))?;
        let vocabulary_size =
            usize::try_from(embedding.shape[1]).map_err(|_| Error("vocabulary overflow".into()))?;
        let metadata = Metadata {
            map: &inventory.metadata,
            prefix,
        };
        let spec = Specification::read(&metadata, kind, vocabulary_size)?;
        let vocabulary = read_vocabulary(&metadata, vocabulary_size)?;
        let mut registry = Registry::new(inventory)?;
        let h = spec.hidden;
        let embedding = registry.bind("token_embd.weight", [h, spec.vocabulary, 1, 1], false)?;
        let output_norm = registry.vector("output_norm.weight", h)?;
        let global_dim = spec
            .attention
            .iter()
            .find(|g| g.kind == AttentionKind::Global)
            .unwrap()
            .head_dim;
        let rope_frequency_factors = registry.vector("rope_freqs.weight", global_dim / 2)?;
        let (self_conditioning_pre_norm, self_conditioning_mlp) = match kind {
            ModelKind::Diffusion => (
                Some(registry.vector("self_cond_pre_norm.weight", h)?),
                Some(registry.mlp("self_cond_", h, spec.shared_ff)?),
            ),
            ModelKind::Autoregressive => (None, None),
        };
        let layer_count = spec.attention.len();
        let ple = spec.per_layer_input;
        let per_layer_input = if ple > 0 {
            let packed = ple
                .checked_mul(layer_count)
                .ok_or_else(|| Error("per-layer width overflow".into()))?;
            Some(PerLayerInputBindings {
                token_embedding: registry.bind(
                    "per_layer_token_embd.weight",
                    [packed, spec.vocabulary, 1, 1],
                    false,
                )?,
                model_projection: registry.bind(
                    "per_layer_model_proj.weight",
                    [h, packed, 1, 1],
                    false,
                )?,
                projection_norm: registry.vector("per_layer_proj_norm.weight", ple)?,
            })
        } else {
            None
        };
        let mut layers = Vec::new();
        for (index, geometry) in spec.attention.iter().enumerate() {
            let prefix = format!("blk.{index}.");
            let q = geometry.query_heads * geometry.head_dim;
            let kv = geometry.kv_heads * geometry.head_dim;
            let d = geometry.head_dim;
            // Shared-K/V layers (HF `is_kv_shared_layer`) reuse the K/V of
            // the LAST earlier layer of the same attention kind.
            let shared_kv = index >= spec.first_shared_kv;
            let kv_source = if shared_kv {
                Some(
                    (0..spec.first_shared_kv)
                        .rev()
                        .find(|&j| spec.attention[j].kind == geometry.kind)
                        .ok_or_else(|| Error("shared-K/V layer without a source".into()))?,
                )
            } else {
                None
            };
            let has_value = !shared_kv && registry.has(&format!("{prefix}attn_v.weight"));
            if geometry.kind == AttentionKind::Sliding && !shared_kv && !has_value {
                return Err(Error(format!("{prefix}: sliding attention needs V")));
            }
            let mut bind =
                |suffix: &str, shape, f32| registry.bind(&format!("{prefix}{suffix}"), shape, f32);
            let query = bind("attn_q.weight", [h, q, 1, 1], false)?;
            let key = (!shared_kv)
                .then(|| bind("attn_k.weight", [h, kv, 1, 1], false))
                .transpose()?;
            let value = has_value
                .then(|| bind("attn_v.weight", [h, kv, 1, 1], false))
                .transpose()?;
            let attention_output = bind("attn_output.weight", [q, h, 1, 1], false)?;
            let query_norm = bind("attn_q_norm.weight", [d, 1, 1, 1], true)?;
            let key_norm = (!shared_kv)
                .then(|| bind("attn_k_norm.weight", [d, 1, 1, 1], true))
                .transpose()?;
            let attention_norm = bind("attn_norm.weight", [h, 1, 1, 1], true)?;
            let post_attention_norm = bind("post_attention_norm.weight", [h, 1, 1, 1], true)?;
            let shared_pre_norm = bind("ffn_norm.weight", [h, 1, 1, 1], true)?;
            let combined_ff_norm = bind("post_ffw_norm.weight", [h, 1, 1, 1], true)?;
            let per_layer = (ple > 0)
                .then(|| -> Result<_, Error> {
                    Ok(PerLayerBindings {
                        gate: bind("inp_gate.weight", [h, ple, 1, 1], false)?,
                        projection: bind("proj.weight", [ple, h, 1, 1], false)?,
                        post_norm: bind("post_norm.weight", [h, 1, 1, 1], true)?,
                    })
                })
                .transpose()?;
            // Gemma 4 has one (causal) layer scale; DiffusionGemma has encoder and decoder ones.
            let decoder_scale = bind("layer_output_scale.weight", [1, 1, 1, 1], true)?;
            let encoder_scale = match kind {
                ModelKind::Diffusion => bind("enc_layer_output_scale.weight", [1, 1, 1, 1], true)?,
                ModelKind::Autoregressive => decoder_scale,
            };
            let moe = (spec.expert_count > 0)
                .then(|| -> Result<_, Error> {
                    let fused = spec
                        .expert_ff
                        .checked_mul(2)
                        .ok_or_else(|| Error("fused FF width overflow".into()))?;
                    Ok(MoeBindings {
                        router: RouterBindings {
                            projection: bind(
                                "ffn_gate_inp.weight",
                                [h, spec.expert_count, 1, 1],
                                true,
                            )?,
                            hidden_scale: bind("ffn_gate_inp.scale", [h, 1, 1, 1], true)?,
                            expert_scale: bind(
                                "ffn_down_exps.scale",
                                [spec.expert_count, 1, 1, 1],
                                true,
                            )?,
                        },
                        expert_gate_up: bind(
                            "ffn_gate_up_exps.weight",
                            [h, fused, spec.expert_count, 1],
                            false,
                        )?,
                        expert_down: bind(
                            "ffn_down_exps.weight",
                            [spec.expert_ff, h, spec.expert_count, 1],
                            false,
                        )?,
                        shared_post_norm: bind("post_ffw_norm_1.weight", [h, 1, 1, 1], true)?,
                        expert_pre_norm: bind("pre_ffw_norm_2.weight", [h, 1, 1, 1], true)?,
                        expert_post_norm: bind("post_ffw_norm_2.weight", [h, 1, 1, 1], true)?,
                    })
                })
                .transpose()?;
            let shared_mlp = registry.mlp(&format!("{prefix}ffn_"), h, spec.layer_ff[index])?;
            layers.push(LayerBindings {
                query,
                key,
                value,
                attention_output,
                query_norm,
                key_norm,
                attention_norm,
                post_attention_norm,
                shared_mlp,
                shared_pre_norm,
                combined_ff_norm,
                moe,
                per_layer,
                kv_source,
                encoder_scale,
                decoder_scale,
            });
        }
        registry.finish()?;
        Ok(Self {
            spec,
            vocabulary,
            embedding,
            output_norm,
            rope_frequency_factors,
            self_conditioning_pre_norm,
            self_conditioning_mlp,
            per_layer_input,
            layers,
        })
    }
}

fn read_vocabulary<'a>(metadata: &Metadata<'a>, count: usize) -> Result<Vocabulary<'a>, Error> {
    if metadata.string("tokenizer.ggml.model")? != "gemma4" {
        return Err(Error("unsupported tokenizer model".into()));
    }
    let tokens = metadata.strings("tokenizer.ggml.tokens")?;
    let scores = match metadata.array("tokenizer.ggml.scores")? {
        ArrayValues::Float(v) => v.as_slice(),
        _ => return Err(Error("expected float token scores".into())),
    };
    let token_types = match metadata.array("tokenizer.ggml.token_type")? {
        ArrayValues::Signed(v) => v.as_slice(),
        _ => return Err(Error("expected signed token types".into())),
    };
    if tokens.len() != count || scores.len() != count || token_types.len() != count {
        return Err(Error("tokenizer array length mismatch".into()));
    }
    if scores.iter().any(|s| s.is_nan() || *s == f64::INFINITY)
        || token_types.iter().any(|t| !(0..=6).contains(t))
    {
        return Err(Error("invalid token score/type".into()));
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
        scores,
        token_types,
        merges: metadata.strings("tokenizer.ggml.merges")?,
        chat_template: metadata.string("tokenizer.chat_template")?,
        bos: id("tokenizer.ggml.bos_token_id")?,
        eos: id("tokenizer.ggml.eos_token_id")?,
        padding: id("tokenizer.ggml.padding_token_id")?,
        unknown: id("tokenizer.ggml.unknown_token_id")?,
        mask: metadata
            .map
            .contains_key("tokenizer.ggml.mask_token_id")
            .then(|| id("tokenizer.ggml.mask_token_id"))
            .transpose()?,
        add_bos: metadata.boolean("tokenizer.ggml.add_bos_token")?,
        add_space_prefix: metadata.boolean("tokenizer.ggml.add_space_prefix")?,
    })
}

#[cfg(test)]
pub(crate) mod tests;
