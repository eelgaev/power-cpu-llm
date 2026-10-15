//! Portable F32 tensor references with F64 reductions (formerly
//! `native/reference.c` and `native/attention.c`). Rust callers validate
//! dimensions and finite inputs. RMS scale is direct, not Gemma's older
//! (1 + weight). Transcendentals (`exp`, `tanh`, `powf`, `sincosf`) are the
//! bit-exact `crate::libm` ports of the C library's, as before.

use std::slice::{from_raw_parts, from_raw_parts_mut};

/// # Safety
/// `x`, `out` hold `n` floats; `scale` is null or holds `n`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_ref_rms_norm(
    x: *const f32,
    scale: *const f32,
    n: usize,
    eps: f32,
    out: *mut f32,
) {
    // SAFETY: caller contract.
    let (x, out) = unsafe { (from_raw_parts(x, n), from_raw_parts_mut(out, n)) };
    let scale = (!scale.is_null()).then(|| unsafe { from_raw_parts(scale, n) });
    let mut sum = 0.0f64;
    for &v in x {
        sum += f64::from(v) * f64::from(v);
    }
    let inv = 1.0 / (sum / n as f64 + f64::from(eps)).sqrt();
    for i in 0..n {
        let normed = (f64::from(x[i]) * inv) as f32;
        out[i] = match scale {
            Some(s) => normed * s[i],
            None => normed,
        };
    }
}

/// One F64 sum per row in ascending column order; eight rows advance
/// together as independent chains.
///
/// # Safety
/// `matrix` [rows, cols], `x` [cols], `out` [rows].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_ref_matvec(
    matrix: *const f32,
    x: *const f32,
    rows: usize,
    cols: usize,
    out: *mut f32,
) {
    // SAFETY: caller contract.
    let (m, x, out) = unsafe {
        (
            from_raw_parts(matrix, rows * cols),
            from_raw_parts(x, cols),
            from_raw_parts_mut(out, rows),
        )
    };
    let mut row = 0;
    while row + 8 <= rows {
        let mut s = [0.0f64; 8];
        for col in 0..cols {
            let v = f64::from(x[col]);
            for (k, sk) in s.iter_mut().enumerate() {
                *sk += f64::from(m[(row + k) * cols + col]) * v;
            }
        }
        for (k, sk) in s.into_iter().enumerate() {
            out[row + k] = sk as f32;
        }
        row += 8;
    }
    while row < rows {
        let mut sum = 0.0f64;
        for col in 0..cols {
            sum += f64::from(m[row * cols + col]) * f64::from(x[col]);
        }
        out[row] = sum as f32;
        row += 1;
    }
}

/// `p9_ref_matvec` for several inputs, bit for bit (4 rows x 4 inputs).
///
/// # Safety
/// `matrix` [rows, cols], `x` [inputs, cols], `out` [inputs, rows].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_ref_matvec_multi(
    matrix: *const f32,
    x: *const f32,
    inputs: usize,
    rows: usize,
    cols: usize,
    out: *mut f32,
) {
    // SAFETY: caller contract.
    let (m, xs, o) = unsafe {
        (
            from_raw_parts(matrix, rows * cols),
            from_raw_parts(x, inputs * cols),
            from_raw_parts_mut(out, inputs * rows),
        )
    };
    let mut p = 0;
    while p + 4 <= inputs {
        let mut row = 0;
        while row + 4 <= rows {
            let mut s = [[0.0f64; 4]; 4]; // [row][input]
            for col in 0..cols {
                let a: [f64; 4] = std::array::from_fn(|i| f64::from(xs[(p + i) * cols + col]));
                let w: [f64; 4] = std::array::from_fn(|r| f64::from(m[(row + r) * cols + col]));
                for r in 0..4 {
                    for i in 0..4 {
                        s[r][i] += w[r] * a[i];
                    }
                }
            }
            for r in 0..4 {
                for i in 0..4 {
                    o[(p + i) * rows + row + r] = s[r][i] as f32;
                }
            }
            row += 4;
        }
        while row < rows {
            for i in 0..4 {
                let mut sum = 0.0f64;
                for col in 0..cols {
                    sum += f64::from(m[row * cols + col]) * f64::from(xs[(p + i) * cols + col]);
                }
                o[(p + i) * rows + row] = sum as f32;
            }
            row += 1;
        }
        p += 4;
    }
    while p < inputs {
        // SAFETY: input p and output p lie inside the buffers above.
        unsafe { p9_ref_matvec(matrix, x.add(p * cols), rows, cols, out.add(p * rows)) };
        p += 1;
    }
}

/// accumulator[col] += weights[row] * matrix[row][col], rows ascending.
///
/// # Safety
/// `matrix` [rows, cols], `weights` [rows], `accumulator` [cols].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_ref_accumulate_transposed(
    matrix: *const f32,
    weights: *const f32,
    rows: usize,
    cols: usize,
    accumulator: *mut f64,
) {
    // SAFETY: caller contract.
    let (m, w, acc) = unsafe {
        (
            from_raw_parts(matrix, rows * cols),
            from_raw_parts(weights, rows),
            from_raw_parts_mut(accumulator, cols),
        )
    };
    for row in 0..rows {
        for col in 0..cols {
            acc[col] += f64::from(w[row]) * f64::from(m[row * cols + col]);
        }
    }
}

