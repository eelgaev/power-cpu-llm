// SPDX-License-Identifier: LGPL-2.1-or-later
use super::*;

const SEED: u64 = 0xcbf29ce484222325;

fn fnv(h: u64, v: u64) -> u64 {
    (h ^ v).wrapping_mul(0x100000001b3)
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

/// Every 61st binary32 pattern (all signs, exponents, NaNs).
fn f32_stride() -> impl Iterator<Item = f32> {
    (0..=u32::MAX / 61).map(|k| f32::from_bits(k * 61))
}

/// Random binary64 patterns, then values in the ranges the engine uses.
fn f64_inputs(lo: f64, hi: f64) -> impl Iterator<Item = f64> {
    let mut r = Rng(0x9e3779b97f4a7c15);
    (0..1u32 << 22).map(move |k| {
        let v = r.next();
        if k % 2 == 0 {
            f64::from_bits(v)
        } else {
            lo + (hi - lo) * ((v >> 11) as f64 / (1u64 << 53) as f64)
        }
    })
}

/// Pairs: random binary64 patterns, then x in (0, 1e4] and y in [-10, 10].
fn f64_pairs() -> impl Iterator<Item = (f64, f64)> {
    let mut r = Rng(0x2545f4914f6cdd1d);
    (0..1u32 << 21).map(move |k| {
        let (a, b) = (r.next(), r.next());
        if k % 2 == 0 {
            (f64::from_bits(a), f64::from_bits(b))
        } else {
            let u = |v: u64| (v >> 11) as f64 / (1u64 << 53) as f64;
            (1e4 * u(a), 20.0 * u(b) - 10.0)
        }
    })
}

/// Pairs: random binary32 patterns, then x in (0, 1e6] and y in [-2, 2].
fn f32_pairs() -> impl Iterator<Item = (f32, f32)> {
    let mut r = Rng(0x2545f4914f6cdd1d);
    (0..1u32 << 22).map(move |k| {
        let v = r.next();
        if k % 2 == 0 {
            (f32::from_bits(v as u32), f32::from_bits((v >> 32) as u32))
        } else {
            let u = |v: u64| (v >> 40) as f32 / (1u32 << 24) as f32;
            (1e6 * u(v), 4.0 * u(v << 24) - 2.0)
        }
    })
}

// Digests of the AC922 system libm (glibc 2.28-251.el8_10.31,
// /lib64/glibc-hwcaps/power9/libm-2.28.so) over the same inputs. The ports
// matched it on all 2^32 binary32 inputs and on billions of binary64
// samples (`matches_system_libm`).
const EXPF: u64 = 0xfe568fd438d7d408;
const SINCOSF: u64 = 0x684c340564e05bfc;
const EXP: u64 = 0xd0abc21cde940e1d;
const LN: u64 = 0xd55c4b00964ed47b;
const TANH: u64 = 0x8f2ca3b1e2096774;
const EXPM1: u64 = 0x3ab310a5b2555b36;
const POW: u64 = 0xf77d28485fb3c153;
const POWF: u64 = 0xe03cb0d507cdcb95;
const FMOD: u64 = 0x188d8e2346373ff4;

#[test]
fn expf_matches_system_libm_digest() {
    let h = f32_stride().fold(SEED, |h, x| fnv(h, u64::from(expf(x).to_bits())));
    assert_eq!(h, EXPF, "{h:#018x}");
}

#[test]
fn sincosf_matches_system_libm_digest() {
    let h = f32_stride().fold(SEED, |h, x| {
        let (s, c) = sincosf(x);
        fnv(fnv(h, u64::from(s.to_bits())), u64::from(c.to_bits()))
    });
    assert_eq!(h, SINCOSF, "{h:#018x}");
}

#[test]
fn exp_matches_system_libm_digest() {
    let h = f64_inputs(-746.0, 710.0)
        .chain(f64_inputs(-1.1, 1.1))
        .fold(SEED, |h, x| fnv(h, exp(x).to_bits()));
    assert_eq!(h, EXP, "{h:#018x}");
}

#[test]
fn ln_matches_system_libm_digest() {
    let h = f64_inputs(0.0, 10.0)
        .chain(f64_inputs(0.9, 1.1))
        .fold(SEED, |h, x| fnv(h, ln(x).to_bits()));
    assert_eq!(h, LN, "{h:#018x}");
}

#[test]
fn tanh_matches_system_libm_digest() {
    let h = f64_inputs(-25.0, 25.0)
        .chain(f64_inputs(-1.2, 1.2))
        .fold(SEED, |h, x| fnv(h, tanh(x).to_bits()));
    assert_eq!(h, TANH, "{h:#018x}");
}

#[test]
fn expm1_matches_system_libm_digest() {
    let h = f64_inputs(-50.0, 710.0)
        .chain(f64_inputs(-1.1, 1.1))
        .fold(SEED, |h, x| fnv(h, expm1(x).to_bits()));
    assert_eq!(h, EXPM1, "{h:#018x}");
}

#[test]
fn pow_matches_system_libm_digest() {
    let h = f64_pairs().fold(SEED, |h, (x, y)| fnv(h, pow(x, y).to_bits()));
    assert_eq!(h, POW, "{h:#018x}");
}

#[test]
fn powf_matches_system_libm_digest() {
    let h = f32_pairs().fold(SEED, |h, (x, y)| fnv(h, u64::from(powf(x, y).to_bits())));
    assert_eq!(h, POWF, "{h:#018x}");
}

#[test]
fn fmod_matches_system_libm_digest() {
    let h = f64_pairs().fold(SEED, |h, (x, y)| fnv(h, fmod(x, y).to_bits()));
    assert_eq!(h, FMOD, "{h:#018x}");
}

#[test]
fn special_values() {
    assert_eq!(expf(0.0).to_bits(), 1.0f32.to_bits());
    assert_eq!(expf(f32::NEG_INFINITY).to_bits(), 0);
    assert_eq!(expf(f32::INFINITY), f32::INFINITY);
    assert_eq!(expf(89.0), f32::INFINITY);
    assert_eq!(expf(-104.0).to_bits(), 0);
    assert!(expf(f32::NAN).is_nan());
    assert_eq!(exp(0.0).to_bits(), 1.0f64.to_bits());
    assert_eq!(exp(f64::NEG_INFINITY).to_bits(), 0);
    assert_eq!(exp(710.0), f64::INFINITY);
    assert_eq!(exp(-746.0).to_bits(), 0);
    assert!(exp(f64::NAN).is_nan());
    assert_eq!(ln(1.0).to_bits(), 0);
    assert_eq!(ln(0.0), f64::NEG_INFINITY);
    assert_eq!(ln(-0.0), f64::NEG_INFINITY);
    assert!(ln(-1.0).is_nan());
    assert_eq!(ln(f64::INFINITY), f64::INFINITY);
    assert_eq!(sincosf(0.0), (0.0, 1.0));
    assert_eq!(sincosf(-0.0).0.to_bits(), (-0.0f32).to_bits());
    let (s, c) = sincosf(f32::INFINITY);
    assert!(s.is_nan() && c.is_nan());
    assert_eq!(tanh(f64::INFINITY), 1.0);
    assert_eq!(tanh(-30.0), -1.0);
    assert_eq!(tanh(-0.0).to_bits(), (-0.0f64).to_bits());
    assert!(tanh(f64::NAN).is_nan());
    assert_eq!(expm1(f64::NEG_INFINITY), -1.0);
    assert_eq!(expm1(710.0), f64::INFINITY);
    assert_eq!(pow(2.0, 10.0), 1024.0);
    assert_eq!(pow(-2.0, 3.0), -8.0);
    assert_eq!(pow(f64::NAN, 0.0), 1.0);
    assert_eq!(pow(0.0, -1.0), f64::INFINITY);
    assert_eq!(pow(-0.0, -1.0), f64::NEG_INFINITY);
    assert!(pow(-2.0, 0.5).is_nan());
    assert_eq!(powf(2.0, 10.0), 1024.0);
    assert_eq!(powf(-2.0, 3.0), -8.0);
    assert_eq!(powf(1.0, f32::NAN), 1.0);
    assert_eq!(powf(-0.0, -3.0), f32::NEG_INFINITY);
    assert_eq!(powf(10.0, 50.0), f32::INFINITY);
    assert!(powf(-2.0, 0.5).is_nan());
    assert_eq!(fmod(7.0, 3.0), 1.0);
    assert_eq!(fmod(-7.0, 3.0), -1.0);
    assert_eq!(fmod(-6.0, 3.0).to_bits(), (-0.0f64).to_bits());
    assert_eq!(fmod(5.5, f64::INFINITY), 5.5);
    assert_eq!(fmod(1e300, 3.0), 0.0);
    assert_eq!(fmod(f64::from_bits(3), f64::from_bits(2)).to_bits(), 1);
    assert_eq!(fmod(1.0, 0.0).to_bits(), 0x7ff8000000000000);
    assert_eq!(fmod(f64::INFINITY, 2.0).to_bits(), 0x7ff8000000000000);
}

/// Every input against the C library this binary is linked with. Meaningful
/// only on the AC922 with the system glibc 2.28 libm, dynamically linked.
#[test]
#[ignore = "needs the AC922's system libm (glibc 2.28, POWER9), dynamically linked"]
#[allow(clippy::disallowed_methods)]
fn matches_system_libm() {
    unsafe extern "C" {
        #[link_name = "sincosf"]
        fn c_sincosf(x: f32, s: *mut f32, c: *mut f32);
    }
    let threads = std::thread::available_parallelism().map_or(8, |n| n.get()) as u64;
    std::thread::scope(|scope| {
        for t in 0..threads {
            scope.spawn(move || {
                let span = (1u64 << 32).div_ceil(threads);
                for b in t * span..((t + 1) * span).min(1 << 32) {
                    let x = f32::from_bits(b as u32);
                    assert_eq!(
                        expf(x).to_bits(),
                        std::hint::black_box(x).exp().to_bits(),
                        "expf {b:#x}"
                    );
                    let (s, c) = sincosf(x);
                    let (mut cs, mut cc) = (0.0, 0.0);
                    unsafe { c_sincosf(x, &mut cs, &mut cc) };
                    assert_eq!(
                        (s.to_bits(), c.to_bits()),
                        (cs.to_bits(), cc.to_bits()),
                        "sincosf {b:#x}"
                    );
                }
                let mut r = Rng(0x2545f4914f6cdd1d ^ (t + 1));
                for k in 0..50_000_000u64 {
                    let v = r.next();
                    let x = match k % 4 {
                        0 => f64::from_bits(v),
                        1 => f64::from(f32::from_bits(v as u32)),
                        2 => -746.0 + 1456.0 * ((v >> 11) as f64 / (1u64 << 53) as f64),
                        _ => 0.8 + 0.4 * ((v >> 11) as f64 / (1u64 << 53) as f64),
                    };
                    let x = std::hint::black_box(x);
                    assert_eq!(
                        exp(x).to_bits(),
                        x.exp().to_bits(),
                        "exp {:#x}",
                        x.to_bits()
                    );
                    let y = x.abs();
                    assert_eq!(ln(y).to_bits(), y.ln().to_bits(), "ln {:#x}", y.to_bits());
                    assert_eq!(
                        tanh(x).to_bits(),
                        x.tanh().to_bits(),
                        "tanh {:#x}",
                        x.to_bits()
                    );
                    assert_eq!(
                        expm1(x).to_bits(),
                        x.exp_m1().to_bits(),
                        "expm1 {:#x}",
                        x.to_bits()
                    );
                    let z = std::hint::black_box(f64::from_bits(r.next()));
                    assert_eq!(
                        pow(y, z).to_bits(),
                        y.powf(z).to_bits(),
                        "pow {:#x} {:#x}",
                        y.to_bits(),
                        z.to_bits()
                    );
                    let w = r.next();
                    let (a, b) = (f32::from_bits(w as u32), f32::from_bits((w >> 32) as u32));
                    assert_eq!(powf(a, b).to_bits(), a.powf(b).to_bits(), "powf {w:#x}");
                }
            });
        }
    });
}

/// `tanh_block` against scalar `tanh`: random patterns, the GELU argument
/// range, every control-flow boundary, and a slice of f32 GELU arguments.
#[test]
fn tanh_block_matches_scalar() {
    let mut inputs: Vec<f64> = f64_inputs(-30.0, 30.0).collect();
    let mut r = Rng(0x1234_5678_9abc_def1);
    for _ in 0..(1 << 22) {
        let g = f32::from_bits(r.next() as u32);
        if g.is_finite() {
            let g = f64::from(g);
            inputs.push(0.7978845608028654 * (g + 0.044715 * g * g * g));
        }
        let u = (r.next() >> 11) as f64 / (1u64 << 53) as f64;
        inputs.push(8.0 * u - 4.0);
    }
    // Boundaries: every threshold +- a few ulps, both signs.
    for hw in [
        0x3c80_0000u32,
        0x3c7f_ffff,
        0x3fd6_2e42 - 0x0010_0000,
        0x3ff0_a2b2 - 0x0010_0000,
        0x3ff0_0000,
        0x4036_0000,
        0x4043_687a - 0x0010_0000,
        0x7ff0_0000,
        0x3fe0_0000,
    ] {
        for low in [0u32, 1, 0x7fff_ffff, 0x8000_0000, 0xffff_ffff, 0xb3f0_0000] {
            for d in -3i64..=3 {
                let bits = ((u64::from(hw) << 32) | u64::from(low)).wrapping_add(d as u64);
                for s in [0u64, 1 << 63] {
                    inputs.push(f64::from_bits(bits | s));
                }
            }
        }
    }
    for n in 0..60 {
        let x = f64::from(n) * core::f64::consts::LN_2 * 0.5;
        for d in -4i64..=4 {
            for s in [x, -x] {
                inputs.push(f64::from_bits(s.to_bits().wrapping_add(d as u64)));
            }
        }
    }
    inputs.extend([
        0.0,
        -0.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        1e-300,
        -1e-300,
    ]);
    let mut out = vec![0.0; TANH_BLOCK];
    for chunk in inputs.chunks(TANH_BLOCK) {
        tanh_block(chunk, &mut out[..chunk.len()]);
        for (&x, &y) in chunk.iter().zip(&out) {
            let want = tanh(x);
            assert!(
                y.to_bits() == want.to_bits() || (y.is_nan() && want.is_nan()),
                "tanh_block {:#x}: {:#x} != {:#x}",
                x.to_bits(),
                y.to_bits(),
                want.to_bits()
            );
        }
    }
}

/// ns per element of `tanh_block` and scalar `tanh` per input range; by hand.
#[test]
#[ignore]
fn tanh_block_bench() {
    let mut r = Rng(0x1234_5678_9abc_def1);
    for (lo, hi) in [
        (0.01, 0.17),
        (0.18, 0.51),
        (0.53, 0.99),
        (1.0, 6.5),
        (7.0, 19.0),
        (20.0, 21.9),
        (0.0, 8.0),
    ] {
        let y: Vec<f64> = (0..TANH_BLOCK)
            .map(|_| {
                let u = (r.next() >> 11) as f64 / (1u64 << 53) as f64;
                let v = lo + (hi - lo) * u;
                if r.next() & 1 == 0 { v } else { -v }
            })
            .collect();
        let mut out = vec![0.0; TANH_BLOCK];
        let reps = 20000;
        let t = std::time::Instant::now();
        for _ in 0..reps {
            tanh_block(&y, &mut out);
            std::hint::black_box(&out);
        }
        let new = t.elapsed().as_secs_f64() / (reps * TANH_BLOCK) as f64 * 1e9;
        let t = std::time::Instant::now();
        for _ in 0..reps {
            for (o, &v) in out.iter_mut().zip(&y) {
                *o = tanh(v);
            }
            std::hint::black_box(&out);
        }
        let old = t.elapsed().as_secs_f64() / (reps * TANH_BLOCK) as f64 * 1e9;
        println!("range {lo}..{hi}: block {new:.2} scalar {old:.2} ns/elem");
    }
}
