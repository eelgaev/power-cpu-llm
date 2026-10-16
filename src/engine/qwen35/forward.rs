//! Reference forward pass of a bound Qwen 3.5/3.6 MoE model, over the
//! batched streamed projections of any matrix backend. Every layer is
//! `x += mixer(norm(x)); x += moe(ffn_norm(x))`. Gated DeltaNet runs its
//! recurrent form one position at a time (value heads on the workers); text
//! positions make IMRoPE the split-half RoPE of each head's first
//! `rope_dims` channels. The MTP block is not executed.

use super::{AttentionBindings, LayerKind, LinearBindings, Mixer, ModelDefinition, MoeBindings};
use crate::engine::execution::Compute;
use crate::engine::operators::{
    self, Error, ExpertActivation, MAX_REFERENCE_MATRIX_ELEMENTS, project_many,
    project_streamed_batch_with, routed_experts,
};
use crate::engine::stats::{Span, Stage};
use crate::engine::weights::{ModelFile, row_count};
use crate::gguf::TensorInfo;
use crate::kernels::reference as math;
use crate::kernels::{deltanet, dot_tile};
use crate::libm;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
enum LayerState {
    Linear {
        /// The last `kernel - 1` convolution inputs, oldest first.
        conv: Vec<f32>,
        /// Per value head `[value channel][key channel]`.
        state: Vec<f32>,
    },
    Attention {
        keys: Vec<f32>,
        values: Vec<f32>,
    },
}

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
                LayerKind::Linear => LayerState::Linear {
                    conv: vec![0.0; (spec.conv_kernel - 1) * spec.conv_channels()],
                    state: vec![0.0; spec.value_heads * spec.state * spec.state],
                },
                LayerKind::Attention => LayerState::Attention {
                    keys: Vec::new(),
                    values: Vec::new(),
                },
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

