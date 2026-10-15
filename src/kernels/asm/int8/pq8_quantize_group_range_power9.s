# int8/pq8_quantize_group_range_power9.s: kernel.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_int8.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function pq8_quantize_group_range_power9
.Lp9_int8_CPI10_0:
	.long	0x42fe0000                      # float 127
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lp9_int8_CPI10_1:
	.long	0x4b400000                      # float 12582912
	.long	0x4b400000                      # float 12582912
	.long	0x4b400000                      # float 12582912
	.long	0x4b400000                      # float 12582912
.Lp9_int8_CPI10_2:
	.long	0xcb400000                      # float -12582912
	.long	0xcb400000                      # float -12582912
	.long	0xcb400000                      # float -12582912
	.long	0xcb400000                      # float -12582912
.Lp9_int8_CPI10_3:
	.long	0xc2fe0000                      # float -127
	.long	0xc2fe0000                      # float -127
	.long	0xc2fe0000                      # float -127
	.long	0xc2fe0000                      # float -127
.Lp9_int8_CPI10_4:
	.long	0x42fe0000                      # float 127
	.long	0x42fe0000                      # float 127
	.long	0x42fe0000                      # float 127
	.long	0x42fe0000                      # float 127
.Lp9_int8_CPI10_5:
	.byte	31                              # 0x1f
	.byte	27                              # 0x1b
	.byte	23                              # 0x17
	.byte	19                              # 0x13
	.byte	15                              # 0xf
	.byte	11                              # 0xb
	.byte	7                               # 0x7
	.byte	3                               # 0x3
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
.Lp9_int8_CPI10_6:
	.byte	31                              # 0x1f
	.byte	30                              # 0x1e
	.byte	29                              # 0x1d
	.byte	28                              # 0x1c
	.byte	27                              # 0x1b
	.byte	26                              # 0x1a
	.byte	25                              # 0x19
	.byte	24                              # 0x18
	.byte	15                              # 0xf
	.byte	11                              # 0xb
	.byte	7                               # 0x7
	.byte	3                               # 0x3
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
.Lp9_int8_CPI10_7:
	.byte	31                              # 0x1f
	.byte	30                              # 0x1e
	.byte	29                              # 0x1d
	.byte	28                              # 0x1c
	.byte	27                              # 0x1b
	.byte	26                              # 0x1a
	.byte	25                              # 0x19
	.byte	24                              # 0x18
	.byte	23                              # 0x17
	.byte	22                              # 0x16
	.byte	21                              # 0x15
	.byte	20                              # 0x14
	.byte	15                              # 0xf
	.byte	11                              # 0xb
	.byte	7                               # 0x7
	.byte	3                               # 0x3
	.section	.text.pq8_quantize_group_range_power9,"ax",@progbits
	.globl	pq8_quantize_group_range_power9
	.p2align 6
	.type	pq8_quantize_group_range_power9,@function
pq8_quantize_group_range_power9:      # @pq8_quantize_group_range_power9
.Lp9_int8_func_begin10:
	.cfi_startproc
.Lp9_int8_func_gep10:
	addis 2, 12, .TOC.-.Lp9_int8_func_gep10@ha
	addi 2, 2, .TOC.-.Lp9_int8_func_gep10@l
.Lp9_int8_func_lep10:
	.localentry	pq8_quantize_group_range_power9, .Lp9_int8_func_lep10-.Lp9_int8_func_gep10
