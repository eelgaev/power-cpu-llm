//! Reference forward pass of a bound Nemotron 3.5 model: F32 activations,
//! F64 reductions, streamed GGUF projections through the batched operators.
//! Every block is `x + mixer(rms_norm(x))`. Mamba-2 runs the recurrent form
//! one position at a time (exact for any prompt length, unlike the chunked
//! scan it matches only up to rounding); dt is not clamped, as in llama.cpp
//! and the transformers recurrent step. Slow correctness path; the MTP block
//! is not executed.

use super::{AttentionBindings, LayerKind, MambaBindings, Mixer, ModelDefinition, MoeBindings};
use crate::engine::execution::Compute;
use crate::engine::operators::{
    Error, ExpertActivation, MAX_REFERENCE_MATRIX_ELEMENTS, load_matrix, project_many,
    project_streamed_batch_with, routed_experts,
};
use crate::engine::stats::{Span, Stage};
use crate::engine::weights::{ModelFile, row_count};
use crate::gguf::TensorInfo;
use crate::kernels::reference as math;
use crate::libm;
use std::sync::Arc;

/// Recurrent state of one layer.
#[derive(Debug, Clone)]
enum LayerState {
    Mamba {
        /// The last `kernel - 1` convolution inputs, oldest first.
        conv: Vec<f32>,
        /// `[head][head_dim][state]`.
        ssm: Vec<f32>,
    },
    Attention {
        /// Token-major `[kv_heads * head_dim]` rows.
        keys: Vec<f32>,
        values: Vec<f32>,
    },
    Moe,
}

/// The state after the positions consumed so far.
#[derive(Debug, Clone)]
pub struct State {
    layers: Vec<LayerState>,
    positions: usize,
}
impl State {
    pub fn new(model: &ModelDefinition<'_>) -> Self {
        let spec = &model.spec;
        let layers = spec
            .layers
            .iter()
            .map(|kind| match kind {
                LayerKind::Mamba => LayerState::Mamba {
                    conv: vec![0.0; (spec.ssm_conv_kernel - 1) * spec.ssm_conv_channels()],
                    ssm: vec![0.0; spec.ssm_inner * spec.ssm_state],
                },
                LayerKind::Attention => LayerState::Attention {
                    keys: Vec::new(),
                    values: Vec::new(),
                },
                LayerKind::Moe => LayerState::Moe,
            })
            .collect();
        Self {
            layers,
            positions: 0,
        }
    }
    pub fn positions(&self) -> usize {
        self.positions
    }
}

/// Every element of a small tensor in row order, decoded once per file.
fn load_all(file: &ModelFile, tensor: &TensorInfo) -> Result<Arc<Vec<f32>>, Error> {
    file.cached(&format!("all:{}", tensor.name), || {
        let rows = row_count(tensor)?;
        let mut out = Vec::new();
        for row in 0..rows {
            out.extend(file.read_row(&tensor.name, row)?.decode()?);
        }
        if out.len() > MAX_REFERENCE_MATRIX_ELEMENTS {
            return Err(Error::ResourceLimit);
        }
        Ok(out)
    })
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + libm::exp(-x))
}
fn silu(x: f32) -> f32 {
    (f64::from(x) * sigmoid(f64::from(x))) as f32
}
/// `torch.nn.functional.softplus` (threshold 20).
fn softplus(x: f64) -> f64 {
    if x > 20.0 {
        x
    } else {
        libm::ln(1.0 + libm::exp(x))
    }
}
fn relu_squared(x: f32) -> f32 {
    let r = x.max(0.0);
    r * r
}

fn residual_add(x: &mut [Vec<f32>], delta: &[Vec<f32>]) {
    for (row, d) in x.iter_mut().zip(delta) {
        for (a, b) in row.iter_mut().zip(d) {
            *a += b;
        }
    }
}

