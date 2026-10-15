//! The Q4_0 int8 prompt tile in assembly (see
//! `asm/q4_0/pq8_q4_0_tile4x4_power9.s` for the schedule and argument layout).

/// `pq8_q4_0_tile4x4_power9`, called by `pq8_project_packed_power9`: 4 row
/// groups x 4 positions of packed Q4_0 weights, bit-identical to the portable
/// projection.
///
/// # Safety
/// `args` must point to a valid `pq8_q4tile_args` whose pointers cover `nb`
/// blocks (assembly caller only).
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pq8_q4_0_tile4x4_power9(_args: *const u8) {
    core::arch::naked_asm!(
        ".machine push\n.machine power9",
        include_str!("../asm/q4_0/pq8_q4_0_tile4x4_power9.s"),
        ".machine pop",
    )
}
