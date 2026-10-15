//! power-cpu-llm: a Rust inference engine for Gemma 4 and DiffusionGemma GGUF
//! models on POWER9, with an OpenAI-compatible HTTP server.
//!
//! - [`libm`]: bit-exact ports of the glibc 2.28 libm functions the numerics
//!   use (LGPL-2.1-or-later, see that module);
//! - [`kernels`]: quantized matrix, attention and dequantization kernels
//!   (portable Rust and POWER9 assembly);
//! - [`gguf`]: the GGUF reader;
//! - [`engine`]: tokenizer, chat template, model execution and generation;
//! - [`server`]: the OpenAI-compatible HTTP server.

pub mod engine;
pub mod gguf;
pub mod kernels;
pub mod libm;
pub mod server;
