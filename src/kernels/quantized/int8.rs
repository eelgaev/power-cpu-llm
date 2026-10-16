//! Integer-dot projections with int8-quantized activations (see
//! `docs/backends.md`).
//! NOT bit-compatible with the F64 kernels; the error is bounded per output by
//! `sum_c |w_c| * d_block(c) / 2` plus binary64/binary32 rounding.

use super::{EncodedRows, Error, status};
use crate::kernels::encoded::Encoding;
use std::ffi::c_int;

unsafe extern "C" {
    fn pq8_shared_quantize(
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
    ) -> c_int;
    fn pq8_shared_project_rows(
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
    ) -> c_int;
    pub(super) fn pa_attention_f32_heads(
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
    pub(super) fn pq_accumulate_transposed_rows_f32(
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
    fn pq8_input_bytes(kind: c_int, cols: usize, positions: usize) -> usize;
    fn pq8_quantize_group_range(
        kind: c_int,
        rows: *const *const f32,
        cols: usize,
        b0: usize,
        b1: usize,
        q: *mut u8,
        da: *mut f32,
        s: *mut i16,
        residual: *const *mut f32,
    ) -> c_int;
    fn pq8_quantize_group(
        kind: c_int,
        rows: *const *const f32,
        cols: usize,
        q: *mut u8,
        da: *mut f32,
        s: *mut i16,
        residual: *const *mut f32,
    ) -> c_int;
    fn pq8_quantize(
        kind: c_int,
        x: *const f32,
        positions: usize,
        cols: usize,
        input: *mut u8,
        input_len: usize,
    ) -> c_int;
    pub(super) fn pq8_project_rows(
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
    ) -> c_int;
}
#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
unsafe extern "C" {

    fn pq8_shared_quantize_power9(
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
    ) -> c_int;
    fn pq8_shared_project_rows_power9(
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
    ) -> c_int;
    pub(super) fn pq8_quantize_group_power9(
        kind: c_int,
        rows: *const *const f32,
        cols: usize,
        q: *mut u8,
        da: *mut f32,
        s: *mut i16,
        residual: *const *mut f32,
    ) -> c_int;
    pub(super) fn pq8_quantize_group_range_power9(
        kind: c_int,
        rows: *const *const f32,
        cols: usize,
        b0: usize,
        b1: usize,
        q: *mut u8,
        da: *mut f32,
        s: *mut i16,
        residual: *const *mut f32,
    ) -> c_int;
    pub(super) fn pa_attention_f32_power9_heads(
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
    pub(super) fn pq_accumulate_transposed_rows_power9_f32(
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
    pub(super) fn pq8_project_rows_power9(
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
    ) -> c_int;
}

pub(super) type Int8Rows = unsafe extern "C" fn(
    c_int,
    *const u8,
    usize,
    usize,
    usize,
    *const u8,
    usize,
    usize,
    *mut f32,
    usize,
    usize,
) -> c_int;

pub(super) type QuantizeGroupFn = unsafe extern "C" fn(
    c_int,
    *const *const f32,
    usize,
    *mut u8,
    *mut f32,
    *mut i16,
    *const *mut f32,
) -> c_int;

type QuantizeRangeFn = unsafe extern "C" fn(
    c_int,
    *const *const f32,
    usize,
    usize,
    usize,
    *mut u8,
    *mut f32,
    *mut i16,
    *const *mut f32,
) -> c_int;
/// The block-range group quantizer (see [`quantize_group_fn`]).
fn quantize_range_fn() -> QuantizeRangeFn {
    static RESOLVED: std::sync::OnceLock<QuantizeRangeFn> = std::sync::OnceLock::new();
    *RESOLVED.get_or_init(|| {
        #[cfg(all(feature = "power9", target_arch = "powerpc64"))]
        if crate::kernels::Kernels::new(crate::kernels::Backend::Power9).is_ok() {
            return pq8_quantize_group_range_power9 as QuantizeRangeFn;
        }
        pq8_quantize_group_range as QuantizeRangeFn
    })
}

/// Block-parallel two-plane quantization of one group (see
/// [`QuantizeGroup::quantize_two_planes`]): threads call
/// [`Self::quantize_blocks`] on disjoint block ranges covering `0..blocks()`;
/// the result is the same bytes as the one-call form.
pub struct TwoPlaneWriter<'a> {
    encoding: Encoding,
    cols: usize,
    pointers: [*const f32; 4],
    outputs: [*mut f32; 4],
    q: *mut u8,
    da: *mut f32,
    s: *mut i16,
    whole: *const QuantizedInput,
    _borrow: std::marker::PhantomData<&'a mut [u8]>,
}
// SAFETY: threads write disjoint block ranges of q/da/s and the residual rows
// (block b of every row only), and read only the input rows and their own
// blocks of the residual rows (see quantize_blocks).
unsafe impl Send for TwoPlaneWriter<'_> {}
unsafe impl Sync for TwoPlaneWriter<'_> {}
impl TwoPlaneWriter<'_> {
    /// Quantization blocks of a row.
    pub fn blocks(&self) -> usize {
        let block = match self.encoding {
            Encoding::Q4K | Encoding::Q5K | Encoding::Q6K | Encoding::TQ2_0 => 256,
            _ => 32,
        };
        self.cols / block
    }
    /// The quantized input, to read once every block has been written.
    ///
    /// # Safety
    /// Every [`Self::quantize_blocks`] call must have returned (and its
    /// writes be visible to this thread) and none may follow.
    pub unsafe fn input(&self) -> &QuantizedInput {
        // SAFETY: per the contract; the pointer comes from the writer's
        // exclusive borrow of the input, now only read.
        unsafe { &*self.whole }
    }
    /// Blocks `[b0, b1)` of every row.
    ///
    /// # Safety
    /// Concurrent calls must use disjoint ranges, and the writer's borrows
    /// must stay alive until every call has returned.
    pub unsafe fn quantize_blocks(&self, b0: usize, b1: usize) -> Result<(), Error> {
        // SAFETY: per the contract above; C bounds-checks the range.
        status(unsafe {
            quantize_range_fn()(
                kernel_kind(self.encoding),
                self.pointers.as_ptr(),
                self.cols,
                b0,
                b1,
                self.q,
                self.da,
                self.s,
                self.outputs.as_ptr(),
            )
        })
    }
}

/// Block-parallel two-plane quantization of P positions (P a multiple of 4)
/// into a [`QuantizedInput`] of 2P positions: plane 1 (the rows) at positions
/// 0..P, plane 2 (their residuals `x - da * a`) at P..2P. Threads call
/// [`Self::quantize`] on disjoint (group, block range) items covering every
/// group and block. The normal writer gives the same bytes as quantizing
/// the rows with residuals, then [rows, residuals] again. The experimental
/// [`QuantizedInput::shared_planes_writer`] instead splits a 15-bit integer
/// into two signed-byte planes sharing a power-of-two scale ratio.
pub struct PlanesWriter<'a> {
    encoding: Encoding,
    cols: usize,
    rows: &'a [&'a [f32]],
    /// Per group of the 2P-position input: q, da and s base addresses.
    groups: Vec<(*mut u8, *mut f32, *mut i16)>,
    shared_scale: bool,
}
// SAFETY: concurrent quantize calls write disjoint (group, block) regions of
// q/da/s (plane 1 group g and plane 2 group P/4 + g, blocks [b0, b1) only)
// and read only the shared input rows and their own residual scratch.
unsafe impl Send for PlanesWriter<'_> {}
unsafe impl Sync for PlanesWriter<'_> {}
impl PlanesWriter<'_> {
    /// Quantization blocks of a row.
    pub fn blocks(&self) -> usize {
        let block = match self.encoding {
            Encoding::Q4K | Encoding::Q5K | Encoding::Q6K | Encoding::TQ2_0 => 256,
            _ => 32,
        };
        self.cols / block
    }
    /// Groups of 4 positions per plane (P / 4).
    pub fn groups(&self) -> usize {
        self.groups.len() / 2
    }
    /// Blocks `[b0, b1)` of plane-1 group `g` and of its plane-2 group;
    /// `residual` is scratch of at least 4 x cols values.
    ///
    /// # Safety
    /// Concurrent calls must use distinct groups or disjoint block ranges,
    /// and the writer's borrows must stay alive until every call returned.
    pub unsafe fn quantize(
        &self,
        g: usize,
        b0: usize,
        b1: usize,
        residual: &mut [f32],
    ) -> Result<(), Error> {
        let plane = self.groups();
        if g >= plane || b0 > b1 || b1 > self.blocks() || residual.len() < 4 * self.cols {
            return Err(Error::Shape("two-plane quantization item"));
        }
        let block = self.cols / self.blocks();
        let mut pointers = [std::ptr::null::<f32>(); 4];
        let mut outputs = [std::ptr::null_mut::<f32>(); 4];
        for (l, (pointer, output)) in pointers.iter_mut().zip(outputs.iter_mut()).enumerate() {
            let row = self.rows[4 * g + l];
            if !crate::kernels::reference::all_finite(&row[b0 * block..b1 * block]) {
                return Err(Error::NonFiniteInput);
            }
            *pointer = row.as_ptr();
            // SAFETY: row l of the scratch (4 x cols values).
            *output = unsafe { residual.as_mut_ptr().add(l * self.cols) };
        }
        if self.shared_scale {
            let (q1, d1, s1) = self.groups[g];
            let (q2, d2, s2) = self.groups[plane + g];
            // SAFETY: the writer holds two distinct groups; only the item's
            // disjoint block ranges are written, with four checked input rows.
            return status(unsafe {
                shared_quantize_fn()(pointers.as_ptr(), self.cols, b0, b1, q1, d1, s1, q2, d2, s2)
            });
        }
        let run = |(q, da, s): (*mut u8, *mut f32, *mut i16),
                   rows: &[*const f32; 4],
                   out: &[*mut f32; 4]| {
            // SAFETY: rows are cols long; q/da/s are one group's regions and
            // only blocks [b0, b1) are written (C bounds-checks the range).
            status(unsafe {
                quantize_range_fn()(
                    kernel_kind(self.encoding),
                    rows.as_ptr(),
                    self.cols,
                    b0,
                    b1,
                    q,
                    da,
                    s,
                    out.as_ptr(),
                )
            })
        };
        run(self.groups[g], &pointers, &outputs)?;
        let residual_rows = outputs.map(|o| o.cast_const());
        run(
            self.groups[plane + g],
            &residual_rows,
            &[std::ptr::null_mut(); 4],
        )
    }
}