# %bb.0:
	mfcr 12
	mflr 0
	stw 12, 8(1)
	stdu 1, -464(1)
	std 0, 480(1)
	.cfi_def_cfa_offset 464
	.cfi_offset lr, 16
	.cfi_offset r14, -152
	.cfi_offset r15, -144
	.cfi_offset r16, -136
	.cfi_offset r17, -128
	.cfi_offset r18, -120
	.cfi_offset r19, -112
	.cfi_offset r20, -104
	.cfi_offset r21, -96
	.cfi_offset r22, -88
	.cfi_offset r23, -80
	.cfi_offset r24, -72
	.cfi_offset r25, -64
	.cfi_offset r26, -56
	.cfi_offset r27, -48
	.cfi_offset r28, -40
	.cfi_offset r29, -32
	.cfi_offset r30, -24
	.cfi_offset r31, -16
	.cfi_offset f31, -8
	.cfi_offset cr2, 8
	.cfi_offset cr3, 8
	.cfi_offset v23, -304
	.cfi_offset v24, -288
	.cfi_offset v25, -272
	.cfi_offset v26, -256
	.cfi_offset v27, -240
	.cfi_offset v28, -224
	.cfi_offset v29, -208
	.cfi_offset v30, -192
	.cfi_offset v31, -176
	std 28, 424(1)                          # 8-byte Folded Spill
	mr	28, 9
	li 9, 2
	cmplwi	3, 35
	stxv 55, 160(1)                         # 16-byte Folded Spill
	std 14, 312(1)                          # 8-byte Folded Spill
	std 15, 320(1)                          # 8-byte Folded Spill
	stxv 56, 176(1)                         # 16-byte Folded Spill
	std 16, 328(1)                          # 8-byte Folded Spill
	stxv 57, 192(1)                         # 16-byte Folded Spill
	std 17, 336(1)                          # 8-byte Folded Spill
	stxv 58, 208(1)                         # 16-byte Folded Spill
	std 18, 344(1)                          # 8-byte Folded Spill
	std 19, 352(1)                          # 8-byte Folded Spill
	stxv 59, 224(1)                         # 16-byte Folded Spill
	std 20, 360(1)                          # 8-byte Folded Spill
	stxv 60, 240(1)                         # 16-byte Folded Spill
	std 21, 368(1)                          # 8-byte Folded Spill
	stxv 61, 256(1)                         # 16-byte Folded Spill
	std 22, 376(1)                          # 8-byte Folded Spill
	std 23, 384(1)                          # 8-byte Folded Spill
	stxv 62, 272(1)                         # 16-byte Folded Spill
	std 24, 392(1)                          # 8-byte Folded Spill
	stxv 63, 288(1)                         # 16-byte Folded Spill
	std 25, 400(1)                          # 8-byte Folded Spill
	std 26, 408(1)                          # 8-byte Folded Spill
	std 27, 416(1)                          # 8-byte Folded Spill
	std 29, 432(1)                          # 8-byte Folded Spill
	std 30, 440(1)                          # 8-byte Folded Spill
	std 31, 448(1)                          # 8-byte Folded Spill
	stfd 31, 456(1)                         # 8-byte Folded Spill
	std 8, 120(1)                           # 8-byte Folded Spill
	std 6, 72(1)                            # 8-byte Folded Spill
	std 4, 64(1)                            # 8-byte Folded Spill
	bgt	0, .Lp9_int8_BB10_36
# %bb.1:
	clrldi	4, 3, 32
	li 6, 1
	mr	26, 7
	mr	29, 10
	sld 7, 6, 4
	andi. 7, 7, 324
	bne	0, .Lp9_int8_BB10_4
# %bb.2:
	sld 4, 6, 4
	lis 6, 128
	li 21, 256
	ori 6, 6, 5
	rldic 6, 6, 12, 28
	and. 4, 4, 6
	beq	0, .Lp9_int8_BB10_36
# %bb.3:
	ld 4, 120(1)                            # 8-byte Folded Reload
	li 9, 1
	cmpdi	4, 0
	bc 4, 2, .Lp9_int8_BB10_5
	b .Lp9_int8_BB10_36
.Lp9_int8_BB10_4:
	li 21, 32
	ld 4, 120(1)                            # 8-byte Folded Reload
	li 9, 1
	cmpdi	4, 0
	bc 12, 2, .Lp9_int8_BB10_36
.Lp9_int8_BB10_5:
	cmpdi	28, 0
	bc 12, 2, .Lp9_int8_BB10_36
# %bb.6:
	cmpdi	29, 0
	bc 12, 2, .Lp9_int8_BB10_36
# %bb.7:
	cmpdi	5, 0
	bc 12, 2, .Lp9_int8_BB10_36
# %bb.8:
	addi 4, 21, -1
	and. 6, 4, 5
	ld 6, 72(1)                             # 8-byte Folded Reload
	cmpld 1, 6, 26
	crorc 20, 5, 2
	bc 12, 20, .Lp9_int8_BB10_36
# %bb.9:
	cnttzd 6, 21
	srd 6, 5, 6
	cmpld	26, 6
	bgt	0, .Lp9_int8_BB10_36
# %bb.10:
	li 9, 0
	bge 1, .Lp9_int8_BB10_36
