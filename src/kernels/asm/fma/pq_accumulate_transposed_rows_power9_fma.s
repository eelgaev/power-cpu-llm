# fma/pq_accumulate_transposed_rows_power9_fma.s: kernel.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_fma.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.text.pq_accumulate_transposed_rows_power9_fma,"ax",@progbits
	.globl	pq_accumulate_transposed_rows_power9_fma # -- Begin function pq_accumulate_transposed_rows_power9_fma
	.p2align 6
	.type	pq_accumulate_transposed_rows_power9_fma,@function
pq_accumulate_transposed_rows_power9_fma: # @pq_accumulate_transposed_rows_power9_fma
.Lp9_fma_func_begin1:
	.cfi_startproc
.Lp9_fma_func_gep1:
	addis 2, 12, .TOC.-.Lp9_fma_func_gep1@ha
	addi 2, 2, .TOC.-.Lp9_fma_func_gep1@l
.Lp9_fma_func_lep1:
	.localentry	pq_accumulate_transposed_rows_power9_fma, .Lp9_fma_func_lep1-.Lp9_fma_func_gep1
# %bb.0:
	mfcr 12
	mflr 0
	stw 12, 8(1)
	stdu 1, -496(1)
	std 0, 512(1)
	.cfi_def_cfa_offset 496
	.cfi_offset lr, 16
	.cfi_offset r14, -144
	.cfi_offset r15, -136
	.cfi_offset r16, -128
	.cfi_offset r17, -120
	.cfi_offset r18, -112
	.cfi_offset r19, -104
	.cfi_offset r20, -96
	.cfi_offset r21, -88
	.cfi_offset r22, -80
	.cfi_offset r23, -72
	.cfi_offset r24, -64
	.cfi_offset r25, -56
	.cfi_offset r26, -48
	.cfi_offset r27, -40
	.cfi_offset r28, -32
	.cfi_offset r29, -24
	.cfi_offset r30, -16
	.cfi_offset r31, -8
	.cfi_offset cr2, 8
	.cfi_offset cr3, 8
	.cfi_offset cr4, 8
	std 17, 376(1)                          # 8-byte Folded Spill
	std 28, 464(1)                          # 8-byte Folded Spill
	ld 28, 608(1)
	ld 17, 600(1)
	mr	11, 3
	cmpdi	4, 0
	li 3, 1
	std 14, 352(1)                          # 8-byte Folded Spill
	std 15, 360(1)                          # 8-byte Folded Spill
	std 16, 368(1)                          # 8-byte Folded Spill
	std 18, 384(1)                          # 8-byte Folded Spill
	std 19, 392(1)                          # 8-byte Folded Spill
	std 20, 400(1)                          # 8-byte Folded Spill
	std 21, 408(1)                          # 8-byte Folded Spill
	std 22, 416(1)                          # 8-byte Folded Spill
	std 23, 424(1)                          # 8-byte Folded Spill
	std 24, 432(1)                          # 8-byte Folded Spill
	std 25, 440(1)                          # 8-byte Folded Spill
	std 26, 448(1)                          # 8-byte Folded Spill
	std 27, 456(1)                          # 8-byte Folded Spill
	std 29, 472(1)                          # 8-byte Folded Spill
	std 30, 480(1)                          # 8-byte Folded Spill
	std 31, 488(1)                          # 8-byte Folded Spill
	bc 12, 2, .Lp9_fma_BB1_10
# %bb.1:
	cmpdi	8, 0
	bc 12, 2, .Lp9_fma_BB1_10
# %bb.2:
	cmpdi	17, 0
	bc 12, 2, .Lp9_fma_BB1_10
# %bb.3:
	cmpdi	28, 0
	bc 12, 2, .Lp9_fma_BB1_10
# %bb.4:
	cmpdi	6, 0
	bc 12, 2, .Lp9_fma_BB1_10
# %bb.5:
	cmpdi	7, 0
	bc 12, 2, .Lp9_fma_BB1_10
# %bb.6:
	cmpdi	9, 0
	bc 12, 2, .Lp9_fma_BB1_10
# %bb.7:
	ld 23, 592(1)
	not	12, 6
	cmpld	23, 12
	ble 0, .Lp9_fma_BB1_9
.Lp9_fma_BB1_8:
	li 3, 4
	b .Lp9_fma_BB1_10
.Lp9_fma_BB1_9:
	add 12, 23, 6
	cmpld	12, 10
	ble 0, .Lp9_fma_BB1_11
.Lp9_fma_BB1_10:
	ld 31, 488(1)                           # 8-byte Folded Reload
	ld 30, 480(1)                           # 8-byte Folded Reload
	ld 29, 472(1)                           # 8-byte Folded Reload
	ld 28, 464(1)                           # 8-byte Folded Reload
	ld 27, 456(1)                           # 8-byte Folded Reload
	ld 26, 448(1)                           # 8-byte Folded Reload
	ld 25, 440(1)                           # 8-byte Folded Reload
	ld 24, 432(1)                           # 8-byte Folded Reload
	ld 23, 424(1)                           # 8-byte Folded Reload
	ld 22, 416(1)                           # 8-byte Folded Reload
	ld 21, 408(1)                           # 8-byte Folded Reload
	ld 20, 400(1)                           # 8-byte Folded Reload
	ld 19, 392(1)                           # 8-byte Folded Reload
	ld 18, 384(1)                           # 8-byte Folded Reload
	ld 17, 376(1)                           # 8-byte Folded Reload
	ld 16, 368(1)                           # 8-byte Folded Reload
	ld 15, 360(1)                           # 8-byte Folded Reload
	ld 14, 352(1)                           # 8-byte Folded Reload
	addi 1, 1, 496
	ld 0, 16(1)
	lwz 12, 8(1)
	mtlr 0
	mtocrf 32, 12
	mtocrf 16, 12
	mtocrf 8, 12
	blr
.Lp9_fma_BB1_11:
	mulhdu. 3, 9, 10
	bne	0, .Lp9_fma_BB1_8
