// SPDX-License-Identifier: LGPL-2.1-or-later
//! Bit-exact Rust ports of the C library transcendentals that the engine's
//! numerics depend on, so results don't change with the C library a binary
//! is linked against (a static binary carries its own libm).
//!
//! Each function returns, for every input, the same bits as the AC922's
//! system libm (`/lib64/glibc-hwcaps/power9/libm-2.28.so`, glibc
//! 2.28-251.el8_10.31):
//! - [`exp`] and [`ln`]: the IBM Accurate Mathematical Library `exp` and
//!   `log` (`sysdeps/ieee754/dbl-64/e_exp.c`, `e_log.c`);
//! - [`expf`]: the POWER8 assembly `expf` (`powerpc64/power8/fpu/e_expf.S`),
//!   which that libm selects on POWER8 and later;
//! - [`sincosf`]: the generic `sincosf` (`sysdeps/ieee754/flt-32/s_sincosf.c`),
//!   which LLVM calls for an `f32` `sin` and `cos` of the same angle.
//! - [`pow`]: the IBM `pow` (`dbl-64/e_pow.c`, with `__exp1` from e_exp.c);
//! - [`powf`]: the generic `powf` (`flt-32/e_powf.c`, `powf@@GLIBC_2.27`);
//! - [`tanh`] and [`expm1`]: fdlibm's `s_tanh.c` and `s_expm1.c`;
//! - [`fmod`]: `dbl-64/wordsize-64/e_fmod.c` (exact; for the `fmod` symbol
//!   that Rust's float `%` calls).
//!
//! The C files were compiled with floating-point contraction, so the
//! operation order and every fused multiply-add here follow that library's
//! disassembly, not just the C source (e_pow.c was built without
//! contraction). `mul_add` lowers to one `fmadd`.
//! Rounding is assumed to be to nearest (glibc's `exp` and `log` force it).
//!
//! Derived from the GNU C Library, so this module (`src/libm.rs`, `src/libm/`) is under the GNU Lesser
//! General Public License, version 2.1 or later.
//!
//! Sun Microsystems / fdlibm notice (applies to `tanh`, `expm1` and `fmod`, which
//! derive from fdlibm's `s_tanh.c`, `s_expm1.c` and `e_fmod.c` as shipped in glibc
//! 2.28; `expm1` also carries glibc's 1997 modifications by Naohiko
//! Shimizu/Tokai University for pipelined processors):
//!
//! ====================================================
//! Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
//!
//! Developed at SunPro, a Sun Microsystems, Inc. business.
//! Permission to use, copy, modify, and distribute this
//! software is freely granted, provided that this notice
//! is preserved.
//! ====================================================

// `x - x` below is deliberate: it is NaN for an infinite or NaN x.
#![allow(clippy::eq_op, clippy::excessive_precision, clippy::unreadable_literal)]

mod tables;
mod tanh_batch;
#[cfg(test)]
mod tests;

use tables::*;
pub use tanh_batch::{BLOCK as TANH_BLOCK, tanh_block};

fn high(x: f64) -> i32 {
    (x.to_bits() >> 32) as i32
}

fn low(x: f64) -> i32 {
    x.to_bits() as i32
}

fn from_words(high: i32, low: i32) -> f64 {
    f64::from_bits((u64::from(high as u32) << 32) | u64::from(low as u32))
}

/// A double whose high word is `high` and low word zero (glibc's `binexp`).
fn from_high(high: i32) -> f64 {
    from_words(high, 0)
}

fn table(t: &[u64], i: usize) -> f64 {
    f64::from_bits(t[i])
}

const HUGE: f64 = 1.0e300;
const TINY: f64 = 1.0e-300;

// IBM exp (e_exp.c) constants, shared by `exp` and `exp1`.
const LOG2E: f64 = f64::from_bits(0x3ff71547652b82fe);
const LN_TWO1: f64 = f64::from_bits(0x3fe62e42fefa3800);
const LN_TWO2: f64 = f64::from_bits(0x3d2ef35793c76730);
const EXP_P2: f64 = f64::from_bits(0x3fe00000000004dc);
const EXP_P3: f64 = f64::from_bits(0x3fc5555555555a0f);
const THREE33: f64 = 25769803776.0;
const THREE51: f64 = 6755399441055744.0;
const T256: f64 = f64::from_bits(0x4ff0000000000000);
const EXP_BIGINT: i32 = 0x40862002;
const EXP_BADINT: i32 = 0x40876000;

