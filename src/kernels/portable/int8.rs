//! Portable integer-dot projection and activation quantization (formerly
//! `native/quantized_int8.c` and the shared parts of `int8_common.h`), the
//! reference form of the int8 contract (`docs/backends.md`).
//!
//! Quantized input layout for P positions, padded to Pp = round_up(P, 4)
//! (padding positions are exact zeros), nb blocks per row, G = Pp / 4 groups:
//!   q   u8 [Pp][cols / 4][4][4]   a as a signed byte (Q6_K: a + 128); each
//!                                 4-element chunk is stored 4 times; padded
//!                                 to 16 bytes
//!   da  f32 [G][nb][4]            lane = position within the group
//!   s   i16 [G][nb][4][4][2]      Q4_K/Q6_K: sub-block sums S_2j, S_2j+1
//!       i16 [G][nb][4]            other types: block sums

use super::dequant::{decode, half, u32le};
use super::matmul::{
    PQ_ERR_ARGUMENT, PQ_ERR_NONFINITE_OUTPUT, PQ_ERR_NONFINITE_WEIGHT, PQ_ERR_OVERFLOW,
    PQ_ERR_SCRATCH, PQ_ERR_SIZE, PQ_ERR_UNSUPPORTED, PQ_OK, pq_row_bytes,
};
use std::ffi::c_int;
use std::slice::{from_raw_parts, from_raw_parts_mut};

/// Replication of each activation chunk in q.
const REP: usize = 4;

fn block_elements(kind: c_int) -> usize {
    match kind {
        12 | 14 | 35 => 256,
        2 | 6 | 8 => 32,
        _ => 0,
    }
}

fn block_bytes(kind: c_int) -> usize {
    match kind {
        12 => 144,
        14 => 210,
        35 => 66,
        2 => 18,
        6 => 22,
        8 => 34,
        _ => 0,
    }
}

/// Offset of the block's binary16 scale d (Q4_K: d, with dmin after it).
fn scale_field(kind: c_int) -> usize {
    match kind {
        14 => 208,
        35 => 64,
        _ => 0,
    }
}

fn k_type(kind: c_int) -> bool {
    kind == 12 || kind == 14
}

/// Activations are stored signed for every type but Q6_K (a + 128).
fn signed_act(kind: c_int) -> bool {
    kind != 14
}

/// int16 activation sums per (group of 4 positions, block).
fn sum_count(kind: c_int) -> usize {
    if k_type(kind) { 32 } else { 4 }
}

fn dots(kind: c_int) -> usize {
    match kind {
        12 => 8,
        14 => 16,
        _ => 1,
    }
}

fn dot_len(kind: c_int) -> usize {
    match kind {
        14 => 16,
        35 => 256,
        _ => 32,
    }
}

fn round(n: usize, m: usize) -> usize {
    n.div_ceil(m) * m
}

fn qbytes(padded: usize, cols: usize) -> usize {
    round(padded * cols * REP, 16)
}

/// Per-block weight metadata.
struct Meta {
    d: f32,
    dmin: f32,
    /// Q4_K: 8 (0..63); Q6_K: 16 signed; else scale[0] = 1.
    scale: [i32; 16],
    /// Q4_K: 8 mins (0..63).
    min: [i16; 8],
}