# %bb.12:
	mulhdu. 3, 9, 7
	bne	0, .Lp9_fma_BB1_8
# %bb.13:
	li 3, 2
	cmplwi	11, 35
	bgt	0, .Lp9_fma_BB1_10
# %bb.14:
	clrldi	12, 11, 32
	li 0, 1
	sld 12, 0, 12
	rldic 0, 0, 35, 28
	ori 0, 0, 20807
	and. 12, 12, 0
	beq	0, .Lp9_fma_BB1_10
# %bb.15:
	mr	25, 4
	mr	3, 11
	mr	4, 7
	mr	27, 5
	mr	21, 8
	mr	22, 6
	mr	26, 9
	mr	24, 10
	mr	20, 11
	mr	30, 7
	bl pq_row_bytes
	nop
	cmpldi	3, 0
	beq	0, .Lp9_fma_BB1_26
# %bb.16:
	mr	29, 3
	mulhdu. 3, 22, 3
	bne	0, .Lp9_fma_BB1_8
# %bb.17:
	mr	19, 22
	mulhdu. 3, 22, 30
	li 3, 4
	bne	0, .Lp9_fma_BB1_10
# %bb.18:
	mulld 3, 29, 19
	cmpld	3, 27
	bne	0, .Lp9_fma_BB1_26
# %bb.19:
	mr	7, 30
	ld 6, 616(1)
	mr	8, 30
	li 3, 5
	mulld 7, 7, 19
	cmpld	6, 7
	blt	0, .Lp9_fma_BB1_10
# %bb.20:
	mr	4, 25
	mr	5, 20
	mr	7, 8
	sldi 25, 8, 2
	mr	27, 28
.Lp9_fma_BB1_21:                               # =>This Inner Loop Header: Depth=1
	mr	3, 5
	mr	5, 29
	mr	6, 27
	mr	18, 4
	bl p9_decode_row_vsx
	nop
	cmpldi	3, 0
	beq	0, .Lp9_fma_BB1_26
# %bb.22:                               #   in Loop: Header=BB1_21 Depth=1
	mr	3, 27
	mr	4, 30
	bl p9_all_finite_vsx
	nop
	cmplwi	3, 0
	beq	0, .Lp9_fma_BB1_45
# %bb.23:                               #   in Loop: Header=BB1_21 Depth=1
	addi 19, 19, -1
	add 4, 18, 29
	add 27, 27, 25
	mr	7, 30
	mr	5, 20
	cmpldi	19, 0
	bc 12, 1, .Lp9_fma_BB1_21
# %bb.24:
	sldi 3, 23, 2
	mr	5, 22
	cmpldi	22, 4
	sldi 19, 7, 3
	add 3, 21, 3
	std 17, 264(1)                          # 8-byte Folded Spill
	std 7, 104(1)                           # 8-byte Folded Spill
	std 19, 248(1)                          # 8-byte Folded Spill
	std 3, 256(1)                           # 8-byte Folded Spill
	bge 0, .Lp9_fma_BB1_46
# %bb.25:
	lis 3, 1365
	addi 4, 17, 96
	addi 6, 28, 48
	li 8, 0
	mr	11, 7
	mr	12, 24
	mr	0, 26
	ori 3, 3, 21845
	rldic 3, 3, 33, 4
	oris 3, 3, 43690
	ori 9, 3, 43691
	lis 3, 21845
	ori 3, 3, 21845
	rldic 3, 3, 33, 0
	oris 3, 3, 43690
	ori 10, 3, 43691
	b .Lp9_fma_BB1_28
.Lp9_fma_BB1_26:
	li 3, 3
	b .Lp9_fma_BB1_10
.Lp9_fma_BB1_27:                               #   in Loop: Header=BB1_28 Depth=1
	ld 7, 104(1)                            # 8-byte Folded Reload
	addi 8, 8, 256
	addi 11, 11, -256
	addi 4, 4, 2048
	addi 6, 6, 1024
	li 3, 0
	cmpld	8, 7
	bge 0, .Lp9_fma_BB1_10
.Lp9_fma_BB1_28:                               # =>This Loop Header: Depth=1
                                        #     Child Loop BB1_32 Depth 2
                                        #       Child Loop BB1_34 Depth 3
                                        #         Child Loop BB1_37 Depth 4
                                        #         Child Loop BB1_41 Depth 4
                                        #         Child Loop BB1_44 Depth 4
	sub	3, 7, 8
	cmpldi	3, 256
	bc 4, 0, .Lp9_fma_BB1_30
# %bb.29:                               #   in Loop: Header=BB1_28 Depth=1
	ld 3, 104(1)                            # 8-byte Folded Reload
	cmpld	3, 8
	bc 4, 1, .Lp9_fma_BB1_27
.Lp9_fma_BB1_30:                               #   in Loop: Header=BB1_28 Depth=1
	cmpldi	11, 256
	li 3, 256
	cmpldi 1, 11, 2
	cmpldi 5, 11, 24
	li 23, 0
	mr	21, 4
	isellt	7, 11, 3
	mulhdu 3, 7, 9
	mulhdu 30, 7, 10
	rlwinm 26, 7, 0, 23, 30
	add 24, 7, 8
	add 27, 8, 26
	cmpd 6, 7, 26
	addi 26, 26, -2
	mulli 3, 3, 24
	rldicl 30, 30, 60, 4
	mulli 30, 30, 24
	addi 3, 3, -24
	mulhdu 3, 3, 10
	sub.	22, 7, 30
	add 29, 8, 30
	cmpldi 7, 22, 1
	mr	22, 17
	rldicl 3, 3, 60, 4
	addi 3, 3, 1
	b .Lp9_fma_BB1_32
.Lp9_fma_BB1_31:                               #   in Loop: Header=BB1_32 Depth=2
	ld 19, 248(1)                           # 8-byte Folded Reload
	addi 23, 23, 1
	ld 17, 264(1)                           # 8-byte Folded Reload
	cmpld 2, 23, 0
	add 21, 21, 19
	add 22, 22, 19
	beq	2, .Lp9_fma_BB1_27
