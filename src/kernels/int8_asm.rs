//! The POWER9 `int8` kernels: the canonical assembly in `asm/int8/`, included
//! as is. The include order is the code layout; keep it when adding a file.
core::arch::global_asm!(
    ".machine push\n.machine power9\n\t.abiversion 2",
    include_str!("asm/int8/pq8_packed_bytes_power9.s"),
    include_str!("asm/int8/record_bytes.s"),
    include_str!("asm/int8/pq8_pack_power9.s"),
    include_str!("asm/int8/group_bytes.s"),
    include_str!("asm/int8/pq8_project_packed_power9.s"),
    include_str!("asm/int8/i8c_input_view.s"),
    include_str!("asm/int8/pq8_project_rows_power9.s"),
    include_str!("asm/int8/i8c_check.s"),
    include_str!("asm/int8/pq_accumulate_transposed_rows_power9_f32.s"),
    include_str!("asm/int8/pq8_quantize_group_power9.s"),
    include_str!("asm/int8/pq8_quantize_group_range_power9.s"),
    include_str!("asm/int8/pq8_shared_quantize_power9.s"),
    include_str!("asm/int8/pq8_shared_project_packed_power9.s"),
    include_str!("asm/int8/pq8_shared_project_rows_power9.s"),
    ".text\n.machine pop",
);
