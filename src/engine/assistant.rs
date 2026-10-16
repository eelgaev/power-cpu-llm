//! Gemma 4 multi-token-prediction drafter (`gemma4-assistant`, e.g.
//! `mtp-gemma-4-26B-A4B-it-Q4_0.gguf`): a small decoder that proposes the
//! target's next tokens for speculative decoding. It follows llama.cpp
//! `src/models/gemma4-assistant.cpp` and `common/speculative.cpp`:
//!
//! - input: a token and `h`, the target's final-normed hidden state of the
//!   position before it; `x = target_embedding[token] * sqrt(target hidden)`,
//!   then `pre_projection([x; h])` to the drafter width;
//! - each layer: pre-norm, Q projection with per-head norm and RoPE, attention
//!   with scale 1 over the TARGET's K/V cache (sliding layers read the target's
//!   second-to-last layer, global layers its last), post-attention norm and
//!   residual, gated GELU MLP with pre/post norms and residual, then the layer
//!   output scale;
//! - output: `norm(x)` times the drafter's own (tied) embedding gives the draft
//!   logits (no softcap); `post_projection(norm(x))` is the next step's `h`.
//!
//! Every draft step uses the same position (the next one of the target cache)
//! and adds nothing to any cache.

use crate::engine::attention::HeadTransform;
use crate::engine::execution::Compute;
use crate::engine::model::{
    self, AttentionGeometry, AttentionKind, Metadata, ModelDefinition, Registry,
};
use crate::engine::operators::{self, Error, MAX_REFERENCE_MATRIX_ELEMENTS};
use crate::engine::text::EncoderCache;
use crate::engine::weights::ModelFile;
use crate::gguf::{ArrayValues, Inventory, TensorInfo};
use crate::kernels::reference as math;

pub const ARCHITECTURE: &str = "gemma4-assistant";

struct LayerBindings<'a> {
    kind: AttentionKind,
    head_dim: usize,
    kv_heads: usize,
    attention_norm: &'a TensorInfo,
    query: &'a TensorInfo,
    query_norm: &'a TensorInfo,
    output: &'a TensorInfo,
    post_attention_norm: &'a TensorInfo,
    ffn_norm: &'a TensorInfo,
    gate: &'a TensorInfo,
    up: &'a TensorInfo,
    down: &'a TensorInfo,
    post_ffw_norm: &'a TensorInfo,
    output_scale: &'a TensorInfo,
}

/// The drafter's tensors, bound against the target it drafts for.
pub struct AssistantDefinition<'a> {
    hidden: usize,
    query_heads: usize,
    rms_epsilon: f32,
    rope_base_global: f32,
    rope_base_sliding: f32,
    embedding: &'a TensorInfo,
    output_norm: &'a TensorInfo,
    pre_projection: &'a TensorInfo,
    post_projection: &'a TensorInfo,
    rope_frequency_factors: &'a TensorInfo,
    layers: Vec<LayerBindings<'a>>,
}

