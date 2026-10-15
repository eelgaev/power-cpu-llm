# int8/i8c_check.s: static helper.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_int8.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.text.i8c_check,"ax",@progbits
	.p2align	4                               # -- Begin function p9_int8_i8c_check
	.type	p9_int8_i8c_check,@function
p9_int8_i8c_check:                              # @p9_int8_i8c_check
.Lp9_int8_func_begin7:
	.cfi_startproc
.Lp9_int8_func_gep7:
	addis 2, 12, .TOC.-.Lp9_int8_func_gep7@ha
	addi 2, 2, .TOC.-.Lp9_int8_func_gep7@l
.Lp9_int8_func_lep7:
	.localentry	p9_int8_i8c_check, .Lp9_int8_func_lep7-.Lp9_int8_func_gep7
# %bb.0:
	mfcr 12
	mflr 0
	stw 12, 8(1)
	stdu 1, -96(1)
	std 0, 112(1)
	.cfi_def_cfa_offset 96
	.cfi_offset lr, 16
	.cfi_offset r24, -64
	.cfi_offset r25, -56
	.cfi_offset r26, -48
	.cfi_offset r27, -40
	.cfi_offset r28, -32
	.cfi_offset r29, -24
	.cfi_offset r30, -16
	.cfi_offset cr2, 8
	.cfi_offset cr3, 8
	ld 11, 128(1)
	cmpdi	4, 0
	cmpdi 1, 8, 0
	std 24, 32(1)                           # 8-byte Folded Spill
	std 25, 40(1)                           # 8-byte Folded Spill
	std 26, 48(1)                           # 8-byte Folded Spill
	std 27, 56(1)                           # 8-byte Folded Spill
	std 28, 64(1)                           # 8-byte Folded Spill
	std 29, 72(1)                           # 8-byte Folded Spill
	std 30, 80(1)                           # 8-byte Folded Spill
	cror 20, 2, 6
	cmpdi	11, 0
	cror 20, 20, 2
	cmpdi	6, 0
	cror 20, 2, 20
	cmpdi	7, 0
	cror 20, 2, 20
	cmpdi	10, 0
	cror 20, 2, 20
	andi. 8, 8, 15
	li 8, 1
	crorc 20, 20, 2
	bc 12, 20, .Lp9_int8_BB7_9
# %bb.1:
	li 8, 2
	cmplwi 2, 3, 35
	bgt	2, .Lp9_int8_BB7_9
# %bb.2:
	clrldi	12, 3, 32
	li 0, 1
	sld 11, 0, 12
	andi. 11, 11, 324
	bne	0, .Lp9_int8_BB7_5
# %bb.3:
	sld 12, 0, 12
	lis 0, 128
	li 11, 255
	ori 0, 0, 5
	rldic 0, 0, 12, 28
	and. 12, 12, 0
	beq	0, .Lp9_int8_BB7_9
# %bb.4:
	and. 8, 11, 7
	li 8, 3
	bne	0, .Lp9_int8_BB7_9
	b .Lp9_int8_BB7_6
.Lp9_int8_BB7_5:
	rlwinm 8, 3, 0, 30, 28
	cmpwi 1, 3, 8
	li 11, -1
	cmpwi	8, 2
	li 8, 31
	crnor 20, 2, 6
	isel 11, 11, 8, 20
	and. 8, 11, 7
	li 8, 3
	bne	0, .Lp9_int8_BB7_9
.Lp9_int8_BB7_6:
	ld 8, 144(1)
	not	11, 6
	cmpld	8, 11
	ble 0, .Lp9_int8_BB7_8
.Lp9_int8_BB7_7:
	li 8, 4
	b .Lp9_int8_BB7_9
.Lp9_int8_BB7_8:
	ld 11, 136(1)
	add 8, 8, 6
	cmpld	8, 11
	li 8, 1
	ble 0, .Lp9_int8_BB7_10
.Lp9_int8_BB7_9:
	ld 30, 80(1)                            # 8-byte Folded Reload
	ld 29, 72(1)                            # 8-byte Folded Reload
	ld 28, 64(1)                            # 8-byte Folded Reload
	ld 27, 56(1)                            # 8-byte Folded Reload
	mr	3, 8
	ld 26, 48(1)                            # 8-byte Folded Reload
	ld 25, 40(1)                            # 8-byte Folded Reload
	ld 24, 32(1)                            # 8-byte Folded Reload
	addi 1, 1, 96
	ld 0, 16(1)
	lwz 12, 8(1)
	mtlr 0
	mtocrf 32, 12
	mtocrf 16, 12
	blr