/// Unpack one block to signed weights and metadata (scale finiteness is
/// checked separately, before any output).
fn unpack_block(kind: c_int, p: &[u8], w: &mut [i8; 256]) -> Meta {
    let mut m = Meta {
        d: 0.0,
        dmin: 0.0,
        scale: [0; 16],
        min: [0; 8],
    };
    match kind {
        12 => {
            let scales = &p[4..16];
            let packed = &p[16..];
            m.d = half(p);
            m.dmin = half(&p[2..]);
            for g in 0..8 {
                let (sc, mn) = if g < 4 {
                    (scales[g] & 63, scales[g + 4] & 63)
                } else {
                    (
                        (scales[g + 4] & 15) | ((scales[g - 4] >> 6) << 4),
                        (scales[g + 4] >> 4) | ((scales[g] >> 6) << 4),
                    )
                };
                m.scale[g] = i32::from(sc);
                m.min[g] = i16::from(mn);
                let src = &packed[(g / 2) * 32..];
                for i in 0..32 {
                    w[g * 32 + i] = (if g % 2 == 0 { src[i] & 15 } else { src[i] >> 4 }) as i8;
                }
            }
        }
        14 => {
            m.d = half(&p[208..]);
            for h in 0..2 {
                for group in 0..4 {
                    let (low_shift, high_shift) = (if group < 2 { 0 } else { 4 }, group * 2);
                    for sub in 0..2 {
                        let low = h * 64 + (group % 2) * 32 + sub * 16;
                        let high = 128 + h * 32 + sub * 16;
                        let k = h * 8 + group * 2 + sub;
                        m.scale[k] = i32::from(p[192 + k] as i8);
                        for lane in 0..16 {
                            let v = ((p[low + lane] >> low_shift) & 15)
                                | (((p[high + lane] >> high_shift) & 3) << 4);
                            w[k * 16 + lane] = (i32::from(v) - 32) as i8;
                        }
                    }
                }
            }
        }
        35 => {
            m.d = half(&p[64..]);
            m.scale[0] = 1;
            for j in [0, 32] {
                for l in 0..4 {
                    for k in 0..32 {
                        w[j * 4 + l * 32 + k] = (i32::from((p[j + k] >> (2 * l)) & 3) - 1) as i8;
                    }
                }
            }
        }
        2 => {
            m.d = half(p);
            m.scale[0] = 1;
            for i in 0..32 {
                let nibble = if i < 16 {
                    p[2 + i] & 15
                } else {
                    p[2 + i - 16] >> 4
                };
                w[i] = (i32::from(nibble) - 8) as i8;
            }
        }
        6 => {
            m.d = half(p);
            m.scale[0] = 1;
            let high = u32le(&p[2..]);
            for i in 0..32 {
                let nibble = u32::from(if i < 16 {
                    p[6 + i] & 15
                } else {
                    p[6 + i - 16] >> 4
                });
                w[i] = ((nibble | (((high >> i) & 1) << 4)) as i32 - 16) as i8;
            }
        }
        _ => {
            m.d = half(p);
            m.scale[0] = 1;
            for i in 0..32 {
                w[i] = p[2 + i] as i8;
            }
        }
    }
    m
}

/// Quantized-input view.
struct Input<'a> {
    q: &'a [u8],
    da: &'a [f32],
    s: &'a [i16],
    nb: usize,
}

impl Input<'_> {
    /// Activation value a of position p, column i.
    fn a(&self, kind: c_int, cols: usize, p: usize, i: usize) -> i32 {
        let stored = self.q[p * cols * REP + (i / 4) * 16 + i % 4];
        if signed_act(kind) {
            i32::from(stored as i8)
        } else {
            i32::from(stored) - 128
        }
    }
    fn da(&self, p: usize, b: usize) -> f32 {
        self.da[((p / 4) * self.nb + b) * 4 + p % 4]
    }
    /// S_g for position p, block b (Q4_K).
    fn sub_sum(&self, p: usize, b: usize, g: usize) -> i32 {
        i32::from(self.s[((((p / 4) * self.nb + b) * 4 + g / 2) * 4 + p % 4) * 2 + g % 2])
    }
}

/// # Safety
/// `input` holds at least `pq8_input_bytes(kind, cols, positions)` bytes,
/// 16-byte aligned (checked by the callers).
unsafe fn input_view<'a>(
    kind: c_int,
    cols: usize,
    positions: usize,
    input: *const u8,
) -> Input<'a> {
    let nb = cols / block_elements(kind);
    let padded = round(positions, 4);
    let qb = qbytes(padded, cols);
    // SAFETY: the layout above lies inside the checked input length; the
    // float and int16 regions are aligned because qb and the float region
    // sizes are multiples of 16 and 4.
    unsafe {
        Input {
            q: from_raw_parts(input, qb),
            da: from_raw_parts(input.add(qb).cast::<f32>(), padded * nb),
            s: from_raw_parts(
                input.add(qb + padded * nb * 4).cast::<i16>(),
                padded * nb * sum_count(kind) / 4,
            ),
            nb,
        }
    }
}

