//! Exact Q4_0 integer-dot decode tile (8 groups x 2 positions).
/// # Safety
/// `args` must point to the validated decode tile arguments.
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pq8_q4_0_tile8x2_power9(_args: *const u8) {
    core::arch::naked_asm!(
        ".machine push\n.machine power9",
        include_str!("../asm/q4_0/pq8_q4_0_tile8x2_power9.s"),
        ".machine pop",
    )
}
