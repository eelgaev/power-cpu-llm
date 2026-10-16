//! Correctness-oriented single-request attention/block execution. Input rows
//! are already embedded hidden states, not token IDs. No tokenizer, full model
//! forward, padding/batching, optimized GEMM, or generation is provided here.

use crate::engine::{
    model::{AttentionGeometry, AttentionKind, ModelDefinition},
    operators::{self, Error, LayerMode},
    weights::ModelFile,
};
use crate::kernels::reference as math;

fn widths(g: &AttentionGeometry) -> Result<(usize, usize), Error> {
    if g.query_heads == 0
        || g.kv_heads == 0
        || !g.query_heads.is_multiple_of(g.kv_heads)
        || g.head_dim == 0
        || !g.head_dim.is_multiple_of(2)
        || g.rotary_dim != g.head_dim
    {
        return Err(Error::Shape(
            "unsupported attention/full-head rotary geometry",
        ));
    }
    Ok((
        g.query_heads
            .checked_mul(g.head_dim)
            .ok_or(Error::ResourceLimit)?,
        g.kv_heads
            .checked_mul(g.head_dim)
            .ok_or(Error::ResourceLimit)?,
    ))
}

fn buffer(count: usize) -> Result<Vec<f32>, Error> {
    let mut out = Vec::new();
    out.try_reserve_exact(count)
        .map_err(|_| Error::ResourceLimit)?;
    Ok(out)
}

fn norm_heads(x: &[f32], dim: usize, scale: Option<&[f32]>, eps: f32) -> Result<Vec<f32>, Error> {
    let mut out = buffer(x.len())?;
    for head in x.chunks_exact(dim) {
        out.extend(math::rms_norm(head, scale, eps)?);
    }
    Ok(out)
}

/// Head normalization and positional transform, separated from projection to
/// permit deterministic independent fixtures (especially global K-as-V).
pub struct HeadTransform {
    geometry: AttentionGeometry,
    query_scale: Vec<f32>,
    key_scale: Vec<f32>,
    epsilon: f32,
    base: f32,
    factors: Option<Vec<f32>>,
}
pub struct PreparedToken {
    pub query: Vec<f32>,
    pub key: Vec<f32>,
    pub value: Vec<f32>,
}
impl HeadTransform {
    pub fn new(
        geometry: AttentionGeometry,
        query_scale: Vec<f32>,
        key_scale: Vec<f32>,
        epsilon: f32,
        base: f32,
        factors: Option<Vec<f32>>,
    ) -> Result<Self, Error> {
        widths(&geometry)?;
        if query_scale.len() != geometry.head_dim || key_scale.len() != geometry.head_dim {
            return Err(Error::Shape("head norm scale width"));
        }
        if !epsilon.is_finite()
            || epsilon <= 0.0
            || !base.is_finite()
            || base <= 0.0
            || query_scale.iter().chain(&key_scale).any(|x| !x.is_finite())
        {
            return Err(Error::Parameter("head normalization/RoPE parameters"));
        }
        if let Some(f) = &factors {
            if f.len() != geometry.head_dim / 2 {
                return Err(Error::Shape("RoPE frequency factor width"));
            }
            if f.iter().any(|&x| !x.is_finite() || x <= 0.0) {
                return Err(Error::Parameter("RoPE frequency factors"));
            }
        }
        Ok(Self {
            geometry,
            query_scale,
            key_scale,
            epsilon,
            base,
            factors,
        })
    }
    pub fn prepare(
        &self,
        raw_query: &[f32],
        raw_key: &[f32],
        raw_value: Option<&[f32]>,
        position: usize,
    ) -> Result<PreparedToken, Error> {
        let (qw, kw) = widths(&self.geometry)?;
        if raw_query.len() != qw || raw_key.len() != kw || raw_value.is_some_and(|v| v.len() != kw)
        {
            return Err(Error::Shape("Q/K/V projection width"));
        }
        // V is projected where the layer has a V matrix; 26B global layers
        // reuse the raw K instead (E2B global layers have their own V).
        let value = match (self.geometry.kind, raw_value) {
            (_, Some(v)) => v,
            (AttentionKind::Global, None) => raw_key,
            (AttentionKind::Sliding, None) => {
                return Err(Error::Shape("sliding attention needs V"));
            }
        };
        let query = norm_heads(
            raw_query,
            self.geometry.head_dim,
            Some(&self.query_scale),
            self.epsilon,
        )?;
        let key = norm_heads(
            raw_key,
            self.geometry.head_dim,
            Some(&self.key_scale),
            self.epsilon,
        )?;
        // Values use raw projections, not normalized/rotated keys. No learned
        // V scale and no RoPE on values, even for global K-as-V.
        let value = norm_heads(value, self.geometry.head_dim, None, self.epsilon)?;
        Ok(PreparedToken {
            query: math::rope(
                &query,
                self.geometry.head_dim,
                position,
                self.base,
                self.factors.as_deref(),
            )?,
            key: math::rope(
                &key,
                self.geometry.head_dim,
                position,
                self.base,
                self.factors.as_deref(),
            )?,
            value,
        })
    }
}