.Lp9_int8_BB7_10:
	mulhdu. 8, 11, 10
	bne	0, .Lp9_int8_BB7_7
# %bb.11:
	bgt	2, .Lp9_int8_BB7_13
# %bb.12:
	clrldi	11, 3, 32
	li 12, 1
	li 8, 256
	sld 11, 12, 11
	lis 12, 128
	ori 12, 12, 5
	rldic 12, 12, 12, 28
	and. 11, 11, 12
	bne	0, .Lp9_int8_BB7_14
.Lp9_int8_BB7_13:
	rlwinm 8, 3, 0, 30, 28
	cmpwi 1, 3, 8
	cmpwi	8, 2
	li 8, 32
	crnor 20, 6, 2
	isel 8, 0, 8, 20
.Lp9_int8_BB7_14:
	cnttzd 8, 8
	ld 29, 152(1)
	cmpwi 3, 3, 11
	srd 8, 7, 8
	ble 3, .Lp9_int8_BB7_19
# %bb.15:
	cmplwi	3, 12
	beq	0, .Lp9_int8_BB7_22
# %bb.16:
	cmplwi	3, 14
	beq	0, .Lp9_int8_BB7_23
# %bb.17:
	bne	2, .Lp9_int8_BB7_25
# %bb.18:
	li 11, 66
	b .Lp9_int8_BB7_26
.Lp9_int8_BB7_19:
	cmplwi	3, 2
	beq	0, .Lp9_int8_BB7_24
# %bb.20:
	cmplwi	3, 6
	bne	0, .Lp9_int8_BB7_25
# %bb.21:
	li 11, 22
	b .Lp9_int8_BB7_26
.Lp9_int8_BB7_22:
	li 11, 144
	b .Lp9_int8_BB7_26
.Lp9_int8_BB7_23:
	li 11, 210
	b .Lp9_int8_BB7_26
.Lp9_int8_BB7_24:
	li 11, 18
	b .Lp9_int8_BB7_26
.Lp9_int8_BB7_25:
	cmplwi	3, 8
	li 11, 0
	li 12, 34
	iseleq	11, 12, 11
.Lp9_int8_BB7_26:
	mulld 8, 11, 8
	mulhdu. 11, 8, 6
	std 8, 0(29)
	bne	0, .Lp9_int8_BB7_7
# %bb.27:
	mulld 8, 8, 6
	cmpld	8, 5
	li 8, 3
	bne	0, .Lp9_int8_BB7_9
# %bb.28:
	mr	28, 4
	mr	4, 7
	mr	5, 10
	mr	30, 3
	mr	27, 6
	mr	26, 7
	mr	25, 9
	mr	24, 10
	bl pq8_input_bytes
	nop
	cmpldi	3, 0
	li 8, 4
	beq	0, .Lp9_int8_BB7_9
# %bb.29:
	mr	5, 24
	mr	4, 26
	mr	3, 30
	bl pq8_input_bytes
	nop
	li 8, 5
	cmpld	25, 3
	blt	0, .Lp9_int8_BB7_9
# %bb.30:
	mr	5, 26
	mr	7, 28
	mr	4, 27
	bgt	2, .Lp9_int8_BB7_32
# %bb.31:
	clrldi	6, 30, 32
	li 8, 1
	li 3, 256
	sld 6, 8, 6
	lis 8, 128
	ori 8, 8, 5
	rldic 8, 8, 12, 28
	and. 6, 6, 8
	bne	0, .Lp9_int8_BB7_33
.Lp9_int8_BB7_32:
	rlwinm 3, 30, 0, 30, 28
	cmpwi 1, 30, 8
	cmpwi	3, 2
	li 3, 32
	crnor 20, 6, 2
	isel 3, 0, 3, 20
.Lp9_int8_BB7_33:
	cnttzd 3, 3
	srd 3, 5, 3
	ble 3, .Lp9_int8_BB7_38
# %bb.34:
	cmplwi	30, 12
	beq	0, .Lp9_int8_BB7_41
# %bb.35:
	cmplwi	30, 14
	beq	0, .Lp9_int8_BB7_42
# %bb.36:
	bne	2, .Lp9_int8_BB7_44
# %bb.37:
	li 5, 66
	b .Lp9_int8_BB7_45
