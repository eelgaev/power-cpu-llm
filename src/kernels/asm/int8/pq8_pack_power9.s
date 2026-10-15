# int8/pq8_pack_power9.s: kernel.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_int8.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function pq8_pack_power9
.Lp9_int8_CPI2_0:
	.byte	23                              # 0x17
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	22                              # 0x16
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	21                              # 0x15
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	20                              # 0x14
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
.Lp9_int8_CPI2_1:
	.byte	23                              # 0x17
	.byte	23                              # 0x17
	.byte	23                              # 0x17
	.byte	23                              # 0x17
	.byte	23                              # 0x17
	.byte	23                              # 0x17
	.byte	23                              # 0x17
	.byte	23                              # 0x17
	.byte	22                              # 0x16
	.byte	22                              # 0x16
	.byte	22                              # 0x16
	.byte	22                              # 0x16
	.byte	22                              # 0x16
	.byte	22                              # 0x16
	.byte	22                              # 0x16
	.byte	22                              # 0x16
.Lp9_int8_CPI2_2:
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	4                               # 0x4
	.byte	8                               # 0x8
	.byte	16                              # 0x10
	.byte	32                              # 0x20
	.byte	64                              # 0x40
	.byte	128                             # 0x80
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	4                               # 0x4
	.byte	8                               # 0x8
	.byte	16                              # 0x10
	.byte	32                              # 0x20
	.byte	64                              # 0x40
	.byte	128                             # 0x80
.Lp9_int8_CPI2_3:
	.byte	31                              # 0x1f
	.byte	14                              # 0xe
	.byte	13                              # 0xd
	.byte	12                              # 0xc
	.byte	30                              # 0x1e
	.byte	10                              # 0xa
	.byte	9                               # 0x9
	.byte	8                               # 0x8
	.byte	29                              # 0x1d
	.byte	6                               # 0x6
	.byte	5                               # 0x5
	.byte	4                               # 0x4
	.byte	28                              # 0x1c
	.byte	2                               # 0x2
	.byte	1                               # 0x1
	.byte	0                               # 0x0
.Lp9_int8_CPI2_4:
	.byte	31                              # 0x1f
	.byte	30                              # 0x1e
	.byte	29                              # 0x1d
	.byte	28                              # 0x1c
	.byte	27                              # 0x1b
	.byte	26                              # 0x1a
	.byte	25                              # 0x19
	.byte	24                              # 0x18
	.byte	15                              # 0xf
	.byte	14                              # 0xe
	.byte	13                              # 0xd
	.byte	12                              # 0xc
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
.Lp9_int8_CPI2_5:
	.byte	31                              # 0x1f
	.byte	30                              # 0x1e
	.byte	29                              # 0x1d
	.byte	28                              # 0x1c
	.byte	27                              # 0x1b
	.byte	26                              # 0x1a
	.byte	25                              # 0x19
	.byte	24                              # 0x18
	.byte	11                              # 0xb
	.byte	10                              # 0xa
	.byte	9                               # 0x9
	.byte	8                               # 0x8
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
.Lp9_int8_CPI2_6:
	.byte	31                              # 0x1f
	.byte	30                              # 0x1e
	.byte	29                              # 0x1d
	.byte	28                              # 0x1c
	.byte	27                              # 0x1b
	.byte	26                              # 0x1a
	.byte	25                              # 0x19
	.byte	24                              # 0x18
	.byte	7                               # 0x7
	.byte	6                               # 0x6
	.byte	5                               # 0x5
	.byte	4                               # 0x4
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
.Lp9_int8_CPI2_7:
	.byte	19                              # 0x13
	.byte	18                              # 0x12
	.byte	17                              # 0x11
	.byte	16                              # 0x10
	.byte	3                               # 0x3
	.byte	2                               # 0x2
	.byte	1                               # 0x1
	.byte	0                               # 0x0
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
.Lp9_int8_CPI2_8:
	.byte	31                              # 0x1f
	.byte	30                              # 0x1e
	.byte	29                              # 0x1d
	.byte	28                              # 0x1c
	.byte	27                              # 0x1b
	.byte	26                              # 0x1a
	.byte	25                              # 0x19
	.byte	24                              # 0x18
	.byte	3                               # 0x3
	.byte	2                               # 0x2
	.byte	1                               # 0x1
	.byte	0                               # 0x0
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.section	.text.pq8_pack_power9,"ax",@progbits
	.globl	pq8_pack_power9
	.p2align 6
	.type	pq8_pack_power9,@function
pq8_pack_power9:                      # @pq8_pack_power9
.Lp9_int8_func_begin2:
	.cfi_startproc
.Lp9_int8_func_gep2:
	addis 2, 12, .TOC.-.Lp9_int8_func_gep2@ha
	addi 2, 2, .TOC.-.Lp9_int8_func_gep2@l
.Lp9_int8_func_lep2:
	.localentry	pq8_pack_power9, .Lp9_int8_func_lep2-.Lp9_int8_func_gep2
# %bb.0:
	mfcr 12
	mflr 0
	stw 12, 8(1)
	stdu 1, -1472(1)
	std 0, 1488(1)
	.cfi_def_cfa_offset 1472
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
	.cfi_offset v23, -288
	.cfi_offset v24, -272
	.cfi_offset v25, -256
	.cfi_offset v26, -240
	.cfi_offset v27, -224
	.cfi_offset v28, -208
	.cfi_offset v29, -192
	.cfi_offset v30, -176
	.cfi_offset v31, -160
	cmpdi	4, 0
	cmpdi 1, 10, 0
	std 22, 1392(1)                         # 8-byte Folded Spill
	mr	22, 10
	stxv 55, 1184(1)                        # 16-byte Folded Spill
	std 28, 1440(1)                         # 8-byte Folded Spill
	li 28, 1
	std 14, 1328(1)                         # 8-byte Folded Spill
	stxv 56, 1200(1)                        # 16-byte Folded Spill
	std 15, 1336(1)                         # 8-byte Folded Spill
	stxv 57, 1216(1)                        # 16-byte Folded Spill
	std 16, 1344(1)                         # 8-byte Folded Spill
	std 17, 1352(1)                         # 8-byte Folded Spill
	stxv 58, 1232(1)                        # 16-byte Folded Spill
	std 18, 1360(1)                         # 8-byte Folded Spill
	stxv 59, 1248(1)                        # 16-byte Folded Spill
	cror 20, 2, 6
	andi. 10, 10, 15
	std 19, 1368(1)                         # 8-byte Folded Spill
	stxv 60, 1264(1)                        # 16-byte Folded Spill
	std 20, 1376(1)                         # 8-byte Folded Spill
	stxv 61, 1280(1)                        # 16-byte Folded Spill
	std 21, 1384(1)                         # 8-byte Folded Spill
	stxv 62, 1296(1)                        # 16-byte Folded Spill
	std 23, 1400(1)                         # 8-byte Folded Spill
	std 24, 1408(1)                         # 8-byte Folded Spill
	stxv 63, 1312(1)                        # 16-byte Folded Spill
	std 25, 1416(1)                         # 8-byte Folded Spill
	std 26, 1424(1)                         # 8-byte Folded Spill
	std 27, 1432(1)                         # 8-byte Folded Spill
	std 29, 1448(1)                         # 8-byte Folded Spill
	std 30, 1456(1)                         # 8-byte Folded Spill
	crorc 20, 20, 2
	std 31, 1464(1)                         # 8-byte Folded Spill
	bc 12, 20, .Lp9_int8_BB2_24
# %bb.1:
	cmplwi 6, 3, 35
	bgt	6, .Lp9_int8_BB2_20
# %bb.2:
	clrldi	11, 3, 32
	li 12, 1
	li 10, 256
	sld 11, 12, 11
	lis 12, 128
	ori 12, 12, 5
	rldic 12, 12, 12, 28
	and. 11, 11, 12
	beq	0, .Lp9_int8_BB2_20
# %bb.3:
	cmpdi	10, 0
	bc 12, 2, .Lp9_int8_BB2_21
.Lp9_int8_BB2_4:
	cmpdi	6, 0
	bc 12, 2, .Lp9_int8_BB2_21
# %bb.5:
	cmpdi	7, 0
	bc 12, 2, .Lp9_int8_BB2_21
# %bb.6:
	addi 11, 10, -1
	li 12, -1
	rldimi 11, 12, 61, 0
	and. 11, 11, 7
	bne	0, .Lp9_int8_BB2_21
# %bb.7:
	addi 11, 6, 3
	cnttzd 10, 10
	rldicl 21, 11, 62, 2
	rlwinm 11, 3, 0, 31, 29
	srd 10, 7, 10
	cmpwi 2, 11, 12
	bgt	6, .Lp9_int8_BB2_9
# %bb.8:
	clrldi	12, 3, 32
	li 0, 1
	li 11, 256
	sld 12, 0, 12
	lis 0, 128
	ori 0, 0, 5
	rldic 0, 0, 12, 28
	and. 12, 12, 0
	bne	0, .Lp9_int8_BB2_10
.Lp9_int8_BB2_9:
	rlwinm 11, 3, 0, 30, 28
	cmpwi 1, 3, 8
	cmpwi	11, 2
	li 11, 32
	crnor 20, 6, 2
	isel 11, 0, 11, 20
.Lp9_int8_BB2_10:
	cmpwi 3, 3, 12
	cmpwi	3, 2
	li 12, 1
	li 30, 2
	li 29, 288
	crnor 20, 2, 14
	cmplwi	3, 14
	isel 27, 30, 12, 20
	li 12, 16
	mfocrf 28, 128
	iseleq	12, 29, 12
	cmplwi	3, 12
	li 29, 224
	sld 11, 11, 27
	stw 28, 548(1)
	iseleq	0, 29, 12
	li 12, 3
	mfocrf 28, 128
	add 11, 11, 0
	isel 20, 30, 12, 10
	stw 28, 628(1)
	sld 11, 11, 20
	mulld 10, 11, 10
	li 11, 7
	isel 11, 12, 11, 10
	add 11, 21, 11
	srd 11, 11, 20
	mulhdu. 12, 10, 11
	mcrf 1, 0
	mulld. 10, 10, 11
	crandc 20, 6, 2
	bc 4, 20, .Lp9_int8_BB2_21
# %bb.11:
	ld 11, 1568(1)
	li 28, 5
	cmpld	11, 10
	blt	0, .Lp9_int8_BB2_24
# %bb.12:
	bgt	6, .Lp9_int8_BB2_14
# %bb.13:
	clrldi	10, 3, 32
	li 11, 1
	li 18, 256
	sld 10, 11, 10
	lis 11, 128
	ori 11, 11, 5
	rldic 11, 11, 12, 28
	and. 10, 10, 11
	bne	0, .Lp9_int8_BB2_15
.Lp9_int8_BB2_14:
	rlwinm 10, 3, 0, 30, 28
	cmpwi 1, 3, 8
	cmpwi	10, 2
	li 10, 32
	crnor 20, 6, 2
	isel 18, 0, 10, 20
.Lp9_int8_BB2_15:
	mfocrf 10, 2
	cmpwi	3, 11
	rotlwi	10, 10, 24
	stw 10, 276(1)
	ble 0, .Lp9_int8_BB2_25
# %bb.16:
	lwz 10, 628(1)
	mtocrf 128, 10
	beq	0, .Lp9_int8_BB2_28
# %bb.17:
	lwz 10, 548(1)
	mtocrf 128, 10
	beq	0, .Lp9_int8_BB2_29
# %bb.18:
	bne	6, .Lp9_int8_BB2_31
# %bb.19:
	li 17, 66
	b .Lp9_int8_BB2_32
.Lp9_int8_BB2_20:
	rlwinm 10, 3, 0, 30, 28
	cmpwi 1, 3, 8
	cmpwi	10, 2
	li 10, 32
	crnor 20, 6, 2
	isel 10, 0, 10, 20
	cmpdi	10, 0
	bc 4, 2, .Lp9_int8_BB2_4
.Lp9_int8_BB2_21:
	li 28, 2
	bgt	6, .Lp9_int8_BB2_24
# %bb.22:
	clrldi	3, 3, 32
	li 4, 1
	sld 3, 4, 3
	rldic 4, 4, 35, 28
	ori 4, 4, 20804
	and. 3, 3, 4
	beq	0, .Lp9_int8_BB2_24
# %bb.23:
	li 28, 3
.Lp9_int8_BB2_24:
	lxv 63, 1312(1)                         # 16-byte Folded Reload
	lxv 62, 1296(1)                         # 16-byte Folded Reload
	lxv 61, 1280(1)                         # 16-byte Folded Reload
	lxv 60, 1264(1)                         # 16-byte Folded Reload
	clrldi	3, 28, 32
	lxv 59, 1248(1)                         # 16-byte Folded Reload
	lxv 58, 1232(1)                         # 16-byte Folded Reload
	lxv 57, 1216(1)                         # 16-byte Folded Reload
	lxv 56, 1200(1)                         # 16-byte Folded Reload
	lxv 55, 1184(1)                         # 16-byte Folded Reload
	ld 31, 1464(1)                          # 8-byte Folded Reload
	ld 30, 1456(1)                          # 8-byte Folded Reload
	ld 29, 1448(1)                          # 8-byte Folded Reload
	ld 28, 1440(1)                          # 8-byte Folded Reload
	ld 27, 1432(1)                          # 8-byte Folded Reload
	ld 26, 1424(1)                          # 8-byte Folded Reload
	ld 25, 1416(1)                          # 8-byte Folded Reload
	ld 24, 1408(1)                          # 8-byte Folded Reload
	ld 23, 1400(1)                          # 8-byte Folded Reload
	ld 22, 1392(1)                          # 8-byte Folded Reload
	ld 21, 1384(1)                          # 8-byte Folded Reload
	ld 20, 1376(1)                          # 8-byte Folded Reload
	ld 19, 1368(1)                          # 8-byte Folded Reload
	ld 18, 1360(1)                          # 8-byte Folded Reload
	ld 17, 1352(1)                          # 8-byte Folded Reload
	ld 16, 1344(1)                          # 8-byte Folded Reload
	ld 15, 1336(1)                          # 8-byte Folded Reload
	ld 14, 1328(1)                          # 8-byte Folded Reload
	addi 1, 1, 1472
	ld 0, 16(1)
	lwz 12, 8(1)
	mtlr 0
	mtocrf 32, 12
	mtocrf 16, 12
	mtocrf 8, 12
	blr
.Lp9_int8_BB2_25:
	cmplwi	3, 2
	beq	0, .Lp9_int8_BB2_30
# %bb.26:
	cmplwi	3, 6
	bne	0, .Lp9_int8_BB2_31
# %bb.27:
	li 17, 22
	b .Lp9_int8_BB2_32
.Lp9_int8_BB2_28:
	li 17, 144
	b .Lp9_int8_BB2_32
.Lp9_int8_BB2_29:
	li 17, 210
	b .Lp9_int8_BB2_32
.Lp9_int8_BB2_30:
	li 17, 18
	b .Lp9_int8_BB2_32
.Lp9_int8_BB2_31:
	cmplwi	3, 8
	li 10, 0
	li 11, 34
	iseleq	17, 11, 10
.Lp9_int8_BB2_32:
	cnttzd 10, 18
	li 28, 3
	srd 10, 7, 10
	mulld 11, 17, 10
	std 10, 704(1)                          # 8-byte Folded Spill
	mulhdu 10, 11, 6
	std 11, 536(1)                          # 8-byte Folded Spill
	mulld 11, 11, 6
	xor 5, 11, 5
	or. 5, 5, 10
	bne	0, .Lp9_int8_BB2_24