type SharedQuantizeFn = unsafe extern "C" fn(
    *const *const f32,
    usize,
    usize,
    usize,
    *mut u8,
    *mut f32,
    *mut i16,
    *mut u8,
    *mut f32,
    *mut i16,
) -> c_int;
fn shared_quantize_fn() -> SharedQuantizeFn {
    static RESOLVED: std::sync::OnceLock<SharedQuantizeFn> = std::sync::OnceLock::new();
    *RESOLVED.get_or_init(|| {
        #[cfg(all(feature = "power9", target_arch = "powerpc64"))]
        if crate::kernels::Kernels::new(crate::kernels::Backend::Power9).is_ok() {
            return pq8_shared_quantize_power9 as SharedQuantizeFn;
        }
        pq8_shared_quantize as SharedQuantizeFn
    })
}

/// The group quantizer: the POWER9 version when compiled in and detected
/// (bit-identical to the portable one), else portable. Resolved once.
pub(super) fn quantize_group_fn() -> QuantizeGroupFn {
    static RESOLVED: std::sync::OnceLock<QuantizeGroupFn> = std::sync::OnceLock::new();
    *RESOLVED.get_or_init(|| {
        #[cfg(all(feature = "power9", target_arch = "powerpc64"))]
        if crate::kernels::Kernels::new(crate::kernels::Backend::Power9).is_ok() {
            return pq8_quantize_group_power9 as QuantizeGroupFn;
        }
        pq8_quantize_group as QuantizeGroupFn
    })
}

#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
unsafe extern "C" {
    pub(super) fn pa_attention_f32_power9_multi_kv(
        query: *const f32,
        nq: usize,
        q_stride: usize,
        keys: *const f32,
        values: *const f32,
        qheads: usize,
        kvheads: usize,
        dim: usize,
        nkeys: usize,
        allowed: *const u8,
        mask_stride: usize,
        scratch: *mut f32,
        out: *mut f32,
        o_stride: usize,
        kh_first: usize,
        kh_end: usize,
    ) -> bool;
}

/// Several queries over the same K/V (`pa_attention_f32_power9_multi_kv`).
pub(super) type AttentionMulti = unsafe extern "C" fn(
    *const f32,
    usize,
    usize,
    *const f32,
    *const f32,
    usize,
    usize,
    usize,
    usize,
    *const u8,
    usize,
    *mut f32,
    *mut f32,
    usize,
    usize,
    usize,
) -> bool;

/// [`attention_with`] for several queries (each with its own optional mask)
/// in one call; every result equals the single-query kernel's.
#[allow(clippy::too_many_arguments)]
pub(super) fn attention_multi_with(
    kernel: AttentionMulti,
    queries: &[&[f32]],
    keys: &[f32],
    values: &[f32],
    kv_heads: usize,
    head_dim: usize,
    masks: Option<&[&[bool]]>,
    kv: std::ops::Range<usize>,
) -> Result<Vec<Vec<f32>>, crate::kernels::reference::Error> {
    use crate::kernels::reference::Error as E;
    let Some(first) = queries.first() else {
        return Ok(Vec::new());
    };
    if kv.is_empty() || kv.end > kv_heads {
        return Err(E::InvalidShape);
    }
    let width = kv_heads
        .checked_mul(head_dim)
        .filter(|&w| w > 0)
        .ok_or(E::InvalidShape)?;
    let qlen = first.len();
    if qlen == 0
        || !head_dim.is_multiple_of(16)
        || qlen % head_dim != 0
        || queries.iter().any(|q| q.len() != qlen)
        || keys.is_empty()
        || !keys.len().is_multiple_of(width)
        || values.len() != keys.len()
        || !(qlen / head_dim).is_multiple_of(kv_heads)
    {
        return Err(E::InvalidShape);
    }
    let nkeys = keys.len() / width;
    if let Some(masks) = masks {
        if masks.len() != queries.len() || masks.iter().any(|m| m.len() != nkeys) {
            return Err(E::InvalidShape);
        }
        if masks.iter().any(|m| !m.iter().any(|&v| v)) {
            return Err(E::InvalidParameter);
        }
    }
    if !queries.iter().all(|q| q.iter().all(|x| x.is_finite())) {
        return Err(E::NonFiniteValue);
    }
    let flat: Vec<f32> = queries.concat();
    let mask: Option<Vec<u8>> = masks.map(|m| {
        m.iter()
            .flat_map(|row| row.iter().map(|&b| u8::from(b)))
            .collect()
    });
    let group = qlen / head_dim / kv_heads;
    let olen = kv.len() * group * head_dim;
    // Scores of every row, plus one packed V key block (see the kernel).
    let block = (32768 / head_dim).max(16) * head_dim;
    let mut scratch = vec![0.0; queries.len() * group * nkeys + block];
    let mut out = vec![0.0; queries.len() * olen];
    // SAFETY: checked dimensions; queries are nq rows of qlen floats, out nq
    // rows of olen (the query heads of KV heads kv), K/V nkeys * kv_heads *
    // head_dim, masks nq rows of nkeys, scratch nq * group * nkeys + block.
    let ok = unsafe {
        kernel(
            flat.as_ptr(),
            queries.len(),
            qlen,
            keys.as_ptr(),
            values.as_ptr(),
            qlen / head_dim,
            kv_heads,
            head_dim,
            nkeys,
            mask.as_ref().map_or(std::ptr::null(), |m| m.as_ptr()),
            nkeys,
            scratch.as_mut_ptr(),
            out.as_mut_ptr(),
            olen,
            kv.start,
            kv.end,
        )
    };
    if !ok {
        return Err(E::NonFiniteValue);
    }
    Ok(out.chunks_exact(olen).map(<[f32]>::to_vec).collect())
}

