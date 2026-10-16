use super::super::Encoding;
use super::*;
use std::ffi::c_int;
use std::hint::black_box;

const ALL: [Encoding; 9] = [
    Encoding::F32,
    Encoding::BF16,
    Encoding::Q4_0,
    Encoding::Q5_0,
    Encoding::Q5_1,
    Encoding::Q8_0,
    Encoding::Q4K,
    Encoding::Q5K,
    Encoding::Q6K,
];

/// (portable, FMA) pairs, or None (with a message) when the FMA kernels are
/// not compiled in or the CPU lacks POWER9.
fn kernels() -> Option<(FmaKernels, FmaKernels)> {
    if let Ok(native) = FmaKernels::new(Request::Power9Fma) {
        return Some((FmaKernels::new(Request::Portable).unwrap(), native));
    }
    eprintln!("POWER9 FMA kernels unavailable in this build/CPU: native comparisons skipped");
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
    /// Any FINITE binary32 bit pattern (normals, subnormals, +-0, +-MAX).
    fn finite_f32(&mut self) -> f32 {
        loop {
            let v = f32::from_bits(self.next() as u32);
            if v.is_finite() {
                return v;
            }
        }
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
            Encoding::Q4K | Encoding::Q5K | Encoding::Q5_1 => {
                block[..2].copy_from_slice(&rng.half());
                block[2..4].copy_from_slice(&rng.half());
            }
            Encoding::Q6K => block[208..].copy_from_slice(&rng.half()),
            Encoding::F16 => block.copy_from_slice(&rng.half()),
            Encoding::BF16 => block.copy_from_slice(&(rng.unit() * 4.0).to_le_bytes()[2..]),
            Encoding::TQ2_0 => block[64..].copy_from_slice(&rng.half()),
        }
        out.extend(block);
    }
    out
}

fn shapes(encoding: Encoding) -> Vec<(usize, usize)> {
    match encoding {
        Encoding::F32 | Encoding::F16 | Encoding::BF16 => {
            vec![(1, 1), (7, 37), (21, 301), (17, 33)]
        }
        Encoding::Q4_0 | Encoding::Q5_0 | Encoding::Q5_1 | Encoding::Q8_0 => {
            vec![(5, 32), (21, 288), (3, 2112)]
        }
        Encoding::Q4K | Encoding::Q5K | Encoding::Q6K | Encoding::TQ2_0 => {
            vec![(5, 256), (21, 512), (2, 2816)]
        }
    }
}

fn bits32(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}
fn bits64(v: &[f64]) -> Vec<u64> {
    v.iter().map(|x| x.to_bits()).collect()
}
fn f32_rows(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

// ---- IEEE rounding modes (glibc on powerpc64: FE_TONEAREST..FE_DOWNWARD = 0..3) ----

unsafe extern "C" {
    fn fegetround() -> c_int;
    fn fesetround(mode: c_int) -> c_int;
}
const ROUNDING_MODES: [(c_int, &str); 4] = [
    (0, "nearest"),
    (1, "toward zero"),
    (2, "upward"),
    (3, "downward"),
];

/// Runs `f` under each rounding mode on this thread, always restoring the
/// original mode (also on panic) before returning.
fn under_each_rounding_mode(mut f: impl FnMut(&str)) {
    struct Restore(c_int);
    impl Drop for Restore {
        fn drop(&mut self) {
            // SAFETY: restores this thread's previously active, valid mode.
            unsafe { fesetround(self.0) };
        }
    }
    // SAFETY: reading the current thread's rounding mode.
    let _restore = Restore(unsafe { fegetround() });
    for (mode, name) in ROUNDING_MODES {
        // SAFETY: valid glibc rounding-mode constant for this target; thread-local.
        assert_eq!(
            unsafe { fesetround(mode) },
            0,
            "cannot set rounding mode {name}"
        );
        assert_eq!(unsafe { fegetround() }, mode);
        f(name);
    }
}

/// The finite binary32 patterns the proof must survive.
fn special_f32() -> Vec<f32> {
    let mut v = vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        f32::MAX,
        -f32::MAX,
        f32::MIN_POSITIVE,
        -f32::MIN_POSITIVE,
        f32::from_bits(1),
        -f32::from_bits(1),
        f32::from_bits(0x007f_ffff),
        -f32::from_bits(0x007f_ffff),
        1.0 + f32::EPSILON,
        1.0 - f32::EPSILON / 2.0,
        3.0,
        1.0 / 3.0,
        2f32.powi(-27),
        2f32.powi(-26),
        2f32.powi(64),
        -2f32.powi(-100),
        16_777_215.0,
        -16_777_217.0,
    ];
    let mut rng = Rng(0x5eed);
    v.extend((0..42).map(|_| rng.finite_f32()));
    v
}

