//! Checked C tensor references. F32 storage, F64 reductions; not PyTorch/BF16
//! bitwise parity or a fast matrix backend. RMS weights are direct multipliers.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidShape,
    InvalidParameter,
    NonFiniteValue,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

unsafe extern "C" {
    fn p9_ref_rms_norm(x: *const f32, scale: *const f32, n: usize, eps: f32, out: *mut f32);
    fn p9_ref_matvec(matrix: *const f32, x: *const f32, rows: usize, cols: usize, out: *mut f32);
    fn p9_ref_matvec_multi(
        matrix: *const f32,
        x: *const f32,
        inputs: usize,
        rows: usize,
        cols: usize,
        out: *mut f32,
    );
    #[cfg(all(feature = "power9", target_arch = "powerpc64"))]
    fn p9_ref_matvec_multi_power9(
        matrix: *const f32,
        blocked: *const f32,
        x: *const f32,
        inputs: usize,
        rows: usize,
        cols: usize,
        out: *mut f32,
        scratch: *mut f64,
    );
    fn p9_ref_accumulate_transposed(
        matrix: *const f32,
        weights: *const f32,
        rows: usize,
        cols: usize,
        accumulator: *mut f64,
    );
    fn p9_ref_gelu_product(gate: *const f32, up: *const f32, n: usize, out: *mut f32);
    fn p9_ref_softmax(x: *const f32, n: usize, out: *mut f32);
    fn p9_ref_softcap(x: *const f32, n: usize, cap: f32, out: *mut f32);
    fn p9_ref_mul_scaled(a: *const f32, b: *const f32, n: usize, factor: f32, out: *mut f32);
    fn p9_ref_add(a: *const f32, b: *const f32, n: usize, out: *mut f32);
    fn p9_ref_rope(
        x: *const f32,
        heads: usize,
        dim: usize,
        position: usize,
        base: f32,
        factors: *const f32,
        out: *mut f32,
    ) -> bool;
    fn p9_ref_attention_heads(
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
    ) -> bool;
}

fn finite(x: &[f32]) -> Result<(), Error> {
    if all_finite(x) {
        Ok(())
    } else {
        Err(Error::NonFiniteValue)
    }
}
/// True when no value is infinite or NaN. Branch-free within chunks (an
/// all-ones exponent marks inf/NaN), so it vectorizes: about 10x faster than
/// an early-exit scan on hot one-token paths.
pub fn all_finite(x: &[f32]) -> bool {
    x.chunks(256).all(|c| {
        c.iter().fold(0u32, |bad, v| {
            bad | u32::from(v.to_bits() & 0x7f80_0000 == 0x7f80_0000)
        }) == 0
    })
}
fn checked(out: Vec<f32>) -> Result<Vec<f32>, Error> {
    finite(&out)?;
    Ok(out)
}
fn pair(a: &[f32], b: &[f32]) -> Result<(), Error> {
    if a.len() != b.len() {
        return Err(Error::InvalidShape);
    }
    finite(a)?;
    finite(b)
}

