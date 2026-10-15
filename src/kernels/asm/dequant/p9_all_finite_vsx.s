# dequant/p9_all_finite_vsx.s: kernel.
# Frozen Clang20 -O3 -mcpu=power9 -ffp-contract=off -std=c11 assembly.
# Retain ELFv2 TOC/localentry, constants, calls, and unwind metadata.
	.text
	.globl	p9_all_finite_vsx
	.p2align	7
	.type	p9_all_finite_vsx,@function
p9_all_finite_vsx:
.Lp9_dequant_func_begin1:
	.cfi_startproc
	xxlxor 34, 34, 34
	li 6, 0
	cmpldi	4, 4
	xvcmpeqsp 35, 34, 34
	blt	0, .Lp9_dequant_BB1_3
	li 7, 0
	mr	5, 3
	.p2align	4
.Lp9_dequant_BB1_2:
	lxv 0, 0(5)
	addi 6, 7, 4
	addi 7, 7, 8
	addi 5, 5, 16
	cmpld	7, 4
	mr	7, 6
	xvsubsp 0, 0, 0
	xvcmpeqsp 0, 0, 34
	xxland 35, 0, 35
	ble 0, .Lp9_dequant_BB1_2
.Lp9_dequant_BB1_3:
	vcmpequw. 2, 3, 2
	li 5, 0
	bne	6, .Lp9_dequant_BB1_9
	cmpld	6, 4
	bge 0, .Lp9_dequant_BB1_8
	sldi 7, 6, 2
	sub	4, 4, 6
	add 3, 7, 3
	mtctr 4
	addi 3, 3, -4
	.p2align	4
.Lp9_dequant_BB1_6:
	lfsu 0, 4(3)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_dequant_BB1_9
	bdnz .Lp9_dequant_BB1_6
.Lp9_dequant_BB1_8:
	li 5, 1
.Lp9_dequant_BB1_9:
	mr	3, 5
	blr
	.long	0
	.quad	0
.Lp9_dequant_func_end1:
	.size	p9_all_finite_vsx, .Lp9_dequant_func_end1-.Lp9_dequant_func_begin1
	.cfi_endproc
