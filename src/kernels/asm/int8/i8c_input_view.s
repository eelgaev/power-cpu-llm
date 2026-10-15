# int8/i8c_input_view.s: static helper.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_int8.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.text.i8c_input_view,"ax",@progbits
	.p2align	4                               # -- Begin function p9_int8_i8c_input_view
	.type	p9_int8_i8c_input_view,@function
p9_int8_i8c_input_view:                         # @p9_int8_i8c_input_view
.Lp9_int8_func_begin5:
	.cfi_startproc
.Lp9_int8_func_gep5:
	addis 2, 12, .TOC.-.Lp9_int8_func_gep5@ha
	addi 2, 2, .TOC.-.Lp9_int8_func_gep5@l
.Lp9_int8_func_lep5:
	.localentry	p9_int8_i8c_input_view, .Lp9_int8_func_lep5-.Lp9_int8_func_gep5
# %bb.0:
	mflr 0
	.cfi_def_cfa_offset 96
	.cfi_offset lr, 16
	.cfi_offset r25, -56
	.cfi_offset r26, -48
	.cfi_offset r27, -40
	.cfi_offset r28, -32
	.cfi_offset r29, -24
	.cfi_offset r30, -16
	std 25, -56(1)                          # 8-byte Folded Spill
	std 26, -48(1)                          # 8-byte Folded Spill
	std 27, -40(1)                          # 8-byte Folded Spill
	std 28, -32(1)                          # 8-byte Folded Spill
	std 29, -24(1)                          # 8-byte Folded Spill
	std 30, -16(1)                          # 8-byte Folded Spill
	stdu 1, -96(1)
	std 0, 112(1)
	mr	30, 8
	mr	25, 7
	mr	29, 6
	mr	27, 5
	mr	28, 4
	mr	26, 3
	bl pq8_input_bytes
	nop
	addi 4, 3, -1
	li 3, 0
	cmpld	4, 25
	bge 0, .Lp9_int8_BB5_5
# %bb.1:
	cmplwi	26, 35
	bgt	0, .Lp9_int8_BB5_3
# %bb.2:
	clrldi	4, 26, 32
	li 5, 1
	li 3, 256
	sld 4, 5, 4
	lis 5, 128
	ori 5, 5, 5
	rldic 5, 5, 12, 28
	and. 4, 4, 5
	bne	0, .Lp9_int8_BB5_4
.Lp9_int8_BB5_3:
	rlwinm 3, 26, 0, 30, 28
	cmpwi 1, 26, 8
	cmpwi	3, 2
	li 3, 32
	crnor 20, 6, 2
	isel 3, 0, 3, 20
.Lp9_int8_BB5_4:
	addi 4, 27, 3
	cnttzd 3, 3
	std 29, 0(30)
	rldicr 4, 4, 0, 61
	srd 3, 28, 3
	mulld 5, 28, 4
	std 4, 32(30)
	std 3, 24(30)
	mulld 3, 4, 3
	sldi 5, 5, 2
	sldi 3, 3, 2
	add 5, 29, 5
	add 3, 5, 3
	std 5, 8(30)
	std 3, 16(30)
	li 3, 1
.Lp9_int8_BB5_5:
	addi 1, 1, 96
	ld 0, 16(1)
	ld 30, -16(1)                           # 8-byte Folded Reload
	ld 29, -24(1)                           # 8-byte Folded Reload
	ld 28, -32(1)                           # 8-byte Folded Reload
	mtlr 0
	ld 27, -40(1)                           # 8-byte Folded Reload
	ld 26, -48(1)                           # 8-byte Folded Reload
	ld 25, -56(1)                           # 8-byte Folded Reload
	blr
	.long	0
	.quad	0
.Lp9_int8_func_end5:
	.size	p9_int8_i8c_input_view, .Lp9_int8_func_end5-.Lp9_int8_func_begin5
	.cfi_endproc
                                        # -- End function
