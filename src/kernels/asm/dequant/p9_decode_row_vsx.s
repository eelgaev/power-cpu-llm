# dequant/p9_decode_row_vsx.s: kernel.
# Frozen Clang20 -O3 -mcpu=power9 -ffp-contract=off -std=c11 assembly.
# Retain ELFv2 TOC/localentry, constants, calls, and unwind metadata.
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lp9_dequant_CPI0_0:
	.byte	31
	.byte	31
	.byte	31
	.byte	31
	.byte	31
	.byte	31
	.byte	31
	.byte	31
	.byte	27
	.byte	27
	.byte	27
	.byte	27
	.byte	27
	.byte	27
	.byte	27
	.byte	27
.Lp9_dequant_CPI0_1:
	.byte	1
	.byte	2
	.byte	4
	.byte	8
	.byte	16
	.byte	32
	.byte	64
	.byte	128
	.byte	1
	.byte	2
	.byte	4
	.byte	8
	.byte	16
	.byte	32
	.byte	64
	.byte	128
.Lp9_dequant_CPI0_2:
	.long	16
	.long	24
	.space	4
	.space	4
	.text
	.globl	p9_decode_row_vsx
	.p2align	6
	.type	p9_decode_row_vsx,@function
p9_decode_row_vsx:
.Lp9_dequant_func_begin0:
	.cfi_startproc
.Lp9_dequant_func_gep0:
	addis 2, 12, .TOC.-.Lp9_dequant_func_gep0@ha
	addi 2, 2, .TOC.-.Lp9_dequant_func_gep0@l
.Lp9_dequant_func_lep0:
	.localentry	p9_decode_row_vsx, .Lp9_dequant_func_lep0-.Lp9_dequant_func_gep0
	mflr 0
	.cfi_def_cfa_offset 144
	.cfi_offset lr, 16
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
	std 18, -112(1)
	std 19, -104(1)
	std 20, -96(1)
	std 21, -88(1)
	std 22, -80(1)
	std 23, -72(1)
	std 24, -64(1)
	std 25, -56(1)
	std 26, -48(1)
	std 27, -40(1)
	std 28, -32(1)
	std 29, -24(1)
	std 30, -16(1)
	stdu 1, -144(1)
	mr	8, 3
	li 3, 0
	std 0, 160(1)
	cmpwi	8, 7
	bgt	0, .Lp9_dequant_BB0_4
	cmpwi	8, 2
	bge 0, .Lp9_dequant_BB0_8
	cmplwi	8, 0
	beq	0, .Lp9_dequant_BB0_14
	cmplwi	8, 1
	beq	0, .Lp9_dequant_BB0_13
	b .Lp9_dequant_BB0_32
.Lp9_dequant_BB0_4:
	cmpwi	8, 14
	bge 0, .Lp9_dequant_BB0_11
	cmplwi	8, 8
	beq	0, .Lp9_dequant_BB0_15
	cmplwi	8, 13                           # hand edit: Q5_K -> portable
	beq	0, .Lp9_dequant_BB0_13
	cmplwi	8, 12
	bne	0, .Lp9_dequant_BB0_32
	li 11, 256
	li 9, 144
	andi. 10, 7, 255
	bne	0, .Lp9_dequant_BB0_32
	b .Lp9_dequant_BB0_18
.Lp9_dequant_BB0_8:
	beq	0, .Lp9_dequant_BB0_16
	cmplwi	8, 7                            # hand edit: Q5_1 -> portable
	beq	0, .Lp9_dequant_BB0_13
	cmplwi	8, 6
	bne	0, .Lp9_dequant_BB0_32
	li 11, 32
	li 9, 22
	andi. 10, 7, 31
	bne	0, .Lp9_dequant_BB0_32
	b .Lp9_dequant_BB0_18
.Lp9_dequant_BB0_11:
	beq	0, .Lp9_dequant_BB0_17
	cmplwi	8, 30                           # hand edit: BF16 -> portable
	beq	0, .Lp9_dequant_BB0_13
	cmplwi	8, 35
	bne	0, .Lp9_dequant_BB0_32
.Lp9_dequant_BB0_13:
	mr	3, 8
	bl p9_decode_row
	nop
	b .Lp9_dequant_BB0_32