# %bb.33:
	cmpld 4, 8, 9
	cmpld	9, 21
	li 28, 1
	cror 20, 17, 1
	bc 12, 20, .Lp9_int8_BB2_24
# %bb.34:
	sldi 26, 7, 2
	std 4, 512(1)                           # 8-byte Folded Spill
	mr	25, 3
	li 3, 16
	mr	23, 7
	addi 4, 26, 15
	std 0, 776(1)                           # 8-byte Folded Spill
	std 8, 304(1)                           # 8-byte Folded Spill
	std 6, 696(1)                           # 8-byte Folded Spill
	std 9, 280(1)                           # 8-byte Folded Spill
	rldicr 28, 4, 0, 59
	mr	4, 28
	bl aligned_alloc
	nop
	mr	29, 3
	li 3, 16
	mr	4, 28
	bl aligned_alloc
	nop
	mr	30, 3
	ld 3, 704(1)                            # 8-byte Folded Reload
	mulli 3, 3, 368
	bl malloc
	nop
	std 3, 672(1)                           # 8-byte Folded Spill
	mr	3, 25
	mr	4, 23
	std 25, 264(1)                          # 8-byte Folded Spill
	std 23, 288(1)                          # 8-byte Folded Spill
	bl p9_int8_group_bytes
	addi 3, 3, 15
	rldicr 4, 3, 0, 59
	li 3, 16
	bl aligned_alloc
	nop
	mr	25, 3
	cmpdi	29, 0
	std 29, 728(1)                          # 8-byte Folded Spill
	bc 12, 2, .Lp9_int8_BB2_201
# %bb.35:
	cmpdi	30, 0
	bc 12, 2, .Lp9_int8_BB2_201
# %bb.36:
	ld 3, 672(1)                            # 8-byte Folded Reload
	cmpdi	3, 0
	bc 12, 2, .Lp9_int8_BB2_201
# %bb.37:
	cmpdi	25, 0
	bc 12, 2, .Lp9_int8_BB2_201
# %bb.38:
	ld 3, 264(1)                            # 8-byte Folded Reload
	std 30, 112(1)                          # 8-byte Folded Spill
	bl p9_int8_record_bytes
	li 28, 0
	bge 4, .Lp9_int8_BB2_204
# %bb.39:
	ld 23, 288(1)                           # 8-byte Folded Reload
	std 3, 472(1)                           # 8-byte Folded Spill
	std 21, 248(1)                          # 8-byte Folded Spill
	li 30, 0
	li 29, 64
	addi 10, 26, -1
	li 14, 8
	li 31, 4
	li 21, 208
	li 6, 1
	sld 19, 18, 27
	li 24, 16
	li 5, 0
	li 16, 8
	rldicl 10, 10, 60, 4
	cmpldi 1, 23, 8
	rlwinm. 3, 23, 29, 30, 31
	rldicl 7, 23, 61, 3
	sld 27, 23, 27
	addi 10, 10, 1
	xxlxor 63, 63, 63
	rotldi	11, 23, 59
	xxleqv 62, 62, 62
	mr	15, 23
	mfocrf 8, 64
	cmpdi 1, 26, 0
	addi 7, 7, -1
	rldic 11, 11, 2, 3
	std 3, 104(1)                           # 8-byte Folded Spill
	mfocrf 3, 128
	rotlwi	8, 8, 4
	stw 3, 172(1)
	lis 3, 5461
	ld 9, 264(1)                            # 8-byte Folded Reload
	ld 28, 704(1)                           # 8-byte Folded Reload
	stw 8, 212(1)
	mfocrf 8, 64
	cmpldi 1, 26, 113
	isel 26, 31, 14, 10
	ori 3, 3, 21845
	rotlwi	8, 8, 4
	mfocrf 12, 64
	cmpldi 1, 23, 16
	rldic 3, 3, 32, 3
	cmpwi 4, 9, 35
	cmpwi	9, 14
	stw 8, 124(1)
	li 8, 16
	rotlwi	12, 12, 4
	addi 4, 9, -2
	cmpdi 6, 28, 0
	oris 3, 3, 21845
	isel 30, 29, 30, 18
	iseleq	8, 8, 6
	stw 12, 92(1)
	add 29, 25, 27
	std 26, 224(1)                          # 8-byte Folded Spill
	rldicl 12, 28, 62, 2
	rotlwi	4, 4, 31
	iseleq	0, 21, 30
	cmpldi	7, 3
	std 29, 632(1)                          # 8-byte Folded Spill
	isel 29, 23, 24, 5
	rldicl 12, 12, 2, 1
	std 0, 680(1)                           # 8-byte Folded Spill
	addi 0, 26, -1
	mfocrf 7, 128
	std 0, 216(1)                           # 8-byte Folded Spill
	stw 7, 168(1)
	andi. 7, 10, 7
	ori 0, 3, 21846
	std 7, 64(1)                            # 8-byte Folded Spill
	mfocrf 7, 128
	stw 7, 88(1)
	rldicl 7, 10, 61, 3
	stw 4, 748(1)                           # 4-byte Folded Spill
	std 27, 360(1)                          # 8-byte Folded Spill
	std 20, 152(1)                          # 8-byte Folded Spill
	std 25, 528(1)                          # 8-byte Folded Spill
	std 22, 464(1)                          # 8-byte Folded Spill
	std 19, 768(1)                          # 8-byte Folded Spill
	ld 30, 776(1)                           # 8-byte Folded Reload
	ld 21, 672(1)                           # 8-byte Folded Reload
	rldicl 7, 7, 3, 3
	sldi 10, 30, 1
	add 26, 30, 27
	add 3, 26, 10
	std 26, 352(1)                          # 8-byte Folded Spill
	std 3, 344(1)                           # 8-byte Folded Spill
	add 3, 27, 10
	std 3, 336(1)                           # 8-byte Folded Spill
	addi 3, 29, -1
	rldicl 3, 3, 60, 4
	addi 3, 3, 1
	std 3, 256(1)                           # 8-byte Folded Spill
	addi 3, 11, -4
	rldicl 3, 3, 62, 2
	addi 3, 3, 1
	std 3, 80(1)                            # 8-byte Folded Spill
	addi 3, 7, -8
	addis 7, 2, .Lp9_int8_CPI2_6@toc@ha
	rldicl 3, 3, 61, 3
	addi 7, 7, .Lp9_int8_CPI2_6@toc@l
	addi 3, 3, 1
	lxv 59, 0(7)
	addis 7, 2, .Lp9_int8_CPI2_1@toc@ha
	std 3, 56(1)                            # 8-byte Folded Spill
	addi 3, 12, -4
	addi 7, 7, .Lp9_int8_CPI2_1@toc@l
	rldicl 3, 3, 62, 2
	lxv 56, 0(7)
	sld 7, 19, 20
	addi 3, 3, 1
	std 7, 144(1)                           # 8-byte Folded Spill
	std 3, 328(1)                           # 8-byte Folded Spill
	isel 3, 6, 5, 18
	addis 6, 2, .Lp9_int8_CPI2_5@toc@ha
	stw 3, 164(1)                           # 4-byte Folded Spill
	addis 3, 2, .Lp9_int8_CPI2_4@toc@ha
	addi 6, 6, .Lp9_int8_CPI2_5@toc@l
	addi 5, 3, .Lp9_int8_CPI2_4@toc@l
	lxv 60, 0(6)
	addis 6, 2, .Lp9_int8_CPI2_8@toc@ha
	isel 3, 16, 8, 14
	li 16, 24
	lxv 61, 0(5)
	addis 5, 2, .Lp9_int8_CPI2_7@toc@ha
	addi 6, 6, .Lp9_int8_CPI2_8@toc@l
	cmplwi 2, 3, 12
	crmove	10, 26
	addi 5, 5, .Lp9_int8_CPI2_7@toc@l
	lxv 57, 0(6)
	addis 6, 2, .Lp9_int8_CPI2_3@toc@ha
	lxv 58, 0(5)
	addis 5, 2, .Lp9_int8_CPI2_2@toc@ha
	addi 6, 6, .Lp9_int8_CPI2_3@toc@l
	addi 5, 5, .Lp9_int8_CPI2_2@toc@l
	std 6, 656(1)                           # 8-byte Folded Spill
	addis 6, 2, .Lp9_int8_CPI2_0@toc@ha
	lxv 55, 0(5)
	clrldi	5, 3, 32
	addi 6, 6, .Lp9_int8_CPI2_0@toc@l
	mulhdu 4, 5, 0
	std 6, 736(1)                           # 8-byte Folded Spill
	std 5, 664(1)                           # 8-byte Folded Spill
	mulli 6, 4, 12
	ld 4, 112(1)                            # 8-byte Folded Reload
	addi 8, 4, 32
	std 6, 720(1)                           # 8-byte Folded Spill
	std 8, 200(1)                           # 8-byte Folded Spill
	addi 8, 4, 64
	addi 4, 4, 16
	std 4, 96(1)                            # 8-byte Folded Spill
	addi 4, 21, 8
	std 8, 72(1)                            # 8-byte Folded Spill
	addi 8, 25, 32
	std 4, 192(1)                           # 8-byte Folded Spill
	sldi 4, 19, 1
	std 8, 176(1)                           # 8-byte Folded Spill
	std 4, 320(1)                           # 8-byte Folded Spill
	add 4, 19, 4
	std 4, 312(1)                           # 8-byte Folded Spill
	add 4, 22, 7
	li 7, 96
	std 4, 136(1)                           # 8-byte Folded Spill
	addi 4, 1, 912
	std 4, 456(1)                           # 8-byte Folded Spill
	addi 4, 1, 800
	std 4, 408(1)                           # 8-byte Folded Spill
	addi 4, 1, 916
	std 4, 448(1)                           # 8-byte Folded Spill
	addi 4, 1, 804
	std 4, 400(1)                           # 8-byte Folded Spill
	addi 4, 1, 920
	std 4, 440(1)                           # 8-byte Folded Spill
	addi 4, 1, 808
	std 4, 392(1)                           # 8-byte Folded Spill
	addi 4, 1, 924
	std 4, 432(1)                           # 8-byte Folded Spill
	addi 4, 1, 812
	std 4, 384(1)                           # 8-byte Folded Spill
	addi 4, 5, -8
	std 4, 688(1)                           # 8-byte Folded Spill
	li 4, 32
	isel 4, 7, 4, 14
	ld 7, 472(1)                            # 8-byte Folded Reload
	std 4, 496(1)                           # 8-byte Folded Spill
	rlwinm 4, 3, 4, 0, 27
	sub.	3, 5, 6
	mcrf 7, 0
	cmpdi	6, 12
	std 4, 488(1)                           # 8-byte Folded Spill
	mulli 4, 28, 92
	mfocrf 3, 128
	std 4, 520(1)                           # 8-byte Folded Spill
	sldi 4, 7, 2
	std 4, 424(1)                           # 8-byte Folded Spill
	sldi 4, 19, 2
	std 4, 760(1)                           # 8-byte Folded Spill
	sldi 4, 30, 2
	std 4, 752(1)                           # 8-byte Folded Spill
	mulld 4, 7, 28
	std 4, 128(1)                           # 8-byte Folded Spill
	stw 3, 716(1)
	andi. 3, 28, 3
	std 3, 368(1)                           # 8-byte Folded Spill
	mfocrf 3, 128
	cmpldi	28, 4
	stw 3, 420(1)
	mfocrf 3, 128
	cmplwi	9, 2
	stw 3, 416(1)
	mfocrf 3, 128
	cmplwi	9, 6
	stw 3, 300(1)
	mfocrf 3, 128
	crmove	2, 30
	stw 3, 188(1)
	mfocrf 3, 128
	std 18, 240(1)                          # 8-byte Folded Spill
	std 17, 232(1)                          # 8-byte Folded Spill
	ld 24, 304(1)                           # 8-byte Folded Reload
	stw 3, 508(1)
	b .Lp9_int8_BB2_41
.Lp9_int8_BB2_40:                               #   in Loop: Header=BB2_41 Depth=1
	ld 24, 304(1)                           # 8-byte Folded Reload
	ld 3, 280(1)                            # 8-byte Folded Reload
	ld 15, 288(1)                           # 8-byte Folded Reload
	ld 18, 240(1)                           # 8-byte Folded Reload
	li 14, 8
	li 31, 4
	crmove	26, 10
	ld 17, 232(1)                           # 8-byte Folded Reload
	ld 21, 672(1)                           # 8-byte Folded Reload
	li 16, 24
	cmpld	24, 3
	lwz 3, 508(1)
	rotlwi	3, 3, 4
	mtocrf 1, 3
	beq	0, .Lp9_int8_BB2_202
.Lp9_int8_BB2_41:                               # =>This Loop Header: Depth=1
                                        #     Child Loop BB2_44 Depth 2
                                        #       Child Loop BB2_50 Depth 3
                                        #     Child Loop BB2_117 Depth 2
                                        #     Child Loop BB2_139 Depth 2
                                        #     Child Loop BB2_142 Depth 2
                                        #     Child Loop BB2_111 Depth 2
                                        #     Child Loop BB2_127 Depth 2
                                        #     Child Loop BB2_130 Depth 2
                                        #     Child Loop BB2_148 Depth 2
                                        #       Child Loop BB2_154 Depth 3
                                        #       Child Loop BB2_157 Depth 3
                                        #       Child Loop BB2_164 Depth 3
                                        #       Child Loop BB2_167 Depth 3
                                        #       Child Loop BB2_174 Depth 3
                                        #       Child Loop BB2_177 Depth 3
                                        #       Child Loop BB2_184 Depth 3
                                        #       Child Loop BB2_187 Depth 3
                                        #     Child Loop BB2_195 Depth 2
                                        #       Child Loop BB2_197 Depth 3
                                        #       Child Loop BB2_200 Depth 3
	ld 23, 680(1)                           # 8-byte Folded Reload
	lwz 25, 748(1)                          # 4-byte Folded Reload
	std 24, 376(1)                          # 8-byte Folded Spill
	sldi 3, 24, 2
	li 28, 0
	li 7, 0
	li 22, 0
	li 24, 32
	b .Lp9_int8_BB2_44
	.p2align	4
.Lp9_int8_BB2_42:                               #   in Loop: Header=BB2_44 Depth=2
	mr	28, 6
.Lp9_int8_BB2_43:                               #   in Loop: Header=BB2_44 Depth=2
	ld 5, 728(1)                            # 8-byte Folded Reload
	sub	4, 9, 3
	addi 6, 1, 896
	maddld 4, 4, 15, 5
	sldi 5, 7, 3
	addi 7, 7, 1
	cmpldi	7, 4
	stdx 4, 6, 5
	beq	0, .Lp9_int8_BB2_107
.Lp9_int8_BB2_44:                               #   Parent Loop BB2_41 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB2_50 Depth 3
	ld 5, 696(1)                            # 8-byte Folded Reload
	or 4, 7, 3
	mr	6, 28
	cmpld	4, 5
	isellt	9, 4, 3
	cmpd	9, 4
	crorc 20, 26, 2
	bc 12, 20, .Lp9_int8_BB2_42
# %bb.45:                               #   in Loop: Header=BB2_44 Depth=2
	stw 6, 888(1)                           # 4-byte Folded Spill
	ld 6, 536(1)                            # 8-byte Folded Reload
	ld 8, 512(1)                            # 8-byte Folded Reload
	ld 5, 520(1)                            # 8-byte Folded Reload
	li 20, 0
	maddld 12, 4, 6, 8
	ld 4, 728(1)                            # 8-byte Folded Reload
	ld 6, 704(1)                            # 8-byte Folded Reload
	maddld 5, 5, 7, 21
	maddld 0, 7, 15, 4
	mulld 4, 7, 6
	mtctr 6
	addi 11, 5, 8
	addi 5, 5, -88
	mulli 4, 4, 92
	add 19, 21, 4
	b .Lp9_int8_BB2_50
