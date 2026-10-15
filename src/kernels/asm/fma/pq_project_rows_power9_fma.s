# fma/pq_project_rows_power9_fma.s: kernel.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_fma.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.text.pq_project_rows_power9_fma,"ax",@progbits
	.globl	pq_project_rows_power9_fma    # -- Begin function pq_project_rows_power9_fma
	.p2align 6
	.type	pq_project_rows_power9_fma,@function
pq_project_rows_power9_fma:           # @pq_project_rows_power9_fma
.Lp9_fma_func_begin0:
	.cfi_startproc
.Lp9_fma_func_gep0:
	addis 2, 12, .TOC.-.Lp9_fma_func_gep0@ha
	addi 2, 2, .TOC.-.Lp9_fma_func_gep0@l
.Lp9_fma_func_lep0:
	.localentry	pq_project_rows_power9_fma, .Lp9_fma_func_lep0-.Lp9_fma_func_gep0
# %bb.0:
	mfcr 12
	mflr 0
	stw 12, 8(1)
	stdu 1, -9888(1)
	std 0, 9904(1)
	.cfi_def_cfa_offset 9888
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
	std 27, 9848(1)                         # 8-byte Folded Spill
	ld 27, 10000(1)
	mr	11, 3
	cmpdi	4, 0
	li 3, 1
	std 14, 9744(1)                         # 8-byte Folded Spill
	std 15, 9752(1)                         # 8-byte Folded Spill
	std 16, 9760(1)                         # 8-byte Folded Spill
	std 17, 9768(1)                         # 8-byte Folded Spill
	std 18, 9776(1)                         # 8-byte Folded Spill
	std 19, 9784(1)                         # 8-byte Folded Spill
	std 20, 9792(1)                         # 8-byte Folded Spill
	std 21, 9800(1)                         # 8-byte Folded Spill
	std 22, 9808(1)                         # 8-byte Folded Spill
	std 23, 9816(1)                         # 8-byte Folded Spill
	std 24, 9824(1)                         # 8-byte Folded Spill
	std 25, 9832(1)                         # 8-byte Folded Spill
	std 26, 9840(1)                         # 8-byte Folded Spill
	std 28, 9856(1)                         # 8-byte Folded Spill
	std 29, 9864(1)                         # 8-byte Folded Spill
	std 30, 9872(1)                         # 8-byte Folded Spill
	std 31, 9880(1)                         # 8-byte Folded Spill
	bc 12, 2, .Lp9_fma_BB0_10
# %bb.1:
	cmpdi	8, 0
	bc 12, 2, .Lp9_fma_BB0_10
# %bb.2:
	cmpdi	10, 0
	bc 12, 2, .Lp9_fma_BB0_10
# %bb.3:
	cmpdi	27, 0
	bc 12, 2, .Lp9_fma_BB0_10
# %bb.4:
	cmpdi	6, 0
	bc 12, 2, .Lp9_fma_BB0_10
# %bb.5:
	cmpdi	7, 0
	bc 12, 2, .Lp9_fma_BB0_10
# %bb.6:
	cmpdi	9, 0
	bc 12, 2, .Lp9_fma_BB0_10
# %bb.7:
	ld 30, 9992(1)
	not	12, 6
	cmpld	30, 12
	ble 0, .Lp9_fma_BB0_9
.Lp9_fma_BB0_8:
	li 3, 4
	b .Lp9_fma_BB0_10
.Lp9_fma_BB0_9:
	ld 22, 9984(1)
	add 12, 30, 6
	cmpld	12, 22
	ble 0, .Lp9_fma_BB0_11
.Lp9_fma_BB0_10:
	ld 31, 9880(1)                          # 8-byte Folded Reload
	ld 30, 9872(1)                          # 8-byte Folded Reload
	ld 29, 9864(1)                          # 8-byte Folded Reload
	ld 28, 9856(1)                          # 8-byte Folded Reload
	ld 27, 9848(1)                          # 8-byte Folded Reload
	ld 26, 9840(1)                          # 8-byte Folded Reload
	ld 25, 9832(1)                          # 8-byte Folded Reload
	ld 24, 9824(1)                          # 8-byte Folded Reload
	ld 23, 9816(1)                          # 8-byte Folded Reload
	ld 22, 9808(1)                          # 8-byte Folded Reload
	ld 21, 9800(1)                          # 8-byte Folded Reload
	ld 20, 9792(1)                          # 8-byte Folded Reload
	ld 19, 9784(1)                          # 8-byte Folded Reload
	ld 18, 9776(1)                          # 8-byte Folded Reload
	ld 17, 9768(1)                          # 8-byte Folded Reload
	ld 16, 9760(1)                          # 8-byte Folded Reload
	ld 15, 9752(1)                          # 8-byte Folded Reload
	ld 14, 9744(1)                          # 8-byte Folded Reload
	addi 1, 1, 9888
	ld 0, 16(1)
	lwz 12, 8(1)
	mtlr 0
	mtocrf 32, 12
	mtocrf 16, 12
	mtocrf 8, 12
	blr
.Lp9_fma_BB0_11:
	mulhdu. 3, 9, 22
	bne	0, .Lp9_fma_BB0_8
# %bb.12:
	mulhdu. 3, 9, 7
	bne	0, .Lp9_fma_BB0_8
# %bb.13:
	li 3, 2
	cmplwi	11, 35
	bgt	0, .Lp9_fma_BB0_10
# %bb.14:
	clrldi	12, 11, 32
	li 0, 1
	sld 12, 0, 12
	rldic 0, 0, 35, 28
	ori 0, 0, 20807
	and. 12, 12, 0
	beq	0, .Lp9_fma_BB0_10
# %bb.15:
	mr	25, 4
	mr	3, 11
	mr	4, 7
	mr	28, 5
	std 8, 888(1)                           # 8-byte Folded Spill
	std 6, 512(1)                           # 8-byte Folded Spill
	mr	26, 11
	std 9, 264(1)                           # 8-byte Folded Spill
	std 10, 504(1)                          # 8-byte Folded Spill
	std 7, 904(1)                           # 8-byte Folded Spill
	bl pq_row_bytes
	nop
	cmpldi	3, 0
	beq	0, .Lp9_fma_BB0_27
# %bb.16:
	ld 24, 512(1)                           # 8-byte Folded Reload
	mr	29, 3
	mulhdu. 3, 24, 3
	bne	0, .Lp9_fma_BB0_8
# %bb.17:
	ld 7, 904(1)                            # 8-byte Folded Reload
	mulhdu. 3, 24, 7
	li 3, 4
	bne	0, .Lp9_fma_BB0_10
# %bb.18:
	mulld 3, 29, 24
	cmpld	3, 28
	bne	0, .Lp9_fma_BB0_27
# %bb.19:
	mulld 8, 7, 24
	ld 6, 10008(1)
	li 3, 5
	cmpld	6, 8
	blt	0, .Lp9_fma_BB0_10
# %bb.20:
	mr	4, 25
	mr	5, 26
	sldi 25, 7, 2
	mr	28, 27
.Lp9_fma_BB0_21:                               # =>This Inner Loop Header: Depth=1
	mr	3, 5
	mr	5, 29
	mr	6, 28
	mr	23, 4
	bl p9_decode_row_vsx
	nop
	cmpldi	3, 0
	beq	0, .Lp9_fma_BB0_27
# %bb.22:                               #   in Loop: Header=BB0_21 Depth=1
	ld 4, 904(1)                            # 8-byte Folded Reload
	mr	3, 28
	bl p9_all_finite_vsx
	nop
	cmplwi	3, 0
	beq	0, .Lp9_fma_BB0_38
