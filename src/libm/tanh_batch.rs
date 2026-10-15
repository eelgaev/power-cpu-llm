// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Sun Microsystems / fdlibm notice. This file derives from fdlibm's `s_tanh.c`
// and `s_expm1.c` as shipped in glibc 2.28 (`expm1` includes glibc's 1997
// modifications by Naohiko Shimizu/Tokai University):
//
// ====================================================
// Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
//
// Developed at SunPro, a Sun Microsystems, Inc. business.
// Permission to use, copy, modify, and distribute this
// software is freely granted, provided that this notice
// is preserved.
// ====================================================
//! Batched [`tanh`](super::tanh) for the GELU tail.
//!
//! Every element gets exactly the operations `tanh` (fdlibm `s_tanh.c` over
//! `s_expm1.c`, as ported in the parent module) performs, in the same order
//! and with the same fused multiply-adds, so the bits are identical. What
//! changes is control flow: the scalar function branches on the exponent
//! range of the argument and on the range-reduction factor `k`, and on real
//! activations those branches are unpredictable. Here a first pass sorts the
//! element indices into classes of identical control flow and a second pass
//! runs each class as straight-line code, which the core can overlap across
//! elements (including the two long divides per element).

use super::{from_high, from_words, high, low, tanh};

const LN2_HI: f64 = f64::from_bits(0x3fe62e42fee00000);
const LN2_LO: f64 = f64::from_bits(0x3dea39ef35793c76);
const INVLN2: f64 = f64::from_bits(0x3ff71547652b82fe);
const Q1: f64 = f64::from_bits(0xbfa11111111110f4);
const Q2: f64 = f64::from_bits(0x3f5a01a019fe5585);
const Q3: f64 = f64::from_bits(0xbf14ce199eaadbb7);
const Q4: f64 = f64::from_bits(0x3ed0cfca86e65239);
const Q5: f64 = f64::from_bits(0xbe8afdb76e09c32d);
const SIGN: u64 = 1 << 63;

/// Elements per call of [`tanh_block`].
pub const BLOCK: usize = 256;

// Classes in order of increasing |y|; a class id is the number of
// `THRESHOLDS` that |y|'s bit pattern has reached.
const ZERO: usize = 0;
const TINY: usize = 1;
const SMALL_K0: usize = 2;
const SMALL_KM1: usize = 3;
const GAP: usize = 4;
const SMALL_KM2: usize = 5;
const BIG_MID: usize = 6;
const BIG_HIGH: usize = 7;
const BIG_HUGE: usize = 8;
const SATURATED: usize = 9;
const OTHER: usize = 10;
const CLASSES: usize = 11;