/// Bytes of the quantized input for (type, cols, positions); 0 if
/// unsupported or on overflow.
#[unsafe(no_mangle)]
pub extern "C" fn pq8_input_bytes(kind: c_int, cols: usize, positions: usize) -> usize {
    let be = block_elements(kind);
    if be == 0 || cols == 0 || positions == 0 || !cols.is_multiple_of(be) {
        return 0;
    }
    if positions > usize::MAX - 3 {
        return 0;
    }
    let (padded, nb) = (round(positions, 4), cols / be);
    if padded > (usize::MAX - 16) / cols || padded > usize::MAX / REP / cols {
        return 0;
    }
    let q = qbytes(padded, cols);
    let per = 4 + (sum_count(kind) / 4) * 2;
    if nb > usize::MAX / padded || padded * nb > usize::MAX / per {
        return 0;
    }
    if q > usize::MAX - padded * nb * per {
        return 0;
    }
    q + padded * nb * per
}

/// # Safety
/// The C contract of `power_llm_quantized_int8.h`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pq8_quantize_group(
    kind: c_int,
    rows: *const *const f32,
    cols: usize,
    q: *mut u8,
    da: *mut f32,
    s: *mut i16,
    residual: *const *mut f32,
) -> c_int {
    let be = block_elements(kind);
    if be == 0 {
        return PQ_ERR_UNSUPPORTED;
    }
    if cols == 0 || !cols.is_multiple_of(be) {
        return PQ_ERR_ARGUMENT;
    }
    // SAFETY: forwarded contract.
    unsafe { pq8_quantize_group_range(kind, rows, cols, 0, cols / be, q, da, s, residual) }
}

/// Blocks [b0, b1) of a group of 4 rows (null rows are zero padding).
///
/// # Safety
/// The C contract of `power_llm_quantized_int8.h`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pq8_quantize_group_range(
    kind: c_int,
    rows: *const *const f32,
    cols: usize,
    b0: usize,
    b1: usize,
    q: *mut u8,
    da: *mut f32,
    s: *mut i16,
    residual: *const *mut f32,
) -> c_int {
    let be = block_elements(kind);
    if be == 0 {
        return PQ_ERR_UNSUPPORTED;
    }
    if q.is_null() || da.is_null() || s.is_null() || cols == 0 || !cols.is_multiple_of(be) {
        return PQ_ERR_ARGUMENT;
    }
    let nb = cols / be;
    if b0 > b1 || b1 > nb {
        return PQ_ERR_ARGUMENT;
    }
    let sgn = signed_act(kind);
    // 1.5 * 2^23: (v + M) - M rounds |v| <= 2^22 to the nearest integer,
    // ties to even, in binary32.
    const MAGIC: f32 = 12582912.0;
    for l in 0..4 {
        // SAFETY: rows (and residual, if non-null) point at 4 entries.
        let row = unsafe { *rows.add(l) };
        let res = if residual.is_null() {
            std::ptr::null_mut()
        } else {
            unsafe { *residual.add(l) }
        };
        for b in b0..b1 {
            let mut d = 0.0f32;
            let mut a = [0i32; 256];
            if !row.is_null() {
                // SAFETY: a non-null row holds cols floats.
                let xb = unsafe { from_raw_parts(row.add(b * be), be) };
                let mut amax = 0.0f32;
                for &x in xb {
                    let v = x.abs();
                    amax = if v > amax { v } else { amax };
                }
                d = amax / 127.0;
                let id = if amax > 0.0 { 127.0 / amax } else { 0.0 };
                for i in 0..be {
                    let v = (xb[i] * id + MAGIC) - MAGIC;
                    // The C comparison chain (clamp gives the same values).
                    #[allow(clippy::manual_clamp)]
                    let v = if v > 127.0 {
                        127.0
                    } else if v < -127.0 {
                        -127.0
                    } else {
                        v
                    };
                    a[i] = v as i32;
                }
            }
            // SAFETY: q holds the group's [4][cols * REP] bytes.
            let qb = unsafe { from_raw_parts_mut(q.add(l * cols * REP + b * be * REP), be * REP) };
            for i in 0..be {
                for j in 0..REP {
                    qb[(i / 4) * 16 + j * 4 + i % 4] = if sgn {
                        a[i] as i8 as u8
                    } else {
                        (a[i] + 128) as u8
                    };
                }
            }
            if !res.is_null() && !row.is_null() {
                // SAFETY: residual rows hold cols floats.
                let (xb, rb) = unsafe {
                    (
                        from_raw_parts(row.add(b * be), be),
                        from_raw_parts_mut(res.add(b * be), be),
                    )
                };
                for i in 0..be {
                    rb[i] = xb[i] - d * a[i] as f32;
                }
            }
            // SAFETY: da holds [nb][4], s [nb][sum_count].
            unsafe {
                *da.add(b * 4 + l) = d;
                if k_type(kind) {
                    for g in 0..8 {
                        let sum: i32 = a[g * 32..][..32].iter().sum();
                        *s.add(((b * 4 + g / 2) * 4 + l) * 2 + g % 2) = sum as i16;
                    }
                } else {
                    let sum: i32 = a[..be].iter().sum();
                    *s.add(b * 4 + l) = sum as i16;
                }
            }
        }
    }
    PQ_OK
}

