# int8/pq8_packed_bytes_power9.s: kernel.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_int8.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.text.pq8_packed_bytes_power9,"ax",@progbits
	.globl	pq8_packed_bytes_power9       # -- Begin function pq8_packed_bytes_power9
	.p2align 6
	.type	pq8_packed_bytes_power9,@function
pq8_packed_bytes_power9:              # @pq8_packed_bytes_power9
.Lp9_int8_func_begin0:
	.cfi_startproc
# %bb.0:
	cmplwi 1, 3, 35
	bgt	1, .Lp9_int8_BB0_7
# %bb.1:
	clrldi	6, 3, 32
	li 8, 1
	li 7, 256
	sld 6, 8, 6
	lis 8, 128
	ori 8, 8, 5
	rldic 8, 8, 12, 28
	and. 6, 6, 8
	beq	0, .Lp9_int8_BB0_7
# %bb.2:
	cmpdi	7, 0
	li 6, 0
	bc 12, 2, .Lp9_int8_BB0_6
.Lp9_int8_BB0_3:
	cmpdi	4, 0
	bc 12, 2, .Lp9_int8_BB0_6
# %bb.4:
	cmpdi	5, 0
	bc 12, 2, .Lp9_int8_BB0_6
# %bb.5:
	addi 8, 7, -1
	li 9, -1
	rldimi 8, 9, 61, 0
	and. 8, 8, 5
	beq	0, .Lp9_int8_BB0_8
.Lp9_int8_BB0_6:
	mr	3, 6
	blr
.Lp9_int8_BB0_7:
	rlwinm 6, 3, 0, 30, 28
	cmpwi 5, 3, 8
	cmpwi	6, 2
	li 6, 32
	crnor 20, 22, 2
	isel 7, 0, 6, 20
	cmpdi	7, 0
	li 6, 0
	bc 4, 2, .Lp9_int8_BB0_3
	b .Lp9_int8_BB0_6
.Lp9_int8_BB0_8:
	rlwinm 6, 3, 0, 31, 29
	addi 4, 4, 3
	cmpwi 5, 6, 12
	cnttzd 6, 7
	rldicl 4, 4, 62, 2
	srd 5, 5, 6
	bgt	1, .Lp9_int8_BB0_10
# %bb.9:
	clrldi	7, 3, 32
	li 8, 1
	li 6, 256
	sld 7, 8, 7
	lis 8, 128
	ori 8, 8, 5
	rldic 8, 8, 12, 28
	and. 7, 7, 8
	bne	0, .Lp9_int8_BB0_11
.Lp9_int8_BB0_10:
	rlwinm 6, 3, 0, 30, 28
	cmpwi 1, 3, 8
	cmpwi	6, 2
	li 6, 32
	crnor 20, 6, 2
	isel 6, 0, 6, 20
.Lp9_int8_BB0_11:
	cmpwi	3, 12
	cmpwi 1, 3, 2
	li 7, 1
	li 8, 2
	li 9, 288
	crnor 20, 6, 2
	cmplwi	3, 14
	isel 7, 8, 7, 20
	sld 6, 6, 7
	li 7, 16
	iseleq	7, 9, 7
	cmplwi	3, 12
	li 3, 224
	iseleq	3, 3, 7
	li 7, 3
	add 3, 6, 3
	li 6, 3
	isel 6, 8, 6, 22
	sld 3, 3, 6
	mulld 3, 3, 5
	li 5, 7
	isel 5, 7, 5, 22
	add 4, 5, 4
	srd 4, 4, 6
	mulhdu. 5, 3, 4
	mulld 3, 3, 4
	li 4, 0
	iseleq	6, 3, 4
	mr	3, 6
	blr
	.long	0
	.quad	0
.Lp9_int8_func_end0:
	.size	pq8_packed_bytes_power9, .Lp9_int8_func_end0-.Lp9_int8_func_begin0
	.cfi_endproc
                                        # -- End function
