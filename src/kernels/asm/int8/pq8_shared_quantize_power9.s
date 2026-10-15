# int8/pq8_shared_quantize_power9.s: kernel.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_int8.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function pq8_shared_quantize_power9
.Lp9_int8_CPI11_0:
	.long	0x7f7fffff                      # float 3.40282347E+38
	.long	0x7f7fffff                      # float 3.40282347E+38
	.long	0x7f7fffff                      # float 3.40282347E+38
	.long	0x7f7fffff                      # float 3.40282347E+38
.Lp9_int8_CPI11_4:
	.long	0x4b400000                      # float 12582912
	.long	0x4b400000                      # float 12582912
	.long	0x4b400000                      # float 12582912
	.long	0x4b400000                      # float 12582912
.Lp9_int8_CPI11_5:
	.long	0xcb400000                      # float -12582912
	.long	0xcb400000                      # float -12582912
	.long	0xcb400000                      # float -12582912
	.long	0xcb400000                      # float -12582912
.Lp9_int8_CPI11_6:
	.long	128                             # 0x80
	.long	128                             # 0x80
	.long	128                             # 0x80
	.long	128                             # 0x80
.Lp9_int8_CPI11_7:
	.byte	31                              # 0x1f
	.byte	27                              # 0x1b
	.byte	23                              # 0x17
	.byte	19                              # 0x13
	.byte	31                              # 0x1f
	.byte	27                              # 0x1b
	.byte	23                              # 0x17
	.byte	19                              # 0x13
	.byte	31                              # 0x1f
	.byte	27                              # 0x1b
	.byte	23                              # 0x17
	.byte	19                              # 0x13
	.byte	31                              # 0x1f
	.byte	27                              # 0x1b
	.byte	23                              # 0x17
	.byte	19                              # 0x13
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lp9_int8_CPI11_1:
	.long	0x42fe0000                      # float 127
.Lp9_int8_CPI11_2:
	.long	0x3b800000                      # float 0.00390625
.Lp9_int8_CPI11_3:
	.long	0x00800000                      # float 1.17549435E-38
	.section	.text.pq8_shared_quantize_power9,"ax",@progbits
	.globl	pq8_shared_quantize_power9
	.p2align 6
	.type	pq8_shared_quantize_power9,@function
pq8_shared_quantize_power9:           # @pq8_shared_quantize_power9
.Lp9_int8_func_begin11:
	.cfi_startproc
.Lp9_int8_func_gep11:
	addis 2, 12, .TOC.-.Lp9_int8_func_gep11@ha
	addi 2, 2, .TOC.-.Lp9_int8_func_gep11@l
.Lp9_int8_func_lep11:
	.localentry	pq8_shared_quantize_power9, .Lp9_int8_func_lep11-.Lp9_int8_func_gep11
# %bb.0:
	cmpdi	3, 0
	cmpdi 1, 7, 0
	ld 12, 96(1)
	std 30, -16(1)                          # 8-byte Folded Spill
	ld 30, 104(1)
	rldicl 0, 4, 59, 5
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
	cror 20, 2, 6
	cmpdi	8, 0
	std 24, -64(1)                          # 8-byte Folded Spill
	std 25, -56(1)                          # 8-byte Folded Spill
	std 26, -48(1)                          # 8-byte Folded Spill
	std 27, -40(1)                          # 8-byte Folded Spill
	std 28, -32(1)                          # 8-byte Folded Spill
	std 29, -24(1)                          # 8-byte Folded Spill
	std 31, -8(1)                           # 8-byte Folded Spill
	cror 20, 20, 2
	cmpdi	9, 0
	cror 20, 20, 2
	cmpdi	10, 0
	cror 20, 20, 2
	cmpdi	12, 0
	cror 20, 20, 2
	cmpdi	30, 0
	cror 20, 20, 2
	cmpdi	4, 0
	cror 20, 2, 20
	andi. 11, 4, 31
	li 11, 1
	crorc 20, 20, 2
	cmpld	5, 6
	cror 20, 1, 20
	bc 12, 20, .Lp9_int8_BB11_29
# %bb.1:
	cmpld 1, 6, 0
	bc 12, 5, .Lp9_int8_BB11_29