/// Least |y| bit pattern in `[lo, hi]` for which `reached(kind, |y|)` holds;
/// it is monotone in the pattern (false then true) and holds at `hi`.
const fn first_bits(lo: u64, hi: u64, kind: u8) -> u64 {
    let (mut lo, mut hi) = (lo, hi);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if reached(kind, f64::from_bits(mid)) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

/// The `k` tests below use the expressions the class bodies use.
const fn reached(kind: u8, a: f64) -> bool {
    match kind {
        // k <= -2 for expm1(-2a) (`small_km2`).
        0 => ((INVLN2 * (-2.0 * a) + -0.5) as i32) <= -2,
        // k >= 20 and k >= 57 for expm1(a + a) (`big_*`).
        1 => ((INVLN2 * (a + a) + 0.5) as i32) >= 20,
        _ => ((INVLN2 * (a + a) + 0.5) as i32) >= 57,
    }
}

const ONE: u64 = 0x3ff0_0000_0000_0000;

const THRESHOLDS: [u64; 10] = [
    1,                                         // 0 < |y|: not zero
    0x3c80_0000_0000_0000,                     // |y| >= 2^-55: not tiny
    0x3fc6_2e43_0000_0000,                     // expm1 argument beyond ln2/2
    0x3fe0_a2b2_0000_0000,                     // expm1 argument at 1.5 ln2
    first_bits(0x3fe0_0000_0000_0000, ONE, 0), // k <= -2 guaranteed
    ONE,                                       // |y| >= 1
    first_bits(ONE, 0x4036_0000_0000_0000, 1),
    first_bits(ONE, 0x4036_0000_0000_0000, 2),
    0x4036_0000_0000_0000, // |y| >= 22: saturated
    0x7ff0_0000_0000_0000, // inf, NaN
];
const PADDED: [u64; 16] = {
    let mut t = [u64::MAX; 16];
    let mut i = 0;
    while i < THRESHOLDS.len() {
        t[i] = THRESHOLDS[i];
        i += 1;
    }
    t
};
const _: () = {
    let mut i = 1;
    while i < THRESHOLDS.len() {
        assert!(THRESHOLDS[i - 1] < THRESHOLDS[i]);
        i += 1;
    }
};

/// `(hxs, e)` of expm1's polynomial core for a reduced argument `x`.
#[inline(always)]
fn core(x: f64) -> (f64, f64) {
    let hfx = 0.5 * x;
    let hxs = x * hfx;
    let r2 = hxs.mul_add(Q3, Q2);
    let r1 = hxs.mul_add(Q1, 1.0);
    let h2 = hxs * hxs;
    let r3 = hxs.mul_add(Q5, Q4);
    let r1 = (h2 * h2).mul_add(r3, h2.mul_add(r2, r1));
    let t = (-hfx).mul_add(r1, 3.0);
    let e = ((r1 - t) / (-x).mul_add(t, 6.0)) * hxs;
    (hxs, e)
}

#[inline(always)]
fn scale(y: f64, k: i32) -> f64 {
    from_words(high(y).wrapping_add(k << 20), low(y))
}

/// Reduction by `k * ln2` for |k| >= 2: `(r, c)`.
#[inline(always)]
fn reduce_k(x: f64, k: i32) -> (f64, f64) {
    let t = f64::from(k);
    let hi = (-t).mul_add(LN2_HI, x);
    let lo = t * LN2_LO;
    let r = hi - lo;
    (r, (hi - r) - lo)
}

#[inline(always)]
fn with_sign(z: f64, y: f64) -> f64 {
    f64::from_bits(z.to_bits() ^ (y.to_bits() & SIGN))
}

/// Runs `f` over `y[i]` for every `i` in `list`, four independent elements
/// at a time so their long dependency chains overlap.
#[inline(always)]
fn run(list: &[u16], y: &[f64], out: &mut [f64], f: impl Fn(f64) -> f64) {
    let (quads, rest) = list.as_chunks::<4>();
    for q in quads {
        let i = [
            usize::from(q[0]),
            usize::from(q[1]),
            usize::from(q[2]),
            usize::from(q[3]),
        ];
        let v = [y[i[0]], y[i[1]], y[i[2]], y[i[3]]];
        let r = [f(v[0]), f(v[1]), f(v[2]), f(v[3])];
        out[i[0]] = r[0];
        out[i[1]] = r[1];
        out[i[2]] = r[2];
        out[i[3]] = r[3];
    }
    for &i in rest {
        let i = usize::from(i);
        out[i] = f(y[i]);
    }
}

/// |y| in [2^-55, 1) with k = 0: t = expm1(-2|y|), tanh = -t / (t + 2).
#[inline(always)]
fn small_k0(y: f64) -> f64 {
    let x = -2.0 * y.abs();
    let (hxs, e) = core(x);
    let t = x - x.mul_add(e, -hxs);
    with_sign(-t / (t + 2.0), y)
}

/// As [`small_k0`] with k = -1.
#[inline(always)]
fn small_km1(y: f64) -> f64 {
    let x = -2.0 * y.abs();
    let hi = x + LN2_HI;
    let lo = -LN2_LO;
    let r = hi - lo;
    let c = (hi - r) - lo;
    let (hxs, e) = core(r);
    let e = (e - c).mul_add(r, -c) - hxs;
    let t = (r - e).mul_add(0.5, -0.5);
    with_sign(-t / (t + 2.0), y)
}

/// As [`small_k0`] with k <= -2 (classified so).
#[inline(always)]
fn small_km2(y: f64) -> f64 {
    let x = -2.0 * y.abs();
    let k = (INVLN2 * x + -0.5) as i32;
    let (r, c) = reduce_k(x, k);
    let (hxs, e) = core(r);
    let e = (e - c).mul_add(r, -c) - hxs;
    let t = scale(1.0 - (e - r), k) - 1.0;
    with_sign(-t / (t + 2.0), y)
}

/// |y| in [1, 22): t = expm1(2|y|) with 3 <= k < 20; tanh = 1 - 2 / (t + 2).
#[inline(always)]
fn big_mid(y: f64) -> f64 {
    let x = y.abs() + y.abs();
    let k = (INVLN2 * x + 0.5) as i32;
    let (r, c) = reduce_k(x, k);
    let (hxs, e) = core(r);
    let e = (e - c).mul_add(r, -c) - hxs;
    let t = scale(from_high(0x3ff0_0000 - (0x20_0000 >> k)) - (e - r), k);
    with_sign(1.0 - 2.0 / (t + 2.0), y)
}

/// As [`big_mid`] with 20 <= k <= 56.
#[inline(always)]
fn big_high(y: f64) -> f64 {
    let x = y.abs() + y.abs();
    let k = (INVLN2 * x + 0.5) as i32;
    let (r, c) = reduce_k(x, k);
    let (hxs, e) = core(r);
    let e = (e - c).mul_add(r, -c) - hxs;
    let t = scale((r - (e + from_high((0x3ff - k) << 20))) + 1.0, k);
    with_sign(1.0 - 2.0 / (t + 2.0), y)
}

/// As [`big_mid`] with k > 56.
#[inline(always)]
fn big_huge(y: f64) -> f64 {
    let x = y.abs() + y.abs();
    let k = (INVLN2 * x + 0.5) as i32;
    let (r, c) = reduce_k(x, k);
    let (hxs, e) = core(r);
    let e = (e - c).mul_add(r, -c) - hxs;
    let t = scale(1.0 - (e - r), k) - 1.0;
    with_sign(1.0 - 2.0 / (t + 2.0), y)
}

/// `tanh(y[i])` into `out[i]`, bit-identical to [`tanh`], for
/// `y.len() <= BLOCK`.
pub fn tanh_block(y: &[f64], out: &mut [f64]) {
    let m = y.len();
    assert!(m <= BLOCK && out.len() >= m);
    let mut lists = [[0u16; BLOCK]; CLASSES];
    let mut count = [0usize; CLASSES];
    for (i, &v) in y.iter().enumerate() {
        let a = v.to_bits() & !SIGN;
        // Number of thresholds reached: branch-free binary search over the
        // table padded to 16 entries.
        let mut cls = 0;
        for step in [8, 4, 2, 1] {
            cls += step * usize::from(a >= PADDED[cls + step - 1]);
        }
        lists[cls][count[cls]] = i as u16;
        count[cls] += 1;
    }
    let list = |c: usize| &lists[c][..count[c]];
    for &i in list(ZERO) {
        let i = usize::from(i);
        out[i] = y[i];
    }
    for &i in list(TINY) {
        let i = usize::from(i);
        out[i] = y[i] * (1.0 + y[i]);
    }
    for &i in list(SATURATED) {
        let i = usize::from(i);
        out[i] = with_sign(1.0, y[i]);
    }
    for &i in list(OTHER).iter().chain(list(GAP)) {
        let i = usize::from(i);
        out[i] = tanh(y[i]);
    }
    run(list(SMALL_K0), y, out, small_k0);
    run(list(SMALL_KM1), y, out, small_km1);
    run(list(SMALL_KM2), y, out, small_km2);
    run(list(BIG_MID), y, out, big_mid);
    run(list(BIG_HIGH), y, out, big_high);
    run(list(BIG_HUGE), y, out, big_huge);
}
