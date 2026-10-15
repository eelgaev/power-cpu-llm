use super::super::reference::Matrix;
use super::*;

const ALL: [Encoding; 8] = [
    Encoding::F32,
    Encoding::F16,
    Encoding::TQ2_0,
    Encoding::Q4_0,
    Encoding::Q5_0,
    Encoding::Q8_0,
    Encoding::Q4K,
    Encoding::Q6K,
];

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn byte(&mut self) -> u8 {
        self.next() as u8
    }
    /// Finite f16 bits, including subnormals, both signs, small magnitudes.
    fn half(&mut self) -> [u8; 2] {
        let bits = (self.next() as u16 & 0x8000)
            | ((self.next() % 19) as u16) << 10
            | (self.next() as u16 & 0x3ff);
        bits.to_le_bytes()
    }
    fn unit(&mut self) -> f32 {
        (self.next() % 2001) as f32 / 1000.0 - 1.0
    }
}

/// Random but FINITE encoded rows (scale fields are finite f16).
fn random_rows(encoding: Encoding, rows: usize, cols: usize, rng: &mut Rng) -> Vec<u8> {
    let (per, size) = encoding.block_layout();
    let mut out = Vec::new();
    for _ in 0..rows * cols / per {
        let mut block: Vec<u8> = (0..size).map(|_| rng.byte()).collect();
        match encoding {
            Encoding::F32 => block.copy_from_slice(&(rng.unit() * 4.0).to_le_bytes()),
            Encoding::Q4_0 | Encoding::Q5_0 | Encoding::Q8_0 => {
                block[..2].copy_from_slice(&rng.half())
            }
            Encoding::Q4K => {
                block[..2].copy_from_slice(&rng.half());
                block[2..4].copy_from_slice(&rng.half());
            }
            Encoding::Q6K => block[208..].copy_from_slice(&rng.half()),
            Encoding::F16 => block.copy_from_slice(&rng.half()),
            Encoding::TQ2_0 => block[64..].copy_from_slice(&rng.half()),
        }
        out.extend(block);
    }
    out
}

// ---- independent scalar decoders, written from the ggml block definitions ----

fn f16(bytes: &[u8]) -> f32 {
    let h = u16::from_le_bytes([bytes[0], bytes[1]]) as u32;
    let (sign, exp, man) = (h >> 15, (h >> 10) & 31, h & 1023);
    let magnitude = match exp {
        0 => man as f32 * 2f32.powi(-24),
        31 => f32::INFINITY,
        e => (1.0 + man as f32 / 1024.0) * 2f32.powi(e as i32 - 15),
    };
    if sign == 1 { -magnitude } else { magnitude }
}