# %bb.11:
	ld 6, 72(1)                             # 8-byte Folded Reload
	rldicl 4, 4, 62, 2
	sldi 10, 5, 2
	cmpwi 1, 3, 14
	li 8, 128
	addis 7, 2, .Lp9_int8_CPI10_0@toc@ha
	ld 11, 560(1)
	cmplwi	3, 14
	addi 19, 1, 124
	li 16, 0
	xxlxor 63, 63, 63
	std 10, 56(1)                           # 8-byte Folded Spill
	addi 4, 4, -7
	li 31, 0
	cmpdi 3, 11, 0
	mulld 6, 21, 6
	rlwinm 5, 3, 0, 31, 29
	cmpwi 2, 5, 12
	addis 5, 2, .Lp9_int8_CPI10_1@toc@ha
	rldicl 4, 4, 61, 3
	addi 5, 5, .Lp9_int8_CPI10_1@toc@l
	li 3, -128
	addi 14, 4, 1
	isel 8, 8, 9, 6
	ld 4, 120(1)                            # 8-byte Folded Reload
	mtvsrd 34, 8
	addis 8, 2, .Lp9_int8_CPI10_3@toc@ha
	lxv 62, 0(5)
	vspltb 28, 2, 7
	iseleq	3, 3, 9
	sldi 23, 21, 2
	rldicl 17, 21, 59, 5
	clrldi	3, 3, 32
	std 11, 48(1)                           # 8-byte Folded Spill
	std 3, 88(1)                            # 8-byte Folded Spill
	sldi 15, 6, 2
	addi 5, 15, 64
	lfs 31, .Lp9_int8_CPI10_0@toc@l(7)
	addis 7, 2, .Lp9_int8_CPI10_2@toc@ha
	addi 7, 7, .Lp9_int8_CPI10_2@toc@l
	lxv 61, 0(7)
	add 30, 4, 5
	addi 4, 8, .Lp9_int8_CPI10_3@toc@l
	std 5, 40(1)                            # 8-byte Folded Spill
	lxv 59, 0(4)
	addis 4, 2, .Lp9_int8_CPI10_4@toc@ha
	addi 4, 4, .Lp9_int8_CPI10_4@toc@l
	lxv 58, 0(4)
	addis 4, 2, .Lp9_int8_CPI10_5@toc@ha
	addi 4, 4, .Lp9_int8_CPI10_5@toc@l
	lxv 57, 0(4)
	addis 4, 2, .Lp9_int8_CPI10_6@toc@ha
	addi 4, 4, .Lp9_int8_CPI10_6@toc@l
	lxv 56, 0(4)
	addis 4, 2, .Lp9_int8_CPI10_7@toc@ha
	addi 4, 4, .Lp9_int8_CPI10_7@toc@l
	lxv 55, 0(4)
	b .Lp9_int8_BB10_13
	.p2align	4
.Lp9_int8_BB10_12:                              #   in Loop: Header=BB10_13 Depth=1
	ld 10, 56(1)                            # 8-byte Folded Reload
	ld 30, 80(1)                            # 8-byte Folded Reload
	addi 31, 31, 1
	cmpldi	31, 4
	add 30, 30, 10
	beq	0, .Lp9_int8_BB10_35
.Lp9_int8_BB10_13:                              # =>This Loop Header: Depth=1
                                        #     Child Loop BB10_17 Depth 2
                                        #       Child Loop BB10_19 Depth 3
                                        #       Child Loop BB10_30 Depth 3
	ld 3, 120(1)                            # 8-byte Folded Reload
	ld 4, 64(1)                             # 8-byte Folded Reload
	maddld 24, 10, 31, 15
	ld 20, 40(1)                            # 8-byte Folded Reload
	ld 18, 72(1)                            # 8-byte Folded Reload
	li 22, 0
	std 30, 80(1)                           # 8-byte Folded Spill
	maddld 3, 10, 31, 3
	std 3, 96(1)                            # 8-byte Folded Spill
	sldi 3, 31, 3
	add 25, 4, 3
	ld 4, 48(1)                             # 8-byte Folded Reload
	add 3, 4, 3
	std 3, 104(1)                           # 8-byte Folded Spill
	sldi 3, 31, 1
	std 3, 112(1)                           # 8-byte Folded Spill
	b .Lp9_int8_BB10_17
	.p2align	4