.Lp9_fma_BB1_32:                               #   Parent Loop BB1_28 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB1_34 Depth 3
                                        #         Child Loop BB1_37 Depth 4
                                        #         Child Loop BB1_41 Depth 4
                                        #         Child Loop BB1_44 Depth 4
	maddld 7, 19, 23, 17
	ld 19, 256(1)                           # 8-byte Folded Reload
	mr	18, 28
	mr	17, 6
	li 16, 0
	addi 20, 7, -8
	mulld 7, 23, 12
	sldi 7, 7, 2
	add 19, 19, 7
	b .Lp9_fma_BB1_34
	.p2align	4
.Lp9_fma_BB1_33:                               #   in Loop: Header=BB1_34 Depth=3
	addi 16, 16, 1
	add 17, 17, 25
	add 18, 18, 25
	cmpld 2, 16, 5
	beq	2, .Lp9_fma_BB1_31
.Lp9_fma_BB1_34:                               #   Parent Loop BB1_28 Depth=1
                                        #     Parent Loop BB1_32 Depth=2
                                        # =>    This Loop Header: Depth=3
                                        #         Child Loop BB1_37 Depth 4
                                        #         Child Loop BB1_41 Depth 4
                                        #         Child Loop BB1_44 Depth 4
	sldi 7, 16, 2
	lfsx 0, 19, 7
	mr	7, 8
	bc 12, 4, .Lp9_fma_BB1_43
# %bb.35:                               #   in Loop: Header=BB1_34 Depth=3
	li 31, 0
	bc 12, 20, .Lp9_fma_BB1_40
# %bb.36:                               #   in Loop: Header=BB1_34 Depth=3
	mtctr 3
	xxspltd	1, 0, 0
	mr	15, 17
	mr	14, 21
	.p2align	5
.Lp9_fma_BB1_37:                               #   Parent Loop BB1_28 Depth=1
                                        #     Parent Loop BB1_32 Depth=2
                                        #       Parent Loop BB1_34 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfd 2, -48(15)
	lfd 3, -40(15)
	lxv 34, -96(14)
	lxv 35, -80(14)
	lxv 36, -64(14)
	lxv 37, -48(14)
	lxv 32, -32(14)
	lxv 33, -16(14)
	lxv 38, 0(14)
	xxmrghw 2, 2, 2
	lfd 4, -32(15)
	lfd 5, -24(15)
	lfd 6, -16(15)
	lfd 7, -8(15)
	lfd 8, 0(15)
	lfd 9, 8(15)
	xxmrghw 3, 3, 3
	xxmrghw 4, 4, 4
	xxmrghw 5, 5, 5
	xxmrghw 6, 6, 6
	xxmrghw 7, 7, 7
	xxmrghw 8, 8, 8
	xxmrghw 9, 9, 9
	lxv 39, 16(14)
	lxv 40, 32(14)
	lxv 41, 48(14)
	lxv 42, 64(14)
	lxv 43, 80(14)
	xvcvspdp 2, 2
	lfd 10, 16(15)
	xxmrghw 10, 10, 10
	xvcvspdp 3, 3
	xvcvspdp 4, 4
	xvcvspdp 5, 5
	xvcvspdp 6, 6
	xvcvspdp 7, 7
	xvcvspdp 8, 8
	xvcvspdp 9, 9
	xvcvspdp 10, 10
	xvmaddadp 34, 1, 2
	lfd 11, 24(15)
	lfd 12, 32(15)
	lfd 13, 40(15)
	xxmrghw 11, 11, 11
	xxmrghw 12, 12, 12
	xxmrghw 13, 13, 13
	xvcvspdp 11, 11
	xvcvspdp 12, 12
	xvcvspdp 13, 13
	xvmaddadp 35, 1, 3
	xvmaddadp 36, 1, 4
	xvmaddadp 37, 1, 5
	xvmaddadp 32, 1, 6
	xvmaddadp 33, 1, 7
	addi 15, 15, 96
	xvmaddadp 38, 1, 8
	xvmaddadp 39, 1, 9
	xvmaddadp 40, 1, 10
	stxv 34, -96(14)
	xvmaddadp 41, 1, 11
	xvmaddadp 42, 1, 12
	xvmaddadp 43, 1, 13
	stxv 35, -80(14)
	stxv 36, -64(14)
	stxv 37, -48(14)
	stxv 32, -32(14)
	stxv 33, -16(14)
	stxv 38, 0(14)
	stxv 39, 16(14)
	stxv 40, 32(14)
	stxv 41, 48(14)
	stxv 42, 64(14)
	stxv 43, 80(14)
	addi 14, 14, 192
	bdnz .Lp9_fma_BB1_37
# %bb.38:                               #   in Loop: Header=BB1_34 Depth=3
	beq	0, .Lp9_fma_BB1_33
# %bb.39:                               #   in Loop: Header=BB1_34 Depth=3
	mr	31, 30
	mr	7, 29
	beq	7, .Lp9_fma_BB1_43
.Lp9_fma_BB1_40:                               #   in Loop: Header=BB1_34 Depth=3
	add 7, 8, 31
	xxspltd	1, 0, 0
	sldi 15, 7, 2
	sldi 7, 7, 3
	add 14, 22, 7
	sub	7, 26, 31
	add 15, 18, 15
	rldicl 7, 7, 63, 1
	addi 7, 7, 1
	mtctr 7
	.p2align	5
.Lp9_fma_BB1_41:                               #   Parent Loop BB1_28 Depth=1
                                        #     Parent Loop BB1_32 Depth=2
                                        #       Parent Loop BB1_34 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfd 2, 0(15)
	lxv 3, 0(14)
	addi 15, 15, 8
	xxmrghw 2, 2, 2
	xvcvspdp 2, 2
	xvmaddadp 3, 1, 2
	stxv 3, 0(14)
	addi 14, 14, 16
	bdnz .Lp9_fma_BB1_41
