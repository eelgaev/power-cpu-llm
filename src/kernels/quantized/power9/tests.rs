use super::super::Encoding;
use super::*;
use std::ffi::c_int;

const ALL: [Encoding; 5] = [
    Encoding::F32,
    Encoding::Q4_0,
    Encoding::Q5_0,
    Encoding::Q8_0,
    Encoding::Q4K,
    Encoding::Q6K,
];

/// Portable pair, and a pair with BOTH native kernels (the dispatched
/// `power9` pair keeps the portable transposed kernel; its native version is
/// still verified here).
fn kernels() -> Option<(MatrixKernels, MatrixKernels)> {
    #[cfg(all(feature = "power9", target_arch = "powerpc64"))]
    if let Ok(native) = MatrixKernels::new(Request::Power9) {
        let portable = MatrixKernels::new(Request::Portable).unwrap();
        return Some((
            portable,
            MatrixKernels {
                transposed: pq_accumulate_transposed_rows_power9,
                ..native
            },
        ));
    }
    assert_eq!(
        MatrixKernels::new(Request::Power9).unwrap_err(),
        Unavailable
    );
    eprintln!("POWER9 kernels unavailable in this build/CPU: native comparisons skipped");
    None
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn half(&mut self) -> [u8; 2] {
        let bits = (self.next() as u16 & 0x8000)
            | (((self.next() % 19) as u16) << 10)
            | (self.next() as u16 & 0x3ff);
        bits.to_le_bytes()
    }
    fn unit(&mut self) -> f32 {
        (self.next() % 2001) as f32 / 1000.0 - 1.0
    }
}

fn random_rows(encoding: Encoding, rows: usize, cols: usize, rng: &mut Rng) -> Vec<u8> {
    let (per, size) = encoding.block_layout();
    let mut out = Vec::new();
    for _ in 0..rows * cols / per {
        let mut block: Vec<u8> = (0..size).map(|_| rng.next() as u8).collect();
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
        }
        out.extend(block);
    }
    out
}

/// (rows, cols): odd F32 shapes, row counts crossing the 4-row and 16-row
/// groups, and column counts crossing the 256-column block.
fn shapes(encoding: Encoding) -> Vec<(usize, usize)> {
    match encoding {
        Encoding::F32 => vec![(1, 1), (7, 37), (21, 301), (17, 33)],
        Encoding::Q4_0 | Encoding::Q5_0 | Encoding::Q8_0 => vec![(5, 32), (21, 288), (3, 2112)],
        Encoding::Q4K | Encoding::Q6K => vec![(5, 256), (21, 512), (2, 2816)],
    }
}

fn bits32(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}
fn bits64(v: &[f64]) -> Vec<u64> {
    v.iter().map(|x| x.to_bits()).collect()
}

#[test]
fn selection_is_checked_and_never_equates_build_with_cpu() {
    let portable = MatrixKernels::new(Request::Portable).unwrap();
    assert_eq!(portable.name(), "portable");
    let detected = super::super::super::Kernels::new(super::super::super::Backend::Power9).is_ok();
    let compiled = cfg!(all(feature = "power9", target_arch = "powerpc64"));
    match MatrixKernels::new(Request::Power9) {
        Ok(native) => {
            assert!(compiled && detected);
            assert_eq!(native.name(), "power9");
            assert!(std::ptr::fn_addr_eq(
                native.transposed,
                pq_accumulate_transposed_rows as TransposedRows
            ));
        }
        Err(e) => {
            assert!(!(compiled && detected));
            assert_eq!(e, Unavailable);
            assert!(!e.to_string().is_empty());
        }
    }
    let auto = MatrixKernels::new(Request::Auto).unwrap();
    assert_eq!(
        auto.name(),
        if compiled && detected {
            "power9"
        } else {
            "portable"
        }
    );
}

#[test]
fn native_projection_is_bit_identical_to_portable() {
    let Some((portable, native)) = kernels() else {
        return;
    };
    let mut rng = Rng(101);
    for encoding in ALL {
        for (rows, cols) in shapes(encoding) {
            // Odd leading pads: unaligned encoded bytes, and f32 buffers that
            // are not 16-byte aligned.
            let mut storage = vec![0x5A];
            storage.extend(random_rows(encoding, rows, cols, &mut rng));
            let weights = EncodedRows::new(encoding, rows, cols, &storage[1..]).unwrap();
            for positions in [1, 2, 3, 4, 5, 7, 8, 9, 32, 256] {
                let mut x = vec![0.0f32; positions * cols + 1];
                x.iter_mut().for_each(|v| *v = rng.unit());
                let x = &x[1..];
                for tile in [1, 3, rows] {
                    let mut scratch = vec![0.0f32; tile * cols + 1];
                    let mut want = vec![f32::NAN; positions * rows + 1];
                    let mut got = vec![f32::NAN; positions * rows + 1];
                    portable
                        .project(&weights, x, positions, &mut want[1..], &mut scratch[1..])
                        .unwrap();
                    native
                        .project(&weights, x, positions, &mut got[1..], &mut scratch[1..])
                        .unwrap();
                    assert_eq!(
                        bits32(&got),
                        bits32(&want),
                        "{encoding:?} rows={rows} cols={cols} positions={positions} tile={tile}"
                    );
                }
            }
        }
    }
}

