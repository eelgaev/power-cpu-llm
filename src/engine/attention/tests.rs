use super::*;
pub(crate) mod fixture;

#[test]
fn complete_local_and_global_blocks_match_independent_tiny_gguf_oracle() {
    let fixture = fixture::Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let input = rows(model.spec.hidden, 6);
    for layer in 0..2 {
        let block = ReferenceBlock::new(&file, &model, layer).unwrap();
        let mut cache = block.new_cache(4096).unwrap();
        let encoder = block.encode(&input[..4], &mut cache).unwrap();
        let decoder = block.decode(&input[4..], &cache).unwrap();
        let expected = fixture.expected(&model, layer, &input, 4);
        close(&encoder, &expected[..4]);
        close(&decoder, &expected[4..]);
        assert_eq!(cache.total_tokens(), 4);
        assert_eq!(cache.retained_tokens(), if layer == 0 { 2 } else { 4 });
        let mut chunked_cache = block.new_cache(4096).unwrap();
        let mut chunked = block.encode(&input[..2], &mut chunked_cache).unwrap();
        chunked.extend(block.encode(&input[2..4], &mut chunked_cache).unwrap());
        close(&chunked, &encoder);
        close(
            &block.decode(&input[4..], &chunked_cache).unwrap(),
            &decoder,
        );
    }
}

#[test]
fn cache_owner_layer_limits_and_failures_checked_without_real_model() {
    let fixture = fixture::Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let other_file = ModelFile::open(&fixture.path).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let other_model = ModelDefinition::bind(other_file.inventory()).unwrap();
    let block = ReferenceBlock::new(&file, &model, 0).unwrap();
    let other = ReferenceBlock::new(&other_file, &other_model, 0).unwrap();
    let mut cache = other.new_cache(4096).unwrap();
    assert!(block.encode(&rows(4, 1), &mut cache).is_err());
    let mut cache = block.new_cache(16).unwrap(); // exactly one local K/V token
    assert!(matches!(
        block.encode(&rows(4, 2), &mut cache),
        Err(Error::ResourceLimit)
    ));
    assert_eq!(cache.total_tokens(), 0);
    block.encode(&rows(4, 1), &mut cache).unwrap();
    let snapshot = (cache.total, cache.keys.clone(), cache.values.clone());
    assert!(block.encode(&[vec![f32::NAN; 4]], &mut cache).is_err());
    assert!(block.decode(&rows(4, 5), &cache).is_err()); // exceeds canvas 4
    assert!(block.encode(&[], &mut cache).is_err());
    assert!(block.encode(&[vec![1.0; 3]], &mut cache).is_err());
    assert_eq!(
        snapshot,
        (cache.total, cache.keys.clone(), cache.values.clone())
    );
    assert!(ReferenceBlock::new(&file, &model, 2).is_err());
}

fn geometry(kind: AttentionKind) -> AttentionGeometry {
    AttentionGeometry {
        kind,
        query_heads: 2,
        kv_heads: 1,
        head_dim: 2,
        rotary_dim: 2,
    }
}

#[test]
fn global_value_is_normalized_raw_key_not_scaled_rotated_key() {
    let transform = HeadTransform::new(
        geometry(AttentionKind::Global),
        vec![1.0; 2],
        vec![2.0, 0.5],
        0.5,
        1e6,
        Some(vec![1.0]),
    )
    .unwrap();
    let result = transform
        .prepare(&[3.0, 4.0, 0.0, 0.0], &[3.0, 4.0], None, 0)
        .unwrap();
    let inv = 1.0 / 13.0_f32.sqrt();
    assert!((result.key[0] - 6.0 * inv).abs() < 1e-6);
    assert!((result.key[1] - 2.0 * inv).abs() < 1e-6);
    assert!((result.value[0] - 3.0 * inv).abs() < 1e-6);
    assert!((result.value[1] - 4.0 * inv).abs() < 1e-6);
    assert_eq!(&result.query[2..], &[0.0, 0.0]);
    let rotated = transform
        .prepare(&[3.0, 4.0, 0.0, 0.0], &[3.0, 4.0], None, 1)
        .unwrap();
    assert_eq!(rotated.value, result.value);
    assert_ne!(rotated.key, result.key);
    // A global layer WITH its own V (Gemma 4 E2B) normalizes that V instead.
    let own_v = transform
        .prepare(&[3.0, 4.0, 0.0, 0.0], &[3.0, 4.0], Some(&[0.0, 2.0]), 0)
        .unwrap();
    assert_eq!(own_v.key, result.key);
    // rms_norm with this test's epsilon 0.5: [0, 2] / sqrt(mean(0, 4) + 0.5).
    assert!(own_v.value[0].abs() < 1e-6 && (own_v.value[1] - 2.0 / 2.5_f32.sqrt()).abs() < 1e-6);
}

