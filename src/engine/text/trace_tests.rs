use super::tests::{close_rows, snapshot};
use super::*;
use crate::engine::attention::tests::fixture::Fixture;

use crate::engine::execution::{MatrixBackend, MatrixKernels};

#[test]
fn batched_runtime_matches_rowwise_across_chunks_self_conditioning_and_generation() {
    check_batch(false, MatrixBackend::Portable);
}
#[test]
fn parallel_runtime_matches_rowwise_across_chunks_self_conditioning_and_generation() {
    check_batch(true, MatrixBackend::Portable);
}
/// POWER9 explicit-FMA kernels: bit-identical to rowwise (and hence to the
/// portable batched path) through every exercised projection.
#[test]
fn fma_batched_runtime_matches_rowwise_bit_for_bit() {
    if native_fma() {
        check_batch(false, MatrixBackend::Power9Fma);
    }
}
#[test]
fn fma_parallel_runtime_matches_rowwise_bit_for_bit() {
    if native_fma() {
        check_batch(true, MatrixBackend::Power9Fma);
    }
}
/// True when the native FMA kernels are compiled in and detected; otherwise
/// checks that forcing them is rejected (Auto then falls back to portable).
fn native_fma() -> bool {
    let available = MatrixKernels::new(MatrixBackend::Power9Fma).is_ok();
    if !available {
        let fixture = Fixture::new();
        let file = ModelFile::open(&fixture.path).unwrap();
        let model = ModelDefinition::bind(file.inventory()).unwrap();
        assert!(matches!(
            ReferenceText::new_with_backend(
                &file,
                &model,
                ReferenceLimits::default(),
                1,
                MatrixBackend::Power9Fma
            ),
            Err(Error::Execution(
                crate::engine::execution::Error::MatrixBackendUnavailable
            ))
        ));
        eprintln!(
            "POWER9 FMA kernels unavailable in this build/CPU: native engine comparisons skipped"
        );
    }
    available
}
#[test]
fn matrix_backend_is_resolved_once_and_reported() {
    let fixture = Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let limits = ReferenceLimits::default;
    let native = MatrixKernels::new(MatrixBackend::Power9Fma).is_ok();
    for (backend, expected) in [
        (MatrixBackend::Portable, "portable"),
        (
            MatrixBackend::Auto,
            if native { "power9-fma" } else { "portable" },
        ),
    ] {
        let runtime = ReferenceText::new_with_backend(&file, &model, limits(), 1, backend).unwrap();
        assert_eq!(runtime.matrix_backend(), expected);
    }
    assert_eq!(
        ReferenceText::new_batched(&file, &model, limits())
            .unwrap()
            .matrix_backend(),
        "portable"
    );
    let workers = crate::engine::execution::Workers::available()
        .unwrap()
        .len()
        .min(5);
    assert_eq!(
        ReferenceText::new_parallel(&file, &model, limits(), workers)
            .unwrap()
            .matrix_backend(),
        "portable"
    );
    for count in [
        0,
        crate::engine::execution::Workers::available()
            .unwrap()
            .len()
            + 1,
    ] {
        assert!(
            ReferenceText::new_with_backend(&file, &model, limits(), count, MatrixBackend::Auto)
                .is_err()
        );
    }
}
fn bits(rows: &[Vec<f32>]) -> Vec<u32> {
    rows.iter().flatten().map(|x| x.to_bits()).collect()
}
fn check_batch(parallel: bool, backend: MatrixBackend) {
    use crate::engine::generation::{GenerationConfig, GenerationSession};
    let fixture = Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let scalar = ReferenceText::new(&file, &model, ReferenceLimits::default()).unwrap();
    let workers = if parallel {
        crate::engine::execution::Workers::available()
            .unwrap()
            .len()
            .min(5)
    } else {
        1
    };
    let batch = match (backend, parallel) {
        (MatrixBackend::Portable, true) => {
            ReferenceText::new_parallel(&file, &model, ReferenceLimits::default(), workers).unwrap()
        }
        (MatrixBackend::Portable, false) => {
            ReferenceText::new_batched(&file, &model, ReferenceLimits::default()).unwrap()
        }
        _ => ReferenceText::new_with_backend(
            &file,
            &model,
            ReferenceLimits::default(),
            workers,
            backend,
        )
        .unwrap(),
    };
    let expected = if backend == MatrixBackend::Portable {
        "portable"
    } else {
        "power9-fma"
    };
    assert_eq!(batch.matrix_backend(), expected);
    let mut a = scalar.new_cache().unwrap();
    let mut b = batch.new_cache().unwrap();
    for prompt in [&[2, 5][..], &[1, 4, 6][..]] {
        assert_eq!(
            bits(&scalar.encode(prompt, &mut a).unwrap()),
            bits(&batch.encode(prompt, &mut b).unwrap())
        );
        assert_eq!(snapshot(&a), snapshot(&b));
    }
    let previous = vec![0.1; 16];
    for logits in [None, Some(previous.as_slice())] {
        assert_eq!(
            bits(&scalar.decode_hidden(&[1, 4], &a, logits).unwrap()),
            bits(&batch.decode_hidden(&[1, 4], &b, logits).unwrap())
        );
        assert_eq!(
            scalar
                .decode_logits(&[1, 4], &a, logits)
                .unwrap()
                .iter()
                .map(|x| x.to_bits())
                .collect::<Vec<_>>(),
            batch
                .decode_logits(&[1, 4], &b, logits)
                .unwrap()
                .iter()
                .map(|x| x.to_bits())
                .collect::<Vec<_>>(),
        );
        assert_eq!(snapshot(&a), snapshot(&b));
    }
    // Router decisions (IDs, weights, probabilities) are bit-identical too.
    let routing = |runtime: &ReferenceText<'_>, cache: &EncoderCache<'_>| {
        let mut rows = Vec::new();
        runtime
            .decode_hidden_traced(&[1, 4], cache, Some(previous.as_slice()), &mut |p, r| {
                if p.is_routing() {
                    rows.push(bits(r));
                }
                Ok::<_, std::convert::Infallible>(())
            })
            .unwrap();
        rows
    };
    assert_eq!(routing(&scalar, &a), routing(&batch, &b));
    let config = GenerationConfig {
        max_new_tokens: 6,
        eos_ids: Vec::new(),
        encoder_chunk_tokens: 2,
        denoising: crate::engine::diffusion::DenoisingConfig {
            max_steps: 2,
            ..Default::default()
        },
        ..Default::default()
    };
    let mut rowwise = GenerationSession::new(&scalar, &[2, 5], config.clone()).unwrap();
    let mut batched = GenerationSession::new(&batch, &[2, 5], config).unwrap();
    while !rowwise.is_finished() {
        rowwise.advance().unwrap();
    }
    while !batched.is_finished() {
        batched.advance().unwrap();
    }
    assert_eq!(
        rowwise.result().unwrap().tokens,
        batched.result().unwrap().tokens
    );
    assert_eq!(
        rowwise.result().unwrap().usage,
        batched.result().unwrap().usage
    );
    let before = snapshot(&b);
    assert!(
        batch
            .encode_traced(&[2], &mut b, &mut |_, _| Err(std::io::Error::other("sink")))
            .is_err()
    );
    assert_eq!(snapshot(&b), before);
}