/// Quantize [positions, cols] finite activations into the input layout.
///
/// # Safety
/// The C contract of `power_llm_quantized_int8.h`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pq8_quantize(
    kind: c_int,
    x: *const f32,
    positions: usize,
    cols: usize,
    input: *mut u8,
    input_len: usize,
) -> c_int {
    if x.is_null() || input.is_null() || input as usize & 15 != 0 {
        return PQ_ERR_ARGUMENT;
    }
    if block_elements(kind) == 0 {
        return PQ_ERR_UNSUPPORTED;
    }
    let need = pq8_input_bytes(kind, cols, positions);
    if need == 0 {
        return PQ_ERR_SIZE;
    }
    if input_len < need {
        return PQ_ERR_SCRATCH;
    }
    let nb = cols / block_elements(kind);
    let padded = round(positions, 4);
    let qb = qbytes(padded, cols);
    // SAFETY: regions of the checked input buffer.
    let (da, s) = unsafe {
        (
            input.add(qb).cast::<f32>(),
            input.add(qb + padded * nb * 4).cast::<i16>(),
        )
    };
    for g in 0..padded / 4 {
        let rows: [*const f32; 4] = std::array::from_fn(|l| {
            if 4 * g + l < positions {
                // SAFETY: row 4g + l of x.
                unsafe { x.add((4 * g + l) * cols) }
            } else {
                std::ptr::null()
            }
        });
        // SAFETY: group g's regions.
        let status = unsafe {
            pq8_quantize_group(
                kind,
                rows.as_ptr(),
                cols,
                input.add(4 * g * cols * REP),
                da.add(g * nb * 4),
                s.add(g * nb * sum_count(kind)),
                std::ptr::null(),
            )
        };
        if status != PQ_OK {
            return status;
        }
    }
    PQ_OK
}