/// e^x, bit-identical to glibc 2.28 `exp` (IBM `__ieee754_exp`).
pub fn exp(x: f64) -> f64 {
    const T2: f64 = f64::from_bits(0x3fc5555555555555);
    const T3: f64 = f64::from_bits(0x3fa5555555555555);
    const T4: f64 = f64::from_bits(0x3f81111111111111);
    const T5: f64 = f64::from_bits(0x3f56c16c16c16c17);
    const INFINT: i32 = 0x7ff00000;

    let m = high(x);
    let n = m & 0x7fffffff;

    if n < 0x3ff0a2b2 {
        // |x| < 1.03972053527832
        if n < 0x3f862e42 {
            // |x| < 3/2 ln 2
            if n < 0x3ed00000 {
                // |x| < 1/64 ln 2
                if n < 0x3e300000 {
                    return 1.0 + x;
                }
                return x.mul_add(0.5, 1.0).mul_add(x, 1.0);
            }
            let t = x * x;
            let a = x.mul_add(T4, T3);
            let tt = t * t;
            let h = x.mul_add(T2, 0.5);
            let b = t.mul_add(T5, a) * tt;
            return (h.mul_add(t, b) + x) + 1.0;
        }
        // The multiple of 2^-6 nearest x.
        let k = n >> 20;
        let mut j = (0x00100000 | (n & 0x000fffff)) >> (0x40c - k);
        j = (j - 1) & !1;
        if m < 0 {
            j += 134;
        }
        let j = j as usize;
        let z = x - table(&EXP_TBL2, j);
        let t = z * z;
        let a = z.mul_add(T4, T3);
        let h = z.mul_add(T2, 0.5);
        let b = t.mul_add(T5, a) * (t * t);
        let r = h.mul_add(t, b) + z;
        let e = table(&EXP_TBL2, j + 1);
        return e.mul_add(r, e);
    }

    if n >= EXP_BADINT {
        if n > INFINT {
            return x + x; // NaN
        }
        if n < INFINT {
            return if x > 0.0 { HUGE * HUGE } else { TINY * TINY };
        }
        if low(x) != 0 {
            return x + x; // NaN
        }
        return if x > 0.0 { f64::INFINITY } else { 0.0 };
    }

    // x = bexp*ln(2) + base + del, with base a multiple of 2^-18.
    let y = x.mul_add(LOG2E, THREE51);
    let bexp = y - THREE51;
    let t = (-bexp).mul_add(LN_TWO1, x);
    let y2 = t + THREE33;
    let base = y2 - THREE33;
    let del = (-bexp).mul_add(LN_TWO2, t - base);
    let (al, rem) = exp_table_step(y2, del);
    let res = rem + al;

    if n < EXP_BIGINT {
        // Maximum ULP error is 0.500007 (glibc).
        return res * from_high(low(y).wrapping_add(1023) << 20);
    }

    let cor = (al - res) + rem;
    if m < 0 {
        let (mut res, mut cor, mut ex) = (res, cor, low(y));
        if res < 1.0 {
            res += res;
            cor += cor;
            ex -= 1;
        }
        if ex >= -1022 {
            return res * from_high((1023 + ex) << 20);
        }
        // Subnormal result.
        let binexp = from_high((ex + 2045) << 20);
        let t = binexp.mul_add(res, 1.0);
        let y = binexp.mul_add(cor, binexp.mul_add(res, 1.0 - t));
        let r = ((t + y) - 1.0) * from_high(0x00100000);
        return if r == 0.0 { TINY * TINY } else { r };
    }
    res * from_high(low(y).wrapping_add(767) << 20) * T256
}

/// Natural logarithm, bit-identical to glibc 2.28 `log` (IBM `__ieee754_log`).
pub fn ln(x: f64) -> f64 {
    const A2: f64 = f64::from_bits(0xbfe000000001aa8f);
    const A3: f64 = f64::from_bits(0x3fd5555555588d2e);
    const B: [u64; 9] = [
        0x3fd5555555555555,
        0xbfcfffffffffffbb,
        0x3fc999999999992f,
        0xbfc55555556503fd,
        0x3fc24924925b3d62,
        0xbfbffffe160472fc,
        0x3fbc71c525db58ac,
        0xbfb9a4ac11a2a61c,
        0x3fb750770df2b591,
    ];
    const SQRT_2: f64 = f64::from_bits(0x3ff6a09e667f3bcc);
    const H1: f64 = f64::from_bits(0x3fd2e00000000000);
    const H2: f64 = f64::from_bits(0x3f66900000000000);
    const DEL_U: f64 = f64::from_bits(0x3f70000000000000);
    const DEL_V: f64 = f64::from_bits(0x3ef0000000000000);
    const LN2A: f64 = f64::from_bits(0x3fe62e42fefa3800);
    const LN2B: f64 = f64::from_bits(0x3d2ef35793c76730);
    const TWO54: f64 = f64::from_bits(0x4350000000000000);
    const U03: f64 = f64::from_bits(0x3f9eb851eb851eb8);
    const MHALF: f64 = -0.5;
    let b = |k: usize| f64::from_bits(B[k]);

    let ux = high(x);
    let mut x = x;
    let mut n = 0;
    if ux < 0x00100000 {
        if ((ux & 0x7fffffff) | low(x)) == 0 {
            return MHALF / 0.0; // -inf
        }
        if ux < 0 {
            return (x - x) / 0.0; // NaN
        }
        n -= 54;
        x *= TWO54;
    }
    if ux >= 0x7ff00000 {
        return x + x; // inf or NaN
    }

    let w = x - 1.0;
    if w.abs() <= U03 {
        // |x - 1| <= 0.03.
        if w == 0.0 {
            return 0.0;
        }
        let t8 = w * MHALF;
        let a = w * t8;
        let aa = w.mul_add(t8, -a);
        let hi = w + a;
        let lo = if w.abs() > a.abs() {
            (w - hi) + a
        } else {
            (a - hi) + w
        };
        let w3 = (w * w) * w;
        let mut pol = b(8).mul_add(w, b(7));
        for k in (0..7).rev() {
            pol = w.mul_add(pol, b(k));
        }
        return w3.mul_add(pol, lo + aa) + hi;
    }

    // x = u * 2^n with 1/sqrt(2) < u < sqrt(2).
    n += (high(x) >> 20) - 1023;
    let mut u = from_words((high(x) & 0x000fffff) | 0x3ff00000, low(x));
    if u > SQRT_2 {
        u *= 0.5;
        n += 1;
    }
    let dbl_n = f64::from(n);

    // u_i = 1 + (i-75)/2^8 nearest u, v_j = 1 + (j-180)/2^16 nearest u/u_i.
    let i = ((high(u + H1) & 0x000fffff) >> 12) as usize;
    let iu = table(&LOG_IU, i);
    let j = ((high(u.mul_add(iu, H2)) & 0x000fffff) >> 4) as usize;
    let pa = f64::from(i as i32 - 75).mul_add(DEL_U, 1.0);
    let pb = f64::from(j as i32 - 180).mul_add(DEL_V, 1.0);
    let q = (-pa).mul_add(pb, u);
    let w = q * (iu * table(&LOG_IV, j));
    let pol = (A3.mul_add(w, A2) * w).mul_add(w, w);

    let nln2a = dbl_n * LN2A;
    let (luai, lubi) = (table(&LOG_LU, 2 * i), table(&LOG_LU, 2 * i + 1));
    let (lvaj, lvbj) = (table(&LOG_LV, 2 * j), table(&LOG_LV, 2 * j + 1));
    let sij = luai + lvaj;
    let ssij = if luai.abs() > lvaj.abs() {
        (luai - sij) + lvaj
    } else {
        (lvaj - sij) + luai
    };
    let a = nln2a + sij;
    let ttij = if nln2a.abs() > sij.abs() {
        (nln2a - a) + sij
    } else {
        (sij - a) + nln2a
    };
    let b0 = LN2B.mul_add(dbl_n, ((lubi + lvbj) + ssij) + ttij);
    a + (b0 + pol)
}