# %bb.2:
	bge 0, .Lp9_int8_BB11_28
# %bb.3:
	addis 27, 2, .Lp9_int8_CPI11_0@toc@ha
	vspltisw 2, 1
	sldi 0, 5, 7
	sub	6, 6, 5
	li 28, 0
	addi 27, 27, .Lp9_int8_CPI11_0@toc@l
	ori 29, 0, 64
	lxv 2, 0(27)
	addis 27, 2, .Lp9_int8_CPI11_1@toc@ha
	xvcvsxwdp 6, 34
	xxlxor 3, 3, 3
	xxlxor 4, 4, 4
	lfs 0, .Lp9_int8_CPI11_1@toc@l(27)
	addis 27, 2, .Lp9_int8_CPI11_2@toc@ha
	lfs 1, .Lp9_int8_CPI11_2@toc@l(27)
	addis 27, 2, .Lp9_int8_CPI11_3@toc@ha
	lfs 5, .Lp9_int8_CPI11_3@toc@l(27)
.Lp9_int8_BB11_4:                               # =>This Loop Header: Depth=1
                                        #     Child Loop BB11_7 Depth 2
	sldi 27, 28, 3
	ldx 27, 3, 27
	cmpldi	27, 0
	beq	0, .Lp9_int8_BB11_18
# %bb.5:                                #   in Loop: Header=BB11_4 Depth=1
	mtctr 6
	add 27, 27, 29
	b .Lp9_int8_BB11_7
	.p2align	5
.Lp9_int8_BB11_6:                               #   in Loop: Header=BB11_7 Depth=2
	addi 27, 27, 128
	bdz .Lp9_int8_BB11_18
.Lp9_int8_BB11_7:                               #   Parent Loop BB11_4 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lxv 7, -64(27)
	xvabssp 7, 7
	xvcmpgesp. 8, 2, 7
	bge 6, .Lp9_int8_BB11_29
# %bb.8:                                #   in Loop: Header=BB11_7 Depth=2
	lxv 8, -48(27)
	xvabssp 8, 8
	xvcmpgesp. 9, 2, 8
	bge 6, .Lp9_int8_BB11_29
# %bb.9:                                #   in Loop: Header=BB11_7 Depth=2
	lxv 9, -32(27)
	xvabssp 9, 9
	xvcmpgesp. 10, 2, 9
	bge 6, .Lp9_int8_BB11_29
# %bb.10:                               #   in Loop: Header=BB11_7 Depth=2
	lxv 10, -16(27)
	xvabssp 10, 10
	xvcmpgesp. 11, 2, 10
	bge 6, .Lp9_int8_BB11_29
# %bb.11:                               #   in Loop: Header=BB11_7 Depth=2
	lxv 11, 0(27)
	xvabssp 11, 11
	xvcmpgesp. 12, 2, 11
	bge 6, .Lp9_int8_BB11_29
# %bb.12:                               #   in Loop: Header=BB11_7 Depth=2
	lxv 12, 16(27)
	xvabssp 12, 12
	xvcmpgesp. 13, 2, 12
	bge 6, .Lp9_int8_BB11_29
# %bb.13:                               #   in Loop: Header=BB11_7 Depth=2
	lxv 13, 32(27)
	xvabssp 13, 13
	xvcmpgesp. 34, 2, 13
	bge 6, .Lp9_int8_BB11_29
# %bb.14:                               #   in Loop: Header=BB11_7 Depth=2
	lxv 34, 48(27)
	xvabssp 34, 34
	xvcmpgesp. 35, 2, 34
	bge 6, .Lp9_int8_BB11_29
# %bb.15:                               #   in Loop: Header=BB11_7 Depth=2
	xvmaxsp 7, 3, 7
	xvmaxsp 7, 7, 8
	xvmaxsp 7, 7, 9
	xvmaxsp 7, 7, 10
	xvmaxsp 7, 7, 11
	xvmaxsp 7, 7, 12
	xvmaxsp 7, 7, 13
	xvmaxsp 7, 7, 34
	xxsldwi 8, 7, 7, 3
	xxswapd	9, 7
	xscvspdpn 8, 8
	xscvspdpn 9, 9
	xsmaxcdp 8, 8, 4
	xsmaxcdp 8, 9, 8
	xxsldwi 9, 7, 7, 1
	xscvspdpn 7, 7
	xscvspdpn 9, 9
	xsmaxcdp 8, 9, 8
	xsmaxcdp 7, 7, 8
	fcmpu 0, 7, 4
	ble 0, .Lp9_int8_BB11_6