# %bb.23:                               #   in Loop: Header=BB0_21 Depth=1
	addi 24, 24, -1
	ld 7, 904(1)                            # 8-byte Folded Reload
	add 4, 23, 29
	add 28, 28, 25
	mr	5, 26
	cmpldi	24, 0
	bc 12, 1, .Lp9_fma_BB0_21
# %bb.24:
	ld 14, 504(1)                           # 8-byte Folded Reload
	sldi 3, 30, 2
	ld 6, 264(1)                            # 8-byte Folded Reload
	ld 4, 512(1)                            # 8-byte Folded Reload
	li 8, 0
	cmpldi	6, 4
	std 3, 40(1)                            # 8-byte Folded Spill
	add 3, 14, 3
	std 22, 256(1)                          # 8-byte Folded Spill
	std 3, 336(1)                           # 8-byte Folded Spill
	sldi 3, 22, 2
	std 3, 296(1)                           # 8-byte Folded Spill
	bge 0, .Lp9_fma_BB0_45
.Lp9_fma_BB0_25:
	ld 3, 888(1)                            # 8-byte Folded Reload
	cmpld	8, 6
	bge 0, .Lp9_fma_BB0_39
# %bb.26:
	ld 9, 904(1)                            # 8-byte Folded Reload
	mr	24, 8
	addi 8, 9, -1
	andi. 7, 9, 7
	rldicr 30, 9, 0, 60
	mulld 9, 24, 9
	addi 0, 30, -8
	cmpldi 1, 8, 7
	addi 8, 27, 16
	rldicl 30, 0, 61, 3
	sldi 12, 9, 2
	addi 0, 30, 1
	li 30, 0
	add 9, 3, 12
	addi 11, 12, -32
	addi 12, 12, -4
	addi 10, 9, 16
	b .Lp9_fma_BB0_29
.Lp9_fma_BB0_27:
	li 3, 3
	b .Lp9_fma_BB0_10
.Lp9_fma_BB0_28:                               #   in Loop: Header=BB0_29 Depth=1
	addi 24, 17, 1
	add 10, 10, 25
	add 9, 9, 25
	addi 30, 30, 1
	cmpld 5, 24, 6
	beq	5, .Lp9_fma_BB0_39
.Lp9_fma_BB0_29:                               # =>This Loop Header: Depth=1
                                        #     Child Loop BB0_31 Depth 2
                                        #       Child Loop BB0_34 Depth 3
                                        #       Child Loop BB0_37 Depth 3
	ld 26, 256(1)                           # 8-byte Folded Reload
	maddld 29, 25, 30, 12
	maddld 28, 25, 30, 11
	mr	17, 24
	mr	23, 8
	li 22, 0
	add 29, 3, 29
	mulld 26, 24, 26
	ld 24, 336(1)                           # 8-byte Folded Reload
	add 28, 3, 28
	sldi 26, 26, 2
	add 26, 24, 26
	mr	24, 27
	b .Lp9_fma_BB0_31
.Lp9_fma_BB0_30:                               #   in Loop: Header=BB0_31 Depth=2
	xsrsp 0, 0
	sldi 21, 22, 2
	addi 22, 22, 1
	add 23, 23, 25
	add 24, 24, 25
	cmpld 5, 22, 4
	stfsx 0, 26, 21
	beq	5, .Lp9_fma_BB0_28
.Lp9_fma_BB0_31:                               #   Parent Loop BB0_29 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB0_34 Depth 3
                                        #       Child Loop BB0_37 Depth 3
	maddld 21, 25, 22, 27
	bc 4, 4, .Lp9_fma_BB0_33
# %bb.32:                               #   in Loop: Header=BB0_31 Depth=2
	xxlxor 0, 0, 0
	li 20, 0
	b .Lp9_fma_BB0_35
.Lp9_fma_BB0_33:                               #   in Loop: Header=BB0_31 Depth=2
	mtctr 0
	addi 19, 21, -32
	li 20, 0
	mr	18, 28
	xxlxor 0, 0, 0
	.p2align	5
.Lp9_fma_BB0_34:                               #   Parent Loop BB0_29 Depth=1
                                        #     Parent Loop BB0_31 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lfsu 1, 32(18)
	addi 20, 20, 8
	lfs 4, 4(18)
	lfsu 2, 32(19)
	xsmaddadp 0, 2, 1
	lfs 3, 4(19)
	lfs 1, 8(19)
	lfs 2, 8(18)
	xsmaddadp 0, 3, 4
	lfs 3, 12(19)
	lfs 4, 24(18)
	xsmaddadp 0, 1, 2
	lfs 1, 12(18)
	lfs 2, 16(18)
	xsmaddadp 0, 3, 1
	lfs 1, 16(19)
	xsmaddadp 0, 1, 2
	lfs 1, 20(19)
	lfs 2, 20(18)
	lfs 3, 24(19)
	xsmaddadp 0, 1, 2
	xsmaddadp 0, 3, 4
	lfs 1, 28(19)
	lfs 2, 28(18)
	xsmaddadp 0, 1, 2
	bdnz .Lp9_fma_BB0_34
.Lp9_fma_BB0_35:                               #   in Loop: Header=BB0_31 Depth=2
	bc 12, 2, .Lp9_fma_BB0_30
# %bb.36:                               #   in Loop: Header=BB0_31 Depth=2
	addi 21, 21, -4
	sldi 20, 20, 2
	mtctr 7
	add 21, 21, 20
	add 20, 29, 20
	.p2align	5
.Lp9_fma_BB0_37:                               #   Parent Loop BB0_29 Depth=1
                                        #     Parent Loop BB0_31 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lfsu 1, 4(20)
	lfsu 2, 4(21)
	xsmaddadp 0, 2, 1
	bdnz .Lp9_fma_BB0_37
	b .Lp9_fma_BB0_30
.Lp9_fma_BB0_38:
	li 3, 6
	b .Lp9_fma_BB0_10
.Lp9_fma_BB0_39:
	ld 3, 40(1)                             # 8-byte Folded Reload
	ld 9, 336(1)                            # 8-byte Folded Reload
	ld 10, 296(1)                           # 8-byte Folded Reload
	li 8, 0
	addi 7, 3, -4
.Lp9_fma_BB0_40:                               # =>This Loop Header: Depth=1
                                        #     Child Loop BB0_41 Depth 2
	maddld 3, 10, 8, 7
	mtctr 4
	add 3, 14, 3
.Lp9_fma_BB0_41:                               #   Parent Loop BB0_40 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lfsu 0, 4(3)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_fma_BB0_44
# %bb.42:                               #   in Loop: Header=BB0_41 Depth=2
	bdnz .Lp9_fma_BB0_41
# %bb.43:                               #   in Loop: Header=BB0_40 Depth=1
	addi 8, 8, 1
	add 9, 9, 10
	li 3, 0
	cmpld	8, 6
	bne	0, .Lp9_fma_BB0_40
	b .Lp9_fma_BB0_10
.Lp9_fma_BB0_44:
	li 3, 7
	b .Lp9_fma_BB0_10