/// The `*_heads` binary32 attention kernels (query heads `[h_first, h_end)`).
pub(super) type AttentionF32 = unsafe extern "C" fn(
    *const f32,
    *const f32,
    *const f32,
    usize,
    usize,
    usize,
    usize,
    *const u8,
    *mut f32,
    *mut f32,
    usize,
    usize,
) -> bool;

/// Binary32 attention (`native/include/power_llm_attention_f32.h`) with the
/// argument contract of [`crate::kernels::reference::attention`]. K/V are not
/// re-scanned for finiteness (they come from checked projections); a
/// nonfinite score or output is an error.
#[allow(clippy::too_many_arguments)]
pub(super) fn attention_with(
    kernel: AttentionF32,
    query: &[f32],
    keys: &[f32],
    values: &[f32],
    kv_heads: usize,
    head_dim: usize,
    allowed: Option<&[bool]>,
    heads: Option<std::ops::Range<usize>>,
) -> Result<Vec<f32>, crate::kernels::reference::Error> {
    use crate::kernels::reference::Error as E;
    let width = kv_heads
        .checked_mul(head_dim)
        .filter(|&w| w > 0)
        .ok_or(E::InvalidShape)?;
    if query.is_empty()
        || !head_dim.is_multiple_of(16)
        || !query.len().is_multiple_of(head_dim)
        || keys.is_empty()
        || !keys.len().is_multiple_of(width)
        || values.len() != keys.len()
    {
        return Err(E::InvalidShape);
    }
    let qheads = query.len() / head_dim;
    let nkeys = keys.len() / width;
    if !qheads.is_multiple_of(kv_heads) {
        return Err(E::InvalidShape);
    }
    if let Some(mask) = allowed {
        if mask.len() != nkeys {
            return Err(E::InvalidShape);
        }
        if !mask.iter().any(|&v| v) {
            return Err(E::InvalidParameter);
        }
    }
    let heads = heads.unwrap_or(0..qheads);
    if heads.start > heads.end || heads.end > qheads {
        return Err(E::InvalidShape);
    }
    if !query.iter().all(|x| x.is_finite()) {
        return Err(E::NonFiniteValue);
    }
    let mask: Option<Vec<u8>> = allowed.map(|m| m.iter().map(|&b| u8::from(b)).collect());
    // Two score rows: the kernel runs two heads of a KV group per pass.
    let mut scratch = vec![0.0; 2 * nkeys];
    let mut out = vec![0.0; heads.len() * head_dim];
    // SAFETY: checked dimensions; K/V hold nkeys * kv_heads * head_dim floats,
    // query/out qheads * head_dim, scratch 2 * nkeys; optional 0/1 mask of nkeys.
    let ok = unsafe {
        kernel(
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
    if ok { Ok(out) } else { Err(E::NonFiniteValue) }
}

/// Whether `encoding` has an integer-dot kernel (the quantized encodings
/// except Q5_1; F32, F16 and BF16 rows always use the F64 kernels).
pub fn supports(encoding: Encoding) -> bool {
    !matches!(encoding, Encoding::F32 | Encoding::F16 | Encoding::BF16)
}

/// The C kernels' encoding for `encoding`: Q5_1 runs as Q5_0 and Q5_K as
/// Q6_K (see [`split_offsets`]); activations are laid out identically.
fn kernel_kind(encoding: Encoding) -> c_int {
    (match encoding {
        Encoding::Q5_1 => Encoding::Q5_0,
        Encoding::Q5K => Encoding::Q6K,
        other => other,
    }) as c_int
}

/// Encodings the kernels run as another encoding plus per-32-element offsets:
/// (the other encoding's bytes, offsets transposed to `[cols / 32][rows]` so
/// that [`add_offsets`] streams them across rows).
fn split_offsets(encoding: Encoding, bytes: &[u8], cols: usize) -> Option<(Vec<u8>, Vec<f32>)> {
    let (bytes, offsets) = match encoding {
        Encoding::Q5_1 => split_q5_1(bytes),
        Encoding::Q5K => split_q5_k(bytes),
        _ => return None,
    };
    let nb = cols / 32;
    let rows = offsets.len() / nb;
    let mut transposed = vec![0.0f32; offsets.len()];
    for (r, row) in offsets.chunks_exact(nb).enumerate() {
        for (j, &o) in row.iter().enumerate() {
            transposed[j * rows + r] = o;
        }
    }
    Some((bytes, transposed))
}

/// Q5_K blocks (per 32 values `d sc q - dmin m`, q in 0..31) as Q6_K blocks
/// (per 16 values `d sc (q6 - 32)`) with `q6 = q + 32`, the same `d` and each
/// 32-value scale for both of its 16-value halves (exact), plus the offset
/// `-dmin m` of every 32 values. Returns the Q6_K bytes and the offsets.
pub(crate) fn split_q5_k(bytes: &[u8]) -> (Vec<u8>, Vec<f32>) {
    use crate::kernels::portable::dequant::half;
    let blocks = bytes.len() / 176;
    let mut q6 = Vec::with_capacity(blocks * 210);
    let mut offsets = Vec::with_capacity(blocks * 8);
    for block in bytes.as_chunks::<176>().0 {
        let (dmin, scales) = (half(&block[2..]), &block[4..16]);
        let (high, low) = (&block[16..48], &block[48..]);
        let mut ql = [0u8; 128];
        let mut qh = [0u8; 64];
        let mut sc = [0i8; 16];
        for g in 0..8 {
            let (scale, min) = if g < 4 {
                (scales[g] & 63, scales[g + 4] & 63)
            } else {
                (
                    (scales[g + 4] & 15) | ((scales[g - 4] >> 6) << 4),
                    (scales[g + 4] >> 4) | ((scales[g] >> 6) << 4),
                )
            };
            offsets.push(-(dmin * f32::from(min)));
            // Q6_K element e = half * 128 + k * 32 + l uses scale
            // half * 8 + 2k + l / 16.
            let (h, k) = (g / 4, g % 4);
            sc[h * 8 + 2 * k] = scale as i8;
            sc[h * 8 + 2 * k + 1] = scale as i8;
            for l in 0..32 {
                let packed = low[(g / 2) * 32 + l];
                let nibble = if g % 2 == 0 { packed & 15 } else { packed >> 4 };
                let q = (nibble | ((high[l] >> g) & 1) << 4) + 32;
                let at = h * 64 + l + if k % 2 == 1 { 32 } else { 0 };
                ql[at] |= if k < 2 { q & 15 } else { (q & 15) << 4 };
                qh[h * 32 + l] |= (q >> 4) << (2 * k);
            }
        }
        q6.extend_from_slice(&ql);
        q6.extend_from_slice(&qh);
        q6.extend(sc.iter().map(|&x| x as u8));
        q6.extend_from_slice(&block[..2]);
    }
    (q6, offsets)
}

/// Q5_1 blocks (`d * q + m`) as Q5_0 blocks (`d * (q - 16)`: the same scale
/// and five-bit values) plus a per-block offset `m + 16 d`, so that
/// `sum((d q + m) x) = (Q5_0 dot) + offset * sum(x)`. Returns the Q5_0 bytes
/// and the offsets in block order (`[rows][blocks]`).
pub(crate) fn split_q5_1(bytes: &[u8]) -> (Vec<u8>, Vec<f32>) {
    use crate::kernels::portable::dequant::half;
    let blocks = bytes.len() / 24;
    let mut q5_0 = Vec::with_capacity(blocks * 22);
    let mut offsets = Vec::with_capacity(blocks);
    for block in bytes.as_chunks::<24>().0 {
        q5_0.extend_from_slice(&block[..2]);
        q5_0.extend_from_slice(&block[4..]);
        offsets.push(half(&block[2..]) + 16.0 * half(block));
    }
    (q5_0, offsets)
}

/// Adds `offsets[b][row0 + r] * sum(x[p][block b])` (offsets `[blocks][rows]`)
/// to `out[p * out_stride + out_offset + r]` for `r` in `0..count`,
/// with `x` the quantized activations of `input` (`da * a`, 32-element
/// blocks, so the Q5_0 projection plus this is the Q5_1 projection of the
/// same quantized activations). A nonfinite result is an error, as from
/// the kernels.
#[allow(clippy::too_many_arguments)]
fn add_offsets(
    offsets: &[f32],
    rows: usize,
    row0: usize,
    count: usize,
    input: &QuantizedInput,
    out: &mut [f32],
    out_stride: usize,
    out_offset: usize,
) -> Result<(), Error> {
    let (cols, nb) = (input.cols, input.cols / 32);
    let bytes = input.as_bytes();
    let (_, da_at, s_at) = input.regions(input.positions);
    let float = |at: usize| f32::from_ne_bytes(bytes[at..at + 4].try_into().expect("4 bytes"));
    // Every position's 32-value block sums first, so each row's offsets are
    // read once for all positions.
    let positions = input.positions;
    let mut sums = vec![0.0f32; positions * nb];
    for (p, sums) in sums.chunks_exact_mut(nb).enumerate() {
        let (g, l) = (p / 4, p % 4);
        if input.encoding == Encoding::Q5K {
            // K layout: one scale per 256 values (`da[group][block][l]`) and
            // the quantizer's int16 sum of every 32 values (`s`, see
            // `pq8_quantize_group_range`).
            let blocks = cols / 256;
            for (j, sum) in sums.iter_mut().enumerate() {
                let (b, sub) = (j / 8, j % 8);
                let da = float(da_at + ((g * blocks + b) * 4 + l) * 4);
                let at = s_at + (g * blocks * 32 + ((b * 4 + sub / 2) * 4 + l) * 2 + sub % 2) * 2;
                let a = i16::from_ne_bytes([bytes[at], bytes[at + 1]]);
                *sum = da * f32::from(a);
            }
            continue;
        }
        // Group g: [4][cols * 4] bytes (each 4-element chunk 4 times).
        let q = &bytes[g * 16 * cols + l * cols * 4..][..cols * 4];
        for (b, sum) in sums.iter_mut().enumerate() {
            let a: i32 = (0..32)
                .map(|i| i32::from(q[b * 128 + (i / 4) * 16 + i % 4] as i8))
                .sum();
            *sum = float(da_at + ((g * nb + b) * 4 + l) * 4) * a as f32;
        }
    }
    // Block by block across the rows: contiguous multiply-adds (vectorized),
    // each offset column read once for every position.
    for j in 0..nb {
        let column = &offsets[j * rows + row0..][..count];
        for p in 0..positions {
            let sum = sums[p * nb + j];
            let dst = &mut out[p * out_stride + out_offset..][..count];
            for (d, &o) in dst.iter_mut().zip(column) {
                *d += o * sum;
            }
        }
    }
    for p in 0..positions {
        if !crate::kernels::reference::all_finite(&out[p * out_stride + out_offset..][..count]) {
            return Err(Error::NonFiniteOutput);
        }
    }
    Ok(())
}

/// Activations `[positions, cols]` quantized for one weight encoding.
#[derive(Debug)]
pub struct QuantizedInput {
    encoding: Encoding,
    cols: usize,
    positions: usize,
    len: usize,
    /// u128 storage: 16-byte aligned, as the native kernels require.
    storage: Vec<u128>,
    shared_scale: bool,
}

impl QuantizedInput {
    /// Quantize finite `x` (`[positions, cols]`) for weights in `encoding`.
    pub fn new(
        encoding: Encoding,
        x: &[f32],
        positions: usize,
        cols: usize,
    ) -> Result<Self, Error> {
        if !supports(encoding) {
            return Err(Error::Shape("integer-dot input for F32 weights"));
        }
        if positions == 0 || positions.checked_mul(cols) != Some(x.len()) {
            return Err(Error::Shape("activations must be [positions, cols]"));
        }
        if !x.iter().all(|v| v.is_finite()) {
            return Err(Error::NonFiniteInput);
        }
        // SAFETY: pure size computation.
        let len = unsafe { pq8_input_bytes(kernel_kind(encoding), cols, positions) };
        if len == 0 {
            return Err(Error::Shape("columns are not a block multiple"));
        }
        let mut storage = Vec::new();
        storage
            .try_reserve_exact(len.div_ceil(16))
            .map_err(|_| Error::Overflow)?;
        storage.resize(len.div_ceil(16), 0);
        // SAFETY: x is [positions, cols]; storage holds >= len bytes.
        status(unsafe {
            pq8_quantize(
                kernel_kind(encoding),
                x.as_ptr(),
                positions,
                cols,
                storage.as_mut_ptr().cast(),
                len,
            )
        })?;
        Ok(Self {
            encoding,
            cols,
            positions,
            len,
            storage,
            shared_scale: false,
        })
    }
    /// The quantized storage (layout of power_llm_quantized_int8.h).
    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: storage holds at least len initialized bytes.
        unsafe { std::slice::from_raw_parts(self.storage.as_ptr().cast::<u8>(), self.len) }
    }
    pub fn encoding(&self) -> Encoding {
        self.encoding
    }
    pub fn positions(&self) -> usize {
        self.positions
    }

    /// Storage for `positions` x `cols` to be filled group by group through
    /// [`Self::groups_mut`]; every group must be quantized before use.
    pub fn allocate(encoding: Encoding, positions: usize, cols: usize) -> Result<Self, Error> {
        if !supports(encoding) || positions == 0 {
            return Err(Error::Shape(
                "integer-dot input for F32 weights or zero positions",
            ));
        }
        // SAFETY: pure size computation.
        let len = unsafe { pq8_input_bytes(kernel_kind(encoding), cols, positions) };
        if len == 0 {
            return Err(Error::Shape("columns are not a block multiple"));
        }
        let mut storage = Vec::new();
        storage
            .try_reserve_exact(len.div_ceil(16))
            .map_err(|_| Error::Overflow)?;
        storage.resize(len.div_ceil(16), 0);
        Ok(Self {
            encoding,
            cols,
            positions,
            len,
            storage,
            shared_scale: false,
        })
    }

    /// A [`TwoPlaneWriter`] for the two planes of `rows` (one or two rows;
    /// this input holds 2 x rows positions, one group), with `residual`.
    pub fn two_plane_writer<'a>(
        &'a mut self,
        rows: &'a [&'a [f32]],
        residual: &'a mut [f32],
    ) -> Result<TwoPlaneWriter<'a>, Error> {
        if rows.is_empty() || rows.len() > 2 || self.positions != 2 * rows.len() {
            return Err(Error::Shape("two-plane input positions"));
        }
        let whole: *const QuantizedInput = self;
        let group = self
            .groups_mut()
            .into_iter()
            .next()
            .ok_or(Error::Shape("quantized group"))?;
        group.two_plane_writer(rows, residual, whole)
    }

    /// As [`Self::two_plane_writer`] over raw rows (one or two, `cols` each)
    /// and a residual buffer of rows x cols, for inputs that are still being
    /// written block by block when the writer is made.
    ///
    /// # Safety
    /// Block b of every row must be written (and finite) before
    /// [`TwoPlaneWriter::quantize_blocks`] covers block b, by the same thread
    /// or with synchronization; rows and residual must stay valid and
    /// otherwise unaliased while the writer is used.
    pub unsafe fn two_plane_writer_from_ptrs(
        &mut self,
        rows: &[*const f32],
        residual: *mut f32,
    ) -> Result<TwoPlaneWriter<'_>, Error> {
        let p = rows.len();
        if p == 0 || p > 2 || self.positions != 2 * p {
            return Err(Error::Shape("two-plane input positions"));
        }
        let whole: *const QuantizedInput = self;
        let cols = self.cols;
        let encoding = self.encoding;
        let group = self
            .groups_mut()
            .into_iter()
            .next()
            .ok_or(Error::Shape("quantized group"))?;
        let mut pointers = [std::ptr::null::<f32>(); 4];
        let mut outputs = [std::ptr::null_mut::<f32>(); 4];
        for (l, &row) in rows.iter().enumerate() {
            pointers[l] = row;
            // SAFETY: row l of the caller's residual buffer (p rows of cols).
            let r = unsafe { residual.add(l * cols) };
            outputs[l] = r;
            pointers[p + l] = r;
        }
        Ok(TwoPlaneWriter {
            encoding,
            cols,
            pointers,
            outputs,
            q: group.q.as_mut_ptr(),
            da: group.da.as_mut_ptr(),
            s: if group.s.is_empty() {
                std::ptr::null_mut()
            } else {
                group.s.as_mut_ptr()
            },
            whole,
            _borrow: std::marker::PhantomData,
        })
    }

    /// A [`PlanesWriter`] for the two planes of `rows` (P rows, P a
    /// multiple of 4; this input holds 2P positions).
    pub fn planes_writer<'a>(
        &'a mut self,
        rows: &'a [&'a [f32]],
    ) -> Result<PlanesWriter<'a>, Error> {
        self.planes_writer_mode(rows, false)
    }

    fn planes_writer_mode<'a>(
        &'a mut self,
        rows: &'a [&'a [f32]],
        shared_scale: bool,
    ) -> Result<PlanesWriter<'a>, Error> {
        let p = rows.len();
        if p == 0 || !p.is_multiple_of(4) || self.positions != 2 * p {
            return Err(Error::Shape("two-plane input positions"));
        }
        if rows.iter().any(|r| r.len() != self.cols) {
            return Err(Error::Shape("activation row length"));
        }
        let (encoding, cols) = (self.encoding, self.cols);
        let groups = self
            .groups_mut()
            .into_iter()
            .map(|group| {
                (
                    group.q.as_mut_ptr(),
                    group.da.as_mut_ptr(),
                    if group.s.is_empty() {
                        std::ptr::null_mut()
                    } else {
                        group.s.as_mut_ptr()
                    },
                )
            })
            .collect();
        self.shared_scale = shared_scale;
        Ok(PlanesWriter {
            encoding,
            cols,
            rows,
            groups,
            shared_scale,
        })
    }

    /// Experimental Q4_0 prompt quantization with d2 = d1/256 and a
    /// fused integer dot. Other encodings and one-token decode stay unchanged.
    pub fn shared_planes_writer<'a>(
        &'a mut self,
        rows: &'a [&'a [f32]],
    ) -> Result<PlanesWriter<'a>, Error> {
        if self.encoding != Encoding::Q4_0
            || !rows.len().is_multiple_of(4)
            || rows.is_empty()
            || self.positions != 2 * rows.len()
            || rows.iter().any(|r| r.len() != self.cols)
        {
            return Err(Error::Shape("shared-scale Q4_0 input"));
        }
        self.planes_writer_mode(rows, true)
    }

    /// A new input whose position i is position `positions[i]` of this one
    /// (bytes, scale and sums copied): the same bytes as quantizing those rows
    /// again, since every position is quantized independently. Padding
    /// positions are those of a missing row.
    pub fn gather(&self, positions: &[usize]) -> Result<Self, Error> {
        if positions.iter().any(|&p| p >= self.positions) {
            return Err(Error::Shape("gathered position out of range"));
        }
        if self.shared_scale {
            let p = positions.len() / 2;
            let old_p = self.positions / 2;
            if !positions.len().is_multiple_of(2)
                || positions[..p].iter().any(|&i| i >= old_p)
                || (0..p).any(|i| positions[p + i] != positions[i] + old_p)
            {
                return Err(Error::Shape("shared-scale gather must retain plane pairs"));
            }
        }
        let mut out = Self::allocate(self.encoding, positions.len(), self.cols)?;
        out.shared_scale = self.shared_scale;
        let (nb, sums) = self.block_layout();
        let src = self.regions(self.positions);
        let dst = out.regions(positions.len());
        let q_row = 4 * self.cols;
        // A missing row's stored bytes: a = 0 (Q6_K stores a + 128).
        let zero = if kernel_kind(self.encoding) == Encoding::Q6K as c_int {
            0x80u8
        } else {
            0
        };
        let base = self.storage.as_ptr().cast::<u8>();
        let out_base = out.storage.as_mut_ptr().cast::<u8>();
        // SAFETY: regions() offsets lie inside each input's `len` bytes (the C
        // layout); source and destination are different allocations; every
        // index below is bounded by its input's padded position count.
        unsafe {
            let padded = positions.len().div_ceil(4) * 4;
            for i in positions.len()..padded {
                std::ptr::write_bytes(out_base.add(dst.0 + i * q_row), zero, q_row);
            }
            for (i, &p) in positions.iter().enumerate() {
                std::ptr::copy_nonoverlapping(
                    base.add(src.0 + p * q_row),
                    out_base.add(dst.0 + i * q_row),
                    q_row,
                );
                let (sda, dda) = (
                    base.add(src.1).cast::<f32>(),
                    out_base.add(dst.1).cast::<f32>(),
                );
                let (ss, ds) = (
                    base.add(src.2).cast::<i16>(),
                    out_base.add(dst.2).cast::<i16>(),
                );
                for b in 0..nb {
                    let from = ((p / 4) * nb + b) * 4 + p % 4;
                    let to = ((i / 4) * nb + b) * 4 + i % 4;
                    *dda.add(to) = *sda.add(from);
                    if sums == 4 {
                        *ds.add(to) = *ss.add(from);
                    } else {
                        for j in 0..4 {
                            for k in 0..2 {
                                let from = ((((p / 4) * nb + b) * 4 + j) * 4 + p % 4) * 2 + k;
                                let to = ((((i / 4) * nb + b) * 4 + j) * 4 + i % 4) * 2 + k;
                                *ds.add(to) = *ss.add(from);
                            }
                        }
                    }
                }
            }
        }
        Ok(out)
    }
    /// Blocks per row and int16 sums per (group, block).
    fn block_layout(&self) -> (usize, usize) {
        let (block, k_type) = match self.encoding {
            Encoding::Q4K | Encoding::Q5K | Encoding::Q6K => (256, true),
            Encoding::TQ2_0 => (256, false),
            _ => (32, false),
        };
        (self.cols / block, if k_type { 32 } else { 4 })
    }
    /// Byte offsets of q, da and s for `positions` positions (C layout).
    fn regions(&self, positions: usize) -> (usize, usize, usize) {
        let (nb, _) = self.block_layout();
        let groups = positions.div_ceil(4);
        let q_bytes = (groups * 4 * self.cols * 4).div_ceil(16) * 16;
        (0, q_bytes, q_bytes + groups * nb * 4 * 4)
    }

    /// The disjoint groups of 4 positions (group g holds positions 4g..4g+3).
    pub fn groups_mut(&mut self) -> Vec<QuantizeGroup<'_>> {
        // Ordinary group quantization writes independent planes. Reusing a
        // shared input through this API must restore the ordinary projection.
        self.shared_scale = false;
        let (block, k_type) = match self.encoding {
            Encoding::Q4K | Encoding::Q5K | Encoding::Q6K => (256, true),
            Encoding::TQ2_0 => (256, false),
            _ => (32, false),
        };
        let nb = self.cols / block;
        // int16 activation sums per (group, block): K types 32, others 4.
        let sums = if k_type { 32 } else { 4 };
        let groups = self.positions.div_ceil(4);
        // Each 4-element activation chunk is stored 4 times (C layout).
        let q_len = groups * 4 * self.cols * 4;
        let q_bytes = q_len.div_ceil(16) * 16;
        let base = self.storage.as_mut_ptr().cast::<u8>();
        // SAFETY: the C layout (power_llm_quantized_int8.h) places, inside
        // `len` bytes, q at 0 (groups * 16 * cols bytes), then 16-aligned da
        // (groups * nb * 4 floats), then s (groups * nb * `sums` int16).
        // The three regions are disjoint; storage is 16-byte aligned.
        let (q, da, s) = unsafe {
            (
                std::slice::from_raw_parts_mut(base, q_len),
                std::slice::from_raw_parts_mut(base.add(q_bytes).cast::<f32>(), groups * nb * 4),
                std::slice::from_raw_parts_mut(
                    base.add(q_bytes + groups * nb * 4 * 4).cast::<i16>(),
                    groups * nb * sums,
                ),
            )
        };
        let s_chunks: Vec<&mut [i16]> = s.chunks_mut(nb * sums).collect();
        q.chunks_mut(16 * self.cols)
            .zip(da.chunks_mut(nb * 4))
            .zip(s_chunks)
            .map(|((q, da), s)| QuantizeGroup {
                encoding: self.encoding,
                cols: self.cols,
                q,
                da,
                s,
            })
            .collect()
    }
}