#[test]
fn scalar_fma_identity_holds_for_binary32_operands_in_every_rounding_mode() {
    // The proof obligation itself: fma(a, b, s) == s + a * b bit for bit when
    // a and b are binary32 values widened to binary64, for finite s.
    let specials = special_f32();
    let mut rng = Rng(0xfaceb00c);
    let accumulators: Vec<f64> = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        f64::MAX / 4.0,
        -f64::MAX / 4.0,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        -f64::from_bits(1),
        1.0 + f64::EPSILON,
        2f64.powi(-53),
        1e300,
        -1e-300,
    ]
    .into_iter()
    .chain((0..64).map(|_| {
        loop {
            let v = f64::from_bits(rng.next());
            if v.is_finite() {
                break v;
            }
        }
    }))
    .collect();
    let mut checked = 0u64;
    under_each_rounding_mode(|mode| {
        let mut check = |a: f32, b: f32, s: f64| {
            let (a, b, s) = (black_box(a as f64), black_box(b as f64), black_box(s));
            let separate = s + a * b;
            let fused = a.mul_add(b, s);
            assert_eq!(
                fused.to_bits(),
                separate.to_bits(),
                "{mode}: a={a:e} b={b:e} s={s:e}"
            );
            // The premise: the binary64 product of two binary32 values is exact,
            // i.e. its rounding error fma(a, b, -(a*b)) is zero.
            let product = black_box(a * b);
            assert_eq!(
                a.mul_add(b, -product),
                0.0,
                "{mode}: inexact product a={a:e} b={b:e}"
            );
            checked += 1;
        };
        for &a in &specials {
            for &b in &specials {
                for &s in &accumulators {
                    check(a, b, s);
                }
            }
        }
        for _ in 0..1_000_000 {
            let (a, b) = (rng.finite_f32(), rng.finite_f32());
            check(
                a,
                b,
                accumulators[(rng.next() % accumulators.len() as u64) as usize],
            );
            // Accumulators near the product itself: cancellation and rounding boundaries.
            let p = a as f64 * b as f64;
            check(a, b, -p);
            check(a, b, -p * (1.0 + f64::EPSILON));
            check(a, b, f64::from_bits(p.to_bits() ^ 1));
        }
    });
    assert!(checked > 4 * 3_000_000);
}

#[test]
fn selection_is_checked_and_never_equates_build_with_cpu() {
    assert_eq!(
        FmaKernels::new(Request::Portable).unwrap().name(),
        "portable"
    );
    let detected = super::super::super::Kernels::new(super::super::super::Backend::Power9).is_ok();
    let compiled = cfg!(all(feature = "power9", target_arch = "powerpc64"));
    match FmaKernels::new(Request::Power9Fma) {
        Ok(native) => {
            assert!(compiled && detected);
            assert_eq!(native.name(), "power9-fma");
        }
        Err(e) => {
            assert!(!(compiled && detected));
            assert_eq!(e, Unavailable);
            assert!(!e.to_string().is_empty());
        }
    }
    let auto = FmaKernels::new(Request::Auto).unwrap();
    assert_eq!(
        auto.name(),
        if compiled && detected {
            "power9-fma"
        } else {
            "portable"
        }
    );
}