.Lp9_fma_BB0_45:
	ld 24, 256(1)                           # 8-byte Folded Reload
	ld 6, 888(1)                            # 8-byte Folded Reload
	ld 23, 40(1)                            # 8-byte Folded Reload
	addi 21, 1, 1568
	addi 16, 1, 1616
	std 16, 752(1)                          # 8-byte Folded Spill
	rldicl 3, 24, 63, 1
	mulli 26, 24, 12
	sldi 22, 24, 3
	std 6, 472(1)                           # 8-byte Folded Spill
	rldicl. 3, 3, 1, 2
	ld 3, 904(1)                            # 8-byte Folded Reload
	cmpdi 1, 22, 0
	std 22, 280(1)                          # 8-byte Folded Spill
	cror 20, 2, 6
	cmpldi	26, 8
	std 26, 288(1)                          # 8-byte Folded Spill
	sldi 7, 3, 3
	mulli 8, 3, 12
	mulli 9, 3, 28
	mulli 10, 3, 24
	mulli 30, 3, 56
	mulli 29, 3, 52
	mulli 28, 3, 48
	cror 8, 20, 0
	add 11, 6, 7
	std 11, 496(1)                          # 8-byte Folded Spill
	add 11, 27, 7
	addi 7, 7, -4
	std 11, 192(1)                          # 8-byte Folded Spill
	add 11, 6, 8
	add 8, 27, 8
	std 7, 440(1)                           # 8-byte Folded Spill
	std 8, 184(1)                           # 8-byte Folded Spill
	add 8, 27, 9
	std 11, 488(1)                          # 8-byte Folded Spill
	std 8, 176(1)                           # 8-byte Folded Spill
	add 8, 27, 10
	std 8, 168(1)                           # 8-byte Folded Spill
	mulli 8, 3, 20
	add 9, 27, 8
	addi 7, 8, -4
	li 8, 0
	std 9, 160(1)                           # 8-byte Folded Spill
	mulli 9, 3, 44
	std 7, 432(1)                           # 8-byte Folded Spill
	addi 7, 10, -4
	li 10, 0
	std 7, 424(1)                           # 8-byte Folded Spill
	add 9, 27, 9
	std 9, 152(1)                           # 8-byte Folded Spill
	mulli 9, 3, 40
	add 11, 27, 9
	std 11, 144(1)                          # 8-byte Folded Spill
	mulli 11, 3, 36
	add 12, 27, 11
	std 12, 136(1)                          # 8-byte Folded Spill
	sldi 12, 3, 5
	add 0, 27, 12
	addi 7, 12, -4
	sldi 12, 3, 6
	std 0, 128(1)                           # 8-byte Folded Spill
	mulli 0, 3, 60
	std 7, 416(1)                           # 8-byte Folded Spill
	addi 7, 11, -4
	ld 11, 336(1)                           # 8-byte Folded Reload
	std 12, 344(1)                          # 8-byte Folded Spill
	std 7, 408(1)                           # 8-byte Folded Spill
	addi 7, 9, -4
	sldi 9, 3, 4
	sldi 3, 24, 4
	std 7, 400(1)                           # 8-byte Folded Spill
	addi 7, 28, -4
	std 3, 48(1)                            # 8-byte Folded Spill
	std 9, 72(1)                            # 8-byte Folded Spill
	add 0, 27, 0
	std 7, 392(1)                           # 8-byte Folded Spill
	addi 7, 29, -4
	std 0, 120(1)                           # 8-byte Folded Spill
	add 0, 27, 30
	std 7, 384(1)                           # 8-byte Folded Spill
	addi 7, 30, -4
	li 30, 0
	std 0, 112(1)                           # 8-byte Folded Spill
	add 0, 27, 29
	std 7, 376(1)                           # 8-byte Folded Spill
	std 0, 104(1)                           # 8-byte Folded Spill
	add 0, 27, 28
	std 0, 96(1)                            # 8-byte Folded Spill
	add 0, 26, 23
	add 20, 14, 0
	add 0, 22, 23
	add 22, 14, 0
	ld 0, 296(1)                            # 8-byte Folded Reload
	std 20, 328(1)                          # 8-byte Folded Spill
	std 22, 320(1)                          # 8-byte Folded Spill
	add 26, 23, 0
	addi 7, 26, -8
	add 19, 14, 26
	std 7, 88(1)                            # 8-byte Folded Spill
	addi 7, 4, -1
	std 19, 312(1)                          # 8-byte Folded Spill
	rldicl 7, 7, 60, 4
	std 7, 368(1)                           # 8-byte Folded Spill
	add 7, 6, 25
	std 7, 480(1)                           # 8-byte Folded Spill
	add 7, 27, 25
	std 7, 80(1)                            # 8-byte Folded Spill
	add 7, 27, 9
	std 7, 64(1)                            # 8-byte Folded Spill
	addi 7, 27, 8
	std 7, 360(1)                           # 8-byte Folded Spill
	addi 7, 1, 1520
	std 7, 864(1)                           # 8-byte Folded Spill
	addi 7, 25, -4
	std 7, 464(1)                           # 8-byte Folded Spill
	addi 7, 9, -4
	std 7, 352(1)                           # 8-byte Folded Spill
	addi 7, 1, 976
	std 7, 272(1)                           # 8-byte Folded Spill
	addi 7, 23, -8
	std 7, 56(1)                            # 8-byte Folded Spill
	li 7, 4
	b .Lp9_fma_BB0_47
.Lp9_fma_BB0_46:                               #   in Loop: Header=BB0_47 Depth=1
	ld 9, 72(1)                             # 8-byte Folded Reload
	ld 3, 472(1)                            # 8-byte Folded Reload
	ld 8, 248(1)                            # 8-byte Folded Reload
	ld 30, 200(1)                           # 8-byte Folded Reload
	add 3, 3, 9
	ld 11, 208(1)                           # 8-byte Folded Reload
	ld 19, 216(1)                           # 8-byte Folded Reload
	ld 22, 224(1)                           # 8-byte Folded Reload
	ld 20, 232(1)                           # 8-byte Folded Reload
	ld 10, 240(1)                           # 8-byte Folded Reload
	ld 6, 264(1)                            # 8-byte Folded Reload
	addi 7, 8, 4
	addi 10, 10, 1
	std 3, 472(1)                           # 8-byte Folded Spill
	ld 3, 488(1)                            # 8-byte Folded Reload
	cmpld	7, 6
	add 3, 3, 9
	std 3, 488(1)                           # 8-byte Folded Spill
	ld 3, 496(1)                            # 8-byte Folded Reload
	add 3, 3, 9
	std 3, 496(1)                           # 8-byte Folded Spill
	ld 3, 480(1)                            # 8-byte Folded Reload
	add 3, 3, 9
	std 3, 480(1)                           # 8-byte Folded Spill
	ld 3, 48(1)                             # 8-byte Folded Reload
	add 30, 30, 3
	add 11, 11, 3
	add 19, 19, 3
	add 22, 22, 3
	add 20, 20, 3
	bgt	0, .Lp9_fma_BB0_25