impl<'a> AssistantDefinition<'a> {
    pub fn bind(
        inventory: &'a Inventory,
        target: &ModelDefinition<'_>,
    ) -> Result<Self, model::Error> {
        let fail = |m: &str| model::Error(m.into());
        if inventory.architecture() != Some(ARCHITECTURE) {
            return Err(fail("not a gemma4-assistant drafter"));
        }
        let metadata = Metadata {
            map: &inventory.metadata,
            prefix: "gemma4-assistant.",
        };
        let spec = &target.spec;
        let count = metadata.positive("block_count")?;
        let hidden = metadata.positive("embedding_length")?;
        if metadata.positive("embedding_length_out")? != spec.hidden {
            return Err(fail(
                "drafter output width is not the target's hidden width",
            ));
        }
        let query_heads = metadata.positive("attention.head_count")?;
        let kv_heads = metadata.per_layer("attention.head_count_kv", count)?;
        let ff = metadata.per_layer("feed_forward_length", count)?;
        let (global_dim, sliding_dim) = (
            metadata.positive("attention.key_length")?,
            metadata.positive("attention.key_length_swa")?,
        );
        if metadata.positive("attention.value_length")? != global_dim
            || metadata.positive("attention.value_length_swa")? != sliding_dim
            || metadata.positive("attention.sliding_window")? != spec.sliding_window
        {
            return Err(fail("drafter K/V widths or window differ from the target"));
        }
        let pattern = match metadata.array(&format!(
            "{}attention.sliding_window_pattern",
            metadata.prefix
        ))? {
            ArrayValues::Bool(v) if v.len() == count => v.clone(),
            _ => return Err(fail("expected one sliding flag per drafter layer")),
        };
        // The target K/V each drafter layer reads (see `source_layer`) must
        // have the drafter's geometry.
        let n = spec.attention.len();
        if n < 2 {
            return Err(fail("target too shallow"));
        }
        let mut registry = Registry::new(inventory)?;
        let h = hidden;
        let embedding = registry.bind("token_embd.weight", [h, spec.vocabulary, 1, 1], false)?;
        let output_norm = registry.vector("output_norm.weight", h)?;
        let pre_projection = registry.bind(
            "nextn.pre_projection.weight",
            [2 * spec.hidden, h, 1, 1],
            false,
        )?;
        let post_projection = registry.bind(
            "nextn.post_projection.weight",
            [h, spec.hidden, 1, 1],
            false,
        )?;
        let rope_frequency_factors = registry.vector("rope_freqs.weight", global_dim / 2)?;
        let mut layers = Vec::with_capacity(count);
        for i in 0..count {
            let kind = if pattern[i] {
                AttentionKind::Sliding
            } else {
                AttentionKind::Global
            };
            let head_dim = if pattern[i] { sliding_dim } else { global_dim };
            let target_geometry = &spec.attention[source_layer(target, kind)];
            if target_geometry.kind != kind
                || target_geometry.head_dim != head_dim
                || target_geometry.kv_heads != kv_heads[i]
                || !query_heads.is_multiple_of(kv_heads[i])
            {
                return Err(fail(
                    "drafter attention geometry differs from the target K/V",
                ));
            }
            let p = format!("blk.{i}.");
            let mut bind = |s: &str, shape, f32| registry.bind(&format!("{p}{s}"), shape, f32);
            layers.push(LayerBindings {
                kind,
                head_dim,
                kv_heads: kv_heads[i],
                attention_norm: bind("attn_norm.weight", [h, 1, 1, 1], true)?,
                query: bind("attn_q.weight", [h, query_heads * head_dim, 1, 1], false)?,
                query_norm: bind("attn_q_norm.weight", [head_dim, 1, 1, 1], true)?,
                output: bind(
                    "attn_output.weight",
                    [query_heads * head_dim, h, 1, 1],
                    false,
                )?,
                post_attention_norm: bind("post_attention_norm.weight", [h, 1, 1, 1], true)?,
                ffn_norm: bind("ffn_norm.weight", [h, 1, 1, 1], true)?,
                gate: bind("ffn_gate.weight", [h, ff[i], 1, 1], false)?,
                up: bind("ffn_up.weight", [h, ff[i], 1, 1], false)?,
                down: bind("ffn_down.weight", [ff[i], h, 1, 1], false)?,
                post_ffw_norm: bind("post_ffw_norm.weight", [h, 1, 1, 1], true)?,
                output_scale: bind("layer_output_scale.weight", [1, 1, 1, 1], true)?,
            });
        }
        registry.finish()?;
        Ok(Self {
            hidden,
            query_heads,
            rms_epsilon: metadata.float("attention.layer_norm_rms_epsilon")?,
            rope_base_global: metadata.float("rope.freq_base")?,
            rope_base_sliding: metadata.float("rope.freq_base_swa")?,
            embedding,
            output_norm,
            pre_projection,
            post_projection,
            rope_frequency_factors,
            layers,
        })
    }
}