#[test]
fn streamed_self_conditioning_rejects_invalid_weights_even_for_zero_signal() {
    use std::os::unix::fs::FileExt;
    let mut backends = vec![MatrixBackend::Portable];
    if MatrixKernels::new(MatrixBackend::Power9Fma).is_ok() {
        backends.push(MatrixBackend::Power9Fma);
    }
    for backend in backends {
        for name in [
            "self_cond_gate.weight",
            "self_cond_up.weight",
            "self_cond_down.weight",
        ] {
            let fixture = Fixture::new();
            let inventory = crate::engine::gguf::inspect(&fixture.path).unwrap();
            let offset = inventory
                .tensors
                .iter()
                .find(|t| t.name == name)
                .unwrap()
                .file_offset;
            // Corrupt only this temporary fixture BEFORE opening the immutable
            // ModelFile. Never mutate the actual model or an active runtime file.
            std::fs::OpenOptions::new()
                .write(true)
                .open(&fixture.path)
                .unwrap()
                .write_all_at(&f32::NAN.to_le_bytes(), offset)
                .unwrap();
            let file = ModelFile::open(&fixture.path).unwrap();
            let model = ModelDefinition::bind(file.inventory()).unwrap();
            let batch = ReferenceText::new_with_backend(
                &file,
                &model,
                ReferenceLimits::default(),
                crate::engine::execution::Workers::available()
                    .unwrap()
                    .len()
                    .min(5),
                backend,
            )
            .unwrap();
            let mut cache = batch.new_cache().unwrap();
            batch.encode(&[2, 5], &mut cache).unwrap();
            let before = snapshot(&cache);
            assert!(matches!(
                batch.decode_hidden(&[1, 4], &cache, None),
                Err(Error::Quantized(
                    crate::engine::kernels::quantized::Error::NonFiniteWeight
                ))
            ));
            assert_eq!(snapshot(&cache), before);
        }
    }
}