#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f32>,
    /// The rows in blocks of 8, column-interleaved (built on first use by
    /// the vectorized [`Self::project_many`]).
    #[cfg_attr(
        not(all(feature = "power9", target_arch = "powerpc64")),
        allow(dead_code)
    )]
    blocked: std::sync::OnceLock<Vec<f32>>,
}
impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f32>) -> Result<Self, Error> {
        if rows == 0 || cols == 0 || rows.checked_mul(cols) != Some(data.len()) {
            return Err(Error::InvalidShape);
        }
        finite(&data)?;
        Ok(Self {
            rows,
            cols,
            data,
            blocked: std::sync::OnceLock::new(),
        })
    }
    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn cols(&self) -> usize {
        self.cols
    }
    /// Rows `[first, first + count)` of [`Self::project`] (identical values).
    pub fn project_rows(&self, x: &[f32], first: usize, count: usize) -> Result<Vec<f32>, Error> {
        if x.len() != self.cols || first.checked_add(count).is_none_or(|end| end > self.rows) {
            return Err(Error::InvalidShape);
        }
        finite(x)?;
        let mut out = vec![0.0; count];
        // SAFETY: rows [first, first + count) lie inside the checked matrix;
        // x matches cols; initialized output of count elements; no retention.
        unsafe {
            p9_ref_matvec(
                self.data.as_ptr().add(first * self.cols),
                x.as_ptr(),
                count,
                self.cols,
                out.as_mut_ptr(),
            )
        };
        checked(out)
    }
    /// [`Self::project`] of each input (identical values), reading the
    /// matrix once per 4 inputs.
    pub fn project_many(&self, xs: &[&[f32]]) -> Result<Vec<Vec<f32>>, Error> {
        let mut flat = Vec::with_capacity(xs.len() * self.cols);
        for x in xs {
            if x.len() != self.cols {
                return Err(Error::InvalidShape);
            }
            finite(x)?;
            flat.extend_from_slice(x);
        }
        let mut out = vec![0.0; xs.len() * self.rows];
        #[cfg(all(feature = "power9", target_arch = "powerpc64"))]
        if crate::kernels::quantized::power9_fma::FmaKernels::new(
            crate::kernels::quantized::power9_fma::Request::Power9Fma,
        )
        .is_ok()
        {
            let blocked = self.blocked.get_or_init(|| {
                let mut b = Vec::with_capacity(self.rows / 8 * 8 * self.cols);
                for block in 0..self.rows / 8 {
                    for col in 0..self.cols {
                        for r in 0..8 {
                            b.push(self.data[(block * 8 + r) * self.cols + col]);
                        }
                    }
                }
                b
            });
            let mut scratch = vec![0.0f64; 8 * self.cols];
            // SAFETY: as below; blocked holds rows / 8 blocks of 8 * cols
            // values; scratch holds 8 * cols doubles.
            unsafe {
                p9_ref_matvec_multi_power9(
                    self.data.as_ptr(),
                    blocked.as_ptr(),
                    flat.as_ptr(),
                    xs.len(),
                    self.rows,
                    self.cols,
                    out.as_mut_ptr(),
                    scratch.as_mut_ptr(),
                )
            };
            finite(&out)?;
            return Ok(out.chunks_exact(self.rows).map(<[f32]>::to_vec).collect());
        }
        // SAFETY: flat holds xs.len() inputs of cols values, out holds
        // xs.len() rows-long outputs; the matrix is checked; no retention.
        unsafe {
            p9_ref_matvec_multi(
                self.data.as_ptr(),
                flat.as_ptr(),
                xs.len(),
                self.rows,
                self.cols,
                out.as_mut_ptr(),
            )
        };
        finite(&out)?;
        Ok(out.chunks_exact(self.rows).map(<[f32]>::to_vec).collect())
    }
    pub fn project(&self, x: &[f32]) -> Result<Vec<f32>, Error> {
        if x.len() != self.cols {
            return Err(Error::InvalidShape);
        }
        finite(x)?;
        let mut out = vec![0.0; self.rows];
        // SAFETY: matrix dimensions checked at construction, matching x,
        // initialized output, immutable non-overlapping buffers, no retention.
        unsafe {
            p9_ref_matvec(
                self.data.as_ptr(),
                x.as_ptr(),
                self.rows,
                self.cols,
                out.as_mut_ptr(),
            )
        };
        checked(out)
    }
    /// Accumulate matrix^T * weights into caller-owned F64 scratch, allowing
    /// tiled vocabulary projections without rounding to F32 between tiles.
    pub fn accumulate_transposed(
        &self,
        weights: &[f32],
        accumulator: &mut [f64],
    ) -> Result<(), Error> {
        if weights.len() != self.rows || accumulator.len() != self.cols {
            return Err(Error::InvalidShape);
        }
        finite(weights)?;
        if accumulator.iter().any(|x| !x.is_finite()) {
            return Err(Error::NonFiniteValue);
        }
        // SAFETY: matrix shape/values checked at construction; matching finite
        // weights and uniquely borrowed F64 scratch, no aliasing or retention.
        unsafe {
            p9_ref_accumulate_transposed(
                self.data.as_ptr(),
                weights.as_ptr(),
                self.rows,
                self.cols,
                accumulator.as_mut_ptr(),
            )
        };
        if accumulator.iter().any(|x| !x.is_finite()) {
            return Err(Error::NonFiniteValue);
        }
        Ok(())
    }
}