# %bb.16:                               #   in Loop: Header=BB11_7 Depth=2
	xsdivsp 7, 7, 0
	xsmulsp 7, 7, 1
	fcmpu 0, 7, 5
	xsdivsp 7, 6, 7
	bc 12, 0, .Lp9_int8_BB11_29
# %bb.17:                               #   in Loop: Header=BB11_7 Depth=2
	xststdcsp 0, 7, 96
	bc 4, 2, .Lp9_int8_BB11_6
	b .Lp9_int8_BB11_29
.Lp9_int8_BB11_18:                              #   in Loop: Header=BB11_4 Depth=1
	addi 28, 28, 1
	cmpldi	28, 4
	bne	0, .Lp9_int8_BB11_4
# %bb.19:
	sldi 28, 5, 3
	vspltisw 2, 1
	sldi 27, 5, 4
	vspltisw 3, 8
	sldi 4, 4, 2
	ori 26, 0, 112
	li 25, 0
	mr	24, 10
	mr	23, 7
	li 22, 0
	add 11, 9, 28
	add 30, 30, 28
	addi 28, 28, -8
	add 5, 8, 27
	add 29, 12, 27
	addi 27, 27, -16
	std 26, -152(1)                         # 8-byte Folded Spill
	std 28, -168(1)                         # 8-byte Folded Spill
	addis 28, 2, .Lp9_int8_CPI11_4@toc@ha
	std 27, -160(1)                         # 8-byte Folded Spill
	addi 28, 28, .Lp9_int8_CPI11_4@toc@l
	xvcvsxwdp 3, 34
	lxv 4, 0(28)
	addis 28, 2, .Lp9_int8_CPI11_5@toc@ha
	xxlxor 2, 2, 2
	addi 28, 28, .Lp9_int8_CPI11_5@toc@l
	lxv 5, 0(28)
	addis 28, 2, .Lp9_int8_CPI11_6@toc@ha
	addi 28, 28, .Lp9_int8_CPI11_6@toc@l
	lxv 34, 0(28)
	addis 28, 2, .Lp9_int8_CPI11_7@toc@ha
	addi 28, 28, .Lp9_int8_CPI11_7@toc@l
	lxv 6, 0(28)
	b .Lp9_int8_BB11_21
.Lp9_int8_BB11_20:                              #   in Loop: Header=BB11_21 Depth=1
	addi 22, 22, 1
	addi 11, 11, 2
	addi 30, 30, 2
	addi 5, 5, 4
	addi 29, 29, 4
	add 23, 23, 4
	add 24, 24, 4
	cmpldi	22, 4
	beq	0, .Lp9_int8_BB11_28
.Lp9_int8_BB11_21:                              # =>This Loop Header: Depth=1
                                        #     Child Loop BB11_24 Depth 2
	maddld 28, 4, 22, 0
	ld 27, -160(1)                          # 8-byte Folded Reload
	li 17, 0
	add 21, 10, 28
	add 20, 7, 28
	sldi 28, 22, 2
	add 28, 27, 28
	ld 27, -168(1)                          # 8-byte Folded Reload
	mtctr 6
	ld 18, -152(1)                          # 8-byte Folded Reload
	add 15, 12, 28
	add 16, 8, 28
	sldi 28, 22, 1
	add 28, 27, 28
	sldi 27, 22, 3
	add 28, 9, 28
	add 19, 3, 27
	b .Lp9_int8_BB11_24
	.p2align	5
