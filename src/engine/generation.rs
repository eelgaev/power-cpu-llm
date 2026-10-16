//! Single-request token-ID generation state machine. This connects real
//! reference model execution to the sampler; it does not assert real-model
//! quality/parity, optimized throughput, chat semantics, or HTTP serving.

use crate::engine::{
    diffusion::{self, CanvasDenoiser, DenoisingConfig, StepReport},
    operators,
    text::{EncoderCache, ReferenceText},
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::fmt;

#[derive(Debug, Clone)]
pub struct GenerationConfig {
    pub max_new_tokens: usize,
    /// Prompt tokens plus `max_new_tokens` rounded up to whole canvases must not
    /// exceed this (or the model context, if smaller). At least 1; the default
    /// (`usize::MAX`) means the model's own context. The runtime's
    /// [`crate::engine::text::ReferenceLimits`] must also admit the context
    /// (see `ReferenceLimits::for_context`).
    pub max_context_tokens: usize,
    pub seed: u64,
    pub denoising: DenoisingConfig,
    pub eos_ids: Vec<u32>,
    /// Bound encoder rows per pass; also used when encoding completed canvases.
    pub encoder_chunk_tokens: usize,
    /// None uses the MODEL canvas, including a full final canvas. Some is an
    /// explicit short-canvas diagnostic, not the trained 256-position procedure.
    pub diagnostic_canvas_length: Option<usize>,
    /// Token sampling for autoregressive models (diffusion models use `denoising`).
    pub sampling: Sampling,
}

/// Autoregressive token sampling (defaults: Gemma 4's recommended settings,
/// also recorded in its GGUF as `general.sampling.*`).
#[derive(Debug, Clone, PartialEq)]
pub struct Sampling {
    /// 0 is greedy.
    pub temperature: f32,
    /// 0 disables top-k.
    pub top_k: usize,
    /// In (0, 1]; 1 disables nucleus filtering.
    pub top_p: f32,
}
impl Default for Sampling {
    fn default() -> Self {
        Self {
            temperature: 1.0,
            top_k: 64,
            top_p: 0.95,
        }
    }
}
impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            max_new_tokens: 256,
            max_context_tokens: usize::MAX,
            seed: 0,
            denoising: DenoisingConfig::default(),
            eos_ids: diffusion::EOS_TOKEN_IDS.to_vec(),
            encoder_chunk_tokens: 256,
            diagnostic_canvas_length: None,
            sampling: Sampling::default(),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidConfig(&'static str),
    Backend(operators::Error),
    Sampling(diffusion::Error),
    Finished,
    ResourceLimit,
    /// The request does not fit the context limit; nothing was executed.
    ContextLength {
        prompt_tokens: usize,
        /// `max_new_tokens` rounded up to whole canvases.
        reserved_output_tokens: usize,
        limit: usize,
    },
    Contract(&'static str),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
impl From<operators::Error> for Error {
    fn from(e: operators::Error) -> Self {
        Self::Backend(e)
    }
}
impl From<diffusion::Error> for Error {
    fn from(e: diffusion::Error) -> Self {
        Self::Sampling(e)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinishReason {
    Eos,
    Length,
    Cancelled,
    Failed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeKind {
    Prompt,
    CompletedCanvas,
}
#[derive(Debug, Clone)]
pub enum Event {
    Encoded {
        kind: EncodeKind,
        tokens: usize,
        cached_tokens: usize,
    },
    /// Progress only: no draft token IDs are exposed.
    Step {
        canvas_index: usize,
        report: StepReport,
    },
    /// The ONLY token-bearing event; EOS/length applied after final argmax.
    Commit {
        canvas_index: usize,
        tokens: Vec<u32>,
        finish_reason: Option<FinishReason>,
    },
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize, // includes a committed EOS, if any
    pub encoder_passes: usize,
    pub decoder_passes: usize,
    pub completed_canvases: usize,
}
#[derive(Debug, Clone)]
pub struct Output {
    pub tokens: Vec<u32>,
    pub finish_reason: Option<FinishReason>,
    pub usage: Usage,
}

#[derive(Debug, Clone, Copy)]
struct Geometry {
    vocabulary: u32,
    canvas: usize,
    context: usize,
}
// Private seam for deterministic tests, not multi-architecture execution API.
trait Backend {
    type Cache;
    fn geometry(&self) -> Geometry;
    fn capacity(
        &self,
        prompt: usize,
        canvas: usize,
        rounds: usize,
        chunk: usize,
    ) -> Result<(), operators::Error>;
    fn new_cache(&self) -> Result<Self::Cache, operators::Error>;
    fn encoded(&self, cache: &Self::Cache) -> usize;
    fn encode(&self, tokens: &[u32], cache: &mut Self::Cache) -> Result<(), operators::Error>;
    fn decode(
        &self,
        tokens: &[u32],
        cache: &Self::Cache,
        previous: Option<&[f32]>,
    ) -> Result<Vec<f32>, operators::Error>;
}
impl<'a> Backend for &ReferenceText<'a> {
    type Cache = EncoderCache<'a>;
    fn geometry(&self) -> Geometry {
        let s = self.specification();
        Geometry {
            vocabulary: s.vocabulary as u32,
            canvas: s.canvas,
            context: s.context,
        }
    }
    fn capacity(&self, p: usize, c: usize, n: usize, chunk: usize) -> Result<(), operators::Error> {
        self.generation_capacity(p, c, n, chunk)
    }
    fn new_cache(&self) -> Result<Self::Cache, operators::Error> {
        ReferenceText::new_cache(self)
    }
    fn encoded(&self, c: &Self::Cache) -> usize {
        c.total_tokens()
    }
    fn encode(&self, t: &[u32], c: &mut Self::Cache) -> Result<(), operators::Error> {
        ReferenceText::encode(self, t, c).map(|_| ())
    }
    fn decode(
        &self,
        t: &[u32],
        c: &Self::Cache,
        p: Option<&[f32]>,
    ) -> Result<Vec<f32>, operators::Error> {
        self.decode_logits(t, c, p)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    Prefill,
    Denoise,
    Commit,
    Extend,
    Finished,
}
struct Runner<B: Backend> {
    backend: B,
    cache: B::Cache,
    geometry: Geometry,
    config: GenerationConfig,
    canvas: usize,
    prompt: Vec<u32>,
    prompt_cursor: usize,
    pending: Vec<u32>,
    extend_cursor: usize,
    rng: Option<ChaCha8Rng>,
    sampler: Option<CanvasDenoiser>,
    phase: Phase,
    output: Output,
}
impl<B: Backend> Runner<B> {
    fn new(backend: B, prompt: &[u32], config: GenerationConfig) -> Result<Self, Error> {
        config.denoising.validate()?;
        let geometry = backend.geometry();
        let canvas = config.diagnostic_canvas_length.unwrap_or(geometry.canvas);
        if geometry.vocabulary == 0
            || geometry.canvas == 0
            || geometry.context == 0
            || canvas == 0
            || canvas > geometry.canvas
            || config.encoder_chunk_tokens == 0
            || config.max_context_tokens == 0
            || prompt.is_empty()
            || prompt
                .iter()
                .chain(&config.eos_ids)
                .any(|&t| t >= geometry.vocabulary)
        {
            return Err(Error::InvalidConfig("geometry/prompt/EOS/chunk/canvas"));
        }
        let rounds = config.max_new_tokens.div_ceil(canvas);
        let reserved = rounds.checked_mul(canvas).ok_or(Error::ResourceLimit)?;
        if rounds.checked_mul(config.denoising.max_steps).is_none() {
            return Err(Error::ResourceLimit);
        }
        let limit = config.max_context_tokens.min(geometry.context);
        if prompt.len().checked_add(reserved).is_none_or(|n| n > limit) {
            return Err(Error::ContextLength {
                prompt_tokens: prompt.len(),
                reserved_output_tokens: reserved,
                limit,
            });
        }
        backend.capacity(prompt.len(), canvas, rounds, config.encoder_chunk_tokens)?;
        let cache = backend.new_cache()?;
        if backend.encoded(&cache) != 0 {
            return Err(Error::Contract("fresh encoder cache not empty"));
        }
        let mut tokens = Vec::new();
        tokens
            .try_reserve_exact(config.max_new_tokens)
            .map_err(|_| Error::ResourceLimit)?;
        let zero = config.max_new_tokens == 0;
        let rng = Some(ChaCha8Rng::seed_from_u64(config.seed));
        Ok(Self {
            backend,
            cache,
            geometry,
            canvas,
            prompt: prompt.to_vec(),
            prompt_cursor: 0,
            pending: Vec::new(),
            extend_cursor: 0,
            sampler: None,
            rng,
            phase: if zero {
                Phase::Finished
            } else {
                Phase::Prefill
            },
            output: Output {
                tokens,
                finish_reason: zero.then_some(FinishReason::Length),
                usage: Usage {
                    prompt_tokens: prompt.len(),
                    ..Usage::default()
                },
            },
            config,
        })
    }
    fn start_canvas(&mut self) -> Result<(), Error> {
        self.sampler = Some(CanvasDenoiser::with_rng(
            self.config.denoising,
            self.canvas,
            self.geometry.vocabulary,
            self.rng
                .take()
                .ok_or(Error::Contract("missing request RNG"))?,
        )?);
        self.phase = Phase::Denoise;
        Ok(())
    }
    fn finish(&mut self, reason: FinishReason) {
        self.output.finish_reason = Some(reason);
        self.phase = Phase::Finished;
        self.sampler = None;
        self.pending.clear();
        self.rng = None;
    }
    fn advance(&mut self) -> Result<Event, Error> {
        if self.phase == Phase::Finished {
            return Err(Error::Finished);
        }
        match self.advance_inner() {
            Ok(event) => Ok(event),
            Err(e) => {
                self.finish(FinishReason::Failed);
                Err(e)
            }
        }
    }
    fn encode_range(&mut self, tokens: &[u32]) -> Result<(), Error> {
        let before = self.backend.encoded(&self.cache);
        self.backend.encode(tokens, &mut self.cache)?;
        if before.checked_add(tokens.len()) != Some(self.backend.encoded(&self.cache)) {
            return Err(Error::Contract("encoder position advance"));
        }
        self.output.usage.encoder_passes += 1;
        Ok(())
    }
    fn advance_inner(&mut self) -> Result<Event, Error> {
        match self.phase {
            Phase::Prefill => {
                let count =
                    (self.prompt.len() - self.prompt_cursor).min(self.config.encoder_chunk_tokens);
                let chunk = self.prompt[self.prompt_cursor..self.prompt_cursor + count].to_vec();
                self.encode_range(&chunk)?;
                self.prompt_cursor += count;
                if self.prompt_cursor == self.prompt.len() {
                    self.start_canvas()?;
                }
                Ok(Event::Encoded {
                    kind: EncodeKind::Prompt,
                    tokens: count,
                    cached_tokens: self.backend.encoded(&self.cache),
                })
            }
            Phase::Denoise => {
                let sampler = self
                    .sampler
                    .as_mut()
                    .ok_or(Error::Contract("missing denoiser"))?;
                let before = self.backend.encoded(&self.cache);
                let logits = self.backend.decode(
                    sampler.current_canvas(),
                    &self.cache,
                    sampler.self_conditioning_logits(),
                )?;
                self.output.usage.decoder_passes += 1;
                if self.backend.encoded(&self.cache) != before {
                    return Err(Error::Contract("decoder modified encoder cache"));
                }
                let report = sampler.advance(&logits)?;
                if report.finish_reason.is_some() {
                    self.phase = Phase::Commit;
                }
                Ok(Event::Step {
                    canvas_index: self.output.usage.completed_canvases,
                    report,
                })
            }
            Phase::Commit => {
                let sampler = self
                    .sampler
                    .take()
                    .ok_or(Error::Contract("missing committed denoiser"))?;
                let argmax = sampler
                    .committed_tokens()
                    .ok_or(Error::Contract("uncommitted argmax"))?;
                let commit = diffusion::commit_canvas(
                    argmax,
                    self.config.max_new_tokens - self.output.tokens.len(),
                    &self.config.eos_ids,
                );
                let reason = commit.finish_reason.map(|r| match r {
                    diffusion::GenerationFinishReason::Eos => FinishReason::Eos,
                    diffusion::GenerationFinishReason::Length => FinishReason::Length,
                });
                let index = self.output.usage.completed_canvases;
                self.output.tokens.extend_from_slice(&commit.tokens);
                self.output.usage.completion_tokens = self.output.tokens.len();
                self.output.usage.completed_canvases += 1;
                if let Some(reason) = reason {
                    self.finish(reason);
                } else {
                    self.pending = argmax.to_vec();
                    self.extend_cursor = 0;
                    self.rng = Some(sampler.into_rng());
                    self.phase = Phase::Extend;
                }
                Ok(Event::Commit {
                    canvas_index: index,
                    tokens: commit.tokens,
                    finish_reason: reason,
                })
            }
            Phase::Extend => {
                let count =
                    (self.pending.len() - self.extend_cursor).min(self.config.encoder_chunk_tokens);
                let chunk = self.pending[self.extend_cursor..self.extend_cursor + count].to_vec();
                self.encode_range(&chunk)?;
                self.extend_cursor += count;
                if self.extend_cursor == self.pending.len() {
                    self.pending.clear();
                    self.start_canvas()?;
                }
                Ok(Event::Encoded {
                    kind: EncodeKind::CompletedCanvas,
                    tokens: count,
                    cached_tokens: self.backend.encoded(&self.cache),
                })
            }
            Phase::Finished => Err(Error::Finished),
        }
    }
}

/// Incremental, single-request generation over the reference model. Cancellation
/// is checked by the caller BETWEEN advance() calls, not within a native/model
/// forward; a slow full pass may delay cancellation. Only committed tokens are
/// observable. Backend/sampler errors terminate the session with Failed while
/// preserving previously committed output; no automatic retry/replay.
///
/// The model kind selects the procedure: diffusion canvases (DiffusionGemma)
/// or token-by-token sampling (Gemma 4, `autoregressive`); both report the
/// same events, usage and output.
/// Speculative decoding totals of a session.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DraftStats {
    pub drafted: usize,
    pub accepted: usize,
    pub verify_passes: usize,
    pub draft_seconds: f64,
    pub verify_seconds: f64,
}

pub struct GenerationSession<'r, 'a> {
    inner: Inner<'r, 'a>,
}
enum Inner<'r, 'a> {
    Diffusion(Box<Runner<&'r ReferenceText<'a>>>),
    Autoregressive(Box<autoregressive::ArRunner<'r, 'a>>),
}
impl<'r, 'a> GenerationSession<'r, 'a> {
    pub fn new(
        runtime: &'r ReferenceText<'a>,
        prompt: &[u32],
        config: GenerationConfig,
    ) -> Result<Self, Error> {
        let inner = match runtime.specification().kind {
            crate::engine::model::ModelKind::Diffusion => {
                Inner::Diffusion(Box::new(Runner::new(runtime, prompt, config)?))
            }
            crate::engine::model::ModelKind::Autoregressive => Inner::Autoregressive(Box::new(
                autoregressive::ArRunner::new(runtime, prompt, config)?,
            )),
        };
        Ok(Self { inner })
    }
    /// Speculative decoding for an autoregressive target: `drafter` proposes
    /// up to `max_draft` tokens per target pass (see `ArRunner::set_drafter`).
    /// Diffusion sessions reject it.
    pub fn set_drafter(
        &mut self,
        drafter: &'r crate::engine::assistant::Drafter<'a>,
        max_draft: usize,
    ) -> Result<(), Error> {
        match &mut self.inner {
            Inner::Autoregressive(r) => {
                r.set_drafter(drafter, max_draft);
                Ok(())
            }
            Inner::Diffusion(_) => Err(Error::InvalidConfig(
                "drafting needs an autoregressive model",
            )),
        }
    }
    /// Speculative decoding totals so far (zeros without a drafter).
    pub fn draft_stats(&self) -> DraftStats {
        match &self.inner {
            Inner::Autoregressive(r) => DraftStats {
                drafted: r.drafted,
                accepted: r.drafts_accepted,
                verify_passes: r.verify_passes,
                draft_seconds: r.draft_time.as_secs_f64(),
                verify_seconds: r.verify_time.as_secs_f64(),
            },
            Inner::Diffusion(_) => DraftStats::default(),
        }
    }
    pub fn advance(&mut self) -> Result<Event, Error> {
        match &mut self.inner {
            Inner::Diffusion(r) => r.advance(),
            Inner::Autoregressive(r) => r.advance(),
        }
    }
    pub fn is_finished(&self) -> bool {
        match &self.inner {
            Inner::Diffusion(r) => r.phase == Phase::Finished,
            Inner::Autoregressive(r) => r.is_finished(),
        }
    }
    pub fn committed_tokens(&self) -> &[u32] {
        &self.output().tokens
    }
    pub fn cached_tokens(&self) -> usize {
        match &self.inner {
            Inner::Diffusion(r) => r.backend.encoded(&r.cache),
            Inner::Autoregressive(r) => r.cache.total_tokens(),
        }
    }
    fn output(&self) -> &Output {
        match &self.inner {
            Inner::Diffusion(r) => &r.output,
            Inner::Autoregressive(r) => &r.output,
        }
    }
    pub fn result(&self) -> Option<&Output> {
        self.is_finished().then_some(self.output())
    }
    pub fn cancel(&mut self) {
        if !self.is_finished() {
            match &mut self.inner {
                Inner::Diffusion(r) => r.finish(FinishReason::Cancelled),
                Inner::Autoregressive(r) => r.finish(FinishReason::Cancelled),
            }
        }
    }
}

mod autoregressive;

#[cfg(test)]
mod tests;
