# attention/paf_softmax.s: static helper.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/attention_f32_power9.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function p9_attention_paf_softmax
.Lp9_attention_CPI1_0:
	.long	0xff800000                      # float -Inf
	.section	.text.paf_softmax,"ax",@progbits
	.p2align	4
	.type	p9_attention_paf_softmax,@function
p9_attention_paf_softmax:                            # @p9_attention_paf_softmax
.Lp9_attention_func_begin1:
	.cfi_startproc
.Lp9_attention_func_gep1:
	addis 2, 12, .TOC.-.Lp9_attention_func_gep1@ha
	addi 2, 2, .TOC.-.Lp9_attention_func_gep1@l
.Lp9_attention_func_lep1:
	.localentry	p9_attention_paf_softmax, .Lp9_attention_func_lep1-.Lp9_attention_func_gep1
# %bb.0:
	mflr 0
	.cfi_def_cfa_offset 112
	.cfi_offset lr, 16
	.cfi_offset r24, -80
	.cfi_offset r25, -72
	.cfi_offset r26, -64
	.cfi_offset r27, -56
	.cfi_offset r28, -48
	.cfi_offset r29, -40
	.cfi_offset r30, -32
	.cfi_offset f30, -16
	.cfi_offset f31, -8
	std 24, -80(1)                          # 8-byte Folded Spill
	std 25, -72(1)                          # 8-byte Folded Spill
	std 26, -64(1)                          # 8-byte Folded Spill
	std 27, -56(1)                          # 8-byte Folded Spill
	std 28, -48(1)                          # 8-byte Folded Spill
	std 29, -40(1)                          # 8-byte Folded Spill
	std 30, -32(1)                          # 8-byte Folded Spill
	stfd 30, -16(1)                         # 8-byte Folded Spill
	stfd 31, -8(1)                          # 8-byte Folded Spill
	stdu 1, -112(1)
	li 30, 1
	cmpldi	4, 0
	std 0, 128(1)
	beq	0, .Lp9_attention_BB1_30
# %bb.1:
	mr	29, 4
	mr	28, 3
	cmpldi	5, 0
	beq	0, .Lp9_attention_BB1_12
# %bb.2:
	addis 6, 2, .Lp9_attention_CPI1_0@toc@ha
	mtctr 29
	addi 3, 5, -1
	addi 4, 28, -4
	lfs 30, .Lp9_attention_CPI1_0@toc@l(6)
	b .Lp9_attention_BB1_4
	.p2align	4
.Lp9_attention_BB1_3:                                #   in Loop: Header=BB1_4 Depth=1
	bdz .Lp9_attention_BB1_8
.Lp9_attention_BB1_4:                                # =>This Inner Loop Header: Depth=1
	lbzu 7, 1(3)
	mr	6, 4
	addi 4, 4, 4
	cmplwi	7, 0
	beq	0, .Lp9_attention_BB1_3
# %bb.5:                                #   in Loop: Header=BB1_4 Depth=1
	lfs 0, 4(6)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB1_16
# %bb.6:                                #   in Loop: Header=BB1_4 Depth=1
	fcmpu 0, 0, 30
	ble 0, .Lp9_attention_BB1_3
# %bb.7:                                #   in Loop: Header=BB1_4 Depth=1
	fmr 30, 0
	b .Lp9_attention_BB1_3
.Lp9_attention_BB1_8:
	addi 27, 5, -1
	addi 25, 28, -4
	mr	26, 29
	xxlxor 31, 31, 31
	b .Lp9_attention_BB1_10
	.p2align	4
.Lp9_attention_BB1_9:                                #   in Loop: Header=BB1_10 Depth=1
	xsaddsp 31, 31, 1
	addi 26, 26, -1
	stfs 1, 4(25)
	mr	25, 24
	cmpldi	26, 0
	bc 4, 1, .Lp9_attention_BB1_19
.Lp9_attention_BB1_10:                               # =>This Inner Loop Header: Depth=1
	lbzu 3, 1(27)
	addi 24, 25, 4
	xxlxor 1, 1, 1
	cmplwi	3, 0
	beq	0, .Lp9_attention_BB1_9
# %bb.11:                               #   in Loop: Header=BB1_10 Depth=1
	lfs 0, 4(25)
	xssubsp 1, 0, 30
	bl power_expf
	nop
	b .Lp9_attention_BB1_9
.Lp9_attention_BB1_12:
	lfs 0, 0(28)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB1_16
# %bb.13:
	addis 3, 2, .Lp9_attention_CPI1_0@toc@ha
	mtctr 29
	lfs 30, .Lp9_attention_CPI1_0@toc@l(3)
	mr	3, 28
	.p2align	5
.Lp9_attention_BB1_14:                               # =>This Inner Loop Header: Depth=1
	xsmaxcdp 30, 0, 30
	bdz .Lp9_attention_BB1_17
# %bb.15:                               #   in Loop: Header=BB1_14 Depth=1
	lfs 0, 4(3)
	addi 3, 3, 4
	xststdcsp 0, 0, 112
	bc 4, 2, .Lp9_attention_BB1_14