.Lp9_int8_BB7_38:
	cmplwi	30, 2
	beq	0, .Lp9_int8_BB7_43
# %bb.39:
	cmplwi	30, 6
	bne	0, .Lp9_int8_BB7_44
# %bb.40:
	li 5, 22
	b .Lp9_int8_BB7_45
.Lp9_int8_BB7_41:
	li 5, 144
	b .Lp9_int8_BB7_45
.Lp9_int8_BB7_42:
	li 5, 210
	b .Lp9_int8_BB7_45
.Lp9_int8_BB7_43:
	li 5, 18
	b .Lp9_int8_BB7_45
.Lp9_int8_BB7_44:
	cmplwi	30, 8
	li 5, 0
	li 6, 34
	iseleq	5, 6, 5
.Lp9_int8_BB7_45:
	cmpldi	3, 0
	beq	0, .Lp9_int8_BB7_76
# %bb.46:
	ld 6, 0(29)
	cmplwi	30, 12
	bne	0, .Lp9_int8_BB7_65
# %bb.47:
	addi 7, 7, 3
	li 9, 0
.Lp9_int8_BB7_48:                               # =>This Loop Header: Depth=1
                                        #     Child Loop BB7_49 Depth 2
	mtctr 3
	mr	10, 7
.Lp9_int8_BB7_49:                               #   Parent Loop BB7_48 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lbz 8, -3(10)
	lbz 30, -2(10)
	clrlwi	12, 8, 24
	rlwinm. 0, 30, 30, 27, 31
	rlwinm 8, 30, 24, 0, 0
	mr	11, 12
	rlwimi 12, 30, 8, 22, 23
	rlwimi 11, 30, 8, 16, 23
	beq	0, .Lp9_int8_BB7_52
# %bb.50:                               #   in Loop: Header=BB7_49 Depth=2
	cmplwi	0, 31
	bne	0, .Lp9_int8_BB7_54
# %bb.51:                               #   in Loop: Header=BB7_49 Depth=2
	slwi 11, 11, 13
	or 8, 11, 8
	oris 8, 8, 32640
	b .Lp9_int8_BB7_55
.Lp9_int8_BB7_52:                               #   in Loop: Header=BB7_49 Depth=2
	cmplwi	12, 0
	beq	0, .Lp9_int8_BB7_55
# %bb.53:                               #   in Loop: Header=BB7_49 Depth=2
	cntlzw	12, 12
	xori 0, 12, 31
	slwi 12, 12, 23
	subfic 0, 0, 9
	slw 11, 11, 0
	rlwinm 11, 11, 14, 9, 17
	sub	11, 11, 12
	add 8, 11, 8
	addis 8, 8, 17152
	b .Lp9_int8_BB7_55
.Lp9_int8_BB7_54:                               #   in Loop: Header=BB7_49 Depth=2
	rlwinm 11, 12, 13, 9, 18
	rlwimi 11, 0, 23, 0, 8
	add 8, 11, 8
	addis 8, 8, 14336
.Lp9_int8_BB7_55:                               #   in Loop: Header=BB7_49 Depth=2
	mtfprd	0, 8
	li 8, 6
	xxsldwi 0, 0, 0, 1
	xscvspdpn 0, 0
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_int8_BB7_9
# %bb.56:                               #   in Loop: Header=BB7_49 Depth=2
	lbz 11, -1(10)
	lbz 29, 0(10)
	clrlwi	0, 11, 24
	rlwinm. 30, 29, 30, 27, 31
	rlwinm 11, 29, 24, 0, 0
	mr	12, 0
	rlwimi 0, 29, 8, 22, 23
	rlwimi 12, 29, 8, 16, 23
	beq	0, .Lp9_int8_BB7_59
# %bb.57:                               #   in Loop: Header=BB7_49 Depth=2
	cmplwi	30, 31
	bne	0, .Lp9_int8_BB7_61
# %bb.58:                               #   in Loop: Header=BB7_49 Depth=2
	slwi 12, 12, 13
	or 11, 12, 11
	oris 11, 11, 32640
	b .Lp9_int8_BB7_62
.Lp9_int8_BB7_59:                               #   in Loop: Header=BB7_49 Depth=2
	cmplwi	0, 0
	beq	0, .Lp9_int8_BB7_62