#[test]
fn local_value_uses_its_separate_projection_and_head_norms_broadcast() {
    let transform = HeadTransform::new(
        geometry(AttentionKind::Sliding),
        vec![2.0, 0.5],
        vec![1.0; 2],
        0.5,
        1e4,
        None,
    )
    .unwrap();
    let result = transform
        .prepare(&[3.0, 4.0, 6.0, 8.0], &[3.0, 4.0], Some(&[0.0, 2.0]), 0)
        .unwrap();
    assert!((result.query[0] - 6.0 / 13.0_f32.sqrt()).abs() < 1e-6);
    assert!((result.query[2] - 12.0 / 50.5_f32.sqrt()).abs() < 1e-6);
    assert!((result.value[1] - 2.0 / 2.5_f32.sqrt()).abs() < 1e-6);
    assert!(transform.prepare(&[1.0; 4], &[1.0; 2], None, 0).is_err());
    assert!(
        transform
            .prepare(&[1.0; 2], &[1.0; 2], Some(&[1.0; 2]), 0)
            .is_err()
    );
}

#[test]
fn encoder_window_and_cache_retention_have_distinct_boundaries() {
    assert_eq!(
        encoder_mask(7, 6, 10, None).unwrap(),
        [true, true, true, true, false, false]
    );
    assert_eq!(
        encoder_mask(7, 6, 10, Some(3)).unwrap(),
        [false, true, true, true, false, false]
    );
    assert_eq!(
        encoder_mask(7, 6, 10, Some(1)).unwrap(),
        [false, false, false, true, false, false]
    );
    assert_eq!(retained_count(10, Some(3)), 2); // decoder sees window-1 prompt keys
    assert_eq!(retained_count(10, Some(1)), 0);
    assert_eq!(retained_count(10, None), 10);
    assert!(encoder_mask(usize::MAX, 2, 0, None).is_err());
    assert!(encoder_mask(0, 1, 0, Some(0)).is_err());
}

#[test]
fn invalid_geometry_and_transform_parameters_rejected() {
    let mut g = geometry(AttentionKind::Sliding);
    g.kv_heads = 3;
    assert!(HeadTransform::new(g, vec![1.0; 2], vec![1.0; 2], 1e-6, 1e4, None).is_err());
    let mut g = geometry(AttentionKind::Sliding);
    g.rotary_dim = 0;
    assert!(HeadTransform::new(g, vec![1.0; 2], vec![1.0; 2], 1e-6, 1e4, None).is_err());
    assert!(
        HeadTransform::new(
            geometry(AttentionKind::Global),
            vec![1.0; 2],
            vec![1.0; 2],
            1e-6,
            1e6,
            Some(vec![0.0])
        )
        .is_err()
    );
    assert!(
        HeadTransform::new(
            geometry(AttentionKind::Global),
            vec![1.0; 1],
            vec![1.0; 2],
            1e-6,
            1e6,
            None
        )
        .is_err()
    );
}

fn rows(width: usize, count: usize) -> Vec<Vec<f32>> {
    (0..count)
        .map(|t| {
            (0..width)
                .map(|i| ((i * 3 + t * 7) % 29) as f32 / 19.0 - 0.5)
                .collect()
        })
        .collect()
}
fn close(a: &[Vec<f32>], b: &[Vec<f32>]) {
    assert_eq!(a.len(), b.len());
    for (a, b) in a.iter().zip(b) {
        assert_eq!(a.len(), b.len());
        for (&a, &b) in a.iter().zip(b) {
            assert!((a - b).abs() <= 1e-5 * (1.0 + b.abs()), "{a} != {b}");
        }
    }
}