.Lp9_attention_BB1_16:
	li 30, 0
	b .Lp9_attention_BB1_30
.Lp9_attention_BB1_17:
	addi 27, 28, -4
	xxlxor 31, 31, 31
	mr	26, 29
	.p2align	4
.Lp9_attention_BB1_18:                               # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(27)
	xssubsp 1, 0, 30
	bl power_expf
	nop
	xsaddsp 31, 31, 1
	addi 26, 26, -1
	stfs 1, 0(27)
	cmpldi	26, 0
	bc 12, 1, .Lp9_attention_BB1_18
.Lp9_attention_BB1_19:
	li 3, 0
	cmpldi	29, 3
	ble 0, .Lp9_attention_BB1_28
# %bb.20:
	xscvdpspn 0, 31
	cmpldi	29, 48
	blt	0, .Lp9_attention_BB1_25
# %bb.21:
	lis 3, 21845
	xxspltw 1, 0, 0
	addi 5, 28, 96
	ori 3, 3, 21845
	rldic 3, 3, 33, 0
	oris 3, 3, 43690
	ori 6, 3, 43691
	mulhdu 3, 29, 6
	rldicl 3, 3, 59, 5
	mulli 3, 3, 48
	addi 7, 3, -48
	sub	4, 29, 3
	mulhdu 6, 7, 6
	rldicl 6, 6, 59, 5
	addi 6, 6, 1
	mtctr 6
	.p2align	4
.Lp9_attention_BB1_22:                               # =>This Inner Loop Header: Depth=1
	lxv 2, -96(5)
	lxv 12, 64(5)
	lxv 3, -80(5)
	lxv 4, -64(5)
	xvdivsp 2, 2, 1
	lxv 5, -48(5)
	lxv 6, -32(5)
	lxv 7, -16(5)
	lxv 8, 0(5)
	lxv 9, 16(5)
	lxv 10, 32(5)
	lxv 11, 48(5)
	lxv 13, 80(5)
	xvdivsp 3, 3, 1
	stxv 2, -96(5)
	xvdivsp 2, 12, 1
	xvdivsp 4, 4, 1
	xvdivsp 5, 5, 1
	stxv 3, -80(5)
	xvdivsp 6, 6, 1
	xvdivsp 7, 7, 1
	xvdivsp 8, 8, 1
	stxv 4, -64(5)
	stxv 5, -48(5)
	stxv 6, -32(5)
	stxv 7, -16(5)
	stxv 8, 0(5)
	stxv 2, 64(5)
	xvdivsp 9, 9, 1
	xvdivsp 10, 10, 1
	xvdivsp 11, 11, 1
	xvdivsp 2, 13, 1
	stxv 9, 16(5)
	stxv 10, 32(5)
	stxv 11, 48(5)
	stxv 2, 80(5)
	addi 5, 5, 192
	bdnz .Lp9_attention_BB1_22
# %bb.23:
	cmpldi	4, 0
	beq	0, .Lp9_attention_BB1_30
# %bb.24:
	cmpldi	4, 4
	blt	0, .Lp9_attention_BB1_28
.Lp9_attention_BB1_25:
	mr	5, 3
	rldicr 3, 29, 0, 61
	xxspltw 0, 0, 0
	sldi 4, 5, 2
	sub	5, 3, 5
	addi 5, 5, -4
	add 4, 28, 4
	rldicl 5, 5, 62, 2
	addi 5, 5, 1
	mtctr 5
	.p2align	5
.Lp9_attention_BB1_26:                               # =>This Inner Loop Header: Depth=1
	lxv 1, 0(4)
	xvdivsp 1, 1, 0
	stxv 1, 0(4)
	addi 4, 4, 16
	bdnz .Lp9_attention_BB1_26
# %bb.27:
	cmpld	29, 3
	beq	0, .Lp9_attention_BB1_30
.Lp9_attention_BB1_28:
	sldi 4, 3, 2
	sub	3, 29, 3
	add 4, 4, 28
	mtctr 3
	addi 4, 4, -4
	.p2align	4
.Lp9_attention_BB1_29:                               # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	xsdivsp 0, 0, 31
	stfs 0, 0(4)
	bdnz .Lp9_attention_BB1_29
.Lp9_attention_BB1_30:
	mr	3, 30
	addi 1, 1, 112
	ld 0, 16(1)
	lfd 31, -8(1)                           # 8-byte Folded Reload
	lfd 30, -16(1)                          # 8-byte Folded Reload
	ld 30, -32(1)                           # 8-byte Folded Reload
	ld 29, -40(1)                           # 8-byte Folded Reload
	ld 28, -48(1)                           # 8-byte Folded Reload
	ld 27, -56(1)                           # 8-byte Folded Reload
	ld 26, -64(1)                           # 8-byte Folded Reload
	ld 25, -72(1)                           # 8-byte Folded Reload
	mtlr 0
	ld 24, -80(1)                           # 8-byte Folded Reload
	blr
	.long	0
	.quad	0
.Lp9_attention_func_end1:
	.size	p9_attention_paf_softmax, .Lp9_attention_func_end1-.Lp9_attention_func_begin1
	.cfi_endproc
                                        # -- End function
