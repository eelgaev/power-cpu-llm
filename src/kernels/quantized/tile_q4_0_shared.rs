//! Experimental Q4_0 shared-scale tile, bit-identical to its portable twin.

/// Four row groups x two logical positions; see
/// `asm/q4_0/pq8_q4_0_shared_tile4x2_power9.s` for its ABI.
///
/// # Safety
/// `args` must describe valid packed weights, paired activation planes,
/// sums/scales covering at least one block, and eight output vectors.
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pq8_q4_0_shared_tile4x2_power9(_args: *const u8) {
    core::arch::naked_asm!(
        ".machine push\n.machine power9",
        include_str!("../asm/q4_0/pq8_q4_0_shared_tile4x2_power9.s"),
        ".machine pop",
    )
}