/// Shared argument checks (before any write), including finite block scales.
/// Returns (status, row bytes).
///
/// # Safety
/// `weights` holds `weight_bytes` bytes when the sizes check out.
#[allow(clippy::too_many_arguments)]
unsafe fn check(
    kind: c_int,
    weights: *const u8,
    weight_bytes: usize,
    rows: usize,
    cols: usize,
    input: *const u8,
    input_len: usize,
    positions: usize,
    out: *const f32,
    out_stride: usize,
    out_offset: usize,
) -> Result<usize, c_int> {
    if weights.is_null()
        || input.is_null()
        || out.is_null()
        || rows == 0
        || cols == 0
        || positions == 0
    {
        return Err(PQ_ERR_ARGUMENT);
    }
    if input as usize & 15 != 0 {
        return Err(PQ_ERR_ARGUMENT);
    }
    let be = block_elements(kind);
    if be == 0 {
        return Err(PQ_ERR_UNSUPPORTED);
    }
    if !cols.is_multiple_of(be) {
        return Err(PQ_ERR_SIZE);
    }
    if out_offset > usize::MAX - rows {
        return Err(PQ_ERR_OVERFLOW);
    }
    if out_offset + rows > out_stride {
        return Err(PQ_ERR_ARGUMENT);
    }
    if positions > usize::MAX / out_stride {
        return Err(PQ_ERR_OVERFLOW);
    }
    let (nb, bb) = (cols / be, block_bytes(kind));
    let row_bytes = nb * bb;
    if rows > usize::MAX / row_bytes {
        return Err(PQ_ERR_OVERFLOW);
    }
    if rows * row_bytes != weight_bytes {
        return Err(PQ_ERR_SIZE);
    }
    let need = pq8_input_bytes(kind, cols, positions);
    if need == 0 {
        return Err(PQ_ERR_OVERFLOW);
    }
    if input_len < need {
        return Err(PQ_ERR_SCRATCH);
    }
    // SAFETY: size checked above.
    let weights = unsafe { from_raw_parts(weights, weight_bytes) };
    for block in weights.chunks_exact(bb) {
        let finite = if kind == 12 {
            half(block).is_finite() && half(&block[2..]).is_finite()
        } else {
            half(&block[scale_field(kind)..]).is_finite()
        };
        if !finite {
            return Err(PQ_ERR_NONFINITE_WEIGHT);
        }
    }
    Ok(row_bytes)
}

/// out[p * out_stride + out_offset + r] = weights row r . a_p, per block
/// fma((float)T, d * da, acc) (Q4_K adds the mins term).
///
/// # Safety
/// The C contract of `power_llm_quantized_int8.h`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pq8_project_rows(
    kind: c_int,
    weights: *const u8,
    weight_bytes: usize,
    rows: usize,
    cols: usize,
    input: *const u8,
    input_len: usize,
    positions: usize,
    out: *mut f32,
    out_stride: usize,
    out_offset: usize,
) -> c_int {
    // SAFETY: forwarded contract.
    let row_bytes = match unsafe {
        check(
            kind,
            weights,
            weight_bytes,
            rows,
            cols,
            input,
            input_len,
            positions,
            out,
            out_stride,
            out_offset,
        )
    } {
        Ok(n) => n,
        Err(status) => return status,
    };
    // SAFETY: checked sizes.
    let (weights, inp) = unsafe {
        (
            from_raw_parts(weights, weight_bytes),
            input_view(kind, cols, positions, input),
        )
    };
    let (be, bb, nd, len) = (
        block_elements(kind),
        block_bytes(kind),
        dots(kind),
        dot_len(kind),
    );
    let mut status = PQ_OK;
    let mut w = [0i8; 256];
    for r in 0..rows {
        for p in 0..positions {
            let mut acc = 0.0f32;
            for b in 0..inp.nb {
                let m = unpack_block(kind, &weights[r * row_bytes + b * bb..][..bb], &mut w);
                let mut t = 0i32;
                for k in 0..nd {
                    let mut dot = 0i32;
                    for i in 0..len {
                        dot +=
                            i32::from(w[k * len + i]) * inp.a(kind, cols, p, b * be + k * len + i);
                    }
                    t += m.scale[k] * dot;
                }
                let da = inp.da(p, b);
                acc = (t as f32).mul_add(m.d * da, acc);
                if kind == 12 {
                    let mut mins = 0i32;
                    for g in 0..8 {
                        mins += i32::from(m.min[g]) * inp.sub_sum(p, b, g);
                    }
                    acc = (-(mins as f32)).mul_add(m.dmin * da, acc);
                }
            }
            // SAFETY: inside the checked output window.
            unsafe { *out.add(p * out_stride + out_offset + r) = acc };
            if !acc.is_finite() {
                status = PQ_ERR_NONFINITE_OUTPUT;
            }
        }
    }
    status
}

