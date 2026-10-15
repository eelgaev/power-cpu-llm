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
            Encoding::Q4K | Encoding::Q6K | Encoding::TQ2_0 => 256,
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
                self.encoding as c_int,
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
            Encoding::Q4K | Encoding::Q6K | Encoding::TQ2_0 => 256,
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
                    self.encoding as c_int,
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

/// Whether `encoding` has an integer-dot kernel (all quantized encodings;
/// F32 rows always use the F64 kernels).
pub fn supports(encoding: Encoding) -> bool {
    !matches!(encoding, Encoding::F32 | Encoding::F16)
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
        let len = unsafe { pq8_input_bytes(encoding as c_int, cols, positions) };
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
                encoding as c_int,
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
        let len = unsafe { pq8_input_bytes(encoding as c_int, cols, positions) };
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
        let zero = if self.encoding == Encoding::Q6K {
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
            Encoding::Q4K | Encoding::Q6K => (256, true),
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
            Encoding::Q4K | Encoding::Q6K => (256, true),
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
                self.encoding as c_int,
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
                self.encoding as c_int,
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
    // SAFETY: weights are exactly rows * row_bytes (checked at construction),
    // input holds `len` bytes for (encoding, cols, positions), out is
    // [positions, rows] with stride rows; C retains nothing.
    status(unsafe {
        kernel(
            weights.encoding as c_int,
            weights.bytes.as_ptr(),
            weights.bytes.len(),
            weights.rows,
            weights.cols,
            input.storage.as_ptr().cast(),
            input.len,
            input.positions,
            out.as_mut_ptr(),
            weights.rows,
            0,
        )
    })
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
    // SAFETY: pure size computation.
    let len =
        unsafe { pq8_packed_bytes_power9(weights.encoding as c_int, weights.rows, weights.cols) };
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
            weights.encoding as c_int,
            weights.bytes.as_ptr(),
            weights.bytes.len(),
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
    // SAFETY: packed holds len bytes from pq8_pack_power9 for these
    // (encoding, rows, cols); input holds `len` bytes for (encoding, cols,
    // positions); every written index is inside out (checked above).
    status(unsafe {
        (if input.shared_scale {
            pq8_shared_project_packed_power9
        } else {
            pq8_project_packed_power9
        })(
            packed.encoding as c_int,
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

#[cfg(test)]
mod tests;