.Lp9_dequant_BB0_14:
	li 11, 1
	li 9, 4
	andi. 10, 7, 0
	bne	0, .Lp9_dequant_BB0_32
	b .Lp9_dequant_BB0_18
.Lp9_dequant_BB0_15:
	li 11, 32
	li 9, 34
	andi. 10, 7, 31
	bne	0, .Lp9_dequant_BB0_32
	b .Lp9_dequant_BB0_18
.Lp9_dequant_BB0_16:
	li 11, 32
	li 9, 18
	andi. 10, 7, 31
	bne	0, .Lp9_dequant_BB0_32
	b .Lp9_dequant_BB0_18
.Lp9_dequant_BB0_17:
	li 11, 256
	li 9, 210
	andi. 10, 7, 255
	bne	0, .Lp9_dequant_BB0_32
.Lp9_dequant_BB0_18:
	cnttzd 10, 11
	srd 10, 7, 10
	mulld 0, 10, 9
	mulhdu 12, 9, 10
	xor 5, 0, 5
	or. 5, 5, 12
	bne	0, .Lp9_dequant_BB0_32
	cmplwi	8, 0
	beq	0, .Lp9_dequant_BB0_31
	li 3, 1
	cmpldi	10, 0
	beq	0, .Lp9_dequant_BB0_32
	addi 5, 8, -2
	rotlwi	5, 5, 31
	cmpwi	5, 3
	bge 0, .Lp9_dequant_BB0_33
	cmplwi	5, 0
	beq	0, .Lp9_dequant_BB0_61
	cmplwi	5, 2
	bne	0, .Lp9_dequant_BB0_49
	addis 8, 2, .Lp9_dequant_CPI0_0@toc@ha
	mtctr 10
	addi 5, 6, 64
	sldi 6, 11, 2
	li 7, 6
	addi 8, 8, .Lp9_dequant_CPI0_0@toc@l
	lxv 0, 0(8)
	addis 8, 2, .Lp9_dequant_CPI0_1@toc@ha
	addi 8, 8, .Lp9_dequant_CPI0_1@toc@l
	lxv 34, 0(8)
	addis 8, 2, .Lp9_dequant_CPI0_2@toc@ha
	addi 8, 8, .Lp9_dequant_CPI0_2@toc@l
	lxv 35, 0(8)
	b .Lp9_dequant_BB0_27
.Lp9_dequant_BB0_25:
	cmplwi	11, 0
	cntlzw	11, 11
	xori 12, 11, 31
	slwi 11, 11, 23
	subfic 12, 12, 9
	slw 10, 10, 12
	rlwinm 10, 10, 14, 9, 17
	sub	10, 10, 11
	add 10, 10, 8
	addis 10, 10, 17152
	iseleq	8, 8, 10
.Lp9_dequant_BB0_26:
	lwz 10, 2(4)
	lxvx 37, 4, 7
	add 4, 4, 9
	mtfprwz	1, 10
	srwi 10, 10, 8
	mtfprwz	2, 10
	xxspltw 36, 1, 1
	xxmrghw 1, 2, 1
	vsrw 4, 4, 3
	xxperm 1, 1, 0
	xxperm 36, 36, 0
	xxland 32, 1, 34
	xxland 36, 36, 34
	xxspltib 1, 15
	xxspltib 33, 4
	xxspltib 2, 16
	vcmpequb 0, 0, 2
	vcmpequb 4, 4, 2
	xxland 1, 37, 1
	vsrb 5, 5, 1
	xxland 3, 32, 2
	xxland 2, 36, 2
	xxspltib 36, 240
	xxlor 32, 3, 1
	xxlor 37, 2, 37
	mtvsrws 1, 8
	vaddubm 0, 0, 4
	vaddubm 4, 5, 4
	vupklsb 5, 0
	vupklsb 1, 4
	vupkhsb 0, 0
	vupkhsb 4, 4
	vupklsh 6, 5
	vupkhsh 5, 5
	vupklsh 7, 0
	vupkhsh 0, 0
	vcfsx 6, 6, 0
	vcfsx 5, 5, 0
	vcfsx 7, 7, 0
	vcfsx 0, 0, 0
	xvmulsp 2, 1, 38
	vupklsh 6, 1
	vupkhsh 1, 1
	xvmulsp 3, 1, 37
	vupklsh 5, 4
	vupkhsh 4, 4
	xvmulsp 4, 1, 39
	xvmulsp 5, 1, 32
	vcfsx 1, 1, 0
	vcfsx 6, 6, 0
	vcfsx 5, 5, 0
	vcfsx 4, 4, 0
	stxv 2, -64(5)
	stxv 3, -48(5)
	stxv 4, -32(5)
	stxv 5, -16(5)
	xvmulsp 2, 1, 33
	xvmulsp 6, 1, 38
	stxv 2, 16(5)
	stxv 6, 0(5)
	xvmulsp 2, 1, 37
	xvmulsp 1, 1, 36
	stxv 2, 32(5)
	stxv 1, 48(5)
	add 5, 5, 6
	bdz .Lp9_dequant_BB0_32