/// accumulator[p * cols + c] += (double)(binary32 fma chain over rows).
///
/// # Safety
/// The C contract of `power_llm_quantized_int8.h`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pq_accumulate_transposed_rows_f32(
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
    if positions > usize::MAX / coeff_stride || positions > usize::MAX / cols {
        return PQ_ERR_OVERFLOW;
    }
    if !matches!(kind, 0 | 1 | 2 | 6 | 8 | 12 | 14 | 35) {
        return PQ_ERR_UNSUPPORTED;
    }
    let row_bytes = pq_row_bytes(kind, cols);
    if row_bytes == 0 {
        return PQ_ERR_SIZE;
    }
    if rows > usize::MAX / row_bytes || rows > usize::MAX / cols {
        return PQ_ERR_OVERFLOW;
    }
    if rows * row_bytes != weight_bytes {
        return PQ_ERR_SIZE;
    }
    if scratch_len < rows * cols {
        return PQ_ERR_SCRATCH;
    }
    // SAFETY: checked sizes.
    let (weights, tile) = unsafe {
        (
            from_raw_parts(weights, weight_bytes),
            from_raw_parts_mut(scratch, rows * cols),
        )
    };
    for (wr, row) in weights
        .chunks_exact(row_bytes)
        .zip(tile.chunks_exact_mut(cols))
    {
        decode(kind, wr, row);
        if !row.iter().all(|v| v.is_finite()) {
            return PQ_ERR_NONFINITE_WEIGHT;
        }
    }
    // SAFETY: [positions, cols] accumulator.
    let acc = unsafe { from_raw_parts_mut(accumulator, positions * cols) };
    for p in 0..positions {
        // SAFETY: coefficient window of position p.
        let k = unsafe { from_raw_parts(coefficients.add(p * coeff_stride + coeff_offset), rows) };
        for c in 0..cols {
            let mut t = 0.0f32;
            for r in 0..rows {
                t = k[r].mul_add(tile[r * cols + c], t);
            }
            acc[p * cols + c] += f64::from(t);
        }
    }
    PQ_OK
}