.Lp9_fma_BB0_47:                               # =>This Loop Header: Depth=1
                                        #     Child Loop BB0_49 Depth 2
                                        #       Child Loop BB0_51 Depth 3
                                        #         Child Loop BB0_53 Depth 4
                                        #         Child Loop BB0_56 Depth 4
                                        #         Child Loop BB0_59 Depth 4
                                        #         Child Loop BB0_62 Depth 4
                                        #         Child Loop BB0_65 Depth 4
                                        #         Child Loop BB0_68 Depth 4
                                        #         Child Loop BB0_73 Depth 4
                                        #           Child Loop BB0_76 Depth 5
                                        #           Child Loop BB0_79 Depth 5
                                        #       Child Loop BB0_82 Depth 3
                                        #       Child Loop BB0_88 Depth 3
	ld 6, 256(1)                            # 8-byte Folded Reload
	std 30, 200(1)                          # 8-byte Folded Spill
	mr	29, 30
	ld 18, 96(1)                            # 8-byte Folded Reload
	ld 17, 112(1)                           # 8-byte Folded Reload
	ld 23, 128(1)                           # 8-byte Folded Reload
	ld 15, 144(1)                           # 8-byte Folded Reload
	ld 0, 160(1)                            # 8-byte Folded Reload
	std 7, 248(1)                           # 8-byte Folded Spill
	ld 7, 152(1)                            # 8-byte Folded Reload
	ld 31, 176(1)                           # 8-byte Folded Reload
	ld 26, 192(1)                           # 8-byte Folded Reload
	mr	24, 4
	li 28, 0
	li 5, 0
	std 10, 240(1)                          # 8-byte Folded Spill
	std 20, 232(1)                          # 8-byte Folded Spill
	std 22, 224(1)                          # 8-byte Folded Spill
	std 19, 216(1)                          # 8-byte Folded Spill
	std 11, 208(1)                          # 8-byte Folded Spill
	mulld 3, 8, 6
	ld 8, 88(1)                             # 8-byte Folded Reload
	add 8, 8, 30
	mulld 6, 9, 10
	addi 6, 6, -4
	sldi 3, 3, 2
	std 8, 456(1)                           # 8-byte Folded Spill
	ld 8, 56(1)                             # 8-byte Folded Reload
	std 6, 880(1)                           # 8-byte Folded Spill
	ld 6, 336(1)                            # 8-byte Folded Reload
	add 8, 8, 30
	ld 30, 168(1)                           # 8-byte Folded Reload
	add 3, 6, 3
	ld 6, 136(1)                            # 8-byte Folded Reload
	std 8, 448(1)                           # 8-byte Folded Spill
	ld 8, 104(1)                            # 8-byte Folded Reload
	std 3, 304(1)                           # 8-byte Folded Spill
	ld 3, 464(1)                            # 8-byte Folded Reload
	std 8, 744(1)                           # 8-byte Folded Spill
	ld 8, 120(1)                            # 8-byte Folded Reload
	maddld 3, 9, 10, 3
	ld 10, 80(1)                            # 8-byte Folded Reload
	mr	9, 27
	std 8, 736(1)                           # 8-byte Folded Spill
	ld 8, 64(1)                             # 8-byte Folded Reload
	std 3, 872(1)                           # 8-byte Folded Spill
	ld 3, 184(1)                            # 8-byte Folded Reload
	std 3, 728(1)                           # 8-byte Folded Spill
	b .Lp9_fma_BB0_49
.Lp9_fma_BB0_48:                               #   in Loop: Header=BB0_49 Depth=2
	ld 3, 368(1)                            # 8-byte Folded Reload
	ld 7, 568(1)                            # 8-byte Folded Reload
	ld 10, 592(1)                           # 8-byte Folded Reload
	ld 9, 600(1)                            # 8-byte Folded Reload
	add 6, 23, 12
	add 4, 4, 12
	add 14, 14, 12
	ld 31, 608(1)                           # 8-byte Folded Reload
	ld 30, 720(1)                           # 8-byte Folded Reload
	ld 8, 624(1)                            # 8-byte Folded Reload
	ld 23, 656(1)                           # 8-byte Folded Reload
	add 28, 28, 12
	addi 5, 5, 16
	ld 11, 688(1)                           # 8-byte Folded Reload
	ld 19, 696(1)                           # 8-byte Folded Reload
	addi 24, 24, -16
	std 4, 736(1)                           # 8-byte Folded Spill
	ld 4, 512(1)                            # 8-byte Folded Reload
	addi 29, 29, 64
	add 26, 26, 12
	add 0, 0, 12
	add 15, 15, 12
	add 17, 17, 12
	add 18, 18, 12
	addi 22, 22, 64
	addi 20, 20, 64
	cmpld	7, 3
	addi 3, 7, 1
	ld 7, 632(1)                            # 8-byte Folded Reload
	add 10, 10, 12
	add 31, 31, 12
	addi 11, 11, 64
	add 9, 9, 12
	add 23, 23, 12
	addi 19, 19, 64
	add 30, 30, 12
	add 8, 8, 12
	add 7, 7, 12
	std 14, 744(1)                          # 8-byte Folded Spill
	ld 14, 504(1)                           # 8-byte Folded Reload
	std 28, 728(1)                          # 8-byte Folded Spill
	mr	28, 3
	beq	0, .Lp9_fma_BB0_46
.Lp9_fma_BB0_49:                               #   Parent Loop BB0_47 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB0_51 Depth 3
                                        #         Child Loop BB0_53 Depth 4
                                        #         Child Loop BB0_56 Depth 4
                                        #         Child Loop BB0_59 Depth 4
                                        #         Child Loop BB0_62 Depth 4
                                        #         Child Loop BB0_65 Depth 4
                                        #         Child Loop BB0_68 Depth 4
                                        #         Child Loop BB0_73 Depth 4
                                        #           Child Loop BB0_76 Depth 5
                                        #           Child Loop BB0_79 Depth 5
                                        #       Child Loop BB0_82 Depth 3
                                        #       Child Loop BB0_88 Depth 3
	std 29, 680(1)                          # 8-byte Folded Spill
	cmpldi	24, 16
	mr	29, 14
	mr	14, 7
	li 7, 16
	std 30, 720(1)                          # 8-byte Folded Spill
	std 22, 704(1)                          # 8-byte Folded Spill
	mr	22, 6
	ld 6, 456(1)                            # 8-byte Folded Reload
	std 11, 688(1)                          # 8-byte Folded Spill
	mr	11, 5
	li 5, 1
	std 19, 696(1)                          # 8-byte Folded Spill
	mr	19, 0
	std 20, 712(1)                          # 8-byte Folded Spill
	mr	20, 8
	isellt	30, 24, 7
	cmpldi	30, 1
	rlwinm 3, 30, 0, 27, 30
	std 24, 576(1)                          # 8-byte Folded Spill
	mr	24, 26
	mr	26, 10
	std 11, 856(1)                          # 8-byte Folded Spill
	std 28, 568(1)                          # 8-byte Folded Spill
	std 3, 552(1)                           # 8-byte Folded Spill
	addi 3, 3, -2
	rldicl 3, 3, 63, 1
	addi 3, 3, 1
	std 3, 536(1)                           # 8-byte Folded Spill
	sldi 3, 28, 6
	add 6, 6, 3
	add 6, 29, 6
	std 6, 528(1)                           # 8-byte Folded Spill
	ld 6, 448(1)                            # 8-byte Folded Reload
	add 3, 6, 3
	add 3, 29, 3
	sub	29, 4, 11
	std 3, 520(1)                           # 8-byte Folded Spill
	ld 3, 376(1)                            # 8-byte Folded Reload
	cmpldi 3, 29, 16
	maddld 3, 12, 28, 3
	std 3, 848(1)                           # 8-byte Folded Spill
	ld 3, 384(1)                            # 8-byte Folded Reload
	maddld 3, 12, 28, 3
	std 3, 840(1)                           # 8-byte Folded Spill
	ld 3, 392(1)                            # 8-byte Folded Reload
	maddld 3, 12, 28, 3
	std 3, 832(1)                           # 8-byte Folded Spill
	ld 3, 400(1)                            # 8-byte Folded Reload
	maddld 3, 12, 28, 3
	std 3, 824(1)                           # 8-byte Folded Spill
	ld 3, 408(1)                            # 8-byte Folded Reload
	maddld 3, 12, 28, 3
	std 3, 816(1)                           # 8-byte Folded Spill
	ld 3, 416(1)                            # 8-byte Folded Reload
	maddld 3, 12, 28, 3
	std 3, 808(1)                           # 8-byte Folded Spill
	ld 3, 424(1)                            # 8-byte Folded Reload
	maddld 3, 12, 28, 3
	std 3, 800(1)                           # 8-byte Folded Spill
	ld 3, 432(1)                            # 8-byte Folded Reload
	maddld 3, 12, 28, 3
	std 3, 792(1)                           # 8-byte Folded Spill
	ld 3, 352(1)                            # 8-byte Folded Reload
	maddld 3, 12, 28, 3
	std 3, 784(1)                           # 8-byte Folded Spill
	ld 3, 440(1)                            # 8-byte Folded Reload
	maddld 3, 12, 28, 3
	std 3, 776(1)                           # 8-byte Folded Spill
	ld 3, 464(1)                            # 8-byte Folded Reload
	maddld 3, 12, 28, 3
	std 3, 768(1)                           # 8-byte Folded Spill
	mulld 3, 12, 28
	addi 3, 3, -4
	std 3, 760(1)                           # 8-byte Folded Spill
	iselgt	3, 30, 5
	std 3, 560(1)                           # 8-byte Folded Spill
	isel 3, 29, 7, 12
	std 3, 896(1)                           # 8-byte Folded Spill
	sldi 3, 28, 4
	mr	28, 9
	sub	3, 4, 3
	li 4, 0
	cmpldi	3, 16
	isellt	3, 3, 7
	sldi 3, 3, 2
	cmpldi	3, 1
	iselgt	3, 3, 5
	sldi 5, 3, 3
	addi 3, 1, 1040
	std 3, 544(1)                           # 8-byte Folded Spill
	bl memset
	nop
	ld 4, 360(1)                            # 8-byte Folded Reload
	std 18, 672(1)                          # 8-byte Folded Spill
	mr	7, 18
	ld 9, 744(1)                            # 8-byte Folded Reload
	mr	10, 17
	ld 8, 736(1)                            # 8-byte Folded Reload
	mr	11, 23
	mr	12, 14
	std 17, 664(1)                          # 8-byte Folded Spill
	ld 17, 480(1)                           # 8-byte Folded Reload
	ld 3, 472(1)                            # 8-byte Folded Reload
	ld 18, 904(1)                           # 8-byte Folded Reload
	li 6, 0
	cmpldi 1, 29, 4
	cmpldi 5, 29, 8
	cmpldi 6, 29, 12
	mr	29, 20
	li 5, 0
	std 23, 656(1)                          # 8-byte Folded Spill
	mr	23, 31
	std 22, 648(1)                          # 8-byte Folded Spill
	std 22, 1024(1)                         # 8-byte Folded Spill
	mr	22, 19
	std 15, 640(1)                          # 8-byte Folded Spill
	std 15, 1016(1)                         # 8-byte Folded Spill
	ld 15, 496(1)                           # 8-byte Folded Reload
	std 14, 632(1)                          # 8-byte Folded Spill
	ld 14, 720(1)                           # 8-byte Folded Reload
	std 19, 616(1)                          # 8-byte Folded Spill
	ld 19, 488(1)                           # 8-byte Folded Reload
	std 31, 608(1)                          # 8-byte Folded Spill
	mr	31, 4
	mr	4, 26
	std 26, 592(1)                          # 8-byte Folded Spill
	ld 26, 728(1)                           # 8-byte Folded Reload
	std 20, 624(1)                          # 8-byte Folded Spill
	mr	20, 27
	std 28, 600(1)                          # 8-byte Folded Spill
	std 24, 584(1)                          # 8-byte Folded Spill
	std 3, 1032(1)                          # 8-byte Folded Spill
	b .Lp9_fma_BB0_51
	.p2align	4
