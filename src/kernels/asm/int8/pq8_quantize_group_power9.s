# int8/pq8_quantize_group_power9.s: kernel.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_int8.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.text.pq8_quantize_group_power9,"ax",@progbits
	.globl	pq8_quantize_group_power9     # -- Begin function pq8_quantize_group_power9
	.p2align 6
	.type	pq8_quantize_group_power9,@function
pq8_quantize_group_power9:            # @pq8_quantize_group_power9
.Lp9_int8_func_begin9:
	.cfi_startproc
.Lp9_int8_func_gep9:
	addis 2, 12, .TOC.-.Lp9_int8_func_gep9@ha
	addi 2, 2, .TOC.-.Lp9_int8_func_gep9@l
.Lp9_int8_func_lep9:
	.localentry	pq8_quantize_group_power9, .Lp9_int8_func_lep9-.Lp9_int8_func_gep9
# %bb.0:
	mr	10, 8
	mr	8, 6
	li 6, 2
	cmplwi	3, 35
	bgt	0, .Lp9_int8_BB9_4
# %bb.1:
	clrldi	12, 3, 32
	li 0, 1
	sld 11, 0, 12
	andi. 11, 11, 324
	bne	0, .Lp9_int8_BB9_5
# %bb.2:
	sld 12, 0, 12
	lis 0, 128
	li 11, 256
	ori 0, 0, 5
	rldic 0, 0, 12, 28
	and. 12, 12, 0
	beq	0, .Lp9_int8_BB9_4
# %bb.3:
	li 6, 1
	cmpldi	5, 0
	bne	0, .Lp9_int8_BB9_6
.Lp9_int8_BB9_4:
	mr	3, 6
	blr
.Lp9_int8_BB9_5:
	li 11, 32
	li 6, 1
	cmpldi	5, 0
	beq	0, .Lp9_int8_BB9_4
.Lp9_int8_BB9_6:
	addi 12, 11, -1
	and. 12, 12, 5
	bne	0, .Lp9_int8_BB9_4
# %bb.7:
	mflr 0
	stdu 1, -112(1)
	std 0, 128(1)
	.cfi_def_cfa_offset 112
	.cfi_offset lr, 16
	cnttzd 6, 11
	std 9, 96(1)
	mr	9, 7
	srd 11, 5, 6
	li 6, 0
	mr	7, 11
	bl pq8_quantize_group_range_power9
	nop
	mr	6, 3
	addi 1, 1, 112
	ld 0, 16(1)
	mtlr 0
	mr	3, 6
	blr
	.long	0
	.quad	0
.Lp9_int8_func_end9:
	.size	pq8_quantize_group_power9, .Lp9_int8_func_end9-.Lp9_int8_func_begin9
	.cfi_endproc
                                        # -- End function
