//! Complete reference text encoder/decoder for the bound DiffusionGemma model.
//! F32 activations, F64 reductions, streamed GGUF projections. This is a slow
//! correctness path, not independently validated real-model generation or an
//! optimized server. Inputs are token IDs; every position is active (no padding masks,
//! batching or multimodal tensors). Tokenization/chat preparation are separate modules.

use crate::engine::{
    attention::{LayerCache, ReferenceBlock},
    model::ModelDefinition,
    operators::{self, Error, GatedMlp, SelfConditioner},
    weights::ModelFile,
};
use crate::kernels::reference as math;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReferenceLimits {
    /// Each staged K+V pair, not a process-wide bound. Old/new caches may coexist.
    pub cache_bytes_per_layer: usize,
    /// Input/output hidden-state cells per call, excluding intermediate scratch.
    pub max_hidden_elements: usize,
    /// One flat position-major logits buffer, excluding retained sampler copies.
    pub max_logit_elements: usize,
}
impl ReferenceLimits {
    /// Default limits with a per-layer K/V budget large enough for `context`
    /// tokens in every layer (an upper bound: sliding layers retain at most a
    /// window). This is a bound, not a reservation: caches grow with use.
    pub fn for_context(spec: &crate::engine::model::Specification, context: usize) -> Self {
        let widest = spec
            .attention
            .iter()
            .map(|g| g.kv_heads.saturating_mul(g.head_dim))
            .max()
            .unwrap_or(0);
        let defaults = Self::default();
        Self {
            cache_bytes_per_layer: widest
                .saturating_mul(context)
                .saturating_mul(2 * size_of::<f32>())
                .max(defaults.cache_bytes_per_layer),
            ..defaults
        }
    }
}
impl Default for ReferenceLimits {
    fn default() -> Self {
        Self {
            cache_bytes_per_layer: 64 * 1024 * 1024,
            max_hidden_elements: 16 * 1024 * 1024,
            max_logit_elements: 256 * 262144, // one target canvas: 256 MiB F32
        }
    }
}

/// Diagnostic snapshot points. Observers borrow rows only for the callback;
/// no additional retained activation buffers are required by normal execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TracePoint {
    EncoderEmbedding,
    EncoderLayer(usize),
    EncoderFinal,
    DecoderEmbedding,
    DecoderSoftEmbedding,
    DecoderInput,
    DecoderLayer(usize),
    DecoderFinal,
    /// Router decisions of encoder/decoder layer `i`, one row per position:
    /// `[top-k expert IDs as F32 (exact), top-k contribution weights, all
    /// router probabilities]`, top-k in router order (descending probability,
    /// ties toward the lower ID). Diagnostic only.
    EncoderRouting(usize),
    DecoderRouting(usize),
}
impl TracePoint {
    pub fn name(self) -> String {
        match self {
            Self::EncoderEmbedding => "encoder.embedding".into(),
            Self::EncoderLayer(i) => format!("encoder.layer.{i}"),
            Self::EncoderFinal => "encoder.final".into(),
            Self::DecoderEmbedding => "decoder.embedding".into(),
            Self::DecoderSoftEmbedding => "decoder.soft_embedding".into(),
            Self::DecoderInput => "decoder.input".into(),
            Self::DecoderLayer(i) => format!("decoder.layer.{i}"),
            Self::DecoderFinal => "decoder.final".into(),
            Self::EncoderRouting(i) => format!("encoder.routing.{i}"),
            Self::DecoderRouting(i) => format!("decoder.routing.{i}"),
        }
    }
    pub fn is_routing(self) -> bool {
        matches!(self, Self::EncoderRouting(_) | Self::DecoderRouting(_))
    }
}

/// Router decisions as trace rows (see [`TracePoint::EncoderRouting`]).
fn routing_rows(routing: &[operators::Routing]) -> Vec<Vec<f32>> {
    routing
        .iter()
        .map(|r| {
            let mut row: Vec<f32> = r.selected.iter().map(|s| s.expert as f32).collect();
            row.extend(r.selected.iter().map(|s| s.weight));
            row.extend_from_slice(&r.probabilities);
            row
        })
        .collect()
}
#[derive(Debug)]
pub enum TraceError<E> {
    Compute(Error),
    Observer(E),
}
impl<E> From<Error> for TraceError<E> {
    fn from(e: Error) -> Self {
        Self::Compute(e)
    }
}
impl<E: std::fmt::Display> std::fmt::Display for TraceError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compute(e) => write!(f, "{e}"),
            Self::Observer(e) => write!(f, "trace observer: {e}"),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for TraceError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::Compute(e) => e,
            Self::Observer(e) => e,
        })
    }
}
impl TraceError<std::convert::Infallible> {
    fn compute(self) -> Error {
        match self {
            Self::Compute(e) => e,
            Self::Observer(e) => match e {},
        }
    }
}

