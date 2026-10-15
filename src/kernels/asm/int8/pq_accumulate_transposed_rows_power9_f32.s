# int8/pq_accumulate_transposed_rows_power9_f32.s: kernel.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_int8.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.text.pq_accumulate_transposed_rows_power9_f32,"ax",@progbits
	.globl	pq_accumulate_transposed_rows_power9_f32 # -- Begin function pq_accumulate_transposed_rows_power9_f32
	.p2align 6
	.type	pq_accumulate_transposed_rows_power9_f32,@function
pq_accumulate_transposed_rows_power9_f32: # @pq_accumulate_transposed_rows_power9_f32
.Lp9_int8_func_begin8:
	.cfi_startproc
.Lp9_int8_func_gep8:
	addis 2, 12, .TOC.-.Lp9_int8_func_gep8@ha
	addi 2, 2, .TOC.-.Lp9_int8_func_gep8@l
.Lp9_int8_func_lep8:
	.localentry	pq_accumulate_transposed_rows_power9_f32, .Lp9_int8_func_lep8-.Lp9_int8_func_gep8
# %bb.0:
	mflr 0
	.cfi_def_cfa_offset 224
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
	std 14, -144(1)                         # 8-byte Folded Spill
	std 15, -136(1)                         # 8-byte Folded Spill
	std 16, -128(1)                         # 8-byte Folded Spill
	std 17, -120(1)                         # 8-byte Folded Spill
	std 18, -112(1)                         # 8-byte Folded Spill
	std 19, -104(1)                         # 8-byte Folded Spill
	std 20, -96(1)                          # 8-byte Folded Spill
	std 21, -88(1)                          # 8-byte Folded Spill
	std 22, -80(1)                          # 8-byte Folded Spill
	std 23, -72(1)                          # 8-byte Folded Spill
	std 24, -64(1)                          # 8-byte Folded Spill
	std 25, -56(1)                          # 8-byte Folded Spill
	std 26, -48(1)                          # 8-byte Folded Spill
	std 27, -40(1)                          # 8-byte Folded Spill
	std 28, -32(1)                          # 8-byte Folded Spill
	std 29, -24(1)                          # 8-byte Folded Spill
	std 30, -16(1)                          # 8-byte Folded Spill
	std 31, -8(1)                           # 8-byte Folded Spill
	stdu 1, -224(1)
	ld 16, 336(1)
	ld 25, 328(1)
	mr	30, 3
	cmpdi	4, 0
	li 3, 1
	std 0, 240(1)
	bc 12, 2, .Lp9_int8_BB8_10
# %bb.1:
	cmpdi	8, 0
	bc 12, 2, .Lp9_int8_BB8_10
# %bb.2:
	cmpdi	25, 0
	bc 12, 2, .Lp9_int8_BB8_10
# %bb.3:
	cmpdi	16, 0
	bc 12, 2, .Lp9_int8_BB8_10
# %bb.4:
	cmpdi	6, 0
	bc 12, 2, .Lp9_int8_BB8_10
# %bb.5:
	cmpdi	7, 0
	bc 12, 2, .Lp9_int8_BB8_10
# %bb.6:
	cmpdi	9, 0
	bc 12, 2, .Lp9_int8_BB8_10
# %bb.7:
	ld 23, 320(1)
	not	11, 6
	cmpld	23, 11
	ble 0, .Lp9_int8_BB8_9
.Lp9_int8_BB8_8:
	li 3, 4
	b .Lp9_int8_BB8_10
.Lp9_int8_BB8_9:
	add 11, 23, 6
	cmpld	11, 10
	ble 0, .Lp9_int8_BB8_11
