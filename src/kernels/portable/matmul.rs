//! Portable batched projections over encoded GGUF rows (formerly
//! `native/quantized_matmul.c`). Arithmetic matches `p9_ref_matvec` /
//! `p9_ref_accumulate_transposed` term for term; register blocking only
//! interleaves independent sums, never reassociates one.

use super::dequant::{decode, layout};
use std::ffi::c_int;

pub(crate) const PQ_OK: c_int = 0;
pub(crate) const PQ_ERR_ARGUMENT: c_int = 1;
pub(crate) const PQ_ERR_UNSUPPORTED: c_int = 2;
pub(crate) const PQ_ERR_SIZE: c_int = 3;
pub(crate) const PQ_ERR_OVERFLOW: c_int = 4;
pub(crate) const PQ_ERR_SCRATCH: c_int = 5;
pub(crate) const PQ_ERR_NONFINITE_WEIGHT: c_int = 6;
pub(crate) const PQ_ERR_NONFINITE_OUTPUT: c_int = 7;

/// Bytes of one row of `cols` elements, 0 if unsupported or not whole blocks.
#[unsafe(no_mangle)]
pub extern "C" fn pq_row_bytes(kind: c_int, cols: usize) -> usize {
    let Some((elements, bytes)) = layout(kind) else {
        return 0;
    };
    if cols == 0 || !cols.is_multiple_of(elements) || cols / elements > usize::MAX / bytes {
        return 0;
    }
    cols / elements * bytes
}

/// Validate and decode a tile into scratch (shared by both projections).
///
/// # Safety
/// `weights` holds `weight_bytes` bytes and `scratch` `scratch_len` floats.
pub(crate) unsafe fn decode_tile(
    kind: c_int,
    weights: *const u8,
    weight_bytes: usize,
    rows: usize,
    cols: usize,
    scratch: *mut f32,
    scratch_len: usize,
) -> c_int {
    if layout(kind).is_none() {
        return PQ_ERR_UNSUPPORTED;
    }
    let row_bytes = pq_row_bytes(kind, cols);
    if row_bytes == 0 {
        return PQ_ERR_SIZE;
    }
    let (Some(total), Some(needed)) = (rows.checked_mul(row_bytes), rows.checked_mul(cols)) else {
        return PQ_ERR_OVERFLOW;
    };
    if total != weight_bytes {
        return PQ_ERR_SIZE;
    }
    if scratch_len < needed {
        return PQ_ERR_SCRATCH;
    }
    // SAFETY: sizes validated against the caller's buffers.
    let (weights, scratch) = unsafe {
        (
            std::slice::from_raw_parts(weights, total),
            std::slice::from_raw_parts_mut(scratch, needed),
        )
    };
    for (w, row) in weights
        .chunks_exact(row_bytes)
        .zip(scratch.chunks_exact_mut(cols))
    {
        decode(kind, w, row);
        if !row.iter().all(|v| v.is_finite()) {
            return PQ_ERR_NONFINITE_WEIGHT;
        }
    }
    PQ_OK
}

/// out[p * out_stride + out_offset + r] = row r . x[p], one F64 sum each.
///
/// # Safety
/// The C contract of `power_llm_quantized.h`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pq_project_rows(
    kind: c_int,
    weights: *const u8,
    weight_bytes: usize,
    rows: usize,
    cols: usize,
    x: *const f32,
    positions: usize,
    out: *mut f32,
    out_stride: usize,
    out_offset: usize,
    scratch: *mut f32,
    scratch_len: usize,
) -> c_int {
    if weights.is_null()
        || x.is_null()
        || out.is_null()
        || scratch.is_null()
        || rows == 0
        || cols == 0
        || positions == 0
    {
        return PQ_ERR_ARGUMENT;
    }
    if out_offset > usize::MAX - rows {
        return PQ_ERR_OVERFLOW;
    }
    if out_offset + rows > out_stride {
        return PQ_ERR_ARGUMENT;
    }
    if positions.checked_mul(out_stride).is_none() || positions.checked_mul(cols).is_none() {
        return PQ_ERR_OVERFLOW;
    }
    // SAFETY: caller contract; decode_tile checks the tile sizes.
    let status = unsafe {
        decode_tile(
            kind,
            weights,
            weight_bytes,
            rows,
            cols,
            scratch,
            scratch_len,
        )
    };
    if status != PQ_OK {
        return status;
    }
    // SAFETY: scratch holds the decoded [rows, cols] tile, x [positions, cols].
    let (w, x) = unsafe {
        (
            std::slice::from_raw_parts(scratch, rows * cols),
            std::slice::from_raw_parts(x, positions * cols),
        )
    };
    // SAFETY: (p, r) addresses lie inside the caller's output window.
    let put = |p: usize, r: usize, v: f64| unsafe {
        *out.add(p * out_stride + out_offset + r) = v as f32
    };
    let mut p = 0;
    // 4 positions x 2 rows: eight independent sequential sums per sweep.
    while p + 4 <= positions {
        let x0 = &x[p * cols..][..cols];
        let x1 = &x[(p + 1) * cols..][..cols];
        let x2 = &x[(p + 2) * cols..][..cols];
        let x3 = &x[(p + 3) * cols..][..cols];
        let mut r = 0;
        while r + 2 <= rows {
            let w0 = &w[r * cols..][..cols];
            let w1 = &w[(r + 1) * cols..][..cols];
            let (mut s00, mut s01, mut s02, mut s03) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
            let (mut s10, mut s11, mut s12, mut s13) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
            for c in 0..cols {
                let (a, b) = (f64::from(w0[c]), f64::from(w1[c]));
                let (v0, v1, v2, v3) = (
                    f64::from(x0[c]),
                    f64::from(x1[c]),
                    f64::from(x2[c]),
                    f64::from(x3[c]),
                );
                s00 += a * v0;
                s01 += a * v1;
                s02 += a * v2;
                s03 += a * v3;
                s10 += b * v0;
                s11 += b * v1;
                s12 += b * v2;
                s13 += b * v3;
            }
            for (i, (s0, s1)) in [(s00, s10), (s01, s11), (s02, s12), (s03, s13)]
                .into_iter()
                .enumerate()
            {
                put(p + i, r, s0);
                put(p + i, r + 1, s1);
            }
            r += 2;
        }
        while r < rows {
            let w0 = &w[r * cols..][..cols];
            let (mut s0, mut s1, mut s2, mut s3) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
            for c in 0..cols {
                let a = f64::from(w0[c]);
                s0 += a * f64::from(x0[c]);
                s1 += a * f64::from(x1[c]);
                s2 += a * f64::from(x2[c]);
                s3 += a * f64::from(x3[c]);
            }
            for (i, s) in [s0, s1, s2, s3].into_iter().enumerate() {
                put(p + i, r, s);
            }
            r += 1;
        }
        p += 4;
    }
    while p < positions {
        let xp = &x[p * cols..][..cols];
        for r in 0..rows {
            let w0 = &w[r * cols..][..cols];
            let mut s = 0.0f64;
            for c in 0..cols {
                s += f64::from(w0[c]) * f64::from(xp[c]);
            }
            put(p, r, s);
        }
        p += 1;
    }
    for p in 0..positions {
        for r in 0..rows {
            // SAFETY: inside the output window written above.
            if !unsafe { *out.add(p * out_stride + out_offset + r) }.is_finite() {
                return PQ_ERR_NONFINITE_OUTPUT;
            }
        }
    }
    PQ_OK
}

