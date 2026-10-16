//! Hand-written K-quant one-token decode tiles (see
//! `asm/k_quant/pq8_q4_k_tile4x2_power9.s` for the schedule and arguments).

/// Arguments of the K-quant decode tiles (field offsets are the assembly ABI).
#[repr(C)]
pub(super) struct KTileArgs {
    /// First block record of the first tile's super-group.
    pub records: *const u8,
    pub tiles: usize,
    pub act0: *const u8,
    pub act1: *const u8,
    pub da: *const u8,
    pub sums: *const u8,
    pub nb: usize,
    pub out: *mut f32,
}

/// Q4_K: `tiles` tiles of 4 packed row groups x 2 activation planes.
///
/// # Safety
/// `args` must describe packed Q4_K rows and a 2-position quantized input
/// covering `nb` blocks for every tile, and `out` must hold 32 floats per tile.
#[unsafe(naked)]
pub(super) unsafe extern "C" fn pq8_q4_k_tile4x2_power9(_args: *const KTileArgs) {
    core::arch::naked_asm!(
        ".machine push\n.machine power9",
        include_str!("../asm/k_quant/pq8_q4_k_tile4x2_power9.s"),
        ".machine pop",
    )
}

/// Q6_K kind (Q6_K, and Q5_K repacked as Q6_K): as [`pq8_q4_k_tile4x2_power9`].
///
/// # Safety
/// As [`pq8_q4_k_tile4x2_power9`], for packed Q6_K-kind rows.
#[unsafe(naked)]
pub(super) unsafe extern "C" fn pq8_q6_k_tile4x2_power9(_args: *const KTileArgs) {
    core::arch::naked_asm!(
        ".machine push\n.machine power9",
        include_str!("../asm/k_quant/pq8_q6_k_tile4x2_power9.s"),
        ".machine pop",
    )
}