.Lp9_int8_BB8_10:
	addi 1, 1, 224
	ld 0, 16(1)
	ld 31, -8(1)                            # 8-byte Folded Reload
	ld 30, -16(1)                           # 8-byte Folded Reload
	ld 29, -24(1)                           # 8-byte Folded Reload
	mtlr 0
	ld 28, -32(1)                           # 8-byte Folded Reload
	ld 27, -40(1)                           # 8-byte Folded Reload
	ld 26, -48(1)                           # 8-byte Folded Reload
	ld 25, -56(1)                           # 8-byte Folded Reload
	ld 24, -64(1)                           # 8-byte Folded Reload
	ld 23, -72(1)                           # 8-byte Folded Reload
	ld 22, -80(1)                           # 8-byte Folded Reload
	ld 21, -88(1)                           # 8-byte Folded Reload
	ld 20, -96(1)                           # 8-byte Folded Reload
	ld 19, -104(1)                          # 8-byte Folded Reload
	ld 18, -112(1)                          # 8-byte Folded Reload
	ld 17, -120(1)                          # 8-byte Folded Reload
	ld 16, -128(1)                          # 8-byte Folded Reload
	ld 15, -136(1)                          # 8-byte Folded Reload
	ld 14, -144(1)                          # 8-byte Folded Reload
	blr
.Lp9_int8_BB8_11:
	mulhdu. 3, 10, 9
	bne	0, .Lp9_int8_BB8_8
# %bb.12:
	li 3, -1
	divdu 27, 3, 7
	cmpld	9, 27
	bgt	0, .Lp9_int8_BB8_8
# %bb.13:
	li 3, 2
	cmplwi	30, 35
	bgt	0, .Lp9_int8_BB8_10
# %bb.14:
	clrldi	11, 30, 32
	li 12, 1
	sld 11, 12, 11
	rldic 12, 12, 35, 28
	ori 12, 12, 20807
	and. 11, 11, 12
	beq	0, .Lp9_int8_BB8_10
# %bb.15:
	mr	18, 4
	mr	3, 30
	mr	4, 7
	mr	24, 5
	mr	22, 6
	mr	20, 10
	mr	21, 8
	mr	19, 9
	mr	29, 7
	bl pq_row_bytes
	nop
	cmpldi	3, 0
	beq	0, .Lp9_int8_BB8_31
# %bb.16:
	cmpld 1, 22, 27
	mr	28, 3
	mulhdu. 3, 3, 22
	li 3, 4
	crorc 20, 5, 2
	bc 12, 20, .Lp9_int8_BB8_10
# %bb.17:
	mulld 3, 28, 22
	mr	26, 22
	cmpld	3, 24
	bne	0, .Lp9_int8_BB8_31
# %bb.18:
	mulld 6, 29, 26
	ld 5, 344(1)
	mr	7, 29
	li 3, 5
	cmpld	5, 6
	blt	0, .Lp9_int8_BB8_10
# %bb.19:
	mr	4, 18
	sldi 24, 7, 2
	mr	27, 16
.Lp9_int8_BB8_20:                               # =>This Inner Loop Header: Depth=1
	mr	3, 30
	mr	5, 28
	mr	6, 27
	mr	18, 4
	bl p9_decode_row_vsx
	nop
	cmpldi	3, 0
	beq	0, .Lp9_int8_BB8_31
# %bb.21:                               #   in Loop: Header=BB8_20 Depth=1
	mr	3, 27
	mr	4, 29
	bl p9_all_finite_vsx
	nop
	cmplwi	3, 0
	beq	0, .Lp9_int8_BB8_32
# %bb.22:                               #   in Loop: Header=BB8_20 Depth=1
	addi 26, 26, -1
	add 4, 18, 28
	add 27, 27, 24
	mr	7, 29
	cmpldi	26, 0
	bc 12, 1, .Lp9_int8_BB8_20
# %bb.23:
	rldicr. 5, 19, 0, 61
	li 4, 0
	cmpldi 1, 7, 16
	std 19, 72(1)                           # 8-byte Folded Spill
	mr	6, 21
	mr	10, 22
	sldi 3, 23, 2
	std 20, 40(1)                           # 8-byte Folded Spill
	std 3, 32(1)                            # 8-byte Folded Spill
	blt	1, .Lp9_int8_BB8_35
# %bb.24:
	beq	0, .Lp9_int8_BB8_33
# %bb.25:
	ld 8, 40(1)                             # 8-byte Folded Reload
	ld 9, 32(1)                             # 8-byte Folded Reload
	addi 30, 16, 32
	li 27, 0
	li 23, 4
	mulli 12, 8, 12
	addi 0, 9, -4
	sldi 3, 8, 3
	sldi 4, 8, 2
	add 11, 6, 9
	sldi 22, 8, 4
	add 29, 0, 4
	add 28, 0, 3
	li 3, 16
	li 4, 0
