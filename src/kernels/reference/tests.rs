use super::*;

#[test]
fn direct_rms_scale_and_unscaled_normalization() {
    let y = rms_norm(&[3.0, 4.0], None, 0.5).unwrap();
    assert!((y[0] - 3.0 / 13.0_f32.sqrt()).abs() < 1e-6);
    let scaled = rms_norm(&[3.0, 4.0], Some(&[2.0, 0.0]), 0.5).unwrap();
    assert_eq!(scaled[0], 2.0 * y[0]);
    assert_eq!(scaled[1], 0.0); // not (1+scale)
    assert_eq!(rms_norm(&[0.0; 2], None, 1e-6).unwrap(), [0.0; 2]);
}

#[test]
fn matrix_orientation_is_output_rows_by_input_columns() {
    let m = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, -1.0, 0.0, 2.0]).unwrap();
    assert_eq!(m.project(&[1.0, 2.0, 4.0]).unwrap(), [17.0, 7.0]);
    assert!(m.project(&[1.0]).is_err());
    assert!(Matrix::new(2, 3, vec![0.0; 5]).is_err());
}

#[test]
fn transposed_projection_accumulates_across_tiles_without_f32_rounding() {
    let matrix = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, -1.0, 0.0, 2.0]).unwrap();
    let mut out = vec![0.25; 3];
    matrix.accumulate_transposed(&[2.0, 3.0], &mut out).unwrap();
    assert_eq!(out, [-0.75, 4.25, 12.25]);
    assert!(matrix.accumulate_transposed(&[1.0], &mut out).is_err());
    assert!(
        matrix
            .accumulate_transposed(&[1.0; 2], &mut [0.0; 2])
            .is_err()
    );
    assert!(
        matrix
            .accumulate_transposed(&[f32::NAN; 2], &mut out)
            .is_err()
    );
    assert!(
        matrix
            .accumulate_transposed(&[1.0; 2], &mut [f64::NAN; 3])
            .is_err()
    );
    let mut accumulator = [0.0_f64];
    for value in [16777216.0, 1.0, -16777216.0] {
        Matrix::new(1, 1, vec![value])
            .unwrap()
            .accumulate_transposed(&[1.0], &mut accumulator)
            .unwrap();
    }
    assert_eq!(accumulator, [1.0]); // tile-wise F32 summation would lose the 1
}

#[test]
fn gelu_matches_independent_tanh_formula() {
    let g = [-3.0f32, -0.25, 0.0, 0.5, 3.0];
    let up = [2.0; 5];
    for (&x, y) in g.iter().zip(gelu_product(&g, &up).unwrap()) {
        let x = f64::from(x);
        let expected = x
            * (1.0
                + crate::libm::tanh(
                    (2.0 / std::f64::consts::PI).sqrt() * (x + 0.044715 * x.powi(3)),
                ));
        assert!((f64::from(y) - expected).abs() < 1e-6);
    }
}

#[test]
fn stable_softmax_softcap_and_masking() {
    assert_eq!(softmax(&[10000.0, 10000.0]).unwrap(), [0.5, 0.5]);
    assert_eq!(softmax(&[f32::NEG_INFINITY, 1.0]).unwrap(), [0.0, 1.0]);
    let capped = softcap(&[-1e30, 0.0, 1e30], 30.0).unwrap();
    assert_eq!(capped, [-30.0, 0.0, 30.0]);
    assert_eq!(multiply_scaled(&[2.0], &[3.0], 0.5).unwrap(), [3.0]);
    assert_eq!(add(&[2.0], &[3.0]).unwrap(), [5.0]);
}

#[test]
fn rope_pairs_split_halves_and_uses_frequency_divisors() {
    let x = [1.0_f32, 2.0, 3.0, 4.0, -1.0, -2.0, -3.0, -4.0];
    assert_eq!(rope(&x, 4, 0, 10000.0, None).unwrap(), x);
    let result = rope(&x, 4, 3, 10000.0, Some(&[2.0, 1e30])).unwrap();
    for (raw, output) in x.as_chunks::<4>().0.iter().zip(result.as_chunks::<4>().0) {
        let (s, c) = 1.5_f64.sin_cos();
        assert!(
            (f64::from(output[0]) - (f64::from(raw[0]) * c - f64::from(raw[2]) * s)).abs() < 1e-6
        );
        assert!(
            (f64::from(output[2]) - (f64::from(raw[2]) * c + f64::from(raw[0]) * s)).abs() < 1e-6
        );
        assert_eq!(output[1], raw[1]);
        assert_eq!(output[3], raw[3]);
    }
}