.Lp9_fma_BB0_50:                               #   in Loop: Header=BB0_51 Depth=3
	ld 3, 1032(1)                           # 8-byte Folded Reload
	ld 24, 912(1)                           # 8-byte Folded Reload
	ld 4, 920(1)                            # 8-byte Folded Reload
	ld 28, 928(1)                           # 8-byte Folded Reload
	addi 5, 5, 256
	addi 18, 18, -256
	addi 19, 19, 1024
	ld 14, 936(1)                           # 8-byte Folded Reload
	ld 22, 944(1)                           # 8-byte Folded Reload
	ld 29, 952(1)                           # 8-byte Folded Reload
	ld 11, 960(1)                           # 8-byte Folded Reload
	addi 15, 15, 1024
	addi 17, 17, 1024
	ld 10, 968(1)                           # 8-byte Folded Reload
	ld 9, 976(1)                            # 8-byte Folded Reload
	ld 7, 984(1)                            # 8-byte Folded Reload
	ld 6, 1008(1)                           # 8-byte Folded Reload
	addi 26, 26, 1024
	addi 23, 23, 1024
	addi 12, 12, 1024
	addi 8, 8, 1024
	addi 31, 31, 1024
	addi 20, 20, 1024
	addi 3, 3, 1024
	addi 24, 24, 1024
	addi 4, 4, 1024
	addi 28, 28, 1024
	addi 14, 14, 1024
	addi 22, 22, 1024
	addi 29, 29, 1024
	addi 11, 11, 1024
	addi 10, 10, 1024
	addi 9, 9, 1024
	addi 7, 7, 1024
	addi 6, 6, 1
	std 3, 1032(1)                          # 8-byte Folded Spill
	ld 3, 1016(1)                           # 8-byte Folded Reload
	addi 3, 3, 1024
	std 3, 1016(1)                          # 8-byte Folded Spill
	ld 3, 1024(1)                           # 8-byte Folded Reload
	addi 3, 3, 1024
	std 3, 1024(1)                          # 8-byte Folded Spill
	ld 3, 904(1)                            # 8-byte Folded Reload
	cmpld	5, 3
	bge 0, .Lp9_fma_BB0_80
.Lp9_fma_BB0_51:                               #   Parent Loop BB0_47 Depth=1
                                        #     Parent Loop BB0_49 Depth=2
                                        # =>    This Loop Header: Depth=3
                                        #         Child Loop BB0_53 Depth 4
                                        #         Child Loop BB0_56 Depth 4
                                        #         Child Loop BB0_59 Depth 4
                                        #         Child Loop BB0_62 Depth 4
                                        #         Child Loop BB0_65 Depth 4
                                        #         Child Loop BB0_68 Depth 4
                                        #         Child Loop BB0_73 Depth 4
                                        #           Child Loop BB0_76 Depth 5
                                        #           Child Loop BB0_79 Depth 5
	cmpldi	18, 256
	li 3, 256
	std 4, 920(1)                           # 8-byte Folded Spill
	std 6, 1008(1)                          # 8-byte Folded Spill
	sldi 6, 6, 10
	std 28, 928(1)                          # 8-byte Folded Spill
	std 11, 960(1)                          # 8-byte Folded Spill
	std 10, 968(1)                          # 8-byte Folded Spill
	std 9, 976(1)                           # 8-byte Folded Spill
	std 7, 984(1)                           # 8-byte Folded Spill
	isellt	4, 18, 3
	li 3, 1
	cmpldi	4, 1
	iselgt	0, 4, 3
	cmpldi	18, 2
	li 3, 0
	blt	0, .Lp9_fma_BB0_55
# %bb.52:                               #   in Loop: Header=BB0_51 Depth=3
	rlwinm 3, 4, 0, 23, 30
	addi 7, 3, -2
	rldicl 7, 7, 63, 1
	addi 9, 7, 1
	addi 7, 1, 1552
	mtctr 9
	li 9, 0
	ld 10, 1032(1)                          # 8-byte Folded Reload
	.p2align	5
