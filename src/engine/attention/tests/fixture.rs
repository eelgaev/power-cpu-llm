//! Independently serialized tiny F32 GGUF and scalar Rust whole-block oracle.
//! No production math wrappers are used to compute expected hidden states.

use super::*;
use crate::gguf::{ArrayValues, MetadataValue};
use std::{
    collections::BTreeMap,
    io::Write,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

pub(crate) struct Fixture {
    pub path: PathBuf,
    weights: BTreeMap<String, Vec<f32>>,
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
fn string(out: &mut Vec<u8>, s: &str) {
    out.extend_from_slice(&(s.len() as u64).to_le_bytes());
    out.extend_from_slice(s.as_bytes());
}
fn metadata(out: &mut Vec<u8>, value: &MetadataValue) {
    match value {
        MetadataValue::Unsigned(n) => {
            out.extend_from_slice(&10u32.to_le_bytes());
            out.extend_from_slice(&n.to_le_bytes());
        }
        MetadataValue::Signed(n) => {
            out.extend_from_slice(&11u32.to_le_bytes());
            out.extend_from_slice(&n.to_le_bytes());
        }
        MetadataValue::Float(n) => {
            out.extend_from_slice(&12u32.to_le_bytes());
            out.extend_from_slice(&n.to_le_bytes());
        }
        MetadataValue::Bool(b) => {
            out.extend_from_slice(&7u32.to_le_bytes());
            out.push(u8::from(*b));
        }
        MetadataValue::String(s) => {
            out.extend_from_slice(&8u32.to_le_bytes());
            string(out, s);
        }
        MetadataValue::Array { values, .. } => {
            out.extend_from_slice(&9u32.to_le_bytes());
            let (kind, len) = match values {
                ArrayValues::Unsigned(v) => (10u32, v.len()),
                ArrayValues::Signed(v) => (11, v.len()),
                ArrayValues::Float(v) => (12, v.len()),
                ArrayValues::Bool(v) => (7, v.len()),
                ArrayValues::String(v) => (8, v.len()),
            };
            out.extend_from_slice(&kind.to_le_bytes());
            out.extend_from_slice(&(len as u64).to_le_bytes());
            match values {
                ArrayValues::Unsigned(v) => {
                    for n in v {
                        out.extend_from_slice(&n.to_le_bytes());
                    }
                }
                ArrayValues::Signed(v) => {
                    for n in v {
                        out.extend_from_slice(&n.to_le_bytes());
                    }
                }
                ArrayValues::Float(v) => {
                    for n in v {
                        out.extend_from_slice(&n.to_le_bytes());
                    }
                }
                ArrayValues::Bool(v) => out.extend(v.iter().map(|&b| u8::from(b))),
                ArrayValues::String(v) => {
                    for s in v {
                        string(out, s);
                    }
                }
            }
        }
    }
}
impl Fixture {
    pub fn new() -> Self {
        Self::with_embedding_shape(4, 8)
    }
    /// Operator-only variants may intentionally fail full model binding; the
    /// serialized F32 tensor layouts still form valid immutable GGUF files.
    pub fn with_embedding_shape(cols: u64, rows: u64) -> Self {
        let mut inventory = crate::engine::model::tests::inventory();
        inventory
            .tensors
            .iter_mut()
            .find(|t| t.name == "token_embd.weight")
            .unwrap()
            .shape = [cols, rows, 1, 1];
        inventory.metadata.insert(
            "diffusion-gemma.attention.sliding_window".into(),
            MetadataValue::Unsigned(3),
        );
        let mut bytes = b"GGUF".to_vec();
        bytes.extend_from_slice(&3u32.to_le_bytes());
        bytes.extend_from_slice(&(inventory.tensors.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&(inventory.metadata.len() as u64).to_le_bytes());
        for (key, value) in &inventory.metadata {
            string(&mut bytes, key);
            metadata(&mut bytes, value);
        }
        let mut data = Vec::new();
        let mut weights = BTreeMap::new();
        for tensor in &inventory.tensors {
            let name = &tensor.name;
            let count = tensor.shape.iter().product::<u64>() as usize;
            let seed: usize = name
                .bytes()
                .enumerate()
                .map(|(i, c)| (i + 1) * usize::from(c))
                .sum();
            let values: Vec<_> = (0..count)
                .map(|i| {
                    if name.contains("enc_layer_output_scale") {
                        0.7
                    } else if name.contains("layer_output_scale") {
                        1.15
                    } else if name == "rope_freqs.weight" {
                        if i == 0 { 1.0 } else { 1e30 }
                    } else if name.contains("norm") || name.ends_with(".scale") {
                        0.6 + (i % 7) as f32 * 0.11
                    } else {
                        ((seed + i * 13) % 37) as f32 / 29.0 - 0.62
                    }
                })
                .collect();
            data.resize(data.len().div_ceil(32) * 32, 0);
            string(&mut bytes, name);
            let dims = tensor
                .shape
                .iter()
                .rposition(|&d| d != 1)
                .map_or(1, |i| i + 1);
            bytes.extend_from_slice(&(dims as u32).to_le_bytes());
            for d in &tensor.shape[..dims] {
                bytes.extend_from_slice(&d.to_le_bytes());
            }
            bytes.extend_from_slice(&0u32.to_le_bytes());
            bytes.extend_from_slice(&(data.len() as u64).to_le_bytes());
            for n in &values {
                data.extend_from_slice(&n.to_le_bytes());
            }
            weights.insert(name.clone(), values);
        }
        bytes.resize(bytes.len().div_ceil(32) * 32, 0);
        bytes.extend(data);
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "power-block-{}-{}.gguf",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        file.write_all(&bytes).unwrap();
        Self { path, weights }
    }
    fn project(&self, name: &str, input: &[f32], expert: Option<usize>) -> Vec<f32> {
        let w = &self.weights[name];
        let slice = if let Some(e) = expert {
            let size = w.len() / 3;
            &w[e * size..(e + 1) * size]
        } else {
            w.as_slice()
        };
        slice
            .chunks_exact(input.len())
            .map(|row| {
                row.iter()
                    .zip(input)
                    .map(|(&w, &x)| f64::from(w) * f64::from(x))
                    .sum::<f64>() as f32
            })
            .collect()
    }
    fn norm(&self, x: &[f32], name: &str) -> Vec<f32> {
        norm(x, Some(&self.weights[name]))
    }

    pub fn raw_embedding(&self, token: u32) -> Vec<f32> {
        self.weights["token_embd.weight"][token as usize * 4..(token as usize + 1) * 4].to_vec()
    }
    pub fn expected_soft(&self, logits: &[f32]) -> Vec<Vec<f32>> {
        logits
            .as_chunks::<8>()
            .0
            .iter()
            .map(|row| {
                let probabilities = softmax(row);
                (0..4)
                    .map(|d| {
                        (0..8)
                            .map(|t| {
                                f64::from(probabilities[t])
                                    * f64::from(self.weights["token_embd.weight"][t * 4 + d])
                            })
                            .sum::<f64>() as f32
                            * 2.0 // sqrt(hidden=4), official decoder:1281
                    })
                    .collect()
            })
            .collect()
    }
    /// Independent two-layer token-ID encoder/decoder, zero or nonzero SC,
    /// final normalization, tied output head and logit softcap.
    pub fn expected_text(
        &self,
        model: &ModelDefinition<'_>,
        prompt: &[u32],
        canvas: &[u32],
        previous: Option<&[f32]>,
    ) -> (Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<f32>) {
        let embed = |tokens: &[u32]| {
            tokens
                .iter()
                .map(|&id| {
                    self.raw_embedding(id)
                        .iter()
                        .map(|x| x * 2.0)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        };
        let mut encoder = embed(prompt);
        let soft = previous.map(|l| self.expected_soft(l));
        let mut decoder: Vec<_> = embed(canvas)
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let zero = vec![0.0; 4];
                let signal = soft.as_ref().map_or(zero.as_slice(), |s| s[i].as_slice());
                let signal = self.norm(signal, "self_cond_pre_norm.weight");
                let signal = self.project(
                    "self_cond_down.weight",
                    &gelu_product(
                        &self.project("self_cond_gate.weight", &signal, None),
                        &self.project("self_cond_up.weight", &signal, None),
                    ),
                    None,
                );
                norm(&add(row, &signal), None)
            })
            .collect();
        for layer in 0..model.layers.len() {
            let combined: Vec<_> = encoder.iter().chain(&decoder).cloned().collect();
            let result = self.expected(model, layer, &combined, prompt.len());
            encoder = result[..prompt.len()].to_vec();
            decoder = result[prompt.len()..].to_vec();
        }
        let encoder: Vec<_> = encoder
            .iter()
            .map(|x| self.norm(x, "output_norm.weight"))
            .collect();
        let decoder: Vec<_> = decoder
            .iter()
            .map(|x| self.norm(x, "output_norm.weight"))
            .collect();
        let logits = decoder
            .iter()
            .flat_map(|x| {
                self.project("token_embd.weight", x, None)
                    .iter()
                    .map(|&x| (30.0 * crate::libm::tanh(f64::from(x) / 30.0)) as f32)
                    .collect::<Vec<_>>()
            })
            .collect();
        (encoder, decoder, logits)
    }

    /// Region-aware whole-block scalar oracle. Prompt queries are causal and
    /// window-clipped; canvas sees all canvas plus last window-1 prompt (local).
    pub fn expected(
        &self,
        model: &ModelDefinition<'_>,
        layer: usize,
        input: &[Vec<f32>],
        prompt: usize,
    ) -> Vec<Vec<f32>> {
        let g = &model.spec.attention[layer];
        let local = g.kind == AttentionKind::Sliding;
        let prefix = format!("blk.{layer}.");
        let name = |s: &str| format!("{prefix}{s}");
        let transform = |x: &[f32], scale: Option<&[f32]>, pos: usize, rotate: bool| {
            x.chunks_exact(g.head_dim)
                .flat_map(|head| {
                    let mut y = norm(head, scale);
                    if rotate {
                        let half = g.head_dim / 2;
                        for i in 0..half {
                            let base = if local { 1e4_f32 } else { 1e6_f32 };
                            let mut inv =
                                1.0 / crate::libm::powf(base, 2.0 * i as f32 / g.head_dim as f32);
                            if !local {
                                inv /= self.weights["rope_freqs.weight"][i];
                            }
                            let (s, c) = crate::libm::sincosf(pos as f32 * inv);
                            let a = y[i];
                            let b = y[i + half];
                            y[i] = a * c - b * s;
                            y[i + half] = b * c + a * s;
                        }
                    }
                    y
                })
                .collect::<Vec<_>>()
        };
        let mut queries = Vec::new();
        let mut keys = Vec::new();
        let mut values = Vec::new();
        for (pos, row) in input.iter().enumerate() {
            let x = self.norm(row, &name("attn_norm.weight"));
            let q = self.project(&name("attn_q.weight"), &x, None);
            let k = self.project(&name("attn_k.weight"), &x, None);
            let v = if local {
                self.project(&name("attn_v.weight"), &x, None)
            } else {
                k.clone()
            };
            queries.push(transform(
                &q,
                Some(&self.weights[&name("attn_q_norm.weight")]),
                pos,
                true,
            ));
            keys.push(transform(
                &k,
                Some(&self.weights[&name("attn_k_norm.weight")]),
                pos,
                true,
            ));
            values.push(transform(&v, None, pos, false));
        }
        (0..input.len())
            .map(|q| {
                let mut attended = vec![0.0; g.query_heads * g.head_dim];
                for h in 0..g.query_heads {
                    let kh = h / (g.query_heads / g.kv_heads);
                    let scores: Vec<_> = (0..input.len())
                        .map(|k| {
                            let allowed = if q < prompt {
                                k <= q && (!local || q - k < 3)
                            } else {
                                !local || k >= prompt.saturating_sub(2)
                            };
                            if !allowed {
                                return f32::NEG_INFINITY;
                            }
                            (0..g.head_dim)
                                .map(|d| {
                                    f64::from(queries[q][h * g.head_dim + d])
                                        * f64::from(keys[k][kh * g.head_dim + d])
                                })
                                .sum::<f64>() as f32
                        })
                        .collect();
                    let probabilities = softmax(&scores);
                    for d in 0..g.head_dim {
                        attended[h * g.head_dim + d] = probabilities
                            .iter()
                            .zip(&values)
                            .map(|(&p, v)| f64::from(p) * f64::from(v[kh * g.head_dim + d]))
                            .sum::<f64>()
                            as f32;
                    }
                }
                let projected = self.project(&name("attn_output.weight"), &attended, None);
                let residual = add(
                    &input[q],
                    &self.norm(&projected, &name("post_attention_norm.weight")),
                );
                let shared_input = self.norm(&residual, &name("ffn_norm.weight"));
                let shared = self.project(
                    &name("ffn_down.weight"),
                    &gelu_product(
                        &self.project(&name("ffn_gate.weight"), &shared_input, None),
                        &self.project(&name("ffn_up.weight"), &shared_input, None),
                    ),
                    None,
                );
                let shared = self.norm(&shared, &name("post_ffw_norm_1.weight"));
                let router: Vec<_> = norm(&residual, None)
                    .iter()
                    .zip(&self.weights[&name("ffn_gate_inp.scale")])
                    .map(|(&x, &s)| x * s * (1.0 / (residual.len() as f32).sqrt()))
                    .collect();
                let probabilities =
                    softmax(&self.project(&name("ffn_gate_inp.weight"), &router, None));
                let mut order: Vec<_> = (0..probabilities.len()).collect();
                order.sort_by(|&a, &b| {
                    probabilities[b]
                        .total_cmp(&probabilities[a])
                        .then(a.cmp(&b))
                });
                order.truncate(2);
                let sum: f64 = order.iter().map(|&i| f64::from(probabilities[i])).sum();
                order.sort_unstable();
                let expert_input = self.norm(&residual, &name("pre_ffw_norm_2.weight"));
                let mut experts = vec![0.0; residual.len()];
                for e in order {
                    let fused =
                        self.project(&name("ffn_gate_up_exps.weight"), &expert_input, Some(e));
                    let (gate, up) = fused.split_at(fused.len() / 2);
                    let result = self.project(
                        &name("ffn_down_exps.weight"),
                        &gelu_product(gate, up),
                        Some(e),
                    );
                    let weight = (f64::from(probabilities[e]) / sum) as f32
                        * self.weights[&name("ffn_down_exps.scale")][e];
                    for (acc, x) in experts.iter_mut().zip(result) {
                        *acc += x * weight;
                    }
                }
                let experts = self.norm(&experts, &name("post_ffw_norm_2.weight"));
                let combined = self.norm(&add(&shared, &experts), &name("post_ffw_norm.weight"));
                let scale = self.weights[&name(if q < prompt {
                    "enc_layer_output_scale.weight"
                } else {
                    "layer_output_scale.weight"
                })][0];
                add(&residual, &combined)
                    .iter()
                    .map(|x| x * scale)
                    .collect()
            })
            .collect()
    }
}
fn norm(x: &[f32], scale: Option<&[f32]>) -> Vec<f32> {
    let inv = 1.0
        / (x.iter().map(|&x| f64::from(x).powi(2)).sum::<f64>() / x.len() as f64
            + f64::from(1e-6_f32))
        .sqrt();
    x.iter()
        .enumerate()
        .map(|(i, &x)| (f64::from(x) * inv) as f32 * scale.map_or(1.0, |s| s[i]))
        .collect()
}
fn add(a: &[f32], b: &[f32]) -> Vec<f32> {
    a.iter().zip(b).map(|(a, b)| a + b).collect()
}
fn gelu_product(g: &[f32], u: &[f32]) -> Vec<f32> {
    g.iter()
        .zip(u)
        .map(|(&g, &u)| {
            let g = f64::from(g);
            (0.5 * g * (1.0 + crate::libm::tanh(0.7978845608028654 * (g + 0.044715 * g * g * g))))
                as f32
                * u
        })
        .collect()
}
fn softmax(x: &[f32]) -> Vec<f32> {
    let max = x.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let exp: Vec<_> = x
        .iter()
        .map(|&x| crate::libm::exp(f64::from(x) - f64::from(max)))
        .collect();
    let sum: f64 = exp.iter().sum();
    exp.iter().map(|x| (x / sum) as f32).collect()
}