# %bb.42:                               #   in Loop: Header=BB1_34 Depth=3
	mr	7, 27
	bc 12, 26, .Lp9_fma_BB1_33
.Lp9_fma_BB1_43:                               #   in Loop: Header=BB1_34 Depth=3
	maddld 15, 25, 16, 28
	sldi 14, 7, 3
	add 14, 20, 14
	sldi 31, 7, 2
	sub	7, 24, 7
	mtctr 7
	addi 15, 15, -4
	add 15, 15, 31
	.p2align	5
.Lp9_fma_BB1_44:                               #   Parent Loop BB1_28 Depth=1
                                        #     Parent Loop BB1_32 Depth=2
                                        #       Parent Loop BB1_34 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfsu 2, 4(15)
	lfdu 1, 8(14)
	xsmaddadp 1, 0, 2
	stfd 1, 0(14)
	bdnz .Lp9_fma_BB1_44
	b .Lp9_fma_BB1_33
.Lp9_fma_BB1_45:
	li 3, 6
	b .Lp9_fma_BB1_10
.Lp9_fma_BB1_46:
	addi 3, 28, 48
	addi 4, 28, 32
	addi 8, 17, 96
	addi 9, 17, 64
	std 28, 136(1)                          # 8-byte Folded Spill
	sldi 12, 7, 4
	li 20, 0
	std 24, 216(1)                          # 8-byte Folded Spill
	std 26, 208(1)                          # 8-byte Folded Spill
	std 3, 200(1)                           # 8-byte Folded Spill
	mulli 3, 7, 12
	add 6, 4, 3
	add 10, 28, 3
	addi 3, 25, -4
	std 4, 192(1)                           # 8-byte Folded Spill
	std 3, 344(1)                           # 8-byte Folded Spill
	lis 3, 1365
	std 6, 184(1)                           # 8-byte Folded Spill
	add 6, 4, 19
	add 4, 4, 25
	std 10, 112(1)                          # 8-byte Folded Spill
	std 10, 160(1)                          # 8-byte Folded Spill
	mr	10, 7
	ori 3, 3, 21845
	std 6, 176(1)                           # 8-byte Folded Spill
	std 4, 168(1)                           # 8-byte Folded Spill
	add 4, 28, 25
	add 6, 28, 19
	rldic 3, 3, 33, 4
	std 6, 120(1)                           # 8-byte Folded Spill
	std 6, 152(1)                           # 8-byte Folded Spill
	std 4, 128(1)                           # 8-byte Folded Spill
	std 4, 144(1)                           # 8-byte Folded Spill
	oris 3, 3, 43690
	ori 3, 3, 43691
	std 3, 64(1)                            # 8-byte Folded Spill
	lis 3, 21845
	ori 3, 3, 21845
	rldic 3, 3, 33, 0
	oris 3, 3, 43690
	ori 3, 3, 43691
	std 3, 56(1)                            # 8-byte Folded Spill
	b .Lp9_fma_BB1_48
.Lp9_fma_BB1_47:                               #   in Loop: Header=BB1_48 Depth=1
	ld 3, 184(1)                            # 8-byte Folded Reload
	ld 10, 80(1)                            # 8-byte Folded Reload
	ld 9, 88(1)                             # 8-byte Folded Reload
	ld 8, 96(1)                             # 8-byte Folded Reload
	addi 3, 3, 1024
	ld 7, 104(1)                            # 8-byte Folded Reload
	ld 20, 72(1)                            # 8-byte Folded Reload
	addi 10, 10, -256
	addi 9, 9, 2048
	addi 8, 8, 2048
	cmpld	20, 7
	std 3, 184(1)                           # 8-byte Folded Spill
	ld 3, 176(1)                            # 8-byte Folded Reload
	addi 3, 3, 1024
	std 3, 176(1)                           # 8-byte Folded Spill
	ld 3, 168(1)                            # 8-byte Folded Reload
	addi 3, 3, 1024
	std 3, 168(1)                           # 8-byte Folded Spill
	ld 3, 192(1)                            # 8-byte Folded Reload
	addi 3, 3, 1024
	std 3, 192(1)                           # 8-byte Folded Spill
	ld 3, 136(1)                            # 8-byte Folded Reload
	addi 3, 3, 1024
	std 3, 136(1)                           # 8-byte Folded Spill
	ld 3, 144(1)                            # 8-byte Folded Reload
	addi 3, 3, 1024
	std 3, 144(1)                           # 8-byte Folded Spill
	ld 3, 152(1)                            # 8-byte Folded Reload
	addi 3, 3, 1024
	std 3, 152(1)                           # 8-byte Folded Spill
	ld 3, 160(1)                            # 8-byte Folded Reload
	addi 3, 3, 1024
	std 3, 160(1)                           # 8-byte Folded Spill
	ld 3, 200(1)                            # 8-byte Folded Reload
	addi 3, 3, 1024
	std 3, 200(1)                           # 8-byte Folded Spill
	li 3, 0
	bge 0, .Lp9_fma_BB1_10
