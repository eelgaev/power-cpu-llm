//! Token-by-token generation for autoregressive models (Gemma 4): the prompt
//! is prefilled with the causal encoder in chunks; each step then samples one
//! token from the logits of the last position and appends it with a one-token
//! causal pass. Every token is a `Commit` event, so completion assembly,
//! output parsing and streaming are shared with diffusion generation.
use super::{EncodeKind, Error, Event, FinishReason, GenerationConfig, Output, Sampling, Usage};
use crate::engine::text::{EncoderCache, ReferenceText};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    Prefill,
    Generate,
    Finished,
}

pub(super) struct ArRunner<'r, 'a> {
    runtime: &'r ReferenceText<'a>,
    pub(super) cache: EncoderCache<'a>,
    prompt: Vec<u32>,
    cursor: usize,
    config: GenerationConfig,
    rng: ChaCha8Rng,
    /// Final-normed hidden row of the last encoded position.
    last: Option<Vec<f32>>,
    pub(super) output: Output,
    phase: Phase,
}

impl<'r, 'a> ArRunner<'r, 'a> {
    pub(super) fn new(
        runtime: &'r ReferenceText<'a>,
        prompt: &[u32],
        config: GenerationConfig,
    ) -> Result<Self, Error> {
        let spec = runtime.specification();
        let s = &config.sampling;
        if prompt.is_empty()
            || config.encoder_chunk_tokens == 0
            || config.max_context_tokens == 0
            || !(s.temperature >= 0.0 && s.temperature.is_finite())
            || !(s.top_p > 0.0 && s.top_p <= 1.0)
            || prompt
                .iter()
                .chain(&config.eos_ids)
                .any(|&t| t as usize >= spec.vocabulary)
        {
            return Err(Error::InvalidConfig("prompt/EOS/chunk/sampling"));
        }
        let limit = config.max_context_tokens.min(spec.context);
        if prompt
            .len()
            .checked_add(config.max_new_tokens)
            .is_none_or(|n| n > limit)
        {
            return Err(Error::ContextLength {
                prompt_tokens: prompt.len(),
                reserved_output_tokens: config.max_new_tokens,
                limit,
            });
        }
        // One-position "canvases": the same capacity check as diffusion.
        runtime.generation_capacity(
            prompt.len(),
            1,
            config.max_new_tokens,
            config.encoder_chunk_tokens,
        )?;
        let cache = runtime.new_cache()?;
        let mut output = Output {
            tokens: Vec::new(),
            finish_reason: None,
            usage: Usage {
                prompt_tokens: prompt.len(),
                ..Usage::default()
            },
        };
        output
            .tokens
            .try_reserve_exact(config.max_new_tokens)
            .map_err(|_| Error::ResourceLimit)?;
        let zero = config.max_new_tokens == 0;
        let mut runner = Self {
            runtime,
            cache,
            prompt: prompt.to_vec(),
            cursor: 0,
            rng: ChaCha8Rng::seed_from_u64(config.seed),
            config,
            last: None,
            output,
            phase: Phase::Prefill,
        };
        if zero {
            runner.finish(FinishReason::Length);
        }
        Ok(runner)
    }

    pub(super) fn is_finished(&self) -> bool {
        self.phase == Phase::Finished
    }

    pub(super) fn finish(&mut self, reason: FinishReason) {
        self.output.finish_reason = Some(reason);
        self.phase = Phase::Finished;
        self.last = None;
    }

    pub(super) fn advance(&mut self) -> Result<Event, Error> {
        match self.advance_inner() {
            Ok(event) => Ok(event),
            Err(error) => {
                if self.phase != Phase::Finished {
                    self.finish(FinishReason::Failed);
                }
                Err(error)
            }
        }
    }