.Lp9_dequant_BB0_27:
	lbz 8, 0(4)
	lbz 0, 1(4)
	clrlwi	11, 8, 24
	rlwinm. 12, 0, 30, 27, 31
	rlwinm 8, 0, 24, 0, 0
	mr	10, 11
	rlwimi 11, 0, 8, 22, 23
	rlwimi 10, 0, 8, 16, 23
	beq	0, .Lp9_dequant_BB0_25
	cmplwi	12, 31
	bne	0, .Lp9_dequant_BB0_30
	slwi 10, 10, 13
	or 8, 10, 8
	oris 8, 8, 32640
	b .Lp9_dequant_BB0_26
.Lp9_dequant_BB0_30:
	rlwinm 10, 11, 13, 9, 18
	rlwimi 10, 12, 23, 0, 8
	add 8, 10, 8
	addis 8, 8, 14336
	b .Lp9_dequant_BB0_26
.Lp9_dequant_BB0_31:
	sldi 5, 7, 2
	mr	3, 6
	bl memcpy
	nop
	li 3, 1
.Lp9_dequant_BB0_32:
	addi 1, 1, 144
	ld 0, 16(1)
	ld 30, -16(1)
	ld 29, -24(1)
	ld 28, -32(1)
	mtlr 0
	ld 27, -40(1)
	ld 26, -48(1)
	ld 25, -56(1)
	ld 24, -64(1)
	ld 23, -72(1)
	ld 22, -80(1)
	ld 21, -88(1)
	ld 20, -96(1)
	ld 19, -104(1)
	ld 18, -112(1)
	blr
.Lp9_dequant_BB0_33:
	beq	0, .Lp9_dequant_BB0_63
	cmplwi	5, 5
	bne	0, .Lp9_dequant_BB0_49
	addi 5, 4, 8
	addi 7, 6, 64
	sldi 8, 11, 2
	li 11, 0
	b .Lp9_dequant_BB0_37
	.p2align	4
.Lp9_dequant_BB0_36:
	addi 11, 11, 1
	add 5, 5, 9
	add 7, 7, 8
	cmpld	11, 10
	beq	0, .Lp9_dequant_BB0_32
.Lp9_dequant_BB0_37:
	mulld 12, 9, 11
	mr	30, 4
	lbzux 12, 30, 12
	lbz 27, 1(30)
	clrlwi	29, 12, 24
	mr	12, 29
	rlwinm 0, 27, 24, 0, 0
	rlwinm. 28, 27, 30, 27, 31
	rlwimi 29, 27, 8, 22, 23
	rlwimi 12, 27, 8, 16, 23
	beq	0, .Lp9_dequant_BB0_39
	rlwinm 29, 29, 13, 9, 18
	slwi 12, 12, 13
	cmplwi	28, 31
	or 12, 12, 0
	rlwimi 29, 28, 23, 0, 8
	add 29, 29, 0
	oris 12, 12, 32640
	addis 29, 29, 14336
	iseleq	0, 12, 29
	b .Lp9_dequant_BB0_41