/// Runs `tokens` after the positions already in `state`, returning the final
/// hidden states (before the output norm). `observe(layer, rows)` sees the
/// residual stream after each layer.
pub fn forward(
    file: &ModelFile,
    model: &ModelDefinition<'_>,
    state: &mut State,
    tokens: &[u32],
    compute: Compute<'_>,
    observe: &mut dyn FnMut(usize, &[Vec<f32>]),
) -> Result<Vec<Vec<f32>>, Error> {
    let spec = &model.spec;
    if tokens.is_empty() || state.layers.len() != spec.layers.len() {
        return Err(Error::Shape("empty input or state of another model"));
    }
    if state.positions + tokens.len() > spec.context {
        return Err(Error::Parameter("context length exceeded"));
    }
    let mut x = tokens
        .iter()
        .map(|&t| {
            if t as usize >= spec.vocabulary {
                return Err(Error::Parameter("token outside the vocabulary"));
            }
            Ok(file
                .read_row(&model.embedding.name, u64::from(t))?
                .decode()?)
        })
        .collect::<Result<Vec<_>, Error>>()?;
    for (index, (layer, layer_state)) in model.layers.iter().zip(&mut state.layers).enumerate() {
        let norm = load_all(file, layer.norm)?;
        let normed = compute.map_positions(x.len(), |i| {
            Ok::<_, Error>(math::rms_norm(&x[i], Some(&norm), spec.rms_epsilon)?)
        })?;
        let delta = match (&layer.mixer, layer_state) {
            (Mixer::Mamba(b), LayerState::Mamba { conv, ssm }) => {
                mamba(file, model, b, conv, ssm, &normed, compute)?
            }
            (Mixer::Attention(b), LayerState::Attention { keys, values }) => {
                attention(file, model, b, keys, values, &normed, compute)?
            }
            (Mixer::Moe(b), LayerState::Moe) => moe(file, model, b, &normed, compute)?,
            _ => return Err(Error::Shape("state does not match the layer kind")),
        };
        residual_add(&mut x, &delta);
        if !x.iter().all(|r| math::all_finite(r)) {
            return Err(Error::Math(math::Error::NonFiniteValue));
        }
        observe(index, &x);
    }
    state.positions += tokens.len();
    Ok(x)
}

/// Packs every projection weight once for the packing (int8) backends, so
/// no token pays for packing an expert on its first use. A no-op for the
/// other backends or without workers.
pub fn prepack(
    file: &ModelFile,
    model: &ModelDefinition<'_>,
    compute: Compute<'_>,
) -> Result<(), Error> {
    let mut tensors: Vec<&TensorInfo> = vec![model.output];
    for layer in &model.layers {
        match &layer.mixer {
            Mixer::Mamba(b) => tensors.extend([b.input, b.output]),
            Mixer::Attention(b) => tensors.extend([b.query, b.key, b.value, b.output]),
            Mixer::Moe(b) => {
                tensors.extend([b.expert_up, b.expert_down, b.shared_up, b.shared_down])
            }
        }
    }
    crate::engine::operators::prepack(file, &tensors, compute)
}

/// Logits of one final hidden state.
pub fn logits(
    file: &ModelFile,
    model: &ModelDefinition<'_>,
    hidden: &[f32],
    compute: Compute<'_>,
) -> Result<Vec<f32>, Error> {
    let _span = Span::new(Stage::Logits);
    let norm = load_all(file, model.output_norm)?;
    let normed = math::rms_norm(hidden, Some(&norm), model.spec.rms_epsilon)?;
    let mut out = project_streamed_batch_with(
        file,
        model.output,
        &[normed],
        MAX_REFERENCE_MATRIX_ELEMENTS,
        compute,
    )?;
    Ok(out.pop().expect("one position"))
}