pub struct EncoderCache<'a> {
    file: &'a ModelFile,
    model: &'a ModelDefinition<'a>,
    limits: ReferenceLimits,
    layers: Vec<LayerCache<'a>>,
}
impl EncoderCache<'_> {
    pub fn total_tokens(&self) -> usize {
        self.layers.first().map_or(0, LayerCache::total_tokens)
    }
    pub fn layer(&self, index: usize) -> Option<&LayerCache<'_>> {
        self.layers.get(index)
    }
}

pub struct ReferenceText<'a> {
    file: &'a ModelFile,
    model: &'a ModelDefinition<'a>,
    limits: ReferenceLimits,
    blocks: Vec<ReferenceBlock<'a>>,
    output_scale: Vec<f32>,
    self_conditioner: Option<SelfConditioner>,
    self_conditioning_scale: Vec<f32>,
    batched: bool,
    workers: Option<std::sync::Arc<crate::engine::execution::Workers>>,
    split: Option<std::sync::Arc<crate::engine::execution::Split>>,
    kernels: crate::engine::execution::MatrixKernels,
}
impl<'a> ReferenceText<'a> {
    pub fn new(
        file: &'a ModelFile,
        model: &'a ModelDefinition<'a>,
        limits: ReferenceLimits,
    ) -> Result<Self, Error> {
        Self::initialize(
            file,
            model,
            limits,
            false,
            None,
            crate::engine::execution::MatrixKernels::portable(),
        )
    }
    fn initialize(
        file: &'a ModelFile,
        model: &'a ModelDefinition<'a>,
        limits: ReferenceLimits,
        batched: bool,
        workers: Option<std::sync::Arc<crate::engine::execution::Workers>>,
        kernels: crate::engine::execution::MatrixKernels,
    ) -> Result<Self, Error> {
        if model.layers.is_empty()
            || model.layers.len() != model.spec.attention.len()
            || limits.cache_bytes_per_layer == 0
            || limits.max_hidden_elements == 0
            || limits.max_logit_elements == 0
        {
            return Err(Error::Parameter("reference model/layer limits"));
        }
        let output_scale = operators::load_vector(file, model.output_norm)?;
        if output_scale.len() != model.spec.hidden {
            return Err(Error::Shape("model output norm"));
        }
        let mut blocks = (0..model.layers.len())
            .map(|i| ReferenceBlock::new(file, model, i))
            .collect::<Result<Vec<_>, _>>()?;
        if batched {
            for block in &mut blocks {
                block.enable_batching(workers.clone(), kernels);
            }
        }
        // Diffusion models only; autoregressive models never run the decoder.
        let self_conditioning_scale = match model.self_conditioning_pre_norm {
            Some(norm) => operators::load_vector(file, norm)?,
            None => vec![0.0; model.spec.hidden],
        };
        if self_conditioning_scale.len() != model.spec.hidden {
            return Err(Error::Shape("self-conditioning pre-norm width"));
        }
        let self_conditioner = match (&model.self_conditioning_mlp, batched) {
            (Some(mlp), false) => Some(SelfConditioner::new(
                self_conditioning_scale.clone(),
                GatedMlp::load(file, mlp)?,
                model.spec.rms_epsilon,
            )?),
            _ => None,
        };
        Ok(Self {
            file,
            model,
            limits,
            blocks,
            output_scale,
            self_conditioner,
            self_conditioning_scale,
            batched,
            workers,
            split: None,
            kernels,
        })
    }
    /// Stream encoded attention/vocabulary/shared/grouped-expert row tiles
    /// into bounded C decode scratch and batched F64 projections. No full
    /// expert/matrix expansion, activation quantization or inference threads.
    /// Self-conditioning MLP is streamed too; only the small router retains
    /// its reference matrix. Matrix weight validation occurs when consumed.
    pub fn new_batched(
        file: &'a ModelFile,
        model: &'a ModelDefinition<'a>,
        limits: ReferenceLimits,
    ) -> Result<Self, Error> {
        Self::initialize(
            file,
            model,
            limits,
            true,
            None,
            crate::engine::execution::MatrixKernels::portable(),
        )
    }
    /// Exact portable kernels on physical-core workers (bounded by the affinity mask) selected
    /// from the caller's affinity. Only child workers are pinned. Memory-node
    /// policy remains inherited (use numactl); this is not a persistent pool.
    pub fn new_parallel(
        file: &'a ModelFile,
        model: &'a ModelDefinition<'a>,
        limits: ReferenceLimits,
        count: usize,
    ) -> Result<Self, Error> {
        Self::new_with_backend(
            file,
            model,
            limits,
            count,
            crate::engine::execution::MatrixBackend::Portable,
        )
    }
    /// Batched encoded execution with `count` (1..=available distinct cores) scoped workers and an
    /// explicit matrix backend, resolved ONCE here: `Portable`, `Auto`
    /// (POWER9 explicit-FMA when compiled in AND detected, else portable) or
    /// forced `Power9Fma` (fails with `MatrixBackendUnavailable` otherwise).
    /// All backends produce bit-identical results (docs/backends.md).
    pub fn new_with_backend(
        file: &'a ModelFile,
        model: &'a ModelDefinition<'a>,
        limits: ReferenceLimits,
        count: usize,
        backend: crate::engine::execution::MatrixBackend,
    ) -> Result<Self, Error> {
        let kernels = crate::engine::execution::matrix_kernels(backend)?;
        // Beyond two threads per core, the extra SMT threads only join
        // prompt attention (dense projections run slower on 4 per core).
        let (workers, extra) = crate::engine::execution::Workers::tiered(count, 2)?;
        let workers = if count == 1 {
            None
        } else {
            Some(std::sync::Arc::new(workers))
        };
        let mut runtime = Self::initialize(file, model, limits, true, workers, kernels)?;
        if let (Some(_), Some(extra)) = (&runtime.workers, extra) {
            let extra = std::sync::Arc::new(extra);
            for block in &mut runtime.blocks {
                block.set_attention_extra(Some(extra.clone()));
            }
        }
        runtime.prepack()?;
        if let Some(workers) = &runtime.workers {
            workers.place_submitter()?;
        }
        Ok(runtime)
    }
    /// As [`Self::new_with_backend`], with the `count` workers split by CPU
    /// package when they span two: decoding uses the first package's
    /// workers; prompt chunks split their positions between both packages,
    /// each with its own copy of the packed weights in its own memory (run
    /// without a memory-node binding so each copy is placed locally).
    /// Results are identical to the one-package runtime.
    pub fn new_split_sockets(
        file: &'a ModelFile,
        model: &'a ModelDefinition<'a>,
        limits: ReferenceLimits,
        count: usize,
        backend: crate::engine::execution::MatrixBackend,
    ) -> Result<Self, Error> {
        let mut sets = crate::engine::execution::Workers::per_package(count)?;
        if sets.len() != 2 {
            return Self::new_with_backend(file, model, limits, count, backend);
        }
        let second = sets.pop().ok_or(Error::Shape("package workers"))?;
        let first = sets.pop().ok_or(Error::Shape("package workers"))?;
        // The calling thread drives the first package's workers (and decodes).
        first.place_submitter()?;
        Self::split_with(file, model, limits, first, second, backend)
    }
    /// Split runtime over explicit worker sets (`second` on weight replica 1).
    pub(crate) fn split_with(
        file: &'a ModelFile,
        model: &'a ModelDefinition<'a>,
        limits: ReferenceLimits,
        first: crate::engine::execution::Workers,
        second: crate::engine::execution::Workers,
        backend: crate::engine::execution::MatrixBackend,
    ) -> Result<Self, Error> {
        let kernels = crate::engine::execution::matrix_kernels(backend)?;
        let split =
            std::sync::Arc::new(crate::engine::execution::Split::new(second, first.count()));
        let mut runtime = Self::initialize(
            file,
            model,
            limits,
            true,
            Some(std::sync::Arc::new(first)),
            kernels,
        )?;
        for block in &mut runtime.blocks {
            block.set_split(Some(split.clone()));
        }
        runtime.prepack()?;
        // The second package packs its own replica (first touch places it in
        // that package's memory).
        std::thread::scope(|s| {
            s.spawn(|| {
                let _ = split.workers.confine();
                crate::engine::execution::with_replica(1, || {
                    let start = std::time::Instant::now();
                    operators::prepack(
                        runtime.file,
                        &runtime.packed_tensors(),
                        crate::engine::execution::Compute {
                            workers: Some(&split.workers),
                            kernels,
                        },
                    )?;
                    eprintln!(
                        "Packed the second socket's weights in {:.1}s.",
                        start.elapsed().as_secs_f64()
                    );
                    Ok::<_, Error>(())
                })
            })
            .join()
            .unwrap_or_else(|e| std::panic::resume_unwind(e))
        })?;
        runtime.split = Some(split);
        Ok(runtime)
    }
    /// The second socket's share of prompt positions, when split.
    pub fn split_share(&self) -> Option<f64> {
        self.split.as_ref().map(|s| 1.0 - s.share())
    }
    /// Pack every projected matrix up front (packing backends only).
    /// Lookup-only tables (E2B's per-layer token embedding) are not packed.
    fn prepack(&self) -> Result<(), Error> {
        let tensors = self.packed_tensors();
        let start = std::time::Instant::now();
        operators::prepack(self.file, &tensors, self.compute())?;
        if self.workers.is_some() {
            eprintln!("Packed weights in {:.1}s.", start.elapsed().as_secs_f64());
        }
        Ok(())
    }
    fn packed_tensors(&self) -> Vec<&'a crate::gguf::TensorInfo> {
        let model = self.model;
        let mut tensors = vec![model.embedding];
        if let Some(mlp) = &model.self_conditioning_mlp {
            tensors.extend([mlp.gate, mlp.up, mlp.down]);
        }
        if let Some(ple) = &model.per_layer_input {
            tensors.push(ple.model_projection);
        }
        for layer in &model.layers {
            tensors.extend([layer.query, layer.attention_output]);
            tensors.extend(layer.key);
            tensors.extend(layer.value);
            tensors.extend([
                layer.shared_mlp.gate,
                layer.shared_mlp.up,
                layer.shared_mlp.down,
            ]);
            if let Some(moe) = &layer.moe {
                tensors.extend([moe.expert_gate_up, moe.expert_down]);
            }
            if let Some(ple) = &layer.per_layer {
                tensors.extend([ple.gate, ple.projection]);
            }
        }
        tensors
    }
    /// Use `backend` for multi-token encoder chunks (prompt prefill) only;
    /// single-token appends and decoder passes keep the main backend.
    /// Prefill may not drop the residual plane of a two-plane runtime:
    /// that changes prompt K/V and can cascade through expert routing.
    pub fn set_prefill_backend(
        &mut self,
        backend: Option<crate::engine::execution::MatrixBackend>,
    ) -> Result<(), Error> {
        let kernels = backend
            .map(crate::engine::execution::matrix_kernels)
            .transpose()?;
        if self.kernels.planes() == 2
            && kernels.is_some_and(|kernels| kernels.approximate() && kernels.planes() < 2)
        {
            return Err(Error::Parameter(
                "single-plane prefill loses residual precision; use int8x2 prefill or omit --prefill-backend",
            ));
        }
        for block in &mut self.blocks {
            block.set_prefill_kernels(kernels);
        }
        Ok(())
    }
    /// The matrix kernels actually in use: `"portable"` or `"power9-fma"`.
    pub fn matrix_backend(&self) -> &'static str {
        self.kernels.name()
    }
    fn compute(&self) -> crate::engine::execution::Compute<'_> {
        crate::engine::execution::Compute {
            workers: self.workers.as_deref(),
            kernels: self.kernels,
        }
    }
    pub fn worker_cpus(&self) -> Option<&[usize]> {
        self.workers.as_ref().map(|w| w.cpus())
    }
    pub fn specification(&self) -> &crate::engine::model::Specification {
        &self.model.spec
    }

    /// Conservative admission for a complete request: full canvases even when
    /// the requested visible tail is shorter. No silent global-cache truncation.
    pub(crate) fn generation_capacity(
        &self,
        prompt: usize,
        canvas: usize,
        canvases: usize,
        encoder_chunk: usize,
    ) -> Result<(), Error> {
        let spec = &self.model.spec;
        let total = canvas
            .checked_mul(canvases)
            .and_then(|n| prompt.checked_add(n))
            .filter(|&n| n <= spec.context)
            .ok_or(Error::ResourceLimit)?;
        if canvases == 0 {
            return Ok(());
        }
        self.check_logits(canvas)?;
        let chunk = encoder_chunk.min(prompt.max(canvas));
        if canvas
            .max(chunk)
            .checked_mul(spec.hidden)
            .filter(|&n| n <= self.limits.max_hidden_elements)
            .is_none()
        {
            return Err(Error::ResourceLimit);
        }
        for g in &spec.attention {
            let tokens = match g.kind {
                crate::engine::model::AttentionKind::Global => total,
                crate::engine::model::AttentionKind::Sliding => total.min(
                    spec.sliding_window
                        .saturating_sub(1)
                        .checked_add(canvas.max(chunk))
                        .ok_or(Error::ResourceLimit)?,
                ),
            };
            if g.kv_heads
                .checked_mul(g.head_dim)
                .and_then(|w| w.checked_mul(tokens))
                .and_then(|n| n.checked_mul(2 * size_of::<f32>()))
                .filter(|&n| n <= self.limits.cache_bytes_per_layer)
                .is_none()
            {
                return Err(Error::ResourceLimit);
            }
        }
        Ok(())
    }

    pub fn new_cache(&self) -> Result<EncoderCache<'a>, Error> {
        let layers = self
            .blocks
            .iter()
            .map(|b| b.new_cache(self.limits.cache_bytes_per_layer))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(EncoderCache {
            file: self.file,
            model: self.model,
            limits: self.limits,
            layers,
        })
    }
    fn check_cache(&self, cache: &EncoderCache<'a>) -> Result<(), Error> {
        if !std::ptr::eq(cache.file, self.file)
            || !std::ptr::eq(cache.model, self.model)
            || cache.limits != self.limits
            || cache.layers.len() != self.blocks.len()
            || cache
                .layers
                .iter()
                .any(|c| c.total_tokens() != cache.total_tokens())
        {
            return Err(Error::Shape("encoder cache owner/limits/layer positions"));
        }
        Ok(())
    }
    fn check_tokens(
        &self,
        tokens: &[u32],
        cache: &EncoderCache<'a>,
        decoder: bool,
    ) -> Result<(), Error> {
        self.check_cache(cache)?;
        if tokens.is_empty()
            || tokens
                .iter()
                .any(|&t| t as usize >= self.model.spec.vocabulary)
            || (decoder && (tokens.len() > self.model.spec.canvas || cache.total_tokens() == 0))
        {
            return Err(Error::Shape("token IDs/canvas dimensions"));
        }
        if tokens
            .len()
            .checked_mul(self.model.spec.hidden)
            .filter(|&n| n <= self.limits.max_hidden_elements)
            .is_none()
            || cache
                .total_tokens()
                .checked_add(tokens.len())
                .filter(|&n| n <= self.model.spec.context)
                .is_none()
        {
            return Err(Error::ResourceLimit);
        }
        Ok(())
    }
    fn check_logits(&self, positions: usize) -> Result<usize, Error> {
        positions
            .checked_mul(self.model.spec.vocabulary)
            .filter(|&n| n > 0 && n <= self.limits.max_logit_elements)
            .ok_or(Error::ResourceLimit)
    }
    fn embeddings(&self, tokens: &[u32]) -> Result<Vec<Vec<f32>>, Error> {
        let factor = (self.model.spec.hidden as f32).sqrt();
        let ones = vec![1.0; self.model.spec.hidden];
        tokens
            .iter()
            .map(|&id| {
                let raw = self
                    .file
                    .read_row(&self.model.embedding.name, u64::from(id))?
                    .decode()?;
                if raw.len() != self.model.spec.hidden {
                    return Err(Error::Shape("embedding width"));
                }
                Ok(math::multiply_scaled(&raw, &ones, factor)?)
            })
            .collect()
    }
    /// Per-layer inputs `[layer][position]` for models with per-layer
    /// embeddings (Gemma4TextModel.get_per_layer_inputs and
    /// project_per_layer_inputs): per layer slice,
    /// `(RMSNorm(projection(embeds) / sqrt(hidden)) + table[id] * sqrt(width)) / sqrt(2)`.
    #[allow(clippy::type_complexity)]
    fn per_layer_inputs(
        &self,
        tokens: &[u32],
        embeds: &[Vec<f32>],
    ) -> Result<Option<Vec<Vec<Vec<f32>>>>, Error> {
        let Some(b) = &self.model.per_layer_input else {
            return Ok(None);
        };
        if !self.batched {
            return Err(Error::Shape("per-layer inputs need batched execution"));
        }
        let spec = &self.model.spec;
        let width = spec.per_layer_input;
        let layers = self.model.layers.len();
        let projected = operators::project_streamed_batch_with(
            self.file,
            b.model_projection,
            embeds,
            operators::MAX_REFERENCE_MATRIX_ELEMENTS,
            self.compute(),
        )?;
        let norm = operators::load_vector(self.file, b.projection_norm)?;
        // Python float scalars applied to binary32 tensors.
        let projection_scale = crate::libm::pow(spec.hidden as f64, -0.5) as f32;
        let token_scale = (width as f64).sqrt() as f32;
        let combine_scale = std::f64::consts::FRAC_1_SQRT_2 as f32;
        let ones = vec![1.0; width];
        let mut out: Vec<Vec<Vec<f32>>> = (0..layers)
            .map(|_| Vec::with_capacity(tokens.len()))
            .collect();
        for (i, &id) in tokens.iter().enumerate() {
            let row = self
                .file
                .read_row(&b.token_embedding.name, u64::from(id))?
                .decode()?;
            if row.len() != layers * width || projected[i].len() != layers * width {
                return Err(Error::Shape("per-layer embedding width"));
            }
            for (l, slot) in out.iter_mut().enumerate() {
                let range = l * width..(l + 1) * width;
                let context = math::rms_norm(
                    &math::multiply_scaled(&projected[i][range.clone()], &ones, projection_scale)?,
                    Some(&norm),
                    spec.rms_epsilon,
                )?;
                let token = math::multiply_scaled(&row[range], &ones, token_scale)?;
                slot.push(math::multiply_scaled(
                    &math::add(&context, &token)?,
                    &ones,
                    combine_scale,
                )?);
            }
        }
        Ok(Some(out))
    }
    fn final_norm(&self, hidden: &[Vec<f32>]) -> Result<Vec<Vec<f32>>, Error> {
        hidden
            .iter()
            .map(|row| {
                Ok(math::rms_norm(
                    row,
                    Some(&self.output_scale),
                    self.model.spec.rms_epsilon,
                )?)
            })
            .collect()
    }
    /// Causal token-ID encoder through every layer and final RMS norm. All
    /// cache replacements commit together AFTER the whole pass succeeds.
    pub fn encode(
        &self,
        tokens: &[u32],
        cache: &mut EncoderCache<'a>,
    ) -> Result<Vec<Vec<f32>>, Error> {
        self.encode_traced(tokens, cache, &mut |_, _| {
            Ok::<_, std::convert::Infallible>(())
        })
        .map_err(TraceError::compute)
    }
    /// Observer errors abort before the all-layer cache commit. Previously
    /// emitted diagnostic snapshots are not rolled back; label them incomplete.
    pub fn encode_traced<E>(
        &self,
        tokens: &[u32],
        cache: &mut EncoderCache<'a>,
        observer: &mut impl FnMut(TracePoint, &[Vec<f32>]) -> Result<(), E>,
    ) -> Result<Vec<Vec<f32>>, TraceError<E>> {
        self.check_tokens(tokens, cache, false)?;
        // One-token steps run one worker per core (see Workers::one_per_core).
        let _cap = self
            .workers
            .as_ref()
            .filter(|_| tokens.len() <= 2)
            .map(|w| w.one_per_core());
        let mut hidden = self.embeddings(tokens)?;
        observer(TracePoint::EncoderEmbedding, &hidden).map_err(TraceError::Observer)?;
        let per_layer = self.per_layer_inputs(tokens, &hidden)?;
        // Layers update their caches in place; window eviction is committed
        // only after the whole pass (and the observer) succeeded, otherwise
        // every touched layer is rolled back.
        let layers = &self.model.layers;
        let mut pending: Vec<crate::engine::attention::PendingKv> =
            Vec::with_capacity(layers.len());
        let mut run = || -> Result<Vec<Vec<f32>>, TraceError<E>> {
            for (i, block) in self.blocks.iter().enumerate() {
                let ple = per_layer.as_ref().map(|p| p[i].as_slice());
                let (before, rest) = cache.layers.split_at_mut(i);
                let own = rest.first_mut().ok_or(Error::Shape("layer cache"))?;
                let (output, routing, step) = match layers[i].kv_source {
                    Some(source) => block.encode_shared_in_place(
                        &hidden,
                        own,
                        before
                            .get(source)
                            .ok_or(Error::Shape("shared-K/V source"))?,
                        pending
                            .get(source)
                            .ok_or(Error::Shape("shared-K/V source not staged"))?,
                        ple,
                    )?,
                    None => block.encode_in_place(&hidden, own, ple)?,
                };
                pending.push(step);
                hidden = output;
                observer(TracePoint::EncoderLayer(i), &hidden).map_err(TraceError::Observer)?;
                observer(TracePoint::EncoderRouting(i), &routing_rows(&routing))
                    .map_err(TraceError::Observer)?;
            }
            let normalized = self.final_norm(&hidden)?;
            observer(TracePoint::EncoderFinal, &normalized).map_err(TraceError::Observer)?;
            Ok(normalized)
        };
        let outcome = run();
        match outcome {
            Ok(normalized) => {
                for layer in &mut cache.layers {
                    layer.commit();
                }
                Ok(normalized)
            }
            Err(error) => {
                for (layer, step) in cache.layers.iter_mut().zip(&pending) {
                    layer.rollback(step);
                }
                Err(error)
            }
        }
    }
    /// Previous TEMPERATURE-PROCESSED logits -> F32 softmax -> raw embedding
    /// matrix -> sqrt(hidden) scale, matching modeling_diffusion_gemma.py:1278–1281.
    /// The tied output head remains unscaled.
    pub fn soft_embeddings(
        &self,
        processed_logits: &[f32],
        positions: usize,
    ) -> Result<Vec<Vec<f32>>, Error> {
        if positions == 0 || positions > self.model.spec.canvas {
            return Err(Error::Shape("soft-embedding canvas length"));
        }
        let expected = self.check_logits(positions)?;
        if processed_logits.len() != expected {
            return Err(Error::Shape("processed logits dimensions"));
        }
        if positions
            .checked_mul(self.model.spec.hidden)
            .filter(|&n| n <= self.limits.max_hidden_elements)
            .is_none()
        {
            return Err(Error::ResourceLimit);
        }
        // Validate every row before weight streaming. -inf is a supported mask.
        for row in processed_logits.chunks_exact(self.model.spec.vocabulary) {
            if row.iter().any(|x| x.is_nan() || *x == f32::INFINITY)
                || !row.iter().any(|x| x.is_finite())
            {
                return Err(Error::Parameter("invalid self-conditioning logits"));
            }
        }
        let _span = crate::engine::stats::Span::new(crate::engine::stats::Stage::SoftEmbedding);
        if self.batched {
            let compute = self.compute();
            let vocabulary = self.model.spec.vocabulary;
            let probabilities = compute.map_positions(positions, |i| {
                Ok::<_, Error>(math::softmax(
                    &processed_logits[i * vocabulary..(i + 1) * vocabulary],
                )?)
            })?;
            return operators::project_transposed_streamed_batch_with(
                self.file,
                self.model.embedding,
                &probabilities,
                self.limits.max_hidden_elements,
                self.compute(),
            )?
            .iter()
            .map(|raw| {
                Ok(math::multiply_scaled(
                    raw,
                    &vec![1.0; self.model.spec.hidden],
                    (self.model.spec.hidden as f32).sqrt(),
                )?)
            })
            .collect();
        }
        processed_logits
            .chunks_exact(self.model.spec.vocabulary)
            .map(|row| {
                let raw = operators::project_transposed_streamed(
                    self.file,
                    self.model.embedding,
                    &math::softmax(row)?,
                )?;
                Ok(math::multiply_scaled(
                    &raw,
                    &vec![1.0; self.model.spec.hidden],
                    (self.model.spec.hidden as f32).sqrt(),
                )?)
            })
            .collect()
    }
    /// Final normalized decoder hidden states. Supports short diagnostic
    /// canvases up to the configured length; no sampling/generation occurs.
    /// Encoder cache is immutably borrowed throughout, also on failure.
    pub fn decode_hidden(
        &self,
        tokens: &[u32],
        cache: &EncoderCache<'a>,
        previous_processed_logits: Option<&[f32]>,
    ) -> Result<Vec<Vec<f32>>, Error> {
        self.decode_hidden_traced(tokens, cache, previous_processed_logits, &mut |_, _| {
            Ok::<_, std::convert::Infallible>(())
        })
        .map_err(TraceError::compute)
    }
    pub fn decode_hidden_traced<E>(
        &self,
        tokens: &[u32],
        cache: &EncoderCache<'a>,
        previous_processed_logits: Option<&[f32]>,
        observer: &mut impl FnMut(TracePoint, &[Vec<f32>]) -> Result<(), E>,
    ) -> Result<Vec<Vec<f32>>, TraceError<E>> {
        if self.model.spec.kind != crate::engine::model::ModelKind::Diffusion {
            return Err(Error::Shape("diffusion decoder on an autoregressive model").into());
        }
        self.check_tokens(tokens, cache, true)?;
        let soft = previous_processed_logits
            .map(|l| self.soft_embeddings(l, tokens.len()))
            .transpose()?;
        if let Some(soft) = &soft {
            observer(TracePoint::DecoderSoftEmbedding, soft).map_err(TraceError::Observer)?;
        }
        let embedded = self.embeddings(tokens)?;
        observer(TracePoint::DecoderEmbedding, &embedded).map_err(TraceError::Observer)?;
        let mut hidden = if self.batched {
            operators::self_condition_batch_with(
                self.file,
                self.model
                    .self_conditioning_mlp
                    .as_ref()
                    .ok_or(Error::Shape("diffusion decoder on an autoregressive model"))?,
                &self.self_conditioning_scale,
                &embedded,
                soft.as_deref(),
                self.model.spec.rms_epsilon,
                self.compute(),
            )?
        } else {
            let conditioner = self
                .self_conditioner
                .as_ref()
                .ok_or(Error::Shape("missing reference self-conditioner"))?;
            embedded
                .iter()
                .enumerate()
                .map(|(i, x)| conditioner.apply(x, soft.as_ref().map(|s| s[i].as_slice())))
                .collect::<Result<Vec<_>, _>>()?
        };
        observer(TracePoint::DecoderInput, &hidden).map_err(TraceError::Observer)?;
        for (i, (block, layer_cache)) in self.blocks.iter().zip(&cache.layers).enumerate() {
            let (output, routing) = block.decode_routed(&hidden, layer_cache)?;
            hidden = output;
            observer(TracePoint::DecoderLayer(i), &hidden).map_err(TraceError::Observer)?;
            observer(TracePoint::DecoderRouting(i), &routing_rows(&routing))
                .map_err(TraceError::Observer)?;
        }
        let normalized = self.final_norm(&hidden)?;
        observer(TracePoint::DecoderFinal, &normalized).map_err(TraceError::Observer)?;
        Ok(normalized)
    }
    /// Tied unscaled embedding/output head + final softcap. Hidden rows must
    /// ALREADY have the model's final norm. Returns flat [positions, vocab].
    pub fn logits(&self, hidden: &[Vec<f32>]) -> Result<Vec<f32>, Error> {
        let count = self.check_logits(hidden.len())?;
        if hidden
            .iter()
            .any(|row| row.len() != self.model.spec.hidden || row.iter().any(|x| !x.is_finite()))
        {
            return Err(Error::Shape("output hidden-state dimensions/values"));
        }
        let _span = crate::engine::stats::Span::new(crate::engine::stats::Stage::Logits);
        let _cap = self
            .workers
            .as_ref()
            .filter(|_| hidden.len() <= 2)
            .map(|w| w.one_per_core());
        if self.batched {
            let raw = operators::project_streamed_batch_with(
                self.file,
                self.model.embedding,
                hidden,
                self.limits.max_logit_elements,
                self.compute(),
            )?;
            let lens: Vec<usize> = raw.iter().map(Vec::len).collect();
            let capped = self.compute().map_split(&lens, 4096, |i, a, b| {
                Ok::<_, Error>(math::softcap(&raw[i][a..b], self.model.spec.logit_softcap)?)
            })?;
            drop(raw);
            let mut out = Vec::new();
            out.try_reserve_exact(count)
                .map_err(|_| Error::ResourceLimit)?;
            for row in capped {
                out.extend(row);
            }
            return Ok(out);
        }
        let mut out = Vec::new();
        out.try_reserve_exact(count)
            .map_err(|_| Error::ResourceLimit)?;
        for row in hidden {
            let raw = operators::project_streamed(self.file, self.model.embedding, row)?;
            out.extend(math::softcap(&raw, self.model.spec.logit_softcap)?);
        }
        Ok(out)
    }
    pub fn decode_logits(
        &self,
        tokens: &[u32],
        cache: &EncoderCache<'a>,
        previous_processed_logits: Option<&[f32]>,
    ) -> Result<Vec<f32>, Error> {
        self.check_logits(tokens.len())?; // fail before an expensive forward
        self.logits(&self.decode_hidden(tokens, cache, previous_processed_logits)?)
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod trace_tests;
