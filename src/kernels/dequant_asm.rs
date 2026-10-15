//! The POWER9 `dequant` kernels: the canonical assembly in `asm/dequant/`, included
//! as is. The include order is the code layout; keep it when adding a file.
core::arch::global_asm!(
    ".machine push\n.machine power9\n\t.abiversion 2",
    include_str!("asm/dequant/p9_decode_row_vsx.s"),
    include_str!("asm/dequant/p9_all_finite_vsx.s"),
    ".text\n.machine pop",
);

#[cfg(test)]
mod tests;