.Lp9_fma_BB0_53:                               #   Parent Loop BB0_47 Depth=1
                                        #     Parent Loop BB0_49 Depth=2
                                        #       Parent Loop BB0_51 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfd 0, 0(10)
	lfdx 1, 17, 9
	addi 10, 10, 8
	xxmrghw 0, 0, 0
	lfdx 2, 15, 9
	lfdx 3, 19, 9
	xxmrghw 1, 1, 1
	xxmrghw 2, 2, 2
	xxmrghw 3, 3, 3
	addi 9, 9, 8
	xvcvspdp 0, 0
	xvcvspdp 1, 1
	xvcvspdp 2, 2
	xvcvspdp 3, 3
	xxmrgld	4, 3, 2
	xxmrghd	2, 3, 2
	xxmrgld	3, 1, 0
	xxmrghd	0, 1, 0
	stxv 2, 48(7)
	stxv 0, 32(7)
	stxv 4, 16(7)
	stxv 3, 0(7)
	addi 7, 7, 64
	bdnz .Lp9_fma_BB0_53
# %bb.54:                               #   in Loop: Header=BB0_51 Depth=3
	cmpld	4, 3
	beq	0, .Lp9_fma_BB0_57
.Lp9_fma_BB0_55:                               #   in Loop: Header=BB0_51 Depth=3
	ld 4, 872(1)                            # 8-byte Folded Reload
	ld 7, 888(1)                            # 8-byte Folded Reload
	sldi 9, 3, 5
	ld 11, 864(1)                           # 8-byte Folded Reload
	add 4, 4, 6
	add 9, 11, 9
	add 10, 7, 4
	ld 4, 880(1)                            # 8-byte Folded Reload
	add 4, 4, 6
	add 7, 7, 4
	sldi 4, 3, 2
	sub	3, 0, 3
	mtctr 3
	add 7, 7, 4
	add 10, 10, 4
	.p2align	5
.Lp9_fma_BB0_56:                               #   Parent Loop BB0_47 Depth=1
                                        #     Parent Loop BB0_49 Depth=2
                                        #       Parent Loop BB0_51 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfsu 1, 4(7)
	stfdu 1, 32(9)
	lfsu 0, 4(10)
	stfd 0, 8(9)
	lfsx 0, 15, 4
	stfd 0, 16(9)
	lfsx 0, 19, 4
	addi 4, 4, 4
	stfd 0, 24(9)
	bdnz .Lp9_fma_BB0_56
.Lp9_fma_BB0_57:                               #   in Loop: Header=BB0_51 Depth=3
	std 14, 936(1)                          # 8-byte Folded Spill
	li 14, 0
	std 24, 912(1)                          # 8-byte Folded Spill
	std 22, 944(1)                          # 8-byte Folded Spill
	std 29, 952(1)                          # 8-byte Folded Spill
	std 31, 992(1)                          # 8-byte Folded Spill
	std 20, 1000(1)                         # 8-byte Folded Spill
	bc 12, 4, .Lp9_fma_BB0_70
# %bb.58:                               #   in Loop: Header=BB0_51 Depth=3
	ld 7, 832(1)                            # 8-byte Folded Reload
	ld 3, 848(1)                            # 8-byte Folded Reload
	ld 4, 840(1)                            # 8-byte Folded Reload
	lxv 0, 1152(1)
	mr	16, 21
	add 10, 7, 6
	ld 7, 824(1)                            # 8-byte Folded Reload
	add 3, 3, 6
	add 4, 4, 6
	lxv 1, 1136(1)
	lxv 2, 1120(1)
	lxv 3, 1104(1)
	lxv 4, 1088(1)
	lxv 5, 1072(1)
	lxv 6, 1056(1)
	lxv 7, 1040(1)
	add 9, 27, 4
	ld 4, 760(1)                            # 8-byte Folded Reload
	add 10, 27, 10
	add 11, 7, 6
	ld 7, 816(1)                            # 8-byte Folded Reload
	add 4, 4, 6
	add 11, 27, 11
	add 29, 7, 6
	ld 7, 808(1)                            # 8-byte Folded Reload
	add 4, 27, 4
	add 20, 27, 29
	li 29, 0
	add 28, 7, 6
	add 7, 27, 3
	ld 3, 800(1)                            # 8-byte Folded Reload
	add 22, 27, 28
	add 3, 3, 6
	add 28, 27, 3
	ld 3, 792(1)                            # 8-byte Folded Reload
	add 3, 3, 6
	add 24, 27, 3
	ld 3, 784(1)                            # 8-byte Folded Reload
	add 3, 3, 6
	add 31, 27, 3
	ld 3, 776(1)                            # 8-byte Folded Reload
	add 3, 3, 6
	add 14, 27, 3
	ld 3, 768(1)                            # 8-byte Folded Reload
	mtctr 0
	add 3, 3, 6
	add 3, 27, 3
	.p2align	5
.Lp9_fma_BB0_59:                               #   Parent Loop BB0_47 Depth=1
                                        #     Parent Loop BB0_49 Depth=2
                                        #       Parent Loop BB0_51 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfsu 8, 4(14)
	lxv 11, -16(16)
	lxv 12, 0(16)
	addi 16, 16, 32
	xxspltd	8, 8, 0
	lfsu 9, 4(3)
	lfsu 10, 4(4)
	lfsx 13, 26, 29
	xxspltd	10, 10, 0
	xxspltd	9, 9, 0
	xxspltd	13, 13, 0
	addi 29, 29, 4
	xvmaddadp 7, 10, 11
	xvmaddadp 6, 10, 12
	xvmaddadp 5, 9, 11
	xvmaddadp 4, 9, 12
	xvmaddadp 3, 8, 11
	xvmaddadp 2, 8, 12
	xvmaddadp 1, 13, 11
	xvmaddadp 0, 13, 12
	bdnz .Lp9_fma_BB0_59
# %bb.60:                               #   in Loop: Header=BB0_51 Depth=3
	ld 16, 752(1)                           # 8-byte Folded Reload
	li 14, 4
	stxv 7, 1040(1)
	stxv 6, 1056(1)
	stxv 5, 1072(1)
	stxv 4, 1088(1)
	stxv 3, 1104(1)
	stxv 2, 1120(1)
	stxv 1, 1136(1)
	stxv 0, 1152(1)
	bc 12, 20, .Lp9_fma_BB0_70
# %bb.61:                               #   in Loop: Header=BB0_51 Depth=3
	lxv 0, 1280(1)
	lxv 1, 1264(1)
	lxv 2, 1248(1)
	lxv 3, 1232(1)
	li 3, 0
	mr	14, 21
	lxv 4, 1216(1)
	lxv 5, 1200(1)
	lxv 6, 1184(1)
	lxv 7, 1168(1)
	mtctr 0
	.p2align	5
.Lp9_fma_BB0_62:                               #   Parent Loop BB0_47 Depth=1
                                        #     Parent Loop BB0_49 Depth=2
                                        #       Parent Loop BB0_51 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfsu 8, 4(28)
	lxv 11, -16(14)
	lxv 12, 0(14)
	addi 14, 14, 32
	xxspltd	8, 8, 0
	lfsu 9, 4(24)
	lfsu 10, 4(31)
	lfsx 13, 23, 3
	xxspltd	10, 10, 0
	xxspltd	9, 9, 0
	xxspltd	13, 13, 0
	addi 3, 3, 4
	xvmaddadp 7, 10, 11
	xvmaddadp 6, 10, 12
	xvmaddadp 5, 9, 11
	xvmaddadp 4, 9, 12
	xvmaddadp 3, 8, 11
	xvmaddadp 2, 8, 12
	xvmaddadp 1, 13, 11
	xvmaddadp 0, 13, 12
	bdnz .Lp9_fma_BB0_62