.Lp9_int8_BB2_46:                               #   in Loop: Header=BB2_50 Depth=3
	lbz 30, 0(6)
	lbz 29, 1(6)
	clrlwi	30, 30, 24
	rlwinm. 26, 29, 30, 27, 31
	rlwinm 27, 29, 24, 0, 0
	mr	28, 30
	rlwimi 30, 29, 8, 22, 23
	rlwinm 25, 30, 13, 9, 18
	cmplwi 5, 30, 0
	cntlzw	30, 30
	cmplwi 1, 26, 31
	rlwimi 28, 29, 8, 16, 23
	rlwimi 25, 26, 23, 0, 8
	slwi 29, 28, 13
	add 26, 25, 27
	xori 25, 30, 31
	slwi 30, 30, 23
	or 29, 29, 27
	subfic 25, 25, 9
	addis 26, 26, 14336
	oris 29, 29, 32640
	slw 28, 28, 25
	oris 25, 27, 17152
	rlwinm 28, 28, 14, 9, 17
	sub	30, 25, 30
	lwz 25, 748(1)                          # 4-byte Folded Reload
	or 30, 30, 28
	isel 30, 27, 30, 22
	iseleq	30, 30, 26
	isel 30, 29, 30, 6
	stw 30, 0(8)
	li 30, 1
	stw 30, 96(5)
	li 5, 2
	lxvx 34, 6, 5
	xxspltib 0, 15
	xxspltib 36, 248
	xxspltib 37, 4
	xxland 35, 34, 0
	vsrb 2, 2, 5
	vaddubm 3, 3, 4
	vaddubm 2, 2, 4
	stxv 35, 0(4)
	stxv 34, 16(4)
	vsum4sbs 3, 3, 31
.Lp9_int8_BB2_47:                               #   in Loop: Header=BB2_50 Depth=3
	vsum4sbs 2, 2, 3
.Lp9_int8_BB2_48:                               #   in Loop: Header=BB2_50 Depth=3
	vextuwrx 4, 22, 2
	vextuwrx 5, 31, 2
	add 4, 4, 5
	mfvsrwz 5, 34
	add 4, 4, 5
	li 5, 12
	vextuwrx 5, 5, 2
	add 4, 4, 5
.Lp9_int8_BB2_49:                               #   in Loop: Header=BB2_50 Depth=3
	addi 20, 20, 1
	addi 11, 11, 92
	slwi 4, 4, 7
	mr	5, 10
	stw 4, 88(8)
	bdz .Lp9_int8_BB2_106
.Lp9_int8_BB2_50:                               #   Parent Loop BB2_41 Depth=1
                                        #     Parent Loop BB2_44 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	maddld 6, 20, 17, 12
	mr	4, 6
	lbzux 8, 4, 23
	lbz 29, 1(4)
	clrlwi	10, 8, 24
	mr	8, 10
	rlwinm 30, 29, 30, 27, 31
	rlwinm 4, 29, 24, 0, 0
	rlwimi 8, 29, 8, 16, 23
	cmplwi	30, 31
	beq	0, .Lp9_int8_BB2_54
# %bb.51:                               #   in Loop: Header=BB2_50 Depth=3
	rlwimi 10, 29, 8, 22, 23
	cmplwi	30, 0
	bne	0, .Lp9_int8_BB2_55
# %bb.52:                               #   in Loop: Header=BB2_50 Depth=3
	cmplwi	10, 0
	beq	0, .Lp9_int8_BB2_56
# %bb.53:                               #   in Loop: Header=BB2_50 Depth=3
	cntlzw	10, 10
	xori 30, 10, 31
	slwi 10, 10, 23
	subfic 30, 30, 9
	slw 8, 8, 30
	rlwinm 8, 8, 14, 9, 17
	sub	8, 8, 10
	add 4, 8, 4
	addis 4, 4, 17152
	b .Lp9_int8_BB2_56
	.p2align	5
.Lp9_int8_BB2_54:                               #   in Loop: Header=BB2_50 Depth=3
	slwi 8, 8, 13
	or 4, 8, 4
	oris 4, 4, 32640
	b .Lp9_int8_BB2_56
	.p2align	5
.Lp9_int8_BB2_55:                               #   in Loop: Header=BB2_50 Depth=3
	rlwinm 8, 10, 13, 9, 18
	rlwimi 8, 30, 23, 0, 8
	add 4, 8, 4
	addis 4, 4, 14336
.Lp9_int8_BB2_56:                               #   in Loop: Header=BB2_50 Depth=3
	mtfprd	0, 4
	li 28, 6
	xxsldwi 0, 0, 0, 1
	xscvspdpn 0, 0
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_int8_BB2_43
# %bb.57:                               #   in Loop: Header=BB2_50 Depth=3
	addi 10, 5, 92
	bc 4, 14, .Lp9_int8_BB2_61
# %bb.58:                               #   in Loop: Header=BB2_50 Depth=3
	lbz 4, 2(6)
	lbz 27, 3(6)
	clrlwi	30, 4, 24
	rlwinm 29, 27, 30, 27, 31
	rlwinm 4, 27, 24, 0, 0
	mr	8, 30
	cmplwi	29, 31
	rlwimi 8, 27, 8, 16, 23
	beq	0, .Lp9_int8_BB2_66
# %bb.59:                               #   in Loop: Header=BB2_50 Depth=3
	rlwimi 30, 27, 8, 22, 23
	cmplwi	29, 0
	bne	0, .Lp9_int8_BB2_67
# %bb.60:                               #   in Loop: Header=BB2_50 Depth=3
	cmplwi	30, 0
	cntlzw	30, 30
	xori 29, 30, 31
	slwi 30, 30, 23
	subfic 29, 29, 9
	slw 8, 8, 29
	rlwinm 8, 8, 14, 9, 17
	sub	8, 8, 30
	add 8, 8, 4
	addis 8, 8, 17152
	iseleq	4, 4, 8
	b .Lp9_int8_BB2_68
	.p2align	5
.Lp9_int8_BB2_61:                               #   in Loop: Header=BB2_50 Depth=3
	maddld 4, 20, 18, 0
	addi 8, 10, -4
	bc 4, 18, .Lp9_int8_BB2_70
# %bb.62:                               #   in Loop: Header=BB2_50 Depth=3
	lbz 30, 64(6)
	lbz 26, 65(6)
	stw 22, 92(5)
	clrlwi	28, 30, 24
	rlwinm 27, 26, 30, 27, 31
	rlwinm 30, 26, 24, 0, 0
	mr	29, 28
	cmplwi	27, 31
	rlwimi 29, 26, 8, 16, 23
	beq	0, .Lp9_int8_BB2_88
# %bb.63:                               #   in Loop: Header=BB2_50 Depth=3
	rlwimi 28, 26, 8, 22, 23
	cmplwi	27, 0
	bne	0, .Lp9_int8_BB2_89
# %bb.64:                               #   in Loop: Header=BB2_50 Depth=3
	cmplwi	28, 0
	beq	0, .Lp9_int8_BB2_90
# %bb.65:                               #   in Loop: Header=BB2_50 Depth=3
	cntlzw	28, 28
	oris 30, 30, 17152
	xori 27, 28, 31
	slwi 28, 28, 23
	subfic 27, 27, 9
	sub	30, 30, 28
	slw 29, 29, 27
	rlwinm 29, 29, 14, 9, 17
	or 30, 29, 30
	b .Lp9_int8_BB2_90
.Lp9_int8_BB2_66:                               #   in Loop: Header=BB2_50 Depth=3
	slwi 8, 8, 13
	or 4, 8, 4
	oris 4, 4, 32640
	b .Lp9_int8_BB2_68
.Lp9_int8_BB2_67:                               #   in Loop: Header=BB2_50 Depth=3
	rlwinm 8, 30, 13, 9, 18
	rlwimi 8, 29, 23, 0, 8
	add 4, 8, 4
	addis 4, 4, 14336
.Lp9_int8_BB2_68:                               #   in Loop: Header=BB2_50 Depth=3
	mtfprd	0, 4
	xxsldwi 0, 0, 0, 1
	xscvspdpn 0, 0
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_int8_BB2_43
# %bb.69:                               #   in Loop: Header=BB2_50 Depth=3
	mulli 8, 20, 92
	maddld 4, 20, 18, 0
	add 8, 19, 8
.Lp9_int8_BB2_70:                               #   in Loop: Header=BB2_50 Depth=3
	cmpwi	25, 4
	bgt	0, .Lp9_int8_BB2_74
# %bb.71:                               #   in Loop: Header=BB2_50 Depth=3
	cmplwi	25, 0
	beq	0, .Lp9_int8_BB2_46
# %bb.72:                               #   in Loop: Header=BB2_50 Depth=3
	cmplwi	25, 2
	bne	0, .Lp9_int8_BB2_80
# %bb.73:                               #   in Loop: Header=BB2_50 Depth=3
	lbz 30, 0(6)
	lbz 29, 1(6)
	clrlwi	30, 30, 24
	rlwinm. 26, 29, 30, 27, 31
	rlwinm 27, 29, 24, 0, 0
	mr	28, 30
	rlwimi 30, 29, 8, 22, 23
	rlwinm 25, 30, 13, 9, 18
	cmplwi 5, 30, 0
	cntlzw	30, 30
	cmplwi 1, 26, 31
	rlwimi 28, 29, 8, 16, 23
	rlwimi 25, 26, 23, 0, 8
	slwi 29, 28, 13
	add 26, 25, 27
	xori 25, 30, 31
	slwi 30, 30, 23
	or 29, 29, 27
	subfic 25, 25, 9
	addis 26, 26, 14336
	oris 29, 29, 32640
	slw 28, 28, 25
	oris 25, 27, 17152
	rlwinm 28, 28, 14, 9, 17
	sub	30, 25, 30
	lwz 25, 748(1)                          # 4-byte Folded Reload
	or 30, 30, 28
	isel 30, 27, 30, 22
	iseleq	30, 30, 26
	isel 30, 29, 30, 6
	stw 30, 0(8)
	li 30, 1
	stw 30, 96(5)
	li 5, 6
	lxvx 34, 6, 5
	li 5, 2
	xxspltib 0, 15
	xxspltib 35, 4
	lxsihzx 1, 6, 5
	xxland 0, 34, 0
	vsrb 2, 2, 3
	xxperm 1, 1, 56
	xxland 35, 1, 55
	xxspltib 1, 16
	xxspltib 36, 240
	vcmpequb 3, 3, 23
	xxland 2, 35, 1
	xxlor 35, 2, 0
	vaddubm 3, 3, 4
	stxv 35, 0(4)
	vsum4sbs 3, 3, 31
	lxsihzx 0, 6, 31
	xxperm 0, 0, 56
	xxland 37, 0, 55
	vcmpequb 5, 5, 23
	xxland 0, 37, 1
	xxlor 34, 0, 34
	vaddubm 2, 2, 4
	stxv 34, 16(4)
	b .Lp9_int8_BB2_47
	.p2align	5
.Lp9_int8_BB2_74:                               #   in Loop: Header=BB2_50 Depth=3
	cmplwi	25, 6
	beq	0, .Lp9_int8_BB2_84
# %bb.75:                               #   in Loop: Header=BB2_50 Depth=3
	cmplwi	25, 5
	bne	0, .Lp9_int8_BB2_80
# %bb.76:                               #   in Loop: Header=BB2_50 Depth=3
	lbz 30, 0(6)
	lbz 26, 1(6)
	clrlwi	28, 30, 24
	rlwinm 27, 26, 30, 27, 31
	rlwinm 30, 26, 24, 0, 0
	mr	29, 28
	cmplwi	27, 31
	rlwimi 29, 26, 8, 16, 23
	beq	0, .Lp9_int8_BB2_92
# %bb.77:                               #   in Loop: Header=BB2_50 Depth=3
	rlwimi 28, 26, 8, 22, 23
	cmplwi	27, 0
	bne	0, .Lp9_int8_BB2_96
# %bb.78:                               #   in Loop: Header=BB2_50 Depth=3
	cmplwi	28, 0
	beq	0, .Lp9_int8_BB2_97
# %bb.79:                               #   in Loop: Header=BB2_50 Depth=3
	cntlzw	28, 28
	oris 30, 30, 17152
	xori 27, 28, 31
	slwi 28, 28, 23
	subfic 27, 27, 9
	sub	30, 30, 28
	slw 29, 29, 27
	rlwinm 29, 29, 14, 9, 17
	or 30, 30, 29
	b .Lp9_int8_BB2_97
.Lp9_int8_BB2_80:                               #   in Loop: Header=BB2_50 Depth=3
	lbz 30, 0(6)
	lbz 26, 1(6)
	clrlwi	28, 30, 24
	rlwinm 27, 26, 30, 27, 31
	rlwinm 30, 26, 24, 0, 0
	mr	29, 28
	cmplwi	27, 31
	rlwimi 29, 26, 8, 16, 23
	beq	0, .Lp9_int8_BB2_91
# %bb.81:                               #   in Loop: Header=BB2_50 Depth=3
	rlwimi 28, 26, 8, 22, 23
	cmplwi	27, 0
	bne	0, .Lp9_int8_BB2_94
# %bb.82:                               #   in Loop: Header=BB2_50 Depth=3
	cmplwi	28, 0
	beq	0, .Lp9_int8_BB2_95
# %bb.83:                               #   in Loop: Header=BB2_50 Depth=3
	cntlzw	28, 28
	oris 30, 30, 17152
	xori 27, 28, 31
	slwi 28, 28, 23
	subfic 27, 27, 9
	sub	30, 30, 28
	slw 29, 29, 27
	rlwinm 29, 29, 14, 9, 17
	or 30, 30, 29
	b .Lp9_int8_BB2_95
.Lp9_int8_BB2_84:                               #   in Loop: Header=BB2_50 Depth=3
	lbz 30, 208(6)
	lbz 26, 209(6)
	clrlwi	28, 30, 24
	rlwinm 27, 26, 30, 27, 31
	rlwinm 30, 26, 24, 0, 0
	mr	29, 28
	cmplwi	27, 31
	rlwimi 29, 26, 8, 16, 23
	beq	0, .Lp9_int8_BB2_93
# %bb.85:                               #   in Loop: Header=BB2_50 Depth=3
	rlwimi 28, 26, 8, 22, 23
	cmplwi	27, 0
	bne	0, .Lp9_int8_BB2_104
# %bb.86:                               #   in Loop: Header=BB2_50 Depth=3
	cmplwi	28, 0
	beq	0, .Lp9_int8_BB2_105
# %bb.87:                               #   in Loop: Header=BB2_50 Depth=3
	cntlzw	28, 28
	oris 30, 30, 17152
	xori 27, 28, 31
	slwi 28, 28, 23
	subfic 27, 27, 9
	sub	30, 30, 28
	slw 29, 29, 27
	rlwinm 29, 29, 14, 9, 17
	or 30, 30, 29
	b .Lp9_int8_BB2_105
.Lp9_int8_BB2_88:                               #   in Loop: Header=BB2_50 Depth=3
	slwi 29, 29, 13
	or 30, 29, 30
	oris 30, 30, 32640
	b .Lp9_int8_BB2_90
.Lp9_int8_BB2_89:                               #   in Loop: Header=BB2_50 Depth=3
	rlwinm 29, 28, 13, 9, 18
	rlwimi 29, 27, 23, 0, 8
	add 30, 29, 30
	addis 30, 30, 14336
