//! The POWER9 `fma` kernels: the canonical assembly in `asm/fma/`, included
//! as is. The include order is the code layout; keep it when adding a file.
core::arch::global_asm!(
    ".machine push\n.machine power9\n\t.abiversion 2",
    include_str!("asm/fma/pq_project_rows_power9_fma.s"),
    include_str!("asm/fma/pq_accumulate_transposed_rows_power9_fma.s"),
    ".text\n.machine pop",
);