.Lp9_dequant_BB0_39:
	cmplwi	29, 0
	beq	0, .Lp9_dequant_BB0_41
	cntlzw	29, 29
	xori 28, 29, 31
	slwi 29, 29, 23
	subfic 28, 28, 9
	slw 12, 12, 28
	rlwinm 12, 12, 14, 9, 17
	sub	12, 12, 29
	add 12, 12, 0
	addis 0, 12, 17152
.Lp9_dequant_BB0_41:
	mtfprd	0, 0
	lbz 0, 2(30)
	lbz 29, 3(30)
	maddld 12, 8, 11, 6
	addi 26, 30, 7
	addi 30, 30, 16
	xxsldwi 0, 0, 0, 1
	clrlwi	0, 0, 24
	rlwinm 27, 29, 24, 0, 0
	rlwinm. 25, 29, 30, 27, 31
	xscvspdpn 0, 0
	mr	28, 0
	rlwimi 0, 29, 8, 22, 23
	rlwimi 28, 29, 8, 16, 23
	cntlzw	29, 0
	cmplwi 1, 0, 0
	rlwinm 0, 0, 13, 9, 18
	rlwimi 0, 25, 23, 0, 8
	xori 24, 29, 31
	slwi 29, 29, 23
	subfic 24, 24, 9
	slw 24, 28, 24
	slwi 28, 28, 13
	rlwinm 24, 24, 14, 9, 17
	or 28, 28, 27
	sub	29, 24, 29
	oris 28, 28, 32640
	add 29, 29, 27
	addis 29, 29, 17152
	isel 29, 27, 29, 6
	cmplwi 1, 25, 31
	add 25, 0, 27
	addis 0, 25, 14336
	isel 0, 28, 0, 6
	li 28, 0
	iseleq	0, 29, 0
	li 29, 8
	mtfprd	1, 0
	mtctr 29
	xxsldwi 1, 1, 1, 1
	xscvspdpn 1, 1
	b .Lp9_dequant_BB0_44
	.p2align	5
.Lp9_dequant_BB0_42:
	xxspltib 4, 15
	xxland 34, 34, 4
	vupklsb 3, 2
	vupkhsb 2, 2
	vupklsh 4, 3
	vupkhsh 3, 3
	vupklsh 5, 2
	vupkhsh 2, 2
	vcfsx 4, 4, 0
	vcfsx 3, 3, 0
	vcfsx 2, 2, 0
	xvmulsp 5, 2, 36
	xvsubsp 5, 5, 3
	stxv 5, 0(12)
	xvmulsp 5, 2, 35
	vcfsx 3, 5, 0
	xvsubsp 5, 5, 3
	stxv 5, 16(12)
	xvmulsp 5, 2, 35
	xvsubsp 5, 5, 3
	stxv 5, 32(12)
	xvmulsp 5, 2, 34
	xvsubsp 5, 5, 3
	stxv 5, 48(12)
	lxv 5, 16(26)
	xxland 34, 5, 4
.Lp9_dequant_BB0_43:
	vupklsb 3, 2
	vupkhsb 2, 2
	addi 28, 28, 16
	addi 29, 29, 1
	mr	26, 27
	vupklsh 4, 3
	vupkhsh 3, 3
	vupklsh 5, 2
	vupkhsh 2, 2
	vcfsx 4, 4, 0
	vcfsx 3, 3, 0
	vcfsx 5, 5, 0
	vcfsx 2, 2, 0
	xvmulsp 4, 2, 36
	xvmulsp 5, 2, 35
	xvsubsp 4, 4, 3
	xvsubsp 5, 5, 3
	stxv 4, 64(12)
	xvmulsp 4, 2, 37
	xvmulsp 2, 2, 34
	stxv 5, 80(12)
	xvsubsp 4, 4, 3
	xvsubsp 2, 2, 3
	stxv 4, 96(12)
	stxv 2, 112(12)
	addi 12, 12, 128
	bdz .Lp9_dequant_BB0_36
.Lp9_dequant_BB0_44:
	addi 0, 29, -8
	addi 27, 26, 1
	cmpldi	0, 4
	bge 0, .Lp9_dequant_BB0_46
	lbz 25, -3(26)
	lbz 26, 1(26)
	clrlwi	25, 25, 26
	clrlwi	26, 26, 26
	b .Lp9_dequant_BB0_47
	.p2align	5
