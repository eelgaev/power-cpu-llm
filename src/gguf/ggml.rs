//! The pinned ggml GGUF reader (`vendor/ggml`), kept only as the test oracle
//! for the Rust parser (`reference-oracle` feature).

use crate::gguf::parse::Parsed;
use crate::gguf::{Error, Inventory, MetadataValue, TensorInfo, ffi, metadata};
use std::collections::BTreeMap;
use std::ffi::{CStr, CString, c_char};
use std::path::Path;
use std::ptr::NonNull;

struct Context(NonNull<ffi::Context>);
impl Drop for Context {
    fn drop(&mut self) {
        // SAFETY: sole ownership of a context returned by gguf_init_from_file.
        unsafe { ffi::gguf_free(self.0.as_ptr()) };
    }
}

// SAFETY requirement: ptr is a non-NULL NUL-terminated string from a live
// gguf_context, or a static type name from ggml. All strings are copied.
pub(crate) unsafe fn copy_string(ptr: *const c_char) -> String {
    unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned()
}

/// The pinned ggml reader's view of `path` (test oracle for `parse.rs`).
pub fn inspect(path: impl AsRef<Path>) -> Result<Inventory, Error> {
    let path = path.as_ref();
    let file_size = std::fs::metadata(path)?.len();
    let path = CString::new(path.as_os_str().as_encoded_bytes()).map_err(|_| Error::InvalidPath)?;
    // SAFETY: NUL-terminated path, ctx=NULL prevents tensor-context creation;
    // no_alloc=true requests metadata-only parsing.
    let raw = unsafe {
        ffi::gguf_init_from_file(
            path.as_ptr(),
            ffi::InitParams {
                no_alloc: true,
                ctx: std::ptr::null_mut(),
            },
        )
    };
    let context = Context(NonNull::new(raw).ok_or(Error::ParseFailed)?);
    let ctx = context.0.as_ptr();

    // SAFETY: ctx remains alive throughout; IDs are bounded by the reader's
    // counts. Every metadata getter is chosen using its actual enum type,
    // because gguf aborts when a getter is used with the wrong type.
    unsafe {
        let version = ffi::gguf_get_version(ctx);
        let alignment = ffi::gguf_get_alignment(ctx);
        let data_offset = ffi::gguf_get_data_offset(ctx) as u64;
        let mut metadata = BTreeMap::new();
        for id in 0..ffi::gguf_get_n_kv(ctx) {
            let key = copy_string(ffi::gguf_get_key(ctx, id));
            let value = match ffi::gguf_get_kv_type(ctx, id) {
                0 => MetadataValue::Unsigned(ffi::gguf_get_val_u8(ctx, id).into()),
                1 => MetadataValue::Signed(ffi::gguf_get_val_i8(ctx, id).into()),
                2 => MetadataValue::Unsigned(ffi::gguf_get_val_u16(ctx, id).into()),
                3 => MetadataValue::Signed(ffi::gguf_get_val_i16(ctx, id).into()),
                4 => MetadataValue::Unsigned(ffi::gguf_get_val_u32(ctx, id).into()),
                5 => MetadataValue::Signed(ffi::gguf_get_val_i32(ctx, id).into()),
                6 => MetadataValue::Float(ffi::gguf_get_val_f32(ctx, id).into()),
                7 => MetadataValue::Bool(ffi::gguf_get_val_bool(ctx, id)),
                8 => MetadataValue::String(copy_string(ffi::gguf_get_val_str(ctx, id))),
                9 => MetadataValue::Array {
                    element_type: copy_string(ffi::gguf_type_name(ffi::gguf_get_arr_type(ctx, id))),
                    len: ffi::gguf_get_arr_n(ctx, id),
                    values: metadata::copy_array(ctx, id)?,
                },
                10 => MetadataValue::Unsigned(ffi::gguf_get_val_u64(ctx, id)),
                11 => MetadataValue::Signed(ffi::gguf_get_val_i64(ctx, id)),
                12 => MetadataValue::Float(ffi::gguf_get_val_f64(ctx, id)),
                other => {
                    return Err(Error::InvalidLayout(format!(
                        "unknown metadata type {other}"
                    )));
                }
            };
            metadata.insert(key, value);
        }
        let mut tensors = Vec::new();
        for id in 0..ffi::gguf_get_n_tensors(ctx) {
            let name = copy_string(ffi::gguf_get_tensor_name(ctx, id));
            let type_id = ffi::gguf_get_tensor_type(ctx, id);
            let ne = std::slice::from_raw_parts(ffi::gguf_get_tensor_ne(ctx, id), 4);
            let shape = std::array::from_fn(|i| ne[i] as u64);
            let file_offset = ffi::gguf_get_tensor_offset(ctx, id) as u64;
            let size_bytes = ffi::gguf_get_tensor_size(ctx, id) as u64;
            tensors.push(TensorInfo {
                name,
                type_id,
                type_name: copy_string(ffi::ggml_type_name(type_id)),
                shape,
                file_offset,
                size_bytes,
            });
        }
        crate::gguf::validate(
            Parsed {
                version,
                alignment,
                data_offset,
                metadata,
                tensors,
            },
            file_size,
        )
    }
}
