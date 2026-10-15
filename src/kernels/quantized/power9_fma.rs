//! Runtime-selected POWER9 kernels using explicit F64 fused multiply-add,
//! bit-identical to the portable kernels because every fused product is a
//! product of two binary32 values, which is exact in binary64 (see
//! `docs/backends.md`). Native code is only reachable after the
//! baseline-compiled Linux HWCAP2 check (`Kernels::new(Power9)`) succeeds.
//! Validation, tiling and error mapping are the parent module's.

use super::{
    EncodedRows, Error, ProjectRows, TransposedRows, accumulate_transposed_with,
    int8::{self, AttentionF32, AttentionMulti, Int8Rows, PackedRows, QuantizedInput},
    pq_accumulate_transposed_rows, pq_project_rows, project_with,
};
use crate::kernels::encoded::Encoding;
use std::fmt;

#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
unsafe extern "C" {
    fn pq_project_rows_power9_fma(
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
    fn pq_accumulate_transposed_rows_power9_fma(
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
    /// The FMA kernels when compiled in AND detected at runtime, else portable.
    Auto,
    Portable,
    /// Fails with [`Unavailable`] unless compiled in and detected at runtime.
    Power9Fma,
    /// Integer-dot (int8 activation) projections for quantized weights on
    /// POWER9 vmsummbm, F64 FMA for F32 rows, binary32-partial transposed
    /// products.
    /// NOT bit-compatible with the other backends (`docs/backends.md`).
    /// Fails with [`Unavailable`] like `Power9Fma`.
    Power9Int8,
    /// The portable scalar twin of `Power9Int8` (bit-identical to it), for
    /// tests and builds without POWER9 support.
    PortableInt8,
    /// `Power9Int8` with two activation planes: the second quantizes the
    /// residual of the first (about 254x finer), at twice the integer work.
    Power9Int8x2,
    /// The portable twin of `Power9Int8x2`.
    PortableInt8x2,
}

/// The forced FMA backend is not compiled into this build or the CPU lacks
/// ISA 3.0 (or the OS is not Linux).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unavailable;
impl fmt::Display for Unavailable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("POWER9 FMA matrix kernels unavailable (build or CPU)")
    }
}
impl std::error::Error for Unavailable {}

/// Resolved, copyable kernel pair. Only constructible through [`FmaKernels::new`].
#[derive(Debug, Clone, Copy)]
pub struct FmaKernels {
    name: &'static str,
    project: ProjectRows,
    transposed: TransposedRows,
    int8: Option<Int8Rows>,
    attention: Option<AttentionF32>,
    /// Several queries per call (native binary32 attention only).
    attention_multi: Option<AttentionMulti>,
    planes: usize,
    /// Native integer-dot kernels: weights can be packed once ([`Self::pack`]).
    packed: bool,
}

impl FmaKernels {
    pub fn new(request: Request) -> Result<Self, Unavailable> {
        match request {
            Request::Portable => Ok(Self::portable()),
            Request::Power9Fma => native().ok_or(Unavailable),
            Request::Auto => Ok(native().unwrap_or_else(Self::portable)),
            Request::Power9Int8 => native_int8().ok_or(Unavailable),
            Request::Power9Int8x2 => native_int8()
                .map(|k| Self {
                    name: "power9-int8x2",
                    planes: 2,
                    ..k
                })
                .ok_or(Unavailable),
            Request::PortableInt8x2 => Ok(Self {
                name: "portable-int8x2",
                planes: 2,
                ..Self::new(Request::PortableInt8)?
            }),
            Request::PortableInt8 => Ok(Self {
                name: "portable-int8",
                int8: Some(int8::pq8_project_rows),
                transposed: int8::pq_accumulate_transposed_rows_f32,
                attention: Some(int8::pa_attention_f32_heads),
                ..Self::portable()
            }),
        }
    }

