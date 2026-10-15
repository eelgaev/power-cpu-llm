use std::ffi::{c_char, c_int, c_void};

pub enum Context {}

#[repr(C)]
pub struct InitParams {
    pub no_alloc: bool,
    pub ctx: *mut *mut c_void,
}

unsafe extern "C" {
    pub fn gguf_init_from_file(path: *const c_char, params: InitParams) -> *mut Context;
    pub fn gguf_free(ctx: *mut Context);
    pub fn gguf_get_version(ctx: *const Context) -> u32;
    pub fn gguf_get_alignment(ctx: *const Context) -> usize;
    pub fn gguf_get_data_offset(ctx: *const Context) -> usize;
    pub fn gguf_get_n_kv(ctx: *const Context) -> i64;
    pub fn gguf_get_key(ctx: *const Context, id: i64) -> *const c_char;
    pub fn gguf_get_kv_type(ctx: *const Context, id: i64) -> c_int;
    pub fn gguf_get_arr_type(ctx: *const Context, id: i64) -> c_int;
    pub fn gguf_get_arr_n(ctx: *const Context, id: i64) -> usize;
    pub fn gguf_get_arr_data(ctx: *const Context, id: i64) -> *const c_void;
    pub fn gguf_get_arr_str(ctx: *const Context, id: i64, index: usize) -> *const c_char;
    pub fn gguf_type_name(kind: c_int) -> *const c_char;
    pub fn gguf_get_val_u8(ctx: *const Context, id: i64) -> u8;
    pub fn gguf_get_val_i8(ctx: *const Context, id: i64) -> i8;
    pub fn gguf_get_val_u16(ctx: *const Context, id: i64) -> u16;
    pub fn gguf_get_val_i16(ctx: *const Context, id: i64) -> i16;
    pub fn gguf_get_val_u32(ctx: *const Context, id: i64) -> u32;
    pub fn gguf_get_val_i32(ctx: *const Context, id: i64) -> i32;
    pub fn gguf_get_val_f32(ctx: *const Context, id: i64) -> f32;
    pub fn gguf_get_val_bool(ctx: *const Context, id: i64) -> bool;
    pub fn gguf_get_val_str(ctx: *const Context, id: i64) -> *const c_char;
    pub fn gguf_get_val_u64(ctx: *const Context, id: i64) -> u64;
    pub fn gguf_get_val_i64(ctx: *const Context, id: i64) -> i64;
    pub fn gguf_get_val_f64(ctx: *const Context, id: i64) -> f64;
    pub fn gguf_get_n_tensors(ctx: *const Context) -> i64;
    pub fn gguf_get_tensor_name(ctx: *const Context, id: i64) -> *const c_char;
    pub fn gguf_get_tensor_ne(ctx: *const Context, id: i64) -> *const i64;
    pub fn gguf_get_tensor_type(ctx: *const Context, id: i64) -> c_int;
    pub fn gguf_get_tensor_offset(ctx: *const Context, id: i64) -> usize;
    pub fn gguf_get_tensor_size(ctx: *const Context, id: i64) -> usize;
    pub fn ggml_type_name(kind: c_int) -> *const c_char;
}