.Lp9_dequant_BB0_46:
	lbz 24, 1(26)
	lbz 23, -7(26)
	clrlwi	25, 24, 28
	rlwimi 25, 23, 30, 2, 27
	lbz 23, -3(26)
	rlwinm 26, 24, 28, 28, 31
	rlwimi 26, 23, 30, 2, 27
.Lp9_dequant_BB0_47:
	mtfprwz	2, 25
	mtfprwz	3, 26
	rlwinm 25, 28, 0, 25, 26
	andi. 0, 0, 1
	xscvuxdsp 2, 2
	xscvuxdsp 3, 3
	lxvx 34, 30, 25
	add 26, 30, 25
	xsmulsp 2, 0, 2
	xsmulsp 3, 1, 3
	xscvdpspn 2, 2
	xscvdpspn 3, 3
	xxspltw 2, 2, 0
	xxspltw 3, 3, 0
	bc 4, 1, .Lp9_dequant_BB0_42
	xxspltib 35, 4
	vsrb 2, 2, 3
	vupklsb 4, 2
	vupkhsb 2, 2
	vupklsh 5, 4
	vupkhsh 4, 4
	vupklsh 0, 2
	vupkhsh 2, 2
	vcfsx 5, 5, 0
	vcfsx 4, 4, 0
	vcfsx 2, 2, 0
	xvmulsp 4, 2, 37
	xvsubsp 4, 4, 3
	stxv 4, 0(12)
	xvmulsp 4, 2, 36
	vcfsx 4, 0, 0
	xvsubsp 4, 4, 3
	stxv 4, 16(12)
	xvmulsp 4, 2, 36
	xvsubsp 4, 4, 3
	stxv 4, 32(12)
	xvmulsp 4, 2, 34
	xvsubsp 4, 4, 3
	stxv 4, 48(12)
	lxv 34, 16(26)
	vsrb 2, 2, 3
	b .Lp9_dequant_BB0_43
.Lp9_dequant_BB0_49:
	addi 5, 6, 64
	addi 8, 4, 193
	addi 6, 6, 624
	sldi 7, 11, 2
	addi 11, 4, 201
	li 12, 0
	li 0, 4
	li 30, 0
	li 29, 4
	li 28, 2
	li 27, 3
	.p2align	4
.Lp9_dequant_BB0_50:
	maddld 26, 9, 12, 4
	lbz 25, 208(26)
	lbz 21, 209(26)
	clrlwi	23, 25, 24
	rlwinm 22, 21, 30, 27, 31
	rlwinm 24, 21, 24, 0, 0
	mr	25, 23
	cmplwi	22, 31
	rlwimi 25, 21, 8, 16, 23
	beq	0, .Lp9_dequant_BB0_54
	rlwimi 23, 21, 8, 22, 23
	cmplwi	22, 0
	bne	0, .Lp9_dequant_BB0_55
	cmplwi	23, 0
	beq	0, .Lp9_dequant_BB0_56
	cntlzw	23, 23
	xori 22, 23, 31
	slwi 23, 23, 23
	subfic 22, 22, 9
	slw 25, 25, 22
	rlwinm 25, 25, 14, 9, 17
	sub	25, 25, 23
	add 25, 25, 24
	addis 24, 25, 17152
	b .Lp9_dequant_BB0_56
.Lp9_dequant_BB0_54:
	slwi 25, 25, 13
	or 25, 25, 24
	oris 24, 25, 32640
	b .Lp9_dequant_BB0_56
.Lp9_dequant_BB0_55:
	rlwinm 25, 23, 13, 9, 18
	rlwimi 25, 22, 23, 0, 8
	add 25, 25, 24
	addis 24, 25, 14336
.Lp9_dequant_BB0_56:
	mtfprd	0, 24
	mtctr 0
	addi 25, 26, 198
	addi 20, 26, 190
	li 24, 0
	li 23, 0
	mr	22, 5
	li 21, 0
	xxsldwi 0, 0, 0, 1
	xscvspdpn 0, 0
	.p2align	5