pub fn rms_norm(x: &[f32], scale: Option<&[f32]>, eps: f32) -> Result<Vec<f32>, Error> {
    if x.is_empty() {
        return Err(Error::InvalidShape);
    }
    if !eps.is_finite() || eps <= 0.0 {
        return Err(Error::InvalidParameter);
    }
    match scale {
        Some(scale) => pair(x, scale)?,
        None => finite(x)?,
    }
    let mut out = vec![0.0; x.len()];
    // SAFETY: nonempty finite x, optional matching scale or NULL, positive eps,
    // initialized output. C only reads scale when non-NULL.
    unsafe {
        p9_ref_rms_norm(
            x.as_ptr(),
            scale.map_or(std::ptr::null(), |s| s.as_ptr()),
            x.len(),
            eps,
            out.as_mut_ptr(),
        )
    };
    checked(out)
}

pub fn gelu_product(gate: &[f32], up: &[f32]) -> Result<Vec<f32>, Error> {
    pair(gate, up)?;
    let mut out = vec![0.0; gate.len()];
    // SAFETY: equal-length finite immutable inputs and initialized output.
    unsafe { p9_ref_gelu_product(gate.as_ptr(), up.as_ptr(), gate.len(), out.as_mut_ptr()) };
    checked(out)
}

pub fn softmax(x: &[f32]) -> Result<Vec<f32>, Error> {
    if x.is_empty() {
        return Err(Error::InvalidShape);
    }
    if x.iter().any(|x| x.is_nan() || *x == f32::INFINITY) || !x.iter().any(|x| x.is_finite()) {
        return Err(Error::NonFiniteValue);
    }
    let mut out = vec![0.0; x.len()];
    // SAFETY: at least one finite logit, others finite/-inf; no empty max read.
    unsafe { p9_ref_softmax(x.as_ptr(), x.len(), out.as_mut_ptr()) };
    checked(out)
}

pub fn softcap(x: &[f32], cap: f32) -> Result<Vec<f32>, Error> {
    if !cap.is_finite() || cap <= 0.0 {
        return Err(Error::InvalidParameter);
    }
    finite(x)?;
    let mut out = vec![0.0; x.len()];
    // SAFETY: finite inputs, positive cap, initialized equally sized output.
    unsafe { p9_ref_softcap(x.as_ptr(), x.len(), cap, out.as_mut_ptr()) };
    checked(out)
}

pub fn multiply_scaled(a: &[f32], b: &[f32], factor: f32) -> Result<Vec<f32>, Error> {
    pair(a, b)?;
    if !factor.is_finite() {
        return Err(Error::InvalidParameter);
    }
    let mut out = vec![0.0; a.len()];
    // SAFETY: matching finite inputs, finite factor, initialized output.
    unsafe { p9_ref_mul_scaled(a.as_ptr(), b.as_ptr(), a.len(), factor, out.as_mut_ptr()) };
    checked(out)
}

pub fn add(a: &[f32], b: &[f32]) -> Result<Vec<f32>, Error> {
    pair(a, b)?;
    let mut out = vec![0.0; a.len()];
    // SAFETY: matching finite inputs and equally sized initialized output.
    unsafe { p9_ref_add(a.as_ptr(), b.as_ptr(), a.len(), out.as_mut_ptr()) };
    checked(out)
}