# %bb.60:                               #   in Loop: Header=BB7_49 Depth=2
	cntlzw	0, 0
	xori 30, 0, 31
	slwi 0, 0, 23
	subfic 30, 30, 9
	slw 12, 12, 30
	rlwinm 12, 12, 14, 9, 17
	sub	12, 12, 0
	add 11, 12, 11
	addis 11, 11, 17152
	b .Lp9_int8_BB7_62
.Lp9_int8_BB7_61:                               #   in Loop: Header=BB7_49 Depth=2
	rlwinm 12, 0, 13, 9, 18
	rlwimi 12, 30, 23, 0, 8
	add 11, 12, 11
	addis 11, 11, 14336
.Lp9_int8_BB7_62:                               #   in Loop: Header=BB7_49 Depth=2
	mtfprd	0, 11
	xxsldwi 0, 0, 0, 1
	xscvspdpn 0, 0
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_int8_BB7_9
# %bb.63:                               #   in Loop: Header=BB7_49 Depth=2
	add 10, 10, 5
	bdnz .Lp9_int8_BB7_49
# %bb.64:                               #   in Loop: Header=BB7_48 Depth=1
	addi 9, 9, 1
	add 7, 7, 6
	cmpld	9, 4
	bne	0, .Lp9_int8_BB7_48
	b .Lp9_int8_BB7_76
.Lp9_int8_BB7_65:
	cmpwi	30, 35
	li 8, 0
	li 9, 64
	cmplwi 1, 30, 14
	iseleq	8, 9, 8
	li 9, 208
	isel 8, 9, 8, 6
	add 7, 8, 7
	li 8, 0
	addi 7, 7, 1
.Lp9_int8_BB7_66:                               # =>This Loop Header: Depth=1
                                        #     Child Loop BB7_67 Depth 2
	mtctr 3
	mr	9, 7
.Lp9_int8_BB7_67:                               #   Parent Loop BB7_66 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lbz 10, -1(9)
	lbz 30, 0(9)
	clrlwi	12, 10, 24
	rlwinm. 0, 30, 30, 27, 31
	rlwinm 10, 30, 24, 0, 0
	mr	11, 12
	rlwimi 12, 30, 8, 22, 23
	rlwimi 11, 30, 8, 16, 23
	beq	0, .Lp9_int8_BB7_70
# %bb.68:                               #   in Loop: Header=BB7_67 Depth=2
	cmplwi	0, 31
	bne	0, .Lp9_int8_BB7_72
# %bb.69:                               #   in Loop: Header=BB7_67 Depth=2
	slwi 11, 11, 13
	or 10, 11, 10
	oris 10, 10, 32640
	b .Lp9_int8_BB7_73
.Lp9_int8_BB7_70:                               #   in Loop: Header=BB7_67 Depth=2
	cmplwi	12, 0
	beq	0, .Lp9_int8_BB7_73
# %bb.71:                               #   in Loop: Header=BB7_67 Depth=2
	cntlzw	12, 12
	xori 0, 12, 31
	slwi 12, 12, 23
	subfic 0, 0, 9
	slw 11, 11, 0
	rlwinm 11, 11, 14, 9, 17
	sub	11, 11, 12
	add 10, 11, 10
	addis 10, 10, 17152
	b .Lp9_int8_BB7_73
.Lp9_int8_BB7_72:                               #   in Loop: Header=BB7_67 Depth=2
	rlwinm 11, 12, 13, 9, 18
	rlwimi 11, 0, 23, 0, 8
	add 10, 11, 10
	addis 10, 10, 14336
.Lp9_int8_BB7_73:                               #   in Loop: Header=BB7_67 Depth=2
	mtfprd	0, 10
	xxsldwi 0, 0, 0, 1
	xscvspdpn 0, 0
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_int8_BB7_77
# %bb.74:                               #   in Loop: Header=BB7_67 Depth=2
	add 9, 9, 5
	bdnz .Lp9_int8_BB7_67
# %bb.75:                               #   in Loop: Header=BB7_66 Depth=1
	addi 8, 8, 1
	add 7, 7, 6
	cmpld	8, 4
	bne	0, .Lp9_int8_BB7_66
.Lp9_int8_BB7_76:
	li 8, 0
	b .Lp9_int8_BB7_9
.Lp9_int8_BB7_77:
	li 8, 6
	b .Lp9_int8_BB7_9
	.long	0
	.quad	0
.Lp9_int8_func_end7:
	.size	p9_int8_i8c_check, .Lp9_int8_func_end7-.Lp9_int8_func_begin7
	.cfi_endproc
                                        # -- End function