.Lp9_int8_BB2_90:                               #   in Loop: Header=BB2_50 Depth=3
	stw 30, 88(5)
	li 30, 1
	lxv 35, 16(6)
	lxv 36, 0(6)
	stw 30, 96(5)
	xxspltib 1, 3
	xxland 34, 36, 1
	xxland 37, 35, 1
	vaddubm 2, 2, 30
	vaddubm 5, 5, 30
	stxv 34, 0(4)
	stxv 37, 16(4)
	xxspltib 37, 2
	vsrb 2, 4, 5
	vsrb 0, 3, 5
	xxland 34, 34, 1
	xxland 32, 32, 1
	vaddubm 2, 2, 30
	vaddubm 0, 0, 30
	stxv 34, 32(4)
	stxv 32, 48(4)
	xxspltib 34, 4
	vsrb 0, 4, 2
	vsrb 1, 3, 2
	xxland 32, 32, 1
	xxland 33, 33, 1
	vaddubm 0, 0, 30
	vaddubm 1, 1, 30
	stxv 32, 64(4)
	stxv 33, 80(4)
	xxspltib 32, 6
	lxv 39, 48(6)
	lxv 33, 32(6)
	ld 5, 736(1)                            # 8-byte Folded Reload
	vsrb 3, 3, 0
	vsrb 4, 4, 0
	lxv 0, 0(5)
	li 5, 12
	vaddubm 3, 3, 30
	xxland 38, 33, 1
	vsrb 8, 1, 5
	vsrb 9, 7, 5
	vsrb 10, 1, 2
	vsrb 11, 7, 2
	vsrb 5, 1, 0
	vsrb 0, 7, 0
	lxsiwzx 34, 0, 4
	vaddubm 4, 4, 30
	stxv 35, 112(4)
	xxland 35, 39, 1
	vaddubm 5, 5, 30
	vaddubm 0, 0, 30
	xxland 39, 42, 1
	stxv 36, 96(4)
	stxv 37, 224(4)
	vaddubm 1, 3, 30
	lxsiwzx 37, 4, 5
	li 5, 16
	stxv 32, 240(4)
	vaddubm 7, 7, 30
	lxsiwzx 35, 4, 31
	vaddubm 4, 6, 30
	stxv 36, 128(4)
	lxsiwzx 36, 4, 14
	xxland 38, 41, 1
	vaddubm 6, 6, 30
	xxperm 34, 34, 0
	stxv 33, 144(4)
	xxland 33, 40, 1
	xxland 40, 43, 1
	lxsiwzx 32, 4, 5
	li 5, 20
	stxv 39, 192(4)
	stxv 38, 176(4)
	lxsiwzx 38, 4, 16
	vaddubm 8, 8, 30
	vaddubm 1, 1, 30
	xxperm 35, 35, 0
	vextsb2w 2, 2
	xxperm 36, 36, 0
	xxperm 37, 37, 0
	stxv 40, 208(4)
	lxsiwzx 40, 4, 24
	stxv 33, 160(4)
	lxsiwzx 33, 4, 5
	li 5, 28
	xxperm 32, 32, 0
	xxperm 38, 38, 0
	lxsiwzx 39, 4, 5
	li 5, 36
	vextsb2w 3, 3
	xxperm 40, 40, 0
	vextsb2w 4, 4
	vextsb2w 5, 5
	vextsb2w 0, 0
	xxperm 33, 33, 0
	xxperm 39, 39, 0
	vextsb2w 6, 6
	vextsb2w 8, 8
	vextsb2w 1, 1
	vadduwm 2, 2, 8
	lxsiwzx 40, 4, 5
	li 5, 40
	vextsb2w 7, 7
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 3, 3, 8
	lxsiwzx 40, 4, 5
	li 5, 44
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 4, 4, 8
	lxsiwzx 40, 4, 5
	li 5, 48
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 5, 5, 8
	lxsiwzx 40, 4, 5
	li 5, 52
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 0, 0, 8
	lxsiwzx 40, 4, 5
	li 5, 56
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 1, 1, 8
	lxsiwzx 40, 4, 5
	li 5, 60
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 6, 6, 8
	lxsiwzx 40, 4, 5
	li 5, 64
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 7, 7, 8
	lxsiwzx 40, 4, 5
	li 5, 68
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 2, 2, 8
	lxsiwzx 40, 4, 5
	li 5, 72
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 3, 3, 8
	lxsiwzx 40, 4, 5
	li 5, 76
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 4, 4, 8
	lxsiwzx 40, 4, 5
	li 5, 80
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 5, 5, 8
	lxsiwzx 40, 4, 5
	li 5, 84
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 0, 0, 8
	lxsiwzx 40, 4, 5
	li 5, 88
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 1, 1, 8
	lxsiwzx 40, 4, 5
	li 5, 92
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 6, 6, 8
	lxsiwzx 40, 4, 5
	li 5, 96
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 7, 7, 8
	lxsiwzx 40, 4, 5
	li 5, 100
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 2, 2, 8
	lxsiwzx 40, 4, 5
	li 5, 104
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 3, 3, 8
	lxsiwzx 40, 4, 5
	li 5, 108
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 4, 4, 8
	lxsiwzx 40, 4, 5
	li 5, 112
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 5, 5, 8
	lxsiwzx 40, 4, 5
	li 5, 116
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 0, 0, 8
	lxsiwzx 40, 4, 5
	li 5, 120
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 1, 1, 8
	lxsiwzx 40, 4, 5
	li 5, 124
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 6, 6, 8
	lxsiwzx 40, 4, 5
	li 5, 128
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 7, 7, 8
	lxsiwzx 40, 4, 5
	li 5, 132
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 2, 2, 8
	lxsiwzx 40, 4, 5
	li 5, 136
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 3, 3, 8
	lxsiwzx 40, 4, 5
	li 5, 140
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 4, 4, 8
	lxsiwzx 40, 4, 5
	li 5, 144
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 5, 5, 8
	lxsiwzx 40, 4, 5
	li 5, 148
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 0, 0, 8
	lxsiwzx 40, 4, 5
	li 5, 152
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 1, 1, 8
	lxsiwzx 40, 4, 5
	li 5, 156
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 6, 6, 8
	lxsiwzx 40, 4, 5
	li 5, 160
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 7, 7, 8
	lxsiwzx 40, 4, 5
	li 5, 164
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 2, 2, 8
	lxsiwzx 40, 4, 5
	li 5, 168
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 3, 3, 8
	lxsiwzx 40, 4, 5
	li 5, 172
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 4, 4, 8
	lxsiwzx 40, 4, 5
	li 5, 176
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 5, 5, 8
	lxsiwzx 40, 4, 5
	li 5, 180
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 0, 0, 8
	lxsiwzx 40, 4, 5
	li 5, 184
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 1, 1, 8
	lxsiwzx 40, 4, 5
	li 5, 188
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 6, 6, 8
	lxsiwzx 40, 4, 5
	li 5, 192
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 7, 7, 8
	lxsiwzx 40, 4, 5
	li 5, 196
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 2, 2, 8
	lxsiwzx 40, 4, 5
	li 5, 200
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 3, 3, 8
	lxsiwzx 40, 4, 5
	li 5, 204
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 4, 4, 8
	lxsiwzx 40, 4, 5
	li 5, 208
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 5, 5, 8
	lxsiwzx 40, 4, 5
	li 5, 212
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 0, 0, 8
	lxsiwzx 40, 4, 5
	li 5, 216
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 1, 1, 8
	lxsiwzx 40, 4, 5
	li 5, 220
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 6, 6, 8
	lxsiwzx 40, 4, 5
	li 5, 224
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 7, 7, 8
	lxsiwzx 40, 4, 5
	li 5, 228
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 2, 2, 8
	lxsiwzx 40, 4, 5
	li 5, 232
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 3, 3, 8
	lxsiwzx 40, 4, 5
	li 5, 236
	vadduwm 2, 3, 2
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 4, 4, 8
	lxsiwzx 40, 4, 5
	li 5, 240
	vadduwm 2, 4, 2
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 5, 5, 8
	lxsiwzx 40, 4, 5
	li 5, 244
	xxperm 40, 40, 0
	vadduwm 2, 5, 2
	vextsb2w 8, 8
	vadduwm 0, 0, 8
	lxsiwzx 40, 4, 5
	li 5, 248
	vadduwm 2, 0, 2
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 1, 1, 8
	lxsiwzx 40, 4, 5
	li 5, 252
	vadduwm 2, 1, 2
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 6, 6, 8
	lxsiwzx 40, 4, 5
	vadduwm 2, 6, 2
	xxperm 40, 40, 0
	vextsb2w 8, 8
	vadduwm 7, 7, 8
	vadduwm 2, 7, 2
	xxswapd	35, 34
	vadduwm 2, 2, 3
	xxspltw 35, 34, 2
	vadduwm 2, 2, 3
	vextuwrx 4, 22, 2
	b .Lp9_int8_BB2_49
.Lp9_int8_BB2_91:                               #   in Loop: Header=BB2_50 Depth=3
	slwi 29, 29, 13
	or 30, 29, 30
	oris 30, 30, 32640
	b .Lp9_int8_BB2_95
.Lp9_int8_BB2_92:                               #   in Loop: Header=BB2_50 Depth=3
	slwi 29, 29, 13
	or 30, 29, 30
	oris 30, 30, 32640
	b .Lp9_int8_BB2_97
.Lp9_int8_BB2_93:                               #   in Loop: Header=BB2_50 Depth=3
	slwi 29, 29, 13
	or 30, 29, 30
	oris 30, 30, 32640
	b .Lp9_int8_BB2_105
.Lp9_int8_BB2_94:                               #   in Loop: Header=BB2_50 Depth=3
	rlwinm 29, 28, 13, 9, 18
	rlwimi 29, 27, 23, 0, 8
	add 30, 29, 30
	addis 30, 30, 14336
.Lp9_int8_BB2_95:                               #   in Loop: Header=BB2_50 Depth=3
	stw 30, 0(8)
	li 30, 1
	stw 30, 96(5)
	li 5, 2
	lxvx 34, 6, 5
	li 5, 18
	stxv 34, 0(4)
	vsum4sbs 2, 2, 31
	lxvx 35, 6, 5
	stxv 35, 16(4)
	vsum4sbs 2, 3, 2
	b .Lp9_int8_BB2_48
.Lp9_int8_BB2_96:                               #   in Loop: Header=BB2_50 Depth=3
	rlwinm 29, 28, 13, 9, 18
	rlwimi 29, 27, 23, 0, 8
	add 30, 29, 30
	addis 30, 30, 14336
.Lp9_int8_BB2_97:                               #   in Loop: Header=BB2_50 Depth=3
	stw 30, 0(8)
	lbz 30, 2(6)
	lbz 26, 3(6)
	clrlwi	28, 30, 24
	rlwinm 27, 26, 30, 27, 31
	rlwinm 30, 26, 24, 0, 0
	mr	29, 28
	cmplwi	27, 31
	rlwimi 29, 26, 8, 16, 23
	beq	0, .Lp9_int8_BB2_101
# %bb.98:                               #   in Loop: Header=BB2_50 Depth=3
	rlwimi 28, 26, 8, 22, 23
	cmplwi	27, 0
	bne	0, .Lp9_int8_BB2_102
# %bb.99:                               #   in Loop: Header=BB2_50 Depth=3
	cmplwi	28, 0
	beq	0, .Lp9_int8_BB2_103
# %bb.100:                              #   in Loop: Header=BB2_50 Depth=3
	cntlzw	28, 28
	oris 30, 30, 17152
	xori 27, 28, 31
	slwi 28, 28, 23
	subfic 27, 27, 9
	sub	30, 30, 28
	slw 29, 29, 27
	rlwinm 29, 29, 14, 9, 17
	or 30, 30, 29
	b .Lp9_int8_BB2_103
.Lp9_int8_BB2_101:                              #   in Loop: Header=BB2_50 Depth=3
	slwi 29, 29, 13
	or 30, 29, 30
	oris 30, 30, 32640
	b .Lp9_int8_BB2_103
.Lp9_int8_BB2_102:                              #   in Loop: Header=BB2_50 Depth=3
	rlwinm 29, 28, 13, 9, 18
	rlwimi 29, 27, 23, 0, 8
	add 30, 29, 30
	addis 30, 30, 14336
.Lp9_int8_BB2_103:                              #   in Loop: Header=BB2_50 Depth=3
	stw 30, 92(5)
	lbz 30, 8(6)
	lfiwzx 0, 6, 31
	li 16, 12
	xxswapd	34, 0
	clrlwi	29, 30, 26
	sth 29, 160(5)
	lbz 29, 9(6)
	clrlwi	28, 29, 26
	sth 28, 162(5)
	lbz 28, 10(6)
	clrlwi	27, 28, 26
	sth 27, 164(5)
	lbz 27, 11(6)
	clrlwi	26, 27, 26
	sth 26, 166(5)
	xxspltib 0, 63
	ld 26, 656(1)                           # 8-byte Folded Reload
	lbz 23, 12(6)
	lbz 24, 13(6)
	lbz 25, 14(6)
	xxland 0, 34, 0
	lxv 1, 0(26)
	rlwinm 23, 23, 28, 28, 31
	lbz 26, 15(6)
	rlwimi 23, 30, 30, 2, 27
	rlwinm 30, 24, 28, 28, 31
	li 24, 32
	rlwimi 30, 29, 30, 2, 27
	sth 23, 168(5)
	xxperm 0, 63, 1
	sth 30, 170(5)
	rlwinm 30, 25, 28, 28, 31
	stxv 0, 0(11)
	lfiwzx 0, 6, 16
	rlwimi 30, 28, 30, 2, 27
	sth 30, 172(5)
	xxspltib 2, 15
	xxspltib 35, 2
	xxspltib 3, 48
	rlwinm 30, 26, 28, 28, 31
	xxswapd	0, 0
	rlwimi 30, 27, 30, 2, 27
	vsrb 2, 2, 3
	lxv 35, 16(6)
	sth 30, 174(5)
	li 5, 16
	xxland 0, 0, 2
	xxland 3, 34, 3
	xxlor 0, 0, 3
	xxperm 0, 63, 1
	xxland 36, 35, 2
	stxv 0, 16(11)
	xxspltib 34, 4
	stxv 36, 0(4)
	vsrb 3, 3, 2
	stxv 35, 32(4)
	vsum4sbs 4, 4, 31
	vsum4sbs 3, 3, 31
	lxv 37, 32(6)
	xxland 32, 37, 2
	stxv 32, 16(4)
	vsrb 5, 5, 2
	stxv 37, 48(4)
	vsum4sbs 4, 0, 4
	vsum4sbs 3, 5, 3
	lxvwsx 37, 0, 11
	vmuluwm 4, 5, 4
	lxvwsx 37, 10, 14
	vmuluwm 3, 5, 3
	vadduwm 3, 3, 4
	lxv 36, 48(6)
	xxland 37, 36, 2
	vsrb 4, 4, 2
	stxv 37, 64(4)
	stxv 36, 96(4)
	vsum4sbs 5, 5, 31
	vsum4sbs 4, 4, 31
	lxv 32, 64(6)
	xxland 33, 32, 2
	vsrb 0, 0, 2
	stxv 33, 80(4)
	stxv 32, 112(4)
	vsum4sbs 5, 1, 5
	vsum4sbs 4, 0, 4
	lxvwsx 32, 10, 16
	li 16, 24
	vmuluwm 5, 0, 5
	vadduwm 3, 3, 5
	lxvwsx 37, 10, 5
	vmuluwm 4, 5, 4
	vadduwm 3, 3, 4
	lxv 36, 80(6)
	xxland 37, 36, 2
	vsrb 4, 4, 2
	stxv 37, 128(4)
	stxv 36, 160(4)
	vsum4sbs 5, 5, 31
	vsum4sbs 4, 4, 31
	lxv 32, 96(6)
	xxland 33, 32, 2
	vsrb 0, 0, 2
	stxv 33, 144(4)
	stxv 32, 176(4)
	vsum4sbs 5, 1, 5
	vsum4sbs 4, 0, 4
	lxvwsx 32, 11, 5
	vmuluwm 5, 0, 5
	vadduwm 3, 3, 5
	lxvwsx 37, 10, 16
	vmuluwm 4, 5, 4
	vadduwm 3, 3, 4
	lxv 36, 112(6)
	xxland 37, 36, 2
	vsrb 4, 4, 2
	stxv 37, 192(4)
	stxv 36, 224(4)
	vsum4sbs 5, 5, 31
	vsum4sbs 4, 4, 31
	lxv 32, 128(6)
	xxland 33, 32, 2
	vsrb 2, 0, 2
	stxv 33, 208(4)
	stxv 34, 240(4)
	vsum4sbs 5, 1, 5
	li 4, 28
	vsum4sbs 2, 2, 4
	ld 23, 680(1)                           # 8-byte Folded Reload
	lwz 25, 748(1)                          # 4-byte Folded Reload
	lxvwsx 36, 10, 4
	vmuluwm 4, 4, 5
	vadduwm 3, 3, 4
	lxvwsx 36, 10, 24
	vmuluwm 2, 4, 2
	vadduwm 2, 3, 2
	b .Lp9_int8_BB2_48