/// The target layer whose K/V a drafter layer of `kind` reads (llama.cpp:
/// sliding layers the target's second-to-last layer, global its last),
/// following a shared-K/V layer to the layer that owns the cache.
fn source_layer(target: &ModelDefinition<'_>, kind: AttentionKind) -> usize {
    let n = target.layers.len();
    let layer = if kind == AttentionKind::Sliding {
        n - 2
    } else {
        n - 1
    };
    target.layers[layer].kv_source.unwrap_or(layer)
}

struct DraftLayer<'a> {
    bindings: LayerBindings<'a>,
    transform: HeadTransform,
    source: usize,
    attention_norm: Vec<f32>,
    post_attention_norm: Vec<f32>,
    ffn_norm: Vec<f32>,
    post_ffw_norm: Vec<f32>,
    output_scale: f32,
}

/// One draft step's result.
#[derive(Debug, Clone)]
pub struct Draft {
    pub token: u32,
    /// Softmax probability of `token` under the draft logits.
    pub probability: f32,
    /// `h` for the next draft step.
    pub hidden: Vec<f32>,
}

/// A bound drafter with its small weights loaded.
pub struct Drafter<'a> {
    file: &'a ModelFile,
    target_file: &'a ModelFile,
    target: &'a ModelDefinition<'a>,
    hidden: usize,
    rms_epsilon: f32,
    embedding: &'a TensorInfo,
    pre_projection: &'a TensorInfo,
    post_projection: &'a TensorInfo,
    output_norm: Vec<f32>,
    layers: Vec<DraftLayer<'a>>,
}

impl<'a> Drafter<'a> {
    pub fn new(
        file: &'a ModelFile,
        target_file: &'a ModelFile,
        target: &'a ModelDefinition<'a>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let def = AssistantDefinition::bind(file.inventory(), target)?;
        let factors = operators::load_vector(file, def.rope_frequency_factors)?;
        let mut layers = Vec::with_capacity(def.layers.len());
        for b in def.layers {
            let global = b.kind == AttentionKind::Global;
            let query_scale = operators::load_vector(file, b.query_norm)?;
            let transform = HeadTransform::new(
                AttentionGeometry {
                    kind: b.kind,
                    query_heads: def.query_heads,
                    kv_heads: b.kv_heads,
                    head_dim: b.head_dim,
                    rotary_dim: b.head_dim,
                },
                // Unused: the drafter has no keys of its own.
                query_scale.clone(),
                query_scale,
                def.rms_epsilon,
                if global {
                    def.rope_base_global
                } else {
                    def.rope_base_sliding
                },
                global.then(|| factors.clone()),
            )?;
            let scale = operators::load_vector(file, b.output_scale)?;
            layers.push(DraftLayer {
                transform,
                source: source_layer(target, b.kind),
                attention_norm: operators::load_vector(file, b.attention_norm)?,
                post_attention_norm: operators::load_vector(file, b.post_attention_norm)?,
                ffn_norm: operators::load_vector(file, b.ffn_norm)?,
                post_ffw_norm: operators::load_vector(file, b.post_ffw_norm)?,
                output_scale: scale[0],
                bindings: b,
            });
        }
        Ok(Self {
            file,
            target_file,
            target,
            hidden: def.hidden,
            rms_epsilon: def.rms_epsilon,
            embedding: def.embedding,
            pre_projection: def.pre_projection,
            post_projection: def.post_projection,
            output_norm: operators::load_vector(file, def.output_norm)?,
            layers,
        })
    }

    /// Packs the drafter's projections for the packing (int8) backends.
    pub fn prepack(&self, compute: Compute<'_>) -> Result<(), Error> {
        let mut tensors = vec![self.embedding, self.pre_projection, self.post_projection];
        for l in &self.layers {
            let b = &l.bindings;
            tensors.extend([b.query, b.output, b.gate, b.up, b.down]);
        }
        operators::prepack(self.file, &tensors, compute)
    }