# %bb.63:                               #   in Loop: Header=BB0_51 Depth=3
	li 14, 8
	stxv 7, 1168(1)
	stxv 6, 1184(1)
	stxv 5, 1200(1)
	stxv 4, 1216(1)
	stxv 3, 1232(1)
	stxv 2, 1248(1)
	stxv 1, 1264(1)
	stxv 0, 1280(1)
	bc 12, 24, .Lp9_fma_BB0_70
# %bb.64:                               #   in Loop: Header=BB0_51 Depth=3
	lxv 0, 1408(1)
	lxv 1, 1392(1)
	lxv 2, 1376(1)
	lxv 3, 1360(1)
	li 3, 0
	mr	28, 21
	lxv 4, 1344(1)
	lxv 5, 1328(1)
	lxv 6, 1312(1)
	lxv 7, 1296(1)
	mtctr 0
	.p2align	5
.Lp9_fma_BB0_65:                               #   Parent Loop BB0_47 Depth=1
                                        #     Parent Loop BB0_49 Depth=2
                                        #       Parent Loop BB0_51 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfsu 8, 4(11)
	lxv 11, -16(28)
	lxv 12, 0(28)
	addi 28, 28, 32
	xxspltd	8, 8, 0
	lfsu 9, 4(20)
	lfsu 10, 4(22)
	lfsx 13, 12, 3
	xxspltd	10, 10, 0
	xxspltd	9, 9, 0
	xxspltd	13, 13, 0
	addi 3, 3, 4
	xvmaddadp 7, 10, 11
	xvmaddadp 6, 10, 12
	xvmaddadp 5, 9, 11
	xvmaddadp 4, 9, 12
	xvmaddadp 3, 8, 11
	xvmaddadp 2, 8, 12
	xvmaddadp 1, 13, 11
	xvmaddadp 0, 13, 12
	bdnz .Lp9_fma_BB0_65
# %bb.66:                               #   in Loop: Header=BB0_51 Depth=3
	li 14, 12
	stxv 7, 1296(1)
	stxv 6, 1312(1)
	stxv 5, 1328(1)
	stxv 4, 1344(1)
	stxv 3, 1360(1)
	stxv 2, 1376(1)
	stxv 1, 1392(1)
	stxv 0, 1408(1)
	bc 12, 12, .Lp9_fma_BB0_70
# %bb.67:                               #   in Loop: Header=BB0_51 Depth=3
	lxv 0, 1536(1)
	lxv 1, 1520(1)
	lxv 2, 1504(1)
	lxv 3, 1488(1)
	li 3, 0
	mr	11, 21
	lxv 4, 1472(1)
	lxv 5, 1456(1)
	lxv 6, 1440(1)
	lxv 7, 1424(1)
	mtctr 0
	.p2align	5
.Lp9_fma_BB0_68:                               #   Parent Loop BB0_47 Depth=1
                                        #     Parent Loop BB0_49 Depth=2
                                        #       Parent Loop BB0_51 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfsu 8, 4(7)
	lxv 11, -16(11)
	lxv 12, 0(11)
	addi 11, 11, 32
	xxspltd	8, 8, 0
	lfsu 9, 4(9)
	lfsu 10, 4(10)
	lfsx 13, 8, 3
	xxspltd	10, 10, 0
	xxspltd	9, 9, 0
	xxspltd	13, 13, 0
	addi 3, 3, 4
	xvmaddadp 7, 10, 11
	xvmaddadp 6, 10, 12
	xvmaddadp 5, 9, 11
	xvmaddadp 4, 9, 12
	xvmaddadp 3, 8, 11
	xvmaddadp 2, 8, 12
	xvmaddadp 1, 13, 11
	xvmaddadp 0, 13, 12
	bdnz .Lp9_fma_BB0_68
# %bb.69:                               #   in Loop: Header=BB0_51 Depth=3
	stxv 7, 1424(1)
	stxv 6, 1440(1)
	li 14, 16
	stxv 5, 1456(1)
	stxv 4, 1472(1)
	stxv 3, 1488(1)
	stxv 2, 1504(1)
	stxv 1, 1520(1)
	stxv 0, 1536(1)
	.p2align	4
.Lp9_fma_BB0_70:                               #   in Loop: Header=BB0_51 Depth=3
	ld 3, 896(1)                            # 8-byte Folded Reload
	ld 20, 1000(1)                          # 8-byte Folded Reload
	ld 31, 992(1)                           # 8-byte Folded Reload
	cmpld	14, 3
	bge 0, .Lp9_fma_BB0_50
# %bb.71:                               #   in Loop: Header=BB0_51 Depth=3
	rlwinm 3, 0, 0, 23, 29
	addi 4, 6, -16
	andi. 7, 0, 3
	cmpldi 7, 18, 4
	li 28, 0
	addi 3, 3, -4
	rldicl 3, 3, 62, 2
	addi 9, 3, 1
	addi 3, 6, -4
	ld 6, 856(1)                            # 8-byte Folded Reload
	add 0, 6, 14
	maddld 6, 25, 0, 31
	maddld 10, 25, 0, 20
	maddld 11, 25, 0, 4
	maddld 0, 25, 0, 3
	b .Lp9_fma_BB0_73
	.p2align	4
.Lp9_fma_BB0_72:                               #   in Loop: Header=BB0_73 Depth=4
	addi 14, 14, 1
	add 6, 6, 25
	add 10, 10, 25
	addi 28, 28, 1
	stxv 1, 0(24)
	cmpld 4, 14, 30
	stxv 0, 16(24)
	beq	4, .Lp9_fma_BB0_50
.Lp9_fma_BB0_73:                               #   Parent Loop BB0_47 Depth=1
                                        #     Parent Loop BB0_49 Depth=2
                                        #       Parent Loop BB0_51 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB0_76 Depth 5
                                        #           Child Loop BB0_79 Depth 5
	addi 4, 1, 1040
	sldi 3, 14, 5
	add 24, 4, 3
	lxvx 1, 4, 3
	lxv 0, 16(24)
	bc 4, 28, .Lp9_fma_BB0_75
# %bb.74:                               #   in Loop: Header=BB0_73 Depth=4
	li 22, 0
	b .Lp9_fma_BB0_77
	.p2align	4
.Lp9_fma_BB0_75:                               #   in Loop: Header=BB0_73 Depth=4
	maddld 3, 25, 28, 11
	mtctr 9
	li 22, 0
	mr	4, 16
	add 3, 27, 3
	.p2align	5
.Lp9_fma_BB0_76:                               #   Parent Loop BB0_47 Depth=1
                                        #     Parent Loop BB0_49 Depth=2
                                        #       Parent Loop BB0_51 Depth=3
                                        #         Parent Loop BB0_73 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	lfsu 2, 16(3)
	lxv 3, -64(4)
	lxv 4, -48(4)
	addi 22, 22, 4
	xxspltd	2, 2, 0
	xvmaddadp 1, 2, 3
	xvmaddadp 0, 2, 4
	lfs 2, 4(3)
	lxv 3, -32(4)
	xxspltd	2, 2, 0
	xvmaddadp 1, 2, 3
	lxv 3, -16(4)
	xvmaddadp 0, 2, 3
	lfs 2, 8(3)
	lxv 3, 0(4)
	xxspltd	2, 2, 0
	xvmaddadp 1, 2, 3
	lxv 3, 16(4)
	xvmaddadp 0, 2, 3
	lfs 2, 12(3)
	lxv 3, 32(4)
	xxspltd	2, 2, 0
	xvmaddadp 1, 2, 3
	lxv 3, 48(4)
	addi 4, 4, 128
	xvmaddadp 0, 2, 3
	bdnz .Lp9_fma_BB0_76
.Lp9_fma_BB0_77:                               #   in Loop: Header=BB0_73 Depth=4
	bc 12, 2, .Lp9_fma_BB0_72