.Lp9_int8_BB8_26:                               # =>This Loop Header: Depth=1
                                        #     Child Loop BB8_27 Depth 2
                                        #       Child Loop BB8_28 Depth 3
	sldi 8, 4, 3
	mr	4, 3
	sldi 3, 27, 6
	li 19, 0
	mr	18, 11
	li 17, 0
	add 21, 16, 3
	add 20, 25, 8
	.p2align	4
.Lp9_int8_BB8_27:                               #   Parent Loop BB8_26 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB8_28 Depth 3
	maddld 3, 22, 19, 28
	maddld 9, 22, 19, 29
	xxlxor 0, 0, 0
	xxlxor 1, 1, 1
	maddld 15, 22, 19, 0
	add 8, 6, 3
	mtctr 10
	xxlxor 2, 2, 2
	xxlxor 3, 3, 3
	xxlxor 4, 4, 4
	add 26, 6, 9
	xxlxor 5, 5, 5
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
	xxlxor 8, 8, 8
	add 9, 6, 15
	mr	3, 21
	xxlxor 9, 9, 9
	xxlxor 10, 10, 10
	xxlxor 11, 11, 11
	xxlxor 12, 12, 12
	xxlxor 13, 13, 13
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
	.p2align	5
.Lp9_int8_BB8_28:                               #   Parent Loop BB8_26 Depth=1
                                        #     Parent Loop BB8_27 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lxvwsx 32, 9, 23
	addi 9, 9, 4
	lxvwsx 36, 8, 23
	lxvwsx 37, 26, 23
	lxv 33, 0(3)
	addi 8, 8, 4
	addi 26, 26, 4
	lxv 38, 16(3)
	lxv 39, 32(3)
	lxv 40, 48(3)
	add 3, 3, 24
	lxvwsx 41, 9, 12
	xvmaddasp 35, 32, 33
	xvmaddasp 34, 32, 38
	xvmaddasp 13, 32, 39
	xvmaddasp 12, 32, 40
	xvmaddasp 11, 37, 33
	xvmaddasp 10, 37, 38
	xvmaddasp 9, 37, 39
	xvmaddasp 8, 37, 40
	xvmaddasp 7, 36, 33
	xvmaddasp 6, 36, 38
	xvmaddasp 5, 36, 39
	xvmaddasp 4, 36, 40
	xvmaddasp 3, 41, 33
	xvmaddasp 2, 41, 38
	xvmaddasp 1, 41, 39
	xvmaddasp 0, 41, 40
	bdnz .Lp9_int8_BB8_28