fn mamba(
    file: &ModelFile,
    model: &ModelDefinition<'_>,
    b: &MambaBindings<'_>,
    conv_state: &mut [f32],
    ssm: &mut [f32],
    input: &[Vec<f32>],
    compute: Compute<'_>,
) -> Result<Vec<Vec<f32>>, Error> {
    let spec = &model.spec;
    let (inner, channels, kernel) = (
        spec.ssm_inner,
        spec.ssm_conv_channels(),
        spec.ssm_conv_kernel,
    );
    let (heads, head_dim, n) = (spec.ssm_heads, spec.ssm_head_dim(), spec.ssm_state);
    let group_state = spec.ssm_groups * n;
    let heads_per_group = heads / spec.ssm_groups;
    let span = Span::new(Stage::MambaProjections);
    let projected =
        project_streamed_batch_with(file, b.input, input, MAX_REFERENCE_MATRIX_ELEMENTS, compute)?;
    drop(span);
    let span = Span::new(Stage::MambaScan);
    let conv_weight = load_all(file, b.conv_weight)?;
    let conv_bias = load_all(file, b.conv_bias)?;
    let dt_bias = load_all(file, b.dt_bias)?;
    let a = load_all(file, b.a)?;
    let d = load_all(file, b.d)?;
    let norm = load_all(file, b.norm)?;
    let group = inner / spec.ssm_groups;
    let mut gated = Vec::with_capacity(input.len());
    for p in &projected {
        let (z, rest) = p.split_at(inner);
        let (xbc, dt) = rest.split_at(channels);
        // Causal depthwise convolution over [state rows..., this row], then
        // SiLU, of channel c.
        let state_rows: &[f32] = conv_state;
        let conv = |c: usize| {
            let taps = &conv_weight[c * kernel..(c + 1) * kernel];
            let mut sum = f64::from(conv_bias[c]);
            for (k, &w) in taps[..kernel - 1].iter().enumerate() {
                sum += f64::from(w) * f64::from(state_rows[k * channels + c]);
            }
            sum += f64::from(taps[kernel - 1]) * f64::from(xbc[c]);
            silu(sum as f32)
        };
        // Heads are independent: a worker computes whole heads (their x
        // channels, their group's B and C, the scan and the gate). Every value
        // is computed by the same arithmetic in any split.
        let width = head_dim * n;
        // Each head's state is updated in place by the worker that owns it.
        let slots: Vec<std::sync::Mutex<&mut [f32]>> =
            ssm.chunks_mut(width).map(std::sync::Mutex::new).collect();
        // A group's B and C (after the convolution), computed once by the
        // first of its heads to need them.
        let group_bc: Vec<std::sync::OnceLock<(Vec<f32>, Vec<f32>)>> = (0..spec.ssm_groups)
            .map(|_| std::sync::OnceLock::new())
            .collect();
        let per_head = compute.map_positions(heads, |h| {
            let g = h / heads_per_group;
            let (bg, cg) = group_bc[g].get_or_init(|| {
                let at =
                    |base: usize| -> Vec<f32> { (0..n).map(|j| conv(base + g * n + j)).collect() };
                (at(inner), at(inner + group_state))
            });
            let delta = softplus(f64::from(dt[h]) + f64::from(dt_bias[h]));
            let decay = libm::exp(delta * f64::from(a[h]));
            let decay = decay as f32;
            let delta_b: Vec<f32> = bg.iter().map(|&b| (delta * f64::from(b)) as f32).collect();
            let mut next = slots[h].lock().unwrap_or_else(|e| e.into_inner());
            let mut y = vec![0.0f32; head_dim];
            for (q, row) in next.chunks_exact_mut(n).enumerate() {
                let xv = conv(h * head_dim + q);
                let yq = scan_row(row, decay, &delta_b, xv, cg) + d[h] * xv;
                // The gate of the gated norm: y * silu(z).
                y[q] = yq * silu(z[h * head_dim + q]);
            }
            Ok::<_, Error>(y)
        })?;
        drop(slots);
        conv_state.copy_within(channels.., 0);
        conv_state[(kernel - 2) * channels..].copy_from_slice(xbc);
        let mut v: Vec<f32> = per_head.concat();
        // Gated RMS norm: weight * group_rms(y * silu(z)).
        for (chunk, weight) in v.chunks_mut(group).zip(norm.chunks(group)) {
            chunk.copy_from_slice(&math::rms_norm(chunk, Some(weight), spec.rms_epsilon)?);
        }
        gated.push(v);
    }
    drop(span);
    let _span = Span::new(Stage::MambaProjections);
    project_streamed_batch_with(
        file,
        b.output,
        &gated,
        MAX_REFERENCE_MATRIX_ELEMENTS,
        compute,
    )
}

