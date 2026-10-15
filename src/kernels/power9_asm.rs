//! POWER9 block dots and the exact router, preserving the frozen C schedules.
core::arch::global_asm!(
    ".pushsection .text.p9_slice_probe,\"ax\",@progbits\n.p2align 4\n.popsection"
);
core::arch::global_asm!(
    ".pushsection .text.p9_q8_power9,\"ax\",@progbits\n.p2align 4\n.popsection"
);
core::arch::global_asm!(
    ".pushsection .text.p9_q4_power9,\"ax\",@progbits\n.p2align 4\n.popsection"
);
core::arch::global_asm!(
    ".pushsection .text.p9_ref_matvec_multi_power9,\"ax\",@progbits\n.p2align 4\n.popsection"
);

/// # Safety
/// C ABI buffers must satisfy the original native kernel contract.
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.p9_q8_power9")]
pub unsafe extern "C" fn p9_q8_power9(_weights: *const u8, _activation: *const u8) -> i32 {
    core::arch::naked_asm!(
        ".machine push\n.machine power9",
        include_str!("asm/reference/p9_q8_power9.s"),
        ".machine pop",
    )
}

/// # Safety
/// C ABI buffers must satisfy the original native kernel contract.
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.p9_q4_power9")]
pub unsafe extern "C" fn p9_q4_power9(_weights: *const u8, _activation: *const u8) -> i32 {
    core::arch::naked_asm!(
        ".machine push\n.machine power9",
        include_str!("asm/reference/p9_q4_power9.s"),
        ".machine pop",
    )
}

/// # Safety
/// C ABI buffers must satisfy the original native kernel contract.
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.p9_ref_matvec_multi_power9")]
pub unsafe extern "C" fn p9_ref_matvec_multi_power9(
    _matrix: *const f32,
    _blocked: *const f32,
    _x: *const f32,
    _inputs: usize,
    _rows: usize,
    _cols: usize,
    _out: *mut f32,
    _scratch: *mut f64,
) {
    core::arch::naked_asm!(
        ".machine push\n.machine power9",
        include_str!("asm/reference/p9_ref_matvec_multi_power9.s"),
        ".machine pop",
    )
}

/// # Safety
/// C ABI buffers must satisfy the original native kernel contract.
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.p9_slice_probe")]
pub unsafe extern "C" fn p9_slice_probe(_rounds: u64) -> u32 {
    core::arch::naked_asm!(
        ".machine push\n.machine power9",
        include_str!("asm/reference/p9_slice_probe.s"),
        ".machine pop",
    )
}

#[cfg(test)]
mod tests;