impl HeadTransform {
    /// [`Self::prepare`] with each head (query, key, value) on the workers:
    /// heads are independent, so every value is identical.
    pub(crate) fn prepare_split(
        &self,
        raw_query: &[f32],
        raw_key: &[f32],
        raw_value: Option<&[f32]>,
        position: usize,
        compute: crate::engine::execution::Compute<'_>,
    ) -> Result<PreparedToken, Error> {
        let (qw, kw) = widths(&self.geometry)?;
        if raw_query.len() != qw || raw_key.len() != kw || raw_value.is_some_and(|v| v.len() != kw)
        {
            return Err(Error::Shape("Q/K/V projection width"));
        }
        let value = match (self.geometry.kind, raw_value) {
            (_, Some(v)) => v,
            (AttentionKind::Global, None) => raw_key,
            (AttentionKind::Sliding, None) => {
                return Err(Error::Shape("sliding attention needs V"));
            }
        };
        let dim = self.geometry.head_dim;
        let (nq, nk) = (qw / dim, kw / dim);
        let rotate = |x: &[f32]| -> Result<Vec<f32>, Error> {
            Ok(math::rope(
                x,
                dim,
                position,
                self.base,
                self.factors.as_deref(),
            )?)
        };
        let heads = compute.map_positions(nq + 2 * nk, |h| {
            if h < nq {
                rotate(&math::rms_norm(
                    &raw_query[h * dim..(h + 1) * dim],
                    Some(&self.query_scale),
                    self.epsilon,
                )?)
            } else if h < nq + nk {
                let h = h - nq;
                rotate(&math::rms_norm(
                    &raw_key[h * dim..(h + 1) * dim],
                    Some(&self.key_scale),
                    self.epsilon,
                )?)
            } else {
                let h = h - nq - nk;
                Ok(math::rms_norm(
                    &value[h * dim..(h + 1) * dim],
                    None,
                    self.epsilon,
                )?)
            }
        })?;
        let mut heads = heads.into_iter();
        let mut take = |n: usize| -> Vec<f32> { heads.by_ref().take(n).flatten().collect() };
        let query = take(nq);
        let key = take(nk);
        let value = take(nk);
        Ok(PreparedToken { query, key, value })
    }
    /// The query half of [`Self::prepare`], for shared-K/V layers.
    pub fn prepare_query(&self, raw_query: &[f32], position: usize) -> Result<Vec<f32>, Error> {
        let (qw, _) = widths(&self.geometry)?;
        if raw_query.len() != qw {
            return Err(Error::Shape("Q projection width"));
        }
        let query = norm_heads(
            raw_query,
            self.geometry.head_dim,
            Some(&self.query_scale),
            self.epsilon,
        )?;
        Ok(math::rope(
            &query,
            self.geometry.head_dim,
            position,
            self.base,
            self.factors.as_deref(),
        )?)
    }
}

/// A cache belongs to a stable borrowed model-file owner and one layer. K/V
/// are token-major and contain only encoder states. No decoder mutation API.
/// max_bytes bounds each staged K+V pair, NOT total process/working-set memory.
/// Staging may coexist with the old cache; matrix/query/output scratch is extra.
pub struct LayerCache<'a> {
    owner: &'a ModelFile,
    layer: usize,
    geometry: AttentionGeometry,
    context: usize,
    window: Option<usize>,
    max_bytes: usize,
    total: usize,
    /// Retained K/V are `keys[start..]` / `values[start..]`: window eviction
    /// moves `start` and compacts the dead prefix only occasionally.
    start: usize,
    keys: Vec<f32>,
    values: Vec<f32>,
}
/// Undo record of one in-place encoder step (see `encode_in_place`).
pub(crate) struct PendingKv {
    old_len: usize,
    old_total: usize,
}
impl LayerCache<'_> {
    pub fn total_tokens(&self) -> usize {
        self.total
    }
    pub fn retained_tokens(&self) -> usize {
        (self.keys.len() - self.start) / (self.geometry.kv_heads * self.geometry.head_dim)
    }
    pub fn keys(&self) -> &[f32] {
        &self.keys[self.start..]
    }
    pub fn values(&self) -> &[f32] {
        &self.values[self.start..]
    }
    /// Undo an uncommitted in-place step (the appended K/V and the position).
    pub(crate) fn rollback(&mut self, pending: &PendingKv) {
        self.keys.truncate(pending.old_len);
        self.values.truncate(pending.old_len);
        self.total = pending.old_total;
    }
    /// Keep only the first `tokens` positions of an uncommitted in-place step
    /// (speculative verification), before [`Self::commit`].
    pub(crate) fn keep(&mut self, pending: &PendingKv, tokens: usize) {
        let width = self.geometry.kv_heads * self.geometry.head_dim;
        let len = pending.old_len + tokens * width;
        if len < self.keys.len() {
            self.keys.truncate(len);
            self.values.truncate(len);
        }
        self.total = pending.old_total + tokens;
    }
    /// Apply window eviction after a successful pass: sliding layers keep the
    /// last `window - 1` tokens (as `DynamicSlidingWindowLayer`). The dead
    /// prefix is dropped once it is at least half the buffer.
    pub(crate) fn commit(&mut self) {
        let width = self.geometry.kv_heads * self.geometry.head_dim;
        let count = (self.keys.len() - self.start) / width;
        let keep = retained_count(count, self.window);
        self.start += (count - keep) * width;
        if self.start > 0 && self.start * 2 >= self.keys.len() {
            self.keys.drain(..self.start);
            self.values.drain(..self.start);
            self.start = 0;
        }
    }
}

