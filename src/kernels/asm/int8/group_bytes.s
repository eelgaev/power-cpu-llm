# int8/group_bytes.s: static helper.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_int8.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.text.group_bytes,"ax",@progbits
	.p2align	4                               # -- Begin function p9_int8_group_bytes
	.type	p9_int8_group_bytes,@function
p9_int8_group_bytes:                            # @p9_int8_group_bytes
.Lp9_int8_func_begin3:
	.cfi_startproc
# %bb.0:
	cmplwi	3, 35
	bgt	0, .Lp9_int8_BB3_2
# %bb.1:
	clrldi	6, 3, 32
	li 7, 1
	li 5, 256
	sld 6, 7, 6
	lis 7, 128
	ori 7, 7, 5
	rldic 7, 7, 12, 28
	and. 6, 6, 7
	bne	0, .Lp9_int8_BB3_3
.Lp9_int8_BB3_2:
	rlwinm 5, 3, 0, 30, 28
	cmpwi 1, 3, 8
	cmpwi	5, 2
	li 5, 32
	crnor 20, 6, 2
	isel 5, 0, 5, 20
.Lp9_int8_BB3_3:
	cmpwi	3, 12
	cmpwi 1, 3, 2
	li 6, 1
	li 7, 2
	cnttzd 5, 5
	crnor 20, 6, 2
	cmplwi	3, 14
	isel 6, 7, 6, 20
	li 7, 288
	sld 6, 4, 6
	srd 4, 4, 5
	li 5, 16
	iseleq	5, 7, 5
	cmplwi	3, 12
	li 3, 224
	iseleq	3, 3, 5
	maddld 3, 4, 3, 6
	blr
	.long	0
	.quad	0
.Lp9_int8_func_end3:
	.size	p9_int8_group_bytes, .Lp9_int8_func_end3-.Lp9_int8_func_begin3
	.cfi_endproc
                                        # -- End function
