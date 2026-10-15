# attention/pafm_rows.s: static helper.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/attention_f32_power9.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.text.pafm_rows,"ax",@progbits
	.p2align	4                               # -- Begin function p9_attention_pafm_rows
	.type	p9_attention_pafm_rows,@function
p9_attention_pafm_rows:                              # @p9_attention_pafm_rows
.Lp9_attention_func_begin5:
	.cfi_startproc
# %bb.0:
	std 27, -40(1)                          # 8-byte Folded Spill
	ld 27, 56(1)
	std 26, -48(1)                          # 8-byte Folded Spill
	ld 12, 40(1)
	cmpldi	7, 0
	divdu 26, 27, 9
	std 25, -56(1)                          # 8-byte Folded Spill
	ld 25, 32(1)
	std 23, -72(1)                          # 8-byte Folded Spill
	std 24, -64(1)                          # 8-byte Folded Spill
	std 28, -32(1)                          # 8-byte Folded Spill
	sub	24, 25, 12
	mulld 28, 25, 9
	std 22, -80(1)                          # 8-byte Folded Spill
	ld 12, 48(1)
	sub	12, 12, 27
	mulld 0, 24, 9
	std 29, -24(1)                          # 8-byte Folded Spill
	ld 29, 64(1)
	std 30, -16(1)                          # 8-byte Folded Spill
	ld 30, 72(1)
	std 21, -88(1)                          # 8-byte Folded Spill
	mulld 11, 26, 9
	mulld 22, 26, 4
	sub	23, 27, 11
	sldi 22, 22, 2
	ld 11, 80(1)
	maddld 25, 25, 9, 23
	maddld 24, 24, 9, 23
	add 22, 3, 22
	mulld 25, 25, 10
	mulld 24, 24, 10
	sldi 25, 25, 2
	sldi 24, 24, 2
	add 25, 22, 25
	mulld 22, 26, 6
	std 25, 0(29)
	sldi 25, 22, 2
	add 25, 5, 25
	add 25, 25, 24
	std 25, 0(30)
	beq	0, .Lp9_attention_BB5_2
# %bb.1:
	subfic 24, 12, 1
	cmpldi	12, 2
	li 24, 0
	li 23, 2
	maddld 26, 26, 8, 7
	li 25, 1
	li 22, 3
	iselgt	23, 23, 24
	cmpldi	12, 3
	subfe 25, 25, 25
	add 23, 23, 27
	iselgt	24, 22, 24
	neg 25, 25
	std 26, 0(11)
	divdu 26, 23, 9
	add 24, 24, 27
	add 27, 27, 25
	divdu 25, 27, 9
	maddld 21, 26, 8, 7
	mulld 22, 26, 9
	std 21, 16(11)
	maddld 21, 25, 8, 7
	sub	23, 23, 22
	std 21, 8(11)
	divdu 21, 24, 9
	mulld 22, 21, 9
	mulld 9, 25, 9
	maddld 7, 21, 8, 7
	sub	24, 24, 22
	mulld 22, 26, 4
	sub	9, 27, 9
	mulld 27, 21, 4
	mulld 4, 25, 4
	mulld 26, 26, 6
	mulld 25, 25, 6
	mulld 6, 21, 6
	sldi 22, 22, 2
	sldi 27, 27, 2
	sldi 4, 4, 2
	sldi 26, 26, 2
	sldi 25, 25, 2
	add 22, 3, 22
	add 27, 3, 27
	add 3, 3, 4
	add 4, 23, 28
	add 23, 23, 0
	add 26, 5, 26
	add 25, 5, 25
	mulld 23, 23, 10
	mulld 4, 4, 10
	sldi 23, 23, 2
	sldi 4, 4, 2
	add 26, 26, 23
	add 23, 24, 28
	add 28, 9, 28
	add 9, 9, 0
	add 4, 22, 4
	mulld 28, 28, 10
	mulld 9, 9, 10
	mulld 23, 23, 10
	sldi 28, 28, 2
	sldi 9, 9, 2
	sldi 23, 23, 2
	add 3, 3, 28
	add 9, 25, 9
	add 27, 27, 23
	std 3, 8(29)
	std 9, 8(30)
	std 4, 16(29)
	add 4, 24, 0
	sldi 3, 6, 2
	std 26, 16(30)
	std 27, 24(29)
	mulld 4, 4, 10
	add 3, 5, 3
	b .Lp9_attention_BB5_3
.Lp9_attention_BB5_2:
	li 8, 1
	subfic 7, 12, 1
	cmpldi	12, 2
	li 7, 0
	std 7, 16(11)
	li 26, 2
	iselgt	26, 26, 7
	cmpldi	12, 3
	li 25, 3
	subfe 8, 8, 8
	xxlxor 0, 0, 0
	iselgt	25, 25, 7
	neg 8, 8
	add 26, 26, 27
	stxv 0, 0(11)
	add 25, 25, 27
	add 8, 27, 8
	divdu 27, 26, 9
	divdu 24, 25, 9
	mulld 23, 27, 9
	mulld 22, 24, 9
	sub	26, 26, 23
	divdu 23, 8, 9
	sub	25, 25, 22
	mulld 22, 27, 4
	mulld 27, 27, 6
	sldi 22, 22, 2
	sldi 27, 27, 2
	add 22, 3, 22
	add 27, 5, 27
	mulld 9, 23, 9
	sub	8, 8, 9
	mulld 9, 24, 4
	mulld 4, 23, 4
	sldi 9, 9, 2
	sldi 4, 4, 2
	add 9, 3, 9
	add 3, 3, 4
	add 4, 26, 28
	add 26, 26, 0
	mulld 26, 26, 10
	mulld 4, 4, 10
	sldi 26, 26, 2
	sldi 4, 4, 2
	add 27, 27, 26
	add 26, 25, 28
	add 28, 8, 28
	add 8, 8, 0
	add 4, 22, 4
	mulld 26, 26, 10
	mulld 28, 28, 10
	mulld 8, 8, 10
	sldi 26, 26, 2
	sldi 28, 28, 2
	sldi 8, 8, 2
	add 9, 9, 26
	mulld 26, 23, 6
	add 3, 3, 28
	std 3, 8(29)
	mulld 3, 24, 6
	sldi 26, 26, 2
	add 26, 5, 26
	sldi 3, 3, 2
	add 8, 26, 8
	add 3, 5, 3
	std 8, 8(30)
	std 4, 16(29)
	add 4, 25, 0
	std 27, 16(30)
	std 9, 24(29)
	mulld 4, 4, 10
.Lp9_attention_BB5_3:
	sldi 4, 4, 2
	add 3, 3, 4
	std 3, 24(30)
	cmpldi	12, 4
	li 3, 4
	ld 30, -16(1)                           # 8-byte Folded Reload
	ld 29, -24(1)                           # 8-byte Folded Reload
	ld 28, -32(1)                           # 8-byte Folded Reload
	ld 27, -40(1)                           # 8-byte Folded Reload
	ld 26, -48(1)                           # 8-byte Folded Reload
	isellt	3, 12, 3
	ld 25, -56(1)                           # 8-byte Folded Reload
	ld 24, -64(1)                           # 8-byte Folded Reload
	ld 23, -72(1)                           # 8-byte Folded Reload
	ld 22, -80(1)                           # 8-byte Folded Reload
	ld 21, -88(1)                           # 8-byte Folded Reload
	std 7, 24(11)
	blr
	.long	0
	.quad	0
.Lp9_attention_func_end5:
	.size	p9_attention_pafm_rows, .Lp9_attention_func_end5-.Lp9_attention_func_begin5
	.cfi_endproc
                                        # -- End function
