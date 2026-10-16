use super::super::power9_fma::{FmaKernels, Request};
use super::*;

const QUANTIZED: [Encoding; 8] = [
    Encoding::Q4_0,
    Encoding::Q5_0,
    Encoding::Q5_1,
    Encoding::Q5K,
    Encoding::Q8_0,
    Encoding::Q4K,
    Encoding::Q6K,
    Encoding::TQ2_0,
];

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    /// Finite binary16 with a moderate exponent (no zero/subnormal bias).
    fn half(&mut self) -> [u8; 2] {
        let bits = (self.next() as u16 & 0x8000)
            | ((((self.next() % 8) + 9) as u16) << 10)
            | (self.next() as u16 & 0x3ff);
        bits.to_le_bytes()
    }
    fn unit(&mut self) -> f32 {
        (self.next() % 20001) as f32 / 10000.0 - 1.0
    }
}

fn weights(encoding: Encoding, rows: usize, cols: usize, rng: &mut Rng) -> Vec<u8> {
    let (per, size) = encoding.block_layout();
    let mut out = Vec::new();
    for _ in 0..rows * cols / per {
        let mut block: Vec<u8> = (0..size).map(|_| rng.next() as u8).collect();
        match encoding {
            Encoding::Q4_0 | Encoding::Q5_0 | Encoding::Q8_0 => {
                block[..2].copy_from_slice(&rng.half())
            }
            Encoding::Q4K | Encoding::Q5K | Encoding::Q5_1 => {
                block[..2].copy_from_slice(&rng.half());
                block[2..4].copy_from_slice(&rng.half());
            }
            Encoding::Q6K => block[208..].copy_from_slice(&rng.half()),
            Encoding::TQ2_0 => block[64..].copy_from_slice(&rng.half()),
            Encoding::F32 | Encoding::F16 | Encoding::BF16 => unreachable!(),
        }
        out.extend(block);
    }
    out
}

/// Activations with per-row scale variety, exact zeros and one all-zero block.
fn activations(positions: usize, cols: usize, rng: &mut Rng) -> Vec<f32> {
    let mut x: Vec<f32> = (0..positions * cols)
        .map(|i| {
            let scale = [1.0, 0.01, 30.0][(i / cols) % 3];
            if rng.next().is_multiple_of(17) {
                0.0
            } else {
                rng.unit() * scale
            }
        })
        .collect();
    x[..32.min(cols)].fill(0.0);
    x
}

fn kernels() -> (FmaKernels, Option<FmaKernels>) {
    let portable = FmaKernels::new(Request::PortableInt8).unwrap();
    match FmaKernels::new(Request::Power9Int8) {
        Ok(native) => (portable, Some(native)),
        Err(_) => {
            eprintln!(
                "POWER9 int8 kernels unavailable in this build/CPU: native comparisons skipped"
            );
            (portable, None)
        }
    }
}

fn shapes(encoding: Encoding) -> Vec<(usize, usize)> {
    let cols: &[usize] = match encoding {
        Encoding::Q4K | Encoding::Q5K | Encoding::Q6K | Encoding::TQ2_0 => &[256, 512, 2816],
        _ => &[32, 96, 704],
    };
    let mut out = Vec::new();
    for &c in cols {
        // 21 and 64 rows fill whole 4-row-group tiles (16 rows) and more.
        for rows in [1, 3, 4, 5, 9, 21, 64] {
            out.push((rows, c));
        }
    }
    out
}

fn run(k: &FmaKernels, w: &EncodedRows<'_>, input: &QuantizedInput) -> Vec<f32> {
    let mut out = vec![f32::NAN; input.positions() * w.rows()];
    k.project_quantized(w, input, &mut out).unwrap();
    out
}

#[test]
fn power9_matches_portable_bitwise() {
    let (portable, native) = kernels();
    let Some(native) = native else { return };
    let mut rng = Rng(0x2545f4914f6cdd1d);
    for encoding in QUANTIZED {
        for (rows, cols) in shapes(encoding) {
            let bytes = weights(encoding, rows, cols, &mut rng);
            let w = EncodedRows::new(encoding, rows, cols, &bytes).unwrap();
            for positions in [1, 2, 3, 5, 8, 33] {
                let x = activations(positions, cols, &mut rng);
                let input = QuantizedInput::new(encoding, &x, positions, cols).unwrap();
                let a = run(&portable, &w, &input);
                let b = run(&native, &w, &input);
                assert!(
                    a.iter().zip(&b).all(|(p, q)| p.to_bits() == q.to_bits()),
                    "{encoding:?} rows {rows} cols {cols} P {positions}"
                );
            }
        }
    }
}

/// Packed weights: any row sub-range, written with a stride and offset, is
/// bit-identical to the portable twin's rows; untouched cells stay NaN.
#[test]
fn packed_rows_match_portable_bitwise() {
    let (portable, native) = kernels();
    let Some(native) = native else { return };
    let mut rng = Rng(0x9e3779b97f4a7c15);
    for encoding in QUANTIZED {
        assert!(native.packs(encoding) && !portable.packs(encoding));
        for (rows, cols) in shapes(encoding) {
            let bytes = weights(encoding, rows, cols, &mut rng);
            let w = EncodedRows::new(encoding, rows, cols, &bytes).unwrap();
            let packed = native.pack(&w).unwrap();
            assert_eq!((packed.rows(), packed.cols()), (rows, cols));
            for positions in [1, 5, 33] {
                let x = activations(positions, cols, &mut rng);
                let input = QuantizedInput::new(encoding, &x, positions, cols).unwrap();
                let expected = run(&portable, &w, &input);
                for (row0, count) in [
                    (0, rows),
                    (rows - 1, 1),
                    (rows / 3, rows - rows / 3),
                    (1, rows.saturating_sub(2)),
                ] {
                    if count == 0 {
                        continue;
                    }
                    let (stride, offset) = (count + 3, 2);
                    let mut out = vec![f32::NAN; positions * stride];
                    native
                        .project_packed(&packed, row0, count, &input, &mut out, stride, offset)
                        .unwrap();
                    for p in 0..positions {
                        for c in 0..stride {
                            let got = out[p * stride + c];
                            if (offset..offset + count).contains(&c) {
                                let want = expected[p * rows + row0 + c - offset];
                                assert_eq!(
                                    got.to_bits(),
                                    want.to_bits(),
                                    "{encoding:?} {rows}x{cols} P {positions} rows {row0}+{count}"
                                );
                            } else {
                                assert!(got.is_nan(), "{encoding:?} wrote outside its range");
                            }
                        }
                    }
                }
                let mut out = vec![0.0; positions * rows];
                assert!(
                    native
                        .project_packed(&packed, rows, 1, &input, &mut out, rows, 0)
                        .is_err()
                );
                assert!(
                    native
                        .project_packed(&packed, 0, rows, &input, &mut out[1..], rows, 0)
                        .is_err()
                );
            }
        }
    }
}