/// One group of 4 positions of a [`QuantizedInput`].
pub struct QuantizeGroup<'a> {
    encoding: Encoding,
    cols: usize,
    q: &'a mut [u8],
    da: &'a mut [f32],
    s: &'a mut [i16],
}
impl<'a> QuantizeGroup<'a> {
    /// Quantize up to 4 finite rows of `cols` values (`None` = padding).
    pub fn quantize(&mut self, rows: [Option<&[f32]>; 4]) -> Result<(), Error> {
        self.quantize_with_residual(rows, [None, None, None, None])
    }

    /// A [`TwoPlaneWriter`] for [`Self::quantize_two_planes`] of `rows` with
    /// `residual`, to be filled block range by block range.
    fn two_plane_writer(
        self,
        rows: &'a [&'a [f32]],
        residual: &'a mut [f32],
        whole: *const QuantizedInput,
    ) -> Result<TwoPlaneWriter<'a>, Error> {
        let p = rows.len();
        if p == 0 || p > 2 || residual.len() != p * self.cols {
            return Err(Error::Shape("two-plane group rows"));
        }
        let mut pointers = [std::ptr::null::<f32>(); 4];
        let mut outputs = [std::ptr::null_mut::<f32>(); 4];
        for (l, row) in rows.iter().enumerate() {
            if row.len() != self.cols {
                return Err(Error::Shape("activation row length"));
            }
            if !crate::kernels::reference::all_finite(row) {
                return Err(Error::NonFiniteInput);
            }
            pointers[l] = row.as_ptr();
            // SAFETY: row l of the residual buffer (len = p * cols).
            let r = unsafe { residual.as_mut_ptr().add(l * self.cols) };
            outputs[l] = r;
            pointers[p + l] = r;
        }
        Ok(TwoPlaneWriter {
            encoding: self.encoding,
            cols: self.cols,
            pointers,
            outputs,
            q: self.q.as_mut_ptr(),
            da: self.da.as_mut_ptr(),
            s: if self.s.is_empty() {
                std::ptr::null_mut()
            } else {
                self.s.as_mut_ptr()
            },
            whole,
            _borrow: std::marker::PhantomData,
        })
    }
    /// Two activation planes of `rows` (at most 2) in this one group: rows
    /// 0..P are the inputs, rows P..2P their residuals `x - da * a`, written
    /// to `residual` (P rows of `cols`) and quantized in the same call.
    /// Identical to quantizing the rows with residuals and then
    /// [rows, residuals] again: both C quantizers finish row l before row
    /// l + 1, so each residual row is complete before it is read.
    pub fn quantize_two_planes(
        &mut self,
        rows: &[&[f32]],
        residual: &mut [f32],
    ) -> Result<(), Error> {
        let p = rows.len();
        if p == 0 || p > 2 || residual.len() != p * self.cols {
            return Err(Error::Shape("two-plane group rows"));
        }
        let mut pointers = [std::ptr::null::<f32>(); 4];
        let mut outputs = [std::ptr::null_mut::<f32>(); 4];
        for (l, row) in rows.iter().enumerate() {
            if row.len() != self.cols {
                return Err(Error::Shape("activation row length"));
            }
            if !crate::kernels::reference::all_finite(row) {
                return Err(Error::NonFiniteInput);
            }
            pointers[l] = row.as_ptr();
            // SAFETY: row l of the residual buffer (in bounds: len = p * cols).
            let r = unsafe { residual.as_mut_ptr().add(l * self.cols) };
            outputs[l] = r;
            pointers[p + l] = r;
        }
        // SAFETY: input rows are cols long; residual rows are written as the
        // outputs of rows 0..p before rows p..2p read them (row order above);
        // q/da/s are this group's exact regions.
        status(unsafe {
            quantize_group_fn()(
                kernel_kind(self.encoding),
                pointers.as_ptr(),
                self.cols,
                self.q.as_mut_ptr(),
                self.da.as_mut_ptr(),
                if self.s.is_empty() {
                    std::ptr::null_mut()
                } else {
                    self.s.as_mut_ptr()
                },
                outputs.as_ptr(),
            )
        })
    }

    /// As [`Self::quantize`], also writing each row's residual
    /// `x - da * a` (the input of a second activation plane).
    pub fn quantize_with_residual(
        &mut self,
        rows: [Option<&[f32]>; 4],
        residual: [Option<&mut [f32]>; 4],
    ) -> Result<(), Error> {
        let mut outputs = [std::ptr::null_mut::<f32>(); 4];
        for (output, r) in outputs.iter_mut().zip(residual) {
            if let Some(r) = r {
                if r.len() != self.cols {
                    return Err(Error::Shape("residual row length"));
                }
                *output = r.as_mut_ptr();
            }
        }
        let mut pointers = [std::ptr::null::<f32>(); 4];
        for (pointer, row) in pointers.iter_mut().zip(rows) {
            if let Some(row) = row {
                if row.len() != self.cols {
                    return Err(Error::Shape("activation row length"));
                }
                if !crate::kernels::reference::all_finite(row) {
                    return Err(Error::NonFiniteInput);
                }
                *pointer = row.as_ptr();
            }
        }
        // SAFETY: rows are cols long or NULL; q/da/s are this group's exact
        // regions (s empty and unused for 32-element encodings).
        status(unsafe {
            quantize_group_fn()(
                kernel_kind(self.encoding),
                pointers.as_ptr(),
                self.cols,
                self.q.as_mut_ptr(),
                self.da.as_mut_ptr(),
                if self.s.is_empty() {
                    std::ptr::null_mut()
                } else {
                    self.s.as_mut_ptr()
                },
                outputs.as_ptr(),
            )
        })
    }
}