.Lp9_fma_BB1_48:                               # =>This Loop Header: Depth=1
                                        #     Child Loop BB1_50 Depth 2
                                        #       Child Loop BB1_69 Depth 3
                                        #         Child Loop BB1_72 Depth 4
                                        #         Child Loop BB1_76 Depth 4
                                        #         Child Loop BB1_79 Depth 4
                                        #       Child Loop BB1_52 Depth 3
                                        #       Child Loop BB1_56 Depth 3
                                        #         Child Loop BB1_59 Depth 4
                                        #         Child Loop BB1_63 Depth 4
                                        #         Child Loop BB1_66 Depth 4
	cmpldi	10, 256
	li 3, 256
	ld 4, 64(1)                             # 8-byte Folded Reload
	sub	6, 7, 20
	cmpldi 5, 10, 2
	addi 11, 20, 256
	std 11, 72(1)                           # 8-byte Folded Spill
	isellt	3, 10, 3
	cmpld	7, 20
	cmpldi 1, 6, 256
	cmpldi 6, 10, 16
	mr	22, 8
	mr	21, 17
	cmpldi 4, 10, 24
	mulhdu 4, 3, 4
	rlwinm 7, 3, 0, 23, 27
	crandc 21, 4, 1
	add 23, 3, 20
	addi 6, 7, -16
	cmpd 7, 3, 7
	std 7, 304(1)                           # 8-byte Folded Spill
	add 7, 20, 7
	std 7, 296(1)                           # 8-byte Folded Spill
	ld 7, 56(1)                             # 8-byte Folded Reload
	rldicl 6, 6, 60, 4
	mulli 30, 4, 24
	addi 6, 6, 1
	std 6, 312(1)                           # 8-byte Folded Spill
	addi 4, 30, -24
	add 26, 20, 30
	mulhdu 4, 4, 7
	rldicl 4, 4, 60, 4
	addi 27, 4, 1
	andi. 4, 3, 14
	mfocrf 4, 128
	sub.	6, 3, 30
	stw 4, 292(1)
	rlwinm 4, 3, 0, 23, 30
	cmpldi 3, 6, 1
	std 8, 96(1)                            # 8-byte Folded Spill
	std 9, 88(1)                            # 8-byte Folded Spill
	std 9, 320(1)                           # 8-byte Folded Spill
	std 10, 80(1)                           # 8-byte Folded Spill
	std 30, 240(1)                          # 8-byte Folded Spill
	std 27, 232(1)                          # 8-byte Folded Spill
	std 26, 224(1)                          # 8-byte Folded Spill
	add 24, 20, 4
	cmpd 2, 3, 4
	addi 4, 4, -2
	std 4, 336(1)                           # 8-byte Folded Spill
	li 4, 0
	b .Lp9_fma_BB1_50
.Lp9_fma_BB1_49:                               #   in Loop: Header=BB1_50 Depth=2
	ld 3, 320(1)                            # 8-byte Folded Reload
	ld 4, 272(1)                            # 8-byte Folded Reload
	ld 17, 264(1)                           # 8-byte Folded Reload
	add 21, 21, 19
	add 22, 22, 19
	add 3, 3, 19
	addi 4, 4, 1
	std 3, 320(1)                           # 8-byte Folded Spill
	ld 3, 208(1)                            # 8-byte Folded Reload
	cmpld 1, 4, 3
	beq	1, .Lp9_fma_BB1_47
.Lp9_fma_BB1_50:                               #   Parent Loop BB1_48 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB1_69 Depth 3
                                        #         Child Loop BB1_72 Depth 4
                                        #         Child Loop BB1_76 Depth 4
                                        #         Child Loop BB1_79 Depth 4
                                        #       Child Loop BB1_52 Depth 3
                                        #       Child Loop BB1_56 Depth 3
                                        #         Child Loop BB1_59 Depth 4
                                        #         Child Loop BB1_63 Depth 4
                                        #         Child Loop BB1_66 Depth 4
	maddld 3, 19, 4, 17
	std 4, 272(1)                           # 8-byte Folded Spill
	addi 29, 3, -8
	ld 3, 216(1)                            # 8-byte Folded Reload
	std 21, 328(1)                          # 8-byte Folded Spill
	std 22, 280(1)                          # 8-byte Folded Spill
	mulld 3, 4, 3
	ld 4, 256(1)                            # 8-byte Folded Reload
	sldi 3, 3, 2
	add 11, 4, 3
	bc 4, 21, .Lp9_fma_BB1_67
# %bb.51:                               #   in Loop: Header=BB1_50 Depth=2
	li 3, 0
	.p2align	5
.Lp9_fma_BB1_52:                               #   Parent Loop BB1_48 Depth=1
                                        #     Parent Loop BB1_50 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	addi 17, 3, 4
	addi 3, 3, 8
	cmpld 1, 3, 5
	mr	3, 17
	ble 1, .Lp9_fma_BB1_52
.Lp9_fma_BB1_53:                               #   in Loop: Header=BB1_50 Depth=2
	cmpld 1, 17, 5
	ld 19, 248(1)                           # 8-byte Folded Reload
	ld 30, 240(1)                           # 8-byte Folded Reload
	ld 27, 232(1)                           # 8-byte Folded Reload
	ld 26, 224(1)                           # 8-byte Folded Reload
	crorc 22, 21, 4
	ld 22, 280(1)                           # 8-byte Folded Reload
	ld 21, 328(1)                           # 8-byte Folded Reload
	bc 12, 22, .Lp9_fma_BB1_49
# %bb.54:                               #   in Loop: Header=BB1_50 Depth=2
	ld 3, 200(1)                            # 8-byte Folded Reload
	mulld 6, 25, 17
	maddld 4, 25, 17, 28
	li 7, 0
	addi 6, 6, -4
	maddld 3, 25, 17, 3
	b .Lp9_fma_BB1_56
	.p2align	4
.Lp9_fma_BB1_55:                               #   in Loop: Header=BB1_56 Depth=3
	addi 17, 17, 1
	add 3, 3, 25
	add 4, 4, 25
	addi 7, 7, 1
	cmpld 1, 17, 5
	beq	1, .Lp9_fma_BB1_49
.Lp9_fma_BB1_56:                               #   Parent Loop BB1_48 Depth=1
                                        #     Parent Loop BB1_50 Depth=2
                                        # =>    This Loop Header: Depth=3
                                        #         Child Loop BB1_59 Depth 4
                                        #         Child Loop BB1_63 Depth 4
                                        #         Child Loop BB1_66 Depth 4
	sldi 8, 17, 2
	mr	0, 20
	lfsx 0, 11, 8
	bc 12, 20, .Lp9_fma_BB1_65