/// Routing rows: `[top-k IDs, top-k weights, all probabilities]` per position,
/// IDs exact integers in descending-probability order (ties toward lower ID).
fn check_routing_rows(
    model: &ModelDefinition<'_>,
    snapshots: &[(TracePoint, Vec<Vec<f32>>)],
    positions: usize,
) {
    let k = model.spec.top_k;
    let experts = model.layers[0]
        .moe
        .as_ref()
        .unwrap()
        .router
        .projection
        .shape[1] as usize;
    let routing: Vec<_> = snapshots.iter().filter(|(p, _)| p.is_routing()).collect();
    assert_eq!(routing.len(), model.layers.len());
    for (_, rows) in routing {
        assert_eq!(rows.len(), positions);
        for row in rows {
            assert_eq!(row.len(), 2 * k + experts);
            let ids: Vec<usize> = row[..k]
                .iter()
                .map(|&x| {
                    assert_eq!(x.fract(), 0.0);
                    x as usize
                })
                .collect();
            let probs = &row[2 * k..];
            assert!((probs.iter().map(|&p| f64::from(p)).sum::<f64>() - 1.0).abs() < 1e-5);
            for pair in ids.windows(2) {
                assert!(
                    probs[pair[0]] > probs[pair[1]]
                        || (probs[pair[0]] == probs[pair[1]] && pair[0] < pair[1])
                );
            }
            let floor = probs[ids[k - 1]];
            assert!(
                (0..experts)
                    .filter(|e| !ids.contains(e))
                    .all(|e| probs[e] <= floor)
            );
            assert!(row[k..2 * k].iter().all(|w| w.is_finite()));
        }
    }
}

