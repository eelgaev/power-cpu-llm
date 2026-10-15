use super::*;

fn identity(n: usize) -> Matrix {
    Matrix::new(
        n,
        n,
        (0..n * n)
            .map(|i| if i / n == i % n { 1.0 } else { 0.0 })
            .collect(),
    )
    .unwrap()
}

#[test]
fn router_renormalizes_before_expert_scaling_and_breaks_ties_by_index() {
    let matrix = Matrix::new(3, 2, vec![0.0; 6]).unwrap();
    let router = Router::new(matrix, vec![1.0; 2], vec![2.0, 3.0, 4.0], 2, 1e-6).unwrap();
    let result = router.route(&[0.0; 2]).unwrap();
    assert_eq!(
        result.selected,
        [
            ExpertWeight {
                expert: 0,
                weight: 1.0
            },
            ExpertWeight {
                expert: 1,
                weight: 1.5
            }
        ]
    );
    assert!((result.probabilities.iter().sum::<f32>() - 1.0).abs() < 1e-6);
}

#[test]
fn fused_expert_splits_gate_and_up_into_contiguous_halves() {
    let gate_up = Matrix::new(4, 2, vec![1.0, 0.0, 0.0, 1.0, 2.0, 0.0, 0.0, 3.0]).unwrap();
    let output = fused_expert(&gate_up, &identity(2), &[0.5, -1.0]).unwrap();
    let gelu = |x: f64| {
        0.5 * x
            * (1.0
                + crate::libm::tanh(
                    (2.0 / std::f64::consts::PI).sqrt() * (x + 0.044715 * x.powi(3)),
                ))
    };
    assert!((f64::from(output[0]) - gelu(0.5)).abs() < 1e-6);
    assert!((f64::from(output[1]) - gelu(-1.0) * -3.0).abs() < 1e-6);
    assert!(fused_expert(&identity(2), &identity(2), &[1.0; 2]).is_err());
}

#[test]
fn routing_uses_rms_hidden_scale_and_inverse_root_width() {
    let matrix = Matrix::new(2, 2, vec![1.0, 0.0, 0.0, 1.0]).unwrap();
    let router = Router::new(matrix, vec![2.0, 1.0], vec![1.0; 2], 1, 0.5).unwrap();
    let result = router.route(&[3.0, 4.0]).unwrap();
    let factor = 1.0 / 13.0_f64.sqrt() / 2.0_f64.sqrt();
    let expected = crate::libm::exp(6.0 * factor)
        / (crate::libm::exp(6.0 * factor) + crate::libm::exp(4.0 * factor));
    assert!((f64::from(result.probabilities[0]) - expected).abs() < 1e-6);
    assert_eq!(
        result.selected,
        [ExpertWeight {
            expert: 0,
            weight: 1.0
        }]
    );
}

#[test]
fn self_conditioning_zero_signal_still_post_normalizes() {
    let mlp = GatedMlp::new(identity(2), identity(2), identity(2)).unwrap();
    let sc = SelfConditioner::new(vec![1.0; 2], mlp, 0.5).unwrap();
    assert_eq!(
        sc.apply(&[3.0, 4.0], None).unwrap(),
        math::rms_norm(&[3.0, 4.0], None, 0.5).unwrap()
    );
    assert_ne!(sc.apply(&[3.0, 4.0], None).unwrap(), vec![3.0, 4.0]);
}

#[test]
fn self_conditioning_matches_independent_small_reference() {
    let mlp = GatedMlp::new(identity(2), identity(2), identity(2)).unwrap();
    let sc = SelfConditioner::new(vec![2.0, 0.5], mlp, 0.5).unwrap();
    let x = [1.0_f32, -2.0];
    let soft = [3.0_f32, 4.0];
    let normed = [6.0 / 13.0_f64.sqrt(), 2.0 / 13.0_f64.sqrt()];
    let combined: Vec<_> = normed
        .iter()
        .zip(x)
        .map(|(&v, x)| {
            let gelu = 0.5
                * v
                * (1.0
                    + crate::libm::tanh(
                        (2.0 / std::f64::consts::PI).sqrt() * (v + 0.044715 * v.powi(3)),
                    ));
            f64::from(x) + gelu * v
        })
        .collect();
    let inv = 1.0 / ((combined[0].powi(2) + combined[1].powi(2)) / 2.0 + 0.5).sqrt();
    for (actual, expected) in sc.apply(&x, Some(&soft)).unwrap().iter().zip(combined) {
        assert!((f64::from(*actual) - expected * inv).abs() < 1e-6);
    }
}

