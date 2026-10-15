# int8/pq8_project_rows_power9.s: kernel.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_int8.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.text.pq8_project_rows_power9,"ax",@progbits
	.globl	pq8_project_rows_power9       # -- Begin function pq8_project_rows_power9
	.p2align 6
	.type	pq8_project_rows_power9,@function
pq8_project_rows_power9:              # @pq8_project_rows_power9
.Lp9_int8_func_begin6:
	.cfi_startproc
.Lp9_int8_func_gep6:
	addis 2, 12, .TOC.-.Lp9_int8_func_gep6@ha
	addi 2, 2, .TOC.-.Lp9_int8_func_gep6@l
.Lp9_int8_func_lep6:
	.localentry	pq8_project_rows_power9, .Lp9_int8_func_lep6-.Lp9_int8_func_gep6
# %bb.0:
	mflr 0
	.cfi_def_cfa_offset 272
	.cfi_offset lr, 16
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
	stdu 1, -272(1)
	mr	25, 3
	addi 3, 1, 144
	ld 17, 384(1)
	ld 18, 376(1)
	ld 19, 368(1)
	std 0, 288(1)
	mr	26, 10
	mr	27, 9
	mr	30, 8
	mr	28, 7
	mr	29, 6
	mr	24, 5
	mr	23, 4
	std 3, 56(1)
	mr	3, 25
	std 17, 48(1)
	std 18, 40(1)
	std 19, 32(1)
	bl p9_int8_i8c_check
	cmplwi	3, 0
	beq	0, .Lp9_int8_BB6_2
.Lp9_int8_BB6_1:
	clrldi	3, 3, 32
	addi 1, 1, 272
	ld 0, 16(1)
	ld 30, -16(1)                           # 8-byte Folded Reload
	ld 29, -24(1)                           # 8-byte Folded Reload
	ld 28, -32(1)                           # 8-byte Folded Reload
	mtlr 0
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
	blr
.Lp9_int8_BB6_2:
	cmplwi 1, 25, 35
	bgt	1, .Lp9_int8_BB6_16
# %bb.3:
	clrldi	3, 25, 32
	li 5, 1
	li 4, 256
	sld 3, 5, 3
	lis 5, 128
	ori 5, 5, 5
	rldic 5, 5, 12, 28
	and. 3, 3, 5
	beq	0, .Lp9_int8_BB6_16
# %bb.4:
	cmpdi	4, 0
	li 3, 5
	bc 12, 2, .Lp9_int8_BB6_1
.Lp9_int8_BB6_5:
	cmpdi	29, 0
	bc 12, 2, .Lp9_int8_BB6_1
# %bb.6:
	cmpdi	28, 0
	bc 12, 2, .Lp9_int8_BB6_1
# %bb.7:
	addi 5, 4, -1
	li 6, -1
	rldimi 5, 6, 61, 0
	and. 5, 5, 28
	bne	0, .Lp9_int8_BB6_1
# %bb.8:
	addi 5, 29, 3
	cnttzd 4, 4
	rldicl 22, 5, 62, 2
	rlwinm 5, 25, 0, 31, 29
	srd 4, 28, 4
	cmpwi 5, 5, 12
	bgt	1, .Lp9_int8_BB6_10
# %bb.9:
	clrldi	6, 25, 32
	li 7, 1
	li 5, 256
	sld 6, 7, 6
	lis 7, 128
	ori 7, 7, 5
	rldic 7, 7, 12, 28
	and. 6, 6, 7
	bne	0, .Lp9_int8_BB6_11
.Lp9_int8_BB6_10:
	rlwinm 5, 25, 0, 30, 28
	cmpwi 1, 25, 8
	cmpwi	5, 2
	li 5, 32
	crnor 20, 6, 2
	isel 5, 0, 5, 20
.Lp9_int8_BB6_11:
	cmpwi	25, 12
	cmpwi 1, 25, 2
	li 6, 1
	li 7, 2
	li 8, 288
	crnor 20, 6, 2
	cmplwi	25, 14
	isel 6, 7, 6, 20
	sld 5, 5, 6
	li 6, 16
	iseleq	6, 8, 6
	cmplwi	25, 12
	li 8, 224
	iseleq	6, 8, 6
	add 5, 5, 6
	li 6, 3
	isel 6, 7, 6, 22
	li 7, 3
	sld 5, 5, 6
	mulld 4, 5, 4
	li 5, 7
	isel 5, 7, 5, 22
	add 5, 22, 5
	srd 5, 5, 6
	mulhdu. 6, 4, 5
	mcrf 1, 0
	mulld. 21, 4, 5
	crorc 20, 2, 6
	bc 12, 20, .Lp9_int8_BB6_1
# %bb.12:
	addi 3, 21, 15
	rldicr 4, 3, 0, 59
	li 3, 16
	bl aligned_alloc
	nop
	mr	20, 3
	li 3, 5
	cmpldi	20, 0
	beq	0, .Lp9_int8_BB6_1
# %bb.13:
	mr	3, 25
	mr	4, 23
	mr	5, 24
	mr	6, 29
	std 21, 96(1)
	mr	7, 28
	li 8, 0
	mr	9, 22
	mr	10, 20
	bl pq8_pack_power9
	nop
	cmplwi	3, 0
	bne	0, .Lp9_int8_BB6_15
# %bb.14:
	mr	3, 25
	mr	4, 20
	mr	5, 21
	mr	6, 29
	std 17, 128(1)
	mr	7, 28
	li 8, 0
	mr	9, 29
	mr	10, 30
	std 18, 120(1)
	std 19, 112(1)
	std 26, 104(1)
	std 27, 96(1)
	bl pq8_project_packed_power9
	nop
.Lp9_int8_BB6_15:
	mr	30, 3
	mr	3, 20
	bl free
	nop
	mr	3, 30
	b .Lp9_int8_BB6_1
.Lp9_int8_BB6_16:
	rlwinm 3, 25, 0, 30, 28
	cmpwi 5, 25, 8
	cmpwi	3, 2
	li 3, 32
	crnor 20, 22, 2
	isel 4, 0, 3, 20
	cmpdi	4, 0
	li 3, 5
	bc 4, 2, .Lp9_int8_BB6_5
	b .Lp9_int8_BB6_1
	.long	0
	.quad	0
.Lp9_int8_func_end6:
	.size	pq8_project_rows_power9, .Lp9_int8_func_end6-.Lp9_int8_func_begin6
	.cfi_endproc
                                        # -- End function