fn independent_decode(encoding: Encoding, row: &[u8], cols: usize) -> Vec<f32> {
    let (per, size) = encoding.block_layout();
    let mut y = Vec::with_capacity(cols);
    for b in row.chunks_exact(size).take(cols / per) {
        match encoding {
            Encoding::F32 => y.push(f32::from_le_bytes([b[0], b[1], b[2], b[3]])),
            Encoding::F16 => y.push(f16(b)),
            Encoding::TQ2_0 => {
                // block_tq2_0 { qs[64]; d }: element j*4 + l*32 + m = (q - 1) * d.
                let d = f16(&b[64..]);
                let mut out = [0f32; 256];
                for j in [0usize, 32] {
                    for l in 0..4 {
                        for m in 0..32 {
                            let q = (b[j + m] >> (2 * l)) & 3;
                            out[j * 4 + l * 32 + m] = (q as i32 - 1) as f32 * d;
                        }
                    }
                }
                y.extend(out);
            }
            Encoding::Q8_0 => y.extend(b[2..].iter().map(|&q| f16(b) * q as i8 as f32)),
            Encoding::Q4_0 => {
                for i in 0..32 {
                    let low = if i < 16 {
                        b[2 + i] & 15
                    } else {
                        b[2 + i - 16] >> 4
                    };
                    y.push(f16(b) * (low as i32 - 8) as f32);
                }
            }
            Encoding::Q5_0 => {
                let high = u32::from_le_bytes([b[2], b[3], b[4], b[5]]);
                for i in 0..32 {
                    let low = if i < 16 {
                        b[6 + i] & 15
                    } else {
                        b[6 + i - 16] >> 4
                    };
                    let q = (low as i32 | (((high >> i) & 1) as i32) << 4) - 16;
                    y.push(f16(b) * q as f32);
                }
            }
            Encoding::Q4K => {
                let (d, dmin, s, q) = (f16(b), f16(&b[2..]), &b[4..16], &b[16..]);
                for j in 0..8 {
                    let (sc, m) = if j < 4 {
                        (s[j] & 63, s[j + 4] & 63)
                    } else {
                        (
                            (s[j + 4] & 15) | ((s[j - 4] >> 6) << 4),
                            (s[j + 4] >> 4) | ((s[j] >> 6) << 4),
                        )
                    };
                    let (ds, dm) = (d * sc as f32, dmin * m as f32);
                    for l in 0..32 {
                        let packed = q[(j / 2) * 32 + l];
                        let v = if j % 2 == 0 { packed & 15 } else { packed >> 4 };
                        y.push(ds * v as f32 - dm);
                    }
                }
            }
            Encoding::Q6K => {
                let (ql, qh, sc, d) = (&b[..128], &b[128..192], &b[192..208], f16(&b[208..]));
                let mut out = [0f32; 256];
                for half in 0..2 {
                    for l in 0..32 {
                        let (lo, hi) = (&ql[half * 64..], qh[half * 32 + l]);
                        let q = [
                            (lo[l] & 15) as i32 | ((hi & 3) as i32) << 4,
                            (lo[l + 32] & 15) as i32 | (((hi >> 2) & 3) as i32) << 4,
                            (lo[l] >> 4) as i32 | (((hi >> 4) & 3) as i32) << 4,
                            (lo[l + 32] >> 4) as i32 | (((hi >> 6) & 3) as i32) << 4,
                        ];
                        for (k, q) in q.into_iter().enumerate() {
                            let scale = sc[half * 8 + l / 16 + 2 * k] as i8 as f32;
                            out[half * 128 + k * 32 + l] = (d * scale) * (q - 32) as f32;
                        }
                    }
                }
                y.extend(out);
            }
        }
    }
    y
}

fn shapes(encoding: Encoding) -> [usize; 2] {
    match encoding {
        Encoding::F32 => [37, 96],
        Encoding::Q4_0 | Encoding::Q5_0 | Encoding::Q8_0 => [32, 160],
        Encoding::Q4K | Encoding::Q6K | Encoding::TQ2_0 => [256, 512],
        Encoding::F16 => [37, 96],
    }
}

fn decoded_matrix(encoding: Encoding, bytes: &[u8], rows: usize, cols: usize) -> Vec<f32> {
    let row_bytes = encoding.row_bytes(cols).unwrap();
    (0..rows)
        .flat_map(|r| {
            encoding
                .decode(&bytes[r * row_bytes..(r + 1) * row_bytes], cols)
                .unwrap()
        })
        .collect()
}

#[test]
fn independent_scalar_decoders_agree_with_native_decoder() {
    let mut rng = Rng(7);
    for encoding in ALL {
        for cols in shapes(encoding) {
            let bytes = random_rows(encoding, 3, cols, &mut rng);
            let native = decoded_matrix(encoding, &bytes, 3, cols);
            let row_bytes = encoding.row_bytes(cols).unwrap();
            let ours: Vec<f32> = bytes
                .chunks(row_bytes)
                .flat_map(|r| independent_decode(encoding, r, cols))
                .collect();
            assert_eq!(
                native.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                ours.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                "{encoding:?} {cols}"
            );
        }
    }
}