fn load_all(file: &ModelFile, tensor: &TensorInfo) -> Result<Arc<Vec<f32>>, Error> {
    file.cached(&format!("all:{}", tensor.name), || {
        let mut out = Vec::new();
        for row in 0..row_count(tensor)? {
            out.extend(file.read_row(&tensor.name, row)?.decode()?);
        }
        if out.len() > MAX_REFERENCE_MATRIX_ELEMENTS {
            return Err(Error::ResourceLimit);
        }
        Ok(out)
    })
}
/// A small F32/BF16 matrix kept expanded (row-major).
fn cached_matrix(file: &ModelFile, tensor: &TensorInfo) -> Result<Arc<Vec<f32>>, Error> {
    load_all(file, tensor)
}
/// `sum(w[i] x[i])` in F64 with eight interleaved partial sums added in a
/// fixed order (it vectorizes; deterministic on every backend).
fn dot_f64(w: &[f32], x: &[f32]) -> f64 {
    let mut partial = [0.0f64; 8];
    let (w8, w_tail) = w.as_chunks::<8>();
    let (x8, x_tail) = x.as_chunks::<8>();
    for (w, x) in w8.iter().zip(x8) {
        for i in 0..8 {
            partial[i] += f64::from(w[i]) * f64::from(x[i]);
        }
    }
    for (i, (w, x)) in w_tail.iter().zip(x_tail).enumerate() {
        partial[i] += f64::from(*w) * f64::from(*x);
    }
    ((partial[0] + partial[1]) + (partial[2] + partial[3]))
        + ((partial[4] + partial[5]) + (partial[6] + partial[7]))
}
/// Every row of a small row-major matrix times each input, row chunks on
/// the workers.
fn small_projection(
    matrix: &[f32],
    input: &[Vec<f32>],
    compute: Compute<'_>,
) -> Result<Vec<Vec<f32>>, Error> {
    const ROWS: usize = 16;
    let cols = input.first().map_or(1, Vec::len);
    if cols == 0 || !matrix.len().is_multiple_of(cols) {
        return Err(Error::Shape("small projection width"));
    }
    let rows = matrix.len() / cols;
    let chunks = rows.div_ceil(ROWS);
    // Exact (F64) for the exact backends, binary32 for the integer ones.
    let approximate = compute.kernels.approximate();
    if approximate && input.len() >= 4 && rows.is_multiple_of(4) {
        // Groups of 4 inputs (the last one overlapping) x row chunks, in
        // 4 x 4 tiles.
        let groups = input.len().div_ceil(4);
        let parts = compute.map_positions(groups * chunks, |i| {
            let (g, c) = (i / chunks, i % chunks);
            let p0 = (4 * g).min(input.len() - 4);
            let x: [&[f32]; 4] = std::array::from_fn(|k| input[p0 + k].as_slice());
            let end = ((c + 1) * ROWS).min(rows);
            let mut out = vec![[0.0f32; 4]; end - c * ROWS];
            for r0 in (c * ROWS..end).step_by(4) {
                let tile = dot_tile::dot_f32_tile(matrix, r0, x);
                for k in 0..4 {
                    out[r0 + k - c * ROWS] = tile[k];
                }
            }
            Ok::<_, Error>((p0, out))
        })?;
        let mut rows_out = vec![vec![0.0f32; rows]; input.len()];
        for (i, (p0, out)) in parts.into_iter().enumerate() {
            let c = i % chunks;
            for (r, values) in out.iter().enumerate() {
                for (k, &v) in values.iter().enumerate() {
                    rows_out[p0 + k][c * ROWS + r] = v;
                }
            }
        }
        return Ok(rows_out);
    }
    let parts = compute.map_positions(input.len() * chunks, |i| {
        let (p, c) = (i / chunks, i % chunks);
        let end = ((c + 1) * ROWS).min(rows);
        Ok::<_, Error>(
            (c * ROWS..end)
                .map(|r| {
                    let row = &matrix[r * cols..(r + 1) * cols];
                    if approximate {
                        dot_tile::dot_f32(row, &input[p])
                    } else {
                        dot_f64(row, &input[p]) as f32
                    }
                })
                .collect::<Vec<f32>>(),
        )
    })?;
    Ok(parts.chunks(chunks).map(<[Vec<f32>]>::concat).collect())
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
/// `x / max(||x||, eps)` (ggml_l2_norm).
fn l2_norm(x: &mut [f32], eps: f32) {
    let sum: f64 = x.iter().map(|&v| f64::from(v) * f64::from(v)).sum();
    let scale = (1.0 / sum.sqrt().max(f64::from(eps))) as f32;
    for v in x {
        *v *= scale;
    }
}
fn add_rows(x: &mut [Vec<f32>], delta: &[Vec<f32>]) {
    for (row, d) in x.iter_mut().zip(delta) {
        for (a, b) in row.iter_mut().zip(d) {
            *a += b;
        }
    }
}

/// Packs every projection weight once for the packing (int8) backends.
pub fn prepack(
    file: &ModelFile,
    model: &ModelDefinition<'_>,
    compute: Compute<'_>,
) -> Result<(), Error> {
    let mut tensors: Vec<&TensorInfo> = vec![model.output];
    for layer in &model.layers {
        match &layer.mixer {
            Mixer::Linear(b) => tensors.extend([b.qkv, b.gate, b.output]),
            Mixer::Attention(b) => tensors.extend([b.query, b.key, b.value, b.output]),
        }
        let m = &layer.moe;
        tensors.extend([
            m.expert_gate,
            m.expert_up,
            m.expert_down,
            m.shared_gate,
            m.shared_up,
            m.shared_down,
        ]);
    }
    operators::prepack(file, &tensors, compute)
}

/// Runs `tokens` after the positions already in `state`; returns the final
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
    let first_position = state.positions;
    let norm = |rows: &[Vec<f32>], tensor| -> Result<Vec<Vec<f32>>, Error> {
        let weight = load_all(file, tensor)?;
        compute.map_positions(rows.len(), |i| {
            Ok::<_, Error>(math::rms_norm(&rows[i], Some(&weight), spec.rms_epsilon)?)
        })
    };
    for (index, (layer, layer_state)) in model.layers.iter().zip(&mut state.layers).enumerate() {
        let normed = norm(&x, layer.norm)?;
        let delta = match (&layer.mixer, layer_state) {
            (Mixer::Linear(b), LayerState::Linear { conv, state }) => {
                linear(file, model, b, conv, state, &normed, compute)?
            }
            (Mixer::Attention(b), LayerState::Attention { keys, values }) => attention(
                file,
                model,
                b,
                keys,
                values,
                first_position,
                &normed,
                compute,
            )?,
            _ => return Err(Error::Shape("state does not match the layer kind")),
        };
        add_rows(&mut x, &delta);
        let normed = norm(&x, layer.ffn_norm)?;
        let delta = moe(file, model, &layer.moe, &normed, compute)?;
        add_rows(&mut x, &delta);
        if !x.iter().all(|r| math::all_finite(r)) {
            return Err(Error::Math(math::Error::NonFiniteValue));
        }
        observe(index, &x);
    }
    state.positions += tokens.len();
    Ok(x)
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

fn linear(
    file: &ModelFile,
    model: &ModelDefinition<'_>,
    b: &LinearBindings<'_>,
    conv_state: &mut [f32],
    ssm: &mut [f32],
    input: &[Vec<f32>],
    compute: Compute<'_>,
) -> Result<Vec<Vec<f32>>, Error> {
    let spec = &model.spec;
    let (n, kernel, channels) = (spec.state, spec.conv_kernel, spec.conv_channels());
    let (key_width, key_heads) = (spec.key_width(), spec.key_heads);
    let span = Span::new(Stage::MambaProjections);
    let mut projected = project_many(file, &[b.qkv, b.gate], input, compute)?.into_iter();
    let (qkv, z) = (
        projected.next().expect("qkv"),
        projected.next().expect("gate"),
    );
    // beta and alpha stacked: one submission, each row computed as alone.
    let stacked = file.cached(&format!("beta+alpha:{}", b.beta.name), || {
        let mut m = cached_matrix(file, b.beta)?.as_ref().clone();
        m.extend_from_slice(&cached_matrix(file, b.alpha)?);
        Ok::<_, Error>(m)
    })?;
    let heads = spec.value_heads;
    let (beta, alpha): (Vec<Vec<f32>>, Vec<Vec<f32>>) = small_projection(&stacked, input, compute)?
        .into_iter()
        .map(|mut row| {
            let alpha = row.split_off(heads);
            (row, alpha)
        })
        .unzip();
    drop(span);
    let span = Span::new(Stage::MambaScan);
    let conv_weight = load_all(file, b.conv_weight)?;
    let dt_bias = load_all(file, b.dt_bias)?;
    let a = load_all(file, b.a)?;
    let norm = load_all(file, b.norm)?;
    let scale = (1.0 / (n as f64).sqrt()) as f32;
    let eps = spec.rms_epsilon;
    let positions = input.len();
    let state_rows: &[f32] = conv_state;
    // Causal depthwise convolution (no bias), then SiLU, of channel c at
    // position p: taps over the previous kernel - 1 inputs (the carried
    // state before position 0), then the current one.
    let conv = |c: usize, p: usize| {
        let taps = &conv_weight[c * kernel..(c + 1) * kernel];
        let mut sum = 0.0f64;
        for (t, &w) in taps[..kernel - 1].iter().enumerate() {
            // Input position p - (kernel - 1) + t; the state's row r holds
            // position r - (kernel - 1).
            let back = p + t;
            let x = if back < kernel - 1 {
                state_rows[back * channels + c]
            } else {
                qkv[back - (kernel - 1)][c]
            };
            sum += f64::from(w) * f64::from(x);
        }
        sum += f64::from(taps[kernel - 1]) * f64::from(qkv[p][c]);
        silu(sum as f32)
    };
    let slots: Vec<Mutex<&mut [f32]>> = ssm.chunks_mut(n * n).map(Mutex::new).collect();
    // One item per key head over every position: its tiled value heads
    // j = kh, kh + key_heads, ... share its q and k, and their states stay
    // in the worker's cache for the whole prompt (one submission per layer;
    // each head's recurrence runs in position order, as before).
    let per_key = compute.map_positions(key_heads, |kh| {
        let mut states: Vec<_> = (kh..spec.value_heads)
            .step_by(key_heads)
            .map(|j| slots[j].lock().unwrap_or_else(|e| e.into_inner()))
            .collect();
        let mut rows = Vec::with_capacity(positions);
        for p in 0..positions {
            let mut q: Vec<f32> = (0..n).map(|i| conv(kh * n + i, p)).collect();
            let mut k: Vec<f32> = (0..n).map(|i| conv(key_width + kh * n + i, p)).collect();
            l2_norm(&mut q, eps);
            l2_norm(&mut k, eps);
            let mut heads = Vec::with_capacity(states.len());
            for (t, m) in states.iter_mut().enumerate() {
                let j = kh + t * key_heads;
                let v: Vec<f32> = (0..n).map(|i| conv(2 * key_width + j * n + i, p)).collect();
                let g = softplus(f64::from(alpha[p][j]) + f64::from(dt_bias[j])) * f64::from(a[j]);
                let decay = libm::exp(g) as f32;
                let beta = sigmoid(f64::from(beta[p][j])) as f32;
                let mut out = vec![0.0f32; n];
                // m[jv][i] = S[i][jv]: decay, delta rule, read out.
                deltanet::head_update(m, &k, &q, &v, decay, beta, scale, &mut out);
                // Gated norm: rms_norm(o) * silu(z).
                let normed = math::rms_norm(&out, Some(&norm), eps)?;
                let zh = &z[p][j * n..(j + 1) * n];
                heads.push(
                    normed
                        .iter()
                        .zip(zh)
                        .map(|(&o, &zz)| o * silu(zz))
                        .collect::<Vec<f32>>(),
                );
            }
            rows.push(heads);
        }
        Ok::<_, Error>(rows)
    })?;
    drop(slots);
    let mut gated = vec![vec![0.0f32; spec.value_heads * n]; positions];
    for (kh, rows) in per_key.iter().enumerate() {
        for (row, heads) in gated.iter_mut().zip(rows) {
            for (t, head) in heads.iter().enumerate() {
                let j = kh + t * key_heads;
                row[j * n..(j + 1) * n].copy_from_slice(head);
            }
        }
    }
    // The carried state: the last kernel - 1 inputs, oldest first (row t
    // holds position positions - keep + t: a later state row, or an input).
    let keep = kernel - 1;
    for t in 0..keep {
        let at = positions + t;
        if at < keep {
            conv_state.copy_within(at * channels..(at + 1) * channels, t * channels);
        } else {
            conv_state[t * channels..(t + 1) * channels].copy_from_slice(&qkv[at - keep]);
        }
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

#[allow(clippy::too_many_arguments)]
fn attention(
    file: &ModelFile,
    model: &ModelDefinition<'_>,
    b: &AttentionBindings<'_>,
    keys: &mut Vec<f32>,
    values: &mut Vec<f32>,
    first_position: usize,
    input: &[Vec<f32>],
    compute: Compute<'_>,
) -> Result<Vec<Vec<f32>>, Error> {
    let spec = &model.spec;
    let (d, heads, kv) = (spec.head_dim, spec.query_heads, spec.kv_heads);
    let span = Span::new(Stage::AttentionProjections);
    let mut qkv = project_many(file, &[b.query, b.key, b.value], input, compute)?.into_iter();
    let (qg, k, v) = (
        qkv.next().expect("query"),
        qkv.next().expect("key"),
        qkv.next().expect("value"),
    );
    let q_norm = load_all(file, b.query_norm)?;
    let k_norm = load_all(file, b.key_norm)?;
    let eps = spec.rms_epsilon;
    let rotate = |head: &mut [f32], position: usize| -> Result<(), Error> {
        let r = spec.rope_dims;
        let rotated = math::rope(&head[..r], r, position, spec.rope_base, None)?;
        head[..r].copy_from_slice(&rotated);
        Ok(())
    };
    let scale = (1.0 / (d as f64).sqrt()) as f32;
    // Per position: rotated, scaled queries and their gates; the head norms
    // and rotations on the workers (query heads, then key heads, in items
    // of NORM_HEADS heads).
    const NORM_HEADS: usize = 2;
    let parts = (heads + kv).div_ceil(NORM_HEADS);
    let normed = compute.map_positions(input.len() * parts, |item| {
        let (p, part) = (item / parts, item % parts);
        let position = first_position + p;
        let mut out = Vec::with_capacity(NORM_HEADS * d);
        for h in part * NORM_HEADS..((part + 1) * NORM_HEADS).min(heads + kv) {
            if h < heads {
                let block = &qg[p][h * 2 * d..h * 2 * d + d];
                let mut q = math::rms_norm(block, Some(&q_norm), eps)?;
                rotate(&mut q, position)?;
                out.extend(q.iter().map(|&x| x * scale));
            } else {
                let g = h - heads;
                let mut key = math::rms_norm(&k[p][g * d..(g + 1) * d], Some(&k_norm), eps)?;
                rotate(&mut key, position)?;
                out.extend_from_slice(&key);
            }
        }
        Ok::<_, Error>(out)
    })?;
    let mut queries = Vec::with_capacity(input.len());
    let mut gates = Vec::with_capacity(input.len());
    for (p, (row, normed)) in qg.iter().zip(normed.chunks(parts)).enumerate() {
        let normed = normed.concat();
        queries.push(normed[..heads * d].to_vec());
        let mut gate = Vec::with_capacity(heads * d);
        for h in 0..heads {
            gate.extend_from_slice(&row[h * 2 * d + d..(h + 1) * 2 * d]);
        }
        gates.push(gate);
        keys.extend_from_slice(&normed[heads * d..]);
        values.extend_from_slice(&v[p]);
    }
    drop(span);
    let span = Span::new(Stage::AttentionCore);
    let width = kv * d;
    let past = keys.len() / width - input.len();
    const HEADS_PER_ITEM: usize = 2;
    let parts = heads.div_ceil(HEADS_PER_ITEM);
    let pieces = compute.map_positions(input.len() * parts, |item| {
        let (i, part) = (item / parts, item % parts);
        let first = part * HEADS_PER_ITEM;
        let visible = (past + i + 1) * width;
        let range = first..(first + HEADS_PER_ITEM).min(heads);
        let gate = &gates[i][range.start * d..range.end * d];
        let mixed = compute.kernels.attention_heads(
            &queries[i],
            &keys[..visible],
            &values[..visible],
            kv,
            d,
            None,
            Some(range),
        )?;
        // The output gate, on the workers.
        Ok::<_, Error>(
            mixed
                .iter()
                .zip(gate)
                .map(|(&o, &g)| (f64::from(o) * sigmoid(f64::from(g))) as f32)
                .collect::<Vec<f32>>(),
        )
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

/// One position's routing: softmax over all experts, the `top_k` most
/// probable (ties to the lower expert), weights renormalized over them.
pub fn route(logits: &[f32], top_k: usize) -> Vec<(usize, f32)> {
    let max = logits.iter().fold(f32::NEG_INFINITY, |m, &l| m.max(l));
    let weights: Vec<f64> = logits
        .iter()
        .map(|&l| libm::exp(f64::from(l) - f64::from(max)))
        .collect();
    let total: f64 = weights.iter().sum();
    let probabilities: Vec<f32> = weights.iter().map(|&w| (w / total) as f32).collect();
    let mut order: Vec<usize> = (0..logits.len()).collect();
    order.sort_by(|&a, &b| probabilities[b].total_cmp(&probabilities[a]));
    order.truncate(top_k);
    let sum: f64 = order.iter().map(|&e| f64::from(probabilities[e])).sum();
    order
        .into_iter()
        .map(|e| (e, (f64::from(probabilities[e]) / sum) as f32))
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
    let span = Span::new(Stage::Router);
    let router = small_projection(&cached_matrix(file, b.router)?, input, compute)?;
    let routes = compute.map_positions(router.len(), |p| {
        Ok::<_, Error>(route(&router[p], spec.top_k))
    })?;
    let shared_gate_input = load_all(file, b.shared_gate_input)?;
    drop(span);
    let span = Span::new(Stage::SharedMlp);
    // The shared expert as a one-expert stack through the batched expert path
    // (its activation runs on the workers).
    let all: Vec<(usize, f32)> = (0..input.len()).map(|i| (i, 1.0)).collect();
    let shared = routed_experts(
        file,
        b.shared_gate,
        Some(b.shared_up),
        b.shared_down,
        ExpertActivation::SiluProduct,
        &[(0, all)],
        input,
        compute,
    )?
    .pop()
    .expect("one group");
    drop(span);
    let span = Span::new(Stage::Experts);
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
        b.expert_gate,
        Some(b.expert_up),
        b.expert_down,
        ExpertActivation::SiluProduct,
        &groups,
        input,
        compute,
    )?;
    drop(span);
    let _span = Span::new(Stage::ExpertCombine);
    // Per position, its (group, user) entries in group order: each output
    // element adds its experts in that order, as one serial loop would.
    let mut uses: Vec<Vec<(usize, usize)>> = vec![Vec::new(); input.len()];
    for (g, (_, users)) in groups.iter().enumerate() {
        for (k, &(i, _)) in users.iter().enumerate() {
            uses[i].push((g, k));
        }
    }
    // On the workers: column chunks of each position (the shared-expert
    // gate dot, serial over all columns, is repeated by each chunk).
    const COMBINE_COLUMNS: usize = 256;
    let parts = spec.hidden.div_ceil(COMBINE_COLUMNS);
    let pieces = compute.map_positions(input.len() * parts, |item| {
        let (i, part) = (item / parts, item % parts);
        let columns = part * COMBINE_COLUMNS..((part + 1) * COMBINE_COLUMNS).min(spec.hidden);
        let dot: f64 = shared_gate_input
            .iter()
            .zip(&input[i])
            .map(|(&w, &v)| f64::from(w) * f64::from(v))
            .sum();
        let s = sigmoid(dot);
        let mut routed = vec![0.0f64; columns.len()];
        for &(g, k) in &uses[i] {
            let w = f64::from(groups[g].1[k].1);
            for (acc, &v) in routed.iter_mut().zip(&experts[g][k][columns.clone()]) {
                *acc += w * f64::from(v);
            }
        }
        Ok::<_, Error>(
            routed
                .iter()
                .zip(&shared[i][columns])
                .map(|(&r, &sh)| (r + s * f64::from(sh)) as f32)
                .collect::<Vec<f32>>(),
        )
    })?;
    Ok(pieces.chunks(parts).map(<[Vec<f32>]>::concat).collect())
}