.Lp9_int8_BB11_22:                              #   in Loop: Header=BB11_24 Depth=2
	xscvdpspn 13, 13
	xxspltw 13, 13, 0
	xvmulsp 12, 13, 12
	xvmulsp 11, 13, 11
	xvmulsp 10, 13, 10
	xvmulsp 9, 13, 9
	xvaddsp 12, 12, 4
	xvaddsp 11, 11, 4
	xvaddsp 10, 10, 4
	xvaddsp 9, 9, 4
	xvaddsp 36, 12, 5
	vctsxs 4, 4, 0
	vadduwm 5, 4, 2
	vsraw 5, 5, 3
	vslw 0, 5, 3
	vsubuwm 4, 4, 0
	xvaddsp 32, 11, 5
	vctsxs 0, 0, 0
	vadduwm 1, 0, 2
	vsraw 1, 1, 3
	vslw 6, 1, 3
	vsubuwm 0, 0, 6
	xvaddsp 38, 10, 5
	vctsxs 6, 6, 0
	vadduwm 7, 6, 2
	vsraw 7, 7, 3
	vslw 8, 7, 3
	vsubuwm 6, 6, 8
	xvaddsp 40, 9, 5
	vctsxs 8, 8, 0
	vadduwm 9, 8, 2
	vsraw 9, 9, 3
	vslw 10, 9, 3
	vsubuwm 8, 8, 10
	vadduwm 10, 1, 5
	xxperm 37, 37, 6
	xxperm 33, 33, 6
	stxv 37, 0(20)
	stxv 33, 16(20)
	vadduwm 5, 10, 7
	xxperm 39, 39, 6
	vadduwm 1, 0, 4
	xxperm 36, 36, 6
	xxperm 32, 32, 6
	stxv 39, 32(20)
	vadduwm 5, 5, 9
	xxperm 41, 41, 6
	stxv 41, 48(20)
	stxv 36, 0(21)
	vadduwm 4, 1, 6
	xxperm 38, 38, 6
	stxv 32, 16(21)
	vadduwm 4, 4, 8
	xxperm 40, 40, 6
	stxv 38, 32(21)
	stxv 40, 48(21)
	lxv 9, -48(28)
	xvmulsp 9, 13, 9
	xvaddsp 9, 9, 4
	xvaddsp 32, 9, 5
	lxv 9, -32(28)
	xvmulsp 9, 13, 9
	vctsxs 0, 0, 0
	xvaddsp 9, 9, 4
	vadduwm 1, 0, 2
	vsraw 1, 1, 3
	vslw 6, 1, 3
	vsubuwm 0, 0, 6
	xvaddsp 38, 9, 5
	lxv 9, -16(28)
	xvmulsp 9, 13, 9
	vctsxs 6, 6, 0
	xvaddsp 9, 9, 4
	vadduwm 7, 6, 2
	vsraw 7, 7, 3
	vslw 8, 7, 3
	vsubuwm 6, 6, 8
	xvaddsp 40, 9, 5
	lxvx 9, 26, 18
	xvmulsp 9, 13, 9
	vctsxs 8, 8, 0
	xvaddsp 9, 9, 4
	vadduwm 9, 8, 2
	vsraw 9, 9, 3
	vslw 10, 9, 3
	vsubuwm 8, 8, 10
	xvaddsp 42, 9, 5
	vctsxs 10, 10, 0
	vadduwm 11, 10, 2
	vsraw 11, 11, 3
	vslw 12, 11, 3
	vsubuwm 10, 10, 12
	vadduwm 12, 7, 1
	xxperm 33, 33, 6
	xxperm 39, 39, 6
	stxv 33, 64(20)
	vadduwm 1, 12, 9
	xxperm 41, 41, 6
	stxv 39, 80(20)
	stxv 41, 96(20)
	vadduwm 1, 1, 11
	xxperm 43, 43, 6
	vadduwm 5, 1, 5
	vadduwm 1, 6, 0
	xxperm 32, 32, 6
	xxperm 38, 38, 6
	stxv 43, 112(20)
	stxv 32, 64(21)
	vadduwm 0, 1, 8
	xxperm 40, 40, 6
	stxv 38, 80(21)
	stxv 40, 96(21)
	vadduwm 0, 0, 10
	xxperm 42, 42, 6
	vadduwm 4, 0, 4
	xxswapd	32, 37
	stxv 42, 112(21)
	stfs 7, 16(16)
	stfs 8, 16(15)
	vadduwm 5, 5, 0
	xxspltw 32, 37, 0
	vadduwm 5, 5, 0
	xxswapd	32, 36
	vsldoi 5, 5, 5, 8
	vadduwm 4, 4, 0
	xxspltw 32, 36, 0
	stxsihx 37, 0, 27
	vadduwm 4, 4, 0
	vsldoi 4, 4, 4, 8
	stxsihx 36, 30, 17