#[test]
fn batched_projection_is_bit_identical_to_decode_plus_reference_matvec() {
    let mut rng = Rng(11);
    for encoding in ALL {
        for cols in shapes(encoding) {
            let rows = 5;
            // Leading byte: the encoded rows start at an odd (unaligned) address.
            let mut storage = vec![0xA5];
            storage.extend(random_rows(encoding, rows, cols, &mut rng));
            let weights = EncodedRows::new(encoding, rows, cols, &storage[1..]).unwrap();
            let decoded = decoded_matrix(encoding, &storage[1..], rows, cols);
            let matrix = Matrix::new(rows, cols, decoded.clone()).unwrap();
            for positions in [1, 2, 3, 4, 5, 8, 9, 32, 256] {
                let x: Vec<f32> = (0..positions * cols).map(|_| rng.unit()).collect();
                let expected: Vec<f32> = x
                    .chunks(cols)
                    .flat_map(|xp| matrix.project(xp).unwrap())
                    .collect();
                // Independent equation: same f64 sequential dot over independently decoded weights.
                let independent: Vec<f32> = x
                    .chunks(cols)
                    .flat_map(|xp| {
                        decoded
                            .chunks(cols)
                            .map(|w| {
                                w.iter()
                                    .zip(xp)
                                    .fold(0f64, |s, (&w, &v)| s + w as f64 * v as f64)
                                    as f32
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect();
                assert_eq!(expected, independent);
                for tile in [1, 2, rows, rows + 3] {
                    let mut out = vec![f32::NAN; positions * rows];
                    let mut scratch = vec![0.0; tile * cols];
                    project(&weights, &x, positions, &mut out, &mut scratch).unwrap();
                    assert_eq!(
                        out.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                        expected.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                        "{encoding:?} cols={cols} positions={positions} tile={tile}"
                    );
                }
            }
        }
    }
}

#[test]
fn known_packed_blocks_give_hand_computed_sums() {
    // Q8_0: d = 0.5, q[i] = i - 16, x = 1 -> 0.5 * sum(i - 16) = -8.
    let mut q8 = vec![0u8; 34];
    q8[..2].copy_from_slice(&0x3800u16.to_le_bytes());
    for i in 0..32 {
        q8[2 + i] = (i as i8 - 16) as u8;
    }
    // Q4_K (same packing as encoded.rs's test): d = 1, dmin = 0.5, 6-bit
    // scales/mins give groups alternating 2.0 / 6.0; x = 1 -> 4*32*(2+6).
    let mut q4 = vec![0u8; 144];
    q4[..2].copy_from_slice(&0x3c00u16.to_le_bytes());
    q4[2..4].copy_from_slice(&0x3800u16.to_le_bytes());
    q4[4..8].fill(1);
    q4[8..12].fill(2);
    q4[12..16].fill(0x21);
    q4[16..].fill(0x73);
    for (encoding, bytes, cols, expected) in [
        (Encoding::Q8_0, q8, 32, -8.0),
        (Encoding::Q4K, q4, 256, 4.0 * 32.0 * (2.0 + 6.0)),
    ] {
        let weights = EncodedRows::new(encoding, 1, cols, &bytes).unwrap();
        let mut out = [0.0; 2];
        let x: Vec<f32> = [vec![1.0; cols], vec![-2.0; cols]].concat();
        project(&weights, &x, 2, &mut out, &mut vec![0.0; cols]).unwrap();
        assert_eq!(out, [expected, -2.0 * expected], "{encoding:?}");
    }
}

#[test]
fn transposed_accumulation_persists_across_tiles_bit_exactly() {
    let mut rng = Rng(23);
    for encoding in ALL {
        let cols = shapes(encoding)[1];
        let total_rows = 7;
        let bytes = random_rows(encoding, total_rows, cols, &mut rng);
        let all = EncodedRows::new(encoding, total_rows, cols, &bytes).unwrap();
        let matrix = Matrix::new(
            total_rows,
            cols,
            decoded_matrix(encoding, &bytes, total_rows, cols),
        )
        .unwrap();
        for positions in [1, 2, 8, 33] {
            let coefficients: Vec<f32> = (0..positions * total_rows).map(|_| rng.unit()).collect();
            let start: Vec<f64> = (0..positions * cols)
                .map(|i| (i % 5) as f64 * 0.25)
                .collect();
            // Reference: one full-matrix pass per position.
            let mut expected = start.clone();
            for p in 0..positions {
                matrix
                    .accumulate_transposed(
                        &coefficients[p * total_rows..(p + 1) * total_rows],
                        &mut expected[p * cols..(p + 1) * cols],
                    )
                    .unwrap();
            }
            // Single call, big scratch.
            let mut single = start.clone();
            accumulate_transposed(
                &all,
                0,
                &coefficients,
                total_rows,
                positions,
                &mut single,
                &mut vec![0.0; total_rows * cols],
            )
            .unwrap();
            // Caller tiles [0,3) + [3,7), each internally split into 2-row scratch tiles.
            let mut tiled = start.clone();
            for (first, count) in [(0, 3), (3, 4)] {
                accumulate_transposed(
                    &all.slice(first, count).unwrap(),
                    first,
                    &coefficients,
                    total_rows,
                    positions,
                    &mut tiled,
                    &mut vec![0.0; 2 * cols],
                )
                .unwrap();
            }
            let bits = |v: &[f64]| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
            assert_eq!(bits(&single), bits(&expected), "{encoding:?} {positions}");
            assert_eq!(bits(&tiled), bits(&expected), "{encoding:?} {positions}");
            let mut out = vec![0.0; positions * cols];
            finish(&tiled, &mut out).unwrap();
            assert_eq!(out, expected.iter().map(|&v| v as f32).collect::<Vec<_>>());
        }
    }
}

#[test]
fn invalid_shapes_lengths_and_scratch_are_rejected() {
    let bytes = vec![0u8; 2 * 34];
    assert!(matches!(
        EncodedRows::new(Encoding::Q8_0, 2, 32, &bytes[1..]),
        Err(Error::Encoding { .. })
    ));
    assert!(matches!(
        EncodedRows::new(Encoding::Q8_0, 2, 48, &bytes),
        Err(Error::Encoding { expected: None, .. })
    ));
    assert!(matches!(
        EncodedRows::new(Encoding::Q8_0, 0, 32, &[]),
        Err(Error::Shape(_))
    ));
    assert!(matches!(
        EncodedRows::new(Encoding::Q4K, usize::MAX / 8, 256, &[]),
        Err(Error::Overflow)
    ));
    let w = EncodedRows::new(Encoding::Q8_0, 2, 32, &bytes).unwrap();
    assert!(w.slice(1, 2).is_err() && w.slice(usize::MAX, 2).is_err() && w.slice(0, 0).is_err());
    let x = vec![0.5; 64];
    let mut out = vec![0.0; 4];
    assert!(matches!(
        project(&w, &x, 2, &mut out, &mut [0.0; 31]),
        Err(Error::ScratchTooSmall { needed: 32, .. })
    ));
    assert!(matches!(
        project(&w, &x[..63], 2, &mut out, &mut [0.0; 64]),
        Err(Error::Shape(_))
    ));
    assert!(matches!(
        project(&w, &x, 2, &mut out[..3], &mut [0.0; 64]),
        Err(Error::Shape(_))
    ));
    assert!(matches!(
        project(&w, &x, 0, &mut [], &mut [0.0; 64]),
        Err(Error::Shape(_))
    ));
    let mut nan = x.clone();
    nan[7] = f32::NAN;
    assert!(matches!(
        project(&w, &nan, 2, &mut out, &mut [0.0; 64]),
        Err(Error::NonFiniteInput)
    ));
    let mut acc = vec![0.0; 64];
    let coefficients = vec![1.0; 4];
    assert!(matches!(
        accumulate_transposed(&w, 1, &coefficients, 2, 2, &mut acc, &mut [0.0; 64]),
        Err(Error::Shape(_))
    ));
    assert!(matches!(
        accumulate_transposed(&w, 0, &coefficients[..3], 2, 2, &mut acc, &mut [0.0; 64]),
        Err(Error::Shape(_))
    ));
    acc[3] = f64::INFINITY;
    assert!(matches!(
        accumulate_transposed(&w, 0, &coefficients, 2, 2, &mut acc, &mut [0.0; 64]),
        Err(Error::NonFiniteInput)
    ));
    assert!(matches!(
        finish(&[1e300], &mut [0.0]),
        Err(Error::NonFiniteOutput)
    ));
}

#[test]
fn nonfinite_weights_and_outputs_are_reported_and_accumulator_is_untouched() {
    // Q8_0 block with an infinite f16 scale.
    let mut bytes = vec![1u8; 2 * 34];
    bytes[34..36].copy_from_slice(&0x7c00u16.to_le_bytes());
    let w = EncodedRows::new(Encoding::Q8_0, 2, 32, &bytes).unwrap();
    let mut out = vec![0.0; 2];
    assert_eq!(
        project(&w, &[1.0; 32], 1, &mut out, &mut [0.0; 64]),
        Err(Error::NonFiniteWeight)
    );
    let mut acc = vec![0.5; 32];
    assert_eq!(
        accumulate_transposed(&w, 0, &[1.0, 1.0], 2, 1, &mut acc, &mut [0.0; 64]),
        Err(Error::NonFiniteWeight)
    );
    assert_eq!(acc, vec![0.5; 32]);
    // Finite F32 weights whose dot overflows F32.
    let big: Vec<u8> = [3e38f32; 2].iter().flat_map(|v| v.to_le_bytes()).collect();
    let w = EncodedRows::new(Encoding::F32, 1, 2, &big).unwrap();
    assert_eq!(
        project(&w, &[10.0, 10.0], 1, &mut [0.0], &mut [0.0; 2]),
        Err(Error::NonFiniteOutput)
    );
}

#[test]
fn native_entry_points_check_arguments_before_any_work() {
    let bytes = [0u8; 34];
    let x = [1.0f32; 32];
    let (mut out, mut scratch, mut acc) = ([7.0f32; 4], [0.0f32; 64], [0.0f64; 32]);
    let call = |kind: c_int,
                rows: usize,
                cols: usize,
                weight_bytes: usize,
                stride: usize,
                offset: usize,
                out: &mut [f32],
                scratch: &mut [f32]| unsafe {
        pq_project_rows(
            kind,
            bytes.as_ptr(),
            weight_bytes,
            rows,
            cols,
            x.as_ptr(),
            1,
            out.as_mut_ptr(),
            stride,
            offset,
            scratch.as_mut_ptr(),
            scratch.len(),
        )
    };
    assert_eq!(call(3, 1, 32, 34, 4, 0, &mut out, &mut scratch), 2); // Q4_1 unsupported
    assert_eq!(call(8, 1, 48, 34, 4, 0, &mut out, &mut scratch), 3); // not a block multiple
    assert_eq!(call(8, 1, 32, 33, 4, 0, &mut out, &mut scratch), 3); // byte count
    assert_eq!(
        call(
            8,
            usize::MAX / 2,
            32,
            34,
            usize::MAX,
            0,
            &mut out,
            &mut scratch
        ),
        4
    ); // overflow
    assert_eq!(call(8, 1, 32, 34, 4, 4, &mut out, &mut scratch), 1); // offset + rows > stride
    assert_eq!(call(8, 1, 32, 34, 4, usize::MAX, &mut out, &mut scratch), 4);
    assert_eq!(call(8, 1, 32, 34, 4, 0, &mut out, &mut scratch[..31]), 5); // scratch
    assert_eq!(call(8, 0, 32, 0, 4, 0, &mut out, &mut scratch), 1);
    assert_eq!(out, [7.0; 4], "no writes on argument errors");
    let null = unsafe {
        pq_project_rows(
            8,
            std::ptr::null(),
            34,
            1,
            32,
            x.as_ptr(),
            1,
            out.as_mut_ptr(),
            4,
            0,
            scratch.as_mut_ptr(),
            64,
        )
    };
    assert_eq!(null, 1);
    let coeff = [1.0f32; 2];
    let transposed = unsafe {
        pq_accumulate_transposed_rows(
            8,
            bytes.as_ptr(),
            34,
            1,
            32,
            coeff.as_ptr(),
            1,
            2,
            2,
            acc.as_mut_ptr(),
            scratch.as_mut_ptr(),
            64,
        )
    };
    assert_eq!(transposed, 1); // offset + rows > coefficient stride
    assert_eq!(acc, [0.0; 32]);
}

macro_rules! real_case {
    ($name:literal) => {
        (
            include_bytes!(concat!("../../../fixtures/quantized/", $name, ".rows")).as_slice(),
            include_bytes!(concat!("../../../fixtures/quantized/", $name, ".x.f32")).as_slice(),
            include_bytes!(concat!(
                "../../../fixtures/quantized/",
                $name,
                ".project.f32"
            ))
            .as_slice(),
            include_bytes!(concat!("../../../fixtures/quantized/", $name, ".coeff.f32")).as_slice(),
            include_bytes!(concat!(
                "../../../fixtures/quantized/",
                $name,
                ".transposed.f64"
            ))
            .as_slice(),
        )
    };
}

fn f32s(bytes: &[u8]) -> Vec<f32> {
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|&b| f32::from_le_bytes(b))
        .collect()
}

/// Real row tiles of every stored type from the target GGUF, with exact
/// (correctly rounded) expected sums computed independently in Python
/// (scripts/bench-quantized-matrix/extract_real_rows.py).
#[test]
fn real_target_row_tiles_match_exact_sums() {
    let cases = include_str!("../../../fixtures/quantized/cases.txt");
    let mut checked = 0;
    for line in cases.lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<&str> = line.split_whitespace().collect();
        let (name, id, rows, cols, positions): (&str, i32, usize, usize, usize) = (
            f[0],
            f[1].parse().unwrap(),
            f[2].parse().unwrap(),
            f[3].parse().unwrap(),
            f[4].parse().unwrap(),
        );
        let (raw, x, project_expected, coeff, transposed_expected) = match name {
            "q4k_attn_q" => real_case!("q4k_attn_q"),
            "q6k_attn_v" => real_case!("q6k_attn_v"),
            "q8_0_ffn_down" => real_case!("q8_0_ffn_down"),
            "q5_0_self_cond_down" => real_case!("q5_0_self_cond_down"),
            "f32_router" => real_case!("f32_router"),
            "q6k_token_embd" => real_case!("q6k_token_embd"),
            other => panic!("unknown fixture {other}"),
        };
        let encoding = Encoding::try_from(id).unwrap();
        let weights = EncodedRows::new(encoding, rows, cols, raw).unwrap();
        let decoded = decoded_matrix(encoding, raw, rows, cols);
        let (x, coeff) = (f32s(x), f32s(coeff));
        // Normal projection, 5-row scratch tiles: within 1 f32 ulp of the exact
        // sum, and bit-identical to the existing decode + reference path.
        let mut out = vec![0.0; positions * rows];
        project(&weights, &x, positions, &mut out, &mut vec![0.0; 5 * cols]).unwrap();
        let matrix = Matrix::new(rows, cols, decoded.clone()).unwrap();
        let reference: Vec<f32> = x
            .chunks(cols)
            .flat_map(|xp| matrix.project(xp).unwrap())
            .collect();
        assert_eq!(out, reference, "{name}");
        for (got, want) in out.iter().zip(f32s(project_expected)) {
            assert!(
                (got.to_bits() as i64 - want.to_bits() as i64).abs() <= 1,
                "{name}: {got} vs exact {want}"
            );
        }
        // Transposed: caller tiles [0, 7) + [7, rows) with 3-row scratch, vs exact
        // sums within the rigorous sequential-summation bound rows * eps * sum|terms|.
        let mut acc = vec![0.0f64; positions * cols];
        let split = 7.min(rows - 1);
        for (first, count) in [(0, split), (split, rows - split)] {
            accumulate_transposed(
                &weights.slice(first, count).unwrap(),
                first,
                &coeff,
                rows,
                positions,
                &mut acc,
                &mut vec![0.0; 3 * cols],
            )
            .unwrap();
        }
        let exact: Vec<f64> = transposed_expected
            .as_chunks::<8>()
            .0
            .iter()
            .map(|&b| f64::from_le_bytes(b))
            .collect();
        for p in 0..positions {
            for c in 0..cols {
                let magnitude: f64 = (0..rows)
                    .map(|r| (coeff[p * rows + r] as f64 * decoded[r * cols + c] as f64).abs())
                    .sum();
                let bound = 2.0 * rows as f64 * f64::EPSILON * magnitude;
                let (got, want) = (acc[p * cols + c], exact[p * cols + c]);
                assert!(
                    (got - want).abs() <= bound,
                    "{name} [{p},{c}]: {got} vs {want} (bound {bound})"
                );
            }
        }
        checked += 1;
    }
    assert_eq!(checked, 6);
}