# %bb.29:                               #   in Loop: Header=BB8_27 Depth=2
	mulld 3, 17, 7
	xxmrglw 36, 35, 35
	xxmrghw 35, 35, 35
	add 18, 18, 22
	addi 19, 19, 1
	sldi 8, 3, 3
	xvcvspdp 36, 36
	xvcvspdp 35, 35
	xxmrglw 37, 34, 34
	xvcvspdp 37, 37
	xxmrghw 34, 34, 34
	xvcvspdp 34, 34
	xxmrglw 32, 13, 13
	xvcvspdp 32, 32
	lxvx 38, 20, 8
	add 3, 20, 8
	xxmrghw 13, 13, 13
	xvcvspdp 13, 13
	xxmrglw 33, 12, 12
	xvcvspdp 33, 33
	xxmrghw 12, 12, 12
	xvcvspdp 12, 12
	xvadddp 36, 38, 36
	lxv 38, 16(3)
	xvadddp 35, 38, 35
	lxv 38, 32(3)
	stxvx 36, 20, 8
	ori 8, 17, 1
	xxmrglw 36, 9, 9
	xxmrghw 9, 9, 9
	mulld 8, 8, 7
	xvadddp 37, 38, 37
	lxv 38, 48(3)
	xvcvspdp 36, 36
	xvcvspdp 9, 9
	stxv 35, 16(3)
	xxmrglw 35, 11, 11
	xxmrghw 11, 11, 11
	sldi 8, 8, 3
	xvadddp 34, 38, 34
	lxv 38, 64(3)
	stxv 37, 32(3)
	xvcvspdp 35, 35
	xvcvspdp 11, 11
	xvadddp 32, 38, 32
	lxv 38, 80(3)
	stxv 34, 48(3)
	xxmrglw 34, 10, 10
	xxmrghw 10, 10, 10
	xvadddp 13, 38, 13
	lxv 38, 96(3)
	xvcvspdp 34, 34
	stxv 32, 64(3)
	xvcvspdp 10, 10
	xvadddp 33, 38, 33
	lxv 38, 112(3)
	stxv 13, 80(3)
	xxmrglw 13, 8, 8
	xxmrghw 8, 8, 8
	xvadddp 12, 38, 12
	xvcvspdp 13, 13
	xvcvspdp 8, 8
	stxv 33, 96(3)
	stxv 12, 112(3)
	add 3, 20, 8
	lxvx 12, 20, 8
	lxv 37, 16(3)
	xvadddp 12, 12, 35
	lxv 35, 32(3)
	xvadddp 11, 37, 11
	lxv 37, 48(3)
	xvadddp 34, 35, 34
	lxv 35, 64(3)
	xvadddp 10, 37, 10
	lxv 37, 80(3)
	stxvx 12, 20, 8
	lxv 12, 112(3)
	ori 8, 17, 2
	stxv 11, 16(3)
	xxmrglw 11, 4, 4
	xxmrghw 4, 4, 4
	mulld 8, 8, 7
	xvadddp 35, 35, 36
	lxv 36, 96(3)
	xvadddp 9, 37, 9
	xxmrglw 37, 5, 5
	xxmrghw 5, 5, 5
	xvcvspdp 11, 11
	stxv 34, 32(3)
	xvadddp 8, 12, 8
	xxmrglw 12, 7, 7
	xxmrghw 7, 7, 7
	xvcvspdp 4, 4
	stxv 10, 48(3)
	sldi 8, 8, 3
	xvcvspdp 37, 37
	xvcvspdp 5, 5
	xvadddp 13, 36, 13
	xvcvspdp 12, 12
	xxmrglw 36, 6, 6
	xxmrghw 6, 6, 6
	xvcvspdp 7, 7
	stxv 35, 64(3)
	stxv 9, 80(3)
	xvcvspdp 36, 36
	xvcvspdp 6, 6
	stxv 8, 112(3)
	stxv 13, 96(3)
	add 3, 20, 8
	lxvx 8, 20, 8
	lxv 9, 16(3)
	lxv 10, 32(3)
	lxv 13, 48(3)
	lxv 34, 64(3)
	lxv 35, 80(3)
	xvadddp 8, 8, 12
	lxv 12, 96(3)
	xvadddp 7, 9, 7
	xvadddp 9, 10, 36
	xvadddp 6, 13, 6
	xvadddp 10, 34, 37
	xvadddp 5, 35, 5
	xxmrglw 13, 1, 1
	xxmrghw 1, 1, 1
	xvcvspdp 13, 13
	xvcvspdp 1, 1
	xvadddp 11, 12, 11
	xxmrglw 12, 2, 2
	xxmrghw 2, 2, 2
	stxvx 8, 20, 8
	lxv 8, 112(3)
	ori 8, 17, 3
	addi 17, 17, 4
	xvcvspdp 2, 2
	stxv 7, 16(3)
	stxv 9, 32(3)
	stxv 6, 48(3)
	stxv 10, 64(3)
	stxv 5, 80(3)
	xvcvspdp 12, 12
	mulld 8, 8, 7
	cmpld 1, 17, 5
	xvadddp 4, 8, 4
	stxv 11, 96(3)
	xxmrglw 8, 3, 3
	xxmrghw 3, 3, 3
	sldi 8, 8, 3
	stxv 4, 112(3)
	add 3, 20, 8
	xvcvspdp 8, 8
	xvcvspdp 3, 3
	lxvx 4, 20, 8
	lxv 7, 48(3)
	lxv 9, 64(3)
	lxv 5, 16(3)
	lxv 6, 32(3)
	xvadddp 4, 4, 8
	xvadddp 2, 7, 2
	xvadddp 3, 5, 3
	xvadddp 5, 6, 12
	stxvx 4, 20, 8
	stxv 2, 48(3)
	xvadddp 2, 9, 13
	stxv 3, 16(3)
	stxv 5, 32(3)
	stxv 2, 64(3)
	lxv 2, 80(3)
	xvadddp 1, 2, 1
	xxmrglw 2, 0, 0
	xxmrghw 0, 0, 0
	xvcvspdp 2, 2
	xvcvspdp 0, 0
	stxv 1, 80(3)
	lxv 1, 96(3)
	xvadddp 1, 1, 2
	stxv 1, 96(3)
	lxv 1, 112(3)
	xvadddp 0, 1, 0
	stxv 0, 112(3)
	blt	1, .Lp9_int8_BB8_27