# %bb.78:                               #   in Loop: Header=BB0_73 Depth=4
	maddld 3, 25, 28, 0
	sldi 29, 22, 2
	add 4, 27, 3
	sldi 3, 22, 5
	mtctr 7
	add 3, 21, 3
	add 4, 4, 29
	.p2align	5
.Lp9_fma_BB0_79:                               #   Parent Loop BB0_47 Depth=1
                                        #     Parent Loop BB0_49 Depth=2
                                        #       Parent Loop BB0_51 Depth=3
                                        #         Parent Loop BB0_73 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	lfsu 2, 4(4)
	lxv 3, -16(3)
	xxspltd	2, 2, 0
	xvmaddadp 1, 2, 3
	lxv 3, 0(3)
	addi 3, 3, 32
	xvmaddadp 0, 2, 3
	bdnz .Lp9_fma_BB0_79
	b .Lp9_fma_BB0_72
.Lp9_fma_BB0_80:                               #   in Loop: Header=BB0_49 Depth=2
	ld 24, 576(1)                           # 8-byte Folded Reload
	ld 12, 344(1)                           # 8-byte Folded Reload
	ld 20, 712(1)                           # 8-byte Folded Reload
	ld 22, 704(1)                           # 8-byte Folded Reload
	li 11, 0
	cmpldi	24, 2
	ld 29, 680(1)                           # 8-byte Folded Reload
	ld 18, 672(1)                           # 8-byte Folded Reload
	ld 14, 744(1)                           # 8-byte Folded Reload
	ld 17, 664(1)                           # 8-byte Folded Reload
	ld 4, 736(1)                            # 8-byte Folded Reload
	ld 23, 648(1)                           # 8-byte Folded Reload
	ld 15, 640(1)                           # 8-byte Folded Reload
	ld 0, 616(1)                            # 8-byte Folded Reload
	ld 26, 584(1)                           # 8-byte Folded Reload
	ld 28, 728(1)                           # 8-byte Folded Reload
	ld 5, 856(1)                            # 8-byte Folded Reload
	ld 19, 560(1)                           # 8-byte Folded Reload
	cror 20, 0, 8
	bc 12, 20, .Lp9_fma_BB0_84
# %bb.81:                               #   in Loop: Header=BB0_49 Depth=2
	ld 3, 536(1)                            # 8-byte Folded Reload
	mtctr 3
	mr	3, 29
	ld 7, 336(1)                            # 8-byte Folded Reload
	ld 8, 328(1)                            # 8-byte Folded Reload
	ld 9, 320(1)                            # 8-byte Folded Reload
	ld 10, 312(1)                           # 8-byte Folded Reload
	ld 11, 544(1)                           # 8-byte Folded Reload
	.p2align	5
.Lp9_fma_BB0_82:                               #   Parent Loop BB0_47 Depth=1
                                        #     Parent Loop BB0_49 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lxv 1, 0(11)
	lxv 2, 32(11)
	lxv 0, 16(11)
	lxv 5, 48(11)
	addi 11, 11, 64
	xxswapd	3, 2
	xxswapd	4, 1
	xxswapd	6, 5
	xsrsp 4, 4
	xsrsp 3, 3
	xscvdpspn 4, 4
	xscvdpspn 3, 3
	xxmrghw 3, 3, 4
	xxswapd	4, 0
	xxswapd	3, 3
	stfdx 3, 7, 3
	xsrsp 1, 1
	xsrsp 2, 2
	xscvdpspn 1, 1
	xscvdpspn 2, 2
	xxmrghw 1, 2, 1
	xxswapd	1, 1
	stfdx 1, 10, 3
	xsrsp 1, 4
	xsrsp 2, 6
	xscvdpspn 1, 1
	xscvdpspn 2, 2
	xxmrghw 1, 2, 1
	xxswapd	1, 1
	stfdx 1, 9, 3
	xsrsp 0, 0
	xsrsp 1, 5
	xscvdpspn 0, 0
	xscvdpspn 1, 1
	xxmrghw 0, 1, 0
	xxswapd	0, 0
	stfdx 0, 8, 3
	addi 3, 3, 8
	bdnz .Lp9_fma_BB0_82
# %bb.83:                               #   in Loop: Header=BB0_49 Depth=2
	ld 11, 552(1)                           # 8-byte Folded Reload
	cmpld	30, 11
	beq	0, .Lp9_fma_BB0_48
.Lp9_fma_BB0_84:                               #   in Loop: Header=BB0_49 Depth=2
	ori 3, 11, 1
	andi. 7, 19, 1
	bc 4, 1, .Lp9_fma_BB0_86
# %bb.85:                               #   in Loop: Header=BB0_49 Depth=2
	addi 10, 1, 1040
	sldi 9, 11, 5
	ld 8, 304(1)                            # 8-byte Folded Reload
	sldi 7, 5, 2
	add 7, 8, 7
	lfdx 0, 10, 9
	sldi 8, 11, 2
	mr	11, 3
	xsrsp 0, 0
	stfsux 0, 7, 8
	add 8, 10, 9
	lfd 0, 8(8)
	xsrsp 0, 0
	ld 9, 296(1)                            # 8-byte Folded Reload
	stfsx 0, 7, 9
	lfd 0, 16(8)
	xsrsp 0, 0
	ld 9, 280(1)                            # 8-byte Folded Reload
	stfsx 0, 7, 9
	lfd 0, 24(8)
	xsrsp 0, 0
	ld 8, 288(1)                            # 8-byte Folded Reload
	stfsx 0, 7, 8
.Lp9_fma_BB0_86:                               #   in Loop: Header=BB0_49 Depth=2
	cmpld	19, 3
	beq	0, .Lp9_fma_BB0_48
# %bb.87:                               #   in Loop: Header=BB0_49 Depth=2
	sldi 10, 11, 2
	sldi 8, 11, 5
	sub	11, 30, 11
	ld 9, 272(1)                            # 8-byte Folded Reload
	ld 19, 528(1)                           # 8-byte Folded Reload
	addi 11, 11, -2
	add 3, 22, 10
	add 7, 20, 10
	rldicl 11, 11, 63, 1
	add 8, 9, 8
	ld 9, 520(1)                            # 8-byte Folded Reload
	addi 11, 11, 1
	mtctr 11
	add 9, 9, 10
	add 10, 19, 10
	.p2align	5
.Lp9_fma_BB0_88:                               #   Parent Loop BB0_47 Depth=1
                                        #     Parent Loop BB0_49 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lfdu 0, 64(8)
	xsrsp 0, 0
	stfsu 0, 8(9)
	lfd 0, 8(8)
	xsrsp 0, 0
	stfsu 0, 8(10)
	lfd 0, 16(8)
	xsrsp 0, 0
	lfd 1, 24(8)
	stfs 0, 0(3)
	xsrsp 0, 1
	stfs 0, 0(7)
	lfd 0, 32(8)
	xsrsp 0, 0
	stfs 0, 4(9)
	lfd 0, 40(8)
	xsrsp 0, 0
	stfs 0, 4(10)
	lfd 0, 48(8)
	xsrsp 0, 0
	stfs 0, 4(3)
	lfd 0, 56(8)
	addi 3, 3, 8
	xsrsp 0, 0
	stfs 0, 4(7)
	addi 7, 7, 8
	bdnz .Lp9_fma_BB0_88
	b .Lp9_fma_BB0_48
	.long	0
	.quad	0
.Lp9_fma_func_end0:
	.size	pq_project_rows_power9_fma, .Lp9_fma_func_end0-.Lp9_fma_func_begin0
	.cfi_endproc
                                        # -- End function