fn attention(
    file: &ModelFile,
    model: &ModelDefinition<'_>,
    b: &AttentionBindings<'_>,
    keys: &mut Vec<f32>,
    values: &mut Vec<f32>,
    input: &[Vec<f32>],
    compute: Compute<'_>,
) -> Result<Vec<Vec<f32>>, Error> {
    let spec = &model.spec;
    let span = Span::new(Stage::AttentionProjections);
    // One submission for all three (decode), as separate projections otherwise.
    let mut qkv = project_many(file, &[b.query, b.key, b.value], input, compute)?.into_iter();
    let (q, k, v) = (
        qkv.next().expect("query"),
        qkv.next().expect("key"),
        qkv.next().expect("value"),
    );
    drop(span);
    let span = Span::new(Stage::AttentionCore);
    let past = keys.len() / (spec.kv_heads * spec.head_dim);
    for (k, v) in k.iter().zip(&v) {
        keys.extend_from_slice(k);
        values.extend_from_slice(v);
    }
    let width = spec.kv_heads * spec.head_dim;
    // The kernel's attention scale is one: scale the queries by 1/sqrt(d).
    let scale = (1.0 / (spec.head_dim as f64).sqrt()) as f32;
    // Items of (position, range of query heads), so one position's heads
    // spread over the workers; each head is computed as by the whole call.
    const HEADS_PER_ITEM: usize = 4;
    let parts = spec.query_heads.div_ceil(HEADS_PER_ITEM);
    let pieces = compute.map_positions(q.len() * parts, |item| {
        let (i, part) = (item / parts, item % parts);
        let first = part * HEADS_PER_ITEM;
        let heads = first..(first + HEADS_PER_ITEM).min(spec.query_heads);
        let query: Vec<f32> = q[i].iter().map(|&x| x * scale).collect();
        let visible = (past + i + 1) * width;
        // Exact for the exact backends, binary32 for the integer ones.
        Ok::<_, Error>(compute.kernels.attention_heads(
            &query,
            &keys[..visible],
            &values[..visible],
            spec.kv_heads,
            spec.head_dim,
            None,
            Some(heads),
        )?)
    })?;
    let mixed: Vec<Vec<f32>> = pieces.chunks(parts).map(<[Vec<f32>]>::concat).collect();
    drop(span);
    let _span = Span::new(Stage::AttentionOutput);
    project_streamed_batch_with(
        file,
        b.output,
        &mixed,
        MAX_REFERENCE_MATRIX_ELEMENTS,
        compute,
    )
}

/// One state row of one head: `s[j] = s[j] * decay + (delta B[j]) x`, in
/// place, returning `sum_j s[j] C[j]`. Binary32, with eight interleaved
/// partial sums added in a fixed order (it vectorizes; every backend runs
/// this same code).
fn scan_row(row: &mut [f32], decay: f32, delta_b: &[f32], x: f32, c: &[f32]) -> f32 {
    let mut partial = [0.0f32; 8];
    let (rows, row_tail) = row.as_chunks_mut::<8>();
    let (bs, b_tail) = delta_b.as_chunks::<8>();
    let (cs, c_tail) = c.as_chunks::<8>();
    for ((s, b), c) in rows.iter_mut().zip(bs).zip(cs) {
        for i in 0..8 {
            s[i] = s[i] * decay + b[i] * x;
            partial[i] += s[i] * c[i];
        }
    }
    for (i, ((s, b), c)) in row_tail.iter_mut().zip(b_tail).zip(c_tail).enumerate() {
        *s = *s * decay + b * x;
        partial[i] += *s * c;
    }
    ((partial[0] + partial[1]) + (partial[2] + partial[3]))
        + ((partial[4] + partial[5]) + (partial[6] + partial[7]))
}

