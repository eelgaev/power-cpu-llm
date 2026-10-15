use super::*;
use std::cell::{Cell, RefCell};

#[derive(Debug, Clone, PartialEq)]
enum Call {
    Encode(Vec<u32>),
    Decode {
        tokens: Vec<u32>,
        prefix: Vec<u32>,
        previous: Option<Vec<f32>>,
    },
}
struct Mock {
    geometry: Geometry,
    scripts: Vec<Vec<f32>>,
    calls: RefCell<Vec<Call>>,
    decoded: Cell<usize>,
    encoded: Cell<usize>,
    fail_decode: Option<usize>,
    fail_encode: Option<usize>,
}
impl Mock {
    fn new(scripts: Vec<Vec<f32>>) -> Self {
        Self {
            geometry: Geometry {
                vocabulary: 8,
                canvas: 4,
                context: 64,
            },
            scripts,
            calls: RefCell::new(Vec::new()),
            decoded: Cell::new(0),
            encoded: Cell::new(0),
            fail_decode: None,
            fail_encode: None,
        }
    }
}
impl Backend for &Mock {
    type Cache = Vec<u32>;
    fn geometry(&self) -> Geometry {
        self.geometry
    }
    fn capacity(&self, _: usize, _: usize, _: usize, _: usize) -> Result<(), operators::Error> {
        Ok(())
    }
    fn new_cache(&self) -> Result<Self::Cache, operators::Error> {
        Ok(Vec::new())
    }
    fn encoded(&self, cache: &Self::Cache) -> usize {
        cache.len()
    }
    fn encode(&self, t: &[u32], c: &mut Self::Cache) -> Result<(), operators::Error> {
        let n = self.encoded.get();
        self.encoded.set(n + 1);
        if self.fail_encode == Some(n) {
            return Err(operators::Error::Parameter("injected encoder failure"));
        }
        self.calls.borrow_mut().push(Call::Encode(t.to_vec()));
        c.extend_from_slice(t);
        Ok(())
    }
    fn decode(
        &self,
        t: &[u32],
        c: &Self::Cache,
        p: Option<&[f32]>,
    ) -> Result<Vec<f32>, operators::Error> {
        let n = self.decoded.get();
        self.decoded.set(n + 1);
        if self.fail_decode == Some(n) {
            return Err(operators::Error::Parameter("injected decoder failure"));
        }
        self.calls.borrow_mut().push(Call::Decode {
            tokens: t.to_vec(),
            prefix: c.clone(),
            previous: p.map(Vec::from),
        });
        Ok(self.scripts.get(n).or(self.scripts.last()).unwrap().clone())
    }
}
fn logits(argmax: &[u32]) -> Vec<f32> {
    argmax
        .iter()
        .flat_map(|&id| (0..8).map(move |i| if i == id { 10.0 } else { f32::NEG_INFINITY }))
        .collect()
}
fn config(max: usize) -> GenerationConfig {
    GenerationConfig {
        max_new_tokens: max,
        max_context_tokens: usize::MAX,
        seed: 7,
        eos_ids: Vec::new(),
        encoder_chunk_tokens: 2,
        denoising: DenoisingConfig {
            max_steps: 2,
            ..DenoisingConfig::default()
        },
        diagnostic_canvas_length: None,
        sampling: Sampling::default(),
    }
}
fn run<B: Backend>(runner: &mut Runner<B>) -> Vec<Event> {
    let mut events = Vec::new();
    while runner.phase != Phase::Finished {
        events.push(runner.advance().unwrap());
    }
    events
}