.Lp9_int8_BB2_104:                              #   in Loop: Header=BB2_50 Depth=3
	rlwinm 29, 28, 13, 9, 18
	rlwimi 29, 27, 23, 0, 8
	add 30, 29, 30
	addis 30, 30, 14336
.Lp9_int8_BB2_105:                              #   in Loop: Header=BB2_50 Depth=3
	stw 30, 0(8)
	lbz 30, 192(6)
	extsb 30, 30
	stw 30, 96(5)
	xxspltib 1, 15
	lxv 33, 0(6)
	xxspltib 34, 4
	lxv 37, 128(6)
	xxspltib 0, 48
	vslb 3, 5, 2
	xxland 2, 33, 1
	vsrb 1, 1, 2
	xxland 3, 35, 0
	xxspltib 35, 224
	xxlor 36, 3, 2
	mtfprwz	2, 30
	vaddubm 4, 4, 3
	xxspltw 32, 2, 1
	stxv 36, 0(4)
	vsum4sbs 4, 4, 31
	lxv 38, 16(6)
	lbz 30, 193(6)
	extsb 30, 30
	stw 30, 100(5)
	xxland 2, 38, 1
	vsrb 6, 6, 2
	vmuluwm 4, 0, 4
	lxv 32, 144(6)
	vslb 7, 0, 2
	xxland 3, 39, 0
	xxlor 39, 3, 2
	mtfprwz	2, 30
	xxspltw 40, 2, 1
	vaddubm 7, 7, 3
	stxv 39, 16(4)
	vsum4sbs 7, 7, 31
	lbz 30, 194(6)
	extsb 30, 30
	stw 30, 104(5)
	vmuluwm 7, 8, 7
	lxv 40, 32(6)
	xxland 2, 40, 1
	vadduwm 7, 7, 4
	xxspltib 36, 2
	vslb 9, 5, 4
	xxland 3, 41, 0
	xxlor 41, 3, 2
	mtfprwz	2, 30
	xxspltw 42, 2, 1
	vaddubm 9, 9, 3
	stxv 41, 32(4)
	vsum4sbs 9, 9, 31
	lbz 30, 195(6)
	extsb 30, 30
	stw 30, 108(5)
	vmuluwm 9, 10, 9
	vslb 10, 0, 4
	xxland 3, 42, 0
	vadduwm 7, 7, 9
	lxv 41, 48(6)
	xxland 2, 41, 1
	xxlor 42, 3, 2
	mtfprwz	2, 30
	vaddubm 10, 10, 3
	xxspltw 43, 2, 1
	xxland 2, 37, 0
	vsrb 5, 5, 4
	stxv 42, 48(4)
	vsum4sbs 10, 10, 31
	lbz 30, 196(6)
	xxlor 33, 2, 33
	vaddubm 1, 1, 3
	extsb 30, 30
	stxv 33, 64(4)
	stw 30, 112(5)
	vsum4sbs 1, 1, 31
	mtfprwz	2, 30
	lbz 30, 197(6)
	vmuluwm 10, 11, 10
	extsb 30, 30
	stw 30, 116(5)
	vadduwm 7, 7, 10
	xxspltw 42, 2, 1
	xxland 2, 32, 0
	vsrb 0, 0, 4
	xxlor 38, 2, 38
	mtfprwz	2, 30
	vmuluwm 1, 10, 1
	vaddubm 6, 6, 3
	stxv 38, 80(4)
	vsum4sbs 6, 6, 31
	lbz 30, 198(6)
	vadduwm 1, 7, 1
	xxspltw 39, 2, 1
	xxland 2, 37, 0
	extsb 30, 30
	stw 30, 120(5)
	vmuluwm 6, 7, 6
	vadduwm 1, 1, 6
	vsrb 6, 8, 2
	xxlor 37, 2, 38
	mtfprwz	2, 30
	vaddubm 5, 5, 3
	xxspltw 38, 2, 1
	xxland 2, 32, 0
	stxv 37, 96(4)
	vsum4sbs 5, 5, 31
	lbz 30, 199(6)
	extsb 30, 30
	stw 30, 124(5)
	vmuluwm 5, 6, 5
	vadduwm 5, 1, 5
	vsrb 1, 9, 2
	xxlor 32, 2, 33
	mtfprwz	2, 30
	xxspltw 33, 2, 1
	vaddubm 0, 0, 3
	stxv 32, 112(4)
	vsum4sbs 0, 0, 31
	lbz 30, 200(6)
	extsb 30, 30
	stw 30, 128(5)
	vmuluwm 0, 1, 0
	lxv 33, 64(6)
	xxland 2, 33, 1
	vsrb 1, 1, 2
	vadduwm 0, 5, 0
	lxv 37, 160(6)
	vslb 6, 5, 2
	xxland 3, 38, 0
	xxlor 38, 3, 2
	mtfprwz	2, 30
	vaddubm 6, 6, 3
	xxspltw 39, 2, 1
	stxv 38, 128(4)
	vsum4sbs 6, 6, 31
	lbz 30, 201(6)
	extsb 30, 30
	stw 30, 132(5)
	vmuluwm 6, 7, 6
	vadduwm 7, 0, 6
	lxv 32, 176(6)
	lxv 38, 80(6)
	vslb 8, 0, 2
	xxland 2, 38, 1
	vsrb 6, 6, 2
	xxland 3, 40, 0
	xxlor 40, 3, 2
	mtfprwz	2, 30
	xxspltw 41, 2, 1
	vaddubm 8, 8, 3
	stxv 40, 144(4)
	vsum4sbs 8, 8, 31
	lbz 30, 202(6)
	extsb 30, 30
	stw 30, 136(5)
	vmuluwm 8, 9, 8
	vslb 9, 5, 4
	xxland 3, 41, 0
	vadduwm 7, 7, 8
	lxv 40, 96(6)
	xxland 2, 40, 1
	xxlor 41, 3, 2
	mtfprwz	2, 30
	vaddubm 9, 9, 3
	xxspltw 42, 2, 1
	stxv 41, 160(4)
	vsum4sbs 9, 9, 31
	lbz 30, 203(6)
	extsb 30, 30
	stw 30, 140(5)
	vmuluwm 9, 10, 9
	vslb 10, 0, 4
	xxland 2, 42, 0
	vadduwm 7, 7, 9
	lxv 41, 112(6)
	xxland 1, 41, 1
	xxlor 42, 2, 1
	mtfprwz	1, 30
	vaddubm 10, 10, 3
	xxspltw 43, 1, 1
	xxland 1, 37, 0
	vsrb 5, 5, 4
	vsrb 4, 0, 4
	stxv 42, 176(4)
	vsum4sbs 10, 10, 31
	lbz 30, 204(6)
	xxlor 33, 1, 33
	vaddubm 1, 1, 3
	extsb 30, 30
	stxv 33, 192(4)
	stw 30, 144(5)
	vsum4sbs 1, 1, 31
	mtfprwz	1, 30
	lbz 30, 205(6)
	vmuluwm 10, 11, 10
	extsb 30, 30
	stw 30, 148(5)
	vadduwm 7, 7, 10
	xxspltw 42, 1, 1
	xxland 1, 32, 0
	xxlor 38, 1, 38
	mtfprwz	1, 30
	vmuluwm 1, 10, 1
	vaddubm 6, 6, 3
	stxv 38, 208(4)
	vsum4sbs 6, 6, 31
	lbz 30, 206(6)
	vadduwm 1, 7, 1
	xxspltw 39, 1, 1
	xxland 1, 37, 0
	extsb 30, 30
	xxland 0, 36, 0
	stw 30, 152(5)
	vmuluwm 6, 7, 6
	vadduwm 1, 1, 6
	vsrb 6, 8, 2
	vsrb 2, 9, 2
	xxlor 37, 1, 38
	xxlor 34, 0, 34
	mtfprwz	1, 30
	vaddubm 5, 5, 3
	vaddubm 2, 2, 3
	xxspltw 38, 1, 1
	stxv 37, 224(4)
	vsum4sbs 5, 5, 31
	lbz 6, 207(6)
	stxv 34, 240(4)
	extsb 6, 6
	stw 6, 156(5)
	vsum4sbs 2, 2, 31
	mtfprwz	0, 6
	vmuluwm 5, 6, 5
	xxspltw 35, 0, 1
	vmuluwm 2, 3, 2
	vadduwm 5, 1, 5
	vadduwm 2, 5, 2
	b .Lp9_int8_BB2_48
.Lp9_int8_BB2_106:                              #   in Loop: Header=BB2_44 Depth=2
	lwz 28, 888(1)                          # 4-byte Folded Reload
	b .Lp9_int8_BB2_43
.Lp9_int8_BB2_107:                              #   in Loop: Header=BB2_41 Depth=1
	cmplwi	28, 0
	bne	0, .Lp9_int8_BB2_203
# %bb.108:                              #   in Loop: Header=BB2_41 Depth=1
	lwz 4, 628(1)
	ld 10, 528(1)                           # 8-byte Folded Reload
	mtocrf 128, 4
	beq	0, .Lp9_int8_BB2_110
# %bb.109:                              #   in Loop: Header=BB2_41 Depth=1
	lwz 4, 300(1)
	mtocrf 128, 4
	bne	0, .Lp9_int8_BB2_116
.Lp9_int8_BB2_110:                              #   in Loop: Header=BB2_41 Depth=1
	ld 11, 256(1)                           # 8-byte Folded Reload
	ld 4, 896(1)
	ld 5, 904(1)
	ld 6, 912(1)
	li 8, 0
	ld 7, 920(1)
	ld 9, 200(1)                            # 8-byte Folded Reload
	mtctr 11
	ld 16, 664(1)                           # 8-byte Folded Reload
	.p2align	5
.Lp9_int8_BB2_111:                              #   Parent Loop BB2_41 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lxvx 34, 4, 8
	lxvx 35, 5, 8
	lxvx 0, 6, 8
	lxvx 1, 7, 8
	addi 8, 8, 16
	xxmrglw 2, 35, 34
	xxswapd	3, 1
	xxperm 2, 0, 61
	vmrgew 4, 3, 2
	xxperm 36, 0, 60
	xxinsertw 2, 3, 0
	xxsldwi 3, 1, 1, 1
	xxinsertw 36, 3, 0
	xxmrghw 3, 35, 34
	vperm 2, 3, 2, 26
	stxv 2, -32(9)
	xxperm 3, 0, 59
	xxperm 34, 0, 57
	xxsldwi 0, 1, 1, 3
	stxv 36, -16(9)
	xxinsertw 3, 1, 0
	xxinsertw 34, 0, 0
	stxv 3, 0(9)
	stxv 34, 16(9)
	addi 9, 9, 64
	bdnz .Lp9_int8_BB2_111
# %bb.112:                              #   in Loop: Header=BB2_41 Depth=1
	lwz 4, 300(1)
	mtocrf 128, 4
	beq	0, .Lp9_int8_BB2_123
# %bb.113:                              #   in Loop: Header=BB2_41 Depth=1
	lwz 4, 188(1)
	mtocrf 128, 4
	beq	0, .Lp9_int8_BB2_122
# %bb.114:                              #   in Loop: Header=BB2_41 Depth=1
	ld 4, 264(1)                            # 8-byte Folded Reload
	cmplwi	4, 8
	lwz 4, 164(1)                           # 4-byte Folded Reload
                                        # kill: def $r4 killed $r4 def $x4
	bne	0, .Lp9_int8_BB2_124
# %bb.115:                              #   in Loop: Header=BB2_41 Depth=1
	li 4, 128
	b .Lp9_int8_BB2_124
.Lp9_int8_BB2_116:                              #   in Loop: Header=BB2_41 Depth=1
	ld 11, 256(1)                           # 8-byte Folded Reload
	ld 4, 896(1)
	ld 5, 904(1)
	ld 6, 912(1)
	li 8, 0
	ld 7, 920(1)
	ld 9, 176(1)                            # 8-byte Folded Reload
	mtctr 11
	ld 16, 664(1)                           # 8-byte Folded Reload
	.p2align	5
.Lp9_int8_BB2_117:                              #   Parent Loop BB2_41 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lxvx 34, 4, 8
	lxvx 35, 5, 8
	lxvx 0, 6, 8
	lxvx 1, 7, 8
	addi 8, 8, 16
	xxmrglw 2, 35, 34
	xxswapd	3, 1
	xxperm 2, 0, 61
	vmrgew 4, 3, 2
	xxperm 36, 0, 60
	xxinsertw 2, 3, 0
	xxsldwi 3, 1, 1, 1
	xxinsertw 36, 3, 0
	xxmrghw 3, 35, 34
	vperm 2, 3, 2, 26
	stxv 2, -32(9)
	xxperm 3, 0, 59
	xxperm 34, 0, 57
	xxsldwi 0, 1, 1, 3
	stxv 36, -16(9)
	xxinsertw 3, 1, 0
	xxinsertw 34, 0, 0
	stxv 3, 0(9)
	stxv 34, 16(9)
	addi 9, 9, 64
	bdnz .Lp9_int8_BB2_117
# %bb.118:                              #   in Loop: Header=BB2_41 Depth=1
	ld 4, 264(1)                            # 8-byte Folded Reload
	cmpwi	4, 8
	bge 0, .Lp9_int8_BB2_131
# %bb.119:                              #   in Loop: Header=BB2_41 Depth=1
	lwz 4, 300(1)
	mtocrf 128, 4
	beq	0, .Lp9_int8_BB2_134
# %bb.120:                              #   in Loop: Header=BB2_41 Depth=1
	lwz 4, 188(1)
	mtocrf 128, 4
	bne	0, .Lp9_int8_BB2_143
# %bb.121:                              #   in Loop: Header=BB2_41 Depth=1
	li 4, 16
	b .Lp9_int8_BB2_136
.Lp9_int8_BB2_122:                              #   in Loop: Header=BB2_41 Depth=1
	li 4, 16
	b .Lp9_int8_BB2_124
.Lp9_int8_BB2_123:                              #   in Loop: Header=BB2_41 Depth=1
	li 4, 8
.Lp9_int8_BB2_124:                              #   in Loop: Header=BB2_41 Depth=1
	lwz 5, 212(1)
	mtocrf 128, 5
	bc 12, 0, .Lp9_int8_BB2_143