# %bb.30:                               #   in Loop: Header=BB8_26 Depth=1
	addi 3, 4, 16
	addi 30, 30, 64
	addi 27, 27, 1
	cmpld 1, 3, 7
	ble 1, .Lp9_int8_BB8_26
	b .Lp9_int8_BB8_35
.Lp9_int8_BB8_31:
	li 3, 3
	b .Lp9_int8_BB8_10
.Lp9_int8_BB8_32:
	li 3, 6
	b .Lp9_int8_BB8_10
.Lp9_int8_BB8_33:
	li 3, 0
.Lp9_int8_BB8_34:                               # =>This Inner Loop Header: Depth=1
	addi 4, 3, 16
	addi 3, 3, 32
	cmpld 1, 3, 7
	mr	3, 4
	ble 1, .Lp9_int8_BB8_34
.Lp9_int8_BB8_35:
	beq	0, .Lp9_int8_BB8_48
# %bb.36:
	cmpld	4, 7
	bge 0, .Lp9_int8_BB8_48
# %bb.37:
	addi 3, 10, -1
	ld 9, 32(1)                             # 8-byte Folded Reload
	rldicr 8, 10, 0, 60
	mulli 23, 7, 12
	andi. 12, 10, 7
	li 26, 0
	cmpldi 1, 3, 7
	sldi 3, 4, 2
	mulli 21, 7, 28
	mulli 20, 7, 24
	add 3, 16, 3
	mulli 15, 7, 20
	add 11, 6, 9
	sldi 30, 7, 5
	sldi 27, 7, 3
	sldi 22, 7, 4
	std 3, 64(1)                            # 8-byte Folded Spill
	add 3, 6, 9
	addi 29, 3, 16
	ld 3, 40(1)                             # 8-byte Folded Reload
	sldi 28, 3, 2
	addi 3, 9, -32
	std 3, 56(1)                            # 8-byte Folded Spill
	addi 3, 9, -4
	std 3, 48(1)                            # 8-byte Folded Spill
	addi 3, 8, -8
	rldicl 3, 3, 61, 3
	addi 17, 3, 1
	b .Lp9_int8_BB8_39
.Lp9_int8_BB8_38:                               #   in Loop: Header=BB8_39 Depth=1
	addi 26, 26, 1
	add 29, 29, 28
	add 11, 11, 28
	cmpld 5, 26, 5
	beq	5, .Lp9_int8_BB8_48
.Lp9_int8_BB8_39:                               # =>This Loop Header: Depth=1
                                        #     Child Loop BB8_41 Depth 2
                                        #       Child Loop BB8_44 Depth 3
                                        #       Child Loop BB8_47 Depth 3
	ld 3, 48(1)                             # 8-byte Folded Reload
	ld 18, 64(1)                            # 8-byte Folded Reload
	maddld 3, 28, 26, 3
	add 14, 6, 3
	ld 3, 56(1)                             # 8-byte Folded Reload
	maddld 3, 28, 26, 3
	add 0, 6, 3
	mulld 3, 26, 7
	sldi 3, 3, 3
	add 9, 25, 3
	mr	3, 4
	b .Lp9_int8_BB8_41
	.p2align	4