#[test]
fn proportional_rope_matches_reference_pair_layout_at_large_position() {
    let x: Vec<_> = (0..512).map(|i| (i % 13) as f32 - 6.0).collect();
    let factors: Vec<_> = (0..256).map(|i| if i < 64 { 1.0 } else { 1e30 }).collect();
    let y = rope(&x, 512, 262143, 1e6, Some(&factors)).unwrap();
    for i in 0..256 {
        // Reference proportional RoPE uses F32 inverse frequencies and ZERO
        // frequencies after pair 63, pairing i with i+256 (not i+64).
        let angle = if i < 64 {
            262143.0_f32 * (1.0 / crate::libm::powf(1e6, 2.0 * i as f32 / 512.0))
        } else {
            0.0
        };
        let (s, c) = crate::libm::sincosf(angle);
        assert!((y[i] - (x[i] * c - x[i + 256] * s)).abs() < 2e-6);
        assert!((y[i + 256] - (x[i + 256] * c + x[i] * s)).abs() < 2e-6);
    }
}

#[test]
fn grouped_attention_is_token_major_contiguous_heads_and_scale_one() {
    let query = [1.0_f32, 0.0, 2.0, 0.0, 0.0, 1.0, 0.0, 2.0];
    let keys = [1.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, -1.0];
    let values = [10.0, 11.0, 20.0, 21.0, 30.0, 31.0, 40.0, 41.0];
    let y = attention(&query, &keys, &values, 2, 2, None).unwrap();
    for h in 0..4 {
        let score: f64 = if h % 2 == 0 { 1.0 } else { 2.0 };
        let p = crate::libm::exp(score) / (crate::libm::exp(score) + crate::libm::exp(-score));
        let kh = h / 2;
        for d in 0..2 {
            let expected =
                p * f64::from(values[kh * 2 + d]) + (1.0 - p) * f64::from(values[4 + kh * 2 + d]);
            assert!((f64::from(y[h * 2 + d]) - expected).abs() < 3e-6);
        }
    }
    assert_eq!(
        attention(&query, &keys, &values, 2, 2, Some(&[false, true])).unwrap(),
        [30.0, 31.0, 30.0, 31.0, 40.0, 41.0, 40.0, 41.0]
    );
}

#[test]
fn attention_matches_independent_nontrivial_masked_equations() {
    let query: Vec<_> = (0..24).map(|i| (i % 11) as f32 / 7.0 - 0.5).collect();
    let keys: Vec<_> = (0..60).map(|i| (i % 17) as f32 / 13.0 - 0.5).collect();
    let values: Vec<_> = (0..60).map(|i| (i % 19) as f32 / 3.0 - 2.0).collect();
    let mask = [true, false, true, true, false];
    let actual = attention(&query, &keys, &values, 2, 6, Some(&mask)).unwrap();
    for h in 0..4 {
        let kh = h / 2;
        let scores: Vec<_> = (0..5)
            .map(|t| {
                if !mask[t] {
                    return f32::NEG_INFINITY;
                }
                (0..6)
                    .map(|d| f64::from(query[h * 6 + d]) * f64::from(keys[t * 12 + kh * 6 + d]))
                    .sum::<f64>() as f32
            })
            .collect();
        let max = scores.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let exps: Vec<_> = scores
            .iter()
            .map(|&s| crate::libm::exp(f64::from(s) - f64::from(max)))
            .collect();
        let sum: f64 = exps.iter().sum();
        let probs: Vec<_> = exps.iter().map(|p| (p / sum) as f32).collect();
        for d in 0..6 {
            let expected = (0..5)
                .map(|t| f64::from(probs[t]) * f64::from(values[t * 12 + kh * 6 + d]))
                .sum::<f64>() as f32;
            assert!((actual[h * 6 + d] - expected).abs() < 1e-6);
        }
    }
}

