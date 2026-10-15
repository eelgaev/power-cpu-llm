use super::*;
use crate::engine::attention::tests::fixture::Fixture;

pub(super) fn close_rows(a: &[Vec<f32>], b: &[Vec<f32>]) {
    assert_eq!(a.len(), b.len());
    for (a, b) in a.iter().zip(b) {
        close(a, b);
    }
}
fn close(a: &[f32], b: &[f32]) {
    assert_eq!(a.len(), b.len());
    for (&a, &b) in a.iter().zip(b) {
        assert!((a - b).abs() <= 1e-5 * (1.0 + b.abs()), "{a} != {b}");
    }
}
pub(super) fn snapshot(cache: &EncoderCache<'_>) -> Vec<(usize, Vec<f32>, Vec<f32>)> {
    cache
        .layers
        .iter()
        .map(|c| (c.total_tokens(), c.keys().to_vec(), c.values().to_vec()))
        .collect()
}

#[test]
fn complete_token_id_encoder_decoder_and_logits_match_independent_oracle() {
    let fixture = Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let runtime = ReferenceText::new(&file, &model, ReferenceLimits::default()).unwrap();
    let mut cache = runtime.new_cache().unwrap();
    let prompt = [2, 5, 6, 7];
    let canvas = [1, 4];
    let encoded = runtime.encode(&prompt, &mut cache).unwrap();
    let before = snapshot(&cache);
    let (encoder_expected, hidden_expected, logits_expected) =
        fixture.expected_text(&model, &prompt, &canvas, None);
    close_rows(&encoded, &encoder_expected);
    close_rows(
        &runtime.decode_hidden(&canvas, &cache, None).unwrap(),
        &hidden_expected,
    );
    close(
        &runtime.decode_logits(&canvas, &cache, None).unwrap(),
        &logits_expected,
    );
    assert_eq!(cache.total_tokens(), 4);
    assert_eq!(cache.layer(0).unwrap().retained_tokens(), 2);
    assert_eq!(cache.layer(1).unwrap().retained_tokens(), 4);
    assert_eq!(snapshot(&cache), before);
    let previous: Vec<_> = (0..16).map(|i| (i % 7) as f32 * 0.4 - 1.0).collect();
    let (_, hidden_expected, logits_expected) =
        fixture.expected_text(&model, &prompt, &canvas, Some(&previous));
    close_rows(
        &runtime
            .decode_hidden(&canvas, &cache, Some(&previous))
            .unwrap(),
        &hidden_expected,
    );
    close(
        &runtime
            .decode_logits(&canvas, &cache, Some(&previous))
            .unwrap(),
        &logits_expected,
    );
    close_rows(
        &runtime.soft_embeddings(&previous, 2).unwrap(),
        &fixture.expected_soft(&previous),
    );
    assert_eq!(snapshot(&cache), before);
    // Whole-model causal chunking, including final norm and all cache layers.
    let mut chunked = runtime.new_cache().unwrap();
    let mut output = runtime.encode(&prompt[..1], &mut chunked).unwrap();
    output.extend(runtime.encode(&prompt[1..], &mut chunked).unwrap());
    close_rows(&output, &encoded);
    assert_eq!(snapshot(&chunked), before);
    close(
        &runtime.decode_logits(&canvas, &chunked, None).unwrap(),
        &fixture.expected_text(&model, &prompt, &canvas, None).2,
    );
}

#[test]
fn soft_embeddings_match_scaled_lookup_and_support_masking() {
    let fixture = Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let runtime = ReferenceText::new(&file, &model, ReferenceLimits::default()).unwrap();
    let mut logits = vec![f32::NEG_INFINITY; 8];
    logits[3] = 0.0;
    assert_eq!(
        runtime.soft_embeddings(&logits, 1).unwrap()[0],
        fixture
            .raw_embedding(3)
            .iter()
            .map(|x| x * 2.0)
            .collect::<Vec<_>>()
    );
    assert!(runtime.soft_embeddings(&[f32::NEG_INFINITY; 8], 1).is_err());
    assert!(runtime.soft_embeddings(&[f32::NAN; 8], 1).is_err());
    assert!(runtime.soft_embeddings(&[f32::INFINITY; 8], 1).is_err());
    assert!(runtime.soft_embeddings(&[], 1).is_err());
    assert!(runtime.soft_embeddings(&[0.0; 8], 0).is_err());
}

#[test]
fn whole_model_cache_commit_is_transactional_on_late_layer_and_final_norm_failure() {
    let fixture = Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    // Local layer can stage two tokens in 32 bytes; global layer cannot.
    let limits = ReferenceLimits {
        cache_bytes_per_layer: 32,
        ..ReferenceLimits::default()
    };
    let runtime = ReferenceText::new(&file, &model, limits).unwrap();
    let mut cache = runtime.new_cache().unwrap();
    runtime.encode(&[2], &mut cache).unwrap();
    let before = snapshot(&cache);
    assert!(matches!(
        runtime.encode(&[5], &mut cache),
        Err(Error::ResourceLimit)
    ));
    assert_eq!(snapshot(&cache), before);
    assert!(cache.layers.iter().all(|c| c.total_tokens() == 1));

    let mut runtime = ReferenceText::new(&file, &model, ReferenceLimits::default()).unwrap();
    let mut cache = runtime.new_cache().unwrap();
    runtime.encode(&[2], &mut cache).unwrap();
    let before = snapshot(&cache);
    // Test-only poison of final norm, after every layer has prepared updates.
    // No real model file is modified.
    runtime.output_scale[0] = f32::NAN;
    assert!(runtime.encode(&[5], &mut cache).is_err());
    assert_eq!(snapshot(&cache), before);
}