#[test]
#[ignore = "set POWER_LLM_TEST_MODEL to the completed target GGUF"]
fn actual_diffusiongemma_router_and_self_conditioning() {
    use crate::engine::model::ModelDefinition;
    let file =
        ModelFile::open(std::env::var_os("POWER_LLM_TEST_MODEL").expect("model path")).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let spec = &model.spec;
    let raw = file
        .read_row(&model.embedding.name, u64::from(model.vocabulary.bos))
        .unwrap()
        .decode()
        .unwrap();
    let embedding =
        math::multiply_scaled(&raw, &vec![1.0; spec.hidden], (spec.hidden as f32).sqrt()).unwrap();
    let router = Router::load(
        &file,
        &model.layers[0].moe.as_ref().unwrap().router,
        spec.top_k,
        spec.rms_epsilon,
    )
    .unwrap();
    let routing = router.route(&embedding).unwrap();

    // Independent scalar Rust equations over actual F32 router rows. Do not
    // call our C normalization/projection/softmax to obtain expected values.
    let scale = file
        .read_row(
            &model.layers[0]
                .moe
                .as_ref()
                .unwrap()
                .router
                .hidden_scale
                .name,
            0,
        )
        .unwrap()
        .decode()
        .unwrap();
    let expert_scale = file
        .read_row(
            &model.layers[0]
                .moe
                .as_ref()
                .unwrap()
                .router
                .expert_scale
                .name,
            0,
        )
        .unwrap()
        .decode()
        .unwrap();
    let inv = 1.0
        / (embedding.iter().map(|&x| f64::from(x).powi(2)).sum::<f64>() / embedding.len() as f64
            + f64::from(spec.rms_epsilon))
        .sqrt();
    let factor = 1.0 / (spec.hidden as f32).sqrt();
    let normalized: Vec<_> = embedding
        .iter()
        .zip(scale)
        .map(|(&x, scale)| ((f64::from(x) * inv) as f32 * scale) * factor)
        .collect();
    let scores: Vec<_> = (0..spec.expert_count)
        .map(|row| {
            let w = file
                .read_row(
                    &model.layers[0].moe.as_ref().unwrap().router.projection.name,
                    row as u64,
                )
                .unwrap()
                .decode()
                .unwrap();
            w.iter()
                .zip(&normalized)
                .map(|(&w, &x)| f64::from(w) * f64::from(x))
                .sum::<f64>() as f32
        })
        .collect();
    let max = scores.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let exponentials: Vec<_> = scores
        .iter()
        .map(|&s| crate::libm::exp(f64::from(s) - f64::from(max)))
        .collect();
    let total: f64 = exponentials.iter().sum();
    let probabilities: Vec<_> = exponentials.iter().map(|&p| (p / total) as f32).collect();
    let mut order: Vec<_> = (0..probabilities.len()).collect();
    order.sort_by(|&a, &b| {
        probabilities[b]
            .total_cmp(&probabilities[a])
            .then(a.cmp(&b))
    });
    let selected_sum: f64 = order[..spec.top_k]
        .iter()
        .map(|&i| f64::from(probabilities[i]))
        .sum();
    for (&a, &b) in routing.probabilities.iter().zip(&probabilities) {
        assert!((a - b).abs() < 1e-6);
    }
    for (actual, &expert) in routing.selected.iter().zip(&order) {
        assert_eq!(actual.expert, expert);
        let expected =
            (f64::from(probabilities[expert]) / selected_sum) as f32 * expert_scale[expert];
        assert!((actual.weight - expected).abs() < 1e-6);
    }

    let mlp = GatedMlp::load(&file, model.self_conditioning_mlp.as_ref().unwrap()).unwrap();
    let sc = SelfConditioner::new(
        load_vector(&file, model.self_conditioning_pre_norm.unwrap()).unwrap(),
        mlp,
        spec.rms_epsilon,
    )
    .unwrap();
    let zero = sc.apply(&embedding, None).unwrap();
    let expected = math::rms_norm(&embedding, None, spec.rms_epsilon).unwrap();
    assert_eq!(zero, expected);
    let synthetic: Vec<_> = (0..spec.hidden)
        .map(|i| (i % 17) as f32 / 17.0 - 0.5)
        .collect();
    let conditioned = sc.apply(&embedding, Some(&synthetic)).unwrap();
    assert!(conditioned.iter().all(|x| x.is_finite()));
    let rms = (conditioned
        .iter()
        .map(|&x| f64::from(x).powi(2))
        .sum::<f64>()
        / spec.hidden as f64)
        .sqrt();
    assert!((rms - 1.0).abs() < 1e-4);
    assert!(
        conditioned
            .iter()
            .zip(&zero)
            .any(|(a, b)| (a - b).abs() > 1e-4)
    );
    assert!(matches!(
        load_matrix(&file, model.embedding, MAX_REFERENCE_MATRIX_ELEMENTS),
        Err(Error::ResourceLimit)
    ));
    println!(
        "Actual layer-0 router matches independent equations; decoder self-conditioning probes finite and normalized"
    );
}