.Lp9_dequant_BB0_57:
	lxsibzx 36, 20, 28
	cmpldi	21, 1
	rlwinm 19, 24, 0, 26, 26
	addi 21, 21, 1
	addi 24, 24, 32
	vextsb2d 4, 4
	iselgt	18, 29, 30
	mtvsrd 34, 18
	lxvx 37, 26, 19
	rlwinm 18, 23, 0, 29, 30
	xxspltib 2, 15
	add 19, 26, 19
	addi 23, 23, 2
	vspltb 2, 2, 7
	mtvsrd 35, 18
	xscvsxdsp 1, 36
	vspltb 3, 3, 7
	vsrb 4, 5, 2
	lxv 37, 128(26)
	xxland 3, 36, 2
	xxspltib 36, 4
	xxspltib 4, 48
	xxspltib 32, 224
	xsmulsp 1, 0, 1
	vsrb 5, 5, 3
	vslb 5, 5, 4
	xscvdpspn 1, 1
	xxland 5, 37, 4
	xxlor 37, 5, 3
	vaddubm 5, 5, 0
	vupklsb 1, 5
	vupkhsb 5, 5
	vupklsh 6, 1
	vupkhsh 1, 1
	xxspltw 1, 1, 0
	vupklsh 7, 5
	vupkhsh 5, 5
	vcfsx 6, 6, 0
	vcfsx 1, 1, 0
	vcfsx 5, 5, 0
	xvmulsp 3, 1, 38
	stxv 3, -64(22)
	xvmulsp 3, 1, 33
	vcfsx 1, 7, 0
	stxv 3, -48(22)
	xvmulsp 3, 1, 33
	stxv 3, -32(22)
	xvmulsp 1, 1, 37
	stxv 1, -16(22)
	lxsibzx 37, 20, 27
	addi 20, 20, 2
	vextsb2d 5, 5
	lxv 33, 16(19)
	xscvsxdsp 1, 37
	lxv 37, 144(26)
	vsrb 2, 1, 2
	vsrb 3, 5, 3
	xxland 2, 34, 2
	vslb 3, 3, 4
	xsmulsp 1, 0, 1
	xxland 3, 35, 4
	xxlor 34, 3, 2
	xscvdpspn 1, 1
	vaddubm 2, 2, 0
	vupklsb 3, 2
	vupkhsb 2, 2
	vupklsh 4, 3
	vupkhsh 3, 3
	vupklsh 5, 2
	vupkhsh 2, 2
	vcfsx 4, 4, 0
	vcfsx 3, 3, 0
	xxspltw 1, 1, 0
	vcfsx 2, 2, 0
	xvmulsp 2, 1, 36
	xvmulsp 3, 1, 35
	vcfsx 3, 5, 0
	stxv 2, 0(22)
	stxv 3, 16(22)
	xvmulsp 2, 1, 35
	xvmulsp 1, 1, 34
	stxv 2, 32(22)
	stxv 1, 48(22)
	addi 22, 22, 128
	bdnz .Lp9_dequant_BB0_57
	mtctr 0
	addi 24, 26, 64
	li 23, 0
	mr	22, 6
	li 21, 0
	li 20, 0
	.p2align	5