#[test]
fn fma_projection_is_bit_identical_to_portable() {
    let Some((portable, fma)) = kernels() else {
        return;
    };
    let mut rng = Rng(303);
    for encoding in ALL {
        for (rows, cols) in shapes(encoding) {
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
                    fma.project(&weights, x, positions, &mut got[1..], &mut scratch[1..])
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
fn fma_transposed_accumulation_is_bit_identical_across_tiles() {
    let Some((portable, fma)) = kernels() else {
        return;
    };
    let mut rng = Rng(404);
    for encoding in ALL {
        for (rows, cols) in shapes(encoding) {
            let bytes = random_rows(encoding, rows, cols, &mut rng);
            let all = EncodedRows::new(encoding, rows, cols, &bytes).unwrap();
            for positions in [1, 2, 3, 4, 5, 8, 32, 256] {
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
                let mut got = start.clone();
                let split = (rows / 2).max(1).min(rows);
                let mut tiles = vec![(0, split)];
                if split < rows {
                    tiles.push((split, rows - split));
                }
                for (first, count) in tiles {
                    fma.accumulate_transposed(
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

/// Extreme finite F32 weights/activations/coefficients (F32 encoding stores any
/// bit pattern) and F64 accumulator edge values, in every rounding mode. The
/// transposed F64 accumulators expose any F64 difference directly.
#[test]
fn arithmetic_stress_is_bit_identical_in_every_rounding_mode() {
    let Some((portable, fma)) = kernels() else {
        return;
    };
    let specials = special_f32();
    let n = specials.len(); // 64 columns
    // Rows: the specials, rotations of them, negations, and cancellation pairs.
    let mut rows_values: Vec<f32> = Vec::new();
    for r in 0..12 {
        for c in 0..n {
            let v = specials[(c + r * 5) % n];
            rows_values.push(match r % 4 {
                0 => v,
                1 => -v,
                2 => specials[(c * 7 + r) % n],
                _ => {
                    if c % 2 == 0 {
                        v
                    } else {
                        -specials[(c - 1 + r * 5) % n]
                    }
                }
            });
        }
    }
    let rows = 12;
    let bytes = f32_rows(&rows_values);
    let weights = EncodedRows::new(Encoding::F32, rows, n, &bytes).unwrap();
    let accumulator_starts = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        1e300,
        -1e300,
        f64::from_bits(1),
        2f64.powi(-53),
        1e-300,
    ];
    under_each_rounding_mode(|mode| {
        for positions in [1, 2, 3, 4, 5, 8] {
            // Coefficients: specials (so products reach 2^-298 and ~2^255) and exact halves for ties.
            let coefficients: Vec<f32> = (0..positions * rows)
                .map(|i| {
                    if i % 3 == 0 {
                        specials[i % n]
                    } else {
                        2f32.powi(-(i as i32 % 30)) * if i % 2 == 0 { 1.0 } else { -1.0 }
                    }
                })
                .collect();
            for &start in &accumulator_starts {
                let mut want = vec![start; positions * n];
                let mut got = want.clone();
                let a = portable.accumulate_transposed(
                    &weights,
                    0,
                    &coefficients,
                    rows,
                    positions,
                    &mut want,
                    &mut vec![0.0; rows * n],
                );
                let b = fma.accumulate_transposed(
                    &weights,
                    0,
                    &coefficients,
                    rows,
                    positions,
                    &mut got,
                    &mut vec![0.0; 5 * n],
                );
                assert_eq!(a, b, "{mode}");
                assert_eq!(
                    bits64(&got),
                    bits64(&want),
                    "{mode} positions={positions} start={start:e}"
                );
            }
            // Projection over the same rows: outputs, and the error for F32 overflow, must match.
            let x: Vec<f32> = (0..positions * n)
                .map(|i| specials[(i * 11 + positions) % n])
                .collect();
            let mut want = vec![0.0f32; positions * rows];
            let mut got = vec![0.0f32; positions * rows];
            let a = portable.project(&weights, &x, positions, &mut want, &mut vec![0.0; rows * n]);
            let b = fma.project(&weights, &x, positions, &mut got, &mut vec![0.0; 5 * n]);
            assert_eq!(a, b, "{mode} positions={positions}");
            if a.is_ok() {
                assert_eq!(bits32(&got), bits32(&want), "{mode} positions={positions}");
            }
            // Bounded-magnitude version that never overflows F32, so outputs are always compared.
            let small: Vec<f32> = x
                .iter()
                .map(|v| if v.abs() > 1e10 { v.signum() } else { *v })
                .collect();
            let tame: Vec<f32> = rows_values
                .iter()
                .map(|v| {
                    if v.abs() > 1e10 {
                        v.signum() * 1e10
                    } else {
                        *v
                    }
                })
                .collect();
            let tame_bytes = f32_rows(&tame);
            let tame_w = EncodedRows::new(Encoding::F32, rows, n, &tame_bytes).unwrap();
            portable
                .project(
                    &tame_w,
                    &small,
                    positions,
                    &mut want,
                    &mut vec![0.0; rows * n],
                )
                .unwrap();
            fma.project(&tame_w, &small, positions, &mut got, &mut vec![0.0; 5 * n])
                .unwrap();
            assert_eq!(
                bits32(&got),
                bits32(&want),
                "{mode} tame positions={positions}"
            );
        }
    });
}

#[test]
fn signed_zero_results_match() {
    let Some((portable, fma)) = kernels() else {
        return;
    };
    // Every product is -0 or +0; sums of zeros take the IEEE sign rules.
    let bytes = f32_rows(&[-0.0, 0.0, -0.0, -0.0, 0.0, -0.0, -0.0, -0.0]);
    let w = EncodedRows::new(Encoding::F32, 2, 4, &bytes).unwrap();
    under_each_rounding_mode(|mode| {
        for x in [
            [1.0f32, 1.0, 1.0, 1.0],
            [-1.0, -1.0, -1.0, -1.0],
            [1.0, -1.0, 1.0, -1.0],
            [0.0, -0.0, 0.0, -0.0],
        ] {
            for positions in [1usize, 4, 5] {
                let xs: Vec<f32> = x.iter().cycle().take(positions * 4).copied().collect();
                let (mut a, mut b) = (vec![9.0f32; positions * 2], vec![9.0f32; positions * 2]);
                portable
                    .project(&w, &xs, positions, &mut a, &mut [0.0; 8])
                    .unwrap();
                fma.project(&w, &xs, positions, &mut b, &mut [0.0; 8])
                    .unwrap();
                assert_eq!(bits32(&a), bits32(&b), "{mode} x={x:?}");
                let coeff: Vec<f32> = x
                    .iter()
                    .take(2)
                    .cycle()
                    .take(positions * 2)
                    .copied()
                    .collect();
                for start in [0.0f64, -0.0] {
                    let (mut a, mut b) = (vec![start; positions * 4], vec![start; positions * 4]);
                    portable
                        .accumulate_transposed(&w, 0, &coeff, 2, positions, &mut a, &mut [0.0; 8])
                        .unwrap();
                    fma.accumulate_transposed(&w, 0, &coeff, 2, positions, &mut b, &mut [0.0; 8])
                        .unwrap();
                    assert_eq!(bits64(&a), bits64(&b), "{mode} start={start:?}");
                }
            }
        }
    });
}

macro_rules! real_case {
    ($name:literal) => {
        (
            include_bytes!(concat!("../../../../fixtures/quantized/", $name, ".rows")).as_slice(),
            include_bytes!(concat!("../../../../fixtures/quantized/", $name, ".x.f32")).as_slice(),
            include_bytes!(concat!(
                "../../../../fixtures/quantized/",
                $name,
                ".coeff.f32"
            ))
            .as_slice(),
        )
    };
}

#[test]
fn fma_matches_portable_on_real_target_rows() {
    let Some((portable, fma)) = kernels() else {
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
            b.as_chunks::<4>()
                .0
                .iter()
                .map(|&c| f32::from_le_bytes(c))
                .collect::<Vec<_>>()
        };
        let (x, coeff) = (f32s(x), f32s(coeff));
        let weights = EncodedRows::new(Encoding::try_from(id).unwrap(), rows, cols, raw).unwrap();
        let x9: Vec<f32> = x.iter().cycle().take(9 * cols).copied().collect();
        let (mut want, mut got) = (vec![0.0; 9 * rows], vec![0.0; 9 * rows]);
        portable
            .project(&weights, &x9, 9, &mut want, &mut vec![0.0; 5 * cols])
            .unwrap();
        fma.project(&weights, &x9, 9, &mut got, &mut vec![0.0; 5 * cols])
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
        fma.accumulate_transposed(
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
fn errors_and_mutation_contracts_match_portable() {
    let Some((portable, fma)) = kernels() else {
        return;
    };
    let mut bytes = vec![1u8; 2 * 34];
    bytes[34..36].copy_from_slice(&0x7c00u16.to_le_bytes());
    let w = EncodedRows::new(Encoding::Q8_0, 2, 32, &bytes).unwrap();
    for k in [portable, fma] {
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
    // A final F32 output overflow is still an error for both (4-position and tail paths).
    let big = f32_rows(&[3e38f32; 2]);
    let w = EncodedRows::new(Encoding::F32, 1, 2, &big).unwrap();
    for k in [portable, fma] {
        for positions in [1, 4, 5] {
            let x = vec![10.0f32; positions * 2];
            assert_eq!(
                k.project(&w, &x, positions, &mut vec![0.0; positions], &mut [0.0; 2]),
                Err(Error::NonFiniteOutput),
                "{} positions={positions}",
                k.name()
            );
        }
    }
    let w = EncodedRows::new(Encoding::Q8_0, 1, 32, &bytes[..34]).unwrap();
    for k in [portable, fma] {
        assert!(matches!(
            k.project(&w, &[f32::NAN; 32], 1, &mut [0.0], &mut [0.0; 32]),
            Err(Error::NonFiniteInput)
        ));
        assert!(matches!(
            k.project(&w, &[1.0; 32], 1, &mut [0.0], &mut [0.0; 31]),
            Err(Error::ScratchTooSmall { .. })
        ));
        let mut acc = vec![f64::NAN; 32];
        assert!(matches!(
            k.accumulate_transposed(&w, 0, &[1.0], 1, 1, &mut acc, &mut [0.0; 32]),
            Err(Error::NonFiniteInput)
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
fn raw_fma_entry_points_return_portable_statuses_before_any_write() {
    let Some((portable, fma)) = kernels() else {
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
    let (pp, fp): (Project, Project) = (portable.project, fma.project);
    for (i, &(kind, rows, cols, weight_bytes, stride, offset, scratch_len, null)) in
        cases.iter().enumerate()
    {
        let mut statuses = Vec::new();
        for f in [pp, fp] {
            let (mut out, mut scratch) = ([7.0f32; 4], [0.0f32; 64]);
            let w = if null {
                std::ptr::null()
            } else {
                bytes.as_ptr()
            };
            // SAFETY: deliberately invalid arguments that both kernels reject before
            // touching memory; non-null pointers are valid for what is checked.
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
    for k in [portable, fma] {
        let mut acc = [0.0f64; 32];
        let mut scratch = [0.0f32; 64];
        // SAFETY: valid buffers; the coefficient window is rejected before any access.
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