/// `out[p, r]` for every row of `weights` (one call; no scratch needed).
pub(super) fn project_with(
    kernel: Int8Rows,
    weights: &EncodedRows<'_>,
    input: &QuantizedInput,
    out: &mut [f32],
) -> Result<(), Error> {
    if weights.encoding != input.encoding || weights.cols != input.cols {
        return Err(Error::Shape("quantized input does not match the weights"));
    }
    if input.positions.checked_mul(weights.rows) != Some(out.len()) {
        return Err(Error::Shape("output must be [positions, rows]"));
    }
    let kernel = if input.shared_scale {
        #[cfg(all(feature = "power9", target_arch = "powerpc64"))]
        if std::ptr::fn_addr_eq(kernel, pq8_project_rows_power9 as Int8Rows) {
            pq8_shared_project_rows_power9 as Int8Rows
        } else {
            pq8_shared_project_rows as Int8Rows
        }
        #[cfg(not(all(feature = "power9", target_arch = "powerpc64")))]
        {
            pq8_shared_project_rows as Int8Rows
        }
    } else {
        kernel
    };
    let split = split_offsets(weights.encoding, weights.bytes, weights.cols);
    let bytes = split.as_ref().map_or(weights.bytes, |(q, _)| q.as_slice());
    // SAFETY: weights are exactly rows * row_bytes (checked at construction;
    // a Q5_1 split keeps the rows as Q5_0), input holds `len` bytes for
    // (encoding, cols, positions), out is [positions, rows] with stride rows;
    // C retains nothing.
    status(unsafe {
        kernel(
            kernel_kind(weights.encoding),
            bytes.as_ptr(),
            bytes.len(),
            weights.rows,
            weights.cols,
            input.storage.as_ptr().cast(),
            input.len,
            input.positions,
            out.as_mut_ptr(),
            weights.rows,
            0,
        )
    })?;
    if let Some((_, offsets)) = &split {
        add_offsets(
            offsets,
            weights.rows,
            0,
            weights.rows,
            input,
            out,
            weights.rows,
            0,
        )?;
    }
    Ok(())
}