.Lp9_int8_BB8_40:                               #   in Loop: Header=BB8_41 Depth=2
	sldi 8, 3, 3
	addi 3, 3, 1
	addi 18, 18, 4
	lfdx 1, 9, 8
	cmpld 5, 3, 7
	xsadddp 0, 1, 0
	stfdx 0, 9, 8
	beq	5, .Lp9_int8_BB8_38
.Lp9_int8_BB8_41:                               #   Parent Loop BB8_39 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB8_44 Depth 3
                                        #       Child Loop BB8_47 Depth 3
	bc 4, 4, .Lp9_int8_BB8_43
# %bb.42:                               #   in Loop: Header=BB8_41 Depth=2
	xxlxor 0, 0, 0
	li 19, 0
	b .Lp9_int8_BB8_45
.Lp9_int8_BB8_43:                               #   in Loop: Header=BB8_41 Depth=2
	mtctr 17
	li 19, 0
	mr	8, 18
	mr	31, 0
	xxlxor 0, 0, 0
	.p2align	5
.Lp9_int8_BB8_44:                               #   Parent Loop BB8_39 Depth=1
                                        #     Parent Loop BB8_41 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lfsu 1, 32(31)
	addi 19, 19, 8
	lfs 2, 0(8)
	xsmaddasp 0, 1, 2
	lfs 1, 4(31)
	lfsx 2, 8, 24
	lfsx 3, 8, 23
	lfs 4, 12(31)
	xsmaddasp 0, 1, 2
	lfs 1, 8(31)
	lfsx 2, 8, 27
	xsmaddasp 0, 1, 2
	lfs 1, 16(31)
	lfsx 2, 8, 22
	xsmaddasp 0, 4, 3
	lfs 3, 20(31)
	lfsx 4, 8, 20
	xsmaddasp 0, 1, 2
	lfsx 1, 8, 15
	lfs 2, 24(31)
	xsmaddasp 0, 3, 1
	lfs 1, 28(31)
	xsmaddasp 0, 2, 4
	lfsx 2, 8, 21
	add 8, 8, 30
	xsmaddasp 0, 1, 2
	bdnz .Lp9_int8_BB8_44
.Lp9_int8_BB8_45:                               #   in Loop: Header=BB8_41 Depth=2
	bc 12, 2, .Lp9_int8_BB8_40
# %bb.46:                               #   in Loop: Header=BB8_41 Depth=2
	mulld 8, 24, 19
	sldi 19, 19, 2
	add 19, 14, 19
	mtctr 12
	.p2align	5
.Lp9_int8_BB8_47:                               #   Parent Loop BB8_39 Depth=1
                                        #     Parent Loop BB8_41 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lfsu 1, 4(19)
	lfsx 2, 18, 8
	add 8, 8, 24
	xsmaddasp 0, 1, 2
	bdnz .Lp9_int8_BB8_47
	b .Lp9_int8_BB8_40
.Lp9_int8_BB8_48:
	ld 8, 72(1)                             # 8-byte Folded Reload
	li 3, 0
	cmpld	5, 8
	beq	0, .Lp9_int8_BB8_10
# %bb.49:
	ld 30, 40(1)                            # 8-byte Folded Reload
	rldicl 8, 8, 62, 2
	cmpldi	7, 1
	li 4, 1
	ld 9, 32(1)                             # 8-byte Folded Reload
	li 18, 0
	mulli 29, 7, 12
	mulli 27, 7, 28
	mulli 23, 7, 24
	mulli 22, 7, 20
	sldi 28, 7, 4
	iselgt	11, 7, 4
	addi 4, 10, -1
	andi. 12, 10, 7
	cmpldi 1, 4, 7
	mulld 8, 30, 8
	rldicr 4, 10, 0, 60
	addi 4, 4, -8
	sldi 10, 7, 5
	rldicl 4, 4, 61, 3
	sldi 8, 8, 4
	addi 19, 4, 1
	add 26, 8, 9
	sldi 8, 30, 2
	sldi 30, 7, 3
	add 9, 6, 26
	addi 21, 26, -32
	addi 26, 26, -4
	addi 0, 9, 16
	std 21, 64(1)                           # 8-byte Folded Spill
	std 26, 56(1)                           # 8-byte Folded Spill
	b .Lp9_int8_BB8_51