/// Experimental Q4_0 two-plane quantizer: d1 = max|x| / 127, d2 = d1 / 256,
/// target = round_even(x / d2), a1 = floor((target + 128) / 256),
/// a2 = target - 256 a1. Rejects underflow/overflow before writing.
///
/// # Safety
/// The C contract of `power_llm_quantized_int8.h`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pq8_shared_quantize(
    rows: *const *const f32,
    cols: usize,
    b0: usize,
    b1: usize,
    q1: *mut u8,
    d1: *mut f32,
    s1: *mut i16,
    q2: *mut u8,
    d2: *mut f32,
    s2: *mut i16,
) -> c_int {
    if rows.is_null()
        || q1.is_null()
        || d1.is_null()
        || s1.is_null()
        || q2.is_null()
        || d2.is_null()
        || s2.is_null()
        || cols == 0
        || !cols.is_multiple_of(32)
        || b0 > b1
        || b1 > cols / 32
    {
        return PQ_ERR_ARGUMENT;
    }
    // SAFETY: rows points at 4 entries; non-null rows hold cols floats.
    let rows: [Option<&[f32]>; 4] = std::array::from_fn(|l| {
        let r = unsafe { *rows.add(l) };
        (!r.is_null()).then(|| unsafe { from_raw_parts(r, cols) })
    });
    for row in rows.iter().flatten() {
        for b in b0..b1 {
            let mut amax = 0.0f32;
            for &x in &row[b * 32..][..32] {
                if !x.is_finite() {
                    return PQ_ERR_ARGUMENT;
                }
                amax = amax.max(x.abs());
            }
            let fine = amax / 127.0 / 256.0;
            if amax > 0.0 && (fine < f32::MIN_POSITIVE || !(1.0 / fine).is_finite()) {
                return PQ_ERR_ARGUMENT;
            }
        }
    }
    const MAGIC: f32 = 12582912.0;
    for (l, row) in rows.iter().enumerate() {
        for b in b0..b1 {
            let mut amax = 0.0f32;
            if let Some(row) = row {
                for &x in &row[b * 32..][..32] {
                    amax = amax.max(x.abs());
                }
            }
            let d = amax / 127.0;
            let fine = d / 256.0;
            let invfine = if fine > 0.0 { 1.0 / fine } else { 0.0 };
            let (mut sum1, mut sum2) = (0i32, 0i32);
            for i in 0..32 {
                let x = row.map_or(0.0, |r| r[b * 32 + i]);
                let target = ((x * invfine + MAGIC) - MAGIC) as i32;
                let a = if target >= 0 {
                    (target + 128) / 256
                } else {
                    -((-target + 127) / 256)
                };
                let c = target - 256 * a;
                sum1 += a;
                sum2 += c;
                for j in 0..REP {
                    let ix = l * cols * REP + b * 128 + (i / 4) * 16 + j * 4 + i % 4;
                    // SAFETY: q planes hold [4][cols * REP] bytes.
                    unsafe {
                        *q1.add(ix) = a as i8 as u8;
                        *q2.add(ix) = c as i8 as u8;
                    }
                }
            }
            // SAFETY: d/s planes hold [nb][4] entries.
            unsafe {
                *d1.add(b * 4 + l) = d;
                *d2.add(b * 4 + l) = fine;
                *s1.add(b * 4 + l) = sum1 as i16;
                *s2.add(b * 4 + l) = sum2 as i16;
            }
        }
    }
    PQ_OK
}

/// Q4_0 with a shared-scale two-plane input: positions [0, P/2) are the
/// coarse plane, [P/2, P) the fine plane; outputs of the fine half are 0.
///
/// # Safety
/// The C contract of `power_llm_quantized_int8.h`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pq8_shared_project_rows(
    kind: c_int,
    weights: *const u8,
    weight_bytes: usize,
    rows: usize,
    cols: usize,
    input: *const u8,
    input_len: usize,
    positions: usize,
    out: *mut f32,
    out_stride: usize,
    out_offset: usize,
) -> c_int {
    if kind != 2 || !positions.is_multiple_of(2) {
        return PQ_ERR_ARGUMENT;
    }
    // SAFETY: forwarded contract.
    let row_bytes = match unsafe {
        check(
            kind,
            weights,
            weight_bytes,
            rows,
            cols,
            input,
            input_len,
            positions,
            out,
            out_stride,
            out_offset,
        )
    } {
        Ok(n) => n,
        Err(status) => return status,
    };
    // SAFETY: checked sizes.
    let (weights, inp) = unsafe {
        (
            from_raw_parts(weights, weight_bytes),
            input_view(kind, cols, positions, input),
        )
    };
    let plane = positions / 2;
    let mut status = PQ_OK;
    let mut w = [0i8; 256];
    for r in 0..rows {
        for p in 0..plane {
            let mut acc = 0.0f32;
            for b in 0..inp.nb {
                let m = unpack_block(2, &weights[r * row_bytes + b * 18..][..18], &mut w);
                let (mut hi, mut lo) = (0i32, 0i32);
                for i in 0..32 {
                    hi += i32::from(w[i]) * inp.a(2, cols, p, b * 32 + i);
                    lo += i32::from(w[i]) * inp.a(2, cols, p + plane, b * 32 + i);
                }
                acc = ((hi * 256 + lo) as f32).mul_add(m.d * inp.da(p + plane, b), acc);
            }
            // SAFETY: inside the checked output window.
            unsafe {
                *out.add(p * out_stride + out_offset + r) = acc;
                *out.add((p + plane) * out_stride + out_offset + r) = 0.0;
            }
            if !acc.is_finite() {
                status = PQ_ERR_NONFINITE_OUTPUT;
            }
        }
    }
    status
}