# %bb.125:                              #   in Loop: Header=BB2_41 Depth=1
	lwz 5, 168(1)
	mtvsrd 34, 4
	li 4, 0
	vspltb 2, 2, 7
	mtocrf 128, 5
	bc 12, 0, .Lp9_int8_BB2_128
# %bb.126:                              #   in Loop: Header=BB2_41 Depth=1
	ld 7, 80(1)                             # 8-byte Folded Reload
	ld 5, 176(1)                            # 8-byte Folded Reload
	ld 6, 72(1)                             # 8-byte Folded Reload
	mtctr 7
.Lp9_int8_BB2_127:                              #   Parent Loop BB2_41 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lxv 35, -64(6)
	xxspltib 0, 15
	lxv 36, -48(6)
	xxspltib 37, 4
	lxv 32, -32(6)
	addi 4, 4, 4
	vaddubm 3, 3, 2
	vaddubm 0, 0, 2
	xxland 1, 35, 0
	vaddubm 3, 4, 2
	xxland 2, 32, 0
	vslb 3, 3, 5
	xxlor 1, 35, 1
	lxv 35, -16(6)
	stxv 1, -32(5)
	vaddubm 3, 3, 2
	vslb 3, 3, 5
	xxlor 2, 35, 2
	lxv 35, 0(6)
	stxv 2, -16(5)
	vaddubm 3, 3, 2
	xxland 3, 35, 0
	lxv 35, 16(6)
	vaddubm 3, 3, 2
	vslb 3, 3, 5
	xxlor 1, 35, 3
	lxv 35, 32(6)
	stxv 1, 0(5)
	vaddubm 3, 3, 2
	xxland 0, 35, 0
	lxv 35, 48(6)
	addi 6, 6, 128
	vaddubm 3, 3, 2
	vslb 3, 3, 5
	xxlor 0, 35, 0
	stxv 0, 16(5)
	addi 5, 5, 64
	bdnz .Lp9_int8_BB2_127
.Lp9_int8_BB2_128:                              #   in Loop: Header=BB2_41 Depth=1
	lwz 5, 172(1)
	mtocrf 128, 5
	bc 12, 2, .Lp9_int8_BB2_143
# %bb.129:                              #   in Loop: Header=BB2_41 Depth=1
	ld 6, 96(1)                             # 8-byte Folded Reload
	sldi 5, 4, 4
	sldi 4, 4, 5
	add 5, 10, 5
	add 4, 6, 4
	ld 6, 104(1)                            # 8-byte Folded Reload
	mtctr 6
.Lp9_int8_BB2_130:                              #   Parent Loop BB2_41 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lxv 35, -16(4)
	xxspltib 0, 15
	lxv 36, 0(4)
	addi 4, 4, 32
	vaddubm 3, 3, 2
	vaddubm 4, 4, 2
	xxland 0, 35, 0
	xxspltib 35, 4
	vslb 3, 4, 3
	xxlor 0, 35, 0
	stxv 0, 0(5)
	addi 5, 5, 16
	bdnz .Lp9_int8_BB2_130
	b .Lp9_int8_BB2_143
.Lp9_int8_BB2_131:                              #   in Loop: Header=BB2_41 Depth=1
	beq	0, .Lp9_int8_BB2_135
# %bb.132:                              #   in Loop: Header=BB2_41 Depth=1
	lwz 4, 276(1)
	mtocrf 128, 4
	bne	0, .Lp9_int8_BB2_143
# %bb.133:                              #   in Loop: Header=BB2_41 Depth=1
	li 4, 1
	b .Lp9_int8_BB2_136
.Lp9_int8_BB2_134:                              #   in Loop: Header=BB2_41 Depth=1
	li 4, 8
	b .Lp9_int8_BB2_136
.Lp9_int8_BB2_135:                              #   in Loop: Header=BB2_41 Depth=1
	li 4, 128
.Lp9_int8_BB2_136:                              #   in Loop: Header=BB2_41 Depth=1
	lwz 5, 124(1)
	mtocrf 128, 5
	bc 12, 2, .Lp9_int8_BB2_143
# %bb.137:                              #   in Loop: Header=BB2_41 Depth=1
	lwz 5, 92(1)
	mtvsrd 34, 4
	li 4, 0
	vspltb 2, 2, 7
	mtocrf 128, 5
	bc 12, 0, .Lp9_int8_BB2_140
# %bb.138:                              #   in Loop: Header=BB2_41 Depth=1
	ld 4, 56(1)                             # 8-byte Folded Reload
	mr	5, 10
	mtctr 4
	li 4, 0
.Lp9_int8_BB2_139:                              #   Parent Loop BB2_41 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lxv 35, 0(5)
	lxv 36, 16(5)
	addi 4, 4, 128
	vaddubm 3, 3, 2
	vaddubm 4, 4, 2
	stxv 35, 0(5)
	lxv 35, 32(5)
	stxv 36, 16(5)
	lxv 36, 48(5)
	vaddubm 3, 3, 2
	vaddubm 4, 4, 2
	stxv 35, 32(5)
	lxv 35, 64(5)
	stxv 36, 48(5)
	lxv 36, 96(5)
	vaddubm 3, 3, 2
	vaddubm 4, 4, 2
	stxv 35, 64(5)
	lxv 35, 80(5)
	stxv 36, 96(5)
	vaddubm 3, 3, 2
	stxv 35, 80(5)
	lxv 35, 112(5)
	vaddubm 3, 3, 2
	stxv 35, 112(5)
	addi 5, 5, 128
	bdnz .Lp9_int8_BB2_139
.Lp9_int8_BB2_140:                              #   in Loop: Header=BB2_41 Depth=1
	lwz 5, 88(1)
	mtocrf 128, 5
	bc 12, 2, .Lp9_int8_BB2_143
# %bb.141:                              #   in Loop: Header=BB2_41 Depth=1
	ld 5, 64(1)                             # 8-byte Folded Reload
	add 4, 10, 4
	mtctr 5
.Lp9_int8_BB2_142:                              #   Parent Loop BB2_41 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lxv 35, 0(4)
	vaddubm 3, 3, 2
	stxv 35, 0(4)
	addi 4, 4, 16
	bdnz .Lp9_int8_BB2_142
.Lp9_int8_BB2_143:                              #   in Loop: Header=BB2_41 Depth=1
	bc 12, 26, .Lp9_int8_BB2_191
# %bb.144:                              #   in Loop: Header=BB2_41 Depth=1
	ld 8, 696(1)                            # 8-byte Folded Reload
	ori 4, 3, 1
	ld 7, 704(1)                            # 8-byte Folded Reload
	li 20, 0
	ori 5, 3, 2
	ori 3, 3, 3
	ld 14, 192(1)                           # 8-byte Folded Reload
	mr	24, 21
	cmpld	4, 8
	isellt	4, 7, 20
	cmpld	5, 8
	li 5, 2
	isellt	5, 5, 20
	cmpld	3, 8
	mulli 9, 4, 92
	ld 8, 520(1)                            # 8-byte Folded Reload
	li 3, 3
	isellt	3, 3, 20
	mulld 6, 5, 7
	mulld 4, 8, 5
	mulld 7, 3, 7
	maddld 18, 8, 5, 21
	mulld 5, 8, 3
	maddld 31, 8, 3, 21
	addi 3, 9, 4
	add 19, 21, 9
	std 9, 616(1)                           # 8-byte Folded Spill
	std 3, 592(1)                           # 8-byte Folded Spill
	addi 3, 9, -24
	std 3, 568(1)                           # 8-byte Folded Spill
	addi 3, 4, 4
	std 3, 584(1)                           # 8-byte Folded Spill
	addi 3, 4, -24
	std 3, 560(1)                           # 8-byte Folded Spill
	addi 3, 5, 4
	std 3, 576(1)                           # 8-byte Folded Spill
	addi 3, 5, -24
	std 3, 552(1)                           # 8-byte Folded Spill
	mulli 3, 6, 92
	std 3, 608(1)                           # 8-byte Folded Spill
	mulli 3, 7, 92
	std 3, 600(1)                           # 8-byte Folded Spill
	b .Lp9_int8_BB2_148
.Lp9_int8_BB2_145:                              #   in Loop: Header=BB2_148 Depth=2
	lwz 5, 832(1)                           # 4-byte Folded Reload
	lwz 3, 792(1)                           # 4-byte Folded Reload
	sth 21, 60(4)
	sth 16, 62(4)
	ld 21, 672(1)                           # 8-byte Folded Reload
	ld 16, 664(1)                           # 8-byte Folded Reload
	stfs 0, 16(4)
	stfs 2, 20(4)
	stfs 4, 24(4)
	stfs 6, 28(4)
	sth 22, 40(4)
	sth 15, 42(4)
	sth 7, 44(4)
	sth 8, 46(4)
	sth 23, 56(4)
	sth 17, 58(4)
	sth 12, 68(4)
	sth 0, 70(4)
	sth 27, 72(4)
	sth 26, 74(4)
	sth 29, 76(4)
	sth 25, 78(4)
	sth 10, 84(4)
	sth 11, 86(4)
	sth 28, 90(4)
	sth 30, 92(4)
	sth 6, 94(4)
	sth 5, 32(4)
	lwz 5, 824(1)                           # 4-byte Folded Reload
	sth 3, 36(4)
	lwz 3, 784(1)                           # 4-byte Folded Reload
	sth 5, 34(4)
	lwz 5, 848(1)                           # 4-byte Folded Reload
	sth 3, 38(4)
	lwz 3, 808(1)                           # 4-byte Folded Reload
	sth 5, 48(4)
	lwz 5, 840(1)                           # 4-byte Folded Reload
	sth 3, 52(4)
	lwz 3, 800(1)                           # 4-byte Folded Reload
	sth 5, 50(4)
	lwz 5, 864(1)                           # 4-byte Folded Reload
	sth 3, 54(4)
	lwz 3, 640(1)                           # 4-byte Folded Reload
	sth 5, 64(4)
	lwz 5, 856(1)                           # 4-byte Folded Reload
	sth 3, 88(4)
	sth 5, 66(4)
	lwz 5, 880(1)                           # 4-byte Folded Reload
	sth 5, 80(4)
	lwz 5, 872(1)                           # 4-byte Folded Reload
	sth 5, 82(4)
.Lp9_int8_BB2_146:                              #   in Loop: Header=BB2_148 Depth=2
	ld 3, 496(1)                            # 8-byte Folded Reload
	ld 5, 488(1)                            # 8-byte Folded Reload
	add 3, 4, 3
	addi 4, 1, 928
	bl memcpy
	nop
	lwz 3, 508(1)
	rotlwi	3, 3, 4
	mtocrf 1, 3
.Lp9_int8_BB2_147:                              #   in Loop: Header=BB2_148 Depth=2
	ld 3, 704(1)                            # 8-byte Folded Reload
	addi 20, 20, 1
	addi 14, 14, 92
	addi 24, 24, 92
	addi 19, 19, 92
	addi 18, 18, 92
	addi 31, 31, 92
	cmpld	20, 3
	beq	0, .Lp9_int8_BB2_191
.Lp9_int8_BB2_148:                              #   Parent Loop BB2_41 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB2_154 Depth 3
                                        #       Child Loop BB2_157 Depth 3
                                        #       Child Loop BB2_164 Depth 3
                                        #       Child Loop BB2_167 Depth 3
                                        #       Child Loop BB2_174 Depth 3
                                        #       Child Loop BB2_177 Depth 3
                                        #       Child Loop BB2_184 Depth 3
                                        #       Child Loop BB2_187 Depth 3
	mulli 4, 20, 92
	mr	3, 21
	lfsux 1, 3, 4
	mr	5, 3
	lwz 6, 88(3)
	stw 6, 888(1)                           # 4-byte Folded Spill
	lfsu 0, 4(5)
	li 6, 0
	bc 12, 8, .Lp9_int8_BB2_152
# %bb.149:                              #   in Loop: Header=BB2_148 Depth=2
	lwz 6, 8(3)
	lwz 7, 12(3)
	lwz 8, 16(3)
	lwz 9, 24(3)
	lwz 10, 28(3)
	lwz 11, 36(3)
	lwz 12, 40(3)
	lwz 0, 44(3)
	lwz 30, 48(3)
	stw 6, 928(1)
	lwz 6, 20(3)
	stw 7, 944(1)
	lwz 7, 32(3)
	stw 8, 960(1)
	lwz 8, 52(3)
	stw 9, 992(1)
	stw 10, 1008(1)
	stw 11, 1040(1)
	stw 12, 1056(1)
	stw 0, 1072(1)
	stw 30, 1088(1)
	stw 6, 976(1)
	stw 7, 1024(1)
	stw 8, 1104(1)
	lwz 6, 716(1)
	mtocrf 128, 6
	bc 12, 2, .Lp9_int8_BB2_151
# %bb.150:                              #   in Loop: Header=BB2_148 Depth=2
	lwz 6, 56(3)
	lwz 7, 60(3)
	lwz 8, 64(3)
	lwz 9, 72(3)
	lwz 10, 76(3)
	lwz 11, 84(3)
	lwz 12, 88(3)
	lwz 0, 92(3)
	lwz 30, 96(3)
	stw 6, 1120(1)
	lwz 6, 68(3)
	stw 7, 1136(1)
	lwz 7, 80(3)
	stw 8, 1152(1)
	lwz 8, 100(3)
	stw 9, 1184(1)
	stw 10, 1200(1)
	stw 11, 1232(1)
	stw 12, 1248(1)
	stw 0, 1264(1)
	stw 30, 1280(1)
	stw 6, 1168(1)
	stw 7, 1216(1)
	stw 8, 1296(1)
.Lp9_int8_BB2_151:                              #   in Loop: Header=BB2_148 Depth=2
	ld 6, 720(1)                            # 8-byte Folded Reload
	bc 12, 30, .Lp9_int8_BB2_158
.Lp9_int8_BB2_152:                              #   in Loop: Header=BB2_148 Depth=2
	sub	7, 16, 6
	mr	8, 6
	andi. 7, 7, 7
	beq	0, .Lp9_int8_BB2_155
# %bb.153:                              #   in Loop: Header=BB2_148 Depth=2
	sldi 9, 6, 2
	sldi 10, 6, 4
	add 8, 6, 7
	add 5, 5, 9
	ld 9, 456(1)                            # 8-byte Folded Reload
	mtctr 7
	add 9, 9, 10
	.p2align	5
.Lp9_int8_BB2_154:                              #   Parent Loop BB2_41 Depth=1
                                        #     Parent Loop BB2_148 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lwzu 10, 4(5)
	stwu 10, 16(9)
	bdnz .Lp9_int8_BB2_154
.Lp9_int8_BB2_155:                              #   in Loop: Header=BB2_148 Depth=2
	sub	5, 6, 16
	li 9, -8
	cmpld	5, 9
	bgt	0, .Lp9_int8_BB2_158
# %bb.156:                              #   in Loop: Header=BB2_148 Depth=2
	addi 5, 3, -24
	sldi 9, 8, 2
	sldi 8, 8, 4
	add 5, 5, 9
	ld 9, 408(1)                            # 8-byte Folded Reload
	add 8, 9, 8
	ld 9, 688(1)                            # 8-byte Folded Reload
	sub	6, 9, 6
	sub	6, 6, 7
	rldicl 6, 6, 61, 3
	addi 6, 6, 1
	mtctr 6
	.p2align	5
.Lp9_int8_BB2_157:                              #   Parent Loop BB2_41 Depth=1
                                        #     Parent Loop BB2_148 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lwzu 6, 32(5)
	stwu 6, 128(8)
	lwz 6, 4(5)
	stw 6, 16(8)
	lwz 6, 8(5)
	stw 6, 32(8)
	lwz 6, 12(5)
	stw 6, 48(8)
	lwz 6, 16(5)
	stw 6, 64(8)
	lwz 6, 20(5)
	stw 6, 80(8)
	lwz 6, 24(5)
	stw 6, 96(8)
	lwz 6, 28(5)
	stw 6, 112(8)
	bdnz .Lp9_int8_BB2_157
