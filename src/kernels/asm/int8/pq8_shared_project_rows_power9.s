# int8/pq8_shared_project_rows_power9.s: kernel.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_int8.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.text.pq8_shared_project_rows_power9,"ax",@progbits
	.globl	pq8_shared_project_rows_power9 # -- Begin function pq8_shared_project_rows_power9
	.p2align 6
	.type	pq8_shared_project_rows_power9,@function
pq8_shared_project_rows_power9:       # @pq8_shared_project_rows_power9
.Lp9_int8_func_begin13:
	.cfi_startproc
.Lp9_int8_func_gep13:
	addis 2, 12, .TOC.-.Lp9_int8_func_gep13@ha
	addi 2, 2, .TOC.-.Lp9_int8_func_gep13@l
.Lp9_int8_func_lep13:
	.localentry	pq8_shared_project_rows_power9, .Lp9_int8_func_lep13-.Lp9_int8_func_gep13
# %bb.0:
	cmpwi 1, 3, 2
	andi. 3, 10, 1
	li 3, 1
	crorc 20, 1, 6
	bc 12, 20, .Lp9_int8_BB13_10
# %bb.1:
	mflr 0
	.cfi_def_cfa_offset 256
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
	stdu 1, -256(1)
	addi 3, 1, 136
	ld 21, 368(1)
	ld 22, 360(1)
	ld 23, 352(1)
	std 0, 272(1)
	mr	29, 7
	mr	30, 6
	mr	26, 4
	mr	27, 5
	mr	28, 8
	mr	20, 9
	mr	19, 10
	std 3, 56(1)
	li 3, 2
	std 21, 48(1)
	std 22, 40(1)
	std 23, 32(1)
	bl p9_int8_i8c_check
	cmplwi	3, 0
	bne	0, .Lp9_int8_BB13_9
# %bb.2:
	cmpdi	30, 0
	cmpdi 1, 29, 0
	rldicl 3, 29, 3, 56
	li 25, 0
	addi 18, 30, 3
	cror 20, 2, 6
	rotldi.	3, 3, 61
	crorc 20, 20, 2
	bc 12, 20, .Lp9_int8_BB13_4
# %bb.3:
	rldicl 4, 29, 59, 5
	rldicl 3, 18, 62, 2
	addi 3, 3, 7
	mulli 4, 4, 640
	rldicl 3, 3, 61, 3
	mulhdu. 5, 4, 3
	mulld 3, 3, 4
	li 4, 0
	iseleq	25, 3, 4
.Lp9_int8_BB13_4:
	addi 3, 25, 15
	rldicr 4, 3, 0, 59
	li 3, 16
	bl aligned_alloc
	nop
	cmpldi	3, 0
	beq	0, .Lp9_int8_BB13_8
# %bb.5:
	mr	24, 3
	rldicl 9, 18, 62, 2
	li 3, 2
	mr	4, 26
	std 25, 96(1)
	mr	5, 27
	mr	6, 30
	mr	7, 29
	li 8, 0
	mr	10, 24
	bl pq8_pack_power9
	nop
	cmplwi	3, 0
	bne	0, .Lp9_int8_BB13_7
# %bb.6:
	li 3, 2
	mr	4, 24
	mr	5, 25
	mr	6, 30
	std 21, 128(1)
	mr	7, 29
	li 8, 0
	mr	9, 30
	mr	10, 28
	std 22, 120(1)
	std 23, 112(1)
	std 19, 104(1)
	std 20, 96(1)
	bl pq8_shared_project_packed_power9
	nop
.Lp9_int8_BB13_7:
	mr	30, 3
	mr	3, 24
	bl free
	nop
	mr	3, 30
	b .Lp9_int8_BB13_9
.Lp9_int8_BB13_8:
	li 3, 5
.Lp9_int8_BB13_9:
	addi 1, 1, 256
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
.Lp9_int8_BB13_10:
	clrldi	3, 3, 32
	blr
	.long	0
	.quad	0
.Lp9_int8_func_end13:
	.size	pq8_shared_project_rows_power9, .Lp9_int8_func_end13-.Lp9_int8_func_begin13
	.cfi_endproc
                                        # -- End function