#[test]
fn full_canvases_keep_cache_frozen_commit_argmax_and_encode_before_next_canvas() {
    let first = [4, 5, 6, 7];
    let second = [3, 2, 1, 0];
    let backend = Mock::new(vec![
        logits(&first),
        logits(&first),
        logits(&second),
        logits(&second),
    ]);
    let mut runner = Runner::new(&backend, &[2, 3, 4], config(6)).unwrap();
    let events = run(&mut runner);
    assert_eq!(runner.output.tokens, [4, 5, 6, 7, 3, 2]);
    assert_eq!(runner.output.finish_reason, Some(FinishReason::Length));
    assert_eq!(
        runner.output.usage,
        Usage {
            prompt_tokens: 3,
            completion_tokens: 6,
            encoder_passes: 4,
            decoder_passes: 4,
            completed_canvases: 2
        }
    );
    assert_eq!(runner.cache, [2, 3, 4, 4, 5, 6, 7]); // final partial canvas NOT encoded
    let calls = backend.calls.borrow();
    let decode: Vec<_> = calls
        .iter()
        .filter_map(|c| {
            if let Call::Decode {
                tokens,
                prefix,
                previous,
            } = c
            {
                Some((tokens, prefix, previous))
            } else {
                None
            }
        })
        .collect();
    assert_eq!(decode.len(), 4);
    assert!(decode.iter().all(|(t, _, _)| t.len() == 4)); // no shrinking last canvas to 2
    assert_eq!(decode[0].1, &vec![2, 3, 4]);
    assert_eq!(decode[1].1, decode[0].1);
    assert_eq!(decode[2].1, &runner.cache);
    assert_eq!(decode[3].1, decode[2].1);
    assert!(decode[0].2.is_none() && decode[2].2.is_none()); // reset per canvas
    let processed = decode[1].2.as_ref().unwrap();
    assert_eq!(processed[4], 12.5); // first temperature .8, NOT raw logit 10
    assert_eq!(
        events
            .iter()
            .filter(|e| matches!(e, Event::Commit { .. }))
            .count(),
        2
    );
    assert!(matches!(runner.advance(), Err(Error::Finished)));
}

#[test]
fn draft_eos_does_not_stop_and_final_argmax_not_noisy_canvas_is_committed() {
    let backend = Mock::new(vec![logits(&[1, 1, 1, 1]), logits(&[4, 5, 6, 7])]);
    let mut cfg = config(4);
    cfg.eos_ids = vec![1];
    let mut runner = Runner::new(&backend, &[2], cfg).unwrap();
    runner.advance().unwrap(); // prompt
    assert!(matches!(
        runner.advance().unwrap(),
        Event::Step {
            report: StepReport {
                finish_reason: None,
                ..
            },
            ..
        }
    ));
    assert!(runner.output.tokens.is_empty());
    assert_eq!(runner.phase, Phase::Denoise); // draft EOS ignored
    runner.advance().unwrap();
    let argmax = runner
        .sampler
        .as_ref()
        .unwrap()
        .committed_tokens()
        .unwrap()
        .to_vec();
    // The commitment mechanism uses this final argmax, not the RNG state.
    let event = runner.advance().unwrap();
    assert!(matches!(
        event,
        Event::Commit {
            finish_reason: Some(FinishReason::Length),
            ..
        }
    ));
    assert_eq!(runner.output.tokens, argmax);
    assert_eq!(runner.output.tokens, [4, 5, 6, 7]);
}

#[test]
fn eos_after_denoising_is_included_and_tail_outside_length_is_ignored() {
    let backend = Mock::new(vec![logits(&[4, 1, 5, 6])]);
    let mut cfg = config(9);
    cfg.eos_ids = vec![1];
    let mut runner = Runner::new(&backend, &[2], cfg).unwrap();
    run(&mut runner);
    assert_eq!(runner.output.tokens, [4, 1]);
    assert_eq!(runner.output.finish_reason, Some(FinishReason::Eos));
    assert_eq!(runner.cache, [2]);
    let backend = Mock::new(vec![logits(&[4, 5, 1, 6])]);
    let mut cfg = config(2);
    cfg.eos_ids = vec![1];
    let mut runner = Runner::new(&backend, &[2], cfg).unwrap();
    run(&mut runner);
    assert_eq!(runner.output.tokens, [4, 5]);
    assert_eq!(runner.output.finish_reason, Some(FinishReason::Length));
}

#[test]
fn adaptive_stop_and_fresh_rng_stream_are_request_reproducible() {
    let backend = Mock::new(vec![logits(&[4, 5, 6, 7])]);
    let mut cfg = config(8);
    cfg.denoising.max_steps = 5;
    let mut runner = Runner::new(&backend, &[2], cfg.clone()).unwrap();
    let events = run(&mut runner);
    assert_eq!(runner.output.usage.decoder_passes, 4); // stable/confident after 2 each
    for event in events {
        if let Event::Step { report, .. } = event
            && report.step == 2
        {
            assert_eq!(
                report.finish_reason,
                Some(diffusion::CanvasFinishReason::StableAndConfident)
            );
        }
    }
    let calls = backend.calls.borrow().clone();
    let repeat = Mock::new(vec![logits(&[4, 5, 6, 7])]);
    run(&mut Runner::new(&repeat, &[2], cfg.clone()).unwrap());
    assert_eq!(calls, *repeat.calls.borrow());
    let different = Mock::new(vec![logits(&[4, 5, 6, 7])]);
    cfg.seed += 1;
    run(&mut Runner::new(&different, &[2], cfg).unwrap());
    assert_ne!(calls, *different.calls.borrow());
}