/// e^x in binary32, bit-identical to glibc 2.28 `expf` on POWER8 and later
/// (the `e_expf.S` assembly).
pub fn expf(x: f32) -> f32 {
    const KLN2: f64 = f64::from_bits(0x40571547652b82fe); // 64/ln 2
    const RS: f64 = f64::from_bits(0x4168000000000000); // 2^23 + 2^22
    const P0: f64 = f64::from_bits(0x3fefffffffffe7c6);
    const P1: f64 = f64::from_bits(0x3fe00000008d6118);
    const P2: f64 = f64::from_bits(0x3fc55550da752d4f);
    const P3: f64 = f64::from_bits(0x3fa56420eb78fa85);
    const NLN2K: f64 = f64::from_bits(0xbf862e42fefa39ef); // -ln 2/64

    let bits = x.to_bits();
    let abs = (bits & 0x7fffffff) as i32;
    let xd = f64::from(x);
    let kd = xd.mul_add(KLN2, RS);

    // x = (64n + j) ln2/64 + y; e^x = 2^n T[j] (1 + P(y)).
    let reduce = |near_limit: bool| {
        let k = kd as f32;
        let kbits = k.to_bits();
        let j = (kbits & 63) as usize;
        let y = f64::from(k - 12582912.0).mul_add(NLN2K, xd);
        let z = y * y;
        let p = y.mul_add(P2.mul_add(z, P0), P3.mul_add(z, P1) * z);
        let t = table(&EXPF_T, j);
        let r = p.mul_add(t, t);
        let n = kbits & !63;
        let scale = if near_limit {
            f64::from_bits((u64::from(n.wrapping_add(0xffc0)) & 0x1ffff) << 46)
        } else {
            f64::from(f32::from_bits((n.wrapping_add(0x1fc0) & 0x3fff) << 17))
        };
        (scale * r) as f32
    };

    if abs >= 0x42ad496b {
        // |x| >= 125 ln 2: near the limits, overflow, underflow, inf or NaN.
        let negative = bits >> 31 != 0;
        if abs <= if negative { 0x42cff1b4 } else { 0x42b17217 } {
            return reduce(true);
        }
        if abs >= 0x7f800000 {
            if abs != 0x7f800000 {
                return (xd + xd) as f32; // NaN
            }
            return if negative { 0.0 } else { f32::INFINITY };
        }
        let s = f32::from_bits(if negative { 0x0d800000 } else { 0x71800000 });
        return s * s;
    }
    if abs < 0x31800000 {
        return x + 1.0; // |x| < 2^-28
    }
    reduce(false)
}