#[test]
fn attention_and_rope_reject_invalid_shapes_masks_and_overflow() {
    assert!(rope(&[], 2, 0, 1e4, None).is_err());
    assert!(rope(&[1.0; 2], 0, 0, 1e4, None).is_err());
    assert!(rope(&[1.0; 3], 3, 0, 1e4, None).is_err());
    assert!(rope(&[1.0; 2], 2, 0, 0.0, None).is_err());
    assert!(rope(&[1.0; 2], 2, 0, 1e4, Some(&[0.0])).is_err());
    assert!(rope(&[1.0; 2], 2, 0, 1e4, Some(&[])).is_err());
    assert!(attention(&[1.0; 2], &[1.0; 2], &[1.0; 2], 0, 2, None).is_err());
    assert!(attention(&[1.0; 2], &[1.0; 2], &[1.0; 2], 1, 0, None).is_err());
    assert!(attention(&[1.0; 2], &[1.0; 2], &[1.0; 2], 1, 2, Some(&[false])).is_err());
    assert!(attention(&[1.0; 2], &[1.0; 2], &[1.0; 2], 1, 2, Some(&[])).is_err());
    assert!(attention(&[1.0; 3], &[1.0; 4], &[1.0; 4], 2, 1, None).is_err());
    assert!(attention(&[1.0; 2], &[1.0; 2], &[1.0; 1], 1, 2, None).is_err());
    assert!(attention(&[f32::MAX; 2], &[f32::MAX; 2], &[1.0; 2], 1, 2, None).is_err());
}

#[test]
fn invalid_and_overflowing_inputs_return_errors() {
    assert!(rms_norm(&[], None, 1e-6).is_err());
    assert!(rms_norm(&[1.0], None, 0.0).is_err());
    assert!(rms_norm(&[1.0], Some(&[]), 1e-6).is_err());
    assert!(softmax(&[f32::NEG_INFINITY; 2]).is_err());
    assert!(softmax(&[f32::INFINITY]).is_err());
    assert!(softcap(&[1.0], f32::NAN).is_err());
    assert!(add(&[f32::MAX], &[f32::MAX]).is_err());
    assert!(Matrix::new(1, 1, vec![f32::NAN]).is_err());
}

/// The row-blocked matvec equals one sequential F64 sum per row, bit for bit.
#[test]
fn matvec_matches_sequential_f64_rows() {
    let mut state = 0x243f6a8885a308d3u64;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        ((state >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0) as f32 * 3.0
    };
    for (rows, cols) in [(1, 1), (7, 5), (8, 3), (9, 64), (17, 300), (128, 2816)] {
        let data: Vec<f32> = (0..rows * cols).map(|_| next()).collect();
        let x: Vec<f32> = (0..cols).map(|_| next()).collect();
        let m = Matrix::new(rows, cols, data.clone()).unwrap();
        let got = m.project(&x).unwrap();
        for r in 0..rows {
            let mut sum = 0.0f64;
            for c in 0..cols {
                sum += f64::from(data[r * cols + c]) * f64::from(x[c]);
            }
            assert_eq!(
                got[r].to_bits(),
                (sum as f32).to_bits(),
                "{rows}x{cols} row {r}"
            );
        }
    }
}

#[test]
fn project_many_matches_project_bitwise_wide_range() {
    // Values over many binades (sums that round at every step), 1 to 9
    // inputs, rows not a multiple of the vector tile.
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let mantissa = (seed % 2_000_001) as f32 / 1_000_000.0 - 1.0;
        mantissa * 2f32.powi((seed >> 40) as i32 % 24 - 12)
    };
    let (rows, cols) = (21, 300);
    let m = Matrix::new(rows, cols, (0..rows * cols).map(|_| next()).collect()).unwrap();
    for inputs in 1..=17 {
        let xs: Vec<Vec<f32>> = (0..inputs)
            .map(|_| (0..cols).map(|_| next()).collect())
            .collect();
        let refs: Vec<&[f32]> = xs.iter().map(Vec::as_slice).collect();
        let many = m.project_many(&refs).unwrap();
        for (x, got) in xs.iter().zip(&many) {
            let want = m.project(x).unwrap();
            assert_eq!(
                want.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                got.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
            );
        }
    }
}