#[test]
fn observers_preserve_outputs_and_fail_before_encoder_commit() {
    let fixture = Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let runtime = ReferenceText::new(&file, &model, ReferenceLimits::default()).unwrap();
    let mut cache = runtime.new_cache().unwrap();
    let initial = snapshot(&cache);
    let fail = runtime.encode_traced(&[2, 5], &mut cache, &mut |point, _| {
        if point == TracePoint::EncoderFinal {
            Err(std::io::Error::other("injected sink failure"))
        } else {
            Ok(())
        }
    });
    assert!(matches!(fail, Err(TraceError::Observer(_))));
    assert_eq!(snapshot(&cache), initial);
    // A failing routing observer also aborts before the all-layer cache commit.
    let fail = runtime.encode_traced(&[2, 5], &mut cache, &mut |point, _| {
        if point == TracePoint::EncoderRouting(1) {
            Err(std::io::Error::other("routing sink failure"))
        } else {
            Ok(())
        }
    });
    assert!(matches!(fail, Err(TraceError::Observer(_))));
    assert_eq!(snapshot(&cache), initial);
    let mut snapshots = Vec::new();
    let encoded = runtime
        .encode_traced(&[2, 5], &mut cache, &mut |point, rows| {
            snapshots.push((point, rows.to_vec()));
            Ok::<_, std::convert::Infallible>(())
        })
        .unwrap();
    assert_eq!(
        snapshots.iter().map(|(p, _)| *p).collect::<Vec<_>>(),
        [
            TracePoint::EncoderEmbedding,
            TracePoint::EncoderLayer(0),
            TracePoint::EncoderRouting(0),
            TracePoint::EncoderLayer(1),
            TracePoint::EncoderRouting(1),
            TracePoint::EncoderFinal
        ]
    );
    check_routing_rows(&model, &snapshots, 2);
    close_rows(
        &encoded,
        &fixture.expected_text(&model, &[2, 5], &[1, 4], None).0,
    );
    assert_eq!(encoded, snapshots.last().unwrap().1);
    let before = snapshot(&cache);
    snapshots.clear();
    let hidden = runtime
        .decode_hidden_traced(&[1, 4], &cache, None, &mut |point, rows| {
            snapshots.push((point, rows.to_vec()));
            Ok::<_, std::convert::Infallible>(())
        })
        .unwrap();
    assert_eq!(
        snapshots.iter().map(|(p, _)| *p).collect::<Vec<_>>(),
        [
            TracePoint::DecoderEmbedding,
            TracePoint::DecoderInput,
            TracePoint::DecoderLayer(0),
            TracePoint::DecoderRouting(0),
            TracePoint::DecoderLayer(1),
            TracePoint::DecoderRouting(1),
            TracePoint::DecoderFinal
        ]
    );
    check_routing_rows(&model, &snapshots, 2);
    assert_eq!(
        hidden,
        runtime.decode_hidden(&[1, 4], &cache, None).unwrap()
    );
    for failure_point in [
        TracePoint::DecoderEmbedding,
        TracePoint::DecoderInput,
        TracePoint::DecoderLayer(1),
        TracePoint::DecoderRouting(1),
        TracePoint::DecoderFinal,
    ] {
        assert!(
            runtime
                .decode_hidden_traced(&[1, 4], &cache, None, &mut |p, _| {
                    if p == failure_point {
                        Err(std::io::Error::other("sink"))
                    } else {
                        Ok(())
                    }
                })
                .is_err()
        );
        assert_eq!(snapshot(&cache), before);
    }
    let previous = vec![0.0; 16];
    snapshots.clear();
    runtime
        .decode_hidden_traced(&[1, 4], &cache, Some(&previous), &mut |point, rows| {
            snapshots.push((point, rows.to_vec()));
            Ok::<_, std::convert::Infallible>(())
        })
        .unwrap();
    assert_eq!(snapshots[0].0, TracePoint::DecoderSoftEmbedding);
    close_rows(&snapshots[0].1, &fixture.expected_soft(&previous));
    assert_eq!(snapshot(&cache), before);
}

/// Prompt chunks split between two worker sets (the second on weight
/// replica 1) give bit-identical hidden states and caches, for split and
/// unsplit chunk sizes alike.
#[test]
fn split_sockets_match_one_socket_bit_for_bit() {
    use crate::engine::execution::Workers;
    let fixture = Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let cpus = Workers::available().unwrap();
    let n = cpus.len().min(2);
    let one = ReferenceText::new_with_backend(
        &file,
        &model,
        ReferenceLimits::default(),
        n,
        MatrixBackend::Auto,
    )
    .unwrap();
    let split = ReferenceText::split_with(
        &file,
        &model,
        ReferenceLimits::default(),
        Workers::on(cpus[..n].to_vec(), 0).unwrap(),
        Workers::on(cpus[cpus.len() - n..].to_vec(), 1).unwrap(),
        MatrixBackend::Auto,
    )
    .unwrap();
    let tokens: Vec<u32> = (0..110).map(|i| (i * 5 % 7 + 1) as u32).collect();
    let (mut a, mut b) = (one.new_cache().unwrap(), split.new_cache().unwrap());
    // 70 positions split 35/35, then 30 (too few to split), then 10.
    for chunk in [&tokens[..70], &tokens[70..100], &tokens[100..]] {
        let x = one.encode(chunk, &mut a).unwrap();
        let y = split.encode(chunk, &mut b).unwrap();
        assert_eq!(bits(&x), bits(&y));
    }
    for (x, y) in a.layers.iter().zip(&b.layers) {
        assert_eq!(x.keys().len(), y.keys().len());
        assert!(
            x.keys()
                .iter()
                .zip(y.keys())
                .all(|(p, q)| p.to_bits() == q.to_bits())
        );
        assert!(
            x.values()
                .iter()
                .zip(y.values())
                .all(|(p, q)| p.to_bits() == q.to_bits())
        );
    }
    assert!(split.split_share().is_some());
}