/// (sin x, cos x) in binary32, bit-identical to glibc 2.28 `sincosf` (the
/// generic `s_sincosf.c`).
pub fn sincosf(x: f32) -> (f32, f32) {
    const C: [u64; 5] = [
        0xbfdffffffffe98ae,
        0x3fa55555545c50c7,
        0xbf56c16b348b6874,
        0x3efa00eb9ac43cc0,
        0xbe923c97dd8844d7,
    ];
    const S: [u64; 5] = [
        0xbfc5555555551cd9,
        0x3f81111110c2688b,
        0xbf2a019f8b4bd1f9,
        0x3ec71d7264e6b5b4,
        0xbe5a947e1674b58a,
    ];
    const SS0: f64 = f64::from_bits(0xbfc555555543d49d);
    const SS1: f64 = f64::from_bits(0x3f8110f475cec8c5);
    const CC0: f64 = f64::from_bits(0xbfdfffffff5cc6fd);
    const CC1: f64 = f64::from_bits(0x3fa55514b178dac5);
    const PI_2_HI: f64 = f64::from_bits(0x3ff921fb54400000);
    const PI_2_LO: f64 = f64::from_bits(0x3dd0b4611a626332);
    const SMALL: f64 = f64::from_bits(0x3cd0000000000000); // 2^-50
    const INV_PI_4: f64 = f64::from_bits(0x3ff45f306dc9c883);
    const PI_4: f64 = f64::from_bits(0x3fe921fb54442d18);
    const NINE_PI_4: f64 = f64::from_bits(0x401c463abeccb2bb);
    const PIO2: [u64; 6] = [
        0x0000000000000000,
        0x3ff921fb54442d18,
        0x400921fb54442d18,
        0x4012d97c7f3321d2,
        0x401921fb54442d18,
        0x401f6a7a2955385e,
    ];
    const INVPIO4: [u64; 8] = [
        0x0000000000000000,
        0x3ff45f306c000000,
        0x3e3c9c882a000000,
        0x3c54fe13a8000000,
        0x3aaf47d4d0000000,
        0x38fbb81b6c000000,
        0x3714acc9e0000000,
        0x3560e4107c000000,
    ];
    const ONES: [f64; 2] = [1.0, -1.0];

    // The two Chebyshev forms on [-pi/4, pi/4].
    let sin_poly = |theta: f64, theta2: f64| {
        let mut p = f64::from_bits(S[4]).mul_add(theta2, f64::from_bits(S[3]));
        for k in (0..3).rev() {
            p = theta2.mul_add(p, f64::from_bits(S[k]));
        }
        (theta * theta2).mul_add(p, theta)
    };
    let cos_poly = |theta2: f64| {
        let mut p = f64::from_bits(C[4]).mul_add(theta2, f64::from_bits(C[3]));
        for k in (0..3).rev() {
            p = theta2.mul_add(p, f64::from_bits(C[k]));
        }
        theta2.mul_add(p, 1.0)
    };
    // theta: reduced |x|; n: trunc(|x| / (pi/4)) + 1.
    let reduced = |theta: f64, n: u32, signbit: usize| {
        let theta2 = theta * theta;
        let pick = |n: u32| {
            if n & 2 == 0 {
                sin_poly(theta, theta2)
            } else {
                cos_poly(theta2)
            }
        };
        let s = ONES[((n >> 2) & 1) as usize ^ signbit] * pick(n);
        let n = n.wrapping_add(2);
        let c = ONES[((n >> 2) & 1) as usize] * pick(n);
        (s as f32, c as f32)
    };

    let theta = f64::from(x);
    let abstheta = theta.abs();
    if abstheta < PI_4 {
        // |x| >= 2^-5
        if abstheta >= 0.03125 {
            let theta2 = theta * theta;
            return (sin_poly(theta, theta2) as f32, cos_poly(theta2) as f32);
        }
        // |x| >= 2^-27
        if abstheta >= 7.450580596923828125e-9 {
            let theta2 = theta * theta;
            let theta3 = theta * theta2;
            let c = theta2.mul_add(theta3.mul_add(CC1, CC0), 1.0);
            let s = theta3.mul_add(theta2.mul_add(SS1, SS0), theta);
            return (s as f32, c as f32);
        }
        let s = if theta != 0.0 {
            (-theta).mul_add(SMALL, theta)
        } else {
            theta
        };
        return (s as f32, (1.0 - abstheta) as f32);
    }

    let signbit = usize::from(x < 0.0);
    if abstheta < NINE_PI_4 {
        let n = abstheta.mul_add(INV_PI_4, 1.0) as u32;
        return reduced(
            abstheta - f64::from_bits(PIO2[(n / 2) as usize]),
            n,
            signbit,
        );
    }
    if !abstheta.is_finite() {
        let nan = x - x;
        return (nan, nan);
    }
    // |x| < 2^23
    if abstheta < 8388608.0 {
        let n = ((abstheta * INV_PI_4) as u32).wrapping_add(1);
        let k = f64::from(n / 2);
        let theta = (-k).mul_add(PI_2_LO, (-k).mul_add(PI_2_HI, abstheta));
        return reduced(theta, n, signbit);
    }

    // |x| >= 2^23: reduce with 28-bit pieces of 4/pi.
    let exponent = (((x.abs().to_bits() as i32) >> 23) - 127 + 3) / 28;
    let piece = |k: i32| f64::from_bits(INVPIO4[k as usize]) * abstheta;
    let (mut a, b, c, d) = (
        piece(exponent),
        piece(exponent + 1),
        piece(exponent + 2),
        piece(exponent + 3),
    );
    let mut l = a as u64;
    l &= !7;
    a -= l as f64;
    let mut e = a + b;
    let mut l = e as u64;
    e = a - l as f64;
    if l & 1 != 0 {
        e -= 1.0;
        e = d + (c + (b + e));
    } else {
        e = d + (c + (b + e));
        if e > 1.0 {
            l += 1;
            e -= 2.0;
        }
    }
    reduced(e * PI_4, (l as u32).wrapping_add(1), signbit)
}

/// e^(x + xx) for [`pow`], bit-identical to glibc 2.28's `__exp1` (e_exp.c).
fn exp1(x: f64, xx: f64) -> f64 {
    let m = high(x);
    let n = m & 0x7fffffff;

    if n > 0x3c8fffff && n < EXP_BIGINT {
        let y = x.mul_add(LOG2E, THREE51);
        let bexp = y - THREE51;
        let t = (-bexp).mul_add(LN_TWO1, x);
        let y2 = t + THREE33;
        let base = y2 - THREE33;
        let del = (-bexp).mul_add(LN_TWO2, xx) + (t - base);
        let (al, rem) = exp_table_step(y2, del);
        return (rem + al) * from_high(low(y).wrapping_add(1023) << 20);
    }
    if n <= 0x3c8fffff {
        return 1.0;
    }
    if n >= EXP_BADINT {
        if n > 0x7ff00000 {
            return f64::NAN;
        }
        if n < 0x7ff00000 {
            return if x > 0.0 { HUGE * HUGE } else { TINY * TINY };
        }
        if low(x) != 0 {
            return f64::NAN;
        }
        return if x > 0.0 { f64::INFINITY } else { 0.0 };
    }

    let y = x.mul_add(LOG2E, THREE51);
    let bexp = y - THREE51;
    let t = (-bexp).mul_add(LN_TWO1, x);
    let y2 = t + THREE33;
    let base = y2 - THREE33;
    let del = (-bexp).mul_add(LN_TWO2, xx) + (t - base);
    let (al, rem) = exp_table_step(y2, del);
    let res = al + rem;
    if m < 0 {
        let (mut res, mut cor, mut ex) = (res, (al - res) + rem, low(y));
        if res < 1.0 {
            res += res;
            cor += cor;
            ex -= 1;
        }
        if ex >= -1022 {
            return res * from_high((1023 + ex) << 20);
        }
        // Subnormal result.
        let binexp = from_high((ex + 2045) << 20);
        let t = binexp.mul_add(res, 1.0);
        let y = binexp.mul_add(cor, binexp.mul_add(res, 1.0 - t));
        return ((t + y) - 1.0) * from_high(0x00100000);
    }
    res * from_high(low(y).wrapping_add(767) << 20) * T256
}