#[test]
fn reference_limits_context_owners_and_invalid_inputs_are_rejected() {
    let fixture = Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let other_model = ModelDefinition::bind(file.inventory()).unwrap();
    let runtime = ReferenceText::new(&file, &model, ReferenceLimits::default()).unwrap();
    let other = ReferenceText::new(&file, &other_model, ReferenceLimits::default()).unwrap();
    let mut cache = runtime.new_cache().unwrap();
    assert!(runtime.decode_logits(&[2], &cache, None).is_err()); // encoder required
    assert!(runtime.encode(&[], &mut cache).is_err());
    assert!(runtime.encode(&[8], &mut cache).is_err());
    assert!(other.encode(&[2], &mut cache).is_err());
    runtime.encode(&[2], &mut cache).unwrap();
    let before = snapshot(&cache);
    assert!(runtime.decode_logits(&[2; 5], &cache, None).is_err());
    assert!(
        runtime
            .decode_logits(&[2], &cache, Some(&[0.0; 7]))
            .is_err()
    );
    assert!(runtime.logits(&[vec![0.0; 3]]).is_err());
    assert!(runtime.logits(&[vec![f32::NAN; 4]]).is_err());
    assert_eq!(snapshot(&cache), before);
    let limited = ReferenceText::new(
        &file,
        &model,
        ReferenceLimits {
            max_logit_elements: 7,
            max_hidden_elements: 4,
            ..ReferenceLimits::default()
        },
    )
    .unwrap();
    let mut small_cache = limited.new_cache().unwrap();
    assert!(limited.encode(&[2; 2], &mut small_cache).is_err());
    limited.encode(&[2], &mut small_cache).unwrap();
    assert!(matches!(
        limited.decode_logits(&[2], &small_cache, None),
        Err(Error::ResourceLimit)
    ));
}

#[test]
#[ignore = "set POWER_LLM_TEST_MODEL; slow full 30-layer one-position reference diagnostic"]
fn actual_diffusiongemma_full_reference_encoder_decoder_logits() {
    let file =
        ModelFile::open(std::env::var_os("POWER_LLM_TEST_MODEL").expect("model path")).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let runtime = ReferenceText::new(&file, &model, ReferenceLimits::default()).unwrap();
    let mut cache = runtime.new_cache().unwrap();
    let encoded = runtime.encode(&[model.vocabulary.bos], &mut cache).unwrap();
    assert_eq!(encoded[0].len(), 2816);
    assert!(encoded.iter().flatten().all(|x| x.is_finite()));
    assert!(
        cache
            .layers
            .iter()
            .all(|c| c.total_tokens() == 1 && c.retained_tokens() == 1)
    );
    let before = snapshot(&cache);
    // Real Q6_K embedding transpose path: a one-hot distribution must yield
    // sqrt(hidden)-scaled token lookup, as the official decoder specifies.
    let mut processed = vec![f32::NEG_INFINITY; model.spec.vocabulary];
    processed[42] = 0.0;
    let soft = runtime.soft_embeddings(&processed, 1).unwrap();
    assert_eq!(
        soft[0],
        file.read_row(&model.embedding.name, 42)
            .unwrap()
            .decode()
            .unwrap()
            .iter()
            .map(|x| x * (model.spec.hidden as f32).sqrt())
            .collect::<Vec<_>>()
    );
    let hidden = runtime.decode_hidden(&[42], &cache, None).unwrap();
    assert!(hidden.iter().flatten().all(|x| x.is_finite()));
    let logits = runtime.logits(&hidden).unwrap();
    assert_eq!(logits.len(), 262144);
    assert!(logits.iter().all(|&x| x.is_finite() && x.abs() <= 30.0));
    assert_eq!(snapshot(&cache), before);
    let best = logits
        .iter()
        .enumerate()
        .max_by(|(a, x), (b, y)| x.total_cmp(y).then(b.cmp(a)))
        .unwrap();
    println!(
        "Actual full 30-layer BOS encoder / one-position decoder: finite hidden states and {} logits; diagnostic argmax ID={} score={:.6}",
        logits.len(),
        best.0,
        best.1
    );
    println!(
        "Short-canvas/reference smoke test only: NOT independently validated real-model logits, generated text, or throughput"
    );
}

#[test]
fn prefill_cannot_drop_residual_precision() {
    use crate::engine::execution::MatrixBackend;
    let fixture = Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let mut runtime = ReferenceText::new_with_backend(
        &file,
        &model,
        ReferenceLimits::default(),
        1,
        MatrixBackend::PortableInt8x2,
    )
    .unwrap();
    runtime
        .set_prefill_backend(Some(MatrixBackend::PortableInt8x2))
        .unwrap();
    assert!(matches!(
        runtime.set_prefill_backend(Some(MatrixBackend::PortableInt8)),
        Err(Error::Parameter(_))
    ));
    assert_eq!(runtime.matrix_backend(), "portable-int8x2");
    runtime
        .set_prefill_backend(Some(MatrixBackend::Portable))
        .unwrap();
    runtime.set_prefill_backend(None).unwrap();
}