#[test]
fn cancellation_and_failures_never_commit_unfinished_drafts() {
    let backend = Mock::new(vec![logits(&[4, 5, 6, 7])]);
    let mut runner = Runner::new(&backend, &[2], config(8)).unwrap();
    runner.advance().unwrap();
    runner.advance().unwrap();
    runner.finish(FinishReason::Cancelled);
    assert!(runner.output.tokens.is_empty());
    assert_eq!(runner.output.finish_reason, Some(FinishReason::Cancelled));
    assert!(matches!(runner.advance(), Err(Error::Finished)));
    let mut backend = Mock::new(vec![logits(&[4, 5, 6, 7])]);
    backend.fail_decode = Some(2);
    let mut runner = Runner::new(&backend, &[2], config(8)).unwrap();
    while runner.output.tokens.is_empty() {
        runner.advance().unwrap();
    }
    while runner.phase == Phase::Extend {
        runner.advance().unwrap();
    }
    assert!(runner.advance().is_err());
    assert_eq!(runner.output.tokens, [4, 5, 6, 7]);
    assert_eq!(runner.output.finish_reason, Some(FinishReason::Failed));
    assert!(runner.sampler.is_none());
    let mut backend = Mock::new(vec![logits(&[4, 5, 6, 7])]);
    backend.fail_encode = Some(2);
    let mut runner = Runner::new(&backend, &[2], config(8)).unwrap();
    while runner.output.tokens.is_empty() {
        runner.advance().unwrap();
    }
    runner.advance().unwrap(); // first completed-canvas chunk succeeds
    assert!(runner.advance().is_err());
    assert_eq!(runner.output.tokens, [4, 5, 6, 7]);
    assert_eq!(runner.output.finish_reason, Some(FinishReason::Failed));
}

#[test]
fn invalid_logits_fail_session_without_committing_and_zero_budget_does_no_work() {
    for bad in [
        vec![f32::NAN; 32],
        vec![f32::NEG_INFINITY; 32],
        vec![0.0; 31],
    ] {
        let backend = Mock::new(vec![bad]);
        let mut runner = Runner::new(&backend, &[2], config(4)).unwrap();
        runner.advance().unwrap();
        assert!(runner.advance().is_err());
        assert_eq!(runner.output.finish_reason, Some(FinishReason::Failed));
        assert!(runner.output.tokens.is_empty());
    }
    let backend = Mock::new(Vec::new());
    let mut runner = Runner::new(&backend, &[2], config(0)).unwrap();
    assert_eq!(runner.phase, Phase::Finished);
    assert!(matches!(runner.advance(), Err(Error::Finished)));
    assert_eq!(runner.output.usage.decoder_passes, 0);
    assert!(backend.calls.borrow().is_empty());
}

#[test]
fn full_final_canvas_context_and_configuration_are_admitted_before_prefill() {
    let mut backend = Mock::new(Vec::new());
    backend.geometry.context = 6;
    // Visible p=2+n=1 fits, but full canvas p=2+4 fits too; p=3+4 does NOT.
    assert!(Runner::new(&backend, &[2, 3, 4], config(1)).is_err());
    assert!(Runner::new(&backend, &[2, 3], config(5)).is_err());
    assert!(backend.calls.borrow().is_empty());
    let backend = Mock::new(Vec::new());
    assert!(Runner::new(&backend, &[], config(1)).is_err());
    assert!(Runner::new(&backend, &[8], config(1)).is_err());
    let mut cfg = config(1);
    cfg.eos_ids = vec![8];
    assert!(Runner::new(&backend, &[2], cfg).is_err());
    for size in [0, 5] {
        let mut cfg = config(1);
        cfg.diagnostic_canvas_length = Some(size);
        assert!(Runner::new(&backend, &[2], cfg).is_err());
    }
    let mut cfg = config(1);
    cfg.encoder_chunk_tokens = 0;
    assert!(Runner::new(&backend, &[2], cfg).is_err());
    let mut cfg = config(1);
    cfg.max_context_tokens = 0;
    assert!(matches!(
        Runner::new(&backend, &[2], cfg),
        Err(Error::InvalidConfig(_))
    ));
    assert!(backend.calls.borrow().is_empty());
}