/// The table lookup and polynomial shared by [`exp`] and [`exp1`]: `al` and
/// `rem` with e^(base + del) = al + rem, base from `y2 = base + 3*2^33`.
fn exp_table_step(y2: f64, del: f64) -> (f64, f64) {
    let i = (((low(y2) >> 8) & !1) + 356) as usize;
    let j = ((low(y2) & 511) << 1) as usize;
    let eps = (del * del).mul_add(EXP_P3.mul_add(del, EXP_P2), del);
    let (ci, ci1) = (table(&EXP_COAR, i), table(&EXP_COAR, i + 1));
    let (fj, fj1) = (table(&EXP_FINE, j), table(&EXP_FINE, j + 1));
    let al = ci * fj;
    let bet = fj1.mul_add(ci1, ci.mul_add(fj1, fj * ci1));
    (al, eps.mul_add(al, eps.mul_add(bet, bet)))
}

fn is_signaling(x: f64) -> bool {
    x.is_nan() && x.to_bits() & (1 << 51) == 0
}

/// 0 if `x` is not an integer, 1 if even, -1 if odd (e_pow.c `checkint`).
fn checkint(x: f64) -> i32 {
    let m = (high(x) & 0x7fffffff) as u32;
    if m >= 0x7ff00000 {
        return 0;
    }
    if m >= 0x43400000 {
        return 1;
    }
    if m < 0x40000000 {
        return 0;
    }
    let n = low(x) as u32;
    let k = (m >> 20) - 1023;
    if k == 52 {
        return if n & 1 != 0 { -1 } else { 1 };
    }
    if k > 20 {
        if n << (k - 20) != 0 {
            return 0;
        }
        return if n << (k - 21) != 0 { -1 } else { 1 };
    }
    if n != 0 {
        return 0;
    }
    if k == 20 {
        return if m & 1 != 0 { -1 } else { 1 };
    }
    if m << (k + 12) != 0 {
        return 0;
    }
    if m << (k + 11) != 0 { -1 } else { 1 }
}

/// log(x) for x > 0 as a double-double (e_pow.c `log1`). e_pow.c is built
/// without contraction, so this follows the C source exactly.
fn pow_log(x: f64) -> (f64, f64) {
    const BIGU: f64 = f64::from_bits(0x4297fffffffffd2c);
    const BIGV: f64 = f64::from_bits(0x4207fffffff8016a);
    const TWO52E: f64 = f64::from_bits(0x43300000000003ff);
    const LN2A: f64 = f64::from_bits(0x3fe62e42fefa3800);
    const LN2B: f64 = f64::from_bits(0x3d2ef35793c76730);
    const P2: f64 = -0.5;
    const P3: f64 = f64::from_bits(0x3fd5555555555555);
    const P4: f64 = -0.25;
    const Q2: f64 = -0.5;
    const Q3: f64 = f64::from_bits(0x3fd55555555553fa);
    const Q4: f64 = f64::from_bits(0xbfcffffffffffafc);
    const Q5: f64 = f64::from_bits(0x3fc9999a7b15fc27);
    const Q6: f64 = f64::from_bits(0xbfc555565c6689b0);
    const R3: f64 = f64::from_bits(0x3fd5555555555555);
    const R4: f64 = -0.25;
    const R5: f64 = f64::from_bits(0x3fc9999999996d12);
    const R6: f64 = f64::from_bits(0xbfc55555555500ba);
    const R7: f64 = f64::from_bits(0x3fc24927b7978eb2);
    const R8: f64 = f64::from_bits(0xbfc00003c65e1a27);
    let ui = |k: usize| table(&POW_UI, k);
    let vj = |k: usize| table(&POW_VJ, k);

    let (mut x, mut add) = (x, 0.0);
    let mut m = high(x);
    if m < 0x00100000 {
        // Subnormal x.
        x *= 4503599627370496.0;
        add = -52.0;
        m = high(x);
    }
    let (hi, ex) = if (m & 0x000fffff) < 0x0006a09e {
        ((m & 0x000fffff) | 0x3ff00000, m >> 20)
    } else {
        ((m & 0x000fffff) | 0x3fe00000, (m >> 20) + 1)
    };
    let u = from_words(hi, low(x));
    let two52 = from_words(0x43300000, ex);
    let v = u + BIGU;
    let uu = v - BIGU;
    let i = ((low(v) & 0x3ff) << 2) as usize;
    if ex == 1023 {
        if i > 1192 && i < 1208 {
            // |x - 1| < 1.5 * 2^-10
            let t = x - 1.0;
            let t1 = (t + 5.0e6) - 5.0e6;
            let t2 = t - t1;
            let e1 = t - 0.5 * t1 * t1;
            let e2 = t * t * t * (R3 + t * (R4 + t * (R5 + t * (R6 + t * (R7 + t * R8)))))
                - 0.5 * t2 * (t + t1);
            let res = e1 + e2;
            return (res, (e1 - res) + e2);
        }
        let v = u * (ui(i) + ui(i + 1)) + BIGV;
        let vv = v - BIGV;
        let j = ((low(v) & 0x0007ffff) * 3) as usize;
        let eps = u - uu * vv;
        let e1 = eps * ui(i);
        let e2 = eps * (ui(i + 1) + vj(j) * (ui(i) + ui(i + 1)));
        let e = e1 + e2;
        let e2 = (e1 - e) + e2;
        let t = ui(i + 2) + vj(j + 1);
        let t1 = t + e;
        let t2 =
            ((((t - t1) + e) + (ui(i + 3) + vj(j + 2))) + e2) + e * e * (P2 + e * (P3 + e * P4));
        let res = t1 + t2;
        return (res, (t1 - res) + t2);
    }
    let eps = u - uu;
    let nx = (two52 - TWO52E) + add;
    let e1 = eps * ui(i);
    let e2 = eps * ui(i + 1);
    let e = e1 + e2;
    let e2 = (e1 - e) + e2;
    let t = nx * LN2A + ui(i + 2);
    let t1 = t + e;
    let t2 = (((t - t1) + e) + nx * LN2B + ui(i + 3) + e2)
        + e * e * (Q2 + e * (Q3 + e * (Q4 + e * (Q5 + e * Q6))));
    let res = t1 + t2;
    (res, (t1 - res) + t2)
}