.Lp9_dequant_BB0_59:
	lxsibzx 36, 25, 28
	cmpldi	20, 1
	rlwinm 19, 23, 0, 26, 26
	addi 20, 20, 1
	addi 23, 23, 32
	vextsb2d 4, 4
	iselgt	18, 29, 30
	mtvsrd 34, 18
	lxvx 37, 24, 19
	rlwinm 18, 21, 0, 29, 30
	xxspltib 2, 15
	add 19, 24, 19
	addi 21, 21, 2
	vspltb 2, 2, 7
	mtvsrd 35, 18
	xscvsxdsp 1, 36
	vspltb 3, 3, 7
	vsrb 4, 5, 2
	lxv 37, 160(26)
	xxland 3, 36, 2
	xxspltib 36, 4
	xxspltib 4, 48
	xxspltib 32, 224
	xsmulsp 1, 0, 1
	vsrb 5, 5, 3
	vslb 5, 5, 4
	xscvdpspn 1, 1
	xxland 5, 37, 4
	xxlor 37, 5, 3
	vaddubm 5, 5, 0
	vupklsb 1, 5
	vupkhsb 5, 5
	vupklsh 6, 1
	vupkhsh 1, 1
	xxspltw 1, 1, 0
	vupklsh 7, 5
	vupkhsh 5, 5
	vcfsx 6, 6, 0
	vcfsx 1, 1, 0
	vcfsx 5, 5, 0
	xvmulsp 3, 1, 38
	stxv 3, -112(22)
	xvmulsp 3, 1, 33
	vcfsx 1, 7, 0
	stxv 3, -96(22)
	xvmulsp 3, 1, 33
	stxv 3, -80(22)
	xvmulsp 1, 1, 37
	stxv 1, -64(22)
	lxsibzx 37, 25, 27
	addi 25, 25, 2
	vextsb2d 5, 5
	lxv 33, 16(19)
	xscvsxdsp 1, 37
	lxv 37, 176(26)
	vsrb 2, 1, 2
	vsrb 3, 5, 3
	xxland 2, 34, 2
	vslb 3, 3, 4
	xsmulsp 1, 0, 1
	xxland 3, 35, 4
	xxlor 34, 3, 2
	xscvdpspn 1, 1
	vaddubm 2, 2, 0
	vupklsb 3, 2
	vupkhsb 2, 2
	vupklsh 4, 3
	vupkhsh 3, 3
	vupklsh 5, 2
	vupkhsh 2, 2
	vcfsx 4, 4, 0
	vcfsx 3, 3, 0
	xxspltw 1, 1, 0
	vcfsx 2, 2, 0
	xvmulsp 2, 1, 36
	xvmulsp 3, 1, 35
	vcfsx 3, 5, 0
	stxv 2, -48(22)
	stxv 3, -32(22)
	xvmulsp 2, 1, 35
	xvmulsp 1, 1, 34
	stxv 2, -16(22)
	stxv 1, 0(22)
	addi 22, 22, 128
	bdnz .Lp9_dequant_BB0_59
	addi 12, 12, 1
	add 5, 5, 7
	add 8, 8, 9
	add 11, 11, 9
	add 6, 6, 7
	cmpld	12, 10
	bne	0, .Lp9_dequant_BB0_50
	b .Lp9_dequant_BB0_32
.Lp9_dequant_BB0_61:
	mtctr 10
	addi 4, 4, 1
	addi 5, 6, 64
	sldi 6, 11, 2
	.p2align	4
.Lp9_dequant_BB0_62:
	lbz 3, -1(4)
	lbz 7, 0(4)
	clrlwi	3, 3, 24
	rlwinm. 11, 7, 30, 27, 31
	rlwinm 10, 7, 24, 0, 0
	mr	8, 3
	rlwimi 3, 7, 8, 22, 23
	rlwimi 8, 7, 8, 16, 23
	rlwinm 7, 3, 13, 9, 18
	cmplwi 5, 11, 31
	cmplwi 1, 3, 0
	cntlzw	3, 3
	rlwimi 7, 11, 23, 0, 8
	slwi 11, 8, 13
	add 7, 7, 10
	or 11, 11, 10
	addis 7, 7, 14336
	oris 11, 11, 32640
	isel 7, 11, 7, 22
	xori 11, 3, 31
	slwi 3, 3, 23
	subfic 11, 11, 9
	slw 8, 8, 11
	rlwinm 8, 8, 14, 9, 17
	sub	3, 8, 3
	add 3, 3, 10
	addis 8, 3, 17152
	li 3, 1
	lxvx 34, 4, 3
	xxspltib 0, 15
	xxspltib 36, 248
	isel 8, 10, 8, 6
	add 4, 4, 9
	iseleq	7, 8, 7
	xxland 35, 34, 0
	mtvsrws 0, 7
	vaddubm 3, 3, 4
	vupklsb 5, 3
	vupkhsb 3, 3
	vupklsh 0, 5
	vupkhsh 5, 5
	vupklsh 1, 3
	vupkhsh 3, 3
	vcfsx 0, 0, 0
	vcfsx 5, 5, 0
	vcfsx 3, 3, 0
	xvmulsp 1, 0, 32
	xvmulsp 2, 0, 37
	vcfsx 5, 1, 0
	xvmulsp 3, 0, 37
	stxv 1, -64(5)
	xvmulsp 1, 0, 35
	stxv 2, -48(5)
	stxv 3, -32(5)
	stxv 1, -16(5)
	xxspltib 35, 4
	vsrb 2, 2, 3
	vaddubm 2, 2, 4
	vupklsb 3, 2
	vupkhsb 2, 2
	vupklsh 4, 3
	vupkhsh 3, 3
	vupklsh 5, 2
	vupkhsh 2, 2
	vcfsx 4, 4, 0
	vcfsx 3, 3, 0
	vcfsx 2, 2, 0
	xvmulsp 1, 0, 36
	stxv 1, 0(5)
	xvmulsp 1, 0, 35
	vcfsx 3, 5, 0
	xvmulsp 2, 0, 35
	xvmulsp 0, 0, 34
	stxv 1, 16(5)
	stxv 2, 32(5)
	stxv 0, 48(5)
	add 5, 5, 6
	bdnz .Lp9_dequant_BB0_62
	b .Lp9_dequant_BB0_32
