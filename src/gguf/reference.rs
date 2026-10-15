//! Test-only oracle from the pinned ggml dequantizers. Not an execution backend.
//! Compiled only with `reference-oracle`; the production runtime does not call it.

use std::ffi::{c_char, c_int, c_void};

type ToFloat = unsafe extern "C" fn(*const c_void, *mut f32, i64);
type FromFloat = unsafe extern "C" fn(*const f32, *mut c_void, i64);

#[repr(C)]
struct TypeTraits {
    name: *const c_char,
    block_size: i64,
    block_size_interleave: i64,
    type_size: usize,
    is_quantized: bool,
    to_float: Option<ToFloat>,
    from_float: Option<FromFloat>,
}
unsafe extern "C" {
    fn ggml_get_type_traits(kind: c_int) -> *const TypeTraits;
}

/// Deliberately restricted to the five verified target formats. Checks buffer
/// sizes before invoking ggml, whose internal decoders otherwise assert.
pub fn decode(kind: i32, bytes: &[u8], count: usize) -> Result<Vec<f32>, &'static str> {
    let (elements, block_bytes) = match kind {
        0 => (1, 4),
        2 => (32, 18),
        6 => (32, 22),
        8 => (32, 34),
        12 => (256, 144),
        14 => (256, 210),
        _ => return Err("unsupported oracle type"),
    };
    let expected = (count / elements)
        .checked_mul(block_bytes)
        .ok_or("size overflow")?;
    if count % elements != 0
        || bytes.len() != expected
        || count > i64::MAX as usize
        || count > isize::MAX as usize / size_of::<f32>()
    {
        return Err("invalid oracle row size");
    }
    if kind == 0 {
        return Ok(bytes
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
            .collect());
    }
    if cfg!(target_endian = "big") {
        return Err("oracle requires little-endian native GGUF blocks");
    }
    // ggml casts to block structs: provide explicit 8-byte alignment rather
    // than relying on the allocator's incidental alignment of a Vec<u8>.
    let mut aligned = vec![0u64; bytes.len().div_ceil(8)];
    let mut out = vec![0.0; count];
    // SAFETY: supported type ID and checked exact block dimensions, count fits
    // i64, output has count initialized floats, and aligned storage is large
    // enough for the raw copy. TypeTraits matches the pinned C declaration.
    unsafe {
        std::ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            aligned.as_mut_ptr().cast::<u8>(),
            bytes.len(),
        );
        let traits = &*ggml_get_type_traits(kind);
        if traits.block_size != elements as i64 || traits.type_size != block_bytes {
            return Err("oracle ABI/layout mismatch");
        }
        let decode = traits.to_float.ok_or("oracle decoder missing")?;
        decode(aligned.as_ptr().cast(), out.as_mut_ptr(), count as i64);
    }
    Ok(out)
}
