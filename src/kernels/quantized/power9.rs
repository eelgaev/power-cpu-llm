//! Runtime-selected POWER9/VSX batched projections, bit-identical to the
//! portable kernels (same F32 outputs and F64 accumulators). Selection happens
//! once in [`MatrixKernels::new`]; the native entry points are only reachable
//! after the baseline-compiled Linux HWCAP2 check (`Kernels::new(Power9)`)
//! succeeds. Validation, tiling and error mapping are the parent module's.

use super::{
    EncodedRows, Error, ProjectRows, TransposedRows, accumulate_transposed_with,
    pq_accumulate_transposed_rows, pq_project_rows, project_with,
};
use std::fmt;

#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
unsafe extern "C" {
    fn pq_project_rows_power9(
        kind: std::ffi::c_int,
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
    ) -> std::ffi::c_int;
    // Bit-exact and tested, but slower than portable on the measured host: not dispatched.
    #[cfg_attr(not(test), allow(dead_code))]
    fn pq_accumulate_transposed_rows_power9(
        kind: std::ffi::c_int,
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
    ) -> std::ffi::c_int;
}

/// Requested matrix backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Request {
    /// POWER9 when compiled in AND detected at runtime, otherwise portable.
    Auto,
    Portable,
    /// Fails with [`Unavailable`] unless compiled in and detected at runtime.
    Power9,
}

/// The forced POWER9 backend is not compiled into this build or the CPU
/// lacks ISA 3.0 (or the OS is not Linux).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unavailable;
impl fmt::Display for Unavailable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("POWER9 matrix kernels unavailable (build or CPU)")
    }
}
impl std::error::Error for Unavailable {}

/// Resolved, copyable kernel pair. Only constructible through [`MatrixKernels::new`].
/// The `power9` pair uses the native projection and the portable transposed
/// accumulation (faster on the measured host); both are bit-identical anyway.
#[derive(Debug, Clone, Copy)]
pub struct MatrixKernels {
    name: &'static str,
    project: ProjectRows,
    transposed: TransposedRows,
}

impl MatrixKernels {
    pub fn new(request: Request) -> Result<Self, Unavailable> {
        let portable = Self {
            name: "portable",
            project: pq_project_rows,
            transposed: pq_accumulate_transposed_rows,
        };
        match request {
            Request::Portable => Ok(portable),
            Request::Power9 => native().ok_or(Unavailable),
            Request::Auto => Ok(native().unwrap_or(portable)),
        }
    }

    /// `"portable"` or `"power9"`.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Same contract and results as [`super::project`].
    pub fn project(
        &self,
        weights: &EncodedRows<'_>,
        x: &[f32],
        positions: usize,
        out: &mut [f32],
        scratch: &mut [f32],
    ) -> Result<(), Error> {
        project_with(self.project, weights, x, positions, out, scratch)
    }

    /// Same contract and results as [`super::accumulate_transposed`].
    #[allow(clippy::too_many_arguments)]
    pub fn accumulate_transposed(
        &self,
        weights: &EncodedRows<'_>,
        first_row: usize,
        coefficients: &[f32],
        total_rows: usize,
        positions: usize,
        accumulator: &mut [f64],
        scratch: &mut [f32],
    ) -> Result<(), Error> {
        accumulate_transposed_with(
            self.transposed,
            weights,
            first_row,
            coefficients,
            total_rows,
            positions,
            accumulator,
            scratch,
        )
    }
}

/// Native kernels only if this build contains them AND the existing
/// baseline-compiled runtime detection reports POWER9 (ISA 3.0) on Linux.
fn native() -> Option<MatrixKernels> {
    #[cfg(all(feature = "power9", target_arch = "powerpc64"))]
    if super::super::Kernels::new(super::super::Backend::Power9).is_ok() {
        return Some(MatrixKernels {
            name: "power9",
            project: pq_project_rows_power9,
            // The native transposed kernel is bit-identical but measured slower
            // than the portable one (docs/backends.md), so it is not selected.
            transposed: pq_accumulate_transposed_rows,
        });
    }
    None
}

#[cfg(test)]
mod tests;