/// One layer's attention inputs for a pass: the queries plus the retained
/// cache followed by this pass's K/V (before window truncation). Shared-K/V
/// layers attend over their source layer's staged K/V of the same pass.
struct StagedKv {
    queries: Vec<Vec<f32>>,
    keys: Vec<f32>,
    values: Vec<f32>,
    first_key: usize,
    first_query: usize,
}
impl StagedKv {
    fn view(&self) -> StageRef<'_> {
        StageRef {
            queries: &self.queries,
            keys: &self.keys,
            values: &self.values,
            first_key: self.first_key,
            first_query: self.first_query,
        }
    }
}
#[derive(Clone, Copy)]
struct StageRef<'s> {
    queries: &'s [Vec<f32>],
    keys: &'s [f32],
    values: &'s [f32],
    first_key: usize,
    first_query: usize,
}

/// Binds the reference attention path for one real GGUF layer. Weight
/// projections and FFN matrices are re-read per input row; not production speed.
pub struct ReferenceBlock<'a> {
    file: &'a ModelFile,
    model: &'a ModelDefinition<'a>,
    layer: usize,
    transform: HeadTransform,
    pre_scale: Vec<f32>,
    post_scale: Vec<f32>,
    batched: bool,
    workers: Option<std::sync::Arc<crate::engine::execution::Workers>>,
    /// Second-socket workers for prompt chunks (see `encode_in_place`).
    split: Option<std::sync::Arc<crate::engine::execution::Split>>,
    /// Higher SMT threads that join prompt attention only (see `finish`).
    extra: Option<std::sync::Arc<crate::engine::execution::Workers>>,
    kernels: crate::engine::execution::MatrixKernels,
    /// Optional kernels for multi-token encoder chunks (prompt prefill), e.g.
    /// single-plane int8 under a two-plane serving backend.
    prefill: Option<crate::engine::execution::MatrixKernels>,
    /// True while an encoder chunk of more than one token runs.
    prefilling: std::sync::atomic::AtomicBool,
}
/// Clears `prefilling` when an encoder step ends (including on error).
struct PrefillGuard<'b>(&'b std::sync::atomic::AtomicBool);
impl Drop for PrefillGuard<'_> {
    fn drop(&mut self) {
        self.0.store(false, std::sync::atomic::Ordering::Relaxed);
    }
}
impl<'a> ReferenceBlock<'a> {
    pub fn new(
        file: &'a ModelFile,
        model: &'a ModelDefinition<'a>,
        layer: usize,
    ) -> Result<Self, Error> {
        let binding = model.layers.get(layer).ok_or(Error::Shape("layer index"))?;
        let g = model
            .spec
            .attention
            .get(layer)
            .ok_or(Error::Shape("layer geometry"))?;
        let global = g.kind == AttentionKind::Global;
        let transform = HeadTransform::new(
            g.clone(),
            operators::load_vector(file, binding.query_norm)?,
            // Shared-K/V layers have no K norm (their keys are the source's).
            match binding.key_norm {
                Some(norm) => operators::load_vector(file, norm)?,
                None => vec![1.0; g.head_dim],
            },
            model.spec.rms_epsilon,
            if global {
                model.spec.rope_base_global
            } else {
                model.spec.rope_base_sliding
            },
            if global {
                Some(operators::load_vector(file, model.rope_frequency_factors)?)
            } else {
                None
            },
        )?;
        let pre_scale = operators::load_vector(file, binding.attention_norm)?;
        let post_scale = operators::load_vector(file, binding.post_attention_norm)?;
        if pre_scale.len() != model.spec.hidden
            || post_scale.len() != model.spec.hidden
            || model.spec.context == 0
            || model.spec.sliding_window == 0
        {
            return Err(Error::Shape("attention block norm/context dimensions"));
        }
        Ok(Self {
            file,
            model,
            layer,
            transform,
            pre_scale,
            post_scale,
            batched: false,
            workers: None,
            split: None,
            extra: None,
            kernels: crate::engine::execution::MatrixKernels::portable(),
            prefill: None,
            prefilling: std::sync::atomic::AtomicBool::new(false),
        })
    }
    pub(crate) fn enable_batching(
        &mut self,
        workers: Option<std::sync::Arc<crate::engine::execution::Workers>>,
        kernels: crate::engine::execution::MatrixKernels,
    ) {
        self.batched = true;
        self.workers = workers;
        self.kernels = kernels;
    }
    pub(crate) fn set_split(
        &mut self,
        split: Option<std::sync::Arc<crate::engine::execution::Split>>,
    ) {
        self.split = split;
    }
    pub(crate) fn set_attention_extra(
        &mut self,
        extra: Option<std::sync::Arc<crate::engine::execution::Workers>>,
    ) {
        self.extra = extra;
    }
    /// The calling thread's workers: the second socket's on its replica.
    fn pool(&self) -> Option<&crate::engine::execution::Workers> {
        match (&self.split, crate::engine::execution::replica()) {
            (Some(split), 1) => Some(&split.workers),
            _ => self.workers.as_deref(),
        }
    }
    pub(crate) fn set_prefill_kernels(
        &mut self,
        kernels: Option<crate::engine::execution::MatrixKernels>,
    ) {
        self.prefill = kernels;
    }
    /// The prefill kernels during a multi-token encoder chunk, else the main ones.
    fn active_kernels(&self) -> crate::engine::execution::MatrixKernels {
        match self.prefill {
            Some(k) if self.prefilling.load(std::sync::atomic::Ordering::Relaxed) => k,
            _ => self.kernels,
        }
    }
    /// Marks a multi-token encoder chunk (prefill kernels, if any) until dropped.
    fn prefill_scope(&self, tokens: usize) -> PrefillGuard<'_> {
        self.prefilling
            .store(tokens > 1, std::sync::atomic::Ordering::Relaxed);
        PrefillGuard(&self.prefilling)
    }
    fn compute(&self) -> crate::engine::execution::Compute<'_> {
        crate::engine::execution::Compute {
            workers: self.pool(),
            kernels: self.active_kernels(),
        }
    }

    pub fn new_cache(&self, max_bytes: usize) -> Result<LayerCache<'a>, Error> {
        let g = &self.transform.geometry;
        let (_, width) = widths(g)?;
        let one_token = width
            .checked_mul(2 * size_of::<f32>())
            .ok_or(Error::ResourceLimit)?;
        if max_bytes < one_token {
            return Err(Error::ResourceLimit);
        }
        Ok(LayerCache {
            owner: self.file,
            layer: self.layer,
            geometry: g.clone(),
            context: self.model.spec.context,
            window: (g.kind == AttentionKind::Sliding).then_some(self.model.spec.sliding_window),
            max_bytes,
            total: 0,
            start: 0,
            keys: Vec::new(),
            values: Vec::new(),
        })
    }
    /// Shared checks of a staged pass; returns (next position, K/V cells).
    fn check_stage(
        &self,
        input: &[Vec<f32>],
        cache: &LayerCache<'_>,
        mode: LayerMode,
    ) -> Result<(usize, usize), Error> {
        let spec = &self.model.spec;
        if !std::ptr::eq(cache.owner, self.file)
            || cache.layer != self.layer
            || cache.geometry != self.transform.geometry
            || cache.context != spec.context
            || cache.window
                != (self.transform.geometry.kind == AttentionKind::Sliding)
                    .then_some(spec.sliding_window)
        {
            return Err(Error::Shape("cache owner/layer/geometry mismatch"));
        }
        if input.is_empty() || input.iter().any(|x| x.len() != spec.hidden) {
            return Err(Error::Shape("hidden-state rows"));
        }
        let nonfinite = |rows: &[Vec<f32>]| !rows.iter().all(|r| math::all_finite(r));
        let found = match self.pool() {
            Some(w) if input.len() >= 32 => w
                .ranges(input.len(), |first, end| {
                    Ok::<_, Error>(nonfinite(&input[first..end]))
                })?
                .contains(&true),
            _ => nonfinite(input),
        };
        if found {
            return Err(Error::Parameter("nonfinite hidden states"));
        }
        if matches!(mode, LayerMode::Decoder) && input.len() > spec.canvas {
            return Err(Error::Shape("decoder input exceeds canvas length"));
        }
        let next_position = cache
            .total
            .checked_add(input.len())
            .filter(|&n| n <= spec.context)
            .ok_or(Error::ResourceLimit)?;
        let (_, width) = widths(&cache.geometry)?;
        let count = cache
            .retained_tokens()
            .checked_add(input.len())
            .ok_or(Error::ResourceLimit)?;
        let cells = count.checked_mul(width).ok_or(Error::ResourceLimit)?;
        if cells
            .checked_mul(2 * size_of::<f32>())
            .filter(|&b| b <= cache.max_bytes)
            .is_none()
        {
            return Err(Error::ResourceLimit);
        }
        Ok((next_position, cells))
    }
    fn stage(
        &self,
        input: &[Vec<f32>],
        cache: &LayerCache<'_>,
        mode: LayerMode,
    ) -> Result<StagedKv, Error> {
        let (_, cells) = self.check_stage(input, cache, mode)?;
        let mut keys = buffer(cells)?;
        let mut values = buffer(cells)?;
        keys.extend_from_slice(cache.keys());
        values.extend_from_slice(cache.values());
        let queries = self.project_tokens(input, cache.total, &mut keys, &mut values)?;
        Ok(StagedKv {
            queries,
            keys,
            values,
            first_key: cache.total - cache.retained_tokens(),
            first_query: cache.total,
        })
    }
    /// Queries of `input` at positions `first..`, with their K/V appended to
    /// `keys` / `values`.
    fn project_tokens(
        &self,
        input: &[Vec<f32>],
        first: usize,
        keys: &mut Vec<f32>,
        values: &mut Vec<f32>,
    ) -> Result<Vec<Vec<f32>>, Error> {
        let spec = &self.model.spec;
        let mut queries = Vec::new();
        queries
            .try_reserve_exact(input.len())
            .map_err(|_| Error::ResourceLimit)?;
        let layer = &self.model.layers[self.layer];
        if self.batched {
            let _span =
                crate::engine::stats::Span::new(crate::engine::stats::Stage::AttentionProjections);
            let compute = self.compute();
            let normalized = compute.map_positions(input.len(), |i| {
                Ok::<_, Error>(math::rms_norm(
                    &input[i],
                    Some(&self.pre_scale),
                    spec.rms_epsilon,
                )?)
            })?;
            let key_tensor = layer
                .key
                .ok_or(Error::Shape("shared-K/V layer staged without its source"))?;
            let mut tensors = vec![layer.query, key_tensor];
            tensors.extend(layer.value);
            let mut projected =
                operators::project_many(self.file, &tensors, &normalized, compute)?.into_iter();
            let (Some(query), Some(key)) = (projected.next(), projected.next()) else {
                return Err(Error::Shape("Q/K projections"));
            };
            let value = projected.next();
            let prepared = if input.len() == 1 && compute.workers.is_some() {
                vec![self.transform.prepare_split(
                    &query[0],
                    &key[0],
                    value.as_ref().map(|v| v[0].as_slice()),
                    first,
                    compute,
                )?]
            } else {
                compute.map_positions(input.len(), |i| {
                    self.transform.prepare(
                        &query[i],
                        &key[i],
                        value.as_ref().map(|v| v[i].as_slice()),
                        first + i,
                    )
                })?
            };
            for prepared in prepared {
                queries.push(prepared.query);
                keys.extend(prepared.key);
                values.extend(prepared.value);
            }
        } else {
            for (i, row) in input.iter().enumerate() {
                let x = math::rms_norm(row, Some(&self.pre_scale), spec.rms_epsilon)?;
                let query = operators::project_streamed(self.file, layer.query, &x)?;
                let key = operators::project_streamed(
                    self.file,
                    layer
                        .key
                        .ok_or(Error::Shape("shared-K/V layer staged without its source"))?,
                    &x,
                )?;
                let value = layer
                    .value
                    .map(|v| operators::project_streamed(self.file, v, &x))
                    .transpose()?;
                let prepared = self
                    .transform
                    .prepare(&query, &key, value.as_deref(), first + i)?;
                queries.push(prepared.query);
                keys.extend(prepared.key);
                values.extend(prepared.value);
            }
        }
        Ok(queries)
    }
    /// Layer output rows plus each position's router decision (diagnostics only).
    fn finish(
        &self,
        input: &[Vec<f32>],
        stage: StageRef<'_>,
        mode: LayerMode,
        per_layer: Option<&[Vec<f32>]>,
    ) -> Result<(Vec<Vec<f32>>, Vec<operators::Routing>), Error> {
        if per_layer.is_some() && !self.batched {
            return Err(Error::Shape("per-layer inputs need batched execution"));
        }
        let g = &self.transform.geometry;
        let (_, width) = widths(g)?;
        let count = stage.keys.len() / width;
        let mut output = Vec::new();
        output
            .try_reserve_exact(input.len())
            .map_err(|_| Error::ResourceLimit)?;
        let mut routing = Vec::new();
        let attend_heads = |i: usize,
                            heads: Option<std::ops::Range<usize>>|
         -> Result<Vec<f32>, Error> {
            let allowed = match mode {
                LayerMode::Encoder => Some(encoder_mask(
                    stage.first_key,
                    count,
                    stage.first_query + i,
                    (g.kind == AttentionKind::Sliding).then_some(self.model.spec.sliding_window),
                )?),
                // Cached local context is already clipped to window-1. All
                // canvas positions remain visible, including future positions.
                LayerMode::Decoder => None,
            };
            Ok(self.active_kernels().attention_heads(
                &stage.queries[i],
                stage.keys,
                stage.values,
                g.kv_heads,
                g.head_dim,
                allowed.as_deref(),
                heads,
            )?)
        };
        let attend = |i: usize| attend_heads(i, None);
        // Queries [first, end) in one kernel call: K/V loads are shared by
        // several positions and the heads of a GQA group (same values as one
        // call per query).
        let attend_block = |first: usize, end: usize| -> Result<Vec<Vec<f32>>, Error> {
            let masks = match mode {
                LayerMode::Encoder => Some(
                    (first..end)
                        .map(|i| {
                            encoder_mask(
                                stage.first_key,
                                count,
                                stage.first_query + i,
                                (g.kind == AttentionKind::Sliding)
                                    .then_some(self.model.spec.sliding_window),
                            )
                        })
                        .collect::<Result<Vec<_>, Error>>()?,
                ),
                LayerMode::Decoder => None,
            };
            let queries: Vec<&[f32]> = stage.queries[first..end]
                .iter()
                .map(Vec::as_slice)
                .collect();
            let masks: Option<Vec<&[bool]>> = masks
                .as_ref()
                .map(|m| m.iter().map(Vec::as_slice).collect());
            Ok(self.active_kernels().attention_multi(
                &queries,
                stage.keys,
                stage.values,
                g.kv_heads,
                g.head_dim,
                masks.as_deref(),
            )?)
        };
        // Batched: queries are independent, so workers split them into
        // contiguous ranges; each query runs the identical computation.
        let core_span = crate::engine::stats::Span::new(crate::engine::stats::Stage::AttentionCore);
        let attended_batch: Vec<Vec<f32>> = match (self.batched, self.pool()) {
            (false, _) => Vec::new(),
            // Fewer queries than workers (one-token decode): each query's heads
            // are split into ranges too; every head is computed as in a whole
            // call, and the ranges are concatenated in head order.
            (true, Some(workers)) if stage.queries.len() < workers.count() => {
                let queries = stage.queries.len();
                let split = (workers.count() / queries).clamp(1, g.query_heads);
                let per = g.query_heads.div_ceil(split);
                let items: Vec<(usize, std::ops::Range<usize>)> = (0..queries)
                    .flat_map(|i| {
                        (0..g.query_heads)
                            .step_by(per)
                            .map(move |a| (i, a..(a + per).min(g.query_heads)))
                    })
                    .collect();
                let parts = self.compute().map_positions(items.len(), |j| {
                    let (i, heads) = items[j].clone();
                    attend_heads(i, Some(heads))
                })?;
                let mut out: Vec<Vec<f32>> = (0..queries)
                    .map(|_| Vec::with_capacity(g.query_heads * g.head_dim))
                    .collect();
                for ((i, _), part) in items.iter().zip(parts) {
                    out[*i].extend(part);
                }
                out
            }
            (true, Some(workers)) if stage.queries.len() > 1 => {
                // Queries [a, b) over `pool`, in blocks of 8 per worker range.
                let over = |pool: &crate::engine::execution::Workers, a: usize, b: usize| {
                    pool.ranges(b - a, |first, end| {
                        let mut rows = Vec::with_capacity(end - first);
                        for block in (a + first..a + end).step_by(8) {
                            rows.extend(attend_block(block, (block + 8).min(a + end))?);
                        }
                        Ok::<_, Error>(rows)
                    })
                    .map(|parts| parts.into_iter().flatten().collect::<Vec<_>>())
                };
                let n = stage.queries.len();
                match self.extra.as_ref().filter(|_| {
                    crate::engine::execution::replica() == 0
                        && n >= 64
                        && workers.count() == workers.cpus().len()
                }) {
                    // The higher SMT threads join (attention gains from 4
                    // threads per POWER9 core; dense projections do not).
                    // Each core takes one contiguous query range, split
                    // among its threads, so the threads sharing a core's
                    // caches walk the same keys and values together.
                    Some(extra) => {
                        let ranges =
                            crate::engine::execution::core_local_ranges(&[workers, extra], n);
                        let run = |pool: &crate::engine::execution::Workers,
                                   ranges: Vec<(usize, usize)>| {
                            pool.tasks(ranges, |(a, b)| {
                                let mut rows = Vec::with_capacity(b - a);
                                for block in (a..b).step_by(8) {
                                    rows.extend(attend_block(block, (block + 8).min(b))?);
                                }
                                Ok::<_, Error>((a, rows))
                            })
                        };
                        let mut ranges = ranges.into_iter();
                        let (mine, theirs) = (
                            ranges.next().unwrap_or_default(),
                            ranges.next().unwrap_or_default(),
                        );
                        let (mine, theirs) = std::thread::scope(|s| {
                            let other = s.spawn(|| run(extra, theirs));
                            let mine = run(workers, mine);
                            (
                                mine,
                                other
                                    .join()
                                    .unwrap_or_else(|e| std::panic::resume_unwind(e)),
                            )
                        });
                        let mut parts: Vec<(usize, Vec<Vec<f32>>)> = mine?;
                        parts.extend(theirs?);
                        parts.sort_by_key(|p| p.0);
                        parts.into_iter().flat_map(|p| p.1).collect()
                    }
                    None => over(workers, 0, n)?,
                }
            }
            (true, _) => (0..stage.queries.len())
                .map(attend)
                .collect::<Result<_, _>>()?,
        };
        drop(core_span);
        let rowwise = if self.batched { 0 } else { stage.queries.len() };
        for (i, row) in input.iter().enumerate().take(rowwise) {
            let attended = attend(i)?;
            let projected = operators::project_streamed(
                self.file,
                self.model.layers[self.layer].attention_output,
                &attended,
            )?;
            let normalized = math::rms_norm(
                &projected,
                Some(&self.post_scale),
                self.model.spec.rms_epsilon,
            )?;
            let residual = math::add(row, &normalized)?;
            let result = operators::feed_forward(
                self.file,
                &self.model.layers[self.layer],
                &self.model.spec,
                &residual,
                mode,
            )?;
            output.push(result.hidden);
            routing.push(result.routing);
        }
        if self.batched {
            let output_span =
                crate::engine::stats::Span::new(crate::engine::stats::Stage::AttentionOutput);
            let projected = operators::project_streamed_batch_with(
                self.file,
                self.model.layers[self.layer].attention_output,
                &attended_batch,
                operators::MAX_REFERENCE_MATRIX_ELEMENTS,
                self.compute(),
            )?;
            let residual = self.compute().map_positions(input.len(), |i| {
                let normalized = math::rms_norm(
                    &projected[i],
                    Some(&self.post_scale),
                    self.model.spec.rms_epsilon,
                )?;
                Ok::<_, Error>(math::add(&input[i], &normalized)?)
            })?;
            drop(output_span);
            for f in operators::feed_forward_layer(
                self.file,
                &self.model.layers[self.layer],
                &self.model.spec,
                &residual,
                mode,
                per_layer,
                self.compute(),
            )? {
                output.push(f.hidden);
                routing.push(f.routing);
            }
        }
        Ok((output, routing))
    }
    /// Causal encoder chunk. On any error this layer's cache is unchanged.
    pub fn encode(
        &self,
        input: &[Vec<f32>],
        cache: &mut LayerCache<'a>,
    ) -> Result<Vec<Vec<f32>>, Error> {
        let (output, proposed) = self.prepare_encode(input, cache)?;
        *cache = proposed;
        Ok(output)
    }
    /// Return an uncommitted replacement so the full-model driver can commit
    /// all layers only after the entire encoder and final normalization succeed.
    pub(crate) fn prepare_encode(
        &self,
        input: &[Vec<f32>],
        cache: &LayerCache<'a>,
    ) -> Result<(Vec<Vec<f32>>, LayerCache<'a>), Error> {
        self.prepare_encode_routed(input, cache)
            .map(|(output, proposed, _)| (output, proposed))
    }
    /// As `prepare_encode`, also returning each position's router decision.
    #[allow(clippy::type_complexity)]
    pub(crate) fn prepare_encode_routed(
        &self,
        input: &[Vec<f32>],
        cache: &LayerCache<'a>,
    ) -> Result<(Vec<Vec<f32>>, LayerCache<'a>, Vec<operators::Routing>), Error> {
        let mut proposed = LayerCache {
            owner: cache.owner,
            layer: cache.layer,
            geometry: cache.geometry.clone(),
            context: cache.context,
            window: cache.window,
            max_bytes: cache.max_bytes,
            total: cache.total,
            start: 0,
            keys: cache.keys().to_vec(),
            values: cache.values().to_vec(),
        };
        let (output, routing, _) = self.encode_in_place(input, &mut proposed, None)?;
        proposed.commit();
        Ok((output, proposed, routing))
    }
    /// Causal encoder chunk in place: this pass's K/V are appended to the
    /// cache and attention reads the retained cache plus them directly (no
    /// copy). Window eviction is deferred to [`LayerCache::commit`] so that
    /// shared-K/V layers of the same pass see the untruncated K/V; on error
    /// the cache is unchanged, otherwise the caller commits or rolls back.
    #[allow(clippy::type_complexity)]
    pub(crate) fn encode_in_place(
        &self,
        input: &[Vec<f32>],
        cache: &mut LayerCache<'a>,
        per_layer: Option<&[Vec<f32>]>,
    ) -> Result<(Vec<Vec<f32>>, Vec<operators::Routing>, PendingKv), Error> {
        let (next_position, cells) = self.check_stage(input, cache, LayerMode::Encoder)?;
        let _prefill = self.prefill_scope(input.len());
        let pending = PendingKv {
            old_len: cache.keys.len(),
            old_total: cache.total,
        };
        let first_key = cache.total - cache.retained_tokens();
        let mut run = || -> Result<_, Error> {
            let extra = cells - (cache.keys.len() - cache.start);
            cache
                .keys
                .try_reserve(extra)
                .map_err(|_| Error::ResourceLimit)?;
            cache
                .values
                .try_reserve(extra)
                .map_err(|_| Error::ResourceLimit)?;
            let split = self
                .split
                .as_ref()
                .filter(|_| self.batched)
                .and_then(|s| Some((s, s.first_part(input.len())?)));
            let Some((split, h)) = split else {
                let queries =
                    self.project_tokens(input, cache.total, &mut cache.keys, &mut cache.values)?;
                let view = StageRef {
                    queries: &queries,
                    keys: &cache.keys[cache.start..],
                    values: &cache.values[cache.start..],
                    first_key,
                    first_query: cache.total,
                };
                return self.finish(input, view, LayerMode::Encoder, per_layer);
            };
            // Positions [0, h) on this socket, [h, n) on the second one (its
            // own workers and weight replica). K/V are appended in position
            // order, then each socket attends and finishes its own positions.
            let total = cache.total;
            let (a, b) = input.split_at(h);
            let (keys, values) = (&mut cache.keys, &mut cache.values);
            let (mine, (theirs, b_time)) = std::thread::scope(|s| {
                let other = s.spawn(|| {
                    // Best effort: an unconfined driver is only slower.
                    let _ = split.workers.confine();
                    crate::engine::execution::with_replica(1, || {
                        let start = std::time::Instant::now();
                        let (mut k, mut v) = (Vec::new(), Vec::new());
                        let q = self.project_tokens(b, total + h, &mut k, &mut v);
                        (q.map(|q| (q, k, v)), start.elapsed().as_secs_f64())
                    })
                });
                let start = std::time::Instant::now();
                let mine = self.project_tokens(a, total, keys, values);
                let a_time = start.elapsed().as_secs_f64();
                let theirs = other
                    .join()
                    .unwrap_or_else(|e| std::panic::resume_unwind(e));
                ((mine, a_time), theirs)
            });
            let (mine, mut a_time) = mine;
            let mut queries = mine?;
            let (their_queries, their_keys, their_values) = theirs?;
            let mut b_time = b_time;
            cache.keys.extend(their_keys);
            cache.values.extend(their_values);
            queries.extend(their_queries);
            let keys = &cache.keys[cache.start..];
            let values = &cache.values[cache.start..];
            let (per_a, per_b) = match per_layer {
                Some(p) => {
                    let (x, y) = p.split_at(h);
                    (Some(x), Some(y))
                }
                None => (None, None),
            };
            let view_a = StageRef {
                queries: &queries[..h],
                keys,
                values,
                first_key,
                first_query: total,
            };
            let view_b = StageRef {
                queries: &queries[h..],
                keys,
                values,
                first_key,
                first_query: total + h,
            };
            let (mine, theirs) = std::thread::scope(|s| {
                let other = s.spawn(|| {
                    let _ = split.workers.confine();
                    crate::engine::execution::with_replica(1, || {
                        let start = std::time::Instant::now();
                        let r = self.finish(b, view_b, LayerMode::Encoder, per_b);
                        (r, start.elapsed().as_secs_f64())
                    })
                });
                let start = std::time::Instant::now();
                let mine = self.finish(a, view_a, LayerMode::Encoder, per_a);
                let elapsed = start.elapsed().as_secs_f64();
                let theirs = other
                    .join()
                    .unwrap_or_else(|e| std::panic::resume_unwind(e));
                ((mine, elapsed), theirs)
            });
            a_time += mine.1;
            b_time += theirs.1;
            split.observe((h, a_time), (input.len() - h, b_time));
            let (mut output, mut routing) = mine.0?;
            let (their_output, their_routing) = theirs.0?;
            output.extend(their_output);
            routing.extend(their_routing);
            Ok((output, routing))
        };
        match run() {
            Ok((output, routing)) => {
                cache.total = next_position;
                Ok((output, routing, pending))
            }
            Err(error) => {
                cache.rollback(&pending);
                Err(error)
            }
        }
    }
    /// Causal encoder chunk of a shared-K/V layer: its own queries over the
    /// source layer's K/V of this pass (HF `shared_kv_states`: the retained
    /// cache plus this pass, before window truncation), with this layer's
    /// mask. Its own cache stores no K/V; only the position advances.
    #[allow(clippy::type_complexity)]
    pub(crate) fn encode_shared_in_place(
        &self,
        input: &[Vec<f32>],
        cache: &mut LayerCache<'a>,
        source: &LayerCache<'a>,
        source_pending: &PendingKv,
        per_layer: Option<&[Vec<f32>]>,
    ) -> Result<(Vec<Vec<f32>>, Vec<operators::Routing>, PendingKv), Error> {
        let spec = &self.model.spec;
        if !self.batched
            || cache.total != source_pending.old_total
            || source.total != cache.total + input.len()
            || cache.geometry != self.transform.geometry
            || source.geometry != self.transform.geometry
        {
            return Err(Error::Shape("shared-K/V source does not match this layer"));
        }
        if input.is_empty() || input.iter().any(|x| x.len() != spec.hidden) {
            return Err(Error::Shape("hidden-state rows"));
        }
        let _prefill = self.prefill_scope(input.len());
        let layer = &self.model.layers[self.layer];
        let queries = {
            let _span =
                crate::engine::stats::Span::new(crate::engine::stats::Stage::AttentionProjections);
            let compute = self.compute();
            let normalized = compute.map_positions(input.len(), |i| {
                Ok::<_, Error>(math::rms_norm(
                    &input[i],
                    Some(&self.pre_scale),
                    spec.rms_epsilon,
                )?)
            })?;
            let query = operators::project_streamed_batch_with(
                self.file,
                layer.query,
                &normalized,
                operators::MAX_REFERENCE_MATRIX_ELEMENTS,
                compute,
            )?;
            compute.map_positions(input.len(), |i| {
                self.transform.prepare_query(&query[i], cache.total + i)
            })?
        };
        let (_, width) = widths(&cache.geometry)?;
        let view = StageRef {
            queries: &queries,
            keys: &source.keys[source.start..],
            values: &source.values[source.start..],
            first_key: source_pending.old_total - (source_pending.old_len - source.start) / width,
            first_query: cache.total,
        };
        let (output, routing) = self.finish(input, view, LayerMode::Encoder, per_layer)?;
        let pending = PendingKv {
            old_len: cache.keys.len(),
            old_total: cache.total,
        };
        cache.total = source.total;
        Ok((output, routing, pending))
    }
    /// Bidirectional decoder chunk. Cache is borrowed immutably, including on
    /// failure. Caller supplies decoder hidden inputs AFTER self-conditioning.
    pub fn decode(
        &self,
        input: &[Vec<f32>],
        cache: &LayerCache<'a>,
    ) -> Result<Vec<Vec<f32>>, Error> {
        self.decode_routed(input, cache).map(|(output, _)| output)
    }
    /// As `decode`, also returning each position's router decision.
    pub(crate) fn decode_routed(
        &self,
        input: &[Vec<f32>],
        cache: &LayerCache<'a>,
    ) -> Result<(Vec<Vec<f32>>, Vec<operators::Routing>), Error> {
        let stage = self.stage(input, cache, LayerMode::Decoder)?;
        self.finish(input, stage.view(), LayerMode::Decoder, None)
    }
}

fn retained_count(count: usize, window: Option<usize>) -> usize {
    window.map_or(count, |w| count.min(w.saturating_sub(1)))
}

fn encoder_mask(
    first_key: usize,
    count: usize,
    query: usize,
    window: Option<usize>,
) -> Result<Vec<bool>, Error> {
    if count == 0 || window == Some(0) || first_key.checked_add(count).is_none() {
        return Err(Error::Shape("encoder mask dimensions"));
    }
    Ok((first_key..first_key + count)
        .map(|k| k <= query && window.is_none_or(|w| query - k < w))
        .collect())
}

#[cfg(test)]
pub(crate) mod tests;