.Lp9_int8_BB10_14:                              #   in Loop: Header=BB10_17 Depth=2
	ld 5, 112(1)                            # 8-byte Folded Reload
	sldi 3, 18, 5
	add 3, 3, 5
	mr	5, 29
	sldi 6, 3, 1
	addi 3, 3, 25
	sthux 4, 5, 6
	lwz 4, 132(1)
	lwz 6, 136(1)
	sth 4, 2(5)
	lwz 4, 140(1)
	sth 6, 16(5)
	sth 4, 18(5)
	lwz 4, 144(1)
	sth 4, 32(5)
	lwz 4, 148(1)
	sth 4, 34(5)
	lwz 4, 152(1)
	sth 4, 48(5)
	lwz 4, 156(1)
.Lp9_int8_BB10_15:                              #   in Loop: Header=BB10_17 Depth=2
	sldi 3, 3, 1
	sthx 4, 29, 3
.Lp9_int8_BB10_16:                              #   in Loop: Header=BB10_17 Depth=2
	addi 18, 18, 1
	add 20, 20, 23
	add 30, 30, 23
	addi 22, 22, 1
	cmpld	18, 26
	beq	0, .Lp9_int8_BB10_12
.Lp9_int8_BB10_17:                              #   Parent Loop BB10_13 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB10_19 Depth 3
                                        #       Child Loop BB10_30 Depth 3
	mulld 3, 18, 21
	ld 5, 0(25)
	sldi 27, 18, 2
	cmpldi	5, 0
	sldi 4, 3, 2
	beq	0, .Lp9_int8_BB10_24
# %bb.18:                               #   in Loop: Header=BB10_17 Depth=2
	maddld 3, 23, 22, 24
	ld 6, 120(1)                            # 8-byte Folded Reload
	mtctr 14
	add 7, 5, 20
	xxlxor 0, 0, 0
	add 3, 6, 3
	maddld 6, 23, 22, 15
	.p2align	5
.Lp9_int8_BB10_19:                              #   Parent Loop BB10_13 Depth=1
                                        #     Parent Loop BB10_17 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lxv 1, -64(7)
	lxv 2, -16(7)
	xvabssp 1, 1
	xvabssp 2, 2
	xvmaxsp 0, 0, 1
	lxv 1, -48(7)
	xvabssp 1, 1
	xvmaxsp 0, 0, 1
	lxv 1, -32(7)
	xvabssp 1, 1
	xvmaxsp 0, 0, 1
	lxv 1, 0(7)
	xvmaxsp 0, 0, 2
	lxv 2, 32(7)
	xvabssp 1, 1
	xvmaxsp 0, 0, 1
	lxv 1, 16(7)
	xvabssp 2, 2
	xvabssp 1, 1
	xvmaxsp 0, 0, 1
	lxv 1, 48(7)
	addi 7, 7, 128
	xvmaxsp 0, 0, 2
	xvabssp 1, 1
	xvmaxsp 0, 0, 1
	bdnz .Lp9_int8_BB10_19
# %bb.20:                               #   in Loop: Header=BB10_17 Depth=2
	xxsldwi 1, 0, 0, 3
	xxswapd	3, 0
	xscvspdpn 2, 1
	xxlxor 1, 1, 1
	xscvspdpn 3, 3
	xsmaxcdp 2, 2, 1
	xsmaxcdp 2, 3, 2
	xxsldwi 3, 0, 0, 1
	xscvspdpn 0, 0
	xscvspdpn 3, 3
	xsmaxcdp 2, 3, 2
	xsmaxcdp 0, 0, 2
	fcmpu 0, 0, 1
	ble 0, .Lp9_int8_BB10_22
# %bb.21:                               #   in Loop: Header=BB10_17 Depth=2
	xsdivsp 1, 31, 0
.Lp9_int8_BB10_22:                              #   in Loop: Header=BB10_17 Depth=2
	bc 12, 14, .Lp9_int8_BB10_26
# %bb.23:                               #   in Loop: Header=BB10_17 Depth=2
	ld 7, 104(1)                            # 8-byte Folded Reload
	ld 7, 0(7)
	add 4, 7, 4
	cmpldi	7, 0
	iseleq	4, 0, 4
	b .Lp9_int8_BB10_27
	.p2align	4
.Lp9_int8_BB10_24:                              #   in Loop: Header=BB10_17 Depth=2
	ld 3, 96(1)                             # 8-byte Folded Reload
	mr	5, 23
	add 3, 3, 4
	ld 4, 88(1)                             # 8-byte Folded Reload
	bl memset
	nop
	or 3, 27, 31
	sldi 4, 3, 2
	stwx 16, 28, 4
	bc 4, 10, .Lp9_int8_BB10_34
