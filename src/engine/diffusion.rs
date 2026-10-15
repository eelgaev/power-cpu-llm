//! Single-canvas DiffusionGemma reference sampler, not a model forward pass.
//!
//! Algorithm reference: Hugging Face Transformers (Apache-2.0), revision
//! bd970fda476f204f9852fb551aa07a4b3fbc5540, generation_diffusion_gemma.py.
//! See docs/models.md for provenance and numerical differences.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::VecDeque;
use std::fmt;

pub const CANVAS_LENGTH: usize = 256;
pub const VOCAB_SIZE: u32 = 262_144;
pub const EOS_TOKEN_IDS: &[u32] = &[1, 106, 50];

#[derive(Debug, Clone, Copy)]
pub struct DenoisingConfig {
    pub max_steps: usize,
    pub temperature_max: f64,
    pub temperature_min: f64,
    pub entropy_bound: f64,
    pub confidence_threshold: f64,
    /// Number of previous argmax canvases that must equal this one.
    pub stability_steps: usize,
}

impl Default for DenoisingConfig {
    fn default() -> Self {
        Self {
            max_steps: 48,
            temperature_max: 0.8,
            temperature_min: 0.4,
            entropy_bound: 0.1,
            confidence_threshold: 0.005,
            stability_steps: 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidConfig(&'static str),
    WrongLogitsShape { expected: usize, actual: usize },
    InvalidLogits { position: usize },
    Finished,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

impl DenoisingConfig {
    pub fn validate(&self) -> Result<(), Error> {
        if self.max_steps == 0 {
            return Err(Error::InvalidConfig("max_steps must be positive"));
        }
        if !self.temperature_min.is_finite()
            || !self.temperature_max.is_finite()
            || self.temperature_min <= 0.0
            || self.temperature_max < self.temperature_min
        {
            return Err(Error::InvalidConfig(
                "temperatures must be finite, positive, and max >= min",
            ));
        }
        if !self.entropy_bound.is_finite() || self.entropy_bound <= 0.0 {
            return Err(Error::InvalidConfig(
                "entropy_bound must be finite and positive",
            ));
        }
        if !self.confidence_threshold.is_finite() || self.confidence_threshold <= 0.0 {
            return Err(Error::InvalidConfig(
                "confidence_threshold must be finite and positive",
            ));
        }
        Ok(())
    }

    /// Matches the reference's remaining-step convention: N, N-1, ..., 1.
    /// The last evaluated temperature is min + (max-min)/N, not exactly min.
    pub fn temperature(&self, remaining_steps: usize) -> Result<f64, Error> {
        self.validate()?;
        if remaining_steps == 0 || remaining_steps > self.max_steps {
            return Err(Error::InvalidConfig(
                "remaining steps must be in 1..=max_steps",
            ));
        }
        Ok(self.temperature_min
            + (self.temperature_max - self.temperature_min) * remaining_steps as f64
                / self.max_steps as f64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasFinishReason {
    StableAndConfident,
    StepLimit,
}

#[derive(Debug, Clone)]
pub struct StepReport {
    pub step: usize,
    pub temperature: f64,
    pub accepted_positions: usize,
    pub mean_entropy: f64,
    pub finish_reason: Option<CanvasFinishReason>,
}

/// Reference implementation for one request and one canvas. No draft tokens
/// become committed until stopping/step limit; committed output is argmax.
/// Full processed logits are retained for the next decoder's self-conditioning.
/// RNG is deterministic for our seed, not seed-equivalent to PyTorch.
pub struct CanvasDenoiser {
    config: DenoisingConfig,
    vocab_size: u32,
    logits_len: usize,
    rng: ChaCha8Rng,
    current: Vec<u32>,
    argmax: Vec<u32>,
    history: VecDeque<Vec<u32>>,
    self_conditioning: Option<Vec<f32>>,
    completed_steps: usize,
    finish_reason: Option<CanvasFinishReason>,
}

impl CanvasDenoiser {
    pub fn new(
        config: DenoisingConfig,
        canvas_length: usize,
        vocab_size: u32,
        seed: u64,
    ) -> Result<Self, Error> {
        Self::with_rng(
            config,
            canvas_length,
            vocab_size,
            ChaCha8Rng::seed_from_u64(seed),
        )
    }

    /// Carry one request RNG stream across canvases, while resetting all
    /// self-conditioning/stability state at each new canvas.
    pub(crate) fn with_rng(
        config: DenoisingConfig,
        canvas_length: usize,
        vocab_size: u32,
        mut rng: ChaCha8Rng,
    ) -> Result<Self, Error> {
        config.validate()?;
        if canvas_length == 0 || vocab_size == 0 {
            return Err(Error::InvalidConfig(
                "canvas length and vocabulary must be positive",
            ));
        }
        let logits_len = canvas_length
            .checked_mul(vocab_size as usize)
            .filter(|&n| n <= isize::MAX as usize / size_of::<f32>())
            .ok_or(Error::InvalidConfig("logits buffer size overflows"))?;
        let current = (0..canvas_length)
            .map(|_| rng.random_range(0..vocab_size))
            .collect();
        Ok(Self {
            config,
            vocab_size,
            logits_len,
            rng,
            current,
            argmax: Vec::new(),
            history: VecDeque::new(),
            self_conditioning: None,
            completed_steps: 0,
            finish_reason: None,
        })
    }

    pub(crate) fn into_rng(self) -> ChaCha8Rng {
        self.rng
    }

    pub fn current_canvas(&self) -> &[u32] {
        &self.current
    }
    pub fn self_conditioning_logits(&self) -> Option<&[f32]> {
        self.self_conditioning.as_deref()
    }
    pub fn committed_tokens(&self) -> Option<&[u32]> {
        self.finish_reason.map(|_| self.argmax.as_slice())
    }
    pub fn completed_steps(&self) -> usize {
        self.completed_steps
    }

    /// Input is position-major [canvas_length, vocab_size] raw decoder logits,
    /// after the model's logit softcap but before temperature shaping.
    /// NaN, +inf and wholly masked rows are rejected without advancing state.
    /// -inf is allowed for vocabulary masking. This reference favors clarity
    /// and numerical stability, not production buffer reuse or speed.
    pub fn advance(&mut self, raw_logits: &[f32]) -> Result<StepReport, Error> {
        if self.finish_reason.is_some() {
            return Err(Error::Finished);
        }
        if raw_logits.len() != self.logits_len {
            return Err(Error::WrongLogitsShape {
                expected: self.logits_len,
                actual: raw_logits.len(),
            });
        }
        let remaining = self.config.max_steps - self.completed_steps;
        let temperature = self.config.temperature(remaining)?;
        // Rows are independent: they are computed in parallel (identical
        // per-row arithmetic); RNG draws below stay sequential and in order.
        let vocab = self.vocab_size as usize;
        let positions = raw_logits.len() / vocab;
        let processed: Vec<f32> = par_rows(positions, |p| {
            raw_logits[p * vocab..(p + 1) * vocab]
                .iter()
                .map(|&x| (f64::from(x) / temperature) as f32)
                .collect::<Vec<f32>>()
        })
        .concat();
        let rows: Vec<_> = par_rows(positions, |p| {
            Distribution::new(&processed[p * vocab..(p + 1) * vocab])
        })
        .into_iter()
        .enumerate()
        .map(|(position, row)| row.ok_or(Error::InvalidLogits { position }))
        .collect::<Result<_, _>>()?;

        // All logits have been validated before consuming RNG or changing state.
        let argmax: Vec<u32> = rows.iter().map(|r| r.argmax as u32).collect();
        let entropies: Vec<f64> = rows.iter().map(|r| r.entropy).collect();
        let selected = entropy_selection(&entropies, self.config.entropy_bound);
        // One uniform draw per row, in row order (exactly as sequential
        // sampling consumes them), then the scans in parallel.
        let draws: Vec<f64> = rows.iter().map(|_| self.rng.random::<f64>()).collect();
        let sampled: Vec<u32> = par_rows(rows.len(), |p| rows[p].sample(draws[p]) as u32);
        let next: Vec<u32> = sampled
            .iter()
            .zip(&selected)
            .map(|(&token, &accept)| {
                // Reference creates a full random canvas, even for accepted positions.
                let noise = self.rng.random_range(0..self.vocab_size);
                if accept { token } else { noise }
            })
            .collect();

        let stable = self.history.len() == self.config.stability_steps
            && self.history.iter().all(|previous| previous == &argmax);
        let mean_entropy = entropies.iter().sum::<f64>() / entropies.len() as f64;
        let confident = mean_entropy < self.config.confidence_threshold;
        if self.config.stability_steps != 0 {
            if self.history.len() == self.config.stability_steps {
                self.history.pop_front();
            }
            self.history.push_back(argmax.clone());
        }
        self.completed_steps += 1;
        self.finish_reason = if stable && confident {
            Some(CanvasFinishReason::StableAndConfident)
        } else if self.completed_steps == self.config.max_steps {
            Some(CanvasFinishReason::StepLimit)
        } else {
            None
        };
        self.current = next;
        self.argmax = argmax;
        self.self_conditioning = Some(processed);
        Ok(StepReport {
            step: self.completed_steps,
            temperature,
            accepted_positions: selected.iter().filter(|&&s| s).count(),
            mean_entropy,
            finish_reason: self.finish_reason,
        })
    }
}

// Entropy-bounded prefix selection: sorted cumulative entropy minus the
// current (largest) entropy <= bound. Acceptance is recomputed every step;
// positions selected earlier are NOT permanently frozen. Ties use index order.
fn entropy_selection(entropies: &[f64], bound: f64) -> Vec<bool> {
    let mut order: Vec<usize> = (0..entropies.len()).collect();
    order.sort_by(|&a, &b| entropies[a].total_cmp(&entropies[b]).then(a.cmp(&b)));
    let mut cumulative = 0.0;
    let mut selected = vec![false; entropies.len()];
    for index in order {
        cumulative += entropies[index];
        selected[index] = cumulative - entropies[index] <= bound;
    }
    selected
}

// Retain row statistics, not an additional canvas-sized probability tensor.
struct Distribution<'a> {
    logits: &'a [f32],
    max: f64,
    total: f64,
    entropy: f64,
    argmax: usize,
}
impl<'a> Distribution<'a> {
    fn new(logits: &'a [f32]) -> Option<Self> {
        let mut max = f32::NEG_INFINITY;
        let mut argmax = 0;
        for (i, &x) in logits.iter().enumerate() {
            if x.is_nan() || x == f32::INFINITY {
                return None;
            }
            if x > max {
                max = x;
                argmax = i;
            }
        }
        if !max.is_finite() {
            return None;
        }
        let max = f64::from(max);
        let total: f64 = logits
            .iter()
            .map(|&x| crate::libm::exp(f64::from(x) - max))
            .sum();
        let log_total = crate::libm::ln(total);
        let entropy = logits
            .iter()
            .map(|&x| {
                let shifted = f64::from(x) - max;
                let weight = crate::libm::exp(shifted);
                if weight == 0.0 {
                    0.0
                } else {
                    -(weight / total) * (shifted - log_total)
                }
            })
            .sum::<f64>()
            .max(0.0);
        Some(Self {
            logits,
            max,
            total,
            entropy,
            argmax,
        })
    }
    /// `uniform` is one `random::<f64>()` draw in [0, 1).
    fn sample(&self, uniform: f64) -> usize {
        let draw = uniform * self.total;
        let mut cumulative = 0.0;
        let mut last_positive = self.argmax;
        for (i, &logit) in self.logits.iter().enumerate() {
            let weight = crate::libm::exp(f64::from(logit) - self.max);
            if weight > 0.0 {
                last_positive = i;
            }
            cumulative += weight;
            if draw < cumulative {
                return i;
            }
        }
        last_positive // Handles floating-point rounding at the upper boundary.
    }
}

/// `f(0..n)` in order, split into contiguous ranges over scoped threads
/// (at most the CPUs in this process's affinity mask).
fn par_rows<T: Send>(n: usize, f: impl Fn(usize) -> T + Sync) -> Vec<T> {
    let threads = std::thread::available_parallelism()
        .map_or(1, std::num::NonZeroUsize::get)
        .min(n)
        .max(1);
    if threads == 1 {
        return (0..n).map(f).collect();
    }
    std::thread::scope(|scope| {
        let f = &f;
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                let (first, end) = (t * n / threads, (t + 1) * n / threads);
                scope.spawn(move || (first..end).map(f).collect::<Vec<T>>())
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().expect("sampler row worker panicked"))
            .collect()
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationFinishReason {
    Eos,
    Length,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanvasCommit {
    /// Includes the first EOS token, if any; subsequent tokens are discarded.
    pub tokens: Vec<u32>,
    pub finish_reason: Option<GenerationFinishReason>,
}

/// Apply output-length/EOS limits only to a completed canvas, never to drafts.
/// Token decoding and removal of special tokens are a separate tokenizer task.
pub fn commit_canvas(tokens: &[u32], remaining_tokens: usize, eos_ids: &[u32]) -> CanvasCommit {
    let limit = tokens.len().min(remaining_tokens);
    let visible = &tokens[..limit];
    if let Some(index) = visible.iter().position(|t| eos_ids.contains(t)) {
        CanvasCommit {
            tokens: visible[..=index].to_vec(),
            finish_reason: Some(GenerationFinishReason::Eos),
        }
    } else {
        CanvasCommit {
            tokens: visible.to_vec(),
            finish_reason: (limit == remaining_tokens).then_some(GenerationFinishReason::Length),
        }
    }
}

#[cfg(test)]
mod tests;