.Lp9_int8_BB11_23:                              #   in Loop: Header=BB11_24 Depth=2
	addi 17, 17, 8
	addi 18, 18, 128
	addi 20, 20, 128
	addi 21, 21, 128
	mr	28, 27
	mr	16, 31
	mr	15, 14
	bdz .Lp9_int8_BB11_20
.Lp9_int8_BB11_24:                              #   Parent Loop BB11_21 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	ld 26, 0(19)
	addi 14, 15, 16
	addi 31, 16, 16
	addi 27, 28, 8
	cmpldi	26, 0
	beq	0, .Lp9_int8_BB11_27
# %bb.25:                               #   in Loop: Header=BB11_24 Depth=2
	add 28, 26, 18
	xxlxor 13, 13, 13
	lxv 12, -112(28)
	lxv 11, -96(28)
	lxv 10, -80(28)
	lxv 9, -64(28)
	xvabssp 7, 12
	xvabssp 8, 11
	xvmaxsp 7, 2, 7
	xvmaxsp 7, 7, 8
	xvabssp 8, 10
	xvmaxsp 7, 7, 8
	xvabssp 8, 9
	xvmaxsp 7, 7, 8
	lxv 8, -48(28)
	xvabssp 8, 8
	xvmaxsp 7, 7, 8
	lxv 8, -32(28)
	xvabssp 8, 8
	xvmaxsp 7, 7, 8
	lxv 8, -16(28)
	xvabssp 8, 8
	xvmaxsp 7, 7, 8
	lxvx 8, 26, 18
	xvabssp 8, 8
	xvmaxsp 7, 7, 8
	xxsldwi 8, 7, 7, 3
	xxswapd	36, 7
	xscvspdpn 8, 8
	xscvspdpn 36, 36
	xsmaxcdp 8, 8, 13
	xsmaxcdp 8, 36, 8
	xxsldwi 36, 7, 7, 1
	xscvspdpn 7, 7
	xscvspdpn 36, 36
	xsmaxcdp 8, 36, 8
	xsmaxcdp 7, 7, 8
	xsdivsp 7, 7, 0
	xsmulsp 8, 7, 1
	fcmpu 0, 8, 13
	ble 0, .Lp9_int8_BB11_22
# %bb.26:                               #   in Loop: Header=BB11_24 Depth=2
	xsdivsp 13, 3, 8
	b .Lp9_int8_BB11_22
	.p2align	5
.Lp9_int8_BB11_27:                              #   in Loop: Header=BB11_24 Depth=2
	stxv 2, 112(20)
	stxv 2, 96(20)
	stxv 2, 80(20)
	stxv 2, 64(20)
	stxv 2, 48(20)
	stxv 2, 32(20)
	stxv 2, 16(20)
	stxv 2, 0(20)
	stxv 2, 112(21)
	stxv 2, 96(21)
	stxv 2, 80(21)
	stxv 2, 64(21)
	stxv 2, 48(21)
	stxv 2, 32(21)
	stxv 2, 16(21)
	stxv 2, 0(21)
	stw 25, 16(15)
	stw 25, 16(16)
	sthx 25, 30, 17
	sth 25, 8(28)
	b .Lp9_int8_BB11_23
.Lp9_int8_BB11_28:
	li 11, 0
.Lp9_int8_BB11_29:
	ld 31, -8(1)                            # 8-byte Folded Reload
	ld 30, -16(1)                           # 8-byte Folded Reload
	ld 29, -24(1)                           # 8-byte Folded Reload
	ld 28, -32(1)                           # 8-byte Folded Reload
	mr	3, 11
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
	.long	0
	.quad	0
.Lp9_int8_func_end11:
	.size	pq8_shared_quantize_power9, .Lp9_int8_func_end11-.Lp9_int8_func_begin11
	.cfi_endproc
                                        # -- End function