# %bb.57:                               #   in Loop: Header=BB1_56 Depth=3
	li 10, 0
	bc 12, 16, .Lp9_fma_BB1_62
# %bb.58:                               #   in Loop: Header=BB1_56 Depth=3
	mtctr 27
	xxspltd	1, 0, 0
	mr	8, 3
	mr	9, 22
	.p2align	5
.Lp9_fma_BB1_59:                               #   Parent Loop BB1_48 Depth=1
                                        #     Parent Loop BB1_50 Depth=2
                                        #       Parent Loop BB1_56 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfd 2, -48(8)
	lfd 3, -40(8)
	lxv 34, -96(9)
	lxv 35, -80(9)
	lxv 36, -64(9)
	lxv 37, -48(9)
	lxv 32, -32(9)
	lxv 33, -16(9)
	lxv 38, 0(9)
	xxmrghw 2, 2, 2
	lfd 4, -32(8)
	lfd 5, -24(8)
	lfd 6, -16(8)
	lfd 7, -8(8)
	lfd 8, 0(8)
	lfd 9, 8(8)
	xxmrghw 3, 3, 3
	xxmrghw 4, 4, 4
	xxmrghw 5, 5, 5
	xxmrghw 6, 6, 6
	xxmrghw 7, 7, 7
	xxmrghw 8, 8, 8
	xxmrghw 9, 9, 9
	lxv 39, 16(9)
	lxv 40, 32(9)
	lxv 41, 48(9)
	lxv 42, 64(9)
	lxv 43, 80(9)
	xvcvspdp 2, 2
	lfd 10, 16(8)
	xxmrghw 10, 10, 10
	xvcvspdp 3, 3
	xvcvspdp 4, 4
	xvcvspdp 5, 5
	xvcvspdp 6, 6
	xvcvspdp 7, 7
	xvcvspdp 8, 8
	xvcvspdp 9, 9
	xvcvspdp 10, 10
	xvmaddadp 34, 1, 2
	lfd 11, 24(8)
	lfd 12, 32(8)
	lfd 13, 40(8)
	xxmrghw 11, 11, 11
	xxmrghw 12, 12, 12
	xxmrghw 13, 13, 13
	xvcvspdp 11, 11
	xvcvspdp 12, 12
	xvcvspdp 13, 13
	xvmaddadp 35, 1, 3
	xvmaddadp 36, 1, 4
	xvmaddadp 37, 1, 5
	xvmaddadp 32, 1, 6
	xvmaddadp 33, 1, 7
	addi 8, 8, 96
	xvmaddadp 38, 1, 8
	xvmaddadp 39, 1, 9
	xvmaddadp 40, 1, 10
	stxv 34, -96(9)
	xvmaddadp 41, 1, 11
	xvmaddadp 42, 1, 12
	xvmaddadp 43, 1, 13
	stxv 35, -80(9)
	stxv 36, -64(9)
	stxv 37, -48(9)
	stxv 32, -32(9)
	stxv 33, -16(9)
	stxv 38, 0(9)
	stxv 39, 16(9)
	stxv 40, 32(9)
	stxv 41, 48(9)
	stxv 42, 64(9)
	stxv 43, 80(9)
	addi 9, 9, 192
	bdnz .Lp9_fma_BB1_59
# %bb.60:                               #   in Loop: Header=BB1_56 Depth=3
	beq	0, .Lp9_fma_BB1_55
# %bb.61:                               #   in Loop: Header=BB1_56 Depth=3
	mr	10, 30
	mr	0, 26
	beq	3, .Lp9_fma_BB1_65
.Lp9_fma_BB1_62:                               #   in Loop: Header=BB1_56 Depth=3
	ld 0, 336(1)                            # 8-byte Folded Reload
	add 9, 20, 10
	xxspltd	1, 0, 0
	sldi 8, 9, 2
	sldi 9, 9, 3
	sub	10, 0, 10
	add 8, 4, 8
	add 9, 21, 9
	rldicl 10, 10, 63, 1
	addi 10, 10, 1
	mtctr 10
	.p2align	5
.Lp9_fma_BB1_63:                               #   Parent Loop BB1_48 Depth=1
                                        #     Parent Loop BB1_50 Depth=2
                                        #       Parent Loop BB1_56 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfd 2, 0(8)
	lxv 3, 0(9)
	addi 8, 8, 8
	xxmrghw 2, 2, 2
	xvcvspdp 2, 2
	xvmaddadp 3, 1, 2
	stxv 3, 0(9)
	addi 9, 9, 16
	bdnz .Lp9_fma_BB1_63
# %bb.64:                               #   in Loop: Header=BB1_56 Depth=3
	mr	0, 24
	bc 12, 10, .Lp9_fma_BB1_55
.Lp9_fma_BB1_65:                               #   in Loop: Header=BB1_56 Depth=3
	maddld 8, 25, 7, 6
	sldi 10, 0, 2
	add 8, 28, 8
	sldi 9, 0, 3
	add 9, 29, 9
	add 8, 8, 10
	sub	10, 23, 0
	mtctr 10
	.p2align	5
.Lp9_fma_BB1_66:                               #   Parent Loop BB1_48 Depth=1
                                        #     Parent Loop BB1_50 Depth=2
                                        #       Parent Loop BB1_56 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfsu 2, 4(8)
	lfdu 1, 8(9)
	xsmaddadp 1, 0, 2
	stfd 1, 0(9)
	bdnz .Lp9_fma_BB1_66
	b .Lp9_fma_BB1_55
.Lp9_fma_BB1_67:                               #   in Loop: Header=BB1_50 Depth=2
	ld 31, 128(1)                           # 8-byte Folded Reload
	ld 21, 120(1)                           # 8-byte Folded Reload
	ld 4, 112(1)                            # 8-byte Folded Reload
	ld 8, 160(1)                            # 8-byte Folded Reload
	li 30, 4
	li 16, 0
	mr	27, 28
	ld 19, 152(1)                           # 8-byte Folded Reload
	ld 3, 144(1)                            # 8-byte Folded Reload
	ld 6, 136(1)                            # 8-byte Folded Reload
	ld 0, 192(1)                            # 8-byte Folded Reload
	li 17, 0
	ld 9, 168(1)                            # 8-byte Folded Reload
	ld 10, 176(1)                           # 8-byte Folded Reload
	ld 7, 184(1)                            # 8-byte Folded Reload
	b .Lp9_fma_BB1_69
	.p2align	4