#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
unsafe extern "C" {
    fn pq8_packed_bytes_power9(kind: c_int, rows: usize, cols: usize) -> usize;
    fn pq8_pack_power9(
        kind: c_int,
        weights: *const u8,
        weight_bytes: usize,
        rows: usize,
        cols: usize,
        g0: usize,
        g1: usize,
        packed: *mut u8,
        packed_len: usize,
    ) -> c_int;
    fn pq8_project_packed_power9(
        kind: c_int,
        packed: *const u8,
        packed_len: usize,
        rows: usize,
        cols: usize,
        row0: usize,
        count: usize,
        input: *const u8,
        input_len: usize,
        positions: usize,
        out: *mut f32,
        out_stride: usize,
        out_offset: usize,
    ) -> c_int;
    fn pq8_shared_project_packed_power9(
        kind: c_int,
        packed: *const u8,
        packed_len: usize,
        rows: usize,
        cols: usize,
        row0: usize,
        count: usize,
        input: *const u8,
        input_len: usize,
        positions: usize,
        out: *mut f32,
        out_stride: usize,
        out_offset: usize,
    ) -> c_int;
}

/// Weights `[rows, cols]` unpacked once into the native integer-dot layout
/// (`pq8_pack_power9`, about 1.25 bytes per weight), so projections skip the
/// per-call unpack. Results are bit-identical to the unpacked path.
#[derive(Debug)]
pub struct PackedRows {
    // Read only by the POWER9 kernels.
    #[cfg_attr(
        not(all(feature = "power9", target_arch = "powerpc64")),
        allow(dead_code)
    )]
    encoding: Encoding,
    rows: usize,
    cols: usize,
    len: usize,
    /// u128 storage: 16-byte aligned, as the native kernels require.
    #[cfg_attr(
        not(all(feature = "power9", target_arch = "powerpc64")),
        allow(dead_code)
    )]
    storage: Vec<u128>,
    /// Q5_1 only (packed as Q5_0): the block offsets of [`split_q5_1`].
    #[cfg_attr(
        not(all(feature = "power9", target_arch = "powerpc64")),
        allow(dead_code)
    )]
    offsets: Vec<f32>,
}