    fn advance_inner(&mut self) -> Result<Event, Error> {
        match self.phase {
            Phase::Finished => Err(Error::Finished),
            Phase::Prefill => {
                let end = (self.cursor + self.config.encoder_chunk_tokens).min(self.prompt.len());
                let hidden = self
                    .runtime
                    .encode(&self.prompt[self.cursor..end], &mut self.cache)?;
                self.last = hidden.into_iter().last();
                self.output.usage.encoder_passes += 1;
                let tokens = end - self.cursor;
                self.cursor = end;
                if self.cursor == self.prompt.len() {
                    self.phase = Phase::Generate;
                }
                Ok(Event::Encoded {
                    kind: EncodeKind::Prompt,
                    tokens,
                    cached_tokens: self.cache.total_tokens(),
                })
            }
            Phase::Generate => {
                // Append the previous token first (none for the first step).
                if let Some(&previous) = self.output.tokens.last() {
                    let hidden = self.runtime.encode(&[previous], &mut self.cache)?;
                    self.last = hidden.into_iter().last();
                    self.output.usage.encoder_passes += 1;
                }
                let last = self
                    .last
                    .take()
                    .ok_or(Error::Contract("missing last hidden row"))?;
                let logits = self.runtime.logits(&[last])?;
                let token = sample(&logits, &self.config.sampling, &mut self.rng)?;
                self.output.tokens.push(token);
                self.output.usage.completion_tokens += 1;
                self.output.usage.decoder_passes += 1;
                let index = self.output.tokens.len() - 1;
                let finish = if self.config.eos_ids.contains(&token) {
                    Some(FinishReason::Eos)
                } else if self.output.tokens.len() == self.config.max_new_tokens {
                    Some(FinishReason::Length)
                } else {
                    None
                };
                if let Some(reason) = finish {
                    self.finish(reason);
                }
                Ok(Event::Commit {
                    canvas_index: index,
                    tokens: vec![token],
                    finish_reason: finish,
                })
            }
        }
    }
}

/// A logit and its token id, ordered best first as `sample`'s `by_score`
/// (higher logit, then lower id): `a < b` means `a` ranks before `b`, so a
/// max-heap keeps the worst of its entries on top.
#[derive(Clone, Copy)]
struct Ranked(f32, usize);
impl PartialEq for Ranked {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}
impl Eq for Ranked {}
impl PartialOrd for Ranked {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Ranked {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.total_cmp(&self.0).then(self.1.cmp(&other.1))
    }
}