.Lp9_fma_BB1_68:                               #   in Loop: Header=BB1_69 Depth=3
	addi 30, 17, 4
	add 7, 7, 12
	add 10, 10, 12
	add 9, 9, 12
	add 0, 0, 12
	add 6, 6, 12
	add 3, 3, 12
	add 19, 19, 12
	add 8, 8, 12
	add 4, 4, 12
	add 21, 21, 12
	add 31, 31, 12
	add 27, 27, 12
	addi 16, 16, 1
	cmpld 1, 30, 5
	bgt	1, .Lp9_fma_BB1_53
.Lp9_fma_BB1_69:                               #   Parent Loop BB1_48 Depth=1
                                        #     Parent Loop BB1_50 Depth=2
                                        # =>    This Loop Header: Depth=3
                                        #         Child Loop BB1_72 Depth 4
                                        #         Child Loop BB1_76 Depth 4
                                        #         Child Loop BB1_79 Depth 4
	sldi 22, 17, 2
	mr	18, 11
	mr	17, 30
	mr	14, 20
	lfsux 0, 18, 22
	lfs 1, 4(18)
	lfs 2, 8(18)
	lfs 3, 12(18)
	bc 12, 20, .Lp9_fma_BB1_78
# %bb.70:                               #   in Loop: Header=BB1_69 Depth=3
	li 18, 0
	bc 12, 24, .Lp9_fma_BB1_75
# %bb.71:                               #   in Loop: Header=BB1_69 Depth=3
	mr	26, 24
	ld 24, 312(1)                           # 8-byte Folded Reload
	ld 30, 320(1)                           # 8-byte Folded Reload
	xxspltd	4, 0, 0
	xxspltd	5, 1, 0
	mr	22, 0
	mr	15, 9
	mr	14, 10
	mr	18, 7
	xxspltd	6, 2, 0
	xxspltd	7, 3, 0
	mtctr 24
	.p2align	5
.Lp9_fma_BB1_72:                               #   Parent Loop BB1_48 Depth=1
                                        #     Parent Loop BB1_50 Depth=2
                                        #       Parent Loop BB1_69 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfd 11, -24(22)
	lxsd 6, 0(22)
	lxsd 7, 8(22)
	lxsd 8, 16(22)
	lxv 9, -48(30)
	xxmrghw 34, 11, 11
	lfd 8, -32(22)
	xxmrghw 10, 8, 8
	lxv 8, -64(30)
	lxv 11, -16(30)
	lxv 12, 0(30)
	lxv 13, 16(30)
	lxv 35, 32(30)
	lxsd 9, 24(22)
	lxsd 10, 24(15)
	xxmrghw 38, 38, 38
	xxmrghw 39, 39, 39
	xxmrghw 40, 40, 40
	xxmrghw 41, 41, 41
	xxmrghw 42, 42, 42
	xvcvspdp 37, 34
	lxsd 2, -16(22)
	xxmrghw 32, 34, 34
	lxsd 2, -8(22)
	xvcvspdp 36, 10
	xvcvspdp 38, 38
	xvcvspdp 39, 39
	lxv 10, -32(30)
	xvcvspdp 40, 40
	xvcvspdp 41, 41
	addi 22, 22, 64
	xvmaddadp 8, 4, 36
	lxsd 4, -32(15)
	xvmaddadp 9, 4, 37
	lxsd 5, -24(15)
	xvcvspdp 32, 32
	xvmaddadp 12, 4, 38
	lxsd 6, 0(15)
	xvmaddadp 13, 4, 39
	lxsd 7, 8(15)
	xvmaddadp 35, 4, 40
	lxsd 8, 16(15)
	xxmrghw 33, 34, 34
	lxv 34, 48(30)
	xxmrghw 36, 36, 36
	xxmrghw 37, 37, 37
	xvcvspdp 33, 33
	xxmrghw 38, 38, 38
	xxmrghw 39, 39, 39
	xxmrghw 40, 40, 40
	xvmaddadp 10, 4, 32
	lxsd 0, -16(15)
	xvcvspdp 36, 36
	xvcvspdp 37, 37
	xxmrghw 32, 32, 32
	xvmaddadp 11, 4, 33
	lxsd 1, -8(15)
	xvcvspdp 38, 38
	xvcvspdp 39, 39
	addi 15, 15, 64
	xvcvspdp 40, 40
	xvmaddadp 34, 4, 41
	xvcvspdp 41, 42
	lxsd 10, 24(14)
	xxmrghw 42, 42, 42
	xvmaddadp 8, 5, 36
	xvmaddadp 9, 5, 37
	lxsd 4, -32(14)
	lxsd 5, -24(14)
	xvmaddadp 34, 5, 41
	xvcvspdp 32, 32
	xvmaddadp 12, 5, 38
	lxsd 6, 0(14)
	xvmaddadp 13, 5, 39
	lxsd 7, 8(14)
	xvmaddadp 35, 5, 40
	lxsd 8, 16(14)
	xvcvspdp 41, 42
	lxsd 10, 24(18)
	xxmrghw 33, 33, 33
	xxmrghw 36, 36, 36
	xxmrghw 37, 37, 37
	xvcvspdp 33, 33
	xxmrghw 38, 38, 38
	xxmrghw 39, 39, 39
	xxmrghw 40, 40, 40
	xxmrghw 42, 42, 42
	xvmaddadp 10, 5, 32
	lxsd 0, -16(14)
	xvcvspdp 36, 36
	xvcvspdp 37, 37
	xxmrghw 32, 32, 32
	xvmaddadp 11, 5, 33
	lxsd 1, -8(14)
	xvcvspdp 38, 38
	xvcvspdp 39, 39
	addi 14, 14, 64
	xvcvspdp 40, 40
	xvmaddadp 34, 6, 41
	xvcvspdp 41, 42
	xvmaddadp 8, 6, 36
	xvmaddadp 9, 6, 37
	lxsd 4, -32(18)
	lxsd 5, -24(18)
	xvcvspdp 32, 32
	xvmaddadp 12, 6, 38
	lxsd 6, 0(18)
	xvmaddadp 13, 6, 39
	lxsd 7, 8(18)
	xvmaddadp 35, 6, 40
	lxsd 8, 16(18)
	xxmrghw 33, 33, 33
	xvmaddadp 34, 7, 41
	xxmrghw 36, 36, 36
	xxmrghw 37, 37, 37
	xvcvspdp 33, 33
	xxmrghw 38, 38, 38
	xxmrghw 39, 39, 39
	xxmrghw 40, 40, 40
	xvmaddadp 10, 6, 32
	lxsd 0, -16(18)
	xvcvspdp 36, 36
	xvcvspdp 37, 37
	stxv 34, 48(30)
	xvcvspdp 38, 38
	xvcvspdp 39, 39
	xvcvspdp 40, 40
	xxmrghw 32, 32, 32
	xvmaddadp 11, 6, 33
	lxsd 1, -8(18)
	addi 18, 18, 64
	xvmaddadp 8, 7, 36
	xvmaddadp 9, 7, 37
	xvcvspdp 32, 32
	xvmaddadp 12, 7, 38
	xvmaddadp 13, 7, 39
	xvmaddadp 35, 7, 40
	xxmrghw 33, 33, 33
	xvcvspdp 33, 33
	stxv 8, -64(30)
	stxv 9, -48(30)
	xvmaddadp 10, 7, 32
	stxv 12, 0(30)
	stxv 13, 16(30)
	stxv 35, 32(30)
	xvmaddadp 11, 7, 33
	stxv 10, -32(30)
	stxv 11, -16(30)
	addi 30, 30, 128
	bdnz .Lp9_fma_BB1_72