#[test]
fn native_transposed_accumulation_is_bit_identical_across_tiles() {
    let Some((portable, native)) = kernels() else {
        return;
    };
    let mut rng = Rng(202);
    for encoding in ALL {
        for (rows, cols) in shapes(encoding) {
            let bytes = random_rows(encoding, rows, cols, &mut rng);
            let all = EncodedRows::new(encoding, rows, cols, &bytes).unwrap();
            for positions in [1, 2, 3, 5, 8, 32] {
                let coefficients: Vec<f32> = (0..positions * rows).map(|_| rng.unit()).collect();
                let start: Vec<f64> = (0..positions * cols + 1)
                    .map(|i| (i % 7) as f64 * 0.125 - 0.3)
                    .collect();
                let mut want = start.clone();
                portable
                    .accumulate_transposed(
                        &all,
                        0,
                        &coefficients,
                        rows,
                        positions,
                        &mut want[1..],
                        &mut vec![0.0; rows * cols],
                    )
                    .unwrap();
                // Caller tiles split at rows/2, each internally split into 3-row
                // scratch tiles; accumulator starts 8 bytes past alignment.
                let mut got = start.clone();
                let split = (rows / 2).max(1).min(rows);
                let mut tiles = vec![(0, split)];
                if split < rows {
                    tiles.push((split, rows - split));
                }
                for (first, count) in tiles {
                    native
                        .accumulate_transposed(
                            &all.slice(first, count).unwrap(),
                            first,
                            &coefficients,
                            rows,
                            positions,
                            &mut got[1..],
                            &mut vec![0.0; 3 * cols + 1][1..],
                        )
                        .unwrap();
                }
                assert_eq!(
                    bits64(&got),
                    bits64(&want),
                    "{encoding:?} rows={rows} cols={cols} positions={positions}"
                );
            }
        }
    }
}

macro_rules! real_case {
    ($name:literal) => {
        (
            include_bytes!(concat!("../../../../fixtures/quantized/", $name, ".rows")).as_slice(),
            include_bytes!(concat!("../../../../fixtures/quantized/", $name, ".x.f32")).as_slice(),
            include_bytes!(concat!("../../../../fixtures/quantized/", $name, ".coeff.f32")).as_slice(),
        )
    };
}

/// Existing real target row fixtures (data only, expected values untouched):
/// native output must equal portable output bit for bit; the portable path is
/// separately checked against the exact sums in `quantized::tests`.
#[test]
fn native_matches_portable_on_real_target_rows() {
    let Some((portable, native)) = kernels() else {
        return;
    };
    let cases = include_str!("../../../../fixtures/quantized/cases.txt");
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
        let (raw, x, coeff) = match name {
            "q4k_attn_q" => real_case!("q4k_attn_q"),
            "q6k_attn_v" => real_case!("q6k_attn_v"),
            "q8_0_ffn_down" => real_case!("q8_0_ffn_down"),
            "q5_0_self_cond_down" => real_case!("q5_0_self_cond_down"),
            "f32_router" => real_case!("f32_router"),
            "q6k_token_embd" => real_case!("q6k_token_embd"),
            other => panic!("unknown fixture {other}"),
        };
        let f32s = |b: &[u8]| {
            b.chunks_exact(4)
                .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
                .collect::<Vec<_>>()
        };
        let (x, coeff) = (f32s(x), f32s(coeff));
        let weights = EncodedRows::new(Encoding::try_from(id).unwrap(), rows, cols, raw).unwrap();
        // Replicate the 3 stored positions to 9 to exercise the 4-position kernel and the tail.
        let x9: Vec<f32> = x.iter().cycle().take(9 * cols).copied().collect();
        let (mut want, mut got) = (vec![0.0; 9 * rows], vec![0.0; 9 * rows]);
        portable
            .project(&weights, &x9, 9, &mut want, &mut vec![0.0; 5 * cols])
            .unwrap();
        native
            .project(&weights, &x9, 9, &mut got, &mut vec![0.0; 5 * cols])
            .unwrap();
        assert_eq!(bits32(&got), bits32(&want), "{name}");
        let (mut want, mut got) = (
            vec![0.0f64; positions * cols],
            vec![0.0f64; positions * cols],
        );
        portable
            .accumulate_transposed(
                &weights,
                0,
                &coeff,
                rows,
                positions,
                &mut want,
                &mut vec![0.0; 3 * cols],
            )
            .unwrap();
        native
            .accumulate_transposed(
                &weights,
                0,
                &coeff,
                rows,
                positions,
                &mut got,
                &mut vec![0.0; 3 * cols],
            )
            .unwrap();
        assert_eq!(bits64(&got), bits64(&want), "{name}");
        checked += 1;
    }
    assert_eq!(checked, 6);
}