    /// Drafts the token after `token`, which will occupy target position
    /// `position` (the target cache's next one); `h` is the target's (or the
    /// previous draft step's) hidden state for it.
    pub fn step(
        &self,
        cache: &EncoderCache<'_>,
        token: u32,
        h: &[f32],
        position: usize,
        compute: Compute<'_>,
    ) -> Result<Draft, Error> {
        let target_hidden = self.target.spec.hidden;
        if h.len() != target_hidden || token as usize >= self.target.spec.vocabulary {
            return Err(Error::Shape("draft input"));
        }
        let project = |t: &TensorInfo, x: &[f32]| -> Result<Vec<f32>, Error> {
            let mut out = operators::project_streamed_batch_with(
                self.file,
                t,
                &[x.to_vec()],
                MAX_REFERENCE_MATRIX_ELEMENTS,
                compute,
            )?;
            Ok(out.pop().expect("one position"))
        };
        // The target's scaled token embedding, then [x; h].
        let raw = self
            .target_file
            .read_row(&self.target.embedding.name, u64::from(token))?
            .decode()?;
        let ones = vec![1.0; target_hidden];
        let mut xh = math::multiply_scaled(&raw, &ones, (target_hidden as f32).sqrt())?;
        xh.extend_from_slice(h);
        let mut x = project(self.pre_projection, &xh)?;
        let eps = self.rms_epsilon;
        for l in &self.layers {
            let b = &l.bindings;
            let normed = math::rms_norm(&x, Some(&l.attention_norm), eps)?;
            let query = l
                .transform
                .prepare_query(&project(b.query, &normed)?, position)?;
            let source = cache
                .layer(l.source)
                .ok_or(Error::Shape("target cache layer"))?;
            // Query heads split over the workers, each computed as by the
            // whole call.
            const HEADS_PER_ITEM: usize = 2;
            let heads = query.len() / b.head_dim;
            let parts = compute.map_positions(heads.div_ceil(HEADS_PER_ITEM), |part| {
                let first = part * HEADS_PER_ITEM;
                Ok::<_, Error>(compute.kernels.attention_heads(
                    &query,
                    source.keys(),
                    source.values(),
                    b.kv_heads,
                    b.head_dim,
                    None,
                    Some(first..(first + HEADS_PER_ITEM).min(heads)),
                )?)
            })?;
            let mixed = parts.concat();
            let attended = math::rms_norm(
                &project(b.output, &mixed)?,
                Some(&l.post_attention_norm),
                eps,
            )?;
            let residual = math::add(&attended, &x)?;
            let normed = math::rms_norm(&residual, Some(&l.ffn_norm), eps)?;
            let mut gate_up =
                operators::project_many(self.file, &[b.gate, b.up], &[normed], compute)?
                    .into_iter();
            let gate = gate_up.next().expect("gate").pop().expect("one position");
            let up = gate_up.next().expect("up").pop().expect("one position");
            let mlp = project(b.down, &math::gelu_product(&gate, &up)?)?;
            let out = math::add(
                &math::rms_norm(&mlp, Some(&l.post_ffw_norm), eps)?,
                &residual,
            )?;
            x = math::multiply_scaled(&out, &vec![1.0; out.len()], l.output_scale)?;
        }
        let normed = math::rms_norm(&x, Some(&self.output_norm), eps)?;
        let mut outputs = operators::project_many(
            self.file,
            &[self.embedding, self.post_projection],
            &[normed],
            compute,
        )?
        .into_iter();
        let logits = outputs.next().expect("logits").pop().expect("one position");
        let hidden = outputs.next().expect("h").pop().expect("one position");
        let (best, &top) = logits
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(b.1).then(b.0.cmp(&a.0)))
            .expect("nonempty vocabulary");
        let total: f64 = logits
            .iter()
            .map(|&l| crate::libm::exp(f64::from(l) - f64::from(top)))
            .sum();
        debug_assert_eq!(self.hidden, x.len());
        Ok(Draft {
            token: best as u32,
            probability: (1.0 / total) as f32,
            hidden,
        })
    }
}