# %bb.25:                               #   in Loop: Header=BB10_17 Depth=2
	ld 4, 112(1)                            # 8-byte Folded Reload
	sldi 3, 18, 5
	add 3, 3, 4
	mr	4, 29
	sldi 3, 3, 1
	stwux 16, 4, 3
	stw 16, 16(4)
	stw 16, 32(4)
	stw 16, 48(4)
	b .Lp9_int8_BB10_16
	.p2align	4
.Lp9_int8_BB10_26:                              #   in Loop: Header=BB10_17 Depth=2
	li 4, 0
.Lp9_int8_BB10_27:                              #   in Loop: Header=BB10_17 Depth=2
	xsdivsp 0, 0, 31
	stxv 63, 144(1)
	stxv 63, 128(1)
	cmpdi	4, 0
	xscvdpspn 1, 1
	add 5, 5, 6
	mr	6, 19
	xscvdpspn 2, 0
	mtctr 17
	xxspltw 1, 1, 0
	xxspltw 2, 2, 0
	b .Lp9_int8_BB10_30
	.p2align	5
.Lp9_int8_BB10_28:                              #   in Loop: Header=BB10_30 Depth=3
	lxv 3, 0(5)
	lxv 4, 16(5)
	lxv 5, 32(5)
	lxv 6, 48(5)
	xvmulsp 3, 1, 3
	xvmulsp 4, 1, 4
	xvmulsp 5, 1, 5
	xvmulsp 6, 1, 6
	xvaddsp 3, 3, 62
	xvaddsp 4, 4, 62
	xvaddsp 5, 5, 62
	xvaddsp 6, 6, 62
	xvaddsp 3, 3, 61
	xvaddsp 4, 4, 61
	xvaddsp 5, 5, 61
	xvaddsp 6, 6, 61
	xvmaxsp 3, 3, 59
	xvmaxsp 4, 4, 59
	xvminsp 34, 3, 58
	xvminsp 35, 4, 58
	xvmaxsp 3, 5, 59
	xvmaxsp 4, 6, 59
	vctsxs 4, 2, 0
	vctsxs 3, 3, 0
	xvminsp 34, 3, 58
	xvminsp 37, 4, 58
	vctsxs 2, 2, 0
	vctsxs 5, 5, 0
	vadduwm 0, 4, 3
	xxperm 36, 35, 57
	xxperm 36, 34, 56
	vadduwm 2, 0, 2
	vadduwm 2, 2, 5
	xxperm 36, 37, 55
	xxlxor 3, 36, 60
	xxspltw 4, 3, 3
	xxspltw 5, 3, 2
	xxspltw 6, 3, 1
	xxspltw 3, 3, 0
	stxv 4, 0(3)
	stxv 5, 16(3)
	stxv 6, 32(3)
	stxv 3, 48(3)
	lxv 3, 64(5)
	lxv 4, 80(5)
	lxv 5, 96(5)
	lxv 6, 112(5)
	xvmulsp 3, 1, 3
	xvmulsp 4, 1, 4
	xvmulsp 5, 1, 5
	xvmulsp 6, 1, 6
	xvaddsp 3, 3, 62
	xvaddsp 4, 4, 62
	xvaddsp 5, 5, 62
	xvaddsp 6, 6, 62
	xvaddsp 3, 3, 61
	xvaddsp 4, 4, 61
	xvaddsp 5, 5, 61
	xvaddsp 6, 6, 61
	xvmaxsp 3, 3, 59
	xvmaxsp 4, 4, 59
	xvminsp 35, 3, 58
	xvmaxsp 3, 5, 59
	xvminsp 36, 4, 58
	xvmaxsp 4, 6, 59
	vctsxs 3, 3, 0
	xvminsp 32, 3, 58
	vctsxs 4, 4, 0
	xvminsp 37, 4, 58
	vctsxs 0, 0, 0
	vctsxs 5, 5, 0
	vadduwm 2, 3, 2
	xxperm 35, 36, 57
	xxperm 35, 32, 56
	vadduwm 2, 2, 4
	vadduwm 2, 2, 0
	xxperm 35, 37, 55
	vadduwm 2, 2, 5
	xxlxor 3, 35, 60
	xxspltw 4, 3, 3
	xxspltw 5, 3, 2
	stxv 4, 64(3)
	xxspltw 4, 3, 1
	xxspltw 3, 3, 0
	stxv 5, 80(3)
	stxv 4, 96(3)