/// |y - exact| <= sum_c |w_c| * (d_c / 2) (activation rounding, d_c the
/// column's activation block scale) plus slack for the binary32 scale
/// products, int->float conversions and block accumulation
/// (about (blocks + 2) * 2^-23 relative to sum |w x|).
#[test]
fn error_is_within_the_activation_rounding_bound() {
    let (portable, native) = kernels();
    let mut rng = Rng(0x9e3779b97f4a7c15);
    for encoding in QUANTIZED {
        let block = if matches!(
            encoding,
            Encoding::Q4K | Encoding::Q5K | Encoding::Q6K | Encoding::TQ2_0
        ) {
            256
        } else {
            32
        };
        for (rows, cols) in shapes(encoding) {
            let bytes = weights(encoding, rows, cols, &mut rng);
            let w = EncodedRows::new(encoding, rows, cols, &bytes).unwrap();
            let decoded = encoding.decode(&bytes, rows * cols).unwrap();
            let positions = 3;
            let x = activations(positions, cols, &mut rng);
            let input = QuantizedInput::new(encoding, &x, positions, cols).unwrap();
            for k in std::iter::once(&portable).chain(native.as_ref()) {
                let y = run(k, &w, &input);
                for p in 0..positions {
                    let xp = &x[p * cols..(p + 1) * cols];
                    let d: Vec<f64> = xp
                        .chunks(block)
                        .map(|b| b.iter().fold(0.0f32, |m, v| m.max(v.abs())) as f64 / 127.0)
                        .collect();
                    for r in 0..rows {
                        let wr = &decoded[r * cols..(r + 1) * cols];
                        let exact: f64 =
                            wr.iter().zip(xp).map(|(&a, &b)| a as f64 * b as f64).sum();
                        let magnitude: f64 = wr
                            .iter()
                            .zip(xp)
                            .map(|(&a, &b)| (a as f64 * b as f64).abs())
                            .sum();
                        let bound: f64 = wr
                            .iter()
                            .enumerate()
                            .map(|(c, &a)| (a as f64).abs() * d[c / block] * 0.5)
                            .sum::<f64>()
                            * (1.0 + 1e-5)
                            + magnitude * (cols / block + 4) as f64 * 1.2e-7
                            + 1e-30;
                        let error = (y[p * rows + r] as f64 - exact).abs();
                        assert!(
                            error <= bound,
                            "{} {encoding:?} {rows}x{cols} p{p} r{r}: error {error} > bound {bound}",
                            k.name()
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn zero_activations_give_exact_zeros() {
    let (portable, native) = kernels();
    let mut rng = Rng(7);
    for encoding in QUANTIZED {
        let (rows, cols) = shapes(encoding)[3];
        let bytes = weights(encoding, rows, cols, &mut rng);
        let w = EncodedRows::new(encoding, rows, cols, &bytes).unwrap();
        let input = QuantizedInput::new(encoding, &vec![0.0; 2 * cols], 2, cols).unwrap();
        for k in std::iter::once(&portable).chain(native.as_ref()) {
            assert!(run(k, &w, &input).iter().all(|v| *v == 0.0));
        }
    }
}

#[test]
fn invalid_inputs_and_nonfinite_scales_are_errors_without_writes() {
    let (portable, native) = kernels();
    let mut rng = Rng(11);
    for encoding in QUANTIZED {
        let (rows, cols) = shapes(encoding)[1];
        let mut bytes = weights(encoding, rows, cols, &mut rng);
        let x = activations(2, cols, &mut rng);
        let input = QuantizedInput::new(encoding, &x, 2, cols).unwrap();
        // Non-finite input, wrong shapes, F32 weights.
        let mut bad = x.clone();
        bad[1] = f32::NAN;
        assert_eq!(
            QuantizedInput::new(encoding, &bad, 2, cols).unwrap_err(),
            Error::NonFiniteInput
        );
        assert!(QuantizedInput::new(encoding, &x, 3, cols).is_err());
        assert!(QuantizedInput::new(Encoding::F32, &x, 2, cols).is_err());
        let other = if encoding == Encoding::Q8_0 {
            Encoding::Q5_0
        } else {
            Encoding::Q8_0
        };
        if cols % 32 == 0 {
            let mismatched = QuantizedInput::new(other, &x, 2, cols).unwrap();
            let w = EncodedRows::new(encoding, rows, cols, &bytes).unwrap();
            let mut out = vec![0.0; 2 * rows];
            assert!(
                portable
                    .project_quantized(&w, &mismatched, &mut out)
                    .is_err()
            );
        }
        // Infinite block scale in the LAST row: error, output untouched.
        let (_, size) = encoding.block_layout();
        let at = bytes.len() - size
            + match encoding {
                Encoding::Q6K => 208,
                Encoding::TQ2_0 => 64,
                _ => 0,
            };
        bytes[at..at + 2].copy_from_slice(&0x7c00u16.to_le_bytes());
        let w = EncodedRows::new(encoding, rows, cols, &bytes).unwrap();
        for k in std::iter::once(&portable).chain(native.as_ref()) {
            let mut out = vec![-3.0; 2 * rows];
            assert_eq!(
                k.project_quantized(&w, &input, &mut out),
                Err(Error::NonFiniteWeight)
            );
            assert!(out.iter().all(|v| *v == -3.0));
        }
    }
    let fma = FmaKernels::portable();
    assert!(!fma.quantizes(Encoding::Q4K));
    assert!(portable.quantizes(Encoding::Q4K) && !portable.quantizes(Encoding::F32));
}

/// Binary32 transposed partials: POWER9 == portable bitwise, and within
/// (rows + 2) * 2^-23 * sum |k w| of the exact F64 kernel.
#[test]
fn transposed_f32_matches_twin_and_f64_within_bound() {
    let (portable, native) = kernels();
    let exact = FmaKernels::portable();
    let mut rng = Rng(0x51ed270b);
    for encoding in [
        Encoding::F32,
        Encoding::Q4_0,
        Encoding::Q5_0,
        Encoding::Q8_0,
        Encoding::Q4K,
        Encoding::Q6K,
    ] {
        let cols = if matches!(
            encoding,
            Encoding::Q4K | Encoding::Q5K | Encoding::Q6K | Encoding::TQ2_0
        ) {
            512
        } else {
            96
        };
        for rows in [1, 7, 33] {
            let bytes: Vec<u8> = if encoding == Encoding::F32 {
                (0..rows * cols)
                    .flat_map(|_| rng.unit().to_le_bytes())
                    .collect()
            } else {
                weights(encoding, rows, cols, &mut rng)
            };
            let w = EncodedRows::new(encoding, rows, cols, &bytes).unwrap();
            let decoded = encoding.decode(&bytes, rows * cols).unwrap();
            for positions in [1, 4, 5, 9] {
                let k: Vec<f32> = (0..positions * rows)
                    .map(|_| rng.unit().abs() * 1e-3)
                    .collect();
                let mut scratch = vec![0.0; rows * cols];
                let mut run = |kern: &FmaKernels| {
                    let mut acc = vec![0.25f64; positions * cols];
                    kern.accumulate_transposed(&w, 0, &k, rows, positions, &mut acc, &mut scratch)
                        .unwrap();
                    acc
                };
                let a = run(&portable);
                let e = run(&exact);
                if let Some(native) = &native {
                    let b = run(native);
                    assert!(
                        a.iter().zip(&b).all(|(x, y)| x.to_bits() == y.to_bits()),
                        "{encoding:?}"
                    );
                }
                for p in 0..positions {
                    for c in 0..cols {
                        let mag: f64 = (0..rows)
                            .map(|r| (k[p * rows + r] as f64 * decoded[r * cols + c] as f64).abs())
                            .sum();
                        let bound = mag * (rows as f64 + 2.0) * 1.2e-7 + 1e-15;
                        let i = p * cols + c;
                        assert!(
                            (a[i] - e[i]).abs() <= bound,
                            "{encoding:?} {rows} {positions}"
                        );
                    }
                }
            }
        }
    }
}

/// Binary32 attention: POWER9 == portable bitwise, close to the exact F64
/// reference, masks honoured, GQA groups shared.
#[test]
fn attention_f32_matches_twin_and_reference() {
    let (portable, native) = kernels();
    let mut rng = Rng(0xa77e);
    for (qheads, kv_heads, dim, nkeys) in [
        (16, 8, 256, 37),
        (16, 2, 512, 5),
        (4, 4, 16, 1),
        (2, 1, 32, 64),
        // Key counts and widths off the 4-key / 4-tile blocking.
        (6, 3, 48, 6),
        (4, 2, 80, 7),
        (8, 2, 144, 3),
        (2, 1, 320, 130),
        // Head pairs: odd groups (pairs never cross a group), one key, two
        // keys, one 16-float tile, tails after 32-float blocks.
        (6, 2, 16, 9),
        (9, 3, 32, 2),
        (10, 2, 48, 11),
        (8, 1, 16, 1),
        (4, 1, 96, 3),
        (16, 2, 512, 130),
    ] {
        let query: Vec<f32> = (0..qheads * dim).map(|_| rng.unit() * 0.5).collect();
        let keys: Vec<f32> = (0..nkeys * kv_heads * dim)
            .map(|_| rng.unit() * 0.5)
            .collect();
        let values: Vec<f32> = (0..nkeys * kv_heads * dim).map(|_| rng.unit()).collect();
        let mask: Vec<bool> = (0..nkeys).map(|t| t % 3 != 1 || nkeys == 1).collect();
        for allowed in [None, Some(mask.as_slice())] {
            let a = portable
                .attention(&query, &keys, &values, kv_heads, dim, allowed)
                .unwrap();
            let e = crate::kernels::reference::attention(
                &query, &keys, &values, kv_heads, dim, allowed,
            )
            .unwrap();
            if let Some(native) = &native {
                let b = native
                    .attention(&query, &keys, &values, kv_heads, dim, allowed)
                    .unwrap();
                assert!(a.iter().zip(&b).all(|(x, y)| x.to_bits() == y.to_bits()));
                // A nonfinite value or key is an error for both.
                let mut bad = values.clone();
                bad[(nkeys - 1) * kv_heads * dim + dim - 1] = f32::INFINITY;
                let mut bad_keys = keys.clone();
                bad_keys[dim - 1] = f32::NAN;
                for k in [&portable, native] {
                    assert!(
                        k.attention(&query, &keys, &bad, kv_heads, dim, None)
                            .is_err()
                    );
                    assert!(
                        k.attention(&query, &bad_keys, &values, kv_heads, dim, None)
                            .is_err()
                    );
                }
            }
            for (x, y) in a.iter().zip(&e) {
                assert!((x - y).abs() <= 2e-5 * (1.0 + y.abs()), "{x} vs {y}");
            }
        }
    }
    let q = vec![1.0; 32];
    let kv = vec![1.0; 32];
    assert!(
        portable
            .attention(&q, &kv, &kv, 1, 32, Some(&[false]))
            .is_err()
    );
    assert!(
        portable
            .attention(&q[..24], &kv[..24], &kv[..24], 1, 24, None)
            .is_err()
    );
}

/// Group-wise quantization (in any order) equals the one-call form exactly.
#[test]
fn grouped_quantization_matches_single_call() {
    let (portable, _) = kernels();
    let mut rng = Rng(0x6006);
    for encoding in QUANTIZED {
        let cols = if matches!(
            encoding,
            Encoding::Q4K | Encoding::Q5K | Encoding::Q6K | Encoding::TQ2_0
        ) {
            512
        } else {
            96
        };
        for positions in [1, 4, 6, 9] {
            let x = activations(positions, cols, &mut rng);
            let whole = QuantizedInput::new(encoding, &x, positions, cols).unwrap();
            let mut grouped = QuantizedInput::allocate(encoding, positions, cols).unwrap();
            let mut groups: Vec<_> = grouped.groups_mut().into_iter().enumerate().collect();
            groups.reverse();
            for (g, group) in &mut groups {
                let rows = std::array::from_fn(|l| {
                    let p = 4 * *g + l;
                    (p < positions).then(|| &x[p * cols..(p + 1) * cols])
                });
                group.quantize(rows).unwrap();
            }
            drop(groups);
            let rows = 3;
            let bytes = weights(encoding, rows, cols, &mut rng);
            let w = EncodedRows::new(encoding, rows, cols, &bytes).unwrap();
            let a = run(&portable, &w, &whole);
            let b = run(&portable, &w, &grouped);
            assert!(
                a.iter().zip(&b).all(|(x, y)| x.to_bits() == y.to_bits()),
                "{encoding:?} {positions}"
            );
        }
    }
}

/// The POWER9 group quantizer equals the portable one bit for bit (bytes,
/// scales, sums and residuals), including padding rows and zero blocks.
#[test]
fn power9_group_quantizer_matches_portable() {
    #[cfg(all(feature = "power9", target_arch = "powerpc64"))]
    {
        if crate::kernels::Kernels::new(crate::kernels::Backend::Power9).is_err() {
            return;
        }
        let mut rng = Rng(0x9a17);
        for encoding in QUANTIZED {
            let cols = if matches!(
                encoding,
                Encoding::Q4K | Encoding::Q5K | Encoding::Q6K | Encoding::TQ2_0
            ) {
                512
            } else {
                96
            };
            let nb = cols
                / if matches!(
                    encoding,
                    Encoding::Q4K | Encoding::Q5K | Encoding::Q6K | Encoding::TQ2_0
                ) {
                    256
                } else {
                    32
                };
            let x = activations(4, cols, &mut rng);
            for present in [[true; 4], [true, false, true, false], [false; 4]] {
                let run = |f: QuantizeGroupFn| {
                    let rows: Vec<*const f32> = (0..4)
                        .map(|l| {
                            if present[l] {
                                x[l * cols..].as_ptr()
                            } else {
                                std::ptr::null()
                            }
                        })
                        .collect();
                    let mut q = vec![0u8; 16 * cols];
                    let mut da = vec![0f32; nb * 4];
                    let mut s = vec![0i16; nb * 32];
                    let mut res = vec![vec![0f32; cols]; 4];
                    let outs: Vec<*mut f32> = res.iter_mut().map(|r| r.as_mut_ptr()).collect();
                    // SAFETY: buffers sized per the C contract.
                    let code = unsafe {
                        f(
                            kernel_kind(encoding),
                            rows.as_ptr(),
                            cols,
                            q.as_mut_ptr(),
                            da.as_mut_ptr(),
                            s.as_mut_ptr(),
                            outs.as_ptr(),
                        )
                    };
                    assert_eq!(code, 0);
                    (
                        q,
                        da.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                        s,
                        res.concat().iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                    )
                };
                assert_eq!(
                    run(pq8_quantize_group),
                    run(pq8_quantize_group_power9),
                    "{encoding:?} {present:?}"
                );
            }
        }
    }
}

/// Any query-head range equals that slice of the full result, bitwise, for
/// the exact, portable binary32 and POWER9 binary32 kernels.
#[test]
fn attention_head_ranges_match_full_attention() {
    let mut rng = Rng(0x7f4a7c159e3779b9);
    let mut backends = vec![
        FmaKernels::new(Request::Portable).unwrap(),
        FmaKernels::new(Request::PortableInt8).unwrap(),
    ];
    backends.extend(FmaKernels::new(Request::Power9Int8).ok());
    for (qheads, kv_heads, dim, nkeys) in [
        (16, 8, 32, 7),
        (8, 1, 64, 33),
        (4, 2, 16, 1),
        (6, 2, 48, 5),
        (9, 3, 16, 2),
    ] {
        let query: Vec<f32> = (0..qheads * dim).map(|_| rng.unit()).collect();
        let keys: Vec<f32> = (0..nkeys * kv_heads * dim).map(|_| rng.unit()).collect();
        let values: Vec<f32> = (0..nkeys * kv_heads * dim).map(|_| rng.unit()).collect();
        let mask: Vec<bool> = (0..nkeys).map(|t| t % 3 != 1 || t == 0).collect();
        for k in &backends {
            for allowed in [None, Some(mask.as_slice())] {
                let full = k
                    .attention(&query, &keys, &values, kv_heads, dim, allowed)
                    .unwrap();
                for a in 0..qheads {
                    for b in a..=qheads {
                        let part = k
                            .attention_heads(
                                &query,
                                &keys,
                                &values,
                                kv_heads,
                                dim,
                                allowed,
                                Some(a..b),
                            )
                            .unwrap();
                        let want = &full[a * dim..b * dim];
                        assert!(
                            part.iter()
                                .zip(want)
                                .all(|(x, y)| x.to_bits() == y.to_bits())
                                && part.len() == want.len(),
                            "{} heads {a}..{b}",
                            k.name()
                        );
                    }
                }
                assert!(
                    k.attention_heads(
                        &query,
                        &keys,
                        &values,
                        kv_heads,
                        dim,
                        allowed,
                        Some(0..qheads + 1)
                    )
                    .is_err()
                );
            }
        }
    }
}

/// The multi-query kernel equals one single-query call per query, bitwise,
/// with per-query masks (causal and windowed shapes).
#[test]
fn attention_multi_matches_single_queries() {
    let mut rng = Rng(0x51ed270b);
    let mut backends = vec![FmaKernels::new(Request::Portable).unwrap()];
    backends.extend(FmaKernels::new(Request::Power9Int8).ok());
    for (qheads, kv_heads, dim, nkeys, nq) in [
        (16, 8, 32, 9, 5),
        (8, 1, 64, 33, 3),
        (4, 2, 16, 2, 7),
        (16, 2, 48, 1, 1),
    ] {
        let queries: Vec<Vec<f32>> = (0..nq)
            .map(|_| (0..qheads * dim).map(|_| rng.unit()).collect())
            .collect();
        let keys: Vec<f32> = (0..nkeys * kv_heads * dim).map(|_| rng.unit()).collect();
        let values: Vec<f32> = (0..nkeys * kv_heads * dim).map(|_| rng.unit()).collect();
        let masks: Vec<Vec<bool>> = (0..nq)
            .map(|i| (0..nkeys).map(|t| t <= nkeys / 2 + i || t == 0).collect())
            .collect();
        let qrefs: Vec<&[f32]> = queries.iter().map(Vec::as_slice).collect();
        let mrefs: Vec<&[bool]> = masks.iter().map(Vec::as_slice).collect();
        for k in &backends {
            for masked in [false, true] {
                let m = masked.then_some(mrefs.as_slice());
                let multi = k
                    .attention_multi(&qrefs, &keys, &values, kv_heads, dim, m)
                    .unwrap();
                for (i, q) in queries.iter().enumerate() {
                    let single = k
                        .attention(q, &keys, &values, kv_heads, dim, m.map(|m| m[i]))
                        .unwrap();
                    assert!(
                        multi[i]
                            .iter()
                            .zip(&single)
                            .all(|(a, b)| a.to_bits() == b.to_bits()),
                        "{} query {i} of {nq}, {qheads}/{kv_heads}x{dim}, {nkeys} keys, masked {masked}",
                        k.name()
                    );
                }
            }
        }
    }
}

/// KV-head ranges of the blocked multi-query kernel: their concatenation
/// equals the single-query results bitwise, with key blocks fully masked for
/// some rows (sliding windows) and more keys than one key block.
#[test]
fn attention_multi_kv_ranges_match_single_queries() {
    let mut rng = Rng(0x7a11ed);
    let mut backends = vec![FmaKernels::new(Request::Portable).unwrap()];
    backends.extend(FmaKernels::new(Request::Power9Int8).ok());
    // dim 512: 64 keys per block; dim 256: 128.
    for (qheads, kv_heads, dim, nkeys, nq, window) in [
        (16, 2, 512, 150, 6, 40),
        (16, 8, 256, 300, 5, 130),
        (4, 2, 16, 37, 9, 5),
    ] {
        let queries: Vec<Vec<f32>> = (0..nq)
            .map(|_| (0..qheads * dim).map(|_| rng.unit()).collect())
            .collect();
        let keys: Vec<f32> = (0..nkeys * kv_heads * dim).map(|_| rng.unit()).collect();
        let values: Vec<f32> = (0..nkeys * kv_heads * dim).map(|_| rng.unit()).collect();
        // Query i at position nkeys - nq + i sees the last `window` keys up to it.
        let masks: Vec<Vec<bool>> = (0..nq)
            .map(|i| {
                let pos = nkeys - nq + i;
                (0..nkeys).map(|t| t <= pos && t + window > pos).collect()
            })
            .collect();
        let qrefs: Vec<&[f32]> = queries.iter().map(Vec::as_slice).collect();
        let mrefs: Vec<&[bool]> = masks.iter().map(Vec::as_slice).collect();
        let group = qheads / kv_heads;
        for k in &backends {
            for masked in [false, true] {
                let m = masked.then_some(mrefs.as_slice());
                let singles: Vec<Vec<f32>> = queries
                    .iter()
                    .enumerate()
                    .map(|(i, q)| {
                        k.attention(q, &keys, &values, kv_heads, dim, m.map(|m| m[i]))
                            .unwrap()
                    })
                    .collect();
                for kv in [0..kv_heads, 0..1, kv_heads - 1..kv_heads] {
                    let part = k
                        .attention_multi_kv(&qrefs, &keys, &values, kv_heads, dim, m, kv.clone())
                        .unwrap();
                    for (i, single) in singles.iter().enumerate() {
                        let want = &single[kv.start * group * dim..kv.end * group * dim];
                        assert!(
                            part[i].len() == want.len()
                                && part[i]
                                    .iter()
                                    .zip(want)
                                    .all(|(a, b)| a.to_bits() == b.to_bits()),
                            "{} query {i}, {qheads}/{kv_heads}x{dim}, {nkeys} keys, kv {kv:?}, masked {masked}",
                            k.name()
                        );
                    }
                }
            }
        }
    }
}

/// Diagnostic: multi-query attention throughput, one thread (run with
/// --ignored --nocapture).
#[test]
#[ignore]
fn attention_multi_throughput() {
    let Ok(k) = FmaKernels::new(Request::Power9Int8) else {
        return;
    };
    let mut rng = Rng(7);
    for (label, qheads, kv_heads, dim, nkeys, nq) in [
        ("sliding", 16, 8, 256, 1279, 8),
        ("global", 16, 2, 512, 3194, 8),
    ] {
        let queries: Vec<Vec<f32>> = (0..nq)
            .map(|_| (0..qheads * dim).map(|_| rng.unit() * 0.1).collect())
            .collect();
        let keys: Vec<f32> = (0..nkeys * kv_heads * dim).map(|_| rng.unit()).collect();
        let values: Vec<f32> = (0..nkeys * kv_heads * dim).map(|_| rng.unit()).collect();
        let qrefs: Vec<&[f32]> = queries.iter().map(Vec::as_slice).collect();
        for _ in 0..2 {
            k.attention_multi(&qrefs, &keys, &values, kv_heads, dim, None)
                .unwrap();
        }
        let n = 10;
        let start = std::time::Instant::now();
        for _ in 0..n {
            std::hint::black_box(
                k.attention_multi(&qrefs, &keys, &values, kv_heads, dim, None)
                    .unwrap(),
            );
        }
        let secs = start.elapsed().as_secs_f64() / n as f64;
        let macs = 2.0 * (nq * qheads * nkeys * dim) as f64;
        eprintln!(
            "{label}: {:.2} ms per call, {:.2} GMAC/s one thread",
            secs * 1e3,
            macs / secs / 1e9
        );
    }
}

#[test]
fn planes_writer_matches_two_passes() {
    let mut rng = Rng(11);
    for encoding in [Encoding::Q4_0, Encoding::Q8_0, Encoding::Q4K, Encoding::Q6K] {
        let (p, cols) = (8, 512);
        let x: Vec<Vec<f32>> = (0..p)
            .map(|_| (0..cols).map(|_| rng.unit() * 3.0).collect())
            .collect();
        let rows: Vec<&[f32]> = x.iter().map(Vec::as_slice).collect();
        // Plane 1 with residuals, then [rows, residuals] (the two-pass form).
        let mut first = QuantizedInput::allocate(encoding, p, cols).unwrap();
        let mut residual = vec![vec![0f32; cols]; p];
        {
            let mut outs = residual.iter_mut();
            for (g, mut group) in first.groups_mut().into_iter().enumerate() {
                let these = std::array::from_fn(|l| Some(rows[4 * g + l]));
                let o = std::array::from_fn(|_| outs.next().map(Vec::as_mut_slice));
                group.quantize_with_residual(these, o).unwrap();
            }
        }
        let both: Vec<&[f32]> = rows
            .iter()
            .copied()
            .chain(residual.iter().map(Vec::as_slice))
            .collect();
        let mut want = QuantizedInput::allocate(encoding, 2 * p, cols).unwrap();
        for (g, mut group) in want.groups_mut().into_iter().enumerate() {
            group
                .quantize(std::array::from_fn(|l| both.get(4 * g + l).copied()))
                .unwrap();
        }
        let mut got = QuantizedInput::allocate(encoding, 2 * p, cols).unwrap();
        {
            let writer = got.planes_writer(&rows).unwrap();
            let blocks = writer.blocks();
            let mut scratch = vec![0f32; 4 * cols];
            for g in (0..writer.groups()).rev() {
                for (b0, b1) in [(blocks / 3, blocks), (0, blocks / 3)] {
                    // SAFETY: one thread; distinct items.
                    unsafe { writer.quantize(g, b0, b1, &mut scratch) }.unwrap();
                }
            }
        }
        assert_eq!(got.as_bytes(), want.as_bytes(), "{encoding:?}");
    }
}

#[test]
fn gather_matches_requantizing() {
    let mut rng = Rng(23);
    for encoding in QUANTIZED {
        let (p, cols) = (11, 512);
        let x: Vec<Vec<f32>> = (0..p)
            .map(|_| (0..cols).map(|_| rng.unit() * 2.0).collect())
            .collect();
        let rows: Vec<&[f32]> = x.iter().map(Vec::as_slice).collect();
        let quantize = |rows: &[&[f32]]| {
            let mut q = QuantizedInput::allocate(encoding, rows.len(), cols).unwrap();
            for (g, mut group) in q.groups_mut().into_iter().enumerate() {
                group
                    .quantize(std::array::from_fn(|l| rows.get(4 * g + l).copied()))
                    .unwrap();
            }
            q
        };
        let all = quantize(&rows);
        for pick in [
            vec![3],
            vec![0, 7],
            vec![10, 2, 5],
            vec![1, 2, 3, 4, 9, 6, 0],
        ] {
            let subset: Vec<&[f32]> = pick.iter().map(|&i| rows[i]).collect();
            assert_eq!(
                all.gather(&pick).unwrap().as_bytes(),
                quantize(&subset).as_bytes(),
                "{encoding:?} {pick:?}"
            );
        }
    }
}

fn shared_input(x: &[f32], cols: usize) -> QuantizedInput {
    let rows: Vec<&[f32]> = x.chunks_exact(cols).collect();
    let mut input = QuantizedInput::allocate(Encoding::Q4_0, rows.len() * 2, cols).unwrap();
    {
        let writer = input.shared_planes_writer(&rows).unwrap();
        let mut scratch = vec![0.0; 4 * cols];
        for g in 0..writer.groups() {
            for b in (0..writer.blocks()).rev() {
                // SAFETY: serial calls cover disjoint blocks of every group.
                unsafe { writer.quantize(g, b, b + 1, &mut scratch) }.unwrap();
            }
        }
    }
    input
}

#[test]
fn shared_scale_twins_and_gather() {
    let (portable, native) = kernels();
    let mut rng = Rng(0x384983);
    for (rows, cols) in shapes(Encoding::Q4_0) {
        let bytes = weights(Encoding::Q4_0, rows, cols, &mut rng);
        let w = EncodedRows::new(Encoding::Q4_0, rows, cols, &bytes).unwrap();
        for positions in [4, 8, 36] {
            let x = activations(positions, cols, &mut rng);
            let whole = shared_input(&x, cols);
            // Include odd plane offsets and incomplete two-position tiles.
            for selected in [vec![positions - 1], vec![3, 0, 2], (0..positions).collect()] {
                let n = selected.len();
                let both: Vec<_> = selected
                    .iter()
                    .copied()
                    .chain(selected.iter().map(|&p| p + positions))
                    .collect();
                let input = whole.gather(&both).unwrap();
                let expected = run(&portable, &w, &input);
                assert!(expected[n * rows..].iter().all(|v| v.to_bits() == 0));
                if let Some(native) = &native {
                    let got = run(native, &w, &input);
                    assert_eq!(
                        got.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                        expected.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                        "shared twin {rows}x{cols} P {n}"
                    );
                    let packed = native.pack(&w).unwrap();
                    for (row0, count) in [(0, rows), (rows - 1, 1), (rows / 3, rows - rows / 3)] {
                        let (stride, offset) = (count + 3, 2);
                        let mut out = vec![f32::NAN; 2 * n * stride];
                        native
                            .project_packed(&packed, row0, count, &input, &mut out, stride, offset)
                            .unwrap();
                        for p in 0..2 * n {
                            for c in 0..stride {
                                if (offset..offset + count).contains(&c) {
                                    assert_eq!(
                                        out[p * stride + c].to_bits(),
                                        expected[p * rows + row0 + c - offset].to_bits(),
                                        "shared packed {rows}x{cols} P {n} row {row0}+{count}"
                                    );
                                } else {
                                    assert!(out[p * stride + c].is_nan());
                                }
                            }
                        }
                    }
                }
            }
            assert!(whole.gather(&[0, 1]).is_err());
        }
    }
}

#[test]
fn shared_scale_quantizer_twins_and_error_bound() {
    let mut rng = Rng(0x3994a);
    for (cols, exponent) in [32, 96, 704]
        .into_iter()
        .flat_map(|cols| [-80, -12, 0, 20, 100].map(move |exponent| (cols, exponent)))
    {
        let mut x = activations(4, cols, &mut rng);
        for v in &mut x {
            *v *= 2.0_f32.powi(exponent);
        }
        let rows: [*const f32; 4] = std::array::from_fn(|p| x[p * cols..].as_ptr());
        let mut reference = QuantizedInput::allocate(Encoding::Q4_0, 8, cols).unwrap();
        {
            let mut groups = reference.groups_mut();
            let (first, second) = groups.split_at_mut(1);
            // SAFETY: exactly two distinct groups of four input rows.
            status(unsafe {
                pq8_shared_quantize(
                    rows.as_ptr(),
                    cols,
                    0,
                    cols / 32,
                    first[0].q.as_mut_ptr(),
                    first[0].da.as_mut_ptr(),
                    first[0].s.as_mut_ptr(),
                    second[0].q.as_mut_ptr(),
                    second[0].da.as_mut_ptr(),
                    second[0].s.as_mut_ptr(),
                )
            })
            .unwrap();
        }
        let actual = shared_input(&x, cols);
        assert_eq!(
            reference.as_bytes(),
            actual.as_bytes(),
            "shared quantizer {cols}"
        );
        let (_, da, _) = actual.regions(8);
        for p in 0..4 {
            for b in 0..cols / 32 {
                let ix = (b * 4 + p) * 4;
                let d1 =
                    f32::from_ne_bytes(actual.as_bytes()[da + ix..da + ix + 4].try_into().unwrap());
                let ix2 = ((cols / 32 + b) * 4 + p) * 4;
                let d2 = f32::from_ne_bytes(
                    actual.as_bytes()[da + ix2..da + ix2 + 4]
                        .try_into()
                        .unwrap(),
                );
                assert_eq!(d2.to_bits(), (d1 / 256.0).to_bits());
                for i in 0..32 {
                    let c = b * 32 + i;
                    let offset = (c / 4) * 16 + c % 4;
                    let hi = actual.as_bytes()[p * cols * 4 + offset] as i8 as f64;
                    let lo = actual.as_bytes()[(p + 4) * cols * 4 + offset] as i8 as f64;
                    let reconstructed = d1 as f64 * hi + d2 as f64 * lo;
                    let v = x[p * cols + c] as f64;
                    let bound = d2 as f64 / 2.0 + 2.0 * f32::EPSILON as f64 * v.abs();
                    assert!(
                        (reconstructed - v).abs() <= bound,
                        "{v} -> {reconstructed}, bound {bound}"
                    );
                }
            }
        }
    }
}

#[test]
fn shared_scale_rejects_invalid_input_without_writes() {
    let mut input = QuantizedInput::allocate(Encoding::Q4_0, 8, 32).unwrap();
    let x = [1.0; 32];
    let rows = [&x[..]; 4];
    assert!(input.shared_planes_writer(&rows[..3]).is_err());
    for value in [f32::NAN, f32::INFINITY, f32::from_bits(1)] {
        let bad = [value; 32];
        let rows = [&bad[..], &x[..], &x[..], &x[..]];
        {
            let writer = input.shared_planes_writer(&rows).unwrap();
            // SAFETY: sole writer of this group and block.
            assert!(unsafe { writer.quantize(0, 0, 1, &mut [0.0; 128]) }.is_err());
        }
        assert!(input.as_bytes().iter().all(|&v| v == 0));
    }
}

#[test]
fn shared_scale_storage_can_be_requantized() {
    let portable = FmaKernels::new(Request::PortableInt8).unwrap();
    let mut rng = Rng(0x4f02d3);
    let cols = 96;
    let x = activations(4, cols, &mut rng);
    let rows: Vec<&[f32]> = x.chunks_exact(cols).collect();
    let mut reused = shared_input(&x, cols);
    let mut ordinary = QuantizedInput::allocate(Encoding::Q4_0, 8, cols).unwrap();
    for input in [&mut reused, &mut ordinary] {
        let writer = input.planes_writer(&rows).unwrap();
        // SAFETY: sole writer of this group and all three blocks.
        unsafe { writer.quantize(0, 0, 3, &mut [0.0; 384]) }.unwrap();
    }
    let bytes = weights(Encoding::Q4_0, 21, cols, &mut rng);
    let w = EncodedRows::new(Encoding::Q4_0, 21, cols, &bytes).unwrap();
    assert_eq!(reused.as_bytes(), ordinary.as_bytes());
    assert_eq!(run(&portable, &w, &reused), run(&portable, &w, &ordinary));
    {
        let writer = reused.shared_planes_writer(&rows).unwrap();
        // SAFETY: sole writer of this group and all three blocks.
        unsafe { writer.quantize(0, 0, 3, &mut [0.0; 384]) }.unwrap();
    }
    let shared = shared_input(&x, cols);
    assert_eq!(run(&portable, &w, &reused), run(&portable, &w, &shared));
    let all = activations(8, cols, &mut rng);
    for (g, mut group) in reused.groups_mut().into_iter().enumerate() {
        group
            .quantize(std::array::from_fn(|l| {
                let p = 4 * g + l;
                Some(&all[p * cols..(p + 1) * cols])
            }))
            .unwrap();
    }
    let fresh = QuantizedInput::new(Encoding::Q4_0, &all, 8, cols).unwrap();
    assert_eq!(reused.as_bytes(), fresh.as_bytes());
    assert_eq!(run(&portable, &w, &reused), run(&portable, &w, &fresh));
}

#[test]
fn shared_scale_signed_low_byte_and_rounding_ties() {
    // max=127 fixes d1=1, d2=1/256. Midpoints carry into the high
    // plane and use -128 in the low plane; finer ties round to even.
    let mut row = [0.0; 32];
    row[..8].copy_from_slice(&[
        127.0,
        -127.0,
        0.5,
        -0.5,
        1.0 / 512.0,
        -1.0 / 512.0,
        3.0 / 512.0,
        -3.0 / 512.0,
    ]);
    let x = row.repeat(4);
    let input = shared_input(&x, 32);
    let high = [127, -127, 1, 0, 0, 0, 0, 0];
    let low = [0, 0, -128, -128, 0, 0, 2, -2];
    for i in 0..8 {
        let offset = i / 4 * 16 + i % 4;
        assert_eq!(input.as_bytes()[offset] as i8 as i32, high[i]);
        assert_eq!(input.as_bytes()[4 * 32 * 4 + offset] as i8 as i32, low[i]);
    }
}

#[test]
fn attention_score_bounds_handle_holes_and_query_tails() {
    let portable = FmaKernels::new(Request::PortableInt8).unwrap();
    let Some(native) = FmaKernels::new(Request::Power9Int8).ok() else {
        return;
    };
    let mut rng = Rng(0x78124);
    let (qheads, kvheads, nkeys) = (3, 3, 259);
    for dim in [48, 256, 512] {
        let keys: Vec<f32> = (0..nkeys * kvheads * dim).map(|_| rng.unit()).collect();
        let values: Vec<f32> = (0..nkeys * kvheads * dim)
            .map(|i| match i % 13 {
                0 => 0.0,
                1 => -0.0,
                _ => rng.unit(),
            })
            .collect();
        for nq in [1, 2, 3, 5] {
            let queries: Vec<Vec<f32>> = (0..nq)
                .map(|_| (0..qheads * dim).map(|_| rng.unit() * 0.1).collect())
                .collect();
            // Odd interval boundaries, holes, wholly masked blocks, and
            // disjoint masks in one four-query tile. Each row has a live key.
            let masks: Vec<Vec<bool>> = (0..nq)
                .map(|i| {
                    let live: &[usize] = match i % 4 {
                        0 => &[5, 7, 13, 17],
                        1 => &[129, 131, 139],
                        2 => &[255, 257],
                        _ => &[258],
                    };
                    (0..nkeys).map(|t| live.contains(&t)).collect()
                })
                .collect();
            let qrefs: Vec<_> = queries.iter().map(Vec::as_slice).collect();
            let mrefs: Vec<_> = masks.iter().map(Vec::as_slice).collect();
            let got = native
                .attention_multi_kv(
                    &qrefs,
                    &keys,
                    &values,
                    kvheads,
                    dim,
                    Some(&mrefs),
                    0..kvheads,
                )
                .unwrap();
            for i in 0..nq {
                let want = portable
                    .attention(&queries[i], &keys, &values, kvheads, dim, Some(&masks[i]))
                    .unwrap();
                assert_eq!(
                    got[i].iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                    want.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                    "dim={dim} nq={nq} query={i}"
                );
            }
        }
    }
}

/// Packed projection with overflowing outputs: the status is
/// `NonFiniteOutput`, and every cell (finite or not) is bit-identical to the
/// portable twin, for whole-tile and ragged row ranges, strides and offsets.
#[test]
fn packed_nonfinite_outputs_match_portable_bitwise() {
    let (portable, native) = kernels();
    let Some(native) = native else { return };
    let mut rng = Rng(0x51ed270b0a5b7c13);
    for encoding in QUANTIZED {
        let (rows, cols) = shapes(encoding)[4];
        let bytes = weights(encoding, rows, cols, &mut rng);
        let w = EncodedRows::new(encoding, rows, cols, &bytes).unwrap();
        let packed = native.pack(&w).unwrap();
        for positions in [1, 2, 5, 6] {
            // Position 1 (when present) is huge: its outputs overflow binary32.
            let mut x = activations(positions, cols, &mut rng);
            if positions > 1 {
                for v in &mut x[cols..2 * cols] {
                    *v = rng.unit() * 3.0e38;
                }
            }
            let input = QuantizedInput::new(encoding, &x, positions, cols).unwrap();
            let mut expected = vec![f32::NAN; positions * rows];
            let want = portable.project_quantized(&w, &input, &mut expected);
            assert_eq!(want.is_err(), positions > 1, "{encoding:?} P {positions}");
            for (row0, count) in [
                (0, rows),
                (0, 4.min(rows)),
                (1, rows - 1),
                (rows / 2, rows - rows / 2),
                (rows - 1, 1),
            ] {
                let (stride, offset) = (count + 5, 3);
                let mut out = vec![f32::NAN; positions * stride];
                let got =
                    native.project_packed(&packed, row0, count, &input, &mut out, stride, offset);
                let any_bad = (0..positions)
                    .any(|p| (0..count).any(|c| !expected[p * rows + row0 + c].is_finite()));
                assert_eq!(
                    got.is_err(),
                    any_bad,
                    "{encoding:?} P {positions} {row0}+{count}"
                );
                if any_bad {
                    assert_eq!(got, Err(Error::NonFiniteOutput));
                }
                for p in 0..positions {
                    for c in 0..count {
                        assert_eq!(
                            out[p * stride + offset + c].to_bits(),
                            expected[p * rows + row0 + c].to_bits(),
                            "{encoding:?} P {positions} {row0}+{count} p {p} c {c}"
                        );
                    }
                }
            }
        }
    }
}

/// Diagnostic: Q4_0 prompt tile throughput, one thread (run with
/// --ignored --nocapture; env TILE_ROWS, TILE_POS, TILE_ITERS).
#[test]
#[ignore]
fn q4_0_tile_throughput() {
    let Ok(k) = FmaKernels::new(Request::Power9Int8) else {
        return;
    };
    let env = |n: &str, d: usize| {
        std::env::var(n)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(d)
    };
    let (rows, positions, iters) = (
        env("TILE_ROWS", 8192),
        env("TILE_POS", 4),
        env("TILE_ITERS", 6),
    );
    let cols = 2816;
    let mut rng = Rng(11);
    let bytes = weights(Encoding::Q4_0, rows, cols, &mut rng);
    let w = EncodedRows::new(Encoding::Q4_0, rows, cols, &bytes).unwrap();
    let packed = k.pack(&w).unwrap();
    let x = activations(positions, cols, &mut rng);
    let input = QuantizedInput::new(Encoding::Q4_0, &x, positions, cols).unwrap();
    let mut out = vec![0.0f32; positions * rows];
    let mut best = f64::MAX;
    for _ in 0..iters {
        let start = std::time::Instant::now();
        k.project_packed(&packed, 0, rows, &input, &mut out, rows, 0)
            .unwrap();
        best = best.min(start.elapsed().as_secs_f64());
    }
    let h = out.iter().fold(0u64, |h, v| {
        (h ^ v.to_bits() as u64).wrapping_mul(0x100000001b3)
    });
    eprintln!(
        "TILE {rows}x{cols} P{positions}: {:.2} ms, {:.2} GMAC/s hash {h:016x}",
        best * 1e3,
        (rows * cols * positions) as f64 / best / 1e9
    );
}

/// The Q4_0 4x4 prompt tile: block counts around the 4-block fast loop
/// (1..=9, 12, 13, 22, 88), several row-group tails and position tails, whole
/// packed range and offset ranges, against the portable twin.
#[test]
fn q4_0_tile_block_counts_match_portable_bitwise() {
    let (portable, native) = kernels();
    let Some(native) = native else { return };
    let mut rng = Rng(0x51ed270b0b1c3d29);
    for nb in [1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 13, 22, 88] {
        let cols = nb * 32;
        for rows in [4, 13, 16, 17, 33, 64] {
            let bytes = weights(Encoding::Q4_0, rows, cols, &mut rng);
            let w = EncodedRows::new(Encoding::Q4_0, rows, cols, &bytes).unwrap();
            let packed = native.pack(&w).unwrap();
            for positions in [4, 5, 8] {
                let x = activations(positions, cols, &mut rng);
                let input = QuantizedInput::new(Encoding::Q4_0, &x, positions, cols).unwrap();
                let expected = run(&portable, &w, &input);
                for (row0, count) in [(0, rows), (rows / 2, rows - rows / 2)] {
                    let (stride, offset) = (count + 1, 1);
                    let mut out = vec![f32::NAN; positions * stride];
                    native
                        .project_packed(&packed, row0, count, &input, &mut out, stride, offset)
                        .unwrap();
                    for p in 0..positions {
                        for c in 0..count {
                            assert_eq!(
                                out[p * stride + offset + c].to_bits(),
                                expected[p * rows + row0 + c].to_bits(),
                                "nb {nb} rows {rows} P {positions} range {row0}+{count} p {p} c {c}"
                            );
                        }
                    }
                }
            }
        }
    }
}

/// The Q5_K -> Q6_K repack is exact: every Q5_K value is the Q6_K value of
/// the repacked block plus its 32-value offset, bit for bit.
#[test]
fn q5_k_repack_is_exact() {
    let mut rng = Rng(0x005e_ed5c);
    let (rows, cols) = (3, 512);
    let bytes = weights(Encoding::Q5K, rows, cols, &mut rng);
    let (q6, offsets) = split_q5_k(&bytes);
    let want = Encoding::Q5K.decode(&bytes, rows * cols).unwrap();
    let got = Encoding::Q6K.decode(&q6, rows * cols).unwrap();
    for (i, (w, g)) in want.iter().zip(&got).enumerate() {
        assert_eq!(w.to_bits(), (g + offsets[i / 32]).to_bits(), "element {i}");
    }
}

/// Diagnostic: one-thread packed projection throughput by encoding on the
/// expert shapes (run with --ignored --nocapture).
#[test]
#[ignore]
fn packed_encoding_throughput() {
    let Ok(k) = FmaKernels::new(Request::Power9Int8) else {
        return;
    };
    for (encoding, rows, cols) in [
        (Encoding::Q4K, 1024, 2048),
        (Encoding::Q5K, 2048, 512),
        (Encoding::Q6K, 2048, 512),
        (Encoding::Q5K, 512, 2048),
        (Encoding::Q6K, 512, 2048),
        (Encoding::Q5_0, 2048, 512),
        (Encoding::Q8_0, 2048, 512),
        (Encoding::Q5_1, 2688, 1856),
    ] {
        let mut rng = Rng(11);
        let bytes = weights(encoding, rows, cols, &mut rng);
        let w = EncodedRows::new(encoding, rows, cols, &bytes).unwrap();
        let packed = k.pack(&w).unwrap();
        let x = activations(2, cols, &mut rng);
        let input = QuantizedInput::new(encoding, &x, 2, cols).unwrap();
        let mut out = vec![0.0f32; 2 * rows];
        let mut best = f64::MAX;
        for _ in 0..20 {
            let start = std::time::Instant::now();
            k.project_packed(&packed, 0, rows, &input, &mut out, rows, 0)
                .unwrap();
            best = best.min(start.elapsed().as_secs_f64());
        }
        eprintln!(
            "{encoding:?} {rows}x{cols}: {:.1} us, {:.2} weights/ns, packed {:.3} B/weight",
            best * 1e6,
            (rows * cols) as f64 / best / 1e9,
            packed.bytes() as f64 / (rows * cols) as f64
        );
    }
}

/// Diagnostic: the cost of the Q5_K offset correction alone.
#[test]
#[ignore]
fn offset_correction_throughput() {
    let (rows, cols) = (2048, 512);
    let mut rng = Rng(11);
    let bytes = weights(Encoding::Q5K, rows, cols, &mut rng);
    let (_, offsets) = split_offsets(Encoding::Q5K, &bytes, cols).unwrap();
    let x = activations(2, cols, &mut rng);
    let input = QuantizedInput::new(Encoding::Q5K, &x, 2, cols).unwrap();
    let mut out = vec![0.0f32; 2 * rows];
    let mut best = f64::MAX;
    for _ in 0..50 {
        let start = std::time::Instant::now();
        add_offsets(&offsets, rows, 0, rows, &input, &mut out, rows, 0).unwrap();
        best = best.min(start.elapsed().as_secs_f64());
    }
    eprintln!("offsets {rows}x{cols}: {:.1} us", best * 1e6);
}

/// Diagnostic: packed projection throughput with the weights streamed
/// from DRAM (distinct matrices, ~300 MB in all), as in one-token decode.
#[test]
#[ignore]
fn packed_encoding_cold_throughput() {
    let Ok(k) = FmaKernels::new(Request::Power9Int8) else {
        return;
    };
    for (encoding, rows, cols) in [
        (Encoding::Q4K, 512, 2048),
        (Encoding::Q5K, 512, 2048),
        (Encoding::Q5K, 2048, 512),
        (Encoding::Q6K, 2048, 512),
        (Encoding::Q8_0, 2048, 2048),
    ] {
        let mut rng = Rng(11);
        let bytes = weights(encoding, rows, cols, &mut rng);
        let w = EncodedRows::new(encoding, rows, cols, &bytes).unwrap();
        let one = k.pack(&w).unwrap().bytes();
        let copies = (300 << 20) / one;
        let packed: Vec<_> = (0..copies).map(|_| k.pack(&w).unwrap()).collect();
        let x = activations(2, cols, &mut rng);
        let input = QuantizedInput::new(encoding, &x, 2, cols).unwrap();
        let mut out = vec![0.0f32; 2 * rows];
        let mut best = f64::MAX;
        for _ in 0..3 {
            let start = std::time::Instant::now();
            for p in &packed {
                k.project_packed(p, 0, rows, &input, &mut out, rows, 0)
                    .unwrap();
            }
            best = best.min(start.elapsed().as_secs_f64());
        }
        let total = (rows * cols * copies) as f64;
        eprintln!(
            "{encoding:?} {rows}x{cols} cold: {:.2} weights/ns, {:.1} GB/s packed, {:.1} GB/s gguf",
            total / best / 1e9,
            (one * copies) as f64 / best / 1e9,
            bytes.len() as f64 * copies as f64 / best / 1e9
        );
    }
}

/// The hand-written K-quant decode tiles equal the frozen C kernel bit for
/// bit, over partial tiles, unaligned row ranges and untouched output cells.
#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
#[test]
fn k_decode_tiles_match_native() {
    for encoding in [Encoding::Q4K, Encoding::Q6K, Encoding::Q5K] {
        for (rows, cols) in [(2048, 512), (512, 2048), (36, 256), (16, 768), (130, 1024)] {
            let mut rng = Rng(rows as u64 * 7 + cols as u64);
            let bytes = weights(encoding, rows, cols, &mut rng);
            let w = EncodedRows::new(encoding, rows, cols, &bytes).unwrap();
            let packed = pack_power9(&w).unwrap();
            let x = activations(2, cols, &mut rng);
            let input = QuantizedInput::new(encoding, &x, 2, cols).unwrap();
            assert!(k_decode(&packed, &input));
            for (row0, count) in [(0, rows), (3, rows - 5), (17, 1), (rows - 7, 7), (4, 12)] {
                if row0 + count > rows {
                    continue;
                }
                let stride = count + 3;
                let mut native = vec![-7.5f32; 2 * stride];
                let mut tiled = native.clone();
                let a = project_packed_native(&packed, row0, count, &input, &mut native, stride, 2);
                let b = project_k_decode(&packed, row0, count, &input, &mut tiled, stride, 2);
                assert_eq!(a.is_ok(), b.is_ok());
                let same = native
                    .iter()
                    .zip(&tiled)
                    .all(|(a, b)| a.to_bits() == b.to_bits());
                assert!(same, "{encoding:?} {rows}x{cols} rows {row0}+{count}");
            }
        }
    }
}