#[test]
#[ignore = "set POWER_LLM_TEST_MODEL to the completed target GGUF"]
fn actual_diffusiongemma_feed_forward_and_expert_slices() {
    let file =
        ModelFile::open(std::env::var_os("POWER_LLM_TEST_MODEL").expect("model path")).unwrap();
    let model = crate::engine::model::ModelDefinition::bind(file.inventory()).unwrap();
    let spec = &model.spec;
    let layer = &model.layers[0];
    let input: Vec<_> = (0..spec.hidden)
        .map(|i| (i % 17) as f32 / 17.0 - 0.5)
        .collect();
    let decoder = feed_forward(&file, layer, spec, &input, LayerMode::Decoder).unwrap();
    let encoder = feed_forward(&file, layer, spec, &input, LayerMode::Encoder).unwrap();
    assert_eq!(decoder.hidden.len(), spec.hidden);
    assert_eq!(decoder.routing.selected.len(), 8);
    assert_eq!(decoder.routing.selected, encoder.routing.selected);
    assert!(
        decoder
            .hidden
            .iter()
            .chain(&encoder.hidden)
            .all(|x| x.is_finite())
    );
    let dec_scale = load_vector(&file, layer.decoder_scale).unwrap()[0];
    let enc_scale = load_vector(&file, layer.encoder_scale).unwrap()[0];
    if dec_scale != 0.0 && enc_scale != 0.0 {
        for (d, e) in decoder.hidden.iter().zip(&encoder.hidden) {
            let d = d / dec_scale;
            let e = e / enc_scale;
            assert!((d - e).abs() <= 1e-5 * (1.0 + d.abs()));
        }
    }
    assert!(
        load_expert_matrix(
            &file,
            layer.moe.as_ref().unwrap().expert_gate_up,
            spec.expert_count,
            MAX_REFERENCE_MATRIX_ELEMENTS
        )
        .is_err()
    );
    assert!(load_expert_matrix(&file, layer.moe.as_ref().unwrap().expert_gate_up, 0, 1).is_err());
    assert!(
        load_matrix(
            &file,
            layer.moe.as_ref().unwrap().expert_gate_up,
            MAX_REFERENCE_MATRIX_ELEMENTS
        )
        .is_err()
    );
    println!(
        "Actual layer-0 shared/routed FFN finite, modes differ only by their layer scalar; expert bounds checked"
    );
}

#[test]
fn operator_shapes_and_invalid_values_are_checked() {
    assert!(GatedMlp::new(identity(2), identity(3), identity(2)).is_err());
    assert!(Router::new(identity(2), vec![1.0; 2], vec![1.0; 2], 0, 1e-6).is_err());
    assert!(Router::new(identity(2), vec![1.0; 2], vec![1.0; 2], 3, 1e-6).is_err());
    assert!(Router::new(identity(2), vec![1.0; 1], vec![1.0; 2], 1, 1e-6).is_err());
    let router = Router::new(identity(2), vec![1.0; 2], vec![1.0; 2], 1, 1e-6).unwrap();
    assert!(router.route(&[f32::NAN, 1.0]).is_err());
    let sc = SelfConditioner::new(
        vec![1.0; 2],
        GatedMlp::new(identity(2), identity(2), identity(2)).unwrap(),
        1e-6,
    )
    .unwrap();
    assert!(sc.apply(&[1.0], None).is_err());
    assert!(sc.apply(&[1.0; 2], Some(&[1.0])).is_err());
}
