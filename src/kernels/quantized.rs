//! Batched projections directly over encoded GGUF rows (F32, Q5_0, Q8_0,
//! Q4_K, Q6_K). Each weight row is decoded once per call into caller scratch
//! and reused for every position; F32 storage with F64 sums. Results are
//! bit-identical to `encoded::Encoding::decode` + `reference::Matrix` per
//! position. The caller owns file access, tiling policy, routing and threads.

use super::encoded::Encoding;
use std::{ffi::c_int, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Zero/mismatched dimensions, buffer lengths or offsets.
    Shape(&'static str),
    /// `cols` is not a block multiple, or the encoded byte count is wrong.
    Encoding {
        expected: Option<usize>,
        actual: usize,
    },
    Overflow,
    /// Scratch must hold at least one decoded row (`cols` floats).
    ScratchTooSmall {
        needed: usize,
        actual: usize,
    },
    NonFiniteInput,
    NonFiniteWeight,
    NonFiniteOutput,
    Native(i32),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

unsafe extern "C" {
    fn pq_project_rows(
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
    ) -> c_int;
    fn pq_accumulate_transposed_rows(
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
    ) -> c_int;
}

/// Native tile entry points share these signatures (portable and `power9_fma`).
type ProjectRows = unsafe extern "C" fn(
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
type TransposedRows = unsafe extern "C" fn(
    c_int,
    *const u8,
    usize,
    usize,
    usize,
    *const f32,
    usize,
    usize,
    usize,
    *mut f64,
    *mut f32,
    usize,
) -> c_int;

fn status(code: c_int) -> Result<(), Error> {
    match code {
        0 => Ok(()),
        6 => Err(Error::NonFiniteWeight),
        7 => Err(Error::NonFiniteOutput),
        4 => Err(Error::Overflow),
        // Everything else is pre-checked in Rust; reaching it is a contract bug.
        code => Err(Error::Native(code)),
    }
}

/// Borrowed encoded rows `[rows, cols]` in one encoding, exactly sized.
#[derive(Debug, Clone, Copy)]
pub struct EncodedRows<'a> {
    encoding: Encoding,
    rows: usize,
    cols: usize,
    row_bytes: usize,
    bytes: &'a [u8],
}

impl<'a> EncodedRows<'a> {
    pub fn new(
        encoding: Encoding,
        rows: usize,
        cols: usize,
        bytes: &'a [u8],
    ) -> Result<Self, Error> {
        if rows == 0 || cols == 0 {
            return Err(Error::Shape("zero rows or columns"));
        }
        let row_bytes = encoding.row_bytes(cols).map_err(|_| Error::Encoding {
            expected: None,
            actual: bytes.len(),
        })?;
        let expected = rows.checked_mul(row_bytes).ok_or(Error::Overflow)?;
        if bytes.len() != expected {
            return Err(Error::Encoding {
                expected: Some(expected),
                actual: bytes.len(),
            });
        }
        Ok(Self {
            encoding,
            rows,
            cols,
            row_bytes,
            bytes,
        })
    }
    pub fn encoding(&self) -> Encoding {
        self.encoding
    }
    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn cols(&self) -> usize {
        self.cols
    }
    /// Rows `[start, start + count)` as an independent view (no copy).
    pub fn slice(&self, start: usize, count: usize) -> Result<Self, Error> {
        let end = start.checked_add(count).ok_or(Error::Overflow)?;
        if count == 0 || end > self.rows {
            return Err(Error::Shape("row slice out of range"));
        }
        let bytes = &self.bytes[start * self.row_bytes..end * self.row_bytes];
        Ok(Self {
            rows: count,
            bytes,
            ..*self
        })
    }
    /// Largest row tile `scratch_len` floats can decode.
    pub fn tile_rows(&self, scratch_len: usize) -> Result<usize, Error> {
        match scratch_len / self.cols {
            0 => Err(Error::ScratchTooSmall {
                needed: self.cols,
                actual: scratch_len,
            }),
            n => Ok(n.min(self.rows)),
        }
    }
}

fn finite32(values: &[f32]) -> Result<(), Error> {
    if values.iter().all(|x| x.is_finite()) {
        Ok(())
    } else {
        Err(Error::NonFiniteInput)
    }
}

/// `out[p, r] = sum_c x[p, c] * W[r, c]` for position-major `x [positions, cols]`
/// and `out [positions, rows]`. Tiles rows to fit `scratch` (at least `cols`
/// floats; larger tiles reuse decoded rows across more positions, not across
/// calls). On error `out` contents are unspecified.
pub fn project(
    weights: &EncodedRows<'_>,
    x: &[f32],
    positions: usize,
    out: &mut [f32],
    scratch: &mut [f32],
) -> Result<(), Error> {
    project_with(pq_project_rows, weights, x, positions, out, scratch)
}

/// Shared validation + tiling; `kernel` must implement `pq_project_rows`'s
/// exact contract (the portable function or a bit-identical native one).
fn project_with(
    kernel: ProjectRows,
    weights: &EncodedRows<'_>,
    x: &[f32],
    positions: usize,
    out: &mut [f32],
    scratch: &mut [f32],
) -> Result<(), Error> {
    if positions == 0 {
        return Err(Error::Shape("zero positions"));
    }
    if positions.checked_mul(weights.cols) != Some(x.len()) {
        return Err(Error::Shape("activations must be [positions, cols]"));
    }
    if positions.checked_mul(weights.rows) != Some(out.len()) {
        return Err(Error::Shape("output must be [positions, rows]"));
    }
    finite32(x)?;
    let tile = weights.tile_rows(scratch.len())?;
    let mut start = 0;
    while start < weights.rows {
        let count = tile.min(weights.rows - start);
        let part = weights.slice(start, count)?;
        // SAFETY: exact encoded bytes for `count` rows (checked at construction
        // and slicing), x/out lengths checked above, scratch >= count * cols,
        // start + count <= rows = out stride; distinct borrows cannot overlap,
        // and C retains nothing.
        status(unsafe {
            kernel(
                part.encoding as c_int,
                part.bytes.as_ptr(),
                part.bytes.len(),
                count,
                part.cols,
                x.as_ptr(),
                positions,
                out.as_mut_ptr(),
                weights.rows,
                start,
                scratch.as_mut_ptr(),
                scratch.len(),
            )
        })?;
        start += count;
    }
    Ok(())
}

/// Persistent F64 transposed accumulation for row tiles of a larger matrix:
/// `accumulator[p, c] += sum_r coefficients[p, first_row + r] * W[r, c]`, where
/// `coefficients` is `[positions, total_rows]` and `weights` holds rows
/// `first_row..first_row + weights.rows()`. Call tiles in ascending row order
/// to reproduce a single-pass sum exactly. Each internal tile (bounded by
/// `scratch`) is checked before it modifies the accumulator; if a later
/// internal tile fails, earlier ones have already been added.
pub fn accumulate_transposed(
    weights: &EncodedRows<'_>,
    first_row: usize,
    coefficients: &[f32],
    total_rows: usize,
    positions: usize,
    accumulator: &mut [f64],
    scratch: &mut [f32],
) -> Result<(), Error> {
    accumulate_transposed_with(
        pq_accumulate_transposed_rows,
        weights,
        first_row,
        coefficients,
        total_rows,
        positions,
        accumulator,
        scratch,
    )
}

/// Shared validation + tiling; `kernel` must implement
/// `pq_accumulate_transposed_rows`'s exact contract.
#[allow(clippy::too_many_arguments)]
fn accumulate_transposed_with(
    kernel: TransposedRows,
    weights: &EncodedRows<'_>,
    first_row: usize,
    coefficients: &[f32],
    total_rows: usize,
    positions: usize,
    accumulator: &mut [f64],
    scratch: &mut [f32],
) -> Result<(), Error> {
    if positions == 0 {
        return Err(Error::Shape("zero positions"));
    }
    if first_row
        .checked_add(weights.rows)
        .is_none_or(|end| end > total_rows)
    {
        return Err(Error::Shape("row tile outside coefficient columns"));
    }
    if positions.checked_mul(total_rows) != Some(coefficients.len()) {
        return Err(Error::Shape("coefficients must be [positions, total_rows]"));
    }
    if positions.checked_mul(weights.cols) != Some(accumulator.len()) {
        return Err(Error::Shape("accumulator must be [positions, cols]"));
    }
    finite32(coefficients)?;
    if accumulator.iter().any(|x| !x.is_finite()) {
        return Err(Error::NonFiniteInput);
    }
    let tile = weights.tile_rows(scratch.len())?;
    let mut start = 0;
    while start < weights.rows {
        let count = tile.min(weights.rows - start);
        let part = weights.slice(start, count)?;
        // SAFETY: exact encoded bytes, coefficient/accumulator lengths and the
        // column window first_row + start + count <= total_rows checked above;
        // scratch >= count * cols; unique borrows, no retention.
        status(unsafe {
            kernel(
                part.encoding as c_int,
                part.bytes.as_ptr(),
                part.bytes.len(),
                count,
                part.cols,
                coefficients.as_ptr(),
                positions,
                total_rows,
                first_row + start,
                accumulator.as_mut_ptr(),
                scratch.as_mut_ptr(),
                scratch.len(),
            )
        })?;
        start += count;
    }
    Ok(())
}

/// Converts a finished F64 accumulator to F32 (only after ALL row tiles).
pub fn finish(accumulator: &[f64], out: &mut [f32]) -> Result<(), Error> {
    if accumulator.len() != out.len() {
        return Err(Error::Shape("accumulator/output length"));
    }
    for (o, &a) in out.iter_mut().zip(accumulator) {
        *o = a as f32;
        if !o.is_finite() {
            return Err(Error::NonFiniteOutput);
        }
    }
    Ok(())
}

/// Runtime-selected POWER9 kernels with explicit F64 FMA, bit-identical to the
/// portable ones (products of two binary32 values are exact; docs/backends.md).
pub mod int8;
pub mod power9_fma;
#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
mod tile_k_decode;
#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
mod tile_q4_0;
#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
mod tile_q4_0_decode;
#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
mod tile_q4_0_shared;

#[cfg(test)]
mod tests;