    /// The portable kernels (always available; never native instructions).
    pub fn portable() -> Self {
        Self {
            name: "portable",
            project: pq_project_rows,
            transposed: pq_accumulate_transposed_rows,
            int8: None,
            attention: None,
            attention_multi: None,
            planes: 1,
            packed: false,
        }
    }

    /// True for the native POWER9 FMA pair.
    pub fn is_native(&self) -> bool {
        self.name != "portable"
    }

    /// `"portable"`, `"power9-fma"`, `"power9-int8"` or `"portable-int8"`.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// True for backends whose results are NOT bit-identical to the
    /// portable F64 kernels (the integer-dot backends). Callers may then use
    /// other documented, bounded approximations too.
    pub fn approximate(&self) -> bool {
        self.int8.is_some()
    }

    /// One query position's attention: [`crate::kernels::reference::attention`]
    /// (exact, F64) for exact backends, binary32 for approximate ones.
    pub fn attention(
        &self,
        query: &[f32],
        keys: &[f32],
        values: &[f32],
        kv_heads: usize,
        head_dim: usize,
        allowed: Option<&[bool]>,
    ) -> Result<Vec<f32>, crate::kernels::reference::Error> {
        self.attention_heads(query, keys, values, kv_heads, head_dim, allowed, None)
    }

    /// [`Self::attention`] for several queries over the same K/V, each with
    /// its own optional mask; every result equals the single-query call's.
    pub fn attention_multi(
        &self,
        queries: &[&[f32]],
        keys: &[f32],
        values: &[f32],
        kv_heads: usize,
        head_dim: usize,
        masks: Option<&[&[bool]]>,
    ) -> Result<Vec<Vec<f32>>, crate::kernels::reference::Error> {
        self.attention_multi_kv(
            queries,
            keys,
            values,
            kv_heads,
            head_dim,
            masks,
            0..kv_heads,
        )
    }

    /// [`Self::attention_multi`] for KV heads `kv` only: each output holds
    /// the query heads of those KV heads, each computed exactly as there.
    #[allow(clippy::too_many_arguments)]
    pub fn attention_multi_kv(
        &self,
        queries: &[&[f32]],
        keys: &[f32],
        values: &[f32],
        kv_heads: usize,
        head_dim: usize,
        masks: Option<&[&[bool]]>,
        kv: std::ops::Range<usize>,
    ) -> Result<Vec<Vec<f32>>, crate::kernels::reference::Error> {
        match self.attention_multi {
            Some(kernel) => int8::attention_multi_with(
                kernel, queries, keys, values, kv_heads, head_dim, masks, kv,
            ),
            None => {
                let group = queries
                    .first()
                    .map_or(0, |q| q.len() / head_dim.max(1) / kv_heads.max(1));
                queries
                    .iter()
                    .enumerate()
                    .map(|(i, q)| {
                        self.attention_heads(
                            q,
                            keys,
                            values,
                            kv_heads,
                            head_dim,
                            masks.map(|m| m[i]),
                            Some(kv.start * group..kv.end * group),
                        )
                    })
                    .collect()
            }
        }
    }

    /// Query heads `heads` (all if None) of [`Self::attention`], each computed
    /// exactly as there; the output holds only those heads.
    #[allow(clippy::too_many_arguments)]
    pub fn attention_heads(
        &self,
        query: &[f32],
        keys: &[f32],
        values: &[f32],
        kv_heads: usize,
        head_dim: usize,
        allowed: Option<&[bool]>,
        heads: Option<std::ops::Range<usize>>,
    ) -> Result<Vec<f32>, crate::kernels::reference::Error> {
        match self.attention {
            Some(kernel) => int8::attention_with(
                kernel, query, keys, values, kv_heads, head_dim, allowed, heads,
            ),
            None => crate::kernels::reference::attention_heads(
                query, keys, values, kv_heads, head_dim, allowed, heads,
            ),
        }
    }

