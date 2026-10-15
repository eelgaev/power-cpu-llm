//! The POWER9 `attention` kernels: the canonical assembly in `asm/attention/`, included
//! as is. The include order is the code layout; keep it when adding a file.
core::arch::global_asm!(
    ".machine push\n.machine power9\n\t.abiversion 2",
    include_str!("asm/attention/pa_attention_f32_power9_heads.s"),
    include_str!("asm/attention/paf_softmax.s"),
    include_str!("asm/attention/pa_attention_f32_power9.s"),
    include_str!("asm/attention/pa_attention_f32_power9_multi.s"),
    include_str!("asm/attention/pa_attention_f32_power9_multi_kv.s"),
    include_str!("asm/attention/pafm_rows.s"),
    ".text\n.machine pop",
);
