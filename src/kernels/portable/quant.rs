//! Portable block dot products and kernel selection (formerly
//! `native/quant.c` and `native/dispatch.c`), the reference for the POWER9
//! block dots in `power9_asm.rs`.

use std::ffi::{c_char, c_int};

const BLOCK: usize = 32;

#[repr(C)]
pub struct Activation {
    q: [i8; BLOCK],
    sum: i32,
    d: f32,
}
#[repr(C)]
pub struct Q8 {
    q: [u8; BLOCK],
    d: f32,
}
#[repr(C)]
pub struct Q4 {
    q: [u8; BLOCK / 2],
    d: f32,
}

type BlockDot = unsafe extern "C" fn(*const u8, *const Activation) -> i32;

#[repr(C)]
pub struct KernelTable {
    name: *const c_char,
    q8: BlockDot,
    q4: BlockDot,
}
// SAFETY: immutable static tables of a string literal and function pointers.
unsafe impl Sync for KernelTable {}

/// # Safety
/// `out` is writable; `q` holds 32 values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_prepare_activation(out: *mut Activation, q: *const i8, d: f32) {
    // SAFETY: caller contract (q may alias out->q, as with memmove).
    let q = unsafe { std::ptr::read_unaligned(q.cast::<[i8; BLOCK]>()) };
    let sum = q.iter().map(|&v| i32::from(v)).sum();
    // SAFETY: caller contract.
    unsafe { out.write(Activation { q, sum, d }) };
}

/// # Safety
/// `out` is writable; `q` holds 32 values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_pack_q8(out: *mut Q8, q: *const i8, d: f32) {
    // SAFETY: caller contract.
    let q = unsafe { std::ptr::read_unaligned(q.cast::<[i8; BLOCK]>()) };
    let packed = Q8 {
        q: q.map(|v| (i32::from(v) + 128) as u8),
        d,
    };
    // SAFETY: caller contract.
    unsafe { out.write(packed) };
}

/// False (and `out` unchanged) for values outside [-8, 7].
///
/// # Safety
/// `out` is writable; `q` holds 32 values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_pack_q4(out: *mut Q4, q: *const i8, d: f32) -> bool {
    // SAFETY: caller contract.
    let q = unsafe { std::ptr::read_unaligned(q.cast::<[i8; BLOCK]>()) };
    if q.iter().any(|&v| !(-8..=7).contains(&v)) {
        return false;
    }
    let packed = Q4 {
        q: std::array::from_fn(|i| ((q[i] + 8) as u8) | (((q[i + 16] + 8) as u8) << 4)),
        d,
    };
    // SAFETY: caller contract.
    unsafe { out.write(packed) };
    true
}

/// # Safety
/// `w` holds 32 bytes; `a` is a valid activation.
pub unsafe extern "C" fn p9_q8_scalar(w: *const u8, a: *const Activation) -> i32 {
    // SAFETY: caller contract.
    let (w, a) = unsafe { (std::slice::from_raw_parts(w, BLOCK), &*a) };
    (0..BLOCK)
        .map(|i| i32::from(a.q[i]) * (i32::from(w[i]) - 128))
        .sum()
}

/// # Safety
/// `w` holds 16 bytes; `a` is a valid activation.
pub unsafe extern "C" fn p9_q4_scalar(w: *const u8, a: *const Activation) -> i32 {
    // SAFETY: caller contract.
    let (w, a) = unsafe { (std::slice::from_raw_parts(w, BLOCK / 2), &*a) };
    let mut dot = 0;
    for i in 0..BLOCK / 2 {
        dot += i32::from(a.q[i]) * (i32::from(w[i] & 15) - 8);
        dot += i32::from(a.q[i + 16]) * (i32::from(w[i] >> 4) - 8);
    }
    dot
}

static SCALAR: KernelTable = KernelTable {
    name: c"scalar".as_ptr(),
    q8: p9_q8_scalar,
    q4: p9_q4_scalar,
};

#[cfg(all(feature = "power9", target_arch = "powerpc64", target_os = "linux"))]
static POWER9: KernelTable = KernelTable {
    name: c"power9".as_ptr(),
    // SAFETY (signature): same C ABI, activation passed by pointer.
    q8: unsafe {
        std::mem::transmute::<unsafe extern "C" fn(*const u8, *const u8) -> i32, BlockDot>(
            crate::kernels::power9_asm::p9_q8_power9,
        )
    },
    q4: unsafe {
        std::mem::transmute::<unsafe extern "C" fn(*const u8, *const u8) -> i32, BlockDot>(
            crate::kernels::power9_asm::p9_q4_power9,
        )
    },
};

/// Runtime POWER9 (ISA 3.0) detection from the Linux auxiliary vector.
#[cfg(all(feature = "power9", target_arch = "powerpc64", target_os = "linux"))]
pub(crate) fn have_power9() -> bool {
    const AT_HWCAP2: std::ffi::c_ulong = 26;
    const PPC_FEATURE2_ARCH_3_00: std::ffi::c_ulong = 0x0080_0000;
    unsafe extern "C" {
        fn getauxval(kind: std::ffi::c_ulong) -> std::ffi::c_ulong;
    }
    // SAFETY: getauxval has no preconditions.
    unsafe { getauxval(AT_HWCAP2) & PPC_FEATURE2_ARCH_3_00 != 0 }
}

/// Backend 0 = auto, 1 = scalar, 2 = POWER9; null if unavailable.
#[unsafe(no_mangle)]
pub extern "C" fn p9_get_kernels(backend: c_int) -> *const KernelTable {
    if backend == 1 {
        return &SCALAR;
    }
    if backend != 0 && backend != 2 {
        return std::ptr::null();
    }
    #[cfg(all(feature = "power9", target_arch = "powerpc64", target_os = "linux"))]
    if have_power9() {
        return &POWER9;
    }
    if backend == 0 {
        &SCALAR
    } else {
        std::ptr::null()
    }
}

/// # Safety
/// `w`, `a` hold `nblocks` entries; `k` is a valid table.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_dot_q8(
    w: *const Q8,
    a: *const Activation,
    nblocks: usize,
    k: *const KernelTable,
) -> f32 {
    let mut result = 0.0f32;
    for b in 0..nblocks {
        // SAFETY: caller contract.
        let (w, a, k) = unsafe { (&*w.add(b), &*a.add(b), &*k) };
        // SAFETY: table entries take one 32-byte block and one activation.
        result += unsafe { (k.q8)(w.q.as_ptr(), a) } as f32 * (w.d * a.d);
    }
    result
}

/// # Safety
/// `w`, `a` hold `nblocks` entries; `k` is a valid table.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_dot_q4(
    w: *const Q4,
    a: *const Activation,
    nblocks: usize,
    k: *const KernelTable,
) -> f32 {
    let mut result = 0.0f32;
    for b in 0..nblocks {
        // SAFETY: caller contract.
        let (w, a, k) = unsafe { (&*w.add(b), &*a.add(b), &*k) };
        // SAFETY: table entries take one 16-byte block and one activation.
        result += unsafe { (k.q4)(w.q.as_ptr(), a) } as f32 * (w.d * a.d);
    }
    result
}