impl PackedRows {
    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn cols(&self) -> usize {
        self.cols
    }
    /// Packed size in bytes.
    pub fn bytes(&self) -> usize {
        self.len
    }
}

#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
pub(super) fn pack_power9(weights: &EncodedRows<'_>) -> Result<PackedRows, Error> {
    if !supports(weights.encoding) {
        return Err(Error::Shape("integer-dot packing of F32 weights"));
    }
    let kind = kernel_kind(weights.encoding);
    let (bytes, offsets) =
        split_offsets(weights.encoding, weights.bytes, weights.cols).unwrap_or_default();
    let bytes = if offsets.is_empty() {
        weights.bytes
    } else {
        &bytes
    };
    // SAFETY: pure size computation.
    let len = unsafe { pq8_packed_bytes_power9(kind, weights.rows, weights.cols) };
    if len == 0 {
        return Err(Error::Shape("columns are not a block multiple"));
    }
    let mut storage = Vec::new();
    storage
        .try_reserve_exact(len.div_ceil(16))
        .map_err(|_| Error::Overflow)?;
    storage.resize(len.div_ceil(16), 0);
    // SAFETY: weights are exactly rows * row_bytes (checked at construction);
    // storage holds len bytes, 16-byte aligned; C retains nothing.
    status(unsafe {
        pq8_pack_power9(
            kind,
            bytes.as_ptr(),
            bytes.len(),
            weights.rows,
            weights.cols,
            0,
            weights.rows.div_ceil(4),
            storage.as_mut_ptr().cast(),
            len,
        )
    })?;
    Ok(PackedRows {
        encoding: weights.encoding,
        rows: weights.rows,
        cols: weights.cols,
        len,
        storage,
        offsets,
    })
}