.Lp9_int8_BB10_29:                              #   in Loop: Header=BB10_30 Depth=3
	stxv 3, 112(3)
	xxswapd	35, 34
	addi 6, 6, 4
	addi 5, 5, 128
	addi 4, 4, 128
	addi 3, 3, 128
	vadduwm 2, 2, 3
	xxspltw 35, 34, 0
	vadduwm 2, 2, 3
	xxsldwi 3, 34, 34, 2
	stfiwx 3, 0, 6
	bdz .Lp9_int8_BB10_32
.Lp9_int8_BB10_30:                              #   Parent Loop BB10_13 Depth=1
                                        #     Parent Loop BB10_17 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	bc 12, 2, .Lp9_int8_BB10_28
# %bb.31:                               #   in Loop: Header=BB10_30 Depth=3
	lxv 3, 0(5)
	xvmulsp 4, 1, 3
	xvaddsp 4, 4, 62
	xvaddsp 4, 4, 61
	xvmaxsp 4, 4, 59
	xvminsp 34, 4, 58
	xvmulsp 4, 2, 34
	vctsxs 5, 2, 0
	xvsubsp 3, 3, 4
	stxv 3, 0(4)
	lxv 3, 16(5)
	xvmulsp 4, 1, 3
	xvaddsp 4, 4, 62
	xvaddsp 4, 4, 61
	xvmaxsp 4, 4, 59
	xvminsp 35, 4, 58
	xvmulsp 4, 2, 35
	vctsxs 3, 3, 0
	xvsubsp 3, 3, 4
	vadduwm 0, 5, 3
	xxperm 37, 35, 57
	stxv 3, 16(4)
	lxv 3, 32(5)
	xvmulsp 4, 1, 3
	xvaddsp 4, 4, 62
	xvaddsp 4, 4, 61
	xvmaxsp 4, 4, 59
	xvminsp 36, 4, 58
	xvmulsp 4, 2, 36
	vctsxs 2, 4, 0
	xvsubsp 3, 3, 4
	xxperm 37, 34, 56
	vadduwm 2, 0, 2
	stxv 3, 32(4)
	lxv 3, 48(5)
	xvmulsp 4, 1, 3
	xvaddsp 4, 4, 62
	xvaddsp 4, 4, 61
	xvmaxsp 4, 4, 59
	xvminsp 36, 4, 58
	vctsxs 1, 4, 0
	xvmulsp 4, 2, 36
	xxperm 37, 33, 55
	xvsubsp 3, 3, 4
	vadduwm 2, 2, 1
	xxlxor 4, 37, 60
	xxspltw 5, 4, 3
	xxspltw 6, 4, 2
	xxspltw 7, 4, 1
	xxspltw 4, 4, 0
	stxv 3, 48(4)
	stxv 5, 0(3)
	stxv 6, 16(3)
	stxv 7, 32(3)
	stxv 4, 48(3)
	lxv 3, 64(5)
	xvmulsp 4, 1, 3
	xvaddsp 4, 4, 62
	xvaddsp 4, 4, 61
	xvmaxsp 4, 4, 59
	xvminsp 35, 4, 58
	xvmulsp 4, 2, 35
	vctsxs 3, 3, 0
	xvsubsp 3, 3, 4
	vadduwm 2, 3, 2
	stxv 3, 64(4)
	lxv 3, 80(5)
	xvmulsp 4, 1, 3
	xvaddsp 4, 4, 62
	xvaddsp 4, 4, 61
	xvmaxsp 4, 4, 59
	xvminsp 36, 4, 58
	xvmulsp 4, 2, 36
	vctsxs 4, 4, 0
	xvsubsp 3, 3, 4
	xxperm 35, 36, 57
	vadduwm 2, 2, 4
	stxv 3, 80(4)
	lxv 3, 96(5)
	xvmulsp 4, 1, 3
	xvaddsp 4, 4, 62
	xvaddsp 4, 4, 61
	xvmaxsp 4, 4, 59
	xvminsp 37, 4, 58
	xvmulsp 4, 2, 37
	vctsxs 5, 5, 0
	xvsubsp 3, 3, 4
	xxperm 35, 37, 56
	vadduwm 2, 2, 5
	stxv 3, 96(4)
	lxv 3, 112(5)
	xvmulsp 4, 1, 3
	xvaddsp 4, 4, 62
	xvaddsp 4, 4, 61
	xvmaxsp 4, 4, 59
	xvminsp 32, 4, 58
	xvmulsp 4, 2, 32
	vctsxs 1, 0, 0
	xvsubsp 3, 3, 4
	xxperm 35, 33, 55
	vadduwm 2, 2, 1
	xxlxor 4, 35, 60
	xxspltw 5, 4, 3
	stxv 3, 112(4)
	xxspltw 3, 4, 2
	stxv 5, 64(3)
	xxspltw 5, 4, 1
	stxv 3, 80(3)
	xxspltw 3, 4, 0
	stxv 5, 96(3)
	b .Lp9_int8_BB10_29
	.p2align	4