    /// Activation planes per quantized projection (1, or 2 for the residual
    /// variants: the caller quantizes the residual rows as extra positions and
    /// adds the two outputs).
    pub fn planes(&self) -> usize {
        self.planes
    }

    /// True when projections of `encoding` weights take a [`QuantizedInput`]
    /// ([`Self::project_quantized`]) instead of F32 activations.
    pub fn quantizes(&self, encoding: Encoding) -> bool {
        self.int8.is_some() && int8::supports(encoding)
    }

    /// Integer-dot projection of every row of `weights`; only for kernels
    /// where [`Self::quantizes`] is true for the weights' encoding.
    pub fn project_quantized(
        &self,
        weights: &EncodedRows<'_>,
        input: &QuantizedInput,
        out: &mut [f32],
    ) -> Result<(), Error> {
        match self.int8 {
            Some(kernel) => int8::project_with(kernel, weights, input, out),
            None => Err(Error::Shape("backend has no integer-dot kernels")),
        }
    }

    /// True when `encoding` weights can be packed once for this backend
    /// ([`Self::pack`], [`Self::project_packed`]).
    pub fn packs(&self, encoding: Encoding) -> bool {
        self.packed && int8::supports(encoding)
    }

    /// Unpack `weights` once into the native integer-dot layout.
    pub fn pack(&self, weights: &EncodedRows<'_>) -> Result<PackedRows, Error> {
        #[cfg(all(feature = "power9", target_arch = "powerpc64"))]
        if self.packed {
            return int8::pack_power9(weights);
        }
        let _ = weights;
        Err(Error::Shape("backend has no packed integer-dot kernels"))
    }

    /// Integer-dot projection of packed rows `[row0, row0 + count)`, written
    /// to `out[p * out_stride + out_offset + (r - row0)]`; bit-identical to
    /// [`Self::project_quantized`] on the same rows.
    #[allow(clippy::too_many_arguments)]
    pub fn project_packed(
        &self,
        packed: &PackedRows,
        row0: usize,
        count: usize,
        input: &QuantizedInput,
        out: &mut [f32],
        out_stride: usize,
        out_offset: usize,
    ) -> Result<(), Error> {
        #[cfg(all(feature = "power9", target_arch = "powerpc64"))]
        if self.packed {
            return int8::project_packed_power9(
                packed, row0, count, input, out, out_stride, out_offset,
            );
        }
        let _ = (packed, row0, count, input, out, out_stride, out_offset);
        Err(Error::Shape("backend has no packed integer-dot kernels"))
    }

    /// Same contract and bit-identical results as [`super::project`].
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

    /// Same contract and bit-identical results as [`super::accumulate_transposed`].
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
fn native() -> Option<FmaKernels> {
    #[cfg(all(feature = "power9", target_arch = "powerpc64"))]
    if super::super::Kernels::new(super::super::Backend::Power9).is_ok() {
        return Some(FmaKernels {
            name: "power9-fma",
            project: pq_project_rows_power9_fma,
            transposed: pq_accumulate_transposed_rows_power9_fma,
            int8: None,
            attention: None,
            attention_multi: None,
            planes: 1,
            packed: false,
        });
    }
    None
}

/// The FMA kernels plus the POWER9 integer-dot projection, under the same
/// build and runtime conditions as [`native`].
fn native_int8() -> Option<FmaKernels> {
    #[cfg(all(feature = "power9", target_arch = "powerpc64"))]
    if let Some(fma) = native() {
        return Some(FmaKernels {
            name: "power9-int8",
            int8: Some(int8::pq8_project_rows_power9),
            transposed: int8::pq_accumulate_transposed_rows_power9_f32,
            attention: Some(int8::pa_attention_f32_power9_heads),
            attention_multi: Some(int8::pa_attention_f32_power9_multi_kv),
            packed: true,
            ..fma
        });
    }
    None
}

#[cfg(test)]
mod tests;
