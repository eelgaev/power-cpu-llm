//! Portable reference form of the binary32 attention contract (formerly
//! `native/attention_f32.c`): sixteen fused-multiply-add score lanes reduced
//! as a fixed tree, binary32 softmax with glibc 2.28's POWER `expf` (the
//! bit-exact port in `power-libm`).

use std::slice::{from_raw_parts, from_raw_parts_mut};

/// The softmax `expf` for the POWER9 attention assembly (`bl power_expf`).
#[unsafe(no_mangle)]
pub extern "C" fn power_expf(x: f32) -> f32 {
    crate::libm::expf(x)
}

/// Softmax over allowed scores in place; false on a nonfinite allowed score.
pub(crate) fn softmax(scores: &mut [f32], allowed: Option<&[u8]>) -> bool {
    let masked = |t: usize| allowed.is_some_and(|a| a[t] == 0);
    let mut max = f32::NEG_INFINITY;
    for (t, &s) in scores.iter().enumerate() {
        if masked(t) {
            continue;
        }
        if !s.is_finite() {
            return false;
        }
        if s > max {
            max = s;
        }
    }
    let mut sum = 0.0f32;
    for t in 0..scores.len() {
        let e = if masked(t) {
            0.0
        } else {
            crate::libm::expf(scores[t] - max)
        };
        scores[t] = e;
        sum += e;
    }
    for s in scores.iter_mut() {
        *s /= sum;
    }
    true
}

/// Query heads [h_first, h_end) of one query position.
///
/// # Safety
/// query [qheads, dim], keys/values [nkeys, kvheads, dim], allowed null or
/// [nkeys], scratch [nkeys], out [(h_end - h_first), dim].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pa_attention_f32_heads(
    query: *const f32,
    keys: *const f32,
    values: *const f32,
    qheads: usize,
    kvheads: usize,
    dim: usize,
    nkeys: usize,
    allowed: *const u8,
    scratch: *mut f32,
    out: *mut f32,
    h_first: usize,
    h_end: usize,
) -> bool {
    if h_first > h_end || h_end > qheads {
        return false;
    }
    if !dim.is_multiple_of(16) || kvheads == 0 || !qheads.is_multiple_of(kvheads) {
        return false;
    }
    let group = qheads / kvheads;
    let stride = kvheads * dim;
    // SAFETY: caller contract.
    let (query, keys, values, scratch, out) = unsafe {
        (
            from_raw_parts(query, qheads * dim),
            from_raw_parts(keys, nkeys * stride),
            from_raw_parts(values, nkeys * stride),
            from_raw_parts_mut(scratch, nkeys),
            from_raw_parts_mut(out, (h_end - h_first) * dim),
        )
    };
    let allowed = (!allowed.is_null()).then(|| unsafe { from_raw_parts(allowed, nkeys) });
    for h in h_first..h_end {
        let q = &query[h * dim..][..dim];
        let kh = h / group;
        for t in 0..nkeys {
            let k = &keys[t * stride + kh * dim..][..dim];
            let mut p = [0.0f32; 16];
            for d in 0..dim {
                p[d % 16] = q[d].mul_add(k[d], p[d % 16]);
            }
            let v: [f32; 4] = std::array::from_fn(|l| (p[l] + p[4 + l]) + (p[8 + l] + p[12 + l]));
            scratch[t] = (v[0] + v[1]) + (v[2] + v[3]);
        }
        if !softmax(scratch, allowed) {
            return false;
        }
        let o = &mut out[(h - h_first) * dim..][..dim];
        o.fill(0.0);
        for t in 0..nkeys {
            let vt = &values[t * stride + kh * dim..][..dim];
            for d in 0..dim {
                o[d] = scratch[t].mul_add(vt[d], o[d]);
            }
        }
        if !o.iter().all(|x| x.is_finite()) {
            return false;
        }
    }
    true
}
