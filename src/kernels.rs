//! Safe block-quantized kernel boundary: portable Rust kernels and POWER9 assembly.
//! These internal formats are independent of any model file format.

use std::ffi::{CStr, c_char, c_int};
use std::fmt;

pub mod deltanet;
pub mod encoded;
pub mod quantized;
pub mod reference;

mod portable;

#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
mod power9_asm;

#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
mod dequant_asm;

#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
mod fma_asm;

#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
mod int8_asm;

#[cfg(all(feature = "power9", target_arch = "powerpc64"))]
mod attention_asm;

pub const BLOCK_SIZE: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    NonFiniteScale,
    Q4OutOfRange,
    BlockCountMismatch { weights: usize, activations: usize },
    BackendUnavailable,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

#[derive(Debug, Clone, Copy)]
pub enum Backend {
    Auto,
    Scalar,
    Power9,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct RawActivation {
    q: [i8; BLOCK_SIZE],
    sum: i32,
    d: f32,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct RawQ8 {
    q: [u8; BLOCK_SIZE],
    d: f32,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct RawQ4 {
    q: [u8; BLOCK_SIZE / 2],
    d: f32,
}

type BlockDot = unsafe extern "C" fn(*const u8, *const RawActivation) -> i32;
#[repr(C)]
struct RawKernels {
    name: *const c_char,
    q8: BlockDot,
    q4: BlockDot,
}

unsafe extern "C" {
    fn p9_prepare_activation(out: *mut RawActivation, q: *const i8, d: f32);
    fn p9_pack_q8(out: *mut RawQ8, q: *const i8, d: f32);
    fn p9_pack_q4(out: *mut RawQ4, q: *const i8, d: f32) -> bool;
    fn p9_get_kernels(backend: c_int) -> *const RawKernels;
    fn p9_dot_q8(w: *const RawQ8, a: *const RawActivation, n: usize, k: *const RawKernels) -> f32;
    fn p9_dot_q4(w: *const RawQ4, a: *const RawActivation, n: usize, k: *const RawKernels) -> f32;
}

/// Runs `rounds` x 12 independent POWER9 vector integer multiply-sums on
/// the calling thread: a fixed amount of work for one execution slice, with
/// no memory traffic. Returns `false` (doing nothing) without POWER9 kernels.
pub fn slice_probe(rounds: u64) -> bool {
    #[cfg(all(feature = "power9", target_arch = "powerpc64"))]
    {
        unsafe extern "C" {
            fn p9_slice_probe(rounds: u64) -> u32;
        }
        // SAFETY: register arithmetic only.
        std::hint::black_box(unsafe { p9_slice_probe(rounds) });
        true
    }
    #[cfg(not(all(feature = "power9", target_arch = "powerpc64")))]
    {
        let _ = rounds;
        false
    }
}

fn check_scale(scale: f32) -> Result<(), Error> {
    if scale.is_finite() {
        Ok(())
    } else {
        Err(Error::NonFiniteScale)
    }
}

/// Immutable so that the activation-sum correction cannot become stale.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ActivationBlock(RawActivation);

impl ActivationBlock {
    pub fn from_quantized(q: &[i8; BLOCK_SIZE], scale: f32) -> Result<Self, Error> {
        check_scale(scale)?;
        let mut raw = RawActivation {
            q: [0; BLOCK_SIZE],
            sum: 0,
            d: 0.0,
        };
        // SAFETY: input and output have the C layouts and contain 32 elements.
        unsafe { p9_prepare_activation(&mut raw, q.as_ptr(), scale) };
        Ok(Self(raw))
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Q8WeightBlock(RawQ8);

impl Q8WeightBlock {
    pub fn from_quantized(q: &[i8; BLOCK_SIZE], scale: f32) -> Result<Self, Error> {
        check_scale(scale)?;
        let mut raw = RawQ8 {
            q: [0; BLOCK_SIZE],
            d: 0.0,
        };
        // SAFETY: initialized output and fixed-size input have the C layouts.
        unsafe { p9_pack_q8(&mut raw, q.as_ptr(), scale) };
        Ok(Self(raw))
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Q4WeightBlock(RawQ4);

impl Q4WeightBlock {
    pub fn from_quantized(q: &[i8; BLOCK_SIZE], scale: f32) -> Result<Self, Error> {
        check_scale(scale)?;
        let mut raw = RawQ4 {
            q: [0; BLOCK_SIZE / 2],
            d: 0.0,
        };
        // SAFETY: initialized output and fixed-size input have the C layouts.
        if !unsafe { p9_pack_q4(&mut raw, q.as_ptr(), scale) } {
            return Err(Error::Q4OutOfRange);
        }
        Ok(Self(raw))
    }
}

/// Copyable function pointers selected once, not dispatched per block.
/// No allocations, global mutable state, or raw pointers are retained.
#[derive(Clone, Copy)]
pub struct Kernels {
    name: &'static str,
    q8: BlockDot,
    q4: BlockDot,
}

impl Kernels {
    pub fn new(backend: Backend) -> Result<Self, Error> {
        let code = match backend {
            Backend::Auto => 0,
            Backend::Scalar => 1,
            Backend::Power9 => 2,
        };
        // SAFETY: valid backend code; C returns NULL or a static immutable table.
        let raw = unsafe { p9_get_kernels(code).as_ref() }.ok_or(Error::BackendUnavailable)?;
        // SAFETY: the table's name is a static NUL-terminated string owned by C.
        let name = match unsafe { CStr::from_ptr(raw.name) }.to_bytes() {
            b"power9" => "power9",
            b"scalar" => "scalar",
            _ => return Err(Error::BackendUnavailable),
        };
        Ok(Self {
            name,
            q8: raw.q8,
            q4: raw.q4,
        })
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn q8_block_dot(&self, w: &Q8WeightBlock, a: &ActivationBlock) -> i32 {
        // SAFETY: immutable valid blocks, with a cached activation sum.
        unsafe { (self.q8)(w.0.q.as_ptr(), &a.0) }
    }

    pub fn q4_block_dot(&self, w: &Q4WeightBlock, a: &ActivationBlock) -> i32 {
        // SAFETY: immutable valid blocks, with a cached activation sum.
        unsafe { (self.q4)(w.0.q.as_ptr(), &a.0) }
    }

    fn raw_table(&self) -> RawKernels {
        RawKernels {
            name: c"rust-dispatch".as_ptr(),
            q8: self.q8,
            q4: self.q4,
        }
    }

    /// One FFI call per row, rather than one per quantization block.
    pub fn q8_dot(&self, w: &[Q8WeightBlock], a: &[ActivationBlock]) -> Result<f32, Error> {
        check_lengths(w.len(), a.len())?;
        // SAFETY: equal-length slices and repr(transparent) wrappers around
        // repr(C) blocks. C reads n entries and does not retain any pointers.
        Ok(unsafe {
            p9_dot_q8(
                w.as_ptr().cast(),
                a.as_ptr().cast(),
                w.len(),
                &self.raw_table(),
            )
        })
    }

    pub fn q4_dot(&self, w: &[Q4WeightBlock], a: &[ActivationBlock]) -> Result<f32, Error> {
        check_lengths(w.len(), a.len())?;
        // SAFETY: same invariants as q8_dot; empty slices are never dereferenced.
        Ok(unsafe {
            p9_dot_q4(
                w.as_ptr().cast(),
                a.as_ptr().cast(),
                w.len(),
                &self.raw_table(),
            )
        })
    }
}

fn check_lengths(weights: usize, activations: usize) -> Result<(), Error> {
    if weights == activations {
        Ok(())
    } else {
        Err(Error::BlockCountMismatch {
            weights,
            activations,
        })
    }
}

#[cfg(test)]
mod tests;