.Lp9_int8_BB2_158:                              #   in Loop: Header=BB2_148 Depth=2
	lhz 6, 72(3)
	ld 5, 616(1)                            # 8-byte Folded Reload
	stw 6, 832(1)                           # 4-byte Folded Spill
	lhz 6, 74(3)
	add 5, 3, 5
	lfs 3, 0(5)
	stw 6, 824(1)                           # 4-byte Folded Spill
	lhz 6, 76(3)
	lfs 2, 4(5)
	stw 6, 848(1)                           # 4-byte Folded Spill
	lhz 6, 78(3)
	stw 6, 840(1)                           # 4-byte Folded Spill
	lhz 6, 80(3)
	stw 6, 864(1)                           # 4-byte Folded Spill
	lhz 6, 82(3)
	stw 6, 856(1)                           # 4-byte Folded Spill
	lhz 6, 84(3)
	stw 6, 880(1)                           # 4-byte Folded Spill
	lhz 6, 86(3)
	stw 6, 872(1)                           # 4-byte Folded Spill
	lwz 6, 88(5)
	stw 6, 816(1)                           # 4-byte Folded Spill
	li 6, 0
	bc 12, 8, .Lp9_int8_BB2_162
# %bb.159:                              #   in Loop: Header=BB2_148 Depth=2
	lwz 6, 8(5)
	lwz 7, 12(5)
	lwz 8, 16(5)
	lwz 9, 24(5)
	lwz 10, 28(5)
	lwz 11, 36(5)
	lwz 12, 40(5)
	lwz 0, 44(5)
	lwz 30, 48(5)
	stw 6, 932(1)
	lwz 6, 20(5)
	stw 7, 948(1)
	lwz 7, 32(5)
	stw 8, 964(1)
	lwz 8, 52(5)
	stw 9, 996(1)
	stw 10, 1012(1)
	stw 11, 1044(1)
	stw 12, 1060(1)
	stw 0, 1076(1)
	stw 30, 1092(1)
	stw 6, 980(1)
	stw 7, 1028(1)
	stw 8, 1108(1)
	lwz 6, 716(1)
	mtocrf 128, 6
	bc 12, 2, .Lp9_int8_BB2_161
# %bb.160:                              #   in Loop: Header=BB2_148 Depth=2
	lwz 6, 56(5)
	lwz 7, 60(5)
	lwz 8, 64(5)
	lwz 9, 72(5)
	lwz 10, 76(5)
	lwz 11, 84(5)
	lwz 12, 88(5)
	lwz 0, 92(5)
	lwz 30, 96(5)
	stw 6, 1124(1)
	lwz 6, 68(5)
	stw 7, 1140(1)
	lwz 7, 80(5)
	stw 8, 1156(1)
	lwz 8, 100(5)
	stw 9, 1188(1)
	stw 10, 1204(1)
	stw 11, 1236(1)
	stw 12, 1252(1)
	stw 0, 1268(1)
	stw 30, 1284(1)
	stw 6, 1172(1)
	stw 7, 1220(1)
	stw 8, 1300(1)
.Lp9_int8_BB2_161:                              #   in Loop: Header=BB2_148 Depth=2
	ld 6, 720(1)                            # 8-byte Folded Reload
	bc 12, 30, .Lp9_int8_BB2_168
.Lp9_int8_BB2_162:                              #   in Loop: Header=BB2_148 Depth=2
	sub	7, 16, 6
	mr	8, 6
	andi. 7, 7, 7
	beq	0, .Lp9_int8_BB2_165
# %bb.163:                              #   in Loop: Header=BB2_148 Depth=2
	ld 8, 592(1)                            # 8-byte Folded Reload
	sldi 10, 6, 2
	sldi 11, 6, 4
	add 8, 8, 4
	add 9, 21, 8
	add 8, 6, 7
	add 9, 9, 10
	ld 10, 448(1)                           # 8-byte Folded Reload
	mtctr 7
	add 10, 10, 11
	.p2align	5
.Lp9_int8_BB2_164:                              #   Parent Loop BB2_41 Depth=1
                                        #     Parent Loop BB2_148 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lwzu 11, 4(9)
	stwu 11, 16(10)
	bdnz .Lp9_int8_BB2_164
.Lp9_int8_BB2_165:                              #   in Loop: Header=BB2_148 Depth=2
	sub	9, 6, 16
	li 10, -8
	cmpld	9, 10
	bgt	0, .Lp9_int8_BB2_168
# %bb.166:                              #   in Loop: Header=BB2_148 Depth=2
	ld 9, 568(1)                            # 8-byte Folded Reload
	sldi 10, 8, 2
	sldi 11, 8, 4
	add 9, 9, 4
	add 9, 21, 9
	add 8, 9, 10
	ld 10, 688(1)                           # 8-byte Folded Reload
	ld 9, 400(1)                            # 8-byte Folded Reload
	sub	6, 10, 6
	add 9, 9, 11
	sub	6, 6, 7
	rldicl 6, 6, 61, 3
	addi 6, 6, 1
	mtctr 6
	.p2align	5
.Lp9_int8_BB2_167:                              #   Parent Loop BB2_41 Depth=1
                                        #     Parent Loop BB2_148 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lwzu 6, 32(8)
	stwu 6, 128(9)
	lwz 6, 4(8)
	stw 6, 16(9)
	lwz 6, 8(8)
	stw 6, 32(9)
	lwz 6, 12(8)
	stw 6, 48(9)
	lwz 6, 16(8)
	stw 6, 64(9)
	lwz 6, 20(8)
	stw 6, 80(9)
	lwz 6, 24(8)
	stw 6, 96(9)
	lwz 6, 28(8)
	stw 6, 112(9)
	bdnz .Lp9_int8_BB2_167
.Lp9_int8_BB2_168:                              #   in Loop: Header=BB2_148 Depth=2
	ld 6, 608(1)                            # 8-byte Folded Reload
	lhz 12, 80(5)
	lhz 0, 82(5)
	lhz 10, 84(5)
	add 28, 3, 6
	lhz 6, 72(5)
	lhz 11, 86(5)
	stw 6, 792(1)                           # 4-byte Folded Spill
	lhz 6, 74(5)
	lfs 5, 0(28)
	lfs 4, 4(28)
	stw 6, 784(1)                           # 4-byte Folded Spill
	lhz 6, 76(5)
	stw 6, 808(1)                           # 4-byte Folded Spill
	lhz 6, 78(5)
	lwz 5, 88(28)
	stw 5, 648(1)                           # 4-byte Folded Spill
	li 5, 0
	stw 6, 800(1)                           # 4-byte Folded Spill
	bc 12, 8, .Lp9_int8_BB2_172
# %bb.169:                              #   in Loop: Header=BB2_148 Depth=2
	lwz 5, 8(28)
	lwz 6, 12(28)
	lwz 7, 16(28)
	lwz 8, 24(28)
	lwz 30, 28(28)
	lwz 29, 36(28)
	lwz 27, 40(28)
	lwz 26, 44(28)
	lwz 25, 48(28)
	stw 5, 936(1)
	lwz 5, 20(28)
	stw 6, 952(1)
	lwz 6, 32(28)
	stw 7, 968(1)
	lwz 7, 52(28)
	stw 8, 1000(1)
	stw 30, 1016(1)
	stw 29, 1048(1)
	stw 27, 1064(1)
	stw 26, 1080(1)
	stw 25, 1096(1)
	stw 5, 984(1)
	stw 6, 1032(1)
	stw 7, 1112(1)
	lwz 5, 716(1)
	mtocrf 128, 5
	bc 12, 2, .Lp9_int8_BB2_171
# %bb.170:                              #   in Loop: Header=BB2_148 Depth=2
	lwz 5, 56(28)
	lwz 6, 60(28)
	lwz 7, 64(28)
	lwz 8, 72(28)
	lwz 30, 76(28)
	lwz 29, 84(28)
	lwz 27, 88(28)
	lwz 26, 92(28)
	lwz 25, 96(28)
	stw 5, 1128(1)
	lwz 5, 68(28)
	stw 6, 1144(1)
	lwz 6, 80(28)
	stw 7, 1160(1)
	lwz 7, 100(28)
	stw 8, 1192(1)
	stw 30, 1208(1)
	stw 29, 1240(1)
	stw 27, 1256(1)
	stw 26, 1272(1)
	stw 25, 1288(1)
	stw 5, 1176(1)
	stw 6, 1224(1)
	stw 7, 1304(1)
.Lp9_int8_BB2_171:                              #   in Loop: Header=BB2_148 Depth=2
	ld 5, 720(1)                            # 8-byte Folded Reload
	bc 12, 30, .Lp9_int8_BB2_178
.Lp9_int8_BB2_172:                              #   in Loop: Header=BB2_148 Depth=2
	sub	6, 16, 5
	mr	30, 5
	andi. 6, 6, 7
	beq	0, .Lp9_int8_BB2_175
# %bb.173:                              #   in Loop: Header=BB2_148 Depth=2
	ld 7, 584(1)                            # 8-byte Folded Reload
	sldi 8, 5, 2
	sldi 29, 5, 4
	add 30, 5, 6
	add 7, 7, 4
	add 7, 21, 7
	add 7, 7, 8
	ld 8, 440(1)                            # 8-byte Folded Reload
	mtctr 6
	add 8, 8, 29
	.p2align	5
.Lp9_int8_BB2_174:                              #   Parent Loop BB2_41 Depth=1
                                        #     Parent Loop BB2_148 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lwzu 29, 4(7)
	stwu 29, 16(8)
	bdnz .Lp9_int8_BB2_174
.Lp9_int8_BB2_175:                              #   in Loop: Header=BB2_148 Depth=2
	sub	7, 5, 16
	li 8, -8
	cmpld	7, 8
	bgt	0, .Lp9_int8_BB2_178
# %bb.176:                              #   in Loop: Header=BB2_148 Depth=2
	ld 7, 560(1)                            # 8-byte Folded Reload
	sldi 8, 30, 2
	sldi 29, 30, 4
	add 7, 7, 4
	add 7, 21, 7
	add 30, 7, 8
	ld 7, 392(1)                            # 8-byte Folded Reload
	add 29, 7, 29
	ld 7, 688(1)                            # 8-byte Folded Reload
	sub	5, 7, 5
	sub	5, 5, 6
	rldicl 5, 5, 61, 3
	addi 5, 5, 1
	mtctr 5
	.p2align	5
.Lp9_int8_BB2_177:                              #   Parent Loop BB2_41 Depth=1
                                        #     Parent Loop BB2_148 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lwzu 5, 32(30)
	stwu 5, 128(29)
	lwz 5, 4(30)
	stw 5, 16(29)
	lwz 5, 8(30)
	stw 5, 32(29)
	lwz 5, 12(30)
	stw 5, 48(29)
	lwz 5, 16(30)
	stw 5, 64(29)
	lwz 5, 20(30)
	stw 5, 80(29)
	lwz 5, 24(30)
	stw 5, 96(29)
	lwz 5, 28(30)
	stw 5, 112(29)
	bdnz .Lp9_int8_BB2_177
.Lp9_int8_BB2_178:                              #   in Loop: Header=BB2_148 Depth=2
	ld 5, 600(1)                            # 8-byte Folded Reload
	lhz 22, 72(28)
	lhz 15, 74(28)
	lhz 23, 76(28)
	li 30, 0
	add 6, 3, 5
	lhz 17, 78(28)
	lhz 27, 80(28)
	lhz 26, 82(28)
	lhz 3, 84(28)
	lhz 28, 86(28)
	lfs 7, 0(6)
	lfs 6, 4(6)
	lwz 5, 88(6)
	stw 3, 640(1)                           # 4-byte Folded Spill
	bc 12, 8, .Lp9_int8_BB2_182
# %bb.179:                              #   in Loop: Header=BB2_148 Depth=2
	lwz 7, 8(6)
	lwz 8, 12(6)
	lwz 30, 16(6)
	mr	3, 0
	mr	0, 12
	mr	12, 11
	mr	11, 10
	lwz 29, 24(6)
	lwz 25, 28(6)
	lwz 21, 36(6)
	lwz 16, 40(6)
	lwz 9, 44(6)
	lwz 10, 48(6)
	stw 7, 940(1)
	lwz 7, 20(6)
	stw 8, 956(1)
	lwz 8, 32(6)
	stw 30, 972(1)
	lwz 30, 52(6)
	stw 29, 1004(1)
	stw 25, 1020(1)
	stw 21, 1052(1)
	stw 16, 1068(1)
	stw 9, 1084(1)
	stw 10, 1100(1)
	stw 7, 988(1)
	stw 8, 1036(1)
	stw 30, 1116(1)
	lwz 7, 716(1)
	mtocrf 128, 7
	bc 12, 2, .Lp9_int8_BB2_181
# %bb.180:                              #   in Loop: Header=BB2_148 Depth=2
	lwz 7, 56(6)
	lwz 8, 60(6)
	lwz 9, 64(6)
	lwz 10, 72(6)
	lwz 30, 76(6)
	lwz 29, 84(6)
	lwz 25, 88(6)
	lwz 21, 92(6)
	lwz 16, 96(6)
	stw 7, 1132(1)
	lwz 7, 68(6)
	stw 8, 1148(1)
	lwz 8, 80(6)
	stw 9, 1164(1)
	lwz 9, 100(6)
	stw 10, 1196(1)
	stw 30, 1212(1)
	stw 29, 1244(1)
	stw 25, 1260(1)
	stw 21, 1276(1)
	stw 16, 1292(1)
	stw 7, 1180(1)
	stw 8, 1228(1)
	stw 9, 1308(1)
.Lp9_int8_BB2_181:                              #   in Loop: Header=BB2_148 Depth=2
	ld 30, 720(1)                           # 8-byte Folded Reload
	ld 21, 672(1)                           # 8-byte Folded Reload
	ld 16, 664(1)                           # 8-byte Folded Reload
	mr	10, 11
	mr	11, 12
	mr	12, 0
	mr	0, 3
	bc 12, 30, .Lp9_int8_BB2_188
.Lp9_int8_BB2_182:                              #   in Loop: Header=BB2_148 Depth=2
	sub	7, 16, 30
	mr	25, 30
	andi. 29, 7, 7
	beq	0, .Lp9_int8_BB2_185
# %bb.183:                              #   in Loop: Header=BB2_148 Depth=2
	ld 7, 576(1)                            # 8-byte Folded Reload
	sldi 8, 30, 2
	sldi 9, 30, 4
	add 25, 30, 29
	add 7, 7, 4
	add 7, 21, 7
	add 7, 7, 8
	ld 8, 432(1)                            # 8-byte Folded Reload
	mtctr 29
	add 8, 8, 9
	.p2align	5
.Lp9_int8_BB2_184:                              #   Parent Loop BB2_41 Depth=1
                                        #     Parent Loop BB2_148 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lwzu 9, 4(7)
	stwu 9, 16(8)
	bdnz .Lp9_int8_BB2_184
.Lp9_int8_BB2_185:                              #   in Loop: Header=BB2_148 Depth=2
	sub	7, 30, 16
	li 8, -8
	cmpld	7, 8
	bgt	0, .Lp9_int8_BB2_188