/// Rows `[row0, row0 + count)` of `packed`:
/// `out[p * out_stride + out_offset + (r - row0)]`.
#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
pub(super) fn project_packed_power9(
    packed: &PackedRows,
    row0: usize,
    count: usize,
    input: &QuantizedInput,
    out: &mut [f32],
    out_stride: usize,
    out_offset: usize,
) -> Result<(), Error> {
    if packed.encoding != input.encoding || packed.cols != input.cols {
        return Err(Error::Shape("quantized input does not match the weights"));
    }
    if count == 0 || row0.checked_add(count).is_none_or(|end| end > packed.rows) {
        return Err(Error::Shape("packed row range"));
    }
    let needed = (input.positions - 1)
        .checked_mul(out_stride)
        .and_then(|n| n.checked_add(out_offset))
        .and_then(|n| n.checked_add(count));
    if out_offset + count > out_stride || needed.is_none_or(|n| n > out.len()) {
        return Err(Error::Shape("output stride/offset"));
    }
    if k_decode(packed, input) {
        project_k_decode(packed, row0, count, input, out, out_stride, out_offset)?;
    } else {
        project_packed_native(packed, row0, count, input, out, out_stride, out_offset)?;
    }
    if !packed.offsets.is_empty() {
        add_offsets(
            &packed.offsets,
            packed.rows,
            row0,
            count,
            input,
            out,
            out_stride,
            out_offset,
        )?;
    }
    Ok(())
}

/// The frozen C kernel (every encoding and batch).
#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
fn project_packed_native(
    packed: &PackedRows,
    row0: usize,
    count: usize,
    input: &QuantizedInput,
    out: &mut [f32],
    out_stride: usize,
    out_offset: usize,
) -> Result<(), Error> {
    // SAFETY: packed holds len bytes from pq8_pack_power9 for these
    // (encoding, rows, cols); input holds `len` bytes for (encoding, cols,
    // positions); every written index is inside out (checked by the caller).
    status(unsafe {
        (if input.shared_scale {
            pq8_shared_project_packed_power9
        } else {
            pq8_project_packed_power9
        })(
            kernel_kind(packed.encoding),
            packed.storage.as_ptr().cast(),
            packed.len,
            packed.rows,
            packed.cols,
            row0,
            count,
            input.storage.as_ptr().cast(),
            input.len,
            input.positions,
            out.as_mut_ptr(),
            out_stride,
            out_offset,
        )
    })
}

/// Whether one-token K-quant rows take the hand-written decode tiles
/// (`POWER_LLM_K_TILE=0` keeps the frozen C kernel, for comparison).
#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
fn k_decode(packed: &PackedRows, input: &QuantizedInput) -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    let kind = kernel_kind(packed.encoding);
    input.positions == 2
        && !input.shared_scale
        && (kind == 12 || kind == 14)
        && packed.rows >= 16
        && *ON.get_or_init(|| std::env::var("POWER_LLM_K_TILE").map_or(true, |v| v != "0"))
}

/// Rows `[row0, row0 + count)` of one-token K-quant `packed` through the
/// decode tiles, one packed super-group (16 rows) per tile; rows outside the
/// range are computed and dropped. Same values and nonfinite-output status
/// as the frozen kernel.
#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
fn project_k_decode(
    packed: &PackedRows,
    row0: usize,
    count: usize,
    input: &QuantizedInput,
    out: &mut [f32],
    out_stride: usize,
    out_offset: usize,
) -> Result<(), Error> {
    use super::tile_k_decode::{KTileArgs, pq8_q4_k_tile4x2_power9, pq8_q6_k_tile4x2_power9};
    const CHUNK: usize = 8;
    let q4k = kernel_kind(packed.encoding) == 12;
    let cols = packed.cols;
    let nb = cols / 256;
    // Tile-major packing: per super-group of 4 row groups, nb block records
    // of 4 groups' block weights then their 4 metadata entries.
    let record = 4 * if q4k { 512 + 224 } else { 1024 + 288 };
    let super_bytes = nb * record;
    // Input layout (positions padded to 4): q [4][cols / 4][16], da [nb][4],
    // then the int16 sub-block sums [nb][4][4][2].
    let qbytes = 16 * cols;
    let supers = packed.rows.div_ceil(16);
    if super_bytes * supers != packed.len || input.len < qbytes + 80 * nb {
        return Err(Error::Shape("K-quant decode tile layout"));
    }
    let base: *const u8 = packed.storage.as_ptr().cast();
    let q: *const u8 = input.storage.as_ptr().cast();
    let tile = if q4k {
        pq8_q4_k_tile4x2_power9
    } else {
        pq8_q6_k_tile4x2_power9
    };
    let end = row0 + count;
    let mut finite = true;
    let mut buffer = [0.0f32; 32 * CHUNK];
    let mut first = row0 / 16;
    while first * 16 < end {
        let tiles = (end.div_ceil(16) - first).min(CHUNK);
        let args = KTileArgs {
            // SAFETY (all pointer offsets): first + tiles <= supers, and
            // the input offsets are inside `input.len` (checked above).
            records: unsafe { base.add(first * super_bytes) },
            tiles,
            act0: q,
            act1: unsafe { q.add(4 * cols) },
            da: unsafe { q.add(qbytes) },
            sums: unsafe { q.add(qbytes + 16 * nb) },
            nb,
            out: buffer.as_mut_ptr(),
        };
        // SAFETY: args describe `tiles` complete super-groups of this packed
        // K-quant matrix and its 2-position input; buffer holds 32 floats per
        // tile.
        unsafe { tile(&args) };
        for (t, values) in buffer.as_chunks::<32>().0.iter().take(tiles).enumerate() {
            for r in 0..4 {
                for lane in 0..4 {
                    let row = 16 * (first + t) + 4 * r + lane;
                    if row < row0 || row >= end {
                        continue;
                    }
                    for p in 0..2 {
                        let y = values[8 * r + 4 * p + lane];
                        finite &= y.is_finite();
                        out[p * out_stride + out_offset + (row - row0)] = y;
                    }
                }
            }
        }
        first += tiles;
    }
    if finite {
        Ok(())
    } else {
        Err(Error::NonFiniteOutput)
    }
}

#[cfg(test)]
mod tests;