/// NeoX/split-half rotation across each complete head, with optional stored
/// frequency divisors. F32 frequencies/angles; not BF16/PyTorch bitwise parity.
pub fn rope(
    x: &[f32],
    head_dim: usize,
    position: usize,
    base: f32,
    factors: Option<&[f32]>,
) -> Result<Vec<f32>, Error> {
    if head_dim == 0
        || !head_dim.is_multiple_of(2)
        || x.is_empty()
        || !x.len().is_multiple_of(head_dim)
    {
        return Err(Error::InvalidShape);
    }
    finite(x)?;
    if !base.is_finite() || base <= 0.0 {
        return Err(Error::InvalidParameter);
    }
    if let Some(factors) = factors {
        if factors.len() != head_dim / 2 {
            return Err(Error::InvalidShape);
        }
        if factors.iter().any(|&f| !f.is_finite() || f <= 0.0) {
            return Err(Error::InvalidParameter);
        }
    }
    let mut out = vec![0.0; x.len()];
    // SAFETY: full even heads, finite x, matching positive optional divisors,
    // separately owned initialized output; pointers never retained by C.
    let ok = unsafe {
        p9_ref_rope(
            x.as_ptr(),
            x.len() / head_dim,
            head_dim,
            position,
            base,
            factors.map_or(std::ptr::null(), |f| f.as_ptr()),
            out.as_mut_ptr(),
        )
    };
    if !ok {
        return Err(Error::NonFiniteValue);
    }
    checked(out)
}

/// One query position against token-major K/V, contiguous GQA head groups,
/// attention scale exactly one. Optional mask has one entry per key token.
pub fn attention(
    query: &[f32],
    keys: &[f32],
    values: &[f32],
    kv_heads: usize,
    head_dim: usize,
    allowed: Option<&[bool]>,
) -> Result<Vec<f32>, Error> {
    attention_heads(query, keys, values, kv_heads, head_dim, allowed, None)
}

/// Query heads `heads` (all if None) of [`attention`], each computed exactly
/// as there; the output holds only those heads.
pub fn attention_heads(
    query: &[f32],
    keys: &[f32],
    values: &[f32],
    kv_heads: usize,
    head_dim: usize,
    allowed: Option<&[bool]>,
    heads: Option<std::ops::Range<usize>>,
) -> Result<Vec<f32>, Error> {
    let width = kv_heads
        .checked_mul(head_dim)
        .filter(|&w| w > 0)
        .ok_or(Error::InvalidShape)?;
    if query.is_empty()
        || !query.len().is_multiple_of(head_dim)
        || keys.is_empty()
        || !keys.len().is_multiple_of(width)
        || values.len() != keys.len()
    {
        return Err(Error::InvalidShape);
    }
    let qheads = query.len() / head_dim;
    let nkeys = keys.len() / width;
    if !qheads.is_multiple_of(kv_heads) {
        return Err(Error::InvalidShape);
    }
    if let Some(mask) = allowed {
        if mask.len() != nkeys {
            return Err(Error::InvalidShape);
        }
        if !mask.iter().any(|&v| v) {
            return Err(Error::InvalidParameter);
        }
    }
    let heads = heads.unwrap_or(0..qheads);
    if heads.start > heads.end || heads.end > qheads {
        return Err(Error::InvalidShape);
    }
    finite(query)?;
    finite(keys)?;
    finite(values)?;
    let mask: Option<Vec<u8>> = allowed.map(|m| m.iter().map(|&b| u8::from(b)).collect());
    let mut scratch = vec![0.0; nkeys];
    let mut out = vec![0.0; heads.len() * head_dim];
    // SAFETY: overflow-checked token/head dimensions, matching finite buffers,
    // valid optional 0/1 mask, non-aliasing scratch/output of exact lengths.
    let ok = unsafe {
        p9_ref_attention_heads(
            query.as_ptr(),
            keys.as_ptr(),
            values.as_ptr(),
            qheads,
            kv_heads,
            head_dim,
            nkeys,
            mask.as_ref().map_or(std::ptr::null(), |m| m.as_ptr()),
            scratch.as_mut_ptr(),
            out.as_mut_ptr(),
            heads.start,
            heads.end,
        )
    };
    if !ok {
        return Err(Error::NonFiniteValue);
    }
    checked(out)
}

#[cfg(test)]
mod tests;