.Lp9_dequant_BB0_63:
	mtctr 10
	addi 5, 6, 64
	sldi 6, 11, 2
	li 7, 2
	li 8, 18
	b .Lp9_dequant_BB0_66
	.p2align	4
.Lp9_dequant_BB0_64:
	rlwinm 12, 12, 13, 9, 18
	slwi 11, 11, 13
	cmplwi	0, 31
	rlwimi 12, 0, 23, 0, 8
	add 12, 12, 10
	or 10, 11, 10
	addis 12, 12, 14336
	oris 10, 10, 32640
	iseleq	10, 10, 12
.Lp9_dequant_BB0_65:
	lxvx 34, 4, 7
	mtfprwz	0, 10
	xxspltw 0, 0, 1
	vupklsb 3, 2
	vupkhsb 2, 2
	vupklsh 4, 3
	vupkhsh 3, 3
	vupklsh 5, 2
	vupkhsh 2, 2
	vcfsx 4, 4, 0
	vcfsx 3, 3, 0
	vcfsx 2, 2, 0
	xvmulsp 1, 0, 36
	stxv 1, -64(5)
	xvmulsp 1, 0, 35
	vcfsx 3, 5, 0
	stxv 1, -48(5)
	xvmulsp 1, 0, 35
	stxv 1, -32(5)
	xvmulsp 1, 0, 34
	stxv 1, -16(5)
	lxvx 34, 4, 8
	add 4, 4, 9
	vupklsb 3, 2
	vupkhsb 2, 2
	vupklsh 4, 3
	vupkhsh 3, 3
	vupklsh 5, 2
	vupkhsh 2, 2
	vcfsx 4, 4, 0
	vcfsx 3, 3, 0
	vcfsx 2, 2, 0
	xvmulsp 1, 0, 36
	stxv 1, 0(5)
	xvmulsp 1, 0, 35
	vcfsx 3, 5, 0
	stxv 1, 16(5)
	xvmulsp 1, 0, 35
	stxv 1, 32(5)
	xvmulsp 0, 0, 34
	stxv 0, 48(5)
	add 5, 5, 6
	bdz .Lp9_dequant_BB0_32
.Lp9_dequant_BB0_66:
	lbz 10, 0(4)
	lbz 30, 1(4)
	clrlwi	12, 10, 24
	rlwinm. 0, 30, 30, 27, 31
	rlwinm 10, 30, 24, 0, 0
	mr	11, 12
	rlwimi 12, 30, 8, 22, 23
	rlwimi 11, 30, 8, 16, 23
	bne	0, .Lp9_dequant_BB0_64
	cmplwi	12, 0
	beq	0, .Lp9_dequant_BB0_65
	cntlzw	12, 12
	xori 0, 12, 31
	slwi 12, 12, 23
	subfic 0, 0, 9
	slw 11, 11, 0
	rlwinm 11, 11, 14, 9, 17
	sub	11, 11, 12
	add 10, 11, 10
	addis 10, 10, 17152
	b .Lp9_dequant_BB0_65
	.long	0
	.quad	0
.Lp9_dequant_func_end0:
	.size	p9_decode_row_vsx, .Lp9_dequant_func_end0-.Lp9_dequant_func_begin0
	.cfi_endproc
