//! Reference operator orchestration. Rust owns buffers/routing; C performs
//! tensor arithmetic. This is not a complete decoder or fast inference path.

use crate::engine::{
    model::{LayerBindings, MlpBindings, RouterBindings, Specification},
    weights::ModelFile,
};
use crate::gguf::TensorInfo;
use crate::kernels::reference::{self as math, Matrix};
use std::fmt;

/// A single F32 expansion may use at most 64 MiB by default. The reference
/// probe intentionally cannot expand the entire vocabulary/expert collection.
pub const MAX_REFERENCE_MATRIX_ELEMENTS: usize = 16 * 1024 * 1024;

#[derive(Debug)]
pub enum Error {
    Shape(&'static str),
    Parameter(&'static str),
    ResourceLimit,
    Weights(crate::engine::weights::Error),
    Math(math::Error),
    Quantized(crate::kernels::quantized::Error),
    Execution(crate::engine::execution::Error),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
impl From<crate::engine::weights::Error> for Error {
    fn from(e: crate::engine::weights::Error) -> Self {
        Self::Weights(e)
    }
}
impl From<math::Error> for Error {
    fn from(e: math::Error) -> Self {
        Self::Math(e)
    }
}

impl From<crate::engine::execution::Error> for Error {
    fn from(e: crate::engine::execution::Error) -> Self {
        Self::Execution(e)
    }
}
impl From<crate::kernels::quantized::Error> for Error {
    fn from(e: crate::kernels::quantized::Error) -> Self {
        Self::Quantized(e)
    }
}

fn matches_file(file: &ModelFile, tensor: &TensorInfo) -> Result<(), Error> {
    let actual = file
        .tensor(&tensor.name)
        .ok_or(Error::Shape("tensor not in model"))?;
    if actual.shape != tensor.shape
        || actual.type_id != tensor.type_id
        || actual.file_offset != tensor.file_offset
        || actual.size_bytes != tensor.size_bytes
    {
        return Err(Error::Shape("tensor binding belongs to a different model"));
    }
    Ok(())
}

pub fn load_vector(file: &ModelFile, tensor: &TensorInfo) -> Result<Vec<f32>, Error> {
    matches_file(file, tensor)?;
    if tensor.shape[1..] != [1, 1, 1] {
        return Err(Error::Shape("expected vector"));
    }
    if tensor.shape[0] > MAX_REFERENCE_MATRIX_ELEMENTS as u64 {
        return Err(Error::ResourceLimit);
    }
    let vector = file.cached(
        &format!("vector:{}", tensor.name),
        || -> Result<Vec<f32>, Error> { Ok(file.read_row(&tensor.name, 0)?.decode()?) },
    )?;
    Ok(vector.as_ref().clone())
}

/// The layer's router, loaded once per file (see [`ModelFile::cached`]).
pub fn cached_router(
    file: &ModelFile,
    binding: &RouterBindings<'_>,
    top_k: usize,
    epsilon: f32,
) -> Result<std::sync::Arc<Router>, Error> {
    file.cached(
        &format!(
            "router:{}:{top_k}:{}",
            binding.projection.name,
            epsilon.to_bits()
        ),
        || Router::load(file, binding, top_k, epsilon),
    )
}

pub fn load_matrix(
    file: &ModelFile,
    tensor: &TensorInfo,
    max_elements: usize,
) -> Result<Matrix, Error> {
    matches_file(file, tensor)?;
    if tensor.shape[2..] != [1, 1] {
        return Err(Error::Shape("expected 2D matrix"));
    }
    load_matrix_rows(file, tensor, 0, max_elements)
}

/// Read one expert's contiguous slice, never the entire expert collection.
pub fn load_expert_matrix(
    file: &ModelFile,
    tensor: &TensorInfo,
    expert: usize,
    max_elements: usize,
) -> Result<Matrix, Error> {
    matches_file(file, tensor)?;
    if tensor.shape[3] != 1 || expert as u64 >= tensor.shape[2] {
        return Err(Error::Shape("expert index or tensor rank"));
    }
    let start = (expert as u64)
        .checked_mul(tensor.shape[1])
        .ok_or(Error::ResourceLimit)?;
    load_matrix_rows(file, tensor, start, max_elements)
}

fn load_matrix_rows(
    file: &ModelFile,
    tensor: &TensorInfo,
    first_row: u64,
    max_elements: usize,
) -> Result<Matrix, Error> {
    let rows = usize::try_from(tensor.shape[1]).map_err(|_| Error::ResourceLimit)?;
    load_row_slice(file, tensor, first_row, rows, max_elements)
}

fn load_row_slice(
    file: &ModelFile,
    tensor: &TensorInfo,
    first_row: u64,
    rows: usize,
    max_elements: usize,
) -> Result<Matrix, Error> {
    let cols = usize::try_from(tensor.shape[0]).map_err(|_| Error::ResourceLimit)?;
    let count = rows
        .checked_mul(cols)
        .filter(|&n| n <= max_elements)
        .ok_or(Error::ResourceLimit)?;
    let end = first_row
        .checked_add(rows as u64)
        .ok_or(Error::ResourceLimit)?;
    let mut data = Vec::new();
    data.try_reserve_exact(count)
        .map_err(|_| Error::ResourceLimit)?;
    for row in first_row..end {
        data.extend(file.read_row(&tensor.name, row)?.decode()?);
    }
    Ok(Matrix::new(rows, cols, data)?)
}

/// Bounded row-tile expansion for a reference projection. This handles global
/// Q/output matrices larger than our single-matrix cap, without expanding the
/// complete matrix (or vocabulary table) at once. No packing/cache/parallelism.
pub fn project_streamed(
    file: &ModelFile,
    tensor: &TensorInfo,
    input: &[f32],
) -> Result<Vec<f32>, Error> {
    matches_file(file, tensor)?;
    if tensor.shape[2..] != [1, 1] || input.is_empty() || tensor.shape[0] != input.len() as u64 {
        return Err(Error::Shape("streamed projection dimensions"));
    }
    if input.iter().any(|x| !x.is_finite()) {
        return Err(Error::Parameter("nonfinite projection input"));
    }
    let rows = usize::try_from(tensor.shape[1]).map_err(|_| Error::ResourceLimit)?;
    if rows == 0 {
        return Err(Error::Shape("empty projection"));
    }
    let tile_rows = (MAX_REFERENCE_MATRIX_ELEMENTS / input.len()).min(128);
    if tile_rows == 0 {
        return Err(Error::ResourceLimit);
    }
    let mut out = Vec::new();
    out.try_reserve_exact(rows)
        .map_err(|_| Error::ResourceLimit)?;
    for first in (0..rows).step_by(tile_rows) {
        let count = (rows - first).min(tile_rows);
        out.extend(
            load_row_slice(
                file,
                tensor,
                first as u64,
                count,
                MAX_REFERENCE_MATRIX_ELEMENTS,
            )?
            .project(input)?,
        );
    }
    Ok(out)
}

/// Bounded streamed matrix^T projection over raw stored weights. The text
/// decoder applies sqrt(hidden) to the resulting soft embeddings separately.
pub fn project_transposed_streamed(
    file: &ModelFile,
    tensor: &TensorInfo,
    weights: &[f32],
) -> Result<Vec<f32>, Error> {
    matches_file(file, tensor)?;
    if tensor.shape[2..] != [1, 1] || weights.is_empty() || tensor.shape[1] != weights.len() as u64
    {
        return Err(Error::Shape("transposed projection dimensions"));
    }
    if weights.iter().any(|x| !x.is_finite()) {
        return Err(Error::Parameter("nonfinite projection weights"));
    }
    let cols = usize::try_from(tensor.shape[0]).map_err(|_| Error::ResourceLimit)?;
    if cols == 0 {
        return Err(Error::Shape("empty transposed projection"));
    }
    let tile_rows = (MAX_REFERENCE_MATRIX_ELEMENTS / cols).min(128);
    if tile_rows == 0 {
        return Err(Error::ResourceLimit);
    }
    let mut accumulator = Vec::new();
    accumulator
        .try_reserve_exact(cols)
        .map_err(|_| Error::ResourceLimit)?;
    accumulator.resize(cols, 0.0_f64);
    for first in (0..weights.len()).step_by(tile_rows) {
        let count = (weights.len() - first).min(tile_rows);
        let slice = &weights[first..first + count];
        if slice.iter().all(|&x| x == 0.0) {
            continue;
        }
        load_row_slice(
            file,
            tensor,
            first as u64,
            count,
            MAX_REFERENCE_MATRIX_ELEMENTS,
        )?
        .accumulate_transposed(slice, &mut accumulator)?;
    }
    let out: Vec<_> = accumulator.into_iter().map(|x| x as f32).collect();
    if out.iter().any(|x| !x.is_finite()) {
        return Err(Error::Math(math::Error::NonFiniteValue));
    }
    Ok(out)
}

pub struct GatedMlp {
    gate: Matrix,
    up: Matrix,
    down: Matrix,
}
impl GatedMlp {
    pub fn new(gate: Matrix, up: Matrix, down: Matrix) -> Result<Self, Error> {
        if gate.rows() != up.rows()
            || gate.cols() != up.cols()
            || down.cols() != gate.rows()
            || down.rows() != gate.cols()
        {
            return Err(Error::Shape("incompatible gated MLP matrices"));
        }
        Ok(Self { gate, up, down })
    }
    pub fn load(file: &ModelFile, binding: &MlpBindings<'_>) -> Result<Self, Error> {
        Self::new(
            load_matrix(file, binding.gate, MAX_REFERENCE_MATRIX_ELEMENTS)?,
            load_matrix(file, binding.up, MAX_REFERENCE_MATRIX_ELEMENTS)?,
            load_matrix(file, binding.down, MAX_REFERENCE_MATRIX_ELEMENTS)?,
        )
    }
    pub fn width(&self) -> usize {
        self.gate.cols()
    }
    pub fn forward(&self, x: &[f32]) -> Result<Vec<f32>, Error> {
        let gate = self.gate.project(x)?;
        let up = self.up.project(x)?;
        Ok(self.down.project(&math::gelu_product(&gate, &up)?)?)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpertWeight {
    pub expert: usize,
    pub weight: f32,
}
#[derive(Debug)]
pub struct Routing {
    pub probabilities: Vec<f32>,
    /// Weights renormalized over top-k, THEN multiplied by per-expert scales.
    /// Consequently these scaled contribution weights need not sum to one.
    pub selected: Vec<ExpertWeight>,
}

pub struct Router {
    projection: Matrix,
    hidden_scale: Vec<f32>,
    expert_scale: Vec<f32>,
    top_k: usize,
    epsilon: f32,
}
impl Router {
    pub fn new(
        projection: Matrix,
        hidden_scale: Vec<f32>,
        expert_scale: Vec<f32>,
        top_k: usize,
        epsilon: f32,
    ) -> Result<Self, Error> {
        if projection.cols() != hidden_scale.len() || projection.rows() != expert_scale.len() {
            return Err(Error::Shape("router scale dimensions"));
        }
        if top_k == 0
            || top_k > projection.rows()
            || !epsilon.is_finite()
            || epsilon <= 0.0
            || hidden_scale
                .iter()
                .chain(&expert_scale)
                .any(|x| !x.is_finite())
        {
            return Err(Error::Parameter("invalid router scales/top-k/epsilon"));
        }
        Ok(Self {
            projection,
            hidden_scale,
            expert_scale,
            top_k,
            epsilon,
        })
    }
    pub fn load(
        file: &ModelFile,
        binding: &RouterBindings<'_>,
        top_k: usize,
        epsilon: f32,
    ) -> Result<Self, Error> {
        Self::new(
            load_matrix(file, binding.projection, MAX_REFERENCE_MATRIX_ELEMENTS)?,
            load_vector(file, binding.hidden_scale)?,
            load_vector(file, binding.expert_scale)?,
            top_k,
            epsilon,
        )
    }
    pub fn route(&self, residual: &[f32]) -> Result<Routing, Error> {
        self.route_with(residual, |m, x| Ok(m.project(x)?))
    }
    /// [`Self::route`] of each residual (identical results), with one
    /// router-matrix pass per 4 positions.
    pub fn route_many(&self, residuals: &[Vec<f32>]) -> Result<Vec<Routing>, Error> {
        let scaled = residuals
            .iter()
            .map(|r| self.scaled_input(r))
            .collect::<Result<Vec<_>, _>>()?;
        let refs: Vec<&[f32]> = scaled.iter().map(Vec::as_slice).collect();
        self.projection
            .project_many(&refs)?
            .into_iter()
            .map(|logits| self.select(logits))
            .collect()
    }
    fn scaled_input(&self, residual: &[f32]) -> Result<Vec<f32>, Error> {
        // Routing receives the residual BEFORE either MLP branch normalization.
        let normalized = math::rms_norm(residual, None, self.epsilon)?;
        Ok(math::multiply_scaled(
            &normalized,
            &self.hidden_scale,
            1.0 / (self.hidden_scale.len() as f32).sqrt(),
        )?)
    }
    /// [`Self::route`] with the router projection computed by `project`
    /// (which must equal `Matrix::project`, e.g. split by rows).
    pub fn route_with(
        &self,
        residual: &[f32],
        project: impl FnOnce(&crate::kernels::reference::Matrix, &[f32]) -> Result<Vec<f32>, Error>,
    ) -> Result<Routing, Error> {
        let scaled = self.scaled_input(residual)?;
        self.select(project(&self.projection, &scaled)?)
    }
    /// Softmax, top-k and renormalized weights of the router logits.
    fn select(&self, logits: Vec<f32>) -> Result<Routing, Error> {
        let probabilities = math::softmax(&logits)?;
        let mut order: Vec<usize> = (0..probabilities.len()).collect();
        order.sort_by(|&a, &b| {
            probabilities[b]
                .total_cmp(&probabilities[a])
                .then(a.cmp(&b))
        });
        order.truncate(self.top_k);
        let sum: f64 = order.iter().map(|&i| f64::from(probabilities[i])).sum();
        let selected: Vec<_> = order
            .into_iter()
            .map(|expert| ExpertWeight {
                expert,
                weight: (f64::from(probabilities[expert]) / sum) as f32 * self.expert_scale[expert],
            })
            .collect();
        if selected.iter().any(|s| !s.weight.is_finite()) {
            return Err(Error::Parameter("router weight overflow"));
        }
        Ok(Routing {
            probabilities,
            selected,
        })
    }
}

/// Fused GGUF rows are [all gate rows | all up rows], not interleaved.
pub fn fused_expert(gate_up: &Matrix, down: &Matrix, input: &[f32]) -> Result<Vec<f32>, Error> {
    if down.cols().checked_mul(2) != Some(gate_up.rows()) || down.rows() != gate_up.cols() {
        return Err(Error::Shape("incompatible fused expert matrices"));
    }
    let projected = gate_up.project(input)?;
    let (gate, up) = projected.split_at(down.cols());
    Ok(down.project(&math::gelu_product(gate, up)?)?)
}

#[derive(Debug, Clone, Copy)]
pub enum LayerMode {
    Encoder,
    Decoder,
}

pub struct FeedForwardResult {
    pub hidden: Vec<f32>,
    pub routing: Routing,
}

/// Full reference feed-forward sub-block, starting AFTER the attention
/// residual addition. Applies the mode-specific layer scalar at the end.
/// This reloads and expands selected matrices each call: correctness only.
/// Peak expansion is bounded by individual matrices, not all 128 experts.
pub fn feed_forward(
    file: &ModelFile,
    layer: &LayerBindings<'_>,
    spec: &Specification,
    residual: &[f32],
    mode: LayerMode,
) -> Result<FeedForwardResult, Error> {
    // The serial reference path covers MoE layers without per-layer inputs.
    let moe = layer
        .moe
        .as_ref()
        .filter(|_| layer.per_layer.is_none())
        .ok_or(Error::Shape(
            "dense/per-layer-input layers need batched execution",
        ))?;
    if residual.len() != spec.hidden {
        return Err(Error::Shape("feed-forward residual width"));
    }
    let shared_input = math::rms_norm(
        residual,
        Some(&load_vector(file, layer.shared_pre_norm)?),
        spec.rms_epsilon,
    )?;
    let shared = GatedMlp::load(file, &layer.shared_mlp)?.forward(&shared_input)?;
    let shared = math::rms_norm(
        &shared,
        Some(&load_vector(file, moe.shared_post_norm)?),
        spec.rms_epsilon,
    )?;

    let routing =
        cached_router(file, &moe.router, spec.top_k, spec.rms_epsilon)?.route(residual)?;
    let expert_input = math::rms_norm(
        residual,
        Some(&load_vector(file, moe.expert_pre_norm)?),
        spec.rms_epsilon,
    )?;
    let ones = vec![1.0; spec.hidden];
    let mut experts = vec![0.0; spec.hidden];
    // Match the reference's ascending expert-ID accumulation order. Sorting
    // a copy preserves probability-ranked routing diagnostics.
    let mut selected: Vec<_> = routing.selected.iter().collect();
    selected.sort_by_key(|s| s.expert);
    for selected in selected {
        let gate_up = load_expert_matrix(
            file,
            moe.expert_gate_up,
            selected.expert,
            MAX_REFERENCE_MATRIX_ELEMENTS,
        )?;
        let down = load_expert_matrix(
            file,
            moe.expert_down,
            selected.expert,
            MAX_REFERENCE_MATRIX_ELEMENTS,
        )?;
        let result = fused_expert(&gate_up, &down, &expert_input)?;
        experts = math::add(
            &experts,
            &math::multiply_scaled(&result, &ones, selected.weight)?,
        )?;
    }
    let experts = math::rms_norm(
        &experts,
        Some(&load_vector(file, moe.expert_post_norm)?),
        spec.rms_epsilon,
    )?;
    let combined = math::add(&shared, &experts)?;
    let combined = math::rms_norm(
        &combined,
        Some(&load_vector(file, layer.combined_ff_norm)?),
        spec.rms_epsilon,
    )?;
    let scale = load_vector(
        file,
        match mode {
            LayerMode::Encoder => layer.encoder_scale,
            LayerMode::Decoder => layer.decoder_scale,
        },
    )?;
    if scale.len() != 1 {
        return Err(Error::Shape("layer scalar must have one element"));
    }
    let hidden = math::multiply_scaled(&math::add(residual, &combined)?, &ones, scale[0])?;
    Ok(FeedForwardResult { hidden, routing })
}

pub struct SelfConditioner {
    pre_scale: Vec<f32>,
    mlp: GatedMlp,
    epsilon: f32,
}
impl SelfConditioner {
    pub fn new(pre_scale: Vec<f32>, mlp: GatedMlp, epsilon: f32) -> Result<Self, Error> {
        if pre_scale.len() != mlp.width() {
            return Err(Error::Shape("self-conditioning width"));
        }
        if !epsilon.is_finite() || epsilon <= 0.0 || pre_scale.iter().any(|x| !x.is_finite()) {
            return Err(Error::Parameter("invalid self-conditioning scale/epsilon"));
        }
        Ok(Self {
            pre_scale,
            mlp,
            epsilon,
        })
    }
    /// Accepts already-computed soft embeddings, not previous logits. The full
    /// vocabulary softmax × embedding projection is not implemented here.
    /// None models the first denoising step: zero self-conditioning signal.
    pub fn apply(
        &self,
        token_embedding: &[f32],
        soft_embedding: Option<&[f32]>,
    ) -> Result<Vec<f32>, Error> {
        if token_embedding.len() != self.pre_scale.len() {
            return Err(Error::Shape("token embedding width"));
        }
        let zero;
        let soft = match soft_embedding {
            Some(s) => s,
            None => {
                zero = vec![0.0; self.pre_scale.len()];
                &zero
            }
        };
        let normalized = math::rms_norm(soft, Some(&self.pre_scale), self.epsilon)?;
        let signal = self.mlp.forward(&normalized)?;
        // Even with no signal the decoder input is normalized, not unchanged.
        Ok(math::rms_norm(
            &math::add(token_embedding, &signal)?,
            None,
            self.epsilon,
        )?)
    }
}

mod batch;
pub(crate) use batch::{ExpertActivation, routed_experts};
pub use batch::{
    feed_forward_batch, feed_forward_batch_with, feed_forward_batch_with_workers,
    project_streamed_batch, project_streamed_batch_with, project_streamed_batch_with_workers,
    project_transposed_streamed_batch, project_transposed_streamed_batch_with,
    project_transposed_streamed_batch_with_workers, self_condition_batch,
    self_condition_batch_with, self_condition_batch_with_workers,
};
pub(crate) use batch::{feed_forward_layer, prepack, project_many};

#[cfg(test)]
mod tests;