# %bb.73:                               #   in Loop: Header=BB1_69 Depth=3
	mr	24, 26
	bc 12, 30, .Lp9_fma_BB1_68
# %bb.74:                               #   in Loop: Header=BB1_69 Depth=3
	ld 18, 304(1)                           # 8-byte Folded Reload
	ld 14, 296(1)                           # 8-byte Folded Reload
	lwz 30, 292(1)
	rotlwi	30, 30, 28
	mtocrf 64, 30
	bc 12, 6, .Lp9_fma_BB1_78
.Lp9_fma_BB1_75:                               #   in Loop: Header=BB1_69 Depth=3
	add 30, 20, 18
	ld 26, 328(1)                           # 8-byte Folded Reload
	sldi 22, 18, 2
	xxspltd	4, 0, 0
	xxspltd	5, 1, 0
	sldi 30, 30, 3
	xxspltd	6, 2, 0
	xxspltd	7, 3, 0
	add 30, 26, 30
	ld 26, 336(1)                           # 8-byte Folded Reload
	sub	18, 26, 18
	rldicl 18, 18, 63, 1
	addi 18, 18, 1
	mtctr 18
	.p2align	5
.Lp9_fma_BB1_76:                               #   Parent Loop BB1_48 Depth=1
                                        #     Parent Loop BB1_50 Depth=2
                                        #       Parent Loop BB1_69 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfdx 9, 6, 22
	lxv 8, 0(30)
	xxmrghw 9, 9, 9
	xvcvspdp 9, 9
	xvmaddadp 8, 4, 9
	lfdx 9, 3, 22
	xxmrghw 9, 9, 9
	xvcvspdp 9, 9
	xvmaddadp 8, 5, 9
	lfdx 9, 19, 22
	xxmrghw 9, 9, 9
	xvcvspdp 9, 9
	xvmaddadp 8, 6, 9
	lfdx 9, 8, 22
	addi 22, 22, 8
	xxmrghw 9, 9, 9
	xvcvspdp 9, 9
	xvmaddadp 8, 7, 9
	stxv 8, 0(30)
	addi 30, 30, 16
	bdnz .Lp9_fma_BB1_76
# %bb.77:                               #   in Loop: Header=BB1_69 Depth=3
	mr	14, 24
	bc 12, 10, .Lp9_fma_BB1_68
.Lp9_fma_BB1_78:                               #   in Loop: Header=BB1_69 Depth=3
	ld 30, 344(1)                           # 8-byte Folded Reload
	sldi 22, 14, 3
	add 22, 29, 22
	maddld 30, 12, 16, 30
	add 15, 28, 30
	maddld 30, 12, 16, 28
	addi 18, 30, -4
	sldi 30, 14, 2
	sub	14, 23, 14
	mtctr 14
	add 18, 18, 30
	add 15, 15, 30
	.p2align	5
.Lp9_fma_BB1_79:                               #   Parent Loop BB1_48 Depth=1
                                        #     Parent Loop BB1_50 Depth=2
                                        #       Parent Loop BB1_69 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfsu 5, 4(18)
	lfdu 6, 8(22)
	xsmaddadp 6, 0, 5
	lfsu 4, 4(15)
	xsmaddadp 6, 1, 4
	lfsx 4, 21, 30
	xsmaddadp 6, 2, 4
	lfsx 4, 4, 30
	addi 30, 30, 4
	xsmaddadp 6, 3, 4
	stfd 6, 0(22)
	bdnz .Lp9_fma_BB1_79
	b .Lp9_fma_BB1_68
	.long	0
	.quad	0
.Lp9_fma_func_end1:
	.size	pq_accumulate_transposed_rows_power9_fma, .Lp9_fma_func_end1-.Lp9_fma_func_begin1
	.cfi_endproc
                                        # -- End function
