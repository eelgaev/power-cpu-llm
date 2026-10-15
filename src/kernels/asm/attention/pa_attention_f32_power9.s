# attention/pa_attention_f32_power9.s: kernel.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/attention_f32_power9.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.text.pa_attention_f32_power9,"ax",@progbits
	.globl	pa_attention_f32_power9       # -- Begin function pa_attention_f32_power9
	.p2align 6
	.type	pa_attention_f32_power9,@function
pa_attention_f32_power9:              # @pa_attention_f32_power9
.Lp9_attention_func_begin2:
	.cfi_startproc
.Lp9_attention_func_gep2:
	addis 2, 12, .TOC.-.Lp9_attention_func_gep2@ha
	addi 2, 2, .TOC.-.Lp9_attention_func_gep2@l
.Lp9_attention_func_lep2:
	.localentry	pa_attention_f32_power9, .Lp9_attention_func_lep2-.Lp9_attention_func_gep2
# %bb.0:
	mflr 0
	stdu 1, -128(1)
	std 0, 144(1)
	.cfi_def_cfa_offset 128
	.cfi_offset lr, 16
	lxv 0, 224(1)
	li 11, 0
	std 6, 120(1)
	std 11, 112(1)
	stxv 0, 96(1)
	bl pa_attention_f32_power9_heads
	nop
	addi 1, 1, 128
	ld 0, 16(1)
	mtlr 0
	blr
	.long	0
	.quad	0
.Lp9_attention_func_end2:
	.size	pa_attention_f32_power9, .Lp9_attention_func_end2-.Lp9_attention_func_begin2
	.cfi_endproc
                                        # -- End function