/// Temperature, then top-k, then top-p (nucleus) over the softcapped logits;
/// temperature 0 is greedy (lowest ID on ties). One uniform draw per token.
pub(super) fn sample(logits: &[f32], s: &Sampling, rng: &mut ChaCha8Rng) -> Result<u32, Error> {
    if logits.is_empty() || logits.iter().any(|x| !x.is_finite()) {
        return Err(Error::Contract("invalid logits"));
    }
    let by_score = |a: &usize, b: &usize| logits[*b].total_cmp(&logits[*a]).then(a.cmp(b));
    if s.temperature == 0.0 || s.top_k == 1 {
        let best = (0..logits.len()).min_by(by_score).expect("non-empty");
        return Ok(best as u32);
    }
    let k = if s.top_k == 0 {
        logits.len()
    } else {
        s.top_k.min(logits.len())
    };
    // The k best under the strict order `by_score` (a unique set), found in
    // one pass: a candidate enters only if it beats the current k-th best.
    let mut order: Vec<usize> = if k < logits.len() {
        let mut heap = std::collections::BinaryHeap::with_capacity(k + 1);
        for (i, &logit) in logits.iter().enumerate() {
            if heap.len() < k {
                heap.push(Ranked(logit, i));
            } else if heap.peek().is_some_and(|w: &Ranked| Ranked(logit, i) < *w) {
                heap.pop();
                heap.push(Ranked(logit, i));
            }
        }
        heap.into_iter().map(|r| r.1).collect()
    } else {
        (0..logits.len()).collect()
    };
    order.sort_by(by_score);
    let max = f64::from(logits[order[0]]) / f64::from(s.temperature);
    let weights: Vec<f64> = order
        .iter()
        .map(|&i| crate::libm::exp(f64::from(logits[i]) / f64::from(s.temperature) - max))
        .collect();
    let total: f64 = weights.iter().sum();
    // Smallest prefix whose probability mass reaches top_p (at least one token).
    let mut kept = 0;
    let mut mass = 0.0;
    while kept < weights.len() {
        mass += weights[kept];
        kept += 1;
        if mass / total >= f64::from(s.top_p) {
            break;
        }
    }
    let draw = rng.random::<f64>() * mass;
    let mut cumulative = 0.0;
    for (i, w) in weights[..kept].iter().enumerate() {
        cumulative += w;
        if draw < cumulative {
            return Ok(order[i] as u32);
        }
    }
    Ok(order[kept - 1] as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The one-pass top-k selection keeps exactly the tokens (and draws) of
    /// the earlier full selection, ties included.
    #[test]
    fn heap_top_k_matches_full_selection() {
        fn sample_before_heap(
            logits: &[f32],
            s: &Sampling,
            rng: &mut ChaCha8Rng,
        ) -> Result<u32, Error> {
            if logits.is_empty() || logits.iter().any(|x| !x.is_finite()) {
                return Err(Error::Contract("invalid logits"));
            }
            let by_score = |a: &usize, b: &usize| logits[*b].total_cmp(&logits[*a]).then(a.cmp(b));
            if s.temperature == 0.0 || s.top_k == 1 {
                let best = (0..logits.len()).min_by(by_score).expect("non-empty");
                return Ok(best as u32);
            }
            let k = if s.top_k == 0 {
                logits.len()
            } else {
                s.top_k.min(logits.len())
            };
            let mut order: Vec<usize> = (0..logits.len()).collect();
            if k < order.len() {
                order.select_nth_unstable_by(k - 1, by_score);
                order.truncate(k);
            }
            order.sort_by(by_score);
            let max = f64::from(logits[order[0]]) / f64::from(s.temperature);
            let weights: Vec<f64> = order
                .iter()
                .map(|&i| crate::libm::exp(f64::from(logits[i]) / f64::from(s.temperature) - max))
                .collect();
            let total: f64 = weights.iter().sum();
            // Smallest prefix whose probability mass reaches top_p (at least one token).
            let mut kept = 0;
            let mut mass = 0.0;
            while kept < weights.len() {
                mass += weights[kept];
                kept += 1;
                if mass / total >= f64::from(s.top_p) {
                    break;
                }
            }
            let draw = rng.random::<f64>() * mass;
            let mut cumulative = 0.0;
            for (i, w) in weights[..kept].iter().enumerate() {
                cumulative += w;
                if draw < cumulative {
                    return Ok(order[i] as u32);
                }
            }
            Ok(order[kept - 1] as u32)
        }
        let mut seed = 0x2545_f491_4f6c_dd1du64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for case in 0..40 {
            // Coarse values: many exact ties.
            let n = 50 + (next() % 3000) as usize;
            let logits: Vec<f32> = (0..n).map(|_| (next() % 41) as f32 * 0.25 - 5.0).collect();
            for top_k in [0, 1, 2, 7, 64, n - 1, n, n + 5] {
                for top_p in [1.0, 0.9, 0.3] {
                    let s = Sampling {
                        temperature: 0.8,
                        top_k,
                        top_p,
                    };
                    let mut a = ChaCha8Rng::seed_from_u64(case);
                    let mut b = ChaCha8Rng::seed_from_u64(case);
                    for _ in 0..5 {
                        assert_eq!(
                            sample(&logits, &s, &mut a).unwrap(),
                            sample_before_heap(&logits, &s, &mut b).unwrap(),
                            "case {case} n {n} top_k {top_k} top_p {top_p}"
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn sampling_is_greedy_at_zero_temperature_and_respects_top_k_top_p() {
        let logits = [0.0f32, 3.0, 2.9, -1.0, 3.0];
        let mut rng = ChaCha8Rng::seed_from_u64(1);
        let greedy = Sampling {
            temperature: 0.0,
            top_k: 64,
            top_p: 1.0,
        };
        assert_eq!(sample(&logits, &greedy, &mut rng).unwrap(), 1); // tie -> lower ID
        let top1 = Sampling {
            temperature: 1.0,
            top_k: 1,
            top_p: 1.0,
        };
        assert_eq!(sample(&logits, &top1, &mut rng).unwrap(), 1);
        // top_k 2 keeps IDs 1 and 4 (equal scores); never anything else.
        let k2 = Sampling {
            temperature: 1.0,
            top_k: 2,
            top_p: 1.0,
        };
        for _ in 0..200 {
            assert!(matches!(sample(&logits, &k2, &mut rng).unwrap(), 1 | 4));
        }
        // A tiny top_p keeps only the single best token.
        let p = Sampling {
            temperature: 1.0,
            top_k: 0,
            top_p: 1e-6,
        };
        for _ in 0..50 {
            assert_eq!(sample(&logits, &p, &mut rng).unwrap(), 1);
        }
        // Same seed, same draws.
        let s = Sampling::default();
        let a: Vec<u32> = {
            let mut r = ChaCha8Rng::seed_from_u64(9);
            (0..20)
                .map(|_| sample(&logits, &s, &mut r).unwrap())
                .collect()
        };
        let b: Vec<u32> = {
            let mut r = ChaCha8Rng::seed_from_u64(9);
            (0..20)
                .map(|_| sample(&logits, &s, &mut r).unwrap())
                .collect()
        };
        assert_eq!(a, b);
        assert!(sample(&[f32::NAN], &s, &mut rng).is_err());
    }
}