/// x^y, bit-identical to glibc 2.28 `pow` (IBM `__ieee754_pow`; the
/// wrapper's error handling never changes the value).
pub fn pow(x: f64, y: f64) -> f64 {
    const CN: f64 = 134217729.0; // 2^27 + 1, Dekker's splitter
    let (hx, lx, hy, ly) = (high(x), low(x), high(y), low(y));
    let (qx, qy) = (hx & 0x7fffffff, hy & 0x7fffffff);

    if ly == 0 {
        if ((qx == 0x7ff00000 && lx != 0) || qx > 0x7ff00000) && (y != 0.0 || is_signaling(x)) {
            return x + x;
        }
        if y == 1.0 {
            return x;
        }
        if y == 2.0 {
            return x * x;
        }
        if y == -1.0 {
            return 1.0 / x;
        }
        if y == 0.0 {
            return 1.0;
        }
    }
    if ((hx > 0 && hx < 0x7ff00000) || (hx == 0 && lx != 0)) && qy < 0x4ff00000 {
        // Finite x > 0: e^(y log x) in double-double.
        let mut y = y;
        const TWO_M64: f64 = f64::from_bits(0x3bf0000000000000);
        if y.abs() < TWO_M64 {
            // The exact value of a y this small no longer matters.
            y = if y < 0.0 { -TWO_M64 } else { TWO_M64 };
        }
        let (z, aa) = pow_log(x);
        let t = y * CN;
        let y1 = t - (t - y);
        let y2 = y - y1;
        let t = z * CN;
        let a1 = t - (t - z);
        let a2 = (z - a1) + aa;
        let a = y1 * a1;
        let aa = y * a2 + y2 * a1;
        let a1 = a + aa;
        let a2 = (a - a1) + aa;
        let r = exp1(a1, a2);
        return if r.is_infinite() {
            HUGE * HUGE
        } else if r == 0.0 {
            TINY * TINY
        } else {
            r
        };
    }

    if x == 0.0 {
        if qy > 0x7ff00000 || (qy == 0x7ff00000 && ly != 0) {
            return y + y; // NaN
        }
        if y.abs() > 1.0e20 {
            return if y > 0.0 { 0.0 } else { f64::INFINITY };
        }
        if checkint(y) == -1 {
            return if y < 0.0 { 1.0 / x } else { x };
        }
        return if y < 0.0 { f64::INFINITY } else { 0.0 };
    }
    if qx >= 0x7ff00000 && (qx > 0x7ff00000 || lx != 0) {
        return x + y; // NaN
    }
    if qy >= 0x7ff00000 && (qy > 0x7ff00000 || ly != 0) {
        return if x == 1.0 && !is_signaling(y) {
            1.0
        } else {
            y + y
        };
    }
    if hx < 0 {
        let k = checkint(y);
        if k == 0 {
            if qy == 0x7ff00000 {
                if x == -1.0 {
                    return 1.0;
                }
                if x > -1.0 {
                    return if hy < 0 { f64::INFINITY } else { 0.0 };
                }
                return if hy < 0 { 0.0 } else { f64::INFINITY };
            }
            if qx == 0x7ff00000 {
                return if y < 0.0 { 0.0 } else { f64::INFINITY };
            }
            return (x - x) / (x - x); // y not an integer and x < 0
        }
        if qx == 0x7ff00000 {
            if k < 0 {
                return if y < 0.0 { -0.0 } else { f64::NEG_INFINITY };
            }
            return if y < 0.0 { 0.0 } else { f64::INFINITY };
        }
        if k == 1 {
            return pow(-x, y);
        }
        let r = -pow(-x, y);
        return if r.is_infinite() {
            -HUGE * HUGE
        } else if r == 0.0 {
            -TINY * TINY
        } else {
            r
        };
    }
    // x > 0
    if qx == 0x7ff00000 {
        return if y > 0.0 { x } else { 0.0 };
    }
    if qy > 0x45f00000 && qy < 0x7ff00000 {
        if x == 1.0 {
            return 1.0;
        }
        if y > 0.0 {
            return if x > 1.0 { HUGE * HUGE } else { TINY * TINY };
        }
        if y < 0.0 {
            return if x < 1.0 { HUGE * HUGE } else { TINY * TINY };
        }
    }
    if x == 1.0 {
        return 1.0;
    }
    if y > 0.0 {
        return if x > 1.0 { f64::INFINITY } else { 0.0 };
    }
    if y < 0.0 {
        return if x < 1.0 { f64::INFINITY } else { 0.0 };
    }
    0.0
}