#[test]
#[ignore = "set POWER_LLM_TEST_MODEL to the completed target GGUF"]
fn actual_diffusiongemma_local_complete_block_chunking_and_readonly_decoder() {
    let file =
        ModelFile::open(std::env::var_os("POWER_LLM_TEST_MODEL").expect("model path")).unwrap();
    let mut model = ModelDefinition::bind(file.inventory()).unwrap();
    // Deliberately reduce the window ONLY for this synthetic-activation test,
    // exercising eviction without an expensive 1024-token reference FFN run.
    model.spec.sliding_window = 3;
    let block = ReferenceBlock::new(&file, &model, 0).unwrap();
    let input = rows(model.spec.hidden, 5);
    let mut whole_cache = block.new_cache(8 * 1024 * 1024).unwrap();
    let whole = block.encode(&input, &mut whole_cache).unwrap();
    assert_eq!(whole_cache.total_tokens(), 5);
    assert_eq!(whole_cache.retained_tokens(), 2);
    let mut chunked_cache = block.new_cache(8 * 1024 * 1024).unwrap();
    let mut chunked = block.encode(&input[..1], &mut chunked_cache).unwrap();
    chunked.extend(block.encode(&input[1..3], &mut chunked_cache).unwrap());
    chunked.extend(block.encode(&input[3..], &mut chunked_cache).unwrap());
    close(&whole, &chunked);
    assert_eq!(whole_cache.keys(), chunked_cache.keys());
    assert_eq!(whole_cache.values(), chunked_cache.values());
    let snapshot = (
        whole_cache.total,
        whole_cache.keys.clone(),
        whole_cache.values.clone(),
    );
    let canvas = rows(model.spec.hidden, 2);
    let decoded = block.decode(&canvas, &whole_cache).unwrap();
    close(&decoded, &block.decode(&canvas, &whole_cache).unwrap());
    assert!(decoded.iter().flatten().all(|x| x.is_finite()));
    assert_eq!(
        snapshot,
        (
            whole_cache.total,
            whole_cache.keys.clone(),
            whole_cache.values.clone()
        )
    );
    // Both decoder positions see the second canvas KV, even though it is a
    // future position for query 0. No causal or canvas-relative sliding mask.
    let stage = block
        .stage(&canvas, &whole_cache, LayerMode::Decoder)
        .unwrap();
    assert_eq!(stage.keys.len() / 2048, 4); // 2 retained prompt + 2 canvas
    let full =
        math::attention(&stage.queries[0], &stage.keys, &stage.values, 8, 256, None).unwrap();
    let causal = math::attention(
        &stage.queries[0],
        &stage.keys,
        &stage.values,
        8,
        256,
        Some(&[true, true, true, false]),
    )
    .unwrap();
    assert!(full.iter().zip(causal).any(|(a, b)| (a - b).abs() > 1e-6));
    let mut invalid = input[..1].to_vec();
    invalid[0][0] = f32::NAN;
    assert!(block.encode(&invalid, &mut whole_cache).is_err());
    assert_eq!(
        snapshot,
        (
            whole_cache.total,
            whole_cache.keys.clone(),
            whole_cache.values.clone()
        )
    );
    let mut limited = block.new_cache(2 * 2048 * size_of::<f32>()).unwrap();
    assert!(matches!(
        block.encode(&input[..2], &mut limited),
        Err(Error::ResourceLimit)
    ));
    assert_eq!(limited.total_tokens(), 0);
    assert_eq!(limited.retained_tokens(), 0);
    println!(
        "Actual local complete block: causal chunking/eviction agree, decoder bidirectional/cache unchanged, allocation failure transactional"
    );
}

#[test]
#[ignore = "set POWER_LLM_TEST_MODEL to the completed target GGUF"]
fn actual_diffusiongemma_global_complete_block_and_streamed_projection() {
    let file =
        ModelFile::open(std::env::var_os("POWER_LLM_TEST_MODEL").expect("model path")).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let input = rows(model.spec.hidden, 2);
    let block = ReferenceBlock::new(&file, &model, 5).unwrap();
    let mut cache = block.new_cache(8 * 1024 * 1024).unwrap();
    let encoded = block.encode(&input, &mut cache).unwrap();
    assert_eq!(cache.retained_tokens(), 2);
    assert_eq!(cache.total_tokens(), 2);
    assert!(encoded.iter().flatten().all(|x| x.is_finite()));
    let snapshot = (cache.total, cache.keys.clone(), cache.values.clone());
    let decoded = block.decode(&input, &cache).unwrap();
    assert!(decoded.iter().flatten().all(|x| x.is_finite()));
    assert_eq!(
        snapshot,
        (cache.total, cache.keys.clone(), cache.values.clone())
    );
    // Global Q exceeds the 64-MiB F32 matrix expansion cap. Streaming handles
    // it and agrees with independent scalar dots on selected stored rows.
    assert!(matches!(
        operators::load_matrix(
            &file,
            model.layers[5].query,
            operators::MAX_REFERENCE_MATRIX_ELEMENTS
        ),
        Err(Error::ResourceLimit)
    ));
    let projected = operators::project_streamed(&file, model.layers[5].query, &input[0]).unwrap();
    for i in [0, 127, 128, 8191] {
        let weights = file
            .read_row(&model.layers[5].query.name, i as u64)
            .unwrap()
            .decode()
            .unwrap();
        let expected = weights
            .iter()
            .zip(&input[0])
            .map(|(&w, &x)| f64::from(w) * f64::from(x))
            .sum::<f64>() as f32;
        assert!((projected[i] - expected).abs() < 1e-6 * (1.0 + expected.abs()));
    }
    let local = ReferenceBlock::new(&file, &model, 0).unwrap();
    assert!(local.decode(&input, &cache).is_err());
    let mut local_cache = local.new_cache(8 * 1024 * 1024).unwrap();
    assert!(block.encode(&input, &mut local_cache).is_err());
    println!(
        "Actual global complete block: K-as-V/proportional RoPE finite, cache read-only, global Q streamed and independently checked"
    );
}