# %bb.186:                              #   in Loop: Header=BB2_148 Depth=2
	ld 7, 552(1)                            # 8-byte Folded Reload
	sldi 8, 25, 4
	add 4, 7, 4
	sldi 7, 25, 2
	add 4, 21, 4
	add 4, 4, 7
	ld 7, 384(1)                            # 8-byte Folded Reload
	add 25, 7, 8
	ld 7, 688(1)                            # 8-byte Folded Reload
	sub	7, 7, 30
	sub	7, 7, 29
	rldicl 7, 7, 61, 3
	addi 7, 7, 1
	mtctr 7
	.p2align	5
.Lp9_int8_BB2_187:                              #   Parent Loop BB2_41 Depth=1
                                        #     Parent Loop BB2_148 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lwzu 7, 32(4)
	stwu 7, 128(25)
	lwz 7, 4(4)
	stw 7, 16(25)
	lwz 7, 8(4)
	stw 7, 32(25)
	lwz 7, 12(4)
	stw 7, 48(25)
	lwz 7, 16(4)
	stw 7, 64(25)
	lwz 7, 20(4)
	stw 7, 80(25)
	lwz 7, 24(4)
	stw 7, 96(25)
	lwz 7, 28(4)
	stw 7, 112(25)
	bdnz .Lp9_int8_BB2_187
.Lp9_int8_BB2_188:                              #   in Loop: Header=BB2_148 Depth=2
	ld 4, 776(1)                            # 8-byte Folded Reload
	lhz 8, 74(6)
	lhz 21, 76(6)
	lhz 16, 78(6)
	mulld 7, 20, 4
	ld 4, 632(1)                            # 8-byte Folded Reload
	lhz 29, 80(6)
	lhz 25, 82(6)
	lhz 30, 84(6)
	stfsux 1, 4, 7
	lhz 7, 72(6)
	lhz 6, 86(6)
	stfs 3, 4(4)
	stfs 5, 8(4)
	stfs 7, 12(4)
	lwz 3, 628(1)
	mtocrf 128, 3
	beq	0, .Lp9_int8_BB2_145
# %bb.189:                              #   in Loop: Header=BB2_148 Depth=2
	lwz 3, 548(1)
	ld 21, 672(1)                           # 8-byte Folded Reload
	ld 16, 664(1)                           # 8-byte Folded Reload
	mtocrf 128, 3
	bne	0, .Lp9_int8_BB2_147
# %bb.190:                              #   in Loop: Header=BB2_148 Depth=2
	lwz 3, 888(1)                           # 4-byte Folded Reload
	stw 5, 28(4)
	stw 3, 16(4)
	lwz 3, 816(1)                           # 4-byte Folded Reload
	stw 3, 20(4)
	lwz 3, 648(1)                           # 4-byte Folded Reload
	stw 3, 24(4)
	b .Lp9_int8_BB2_146
.Lp9_int8_BB2_191:                              #   in Loop: Header=BB2_41 Depth=1
	ld 6, 376(1)                            # 8-byte Folded Reload
	ld 3, 216(1)                            # 8-byte Folded Reload
	ld 5, 248(1)                            # 8-byte Folded Reload
	addi 4, 6, 1
	and 11, 6, 3
	std 4, 304(1)                           # 8-byte Folded Spill
	cmpld	4, 5
	ld 4, 224(1)                            # 8-byte Folded Reload
	addi 3, 11, 1
	iseleq	3, 4, 3
	cmpld	11, 3
	std 3, 480(1)                           # 8-byte Folded Spill
	bge 0, .Lp9_int8_BB2_40
# %bb.192:                              #   in Loop: Header=BB2_41 Depth=1
	bc 12, 10, .Lp9_int8_BB2_40
# %bb.193:                              #   in Loop: Header=BB2_41 Depth=1
	ld 3, 152(1)                            # 8-byte Folded Reload
	ld 4, 704(1)                            # 8-byte Folded Reload
	ld 9, 472(1)                            # 8-byte Folded Reload
	ld 10, 768(1)                           # 8-byte Folded Reload
	srd 3, 6, 3
	ld 8, 776(1)                            # 8-byte Folded Reload
	mulld 12, 3, 4
	ld 4, 144(1)                            # 8-byte Folded Reload
	maddld 4, 8, 11, 4
	addi 5, 12, 3
	std 12, 376(1)                          # 8-byte Folded Spill
	mulld 6, 9, 5
	maddld 5, 9, 5, 4
	maddld 6, 10, 11, 6
	std 5, 648(1)                           # 8-byte Folded Spill
	mulld 5, 10, 11
	std 6, 640(1)                           # 8-byte Folded Spill
	addi 6, 12, 2
	mulld 7, 9, 6
	maddld 6, 9, 6, 4
	std 6, 616(1)                           # 8-byte Folded Spill
	maddld 6, 10, 11, 7
	std 6, 608(1)                           # 8-byte Folded Spill
	addi 6, 12, 1
	mulld 7, 9, 6
	maddld 6, 9, 6, 4
	std 6, 600(1)                           # 8-byte Folded Spill
	maddld 6, 10, 11, 7
	std 6, 592(1)                           # 8-byte Folded Spill
	ld 6, 128(1)                            # 8-byte Folded Reload
	maddld 4, 6, 3, 4
	maddld 3, 6, 3, 5
	std 3, 576(1)                           # 8-byte Folded Spill
	ld 3, 136(1)                            # 8-byte Folded Reload
	std 4, 584(1)                           # 8-byte Folded Spill
	maddld 25, 8, 11, 3
	ld 3, 464(1)                            # 8-byte Folded Reload
	maddld 24, 10, 11, 3
	b .Lp9_int8_BB2_195
	.p2align	4
.Lp9_int8_BB2_194:                              #   in Loop: Header=BB2_195 Depth=2
	ld 3, 648(1)                            # 8-byte Folded Reload
	ld 11, 568(1)                           # 8-byte Folded Reload
	add 25, 25, 28
	add 24, 24, 26
	add 3, 3, 28
	addi 11, 11, 1
	std 3, 648(1)                           # 8-byte Folded Spill
	ld 3, 640(1)                            # 8-byte Folded Reload
	add 3, 3, 26
	std 3, 640(1)                           # 8-byte Folded Spill
	ld 3, 616(1)                            # 8-byte Folded Reload
	add 3, 3, 28
	std 3, 616(1)                           # 8-byte Folded Spill
	ld 3, 608(1)                            # 8-byte Folded Reload
	add 3, 3, 26
	std 3, 608(1)                           # 8-byte Folded Spill
	ld 3, 600(1)                            # 8-byte Folded Reload
	add 3, 3, 28
	std 3, 600(1)                           # 8-byte Folded Spill
	ld 3, 592(1)                            # 8-byte Folded Reload
	add 3, 3, 26
	std 3, 592(1)                           # 8-byte Folded Spill
	ld 3, 584(1)                            # 8-byte Folded Reload
	add 3, 3, 28
	std 3, 584(1)                           # 8-byte Folded Spill
	ld 3, 576(1)                            # 8-byte Folded Reload
	add 3, 3, 26
	std 3, 576(1)                           # 8-byte Folded Spill
	ld 3, 480(1)                            # 8-byte Folded Reload
	cmpld	11, 3
	beq	0, .Lp9_int8_BB2_40
.Lp9_int8_BB2_195:                              #   Parent Loop BB2_41 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB2_197 Depth 3
                                        #       Child Loop BB2_200 Depth 3
	std 24, 552(1)                          # 8-byte Folded Spill
	std 25, 560(1)                          # 8-byte Folded Spill
	ld 23, 464(1)                           # 8-byte Folded Reload
	ld 20, 528(1)                           # 8-byte Folded Reload
	ld 19, 424(1)                           # 8-byte Folded Reload
	li 7, 0
	std 11, 568(1)                          # 8-byte Folded Spill
	lwz 3, 416(1)
	mtocrf 128, 3
	bc 12, 0, .Lp9_int8_BB2_198
# %bb.196:                              #   in Loop: Header=BB2_195 Depth=2
	ld 7, 360(1)                            # 8-byte Folded Reload
	ld 6, 576(1)                            # 8-byte Folded Reload
	ld 17, 584(1)                           # 8-byte Folded Reload
	ld 31, 344(1)                           # 8-byte Folded Reload
	li 5, 0
	ld 30, 312(1)                           # 8-byte Folded Reload
	ld 21, 640(1)                           # 8-byte Folded Reload
	ld 26, 648(1)                           # 8-byte Folded Reload
	ld 28, 328(1)                           # 8-byte Folded Reload
	std 7, 888(1)                           # 8-byte Folded Spill
	ld 7, 352(1)                            # 8-byte Folded Reload
	std 7, 880(1)                           # 8-byte Folded Spill
	ld 7, 768(1)                            # 8-byte Folded Reload
	std 7, 872(1)                           # 8-byte Folded Spill
	ld 7, 592(1)                            # 8-byte Folded Reload
	std 7, 864(1)                           # 8-byte Folded Spill
	ld 7, 600(1)                            # 8-byte Folded Reload
	std 7, 856(1)                           # 8-byte Folded Spill
	ld 7, 336(1)                            # 8-byte Folded Reload
	std 7, 848(1)                           # 8-byte Folded Spill
	ld 7, 320(1)                            # 8-byte Folded Reload
	std 7, 840(1)                           # 8-byte Folded Spill
	ld 7, 608(1)                            # 8-byte Folded Reload
	std 7, 832(1)                           # 8-byte Folded Spill
	ld 7, 616(1)                            # 8-byte Folded Reload
	std 7, 824(1)                           # 8-byte Folded Spill
	li 7, 0
	.p2align	5
.Lp9_int8_BB2_197:                              #   Parent Loop BB2_41 Depth=1
                                        #     Parent Loop BB2_195 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	std 31, 784(1)                          # 8-byte Folded Spill
	ld 31, 768(1)                           # 8-byte Folded Reload
	std 5, 816(1)                           # 8-byte Folded Spill
	add 3, 23, 6
	add 4, 20, 5
	mr	5, 31
	std 7, 792(1)                           # 8-byte Folded Spill
	std 17, 800(1)                          # 8-byte Folded Spill
	std 6, 808(1)                           # 8-byte Folded Spill
	bl memcpy
	nop
	add 3, 23, 17
	ld 14, 888(1)                           # 8-byte Folded Reload
	ld 17, 776(1)                           # 8-byte Folded Reload
	add 4, 20, 14
	mr	5, 17
	bl memcpy
	nop
	ld 18, 864(1)                           # 8-byte Folded Reload
	ld 15, 872(1)                           # 8-byte Folded Reload
	mr	5, 31
	add 3, 23, 18
	add 4, 20, 15
	bl memcpy
	nop
	ld 27, 856(1)                           # 8-byte Folded Reload
	ld 22, 880(1)                           # 8-byte Folded Reload
	mr	5, 17
	add 3, 23, 27
	add 4, 20, 22
	bl memcpy
	nop
	ld 16, 832(1)                           # 8-byte Folded Reload
	ld 29, 840(1)                           # 8-byte Folded Reload
	mr	5, 31
	add 3, 23, 16
	add 4, 20, 29
	bl memcpy
	nop
	ld 25, 824(1)                           # 8-byte Folded Reload
	ld 24, 848(1)                           # 8-byte Folded Reload
	mr	5, 17
	add 3, 23, 25
	add 4, 20, 24
	bl memcpy
	nop
	mr	5, 31
	ld 31, 784(1)                           # 8-byte Folded Reload
	add 3, 23, 21
	add 4, 20, 30
	bl memcpy
	nop
	mr	5, 17
	ld 17, 800(1)                           # 8-byte Folded Reload
	add 3, 23, 26
	add 4, 20, 31
	bl memcpy
	nop
	ld 7, 792(1)                            # 8-byte Folded Reload
	ld 6, 808(1)                            # 8-byte Folded Reload
	ld 5, 816(1)                            # 8-byte Folded Reload
	ld 4, 752(1)                            # 8-byte Folded Reload
	addi 28, 28, -1
	add 26, 26, 19
	add 21, 21, 19
	ld 3, 760(1)                            # 8-byte Folded Reload
	add 25, 25, 19
	add 16, 16, 19
	add 27, 27, 19
	add 18, 18, 19
	add 17, 17, 19
	cmpldi	28, 0
	std 25, 824(1)                          # 8-byte Folded Spill
	std 16, 832(1)                          # 8-byte Folded Spill
	addi 7, 7, 4
	add 30, 30, 3
	add 31, 31, 4
	add 29, 29, 3
	add 24, 24, 4
	add 15, 15, 3
	add 22, 22, 4
	add 14, 14, 4
	add 5, 5, 3
	add 6, 6, 19
	std 27, 856(1)                          # 8-byte Folded Spill
	std 18, 864(1)                          # 8-byte Folded Spill
	std 29, 840(1)                          # 8-byte Folded Spill
	std 24, 848(1)                          # 8-byte Folded Spill
	std 15, 872(1)                          # 8-byte Folded Spill
	std 22, 880(1)                          # 8-byte Folded Spill
	std 14, 888(1)                          # 8-byte Folded Spill
	bc 12, 1, .Lp9_int8_BB2_197
.Lp9_int8_BB2_198:                              #   in Loop: Header=BB2_195 Depth=2
	ld 28, 776(1)                           # 8-byte Folded Reload
	ld 27, 472(1)                           # 8-byte Folded Reload
	ld 26, 768(1)                           # 8-byte Folded Reload
	lwz 3, 420(1)
	mtocrf 128, 3
	ld 25, 560(1)                           # 8-byte Folded Reload
	ld 24, 552(1)                           # 8-byte Folded Reload
	bc 12, 2, .Lp9_int8_BB2_194
# %bb.199:                              #   in Loop: Header=BB2_195 Depth=2
	ld 3, 632(1)                            # 8-byte Folded Reload
	ld 29, 368(1)                           # 8-byte Folded Reload
	maddld 20, 28, 7, 3
	ld 3, 528(1)                            # 8-byte Folded Reload
	maddld 19, 26, 7, 3
	ld 3, 376(1)                            # 8-byte Folded Reload
	add 3, 3, 7
	mulld 30, 27, 3
	.p2align	5
.Lp9_int8_BB2_200:                              #   Parent Loop BB2_41 Depth=1
                                        #     Parent Loop BB2_195 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	add 3, 24, 30
	mr	4, 19
	mr	5, 26
	bl memcpy
	nop
	add 3, 25, 30
	mr	4, 20
	mr	5, 28
	bl memcpy
	nop
	addi 29, 29, -1
	add 20, 20, 28
	add 19, 19, 26
	add 30, 30, 27
	cmpldi	29, 0
	bc 12, 1, .Lp9_int8_BB2_200
	b .Lp9_int8_BB2_194
.Lp9_int8_BB2_201:
	ld 3, 728(1)                            # 8-byte Folded Reload
	bl free
	nop
	mr	3, 30
	bl free
	nop
	ld 3, 672(1)                            # 8-byte Folded Reload
	bl free
	nop
	mr	3, 25
	bl free
	nop
	li 28, 5
	b .Lp9_int8_BB2_24
.Lp9_int8_BB2_202:
	li 28, 0
.Lp9_int8_BB2_203:
	ld 25, 528(1)                           # 8-byte Folded Reload
.Lp9_int8_BB2_204:
	ld 3, 728(1)                            # 8-byte Folded Reload
	bl free
	nop
	ld 3, 112(1)                            # 8-byte Folded Reload
	bl free
	nop
	ld 3, 672(1)                            # 8-byte Folded Reload
	bl free
	nop
	mr	3, 25
	bl free
	nop
	b .Lp9_int8_BB2_24
	.long	0
	.quad	0
.Lp9_int8_func_end2:
	.size	pq8_pack_power9, .Lp9_int8_func_end2-.Lp9_int8_func_begin2
	.cfi_endproc
                                        # -- End function