#[test]
fn project_many_matches_project_bitwise() {
    // 4-input blocks, a remainder of inputs and of rows (rows % 4 != 0).
    let (rows, cols) = (11, 37);
    let data: Vec<f32> = (0..rows * cols)
        .map(|i| ((i * 7919) % 2003) as f32 / 977.0 - 1.0)
        .collect();
    let m = Matrix::new(rows, cols, data).unwrap();
    let xs: Vec<Vec<f32>> = (0..7)
        .map(|p| {
            (0..cols)
                .map(|i| ((i * 31 + p * 17) % 101) as f32 / 53.0 - 0.9)
                .collect()
        })
        .collect();
    let refs: Vec<&[f32]> = xs.iter().map(Vec::as_slice).collect();
    let many = m.project_many(&refs).unwrap();
    for (x, got) in xs.iter().zip(&many) {
        let want = m.project(x).unwrap();
        assert_eq!(
            want.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            got.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
    }
}

/// Exhaustive over every finite f32 gate against the scalar formula; run by
/// hand: `cargo test --release gelu_exhaustive -- --ignored`.
#[test]
#[ignore]
fn gelu_exhaustive() {
    let threads = 8u64;
    std::thread::scope(|sc| {
        for th in 0..threads {
            sc.spawn(move || {
                let total = 1u64 << 32;
                let (lo, hi) = (total / threads * th, total / threads * (th + 1));
                let mut b = lo;
                while b < hi {
                    let m = ((hi - b) as usize).min(1000);
                    let gate: Vec<f32> = (0..m)
                        .map(|j| f32::from_bits((b + j as u64) as u32))
                        .map(|g| if g.is_finite() { g } else { 0.0 })
                        .collect();
                    let up: Vec<f32> = (0..m)
                        .map(|j| 0.5 + 0.001 * (j as f32) - (b >> 20) as f32 * 1e-4)
                        .collect();
                    let got = gelu_product(&gate, &up);
                    // Non-finite products are reported as errors; compare the rest.
                    let Ok(got) = got else {
                        b += m as u64;
                        continue;
                    };
                    for j in 0..m {
                        let g = f64::from(gate[j]);
                        let want = (0.5
                            * g
                            * (1.0
                                + crate::libm::tanh(
                                    0.7978845608028654 * (g + 0.044715 * g * g * g),
                                ))) as f32
                            * up[j];
                        assert_eq!(
                            got[j].to_bits(),
                            want.to_bits(),
                            "gate {:#x}",
                            gate[j].to_bits()
                        );
                    }
                    b += m as u64;
                }
            });
        }
    });
}

/// ns per element, new batched kernel vs the scalar formula; run by hand.
#[test]
#[ignore]
fn gelu_bench() {
    let n = 4096;
    let mut s = 0x9e3779b97f4a7c15u64;
    let mut rnd = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 11) as f32 / (1u64 << 53) as f32
    };
    let gate: Vec<f32> = (0..n)
        .map(|_| (rnd() + rnd() + rnd() + rnd() - 2.0) * 4.0)
        .collect();
    let up: Vec<f32> = (0..n).map(|_| rnd() - 0.5).collect();
    let reps = 2000;
    let t = std::time::Instant::now();
    let mut sink = 0u32;
    for _ in 0..reps {
        sink ^= gelu_product(&gate, &up).unwrap()[17].to_bits();
    }
    let new = t.elapsed().as_secs_f64() / (reps * n) as f64 * 1e9;
    let t = std::time::Instant::now();
    for _ in 0..reps {
        let v: Vec<f32> = (0..n)
            .map(|i| {
                let g = f64::from(gate[i]);
                (0.5 * g
                    * (1.0 + crate::libm::tanh(0.7978845608028654 * (g + 0.044715 * g * g * g))))
                    as f32
                    * up[i]
            })
            .collect();
        sink ^= v[17].to_bits();
    }
    let old = t.elapsed().as_secs_f64() / (reps * n) as f64 * 1e9;
    println!("gelu ns/elem: new {new:.2} old {old:.2} ({sink})");
}