/// accumulator[p * cols + c] += sum_r k[p][r] * w[r][c], rows ascending.
///
/// # Safety
/// The C contract of `power_llm_quantized.h`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pq_accumulate_transposed_rows(
    kind: c_int,
    weights: *const u8,
    weight_bytes: usize,
    rows: usize,
    cols: usize,
    coefficients: *const f32,
    positions: usize,
    coeff_stride: usize,
    coeff_offset: usize,
    accumulator: *mut f64,
    scratch: *mut f32,
    scratch_len: usize,
) -> c_int {
    if weights.is_null()
        || coefficients.is_null()
        || accumulator.is_null()
        || scratch.is_null()
        || rows == 0
        || cols == 0
        || positions == 0
    {
        return PQ_ERR_ARGUMENT;
    }
    if coeff_offset > usize::MAX - rows {
        return PQ_ERR_OVERFLOW;
    }
    if coeff_offset + rows > coeff_stride {
        return PQ_ERR_ARGUMENT;
    }
    if positions.checked_mul(coeff_stride).is_none() || positions.checked_mul(cols).is_none() {
        return PQ_ERR_OVERFLOW;
    }
    // SAFETY: caller contract; the accumulator is untouched on failure.
    let status = unsafe {
        decode_tile(
            kind,
            weights,
            weight_bytes,
            rows,
            cols,
            scratch,
            scratch_len,
        )
    };
    if status != PQ_OK {
        return status;
    }
    // SAFETY: decoded tile, [positions, cols] accumulator, coefficient window.
    let (w, acc_all) = unsafe {
        (
            std::slice::from_raw_parts(scratch, rows * cols),
            std::slice::from_raw_parts_mut(accumulator, positions * cols),
        )
    };
    // Column blocks; every element still receives its rows in ascending order.
    const COLUMN_BLOCK: usize = 256;
    let mut c0 = 0;
    while c0 < cols {
        let c1 = if cols - c0 < COLUMN_BLOCK {
            cols
        } else {
            c0 + COLUMN_BLOCK
        };
        for p in 0..positions {
            let acc = &mut acc_all[p * cols..][..cols];
            // SAFETY: k[0..rows] lies in the caller's coefficient window.
            let k = unsafe {
                std::slice::from_raw_parts(coefficients.add(p * coeff_stride + coeff_offset), rows)
            };
            let mut r = 0;
            while r + 4 <= rows {
                let (k0, k1, k2, k3) = (
                    f64::from(k[r]),
                    f64::from(k[r + 1]),
                    f64::from(k[r + 2]),
                    f64::from(k[r + 3]),
                );
                let w0 = &w[r * cols..][..cols];
                let w1 = &w[(r + 1) * cols..][..cols];
                let w2 = &w[(r + 2) * cols..][..cols];
                let w3 = &w[(r + 3) * cols..][..cols];
                for c in c0..c1 {
                    let mut a = acc[c];
                    a += k0 * f64::from(w0[c]);
                    a += k1 * f64::from(w1[c]);
                    a += k2 * f64::from(w2[c]);
                    a += k3 * f64::from(w3[c]);
                    acc[c] = a;
                }
                r += 4;
            }
            while r < rows {
                let weight = f64::from(k[r]);
                let wr = &w[r * cols..][..cols];
                for c in c0..c1 {
                    acc[c] += weight * f64::from(wr[c]);
                }
                r += 1;
            }
        }
        c0 += COLUMN_BLOCK;
    }
    PQ_OK
}