/// # Safety
/// All buffers hold `n` floats.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_ref_gelu_product(
    gate: *const f32,
    up: *const f32,
    n: usize,
    out: *mut f32,
) {
    // SAFETY: caller contract.
    let (gate, up, out) = unsafe {
        (
            from_raw_parts(gate, n),
            from_raw_parts(up, n),
            from_raw_parts_mut(out, n),
        )
    };
    let mut y = [0.0f64; crate::libm::TANH_BLOCK];
    let mut t = [0.0f64; crate::libm::TANH_BLOCK];
    let mut start = 0;
    while start < n {
        let m = (n - start).min(crate::libm::TANH_BLOCK);
        for j in 0..m {
            let g = f64::from(gate[start + j]);
            y[j] = 0.7978845608028654 * (g + 0.044715 * g * g * g);
        }
        crate::libm::tanh_block(&y[..m], &mut t[..m]);
        for j in 0..m {
            let g = f64::from(gate[start + j]);
            let gelu = 0.5 * g * (1.0 + t[j]);
            out[start + j] = gelu as f32 * up[start + j];
        }
        start += m;
    }
}

/// # Safety
/// `x`, `out` hold `n >= 1` floats.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_ref_softmax(x: *const f32, n: usize, out: *mut f32) {
    // SAFETY: caller contract.
    let (x, out) = unsafe { (from_raw_parts(x, n), from_raw_parts_mut(out, n)) };
    let mut max = x[0];
    for &v in &x[1..] {
        if v > max {
            max = v;
        }
    }
    let mut sum = 0.0f64;
    for &v in x {
        sum += crate::libm::exp(f64::from(v) - f64::from(max));
    }
    for i in 0..n {
        out[i] = (crate::libm::exp(f64::from(x[i]) - f64::from(max)) / sum) as f32;
    }
}

/// # Safety
/// `x`, `out` hold `n` floats.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_ref_softcap(x: *const f32, n: usize, cap: f32, out: *mut f32) {
    // SAFETY: caller contract.
    let (x, out) = unsafe { (from_raw_parts(x, n), from_raw_parts_mut(out, n)) };
    for i in 0..n {
        out[i] = (f64::from(cap) * crate::libm::tanh(f64::from(x[i]) / f64::from(cap))) as f32;
    }
}

/// # Safety
/// All buffers hold `n` floats.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_ref_mul_scaled(
    a: *const f32,
    b: *const f32,
    n: usize,
    factor: f32,
    out: *mut f32,
) {
    // SAFETY: caller contract.
    let (a, b, out) = unsafe {
        (
            from_raw_parts(a, n),
            from_raw_parts(b, n),
            from_raw_parts_mut(out, n),
        )
    };
    for i in 0..n {
        out[i] = (a[i] * b[i]) * factor;
    }
}

/// # Safety
/// All buffers hold `n` floats.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_ref_add(a: *const f32, b: *const f32, n: usize, out: *mut f32) {
    // SAFETY: caller contract.
    let (a, b, out) = unsafe {
        (
            from_raw_parts(a, n),
            from_raw_parts(b, n),
            from_raw_parts_mut(out, n),
        )
    };
    for i in 0..n {
        out[i] = a[i] + b[i];
    }
}

/// Full-head NeoX (split-half) rotation with F32 frequencies and angles.
/// Factors divide inverse frequencies. False on a nonfinite angle.
///
/// # Safety
/// `x`, `out` [heads, dim]; `factors` null or [dim / 2].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_ref_rope(
    x: *const f32,
    heads: usize,
    dim: usize,
    position: usize,
    base: f32,
    factors: *const f32,
    out: *mut f32,
) -> bool {
    let half = dim / 2;
    // SAFETY: caller contract.
    let (x, out) = unsafe {
        (
            from_raw_parts(x, heads * dim),
            from_raw_parts_mut(out, heads * dim),
        )
    };
    let factors = (!factors.is_null()).then(|| unsafe { from_raw_parts(factors, half) });
    for i in 0..half {
        let mut inv = 1.0f32 / crate::libm::powf(base, (2.0f32 * i as f32) / dim as f32);
        if let Some(f) = factors {
            inv /= f[i];
        }
        let angle = position as f32 * inv;
        if !angle.is_finite() {
            return false;
        }
        let (s, c) = crate::libm::sincosf(angle);
        for h in 0..heads {
            let (a, b) = (h * dim + i, h * dim + i + half);
            out[a] = x[a] * c - x[b] * s;
            out[b] = x[b] * c + x[a] * s;
        }
    }
    true
}

/// One query position, query heads [h_first, h_end): F64 scores and value
/// sums, F32 scores and probabilities, scale 1.
///
/// # Safety
/// query [qheads, dim], keys/values [nkeys, kvheads, dim], allowed null or
/// [nkeys], scratch [nkeys], out [(h_end - h_first), dim].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_ref_attention_heads(
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
    let group = qheads / kvheads;
    let stride = kvheads * dim;
    // SAFETY: caller contract.
    let (q, k, v, scratch, out) = unsafe {
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
        let kh = h / group;
        let mut max = f32::NEG_INFINITY;
        for t in 0..nkeys {
            scratch[t] = f32::NEG_INFINITY;
            if allowed.is_some_and(|a| a[t] == 0) {
                continue;
            }
            let mut score = 0.0f64;
            for d in 0..dim {
                score += f64::from(q[h * dim + d]) * f64::from(k[t * stride + kh * dim + d]);
            }
            scratch[t] = score as f32;
            if !scratch[t].is_finite() {
                return false;
            }
            if scratch[t] > max {
                max = scratch[t];
            }
        }
        let mut sum = 0.0f64;
        for &s in scratch.iter() {
            sum += crate::libm::exp(f64::from(s) - f64::from(max));
        }
        for s in scratch.iter_mut() {
            *s = (crate::libm::exp(f64::from(*s) - f64::from(max)) / sum) as f32;
        }
        for d in 0..dim {
            let mut result = 0.0f64;
            for t in 0..nkeys {
                result += f64::from(scratch[t]) * f64::from(v[t * stride + kh * dim + d]);
            }
            out[(h - h_first) * dim + d] = result as f32;
        }
    }
    true
}