/// e^x - 1, bit-identical to glibc 2.28 `expm1` (fdlibm `s_expm1.c`).
pub fn expm1(x: f64) -> f64 {
    const O_THRESHOLD: f64 = f64::from_bits(0x40862e42fefa39ef);
    const LN2_HI: f64 = f64::from_bits(0x3fe62e42fee00000);
    const LN2_LO: f64 = f64::from_bits(0x3dea39ef35793c76);
    const INVLN2: f64 = f64::from_bits(0x3ff71547652b82fe);
    const Q1: f64 = f64::from_bits(0xbfa11111111110f4);
    const Q2: f64 = f64::from_bits(0x3f5a01a019fe5585);
    const Q3: f64 = f64::from_bits(0xbf14ce199eaadbb7);
    const Q4: f64 = f64::from_bits(0x3ed0cfca86e65239);
    const Q5: f64 = f64::from_bits(0xbe8afdb76e09c32d);

    let negative = high(x) < 0;
    let hx = high(x) & 0x7fffffff;

    if hx >= 0x4043687a {
        // |x| >= 56 ln 2
        if hx >= 0x40862e42 {
            if hx >= 0x7ff00000 {
                if (hx & 0xfffff) | low(x) != 0 {
                    return x + x; // NaN
                }
                return if negative { -1.0 } else { x };
            }
            if x > O_THRESHOLD {
                return HUGE * HUGE;
            }
        }
        if negative {
            return TINY - 1.0;
        }
    }

    let (x, c, k) = if hx > 0x3fd62e42 {
        // |x| > ln 2 / 2
        let (hi, lo, k) = if hx < 0x3ff0a2b2 {
            if negative {
                (x + LN2_HI, -LN2_LO, -1)
            } else {
                (x - LN2_HI, LN2_LO, 1)
            }
        } else {
            let k = (INVLN2 * x + if negative { -0.5 } else { 0.5 }) as i32;
            let t = f64::from(k);
            ((-t).mul_add(LN2_HI, x), t * LN2_LO, k)
        };
        let r = hi - lo;
        (r, (hi - r) - lo, k)
    } else if hx < 0x3c900000 {
        return x; // |x| < 2^-54
    } else {
        (x, 0.0, 0)
    };

    let hfx = 0.5 * x;
    let hxs = x * hfx;
    let r2 = hxs.mul_add(Q3, Q2);
    let r1 = hxs.mul_add(Q1, 1.0);
    let h2 = hxs * hxs;
    let r3 = hxs.mul_add(Q5, Q4);
    let r1 = (h2 * h2).mul_add(r3, h2.mul_add(r2, r1));
    let t = (-hfx).mul_add(r1, 3.0);
    let e = ((r1 - t) / (-x).mul_add(t, 6.0)) * hxs;
    if k == 0 {
        return x - x.mul_add(e, -hxs);
    }
    let e = (e - c).mul_add(x, -c) - hxs;
    if k == -1 {
        return (x - e).mul_add(0.5, -0.5);
    }
    if k == 1 {
        if x < -0.25 {
            return (e - (x + 0.5)) * -2.0;
        }
        return (x - e).mul_add(2.0, 1.0);
    }
    let scale = |y: f64| from_words(high(y).wrapping_add(k << 20), low(y));
    if k <= -2 || k > 56 {
        return scale(1.0 - (e - x)) - 1.0;
    }
    if k < 20 {
        return scale(from_high(0x3ff00000 - (0x200000 >> k)) - (e - x));
    }
    scale((x - (e + from_high((0x3ff - k) << 20))) + 1.0)
}

/// Hyperbolic tangent, bit-identical to glibc 2.28 `tanh` (fdlibm `s_tanh.c`).
pub fn tanh(x: f64) -> f64 {
    let jx = high(x);
    let ix = jx & 0x7fffffff;

    if ix >= 0x7ff00000 {
        return if jx >= 0 {
            1.0 / x + 1.0
        } else {
            1.0 / x - 1.0
        };
    }
    let z = if ix < 0x40360000 {
        // |x| < 22
        if ix | low(x) == 0 {
            return x;
        }
        if ix < 0x3c800000 {
            return x * (1.0 + x); // |x| < 2^-55
        }
        if ix >= 0x3ff00000 {
            let t = expm1(x.abs() + x.abs());
            1.0 - 2.0 / (t + 2.0)
        } else {
            let t = expm1(-2.0 * x.abs());
            -t / (t + 2.0)
        }
    } else {
        1.0
    };
    if jx >= 0 { z } else { -z }
}

/// 0 if `iy` is not an integer, 1 if odd, 2 if even (e_powf.c `checkint`).
fn checkintf(iy: u32) -> i32 {
    let e = (iy >> 23 & 0xff) as i32;
    if e < 0x7f {
        return 0;
    }
    if e > 0x7f + 23 {
        return 2;
    }
    if iy & ((1 << (0x7f + 23 - e)) - 1) != 0 {
        return 0;
    }
    if iy & (1 << (0x7f + 23 - e)) != 0 {
        1
    } else {
        2
    }
}

fn zeroinfnan(i: u32) -> bool {
    (i << 1).wrapping_sub(1) >= (0x7f800000 << 1) - 1
}

fn is_signaling_f32(x: f32) -> bool {
    (x.to_bits() ^ 0x00400000) << 1 > 0x7fc00000 << 1
}

/// log2 of a positive binary32 pattern (subnormals pre-normalized to a
/// negative exponent), e_powf.c `log2_inline`.
fn powf_log2(ix: u32) -> f64 {
    const A0: f64 = f64::from_bits(0x3fd27616c9496e0b);
    const A1: f64 = f64::from_bits(0xbfd71969a075c67a);
    const A2: f64 = f64::from_bits(0x3fdec70a6ca7badd);
    const A3: f64 = f64::from_bits(0xbfe7154748bef6c8);
    const A4: f64 = f64::from_bits(0x3ff71547652ab82b);
    let tmp = ix.wrapping_sub(0x3f330000);
    let i = ((tmp >> 19) % 16) as usize;
    let top = tmp & 0xff800000;
    let iz = ix.wrapping_sub(top);
    let k = (top as i32) >> 23;
    let (invc, logc) = (
        table(&POWF_LOG2_TAB, 2 * i),
        table(&POWF_LOG2_TAB, 2 * i + 1),
    );
    let z = f64::from(f32::from_bits(iz));
    let r = z.mul_add(invc, -1.0);
    let y0 = logc + f64::from(k);
    let r2 = r * r;
    let y = r.mul_add(A0, A1);
    let p = r.mul_add(A2, A3);
    let r4 = r2 * r2;
    let q = r.mul_add(A4, y0);
    let q = r2.mul_add(p, q);
    y.mul_add(r4, q)
}