/// One position's routing: `(expert, weight)` in descending choice order.
pub fn route(
    logits: &[f32],
    correction_bias: &[f32],
    top_k: usize,
    scale: f32,
) -> Vec<(usize, f32)> {
    let scores: Vec<f64> = logits.iter().map(|&l| sigmoid(f64::from(l))).collect();
    let mut order: Vec<usize> = (0..scores.len()).collect();
    let choice = |e: usize| scores[e] as f32 + correction_bias[e];
    // Stable: ties keep the lower expert first.
    order.sort_by(|&a, &b| choice(b).total_cmp(&choice(a)));
    order.truncate(top_k);
    let sum: f64 = order.iter().map(|&e| scores[e]).sum::<f64>() + 1e-20;
    order
        .into_iter()
        .map(|e| (e, (scores[e] / sum * f64::from(scale)) as f32))
        .collect()
}

fn moe(
    file: &ModelFile,
    model: &ModelDefinition<'_>,
    b: &MoeBindings<'_>,
    input: &[Vec<f32>],
    compute: Compute<'_>,
) -> Result<Vec<Vec<f32>>, Error> {
    let spec = &model.spec;
    let hidden = spec.hidden;
    let span = Span::new(Stage::Router);
    // The F32 router stays expanded in memory (exact F64 dot products).
    let router = file.cached(&format!("nemotron-router:{}", b.router.name), || {
        load_matrix(file, b.router, MAX_REFERENCE_MATRIX_ELEMENTS)
    })?;
    // Row chunks of every position on the workers (each row's F64 dot
    // product is the same in any split).
    const ROUTER_ROWS: usize = 16;
    let experts = spec.expert_count;
    let chunks = experts.div_ceil(ROUTER_ROWS);
    let parts = compute.map_positions(input.len() * chunks, |i| {
        let (p, c) = (i / chunks, i % chunks);
        let first = c * ROUTER_ROWS;
        Ok::<_, Error>(router.project_rows(&input[p], first, ROUTER_ROWS.min(experts - first))?)
    })?;
    let router: Vec<Vec<f32>> = parts.chunks(chunks).map(<[Vec<f32>]>::concat).collect();
    let bias = load_all(file, b.correction_bias)?;
    let routes: Vec<_> = router
        .iter()
        .map(|l| route(l, &bias, spec.top_k, spec.routed_scale))
        .collect();
    drop(span);
    // Shared expert: down(relu(up(x))^2).
    let span = Span::new(Stage::SharedMlp);
    let up = project_streamed_batch_with(
        file,
        b.shared_up,
        input,
        MAX_REFERENCE_MATRIX_ELEMENTS,
        compute,
    )?;
    let up: Vec<Vec<f32>> = up
        .into_iter()
        .map(|r| r.into_iter().map(relu_squared).collect())
        .collect();
    let mut out = project_streamed_batch_with(
        file,
        b.shared_down,
        &up,
        MAX_REFERENCE_MATRIX_ELEMENTS,
        compute,
    )?;
    drop(span);
    let span = Span::new(Stage::Experts);
    // Routed experts, all of a layer's at once; contributions are
    // accumulated in F64 per position in expert order.
    let mut groups: Vec<(usize, Vec<(usize, f32)>)> = Vec::new();
    for expert in 0..spec.expert_count {
        let users: Vec<(usize, f32)> = routes
            .iter()
            .enumerate()
            .filter_map(|(i, r)| r.iter().find(|(e, _)| *e == expert).map(|&(_, w)| (i, w)))
            .collect();
        if !users.is_empty() {
            groups.push((expert, users));
        }
    }
    let experts = routed_experts(
        file,
        b.expert_up,
        None,
        b.expert_down,
        ExpertActivation::ReluSquared,
        &groups,
        input,
        compute,
    )?;
    drop(span);
    let _span = Span::new(Stage::ExpertCombine);
    let mut routed = vec![vec![0.0f64; hidden]; input.len()];
    for ((_, users), rows) in groups.iter().zip(&experts) {
        for (&(i, w), d) in users.iter().zip(rows) {
            for (acc, &v) in routed[i].iter_mut().zip(d) {
                *acc += f64::from(w) * f64::from(v);
            }
        }
    }
    for (o, r) in out.iter_mut().zip(&routed) {
        for (o, &r) in o.iter_mut().zip(r) {
            *o += r as f32;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests;