.Lp9_int8_BB10_32:                              #   in Loop: Header=BB10_17 Depth=2
	or 3, 27, 31
	sldi 4, 3, 2
	stfsx 0, 28, 4
	lwz 4, 128(1)
	bc 12, 10, .Lp9_int8_BB10_14
# %bb.33:                               #   in Loop: Header=BB10_17 Depth=2
	lwz 5, 132(1)
	add 4, 5, 4
	lwz 5, 136(1)
	add 4, 5, 4
	lwz 5, 140(1)
	add 4, 5, 4
	lwz 5, 144(1)
	add 4, 5, 4
	lwz 5, 148(1)
	add 4, 5, 4
	lwz 5, 152(1)
	add 4, 5, 4
	lwz 5, 156(1)
	add 4, 5, 4
	b .Lp9_int8_BB10_15
.Lp9_int8_BB10_34:                              #   in Loop: Header=BB10_17 Depth=2
	sldi 3, 3, 1
	sthx 16, 29, 3
	b .Lp9_int8_BB10_16
.Lp9_int8_BB10_35:
	li 9, 0
.Lp9_int8_BB10_36:
	lxv 63, 288(1)                          # 16-byte Folded Reload
	lxv 62, 272(1)                          # 16-byte Folded Reload
	lxv 61, 256(1)                          # 16-byte Folded Reload
	lxv 60, 240(1)                          # 16-byte Folded Reload
	mr	3, 9
	lxv 59, 224(1)                          # 16-byte Folded Reload
	lxv 58, 208(1)                          # 16-byte Folded Reload
	lxv 57, 192(1)                          # 16-byte Folded Reload
	lxv 56, 176(1)                          # 16-byte Folded Reload
	lxv 55, 160(1)                          # 16-byte Folded Reload
	lfd 31, 456(1)                          # 8-byte Folded Reload
	ld 31, 448(1)                           # 8-byte Folded Reload
	ld 30, 440(1)                           # 8-byte Folded Reload
	ld 29, 432(1)                           # 8-byte Folded Reload
	ld 28, 424(1)                           # 8-byte Folded Reload
	ld 27, 416(1)                           # 8-byte Folded Reload
	ld 26, 408(1)                           # 8-byte Folded Reload
	ld 25, 400(1)                           # 8-byte Folded Reload
	ld 24, 392(1)                           # 8-byte Folded Reload
	ld 23, 384(1)                           # 8-byte Folded Reload
	ld 22, 376(1)                           # 8-byte Folded Reload
	ld 21, 368(1)                           # 8-byte Folded Reload
	ld 20, 360(1)                           # 8-byte Folded Reload
	ld 19, 352(1)                           # 8-byte Folded Reload
	ld 18, 344(1)                           # 8-byte Folded Reload
	ld 17, 336(1)                           # 8-byte Folded Reload
	ld 16, 328(1)                           # 8-byte Folded Reload
	ld 15, 320(1)                           # 8-byte Folded Reload
	ld 14, 312(1)                           # 8-byte Folded Reload
	addi 1, 1, 464
	ld 0, 16(1)
	lwz 12, 8(1)
	mtlr 0
	mtocrf 32, 12
	mtocrf 16, 12
	blr
	.long	0
	.quad	0
.Lp9_int8_func_end10:
	.size	pq8_quantize_group_range_power9, .Lp9_int8_func_end10-.Lp9_int8_func_begin10
	.cfi_endproc
                                        # -- End function