#[test]
fn context_limit_counts_prompt_plus_whole_reserved_canvases() {
    // Model context 64, canvas 4. The configured limit wins when smaller,
    // the model context when the limit is larger.
    let backend = Mock::new(Vec::new());
    let mut cfg = config(5); // 2 canvases reserved = 8 tokens
    cfg.max_context_tokens = 10;
    assert!(Runner::new(&backend, &[2, 3], cfg.clone()).is_ok()); // 2 + 8 = 10
    match Runner::new(&backend, &[2, 3, 4], cfg) {
        Err(Error::ContextLength {
            prompt_tokens: 3,
            reserved_output_tokens: 8,
            limit: 10,
        }) => {}
        other => panic!("{:?}", other.err()),
    }
    let cfg = config(1); // default limit (model context) = 64
    assert!(Runner::new(&backend, &vec![2; 60], cfg.clone()).is_ok());
    match Runner::new(&backend, &vec![2; 61], cfg) {
        Err(Error::ContextLength { limit: 64, .. }) => {}
        other => panic!("{:?}", other.err()),
    }
    assert!(backend.calls.borrow().is_empty());
}

#[test]
fn tiny_real_runtime_multi_canvas_matches_independent_forward_plus_sampler() {
    use crate::engine::{
        attention::tests::fixture::Fixture,
        model::ModelDefinition,
        text::{ReferenceLimits, ReferenceText},
        weights::ModelFile,
    };
    let fixture = Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let runtime = ReferenceText::new(&file, &model, ReferenceLimits::default()).unwrap();
    let cfg = config(6);
    let prompt = [2, 5];
    let mut expected = Vec::new();
    let mut full_prompt = prompt.to_vec();
    let mut rng = ChaCha8Rng::seed_from_u64(cfg.seed);
    while expected.len() < cfg.max_new_tokens {
        let mut sampler = CanvasDenoiser::with_rng(cfg.denoising, 4, 8, rng).unwrap();
        while sampler.committed_tokens().is_none() {
            let (_, _, raw) = fixture.expected_text(
                &model,
                &full_prompt,
                sampler.current_canvas(),
                sampler.self_conditioning_logits(),
            );
            sampler.advance(&raw).unwrap();
        }
        let committed = sampler.committed_tokens().unwrap().to_vec();
        let commit = diffusion::commit_canvas(
            &committed,
            cfg.max_new_tokens - expected.len(),
            &cfg.eos_ids,
        );
        expected.extend(commit.tokens);
        full_prompt.extend(committed);
        rng = sampler.into_rng();
    }
    let mut session = GenerationSession::new(&runtime, &prompt, cfg).unwrap();
    assert!(session.result().is_none());
    while !session.is_finished() {
        session.advance().unwrap();
    }
    assert_eq!(session.result().unwrap().tokens, expected);
    assert_eq!(session.cached_tokens(), 6); // 2 prompt + first 4-token canvas only
    assert_eq!(
        session.result().unwrap().finish_reason,
        Some(FinishReason::Length)
    );
    session.cancel();
    assert_eq!(
        session.result().unwrap().finish_reason,
        Some(FinishReason::Length)
    );
}

#[test]
fn real_runtime_capacity_is_checked_before_model_execution() {
    use crate::engine::{
        attention::tests::fixture::Fixture,
        model::ModelDefinition,
        text::{ReferenceLimits, ReferenceText},
        weights::ModelFile,
    };
    let fixture = Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let runtime = ReferenceText::new(
        &file,
        &model,
        ReferenceLimits {
            max_logit_elements: 31,
            ..ReferenceLimits::default()
        },
    )
    .unwrap();
    assert!(GenerationSession::new(&runtime, &[2], config(1)).is_err()); // full 4x8 logits, not shortened 1x8
    let runtime = ReferenceText::new(
        &file,
        &model,
        ReferenceLimits {
            cache_bytes_per_layer: 128,
            ..ReferenceLimits::default()
        },
    )
    .unwrap();
    assert!(GenerationSession::new(&runtime, &[2], config(1)).is_err()); // global staging 5*4*8=160 bytes
}

