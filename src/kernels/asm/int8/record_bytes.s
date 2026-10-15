# int8/record_bytes.s: static helper.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_int8.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.text.record_bytes,"ax",@progbits
	.p2align	4                               # -- Begin function p9_int8_record_bytes
	.type	p9_int8_record_bytes,@function
p9_int8_record_bytes:                           # @p9_int8_record_bytes
.Lp9_int8_func_begin1:
	.cfi_startproc
# %bb.0:
	cmplwi	3, 35
	bgt	0, .Lp9_int8_BB1_2
# %bb.1:
	clrldi	5, 3, 32
	li 6, 1
	li 4, 256
	sld 5, 6, 5
	lis 6, 128
	ori 6, 6, 5
	rldic 6, 6, 12, 28
	and. 5, 5, 6
	bne	0, .Lp9_int8_BB1_3
.Lp9_int8_BB1_2:
	rlwinm 4, 3, 0, 30, 28
	cmpwi 1, 3, 8
	cmpwi	4, 2
	li 4, 32
	crnor 20, 6, 2
	isel 4, 0, 4, 20
.Lp9_int8_BB1_3:
	cmpwi	3, 12
	cmpwi 1, 3, 2
	li 6, 1
	li 7, 2
	li 8, 288
	rlwinm 5, 3, 0, 31, 29
	crnor 20, 6, 2
	cmplwi	3, 14
	isel 6, 7, 6, 20
	sld 4, 4, 6
	li 6, 16
	iseleq	6, 8, 6
	cmplwi	3, 12
	li 3, 224
	iseleq	3, 3, 6
	cmplwi	5, 12
	add 3, 4, 3
	li 4, 3
	iseleq	4, 7, 4
	sld 3, 3, 4
	blr
	.long	0
	.quad	0
.Lp9_int8_func_end1:
	.size	p9_int8_record_bytes, .Lp9_int8_func_end1-.Lp9_int8_func_begin1
	.cfi_endproc
                                        # -- End function
