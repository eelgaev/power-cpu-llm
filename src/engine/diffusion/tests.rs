use super::*;

fn small(config: DenoisingConfig, seed: u64) -> CanvasDenoiser {
    CanvasDenoiser::new(config, 4, 3, seed).unwrap()
}

#[test]
fn defaults_match_pinned_official_generation_configuration() {
    let reference: serde_json::Value =
        serde_json::from_str(include_str!("../../../fixtures/generation_config.json")).unwrap();
    let config = DenoisingConfig::default();
    assert_eq!(
        config.max_steps as u64,
        reference["max_denoising_steps"].as_u64().unwrap()
    );
    assert_eq!(config.temperature_max, reference["t_max"].as_f64().unwrap());
    assert_eq!(config.temperature_min, reference["t_min"].as_f64().unwrap());
    assert_eq!(
        config.entropy_bound,
        reference["sampler_config"]["entropy_bound"]
            .as_f64()
            .unwrap()
    );
    assert_eq!(
        config.confidence_threshold,
        reference["confidence_threshold"].as_f64().unwrap()
    );
    assert_eq!(
        config.stability_steps as u64,
        reference["stability_threshold"].as_u64().unwrap()
    );
    let eos: Vec<_> = reference["eos_token_id"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(EOS_TOKEN_IDS, eos);
}

#[test]
fn schedule_uses_remaining_steps_including_one_not_zero() {
    let config = DenoisingConfig::default();
    assert_eq!(config.temperature(48).unwrap(), 0.8);
    assert!((config.temperature(1).unwrap() - (0.4 + 0.4 / 48.0)).abs() < 1e-12);
    assert!(config.temperature(0).is_err());
    assert!(config.temperature(49).is_err());
}

#[test]
fn acceptance_uses_joint_entropy_bound_not_individual_thresholds() {
    assert_eq!(
        entropy_selection(&[0.09, 0.01, 0.03, 0.2], 0.05),
        [true, true, true, false]
    );
    assert_eq!(
        entropy_selection(&[0.125, 0.25, 0.5], 0.125),
        [true, true, false]
    );
    assert_eq!(
        entropy_selection(&[2.0; 4], 0.1),
        [true, false, false, false]
    );
    assert_eq!(entropy_selection(&[0.0; 4], 0.1), [true; 4]);
}

#[test]
fn entropy_is_stable_and_supports_masked_vocabulary() {
    let uniform = Distribution::new(&[0.0; 4]).unwrap();
    assert!((uniform.entropy - crate::libm::ln(4.0)).abs() < 1e-12);
    assert_eq!(uniform.argmax, 0);
    let shifted = Distribution::new(&[10_000.0, 9_999.0]).unwrap();
    let unshifted = Distribution::new(&[0.0, -1.0]).unwrap();
    assert!((shifted.entropy - unshifted.entropy).abs() < 1e-12);
    let masked = Distribution::new(&[f32::NEG_INFINITY, 0.0, f32::NEG_INFINITY]).unwrap();
    assert_eq!(masked.entropy, 0.0);
    let mut rng = ChaCha8Rng::seed_from_u64(1);
    for _ in 0..100 {
        assert_eq!(masked.sample(rng.random::<f64>()), 1);
    }
    for invalid in [
        [f32::NAN, 0.0],
        [f32::INFINITY, 0.0],
        [f32::NEG_INFINITY; 2],
    ] {
        assert!(Distribution::new(&invalid).is_none());
    }
}

#[test]
fn stable_and_confident_requires_both_and_commits_argmax() {
    let mut state = small(DenoisingConfig::default(), 42);
    let logits = [40.0, -40.0, -40.0].repeat(4);
    assert!(state.self_conditioning_logits().is_none());
    assert!(state.committed_tokens().is_none());
    let first = state.advance(&logits).unwrap();
    assert_eq!(first.finish_reason, None); // no previous predictions yet
    assert_eq!(first.accepted_positions, 4);
    assert_eq!(state.self_conditioning_logits().unwrap()[0], 50.0);
    let second = state.advance(&logits).unwrap();
    assert_eq!(
        second.finish_reason,
        Some(CanvasFinishReason::StableAndConfident)
    );
    assert_eq!(state.committed_tokens().unwrap(), [0; 4]);
    assert_eq!(state.advance(&logits).unwrap_err(), Error::Finished);
}

#[test]
fn uncertain_stable_argmax_does_not_stop_early() {
    let config = DenoisingConfig {
        max_steps: 3,
        ..DenoisingConfig::default()
    };
    let mut state = small(config, 99);
    let logits = [0.0; 12];
    assert_eq!(state.advance(&logits).unwrap().finish_reason, None);
    assert_eq!(state.advance(&logits).unwrap().finish_reason, None);
    let last = state.advance(&logits).unwrap();
    assert_eq!(last.finish_reason, Some(CanvasFinishReason::StepLimit));
    assert_eq!(last.accepted_positions, 1);
    // Final output is argmax, not the sampled/noisy current canvas.
    assert_eq!(state.committed_tokens().unwrap(), [0; 4]);
}

#[test]
fn stability_history_is_per_canvas_and_respects_threshold() {
    let config = DenoisingConfig {
        stability_steps: 2,
        ..DenoisingConfig::default()
    };
    let mut state = small(config, 1);
    let zero = [40.0, -40.0, -40.0].repeat(4);
    let one = [-40.0, 40.0, -40.0].repeat(4);
    assert_eq!(state.advance(&zero).unwrap().finish_reason, None);
    assert_eq!(state.advance(&one).unwrap().finish_reason, None);
    assert_eq!(state.advance(&one).unwrap().finish_reason, None);
    assert_eq!(
        state.advance(&one).unwrap().finish_reason,
        Some(CanvasFinishReason::StableAndConfident)
    );
    let mut fresh = small(config, 1);
    assert_eq!(fresh.advance(&one).unwrap().finish_reason, None);
    let no_history = DenoisingConfig {
        stability_steps: 0,
        ..config
    };
    assert_eq!(
        small(no_history, 1).advance(&one).unwrap().finish_reason,
        Some(CanvasFinishReason::StableAndConfident)
    );
}

#[test]
fn acceptance_is_recomputed_and_not_frozen_across_steps() {
    let mut state = small(DenoisingConfig::default(), 2);
    state.advance(&[40.0, -40.0, -40.0].repeat(4)).unwrap();
    let second = state.advance(&[0.0; 12]).unwrap();
    assert_eq!(second.accepted_positions, 1);
    assert_eq!(second.finish_reason, None);
}

#[test]
fn same_seed_reproduces_noise_and_sampling() {
    let mut a = small(DenoisingConfig::default(), 1);
    let mut b = small(DenoisingConfig::default(), 1);
    assert_eq!(a.current_canvas(), b.current_canvas());
    assert!(a.current_canvas().iter().all(|&x| x < 3));
    for _ in 0..3 {
        a.advance(&[0.0; 12]).unwrap();
        b.advance(&[0.0; 12]).unwrap();
        assert_eq!(a.current_canvas(), b.current_canvas());
    }
}

#[test]
fn invalid_logits_do_not_consume_rng_or_advance_state() {
    let mut a = small(DenoisingConfig::default(), 7);
    let mut b = small(DenoisingConfig::default(), 7);
    assert!(matches!(
        a.advance(&[0.0; 2]),
        Err(Error::WrongLogitsShape { .. })
    ));
    let mut invalid = [0.0; 12];
    invalid[11] = f32::NAN;
    assert_eq!(
        a.advance(&invalid).unwrap_err(),
        Error::InvalidLogits { position: 3 }
    );
    assert_eq!(a.completed_steps(), 0);
    assert!(a.self_conditioning_logits().is_none());
    a.advance(&[0.0; 12]).unwrap();
    b.advance(&[0.0; 12]).unwrap();
    assert_eq!(a.current_canvas(), b.current_canvas());
}

#[test]
fn invalid_configs_and_dimensions_are_rejected() {
    let base = DenoisingConfig::default();
    for invalid in [
        DenoisingConfig {
            max_steps: 0,
            ..base
        },
        DenoisingConfig {
            temperature_min: 0.0,
            ..base
        },
        DenoisingConfig {
            temperature_max: f64::NAN,
            ..base
        },
        DenoisingConfig {
            temperature_max: 0.1,
            ..base
        },
        DenoisingConfig {
            entropy_bound: -1.0,
            ..base
        },
        DenoisingConfig {
            confidence_threshold: f64::INFINITY,
            ..base
        },
    ] {
        assert!(CanvasDenoiser::new(invalid, 4, 3, 1).is_err());
    }
    assert!(CanvasDenoiser::new(base, 0, 3, 1).is_err());
    assert!(CanvasDenoiser::new(base, 3, 0, 1).is_err());
    assert!(CanvasDenoiser::new(base, usize::MAX, 3, 1).is_err());
}

#[test]
fn committed_canvas_honors_eos_and_length_without_draft_stopping() {
    assert_eq!(
        commit_canvas(&[8, 1, 9, 106], 10, EOS_TOKEN_IDS),
        CanvasCommit {
            tokens: vec![8, 1],
            finish_reason: Some(GenerationFinishReason::Eos),
        }
    );
    assert_eq!(
        commit_canvas(&[8, 9, 1, 5], 2, EOS_TOKEN_IDS),
        CanvasCommit {
            tokens: vec![8, 9],
            finish_reason: Some(GenerationFinishReason::Length),
        }
    );
    assert_eq!(commit_canvas(&[8, 9], 3, EOS_TOKEN_IDS).finish_reason, None);
    assert_eq!(
        commit_canvas(&[8, 9], 0, EOS_TOKEN_IDS),
        CanvasCommit {
            tokens: vec![],
            finish_reason: Some(GenerationFinishReason::Length),
        }
    );
}