#[test]
fn native_errors_and_mutation_contracts_match_portable() {
    let Some((portable, native)) = kernels() else {
        return;
    };
    // Infinite f16 scale in the second Q8_0 row.
    let mut bytes = vec![1u8; 2 * 34];
    bytes[34..36].copy_from_slice(&0x7c00u16.to_le_bytes());
    let w = EncodedRows::new(Encoding::Q8_0, 2, 32, &bytes).unwrap();
    for k in [portable, native] {
        assert_eq!(
            k.project(&w, &[1.0; 128], 4, &mut [0.0; 8], &mut [0.0; 64]),
            Err(Error::NonFiniteWeight)
        );
        let mut acc = vec![0.5; 4 * 32];
        assert_eq!(
            k.accumulate_transposed(&w, 0, &[1.0; 8], 2, 4, &mut acc, &mut [0.0; 64]),
            Err(Error::NonFiniteWeight)
        );
        assert_eq!(
            acc,
            vec![0.5; 4 * 32],
            "{}: accumulator untouched",
            k.name()
        );
    }
    let big: Vec<u8> = [3e38f32; 2].iter().flat_map(|v| v.to_le_bytes()).collect();
    let w = EncodedRows::new(Encoding::F32, 1, 2, &big).unwrap();
    for k in [portable, native] {
        assert_eq!(
            k.project(&w, &[10.0; 8], 4, &mut [0.0; 4], &mut [0.0; 2]),
            Err(Error::NonFiniteOutput)
        );
    }
    // Shared Rust validation is unchanged for both.
    let w = EncodedRows::new(Encoding::Q8_0, 1, 32, &bytes[..34]).unwrap();
    for k in [portable, native] {
        assert!(matches!(
            k.project(&w, &[f32::NAN; 32], 1, &mut [0.0], &mut [0.0; 32]),
            Err(Error::NonFiniteInput)
        ));
        assert!(matches!(
            k.project(&w, &[1.0; 32], 1, &mut [0.0], &mut [0.0; 31]),
            Err(Error::ScratchTooSmall { .. })
        ));
    }
}

type Project = unsafe extern "C" fn(
    c_int,
    *const u8,
    usize,
    usize,
    usize,
    *const f32,
    usize,
    *mut f32,
    usize,
    usize,
    *mut f32,
    usize,
) -> c_int;

#[test]
fn raw_native_entry_points_return_portable_statuses_before_any_write() {
    let Some((portable, native)) = kernels() else {
        return;
    };
    let bytes = [0u8; 34];
    let x = [1.0f32; 32];
    let coeff = [1.0f32; 2];
    // (type, rows, cols, weight_bytes, out_stride, out_offset, scratch_len, null weights)
    type Case = (c_int, usize, usize, usize, usize, usize, usize, bool);
    let cases: [Case; 9] = [
        (3, 1, 32, 34, 4, 0, 64, false),
        (8, 1, 48, 34, 4, 0, 64, false),
        (8, 1, 32, 33, 4, 0, 64, false),
        (8, usize::MAX / 2, 32, 34, usize::MAX, 0, 64, false),
        (8, 1, 32, 34, 4, 4, 64, false),
        (8, 1, 32, 34, 4, usize::MAX, 64, false),
        (8, 1, 32, 34, 4, 0, 31, false),
        (8, 0, 32, 0, 4, 0, 64, false),
        (8, 1, 32, 34, 4, 0, 64, true),
    ];
    let (pp, np): (Project, Project) = (portable.project, native.project);
    for (i, &(kind, rows, cols, weight_bytes, stride, offset, scratch_len, null)) in
        cases.iter().enumerate()
    {
        let mut statuses = Vec::new();
        for f in [pp, np] {
            let (mut out, mut scratch) = ([7.0f32; 4], [0.0f32; 64]);
            let w = if null {
                std::ptr::null()
            } else {
                bytes.as_ptr()
            };
            // SAFETY: deliberately invalid arguments that both kernels reject before
            // touching memory; all non-null pointers are valid for the claimed sizes
            // or are never dereferenced past the checks.
            let status = unsafe {
                f(
                    kind,
                    w,
                    weight_bytes,
                    rows,
                    cols,
                    x.as_ptr(),
                    1,
                    out.as_mut_ptr(),
                    stride,
                    offset,
                    scratch.as_mut_ptr(),
                    scratch_len,
                )
            };
            assert_eq!(out, [7.0; 4], "case {i}: no writes");
            statuses.push(status);
        }
        assert_ne!(statuses[0], 0, "case {i}");
        assert_eq!(statuses[0], statuses[1], "case {i}");
    }
    for k in [portable, native] {
        let mut acc = [0.0f64; 32];
        let mut scratch = [0.0f32; 64];
        // SAFETY: valid buffers; coefficient window offset + rows > stride is rejected before any access.
        let status = unsafe {
            (k.transposed)(
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
        assert_eq!(status, 1);
        assert_eq!(acc, [0.0; 32]);
    }
}
