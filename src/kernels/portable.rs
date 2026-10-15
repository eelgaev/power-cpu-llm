//! Portable reference kernels in Rust, ported from (and verified bit for bit
//! against) the former baseline C kernels. They keep the C ABI names: the
//! existing `extern "C"` declarations and the frozen POWER9 assembly
//! (`p9_decode_row`, `pq_row_bytes`, `pq8_input_bytes`) call them.

#![allow(
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::needless_range_loop
)]

pub(crate) mod attention;
pub(crate) mod dequant;
pub(crate) mod int8;
pub(crate) mod matmul;
pub(crate) mod quant;
pub(crate) mod reference;