.Lp9_int8_BB8_50:                               #   in Loop: Header=BB8_51 Depth=1
	ld 4, 72(1)                             # 8-byte Folded Reload
	addi 5, 5, 1
	add 0, 0, 8
	add 9, 9, 8
	addi 18, 18, 1
	cmpld 5, 5, 4
	bge 5, .Lp9_int8_BB8_10
.Lp9_int8_BB8_51:                               # =>This Loop Header: Depth=1
                                        #     Child Loop BB8_53 Depth 2
                                        #       Child Loop BB8_56 Depth 3
                                        #       Child Loop BB8_59 Depth 3
	ld 4, 56(1)                             # 8-byte Folded Reload
	mr	26, 16
	maddld 4, 8, 18, 4
	add 17, 6, 4
	ld 4, 64(1)                             # 8-byte Folded Reload
	maddld 4, 8, 18, 4
	add 20, 6, 4
	mulld 4, 5, 7
	sldi 4, 4, 3
	add 15, 25, 4
	li 4, 0
	b .Lp9_int8_BB8_53
	.p2align	4
.Lp9_int8_BB8_52:                               #   in Loop: Header=BB8_53 Depth=2
	sldi 21, 4, 3
	addi 4, 4, 1
	addi 26, 26, 4
	lfdx 1, 15, 21
	cmpld 5, 4, 11
	xsadddp 0, 1, 0
	stfdx 0, 15, 21
	beq	5, .Lp9_int8_BB8_50
.Lp9_int8_BB8_53:                               #   Parent Loop BB8_51 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB8_56 Depth 3
                                        #       Child Loop BB8_59 Depth 3
	bc 4, 4, .Lp9_int8_BB8_55
# %bb.54:                               #   in Loop: Header=BB8_53 Depth=2
	xxlxor 0, 0, 0
	li 31, 0
	b .Lp9_int8_BB8_57
.Lp9_int8_BB8_55:                               #   in Loop: Header=BB8_53 Depth=2
	mtctr 19
	li 31, 0
	mr	14, 26
	mr	21, 20
	xxlxor 0, 0, 0
	.p2align	5
.Lp9_int8_BB8_56:                               #   Parent Loop BB8_51 Depth=1
                                        #     Parent Loop BB8_53 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lfsu 1, 32(21)
	addi 31, 31, 8
	lfs 2, 0(14)
	xsmaddasp 0, 1, 2
	lfs 1, 4(21)
	lfsx 2, 14, 24
	lfsx 3, 14, 29
	lfs 4, 12(21)
	xsmaddasp 0, 1, 2
	lfs 1, 8(21)
	lfsx 2, 14, 30
	xsmaddasp 0, 1, 2
	lfs 1, 16(21)
	lfsx 2, 14, 28
	xsmaddasp 0, 4, 3
	lfs 3, 20(21)
	lfsx 4, 14, 23
	xsmaddasp 0, 1, 2
	lfsx 1, 14, 22
	lfs 2, 24(21)
	xsmaddasp 0, 3, 1
	lfs 1, 28(21)
	xsmaddasp 0, 2, 4
	lfsx 2, 14, 27
	add 14, 14, 10
	xsmaddasp 0, 1, 2
	bdnz .Lp9_int8_BB8_56
.Lp9_int8_BB8_57:                               #   in Loop: Header=BB8_53 Depth=2
	bc 12, 2, .Lp9_int8_BB8_52
# %bb.58:                               #   in Loop: Header=BB8_53 Depth=2
	mulld 21, 24, 31
	sldi 14, 31, 2
	add 14, 17, 14
	mtctr 12
	.p2align	5
.Lp9_int8_BB8_59:                               #   Parent Loop BB8_51 Depth=1
                                        #     Parent Loop BB8_53 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lfsu 1, 4(14)
	lfsx 2, 26, 21
	add 21, 21, 24
	xsmaddasp 0, 1, 2
	bdnz .Lp9_int8_BB8_59
	b .Lp9_int8_BB8_52
	.long	0
	.quad	0
.Lp9_int8_func_end8:
	.size	pq_accumulate_transposed_rows_power9_f32, .Lp9_int8_func_end8-.Lp9_int8_func_begin8
	.cfi_endproc
                                        # -- End function