#[test]
#[ignore = "slow real model two-step short-canvas denoising diagnostic; set POWER_LLM_TEST_MODEL"]
fn actual_diffusiongemma_two_step_denoising_session() {
    use crate::engine::{
        model::ModelDefinition,
        text::{ReferenceLimits, ReferenceText},
        weights::ModelFile,
    };
    let file =
        ModelFile::open(std::env::var_os("POWER_LLM_TEST_MODEL").expect("model path")).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let runtime = ReferenceText::new(&file, &model, ReferenceLimits::default()).unwrap();
    let cfg = GenerationConfig {
        max_new_tokens: 1,
        seed: 7,
        denoising: DenoisingConfig {
            max_steps: 2,
            ..DenoisingConfig::default()
        },
        diagnostic_canvas_length: Some(1),
        ..GenerationConfig::default()
    };
    let mut session = GenerationSession::new(&runtime, &[model.vocabulary.bos], cfg).unwrap();
    let mut commits = 0;
    let mut steps = 0;
    while !session.is_finished() {
        match session.advance().unwrap() {
            Event::Step { report, .. } => {
                steps += 1;
                assert_eq!(report.step, steps);
                assert_eq!(session.cached_tokens(), 1);
                assert!(session.committed_tokens().is_empty());
            }
            Event::Commit { tokens, .. } => {
                commits += 1;
                assert_eq!(tokens.len(), 1);
            }
            Event::Encoded { .. } => {}
        }
    }
    assert_eq!(steps, 2);
    assert_eq!(commits, 1);
    assert_eq!(session.cached_tokens(), 1);
    assert_eq!(session.result().unwrap().usage.decoder_passes, 2);
    println!(
        "Real two-step short-canvas session committed {:?}; fixed encoder cache and processed-logit self-conditioning exercised",
        session.result().unwrap().tokens
    );
    println!(
        "NOT independent real-model parity, trained 256-position generation, or model-quality/throughput validation"
    );
}

#[test]
#[ignore = "real model admission only (no execution); set POWER_LLM_TEST_MODEL"]
fn actual_diffusiongemma_admits_full_max_context_request() {
    use crate::engine::{
        model::ModelDefinition,
        text::{ReferenceLimits, ReferenceText},
        weights::ModelFile,
    };
    let file =
        ModelFile::open(std::env::var_os("POWER_LLM_TEST_MODEL").expect("model path")).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let bos = model.vocabulary.bos;
    // A configured serving limit (here 4096) is enforced exactly.
    let runtime = ReferenceText::new(&file, &model, ReferenceLimits::default()).unwrap();
    let cfg = GenerationConfig {
        max_context_tokens: 4096,
        ..GenerationConfig::default()
    };
    assert!(GenerationSession::new(&runtime, &vec![bos; 4096 - 256], cfg.clone()).is_ok());
    assert!(matches!(
        GenerationSession::new(&runtime, &vec![bos; 4096 - 255], cfg),
        Err(Error::ContextLength { limit: 4096, .. })
    ));
    // Without a serving limit, limits sized for 32768 tokens admit them; the
    // default per-layer budget does not (a resource error, not a context error).
    let limits = ReferenceLimits::for_context(&model.spec, 32768);
    let runtime = ReferenceText::new(&file, &model, limits).unwrap();
    let long = vec![bos; 32768 - 256];
    assert!(GenerationSession::new(&runtime, &long, GenerationConfig::default()).is_ok());
    let small = ReferenceText::new(&file, &model, ReferenceLimits::default()).unwrap();
    assert!(matches!(
        GenerationSession::new(&small, &long, GenerationConfig::default()),
        Err(Error::Backend(_) | Error::ResourceLimit)
    ));
    // The model context itself is the final bound.
    let over = vec![bos; model.spec.context - 255];
    let huge = ReferenceText::new(
        &file,
        &model,
        ReferenceLimits::for_context(&model.spec, model.spec.context),
    )
    .unwrap();
    assert!(matches!(
        GenerationSession::new(&huge, &over, GenerationConfig::default()),
        Err(Error::ContextLength { .. })
    ));
}