/// 2^xd with the sign in `sign_bias`, e_powf.c `exp2_inline`.
fn powf_exp2(xd: f64, sign_bias: u64) -> f64 {
    const SHIFT: f64 = 211106232532992.0; // 0x1.8p52 / 32
    const C0: f64 = f64::from_bits(0x3fac6af84b912394);
    const C1: f64 = f64::from_bits(0x3fcebfce50fac4f3);
    const C2: f64 = f64::from_bits(0x3fe62e42ff0c52d6);
    let kd = xd + SHIFT;
    let ki = kd.to_bits();
    let kd = kd - SHIFT;
    let r = xd - kd;
    let t = EXP2F_TAB[(ki % 32) as usize].wrapping_add(ki.wrapping_add(sign_bias) << 47);
    let s = f64::from_bits(t);
    let z = r.mul_add(C0, C1);
    let r2 = r * r;
    let y = r.mul_add(C2, 1.0);
    z.mul_add(r2, y) * s
}

/// x^y, bit-identical to glibc 2.28 `powf` (the generic `e_powf.c`,
/// `powf@@GLIBC_2.27`).
pub fn powf(x: f32, y: f32) -> f32 {
    const SIGN_BIAS: u64 = 1 << 16;
    let mut sign_bias = 0;
    let mut ix = x.to_bits();
    let iy = y.to_bits();
    if ix.wrapping_sub(0x00800000) >= 0x7f800000 - 0x00800000 || zeroinfnan(iy) {
        // x < 2^-126, inf or NaN, or y zero, inf or NaN.
        if zeroinfnan(iy) {
            if iy << 1 == 0 {
                return if is_signaling_f32(x) { x + y } else { 1.0 };
            }
            if ix == 0x3f800000 {
                return if is_signaling_f32(y) { x + y } else { 1.0 };
            }
            if ix << 1 > 0x7f800000 << 1 || iy << 1 > 0x7f800000 << 1 {
                return x + y;
            }
            if ix << 1 == 0x3f800000 << 1 {
                return 1.0;
            }
            if (ix << 1 < 0x3f800000 << 1) == (iy & 0x80000000 == 0) {
                return 0.0; // |x| < 1 and y == inf, or |x| > 1 and y == -inf
            }
            return y * y;
        }
        if zeroinfnan(ix) {
            let mut x2 = x * x;
            if ix & 0x80000000 != 0 && checkintf(iy) == 1 {
                x2 = -x2;
            }
            return if iy & 0x80000000 != 0 { 1.0 / x2 } else { x2 };
        }
        // x and y are nonzero and finite.
        if ix & 0x80000000 != 0 {
            match checkintf(iy) {
                0 => return (x - x) / (x - x),
                1 => sign_bias = SIGN_BIAS,
                _ => {}
            }
            ix &= 0x7fffffff;
        }
        if ix < 0x00800000 {
            // Normalize a subnormal x so its exponent becomes negative.
            ix = (x * 8388608.0).to_bits() & 0x7fffffff;
            ix -= 23 << 23;
        }
    }
    let ylogx = f64::from(y) * powf_log2(ix);
    if (ylogx.to_bits() >> 47 & 0xffff) >= 126.0f64.to_bits() >> 47 {
        // |y log2 x| >= 126
        let negative = sign_bias != 0;
        if ylogx > f64::from_bits(0x405fffffffd1d571) {
            return if negative {
                f32::NEG_INFINITY
            } else {
                f32::INFINITY
            };
        }
        if ylogx <= -150.0 {
            return if negative { -0.0 } else { 0.0 };
        }
        if ylogx < -149.0 {
            let tiny = f32::from_bits(1);
            return if negative { -tiny } else { tiny };
        }
    }
    powf_exp2(ylogx, sign_bias) as f32
}

/// The remainder of x / y with the sign of x, bit-identical to glibc 2.28
/// `fmod` (`dbl-64/wordsize-64/e_fmod.c` behind `w_fmod_compat.c`). The result
/// is exact, so only NaN bits could differ between implementations.
pub fn fmod(x: f64, y: f64) -> f64 {
    let sx = x.to_bits() & 1 << 63;
    let mut hx = x.to_bits() ^ sx;
    let mut hy = y.to_bits() & !(1 << 63);
    let zero = f64::from_bits(sx);
    if hy == 0 || hx >= 0x7ff0_0000_0000_0000 || hy > 0x7ff0_0000_0000_0000 {
        // glibc returns (x*y)/(x*y), and its wrapper's __kernel_standard
        // (x inf or y zero, neither NaN) 0/0. On POWER that is x quieted if x
        // is NaN, else y quieted if y is NaN, else the default NaN. The bits
        // are built here because LLVM need not preserve NaN payloads.
        let quiet = |v: f64| f64::from_bits(v.to_bits() | 1 << 51);
        return if x.is_nan() {
            quiet(x)
        } else if y.is_nan() {
            quiet(y)
        } else {
            f64::from_bits(0x7ff8_0000_0000_0000)
        };
    }
    if hx <= hy {
        return if hx < hy { x } else { zero };
    }
    // ilogb of |x| and |y|.
    let ilogb = |h: u64| {
        if h < 1 << 52 {
            -1022 - (h << 11).leading_zeros() as i32
        } else {
            (h >> 52) as i32 - 1023
        }
    };
    let ix = ilogb(hx);
    let mut iy = ilogb(hy);
    let mantissa = |h: u64, i: i32| {
        if i >= -1022 {
            1 << 52 | (h & ((1 << 52) - 1))
        } else {
            h << (-1022 - i)
        }
    };
    hx = mantissa(hx, ix);
    hy = mantissa(hy, iy);
    // Fixed-point remainder.
    for _ in 0..ix - iy {
        match hx.cmp(&hy) {
            core::cmp::Ordering::Less => hx += hx,
            core::cmp::Ordering::Equal => return zero,
            core::cmp::Ordering::Greater => hx = 2 * (hx - hy),
        }
    }
    if hx >= hy {
        hx -= hy;
    }
    if hx == 0 {
        return zero;
    }
    while hx < 1 << 52 {
        hx += hx;
        iy -= 1;
    }
    if iy >= -1022 {
        f64::from_bits((hx - (1 << 52)) | ((iy + 1023) as u64) << 52 | sx)
    } else {
        f64::from_bits(hx >> (-1022 - iy) | sx)
    }
}
