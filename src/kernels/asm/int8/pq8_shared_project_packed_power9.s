# int8/pq8_shared_project_packed_power9.s: kernel.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_int8.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.text.pq8_shared_project_packed_power9,"ax",@progbits
	.globl	pq8_shared_project_packed_power9 # -- Begin function pq8_shared_project_packed_power9
	.p2align 6
	.type	pq8_shared_project_packed_power9,@function
pq8_shared_project_packed_power9:     # @pq8_shared_project_packed_power9
.Lp9_int8_func_begin12:
	.cfi_startproc
.Lp9_int8_func_gep12:
	addis 2, 12, .TOC.-.Lp9_int8_func_gep12@ha
	addi 2, 2, .TOC.-.Lp9_int8_func_gep12@l
.Lp9_int8_func_lep12:
	.localentry	pq8_shared_project_packed_power9, .Lp9_int8_func_lep12-.Lp9_int8_func_gep12
# %bb.0:
	mfcr 12
	mflr 0
	stw 12, 8(1)
	stdu 1, -816(1)
	std 0, 832(1)
	.cfi_def_cfa_offset 816
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
	std 27, 776(1)                          # 8-byte Folded Spill
	ld 27, 920(1)
	std 24, 752(1)                          # 8-byte Folded Spill
	cmpwi	3, 2
	li 24, 1
	std 14, 672(1)                          # 8-byte Folded Spill
	std 15, 680(1)                          # 8-byte Folded Spill
	std 16, 688(1)                          # 8-byte Folded Spill
	std 17, 696(1)                          # 8-byte Folded Spill
	std 18, 704(1)                          # 8-byte Folded Spill
	std 19, 712(1)                          # 8-byte Folded Spill
	std 20, 720(1)                          # 8-byte Folded Spill
	std 21, 728(1)                          # 8-byte Folded Spill
	std 22, 736(1)                          # 8-byte Folded Spill
	std 23, 744(1)                          # 8-byte Folded Spill
	std 25, 760(1)                          # 8-byte Folded Spill
	std 26, 768(1)                          # 8-byte Folded Spill
	std 28, 784(1)                          # 8-byte Folded Spill
	std 29, 792(1)                          # 8-byte Folded Spill
	std 30, 800(1)                          # 8-byte Folded Spill
	std 31, 808(1)                          # 8-byte Folded Spill
	bc 4, 2, .Lp9_int8_BB12_7
# %bb.1:
	cmpdi	27, 0
	bc 12, 2, .Lp9_int8_BB12_7
# %bb.2:
	ld 29, 928(1)
	cmpdi 1, 4, 0
	andi. 3, 27, 1
	cror 20, 6, 1
	bc 12, 20, .Lp9_int8_BB12_7
# %bb.3:
	mr	30, 10
	cmpdi	10, 0
	bc 12, 2, .Lp9_int8_BB12_8
# %bb.4:
	cmpdi	29, 0
	bc 12, 2, .Lp9_int8_BB12_8
# %bb.5:
	cmpdi	9, 0
	bc 12, 2, .Lp9_int8_BB12_7
# %bb.6:
	or 3, 30, 4
	andi. 3, 3, 15
	beq	0, .Lp9_int8_BB12_9
.Lp9_int8_BB12_7:
	li 24, 1
.Lp9_int8_BB12_8:
	clrldi	3, 24, 32
	ld 31, 808(1)                           # 8-byte Folded Reload
	ld 30, 800(1)                           # 8-byte Folded Reload
	ld 29, 792(1)                           # 8-byte Folded Reload
	ld 28, 784(1)                           # 8-byte Folded Reload
	ld 27, 776(1)                           # 8-byte Folded Reload
	ld 26, 768(1)                           # 8-byte Folded Reload
	ld 25, 760(1)                           # 8-byte Folded Reload
	ld 24, 752(1)                           # 8-byte Folded Reload
	ld 23, 744(1)                           # 8-byte Folded Reload
	ld 22, 736(1)                           # 8-byte Folded Reload
	ld 21, 728(1)                           # 8-byte Folded Reload
	ld 20, 720(1)                           # 8-byte Folded Reload
	ld 19, 712(1)                           # 8-byte Folded Reload
	ld 18, 704(1)                           # 8-byte Folded Reload
	ld 17, 696(1)                           # 8-byte Folded Reload
	ld 16, 688(1)                           # 8-byte Folded Reload
	ld 15, 680(1)                           # 8-byte Folded Reload
	ld 14, 672(1)                           # 8-byte Folded Reload
	addi 1, 1, 816
	ld 0, 16(1)
	lwz 12, 8(1)
	mtlr 0
	mtocrf 32, 12
	mtocrf 16, 12
	mtocrf 8, 12
	blr
.Lp9_int8_BB12_9:
	cmpdi	6, 0
	cmpdi 1, 7, 0
	rldicl 3, 7, 3, 56
	li 24, 3
	cror 20, 2, 6
	rotldi.	3, 3, 61
	crorc 20, 20, 2
	bc 12, 20, .Lp9_int8_BB12_8
# %bb.10:
	addi 3, 6, 3
	rldicl 26, 7, 59, 5
	mr	16, 4
	rldicl 3, 3, 62, 2
	mulli 10, 26, 640
	addi 3, 3, 7
	rldicl 3, 3, 61, 3
	std 10, 160(1)                          # 8-byte Folded Spill
	mulhdu. 4, 10, 3
	mulld 3, 3, 10
	addi 3, 3, -1
	cmpld 1, 3, 5
	crnand 20, 4, 2
	bc 12, 20, .Lp9_int8_BB12_8
# %bb.11:
	cmpld	6, 8
	sub	3, 6, 8
	bc 12, 0, .Lp9_int8_BB12_7
# %bb.12:
	cmpld	9, 3
	bc 12, 1, .Lp9_int8_BB12_7
# %bb.13:
	ld 25, 944(1)
	not	3, 9
	li 24, 4
	cmpld	25, 3
	bgt	0, .Lp9_int8_BB12_8
# %bb.14:
	ld 28, 936(1)
	add 3, 25, 9
	li 24, 1
	cmpld	3, 28
	bgt	0, .Lp9_int8_BB12_8
# %bb.15:
	li 24, 4
	mulhdu. 3, 28, 27
	bne	0, .Lp9_int8_BB12_8
# %bb.16:
	ld 22, 912(1)
	li 3, 2
	mr	4, 7
	mr	5, 27
	mr	24, 7
	mr	23, 9
	mr	17, 8
	bl pq8_input_bytes
	nop
	addi 3, 3, -1
	mr	4, 24
	li 24, 5
	cmpld	3, 22
	bge 0, .Lp9_int8_BB12_8
# %bb.17:
	add 3, 23, 17
	rldicl 7, 17, 62, 2
	li 24, 0
	std 3, 128(1)                           # 8-byte Folded Spill
	addi 3, 3, -1
	std 7, 56(1)                            # 8-byte Folded Spill
	rldicl 12, 3, 62, 2
	cmpld	7, 12
	bgt	0, .Lp9_int8_BB12_8
# %bb.18:
	rldicl 20, 27, 63, 1
	ld 7, 56(1)                             # 8-byte Folded Reload
	sldi 8, 25, 2
	addi 6, 27, 3
	li 24, 0
	li 22, 7
	li 14, 0
	addi 3, 20, 1
	sldi 10, 4, 5
	std 10, 72(1)                           # 8-byte Folded Spill
	sldi 10, 4, 3
	mulld 5, 28, 20
	sldi 27, 4, 2
	rldicl 18, 4, 62, 2
	sldi 9, 28, 2
	rldicr 6, 6, 0, 61
	sldi 11, 28, 5
	std 11, 64(1)                           # 8-byte Folded Spill
	sldi 11, 28, 3
	std 30, 152(1)                          # 8-byte Folded Spill
	mulld 3, 28, 3
	li 28, 0
	sldi 7, 7, 4
	add 7, 7, 8
	std 10, 264(1)                          # 8-byte Folded Spill
	sldi 10, 17, 2
	rldicl 8, 4, 61, 3
	mulld 4, 4, 20
	maddld 19, 27, 6, 30
	std 11, 256(1)                          # 8-byte Folded Spill
	sldi 5, 5, 2
	cmpldi 2, 5, 16
	sldi 3, 3, 2
	add 3, 7, 3
	sub	3, 3, 10
	sldi 4, 4, 2
	add 3, 3, 29
	std 4, 248(1)                           # 8-byte Folded Spill
	add 4, 7, 5
	sub	4, 4, 10
	crmove	0, 8
	mfocrf 5, 128
	addi 3, 3, 60
	add 4, 4, 29
	maddld 25, 8, 6, 19
	mr	8, 17
	std 3, 192(1)                           # 8-byte Folded Spill
	add 3, 7, 9
	addi 23, 4, 60
	li 4, 0
	sub	3, 3, 10
	add 3, 3, 29
	addi 3, 3, 60
	std 3, 184(1)                           # 8-byte Folded Spill
	sub	3, 7, 10
	stw 5, 204(1)
	std 17, 120(1)                          # 8-byte Folded Spill
	std 16, 112(1)                          # 8-byte Folded Spill
	std 12, 104(1)                          # 8-byte Folded Spill
	add 21, 29, 3
	li 3, 0
	b .Lp9_int8_BB12_20
.Lp9_int8_BB12_19:                              #   in Loop: Header=BB12_20 Depth=1
	ld 3, 72(1)                             # 8-byte Folded Reload
	ld 4, 152(1)                            # 8-byte Folded Reload
	ld 14, 88(1)                            # 8-byte Folded Reload
	add 4, 4, 3
	ld 3, 64(1)                             # 8-byte Folded Reload
	std 4, 152(1)                           # 8-byte Folded Spill
	ld 4, 96(1)                             # 8-byte Folded Reload
	add 14, 14, 3
	ld 3, 80(1)                             # 8-byte Folded Reload
	addi 4, 4, 1
	cmpld	3, 20
	bge 0, .Lp9_int8_BB12_8
.Lp9_int8_BB12_20:                              # =>This Loop Header: Depth=1
                                        #     Child Loop BB12_23 Depth 2
                                        #       Child Loop BB12_26 Depth 3
	mr	7, 3
	sub	3, 20, 3
	std 4, 96(1)                            # 8-byte Folded Spill
	std 14, 88(1)                           # 8-byte Folded Spill
	addi 6, 7, 8
	cmpldi	3, 8
	std 7, 144(1)                           # 8-byte Folded Spill
	isellt	3, 20, 6
	std 6, 80(1)                            # 8-byte Folded Spill
	cmpld	7, 3
	bge 0, .Lp9_int8_BB12_19
# %bb.21:                               #   in Loop: Header=BB12_20 Depth=1
	sldi 3, 4, 3
	li 4, 8
	ld 29, 56(1)                            # 8-byte Folded Reload
	sub	3, 20, 3
	cmpldi	3, 8
	isellt	3, 3, 4
	addi 3, 3, -1
	rldicl 3, 3, 63, 1
	addi 3, 3, 1
	std 3, 136(1)                           # 8-byte Folded Spill
	b .Lp9_int8_BB12_23
.Lp9_int8_BB12_22:                              #   in Loop: Header=BB12_23 Depth=2
	ld 29, 168(1)                           # 8-byte Folded Reload
	ld 14, 176(1)                           # 8-byte Folded Reload
	ld 12, 104(1)                           # 8-byte Folded Reload
	ld 8, 120(1)                            # 8-byte Folded Reload
	addi 29, 29, 4
	ld 16, 112(1)                           # 8-byte Folded Reload
	addi 14, 14, 64
	cmpld	29, 12
	bgt	0, .Lp9_int8_BB12_19
.Lp9_int8_BB12_23:                              #   Parent Loop BB12_20 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB12_26 Depth 3
	ld 9, 128(1)                            # 8-byte Folded Reload
	sldi 3, 29, 2
	rlwinm 10, 29, 6, 23, 25
	ori 4, 3, 1
	cmpld	3, 8
	ori 5, 3, 2
	ori 6, 3, 3
	addi 7, 3, 4
	cmpld 1, 3, 9
	crandc 12, 4, 0
	cmpld	4, 8
	cmpld 1, 4, 9
	addi 4, 3, 5
	crandc 13, 4, 0
	cmpld	5, 8
	cmpld 1, 5, 9
	addi 5, 3, 6
	crandc 15, 4, 0
	cmpld	6, 8
	cmpld 1, 6, 9
	crandc 20, 4, 0
	cmpld	7, 8
	cmpld 1, 7, 9
	setb 6, 5
	crandc 20, 4, 0
	cmpld	4, 8
	cmpld 1, 4, 9
	setb 7, 5
	crandc 20, 4, 0
	cmpld	5, 8
	stw 6, 364(1)
	addi 6, 3, 7
	setb 4, 5
	cmpld 5, 5, 9
	addi 5, 3, 10
	stw 7, 360(1)
	addi 7, 3, 8
	cmpld 1, 6, 8
	crandc 20, 20, 0
	cmpld	6, 9
	addi 6, 3, 11
	stw 4, 356(1)
	setb 4, 5
	crandc 14, 0, 4
	cmpld	7, 8
	cmpld 1, 7, 9
	crandc 20, 4, 0
	cmpld 1, 5, 8
	stw 4, 352(1)
	addi 4, 3, 9
	setb 7, 5
	cmpld	4, 8
	cmpld 5, 4, 9
	crandc 20, 20, 0
	cmpld	6, 8
	stw 7, 348(1)
	setb 4, 5
	cmpld 5, 5, 9
	addi 5, 3, 13
	crandc 20, 20, 4
	cmpld 1, 6, 9
	addi 6, 3, 14
	stw 4, 344(1)
	setb 4, 5
	crandc 20, 4, 0
	stw 4, 340(1)
	setb 4, 5
	stw 4, 336(1)
	addi 4, 3, 12
	addi 3, 3, 15
	cmpld	4, 8
	cmpld 1, 4, 9
	crandc 20, 4, 0
	cmpld	5, 8
	cmpld 1, 5, 9
	rldicl 5, 29, 61, 3
	setb 4, 5
	crandc 20, 4, 0
	cmpld	6, 8
	cmpld 1, 6, 9
	subc	6, 29, 12
	subfe 6, 29, 29
	stw 4, 332(1)
	setb 4, 5
	crandc 20, 4, 0
	cmpld	3, 8
	cmpld 1, 3, 9
	neg 6, 6
	add 6, 29, 6
	stw 4, 328(1)
	setb 4, 5
	crandc 20, 4, 0
	rldicl 7, 6, 61, 3
	setb 3, 5
	stw 4, 324(1)
	addi 4, 29, 3
	stw 3, 320(1)
	addi 3, 29, 2
	std 29, 168(1)                          # 8-byte Folded Spill
	std 14, 176(1)                          # 8-byte Folded Spill
	ld 11, 160(1)                           # 8-byte Folded Reload
	ld 31, 152(1)                           # 8-byte Folded Reload
	ld 17, 144(1)                           # 8-byte Folded Reload
	ld 15, 136(1)                           # 8-byte Folded Reload
	cmpld	3, 12
	iselgt	3, 29, 3
	cmpld	4, 12
	maddld 5, 11, 5, 16
	maddld 7, 11, 7, 16
	rldicl 8, 3, 61, 3
	iselgt	4, 29, 4
	maddld 8, 11, 8, 16
	rldicl 9, 4, 61, 3
	add 5, 5, 10
	std 5, 312(1)                           # 8-byte Folded Spill
	rlwinm 5, 29, 4, 25, 27
	ori 5, 5, 512
	sub	5, 5, 10
	std 5, 304(1)                           # 8-byte Folded Spill
	rlwinm 5, 6, 6, 23, 25
	rlwinm 6, 6, 4, 25, 27
	ori 6, 6, 512
	add 7, 7, 5
	sub	5, 6, 5
	std 7, 296(1)                           # 8-byte Folded Spill
	std 5, 288(1)                           # 8-byte Folded Spill
	rlwinm 5, 3, 6, 23, 25
	rlwinm 3, 3, 4, 25, 27
	ori 3, 3, 512
	add 6, 8, 5
	sub	3, 3, 5
	rlwinm 5, 4, 6, 23, 25
	std 6, 280(1)                           # 8-byte Folded Spill
	std 3, 272(1)                           # 8-byte Folded Spill
	maddld 3, 11, 9, 16
	add 29, 3, 5
	rlwinm 3, 4, 4, 25, 27
	ori 3, 3, 512
	sub	16, 3, 5
	setb 3, 3
	stw 3, 220(1)
	mfocrf 3, 16
	rlwinm 3, 3, 13, 0, 0
	stw 3, 216(1)
	mfocrf 3, 16
	rlwinm 3, 3, 15, 0, 0
	stw 3, 212(1)
	mfocrf 3, 16
	rlwinm 3, 3, 14, 0, 0
	stw 3, 208(1)
	b .Lp9_int8_BB12_26
.Lp9_int8_BB12_24:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 3, 244(1)
                                        # implicit-def: $cr6lt
	rotlwi	3, 3, 4
	mtocrf 1, 3
	lwz 3, 240(1)
	crand 20, 12, 30
	rotlwi	3, 3, 4
	mtocrf 1, 3
	lwz 3, 236(1)
	lwz 4, 364(1)
	crand 21, 13, 30
	rotlwi	3, 3, 4
	mtocrf 1, 3
	mfocrf 3, 2
	rlwimi 3, 4, 8, 24, 24
	crand 23, 15, 30
	mtocrf 2, 3
	lwz 3, 232(1)
	lwz 4, 360(1)
                                        # implicit-def: $cr6gt
	rotlwi	3, 3, 4
	mtocrf 1, 3
	crand 24, 24, 30
	mfocrf 3, 2
	rlwimi 3, 4, 7, 25, 25
	lwz 4, 356(1)
	mtocrf 2, 3
	crand 25, 25, 18
	cror 20, 20, 25
                                        # implicit-def: $cr6gt
	mfocrf 3, 2
	rlwimi 3, 4, 7, 25, 25
	lwz 4, 352(1)
	mtocrf 2, 3
	crand 25, 25, 10
	cror 21, 21, 25
                                        # implicit-def: $cr6gt
	mfocrf 3, 2
	rlwimi 3, 4, 7, 25, 25
	mtocrf 2, 3
	lwz 3, 228(1)
	lwz 4, 208(1)
                                        # implicit-def: $cr6un
	rotlwi	3, 3, 4
	mtocrf 1, 3
	crand 25, 25, 30
	mfocrf 3, 2
	cror 23, 23, 25
	rlwimi 3, 4, 5, 27, 27
	lwz 4, 348(1)
	mtocrf 2, 3
	crand 25, 27, 14
	crmove	14, 27
	cror 24, 24, 25
                                        # implicit-def: $cr6gt
	mfocrf 3, 2
	rlwimi 3, 4, 7, 25, 25
	lwz 4, 344(1)
	mtocrf 2, 3
	crand 25, 25, 26
	cror 20, 20, 25
                                        # implicit-def: $cr6gt
	mfocrf 3, 2
	rlwimi 3, 4, 7, 25, 25
	lwz 4, 340(1)
	mtocrf 2, 3
	crand 25, 25, 6
	cror 21, 21, 25
                                        # implicit-def: $cr6gt
	mfocrf 3, 2
	rlwimi 3, 4, 7, 25, 25
	lwz 4, 336(1)
	mtocrf 2, 3
	crand 25, 25, 2
	xststdcsp 0, 0, 112
	xststdcsp 1, 1, 112
	cror 23, 23, 25
                                        # implicit-def: $cr6gt
	mfocrf 3, 2
	rlwimi 3, 4, 7, 25, 25
	mtocrf 2, 3
	crand 22, 25, 22
	cror 22, 24, 22
	xststdcsp 6, 2, 112
	xststdcsp 7, 3, 112
	lwz 4, 332(1)
                                        # implicit-def: $cr6lt
	mfocrf 3, 2
	rlwimi 3, 4, 8, 24, 24
	lwz 4, 328(1)
	mtocrf 2, 3
	crand 24, 24, 2
	cror 20, 20, 24
                                        # implicit-def: $cr6lt
	mfocrf 3, 2
	rlwimi 3, 4, 8, 24, 24
	lwz 4, 324(1)
	mtocrf 2, 3
	crand 24, 24, 6
	cror 21, 21, 24
                                        # implicit-def: $cr6lt
	mfocrf 3, 2
	rlwimi 3, 4, 8, 24, 24
	lwz 4, 320(1)
	cror 20, 21, 20
	mtocrf 2, 3
	crand 24, 24, 26
	cror 23, 23, 24
                                        # implicit-def: $cr6lt
	mfocrf 3, 2
	rlwimi 3, 4, 8, 24, 24
	cror 20, 23, 20
	mtocrf 2, 3
	lwz 3, 204(1)
	crand 24, 24, 30
	cror 22, 22, 24
	rotlwi	3, 3, 24
	cror 20, 22, 20
	mtocrf 32, 3
	isel 24, 22, 24, 20
	.p2align	5
.Lp9_int8_BB12_25:                              #   in Loop: Header=BB12_26 Depth=3
	ld 3, 264(1)                            # 8-byte Folded Reload
	addi 15, 15, -1
	addi 17, 17, 2
	cmpldi	15, 0
	add 31, 31, 3
	ld 3, 256(1)                            # 8-byte Folded Reload
	add 14, 14, 3
	bc 4, 1, .Lp9_int8_BB12_22
.Lp9_int8_BB12_26:                              #   Parent Loop BB12_20 Depth=1
                                        #     Parent Loop BB12_23 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	ld 5, 248(1)                            # 8-byte Folded Reload
	rldicl 6, 17, 62, 2
	addi 3, 17, 1
	add 4, 20, 17
	cmpld 4, 3, 20
	rlwinm 7, 17, 1, 29, 29
	std 29, 528(1)
	std 16, 560(1)
	std 31, 568(1)
	add 5, 31, 5
	isel 3, 3, 17, 16
	std 26, 648(1)
	std 5, 584(1)
	maddld 5, 6, 18, 25
	rldicl 6, 4, 62, 2
	mulld 6, 6, 26
	add 5, 5, 7
	rlwinm 7, 4, 1, 29, 30
	rlwinm 4, 4, 2, 28, 29
	std 5, 600(1)
	maddld 5, 3, 27, 30
	std 5, 576(1)
	sldi 5, 6, 3
	sldi 6, 6, 4
	add 5, 25, 5
	add 6, 19, 6
	add 5, 5, 7
	add 7, 3, 20
	add 4, 6, 4
	maddld 6, 7, 27, 30
	std 5, 616(1)
	rldicl 5, 7, 62, 2
	std 4, 632(1)
	mulld 5, 5, 26
	std 6, 592(1)
	rldicl 6, 3, 62, 2
	rlwinm 3, 3, 1, 29, 30
	maddld 6, 6, 18, 25
	sldi 4, 5, 3
	sldi 5, 5, 4
	add 4, 25, 4
	add 3, 6, 3
	ld 6, 312(1)                            # 8-byte Folded Reload
	std 3, 608(1)
	rlwinm 3, 7, 1, 29, 30
	std 6, 504(1)
	ld 6, 304(1)                            # 8-byte Folded Reload
	add 3, 4, 3
	rlwinm 4, 7, 2, 28, 29
	std 3, 624(1)
	add 3, 19, 5
	std 6, 536(1)
	ld 6, 296(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	std 3, 640(1)
	li 3, 640
	std 6, 512(1)
	ld 6, 288(1)                            # 8-byte Folded Reload
	std 3, 656(1)
	addi 3, 1, 368
	std 3, 664(1)
	addi 3, 1, 504
	std 6, 544(1)
	ld 6, 280(1)                            # 8-byte Folded Reload
	std 6, 520(1)
	ld 6, 272(1)                            # 8-byte Folded Reload
	std 6, 552(1)
	bl pq8_q4_0_shared_tile4x2_power9
	nop
	cmpld	17, 20
	bge 0, .Lp9_int8_BB12_25
# %bb.27:                               #   in Loop: Header=BB12_26 Depth=3
	bc 4, 8, .Lp9_int8_BB12_60
# %bb.28:                               #   in Loop: Header=BB12_26 Depth=3
	bc 4, 12, .Lp9_int8_BB12_30
# %bb.29:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 368(1)
	add 3, 23, 14
	stfsx 0, 21, 14
	stw 28, -60(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_30:                              #   in Loop: Header=BB12_26 Depth=3
	bc 4, 13, .Lp9_int8_BB12_32
# %bb.31:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 372(1)
	add 3, 21, 14
	stfs 0, 4(3)
	add 3, 23, 14
	stw 28, -56(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_32:                              #   in Loop: Header=BB12_26 Depth=3
	bc 4, 15, .Lp9_int8_BB12_34
# %bb.33:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 376(1)
	add 3, 21, 14
	stfs 0, 8(3)
	add 3, 23, 14
	stw 28, -52(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_34:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 364(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_36
# %bb.35:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 380(1)
	add 3, 21, 14
	stfs 0, 12(3)
	add 3, 23, 14
	stw 28, -48(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_36:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 360(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_38
# %bb.37:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 400(1)
	add 3, 21, 14
	stfs 0, 16(3)
	add 3, 23, 14
	stw 28, -44(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_38:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 356(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_40
# %bb.39:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 404(1)
	add 3, 21, 14
	stfs 0, 20(3)
	add 3, 23, 14
	stw 28, -40(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_40:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 352(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_42
# %bb.41:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 408(1)
	add 3, 21, 14
	stfs 0, 24(3)
	add 3, 23, 14
	stw 28, -36(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_42:                              #   in Loop: Header=BB12_26 Depth=3
	bc 4, 14, .Lp9_int8_BB12_44
# %bb.43:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 412(1)
	add 3, 21, 14
	stfs 0, 28(3)
	add 3, 23, 14
	stw 28, -32(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_44:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 348(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_46
# %bb.45:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 432(1)
	add 3, 21, 14
	stfs 0, 32(3)
	add 3, 23, 14
	stw 28, -28(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_46:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 344(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_48
# %bb.47:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 436(1)
	add 3, 21, 14
	stfs 0, 36(3)
	add 3, 23, 14
	stw 28, -24(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_48:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 340(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_50
# %bb.49:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 440(1)
	add 3, 21, 14
	stfs 0, 40(3)
	add 3, 23, 14
	stw 28, -20(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_50:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 336(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_52
# %bb.51:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 444(1)
	add 3, 21, 14
	stfs 0, 44(3)
	add 3, 23, 14
	stw 28, -16(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_52:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 332(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_54
# %bb.53:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 464(1)
	add 3, 21, 14
	stfs 0, 48(3)
	add 3, 23, 14
	stw 28, -12(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_54:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 328(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_56
# %bb.55:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 468(1)
	add 3, 21, 14
	stfs 0, 52(3)
	add 3, 23, 14
	stw 28, -8(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_56:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 324(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_58
# %bb.57:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 472(1)
	add 3, 21, 14
	stfs 0, 56(3)
	add 3, 23, 14
	stw 28, -4(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_58:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 320(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_91
# %bb.59:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 476(1)
	add 3, 21, 14
	stfs 0, 60(3)
	stwx 28, 23, 14
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
	b .Lp9_int8_BB12_91
	.p2align	5
.Lp9_int8_BB12_60:                              #   in Loop: Header=BB12_26 Depth=3
	add 3, 23, 14
                                        # implicit-def: $f0
	bc 12, 12, .Lp9_int8_BB12_123
# %bb.61:                               #   in Loop: Header=BB12_26 Depth=3
	add 4, 21, 14
                                        # implicit-def: $f1
	bc 12, 13, .Lp9_int8_BB12_124
.Lp9_int8_BB12_62:                              #   in Loop: Header=BB12_26 Depth=3
                                        # implicit-def: $f2
	bc 4, 15, .Lp9_int8_BB12_64
.Lp9_int8_BB12_63:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 2, 376(1)
	stfs 2, 8(4)
	stw 28, -52(3)
.Lp9_int8_BB12_64:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 6, 364(1)
                                        # implicit-def: $cr5lt
	mfocrf 5, 4
                                        # implicit-def: $f3
	rlwimi 5, 6, 12, 20, 20
	mtocrf 4, 5
	bc 4, 20, .Lp9_int8_BB12_66
# %bb.65:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 3, 380(1)
	stfs 3, 12(4)
	stw 28, -48(3)
.Lp9_int8_BB12_66:                              #   in Loop: Header=BB12_26 Depth=3
	xststdcsp 0, 0, 112
                                        # implicit-def: $cr5lt
                                        # implicit-def: $f0
	mfocrf 5, 128
	stw 5, 244(1)
	xststdcsp 0, 1, 112
	mfocrf 5, 128
	stw 5, 240(1)
	xststdcsp 0, 2, 112
	mfocrf 5, 128
	stw 5, 236(1)
	xststdcsp 0, 3, 112
	mfocrf 5, 128
	stw 5, 232(1)
	mfocrf 5, 4
	lwz 6, 360(1)
	rlwimi 5, 6, 12, 20, 20
	mtocrf 4, 5
	bc 4, 20, .Lp9_int8_BB12_68
# %bb.67:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 400(1)
	stfs 0, 16(4)
	stw 28, -44(3)
.Lp9_int8_BB12_68:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 6, 356(1)
                                        # implicit-def: $cr5lt
	mfocrf 5, 4
                                        # implicit-def: $f1
	rlwimi 5, 6, 12, 20, 20
	mtocrf 4, 5
	bc 4, 20, .Lp9_int8_BB12_70
# %bb.69:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 1, 404(1)
	stfs 1, 20(4)
	stw 28, -40(3)
.Lp9_int8_BB12_70:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 6, 352(1)
                                        # implicit-def: $cr5lt
	mfocrf 5, 4
                                        # implicit-def: $f2
	rlwimi 5, 6, 12, 20, 20
	mtocrf 4, 5
	bc 4, 20, .Lp9_int8_BB12_72
# %bb.71:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 2, 408(1)
	stfs 2, 24(4)
	stw 28, -36(3)
.Lp9_int8_BB12_72:                              #   in Loop: Header=BB12_26 Depth=3
                                        # implicit-def: $f3
	bc 4, 14, .Lp9_int8_BB12_74
# %bb.73:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 3, 412(1)
	stfs 3, 28(4)
	stw 28, -32(3)
.Lp9_int8_BB12_74:                              #   in Loop: Header=BB12_26 Depth=3
	xststdcsp 0, 0, 112
	xststdcsp 1, 1, 112
                                        # implicit-def: $cr5lt
                                        # implicit-def: $f0
	mfocrf 5, 64
	rotlwi	5, 5, 4
	stw 5, 224(1)
	xststdcsp 1, 2, 112
	mfocrf 5, 64
	rotlwi	5, 5, 4
	stw 5, 228(1)
	xststdcsp 3, 3, 112
	lwz 6, 348(1)
	mfocrf 5, 4
	rlwimi 5, 6, 12, 20, 20
	mtocrf 4, 5
	bc 4, 20, .Lp9_int8_BB12_76
# %bb.75:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 432(1)
	stfs 0, 32(4)
	stw 28, -28(3)
.Lp9_int8_BB12_76:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 6, 220(1)
                                        # implicit-def: $cr3lt
	mfocrf 5, 16
                                        # implicit-def: $cr5lt
                                        # implicit-def: $f1
	rlwimi 5, 6, 20, 12, 12
	lwz 6, 216(1)
	mtocrf 16, 5
                                        # implicit-def: $cr3gt
	mfocrf 5, 16
	rlwimi 5, 6, 19, 13, 13
	lwz 6, 344(1)
	mtocrf 16, 5
	mfocrf 5, 4
	rlwimi 5, 6, 12, 20, 20
	mtocrf 4, 5
	bc 4, 20, .Lp9_int8_BB12_78
# %bb.77:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 1, 436(1)
	stfs 1, 36(4)
	stw 28, -24(3)
.Lp9_int8_BB12_78:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 6, 212(1)
                                        # implicit-def: $cr3un
	mfocrf 5, 16
                                        # implicit-def: $cr5lt
                                        # implicit-def: $f2
	rlwimi 5, 6, 17, 15, 15
	lwz 6, 340(1)
	mtocrf 16, 5
	mfocrf 5, 4
	rlwimi 5, 6, 12, 20, 20
	mtocrf 4, 5
	bc 4, 20, .Lp9_int8_BB12_80
# %bb.79:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 2, 440(1)
	stfs 2, 40(4)
	stw 28, -20(3)
.Lp9_int8_BB12_80:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 6, 336(1)
                                        # implicit-def: $cr5lt
	mfocrf 5, 4
                                        # implicit-def: $f3
	rlwimi 5, 6, 12, 20, 20
	mtocrf 4, 5
	bc 4, 20, .Lp9_int8_BB12_82
# %bb.81:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 3, 444(1)
	stfs 3, 44(4)
	stw 28, -16(3)
.Lp9_int8_BB12_82:                              #   in Loop: Header=BB12_26 Depth=3
	xststdcsp 7, 0, 112
	xststdcsp 5, 1, 112
                                        # implicit-def: $cr5lt
                                        # implicit-def: $f0
	xststdcsp 1, 2, 112
	xststdcsp 6, 3, 112
	lwz 6, 332(1)
	mfocrf 5, 4
	rlwimi 5, 6, 12, 20, 20
	mtocrf 4, 5
	bc 4, 20, .Lp9_int8_BB12_84
# %bb.83:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 464(1)
	stfs 0, 48(4)
	stw 28, -12(3)
.Lp9_int8_BB12_84:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 6, 328(1)
                                        # implicit-def: $cr5lt
	mfocrf 5, 4
                                        # implicit-def: $f1
	rlwimi 5, 6, 12, 20, 20
	mtocrf 4, 5
	bc 4, 20, .Lp9_int8_BB12_86
# %bb.85:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 1, 468(1)
	stfs 1, 52(4)
	stw 28, -8(3)
.Lp9_int8_BB12_86:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 6, 324(1)
                                        # implicit-def: $cr5lt
	mfocrf 5, 4
                                        # implicit-def: $f2
	rlwimi 5, 6, 12, 20, 20
	mtocrf 4, 5
	bc 4, 20, .Lp9_int8_BB12_88
# %bb.87:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 2, 472(1)
	stfs 2, 56(4)
	stw 28, -4(3)
.Lp9_int8_BB12_88:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 5, 320(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
                                        # implicit-def: $f3
	rlwimi 3, 5, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_90
# %bb.89:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 3, 476(1)
	stfs 3, 60(4)
	stwx 28, 23, 14
.Lp9_int8_BB12_90:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 3, 244(1)
                                        # implicit-def: $cr6lt
	rotlwi	3, 3, 24
	mtocrf 32, 3
	lwz 3, 240(1)
	crand 20, 12, 10
	rotlwi	3, 3, 24
	mtocrf 32, 3
	lwz 3, 236(1)
	lwz 4, 364(1)
	crand 21, 13, 10
	rotlwi	3, 3, 24
	mtocrf 32, 3
	mfocrf 3, 2
	rlwimi 3, 4, 8, 24, 24
	crand 23, 15, 10
	mtocrf 2, 3
	lwz 3, 232(1)
	lwz 4, 360(1)
                                        # implicit-def: $cr6gt
	rotlwi	3, 3, 24
	mtocrf 32, 3
	crand 24, 24, 10
	mfocrf 3, 2
	rlwimi 3, 4, 7, 25, 25
	lwz 4, 356(1)
	mtocrf 2, 3
	crand 25, 25, 2
	cror 20, 20, 25
                                        # implicit-def: $cr6gt
	mfocrf 3, 2
	rlwimi 3, 4, 7, 25, 25
	mtocrf 2, 3
	lwz 3, 224(1)
	lwz 4, 352(1)
	mtocrf 128, 3
	crand 25, 25, 2
	cror 21, 21, 25
                                        # implicit-def: $cr6gt
	mfocrf 3, 2
	rlwimi 3, 4, 7, 25, 25
	mtocrf 2, 3
	lwz 3, 228(1)
	lwz 4, 208(1)
                                        # implicit-def: $cr6un
	mtocrf 128, 3
	crand 25, 25, 2
	mfocrf 3, 2
	cror 23, 23, 25
	rlwimi 3, 4, 5, 27, 27
	lwz 4, 348(1)
	mtocrf 2, 3
	crand 25, 27, 14
	crmove	14, 27
	cror 24, 24, 25
                                        # implicit-def: $cr6gt
	mfocrf 3, 2
	rlwimi 3, 4, 7, 25, 25
	lwz 4, 344(1)
	mtocrf 2, 3
	crand 25, 25, 30
	cror 20, 20, 25
                                        # implicit-def: $cr6gt
	mfocrf 3, 2
	rlwimi 3, 4, 7, 25, 25
	lwz 4, 340(1)
	mtocrf 2, 3
	crand 22, 25, 22
	cror 21, 21, 22
                                        # implicit-def: $cr5eq
	mfocrf 3, 4
	rlwimi 3, 4, 10, 22, 22
	lwz 4, 336(1)
	xststdcsp 0, 0, 112
	mtocrf 4, 3
	crand 22, 22, 6
	xststdcsp 1, 1, 112
	cror 22, 23, 22
                                        # implicit-def: $cr5un
	mfocrf 3, 4
	rlwimi 3, 4, 9, 23, 23
	mtocrf 4, 3
	crand 23, 23, 26
	cror 23, 24, 23
	xststdcsp 6, 2, 112
	xststdcsp 7, 3, 112
	lwz 4, 332(1)
                                        # implicit-def: $cr6lt
	mfocrf 3, 2
	rlwimi 3, 4, 8, 24, 24
	lwz 4, 328(1)
	mtocrf 2, 3
	crand 24, 24, 2
	cror 20, 20, 24
                                        # implicit-def: $cr6lt
	mfocrf 3, 2
	rlwimi 3, 4, 8, 24, 24
	lwz 4, 324(1)
	mtocrf 2, 3
	crand 24, 24, 6
	cror 21, 21, 24
                                        # implicit-def: $cr6lt
	mfocrf 3, 2
	rlwimi 3, 4, 8, 24, 24
	lwz 4, 320(1)
	cror 20, 21, 20
	mtocrf 2, 3
	crand 24, 24, 26
	cror 22, 22, 24
                                        # implicit-def: $cr6lt
	mfocrf 3, 2
	rlwimi 3, 4, 8, 24, 24
	cror 20, 22, 20
	mtocrf 2, 3
	lwz 3, 204(1)
	crand 24, 24, 30
	cror 23, 23, 24
	rotlwi	3, 3, 24
	cror 20, 23, 20
	mtocrf 32, 3
	isel 24, 22, 24, 20
.Lp9_int8_BB12_91:                              #   in Loop: Header=BB12_26 Depth=3
	bc 4, 16, .Lp9_int8_BB12_25
# %bb.92:                               #   in Loop: Header=BB12_26 Depth=3
	ld 5, 192(1)                            # 8-byte Folded Reload
	ld 6, 184(1)                            # 8-byte Folded Reload
	bc 4, 8, .Lp9_int8_BB12_125
# %bb.93:                               #   in Loop: Header=BB12_26 Depth=3
	bc 12, 12, .Lp9_int8_BB12_155
# %bb.94:                               #   in Loop: Header=BB12_26 Depth=3
	bc 12, 13, .Lp9_int8_BB12_156
.Lp9_int8_BB12_95:                              #   in Loop: Header=BB12_26 Depth=3
	bc 4, 15, .Lp9_int8_BB12_97
.Lp9_int8_BB12_96:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 392(1)
	add 3, 6, 14
	stfs 0, -52(3)
	add 3, 5, 14
	stw 28, -52(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_97:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 364(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_99
# %bb.98:                               #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 396(1)
	add 3, 6, 14
	stfs 0, -48(3)
	add 3, 5, 14
	stw 28, -48(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_99:                              #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 360(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_101
# %bb.100:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 416(1)
	add 3, 6, 14
	stfs 0, -44(3)
	add 3, 5, 14
	stw 28, -44(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_101:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 356(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_103
# %bb.102:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 420(1)
	add 3, 6, 14
	stfs 0, -40(3)
	add 3, 5, 14
	stw 28, -40(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_103:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 352(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_105
# %bb.104:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 424(1)
	add 3, 6, 14
	stfs 0, -36(3)
	add 3, 5, 14
	stw 28, -36(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_105:                             #   in Loop: Header=BB12_26 Depth=3
	bc 4, 14, .Lp9_int8_BB12_107
# %bb.106:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 428(1)
	add 3, 6, 14
	stfs 0, -32(3)
	add 3, 5, 14
	stw 28, -32(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_107:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 348(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_109
# %bb.108:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 448(1)
	add 3, 6, 14
	stfs 0, -28(3)
	add 3, 5, 14
	stw 28, -28(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_109:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 344(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_111
# %bb.110:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 452(1)
	add 3, 6, 14
	stfs 0, -24(3)
	add 3, 5, 14
	stw 28, -24(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_111:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 340(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_113
# %bb.112:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 456(1)
	add 3, 6, 14
	stfs 0, -20(3)
	add 3, 5, 14
	stw 28, -20(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_113:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 336(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_115
# %bb.114:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 460(1)
	add 3, 6, 14
	stfs 0, -16(3)
	add 3, 5, 14
	stw 28, -16(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_115:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 332(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_117
# %bb.116:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 480(1)
	add 3, 6, 14
	stfs 0, -12(3)
	add 3, 5, 14
	stw 28, -12(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_117:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 328(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_119
# %bb.118:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 484(1)
	add 3, 6, 14
	stfs 0, -8(3)
	add 3, 5, 14
	stw 28, -8(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_119:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 324(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_121
# %bb.120:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 488(1)
	add 3, 6, 14
	stfs 0, -4(3)
	add 3, 5, 14
	stw 28, -4(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
.Lp9_int8_BB12_121:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 320(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_25
# %bb.122:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 492(1)
	stfsx 0, 6, 14
	stwx 28, 5, 14
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
	b .Lp9_int8_BB12_25
.Lp9_int8_BB12_123:                             #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 368(1)
	stfsx 0, 21, 14
	stw 28, -60(3)
	add 4, 21, 14
                                        # implicit-def: $f1
	bc 4, 13, .Lp9_int8_BB12_62
.Lp9_int8_BB12_124:                             #   in Loop: Header=BB12_26 Depth=3
	lfs 1, 372(1)
	stfs 1, 4(4)
	stw 28, -56(3)
                                        # implicit-def: $f2
	bc 12, 15, .Lp9_int8_BB12_63
	b .Lp9_int8_BB12_64
.Lp9_int8_BB12_125:                             #   in Loop: Header=BB12_26 Depth=3
	add 3, 6, 14
	add 4, 5, 14
                                        # implicit-def: $f0
	bc 12, 12, .Lp9_int8_BB12_157
# %bb.126:                              #   in Loop: Header=BB12_26 Depth=3
                                        # implicit-def: $f1
	bc 12, 13, .Lp9_int8_BB12_158
.Lp9_int8_BB12_127:                             #   in Loop: Header=BB12_26 Depth=3
                                        # implicit-def: $f2
	bc 4, 15, .Lp9_int8_BB12_129
.Lp9_int8_BB12_128:                             #   in Loop: Header=BB12_26 Depth=3
	lfs 2, 392(1)
	stfs 2, -52(3)
	stw 28, -52(4)
.Lp9_int8_BB12_129:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 8, 364(1)
                                        # implicit-def: $cr5lt
	mfocrf 7, 4
                                        # implicit-def: $f3
	rlwimi 7, 8, 12, 20, 20
	mtocrf 4, 7
	bc 4, 20, .Lp9_int8_BB12_131
# %bb.130:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 3, 396(1)
	stfs 3, -48(3)
	stw 28, -48(4)
.Lp9_int8_BB12_131:                             #   in Loop: Header=BB12_26 Depth=3
	xststdcsp 0, 0, 112
                                        # implicit-def: $cr5lt
                                        # implicit-def: $f0
	mfocrf 7, 128
	stw 7, 244(1)
	xststdcsp 0, 1, 112
	mfocrf 7, 128
	stw 7, 240(1)
	xststdcsp 0, 2, 112
	mfocrf 7, 128
	stw 7, 236(1)
	xststdcsp 0, 3, 112
	mfocrf 7, 128
	stw 7, 232(1)
	mfocrf 7, 4
	lwz 8, 360(1)
	rlwimi 7, 8, 12, 20, 20
	mtocrf 4, 7
	bc 4, 20, .Lp9_int8_BB12_133
# %bb.132:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 416(1)
	stfs 0, -44(3)
	stw 28, -44(4)
.Lp9_int8_BB12_133:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 8, 356(1)
                                        # implicit-def: $cr5lt
	mfocrf 7, 4
                                        # implicit-def: $f1
	rlwimi 7, 8, 12, 20, 20
	mtocrf 4, 7
	bc 4, 20, .Lp9_int8_BB12_135
# %bb.134:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 1, 420(1)
	stfs 1, -40(3)
	stw 28, -40(4)
.Lp9_int8_BB12_135:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 8, 352(1)
                                        # implicit-def: $cr5lt
	mfocrf 7, 4
                                        # implicit-def: $f2
	rlwimi 7, 8, 12, 20, 20
	mtocrf 4, 7
	bc 4, 20, .Lp9_int8_BB12_137
# %bb.136:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 2, 424(1)
	stfs 2, -36(3)
	stw 28, -36(4)
.Lp9_int8_BB12_137:                             #   in Loop: Header=BB12_26 Depth=3
                                        # implicit-def: $f3
	bc 4, 14, .Lp9_int8_BB12_139
# %bb.138:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 3, 428(1)
	stfs 3, -32(3)
	stw 28, -32(4)
.Lp9_int8_BB12_139:                             #   in Loop: Header=BB12_26 Depth=3
	xststdcsp 4, 0, 112
	xststdcsp 2, 1, 112
                                        # implicit-def: $cr5lt
                                        # implicit-def: $f0
	xststdcsp 0, 2, 112
	mfocrf 7, 128
	stw 7, 228(1)
	xststdcsp 3, 3, 112
	lwz 8, 348(1)
	mfocrf 7, 4
	rlwimi 7, 8, 12, 20, 20
	mtocrf 4, 7
	bc 4, 20, .Lp9_int8_BB12_141
# %bb.140:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 448(1)
	stfs 0, -28(3)
	stw 28, -28(4)
.Lp9_int8_BB12_141:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 8, 220(1)
                                        # implicit-def: $cr3lt
	mfocrf 7, 16
                                        # implicit-def: $cr5lt
                                        # implicit-def: $f1
	rlwimi 7, 8, 20, 12, 12
	lwz 8, 216(1)
	mtocrf 16, 7
                                        # implicit-def: $cr3gt
	mfocrf 7, 16
	rlwimi 7, 8, 19, 13, 13
	lwz 8, 344(1)
	mtocrf 16, 7
	mfocrf 7, 4
	rlwimi 7, 8, 12, 20, 20
	mtocrf 4, 7
	bc 4, 20, .Lp9_int8_BB12_143
# %bb.142:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 1, 452(1)
	stfs 1, -24(3)
	stw 28, -24(4)
.Lp9_int8_BB12_143:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 8, 212(1)
                                        # implicit-def: $cr3un
	mfocrf 7, 16
                                        # implicit-def: $cr5lt
                                        # implicit-def: $f2
	rlwimi 7, 8, 17, 15, 15
	lwz 8, 340(1)
	mtocrf 16, 7
	mfocrf 7, 4
	rlwimi 7, 8, 12, 20, 20
	mtocrf 4, 7
	bc 4, 20, .Lp9_int8_BB12_145
# %bb.144:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 2, 456(1)
	stfs 2, -20(3)
	stw 28, -20(4)
.Lp9_int8_BB12_145:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 8, 336(1)
                                        # implicit-def: $cr5lt
	mfocrf 7, 4
                                        # implicit-def: $f3
	rlwimi 7, 8, 12, 20, 20
	mtocrf 4, 7
	bc 4, 20, .Lp9_int8_BB12_147
# %bb.146:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 3, 460(1)
	stfs 3, -16(3)
	stw 28, -16(4)
.Lp9_int8_BB12_147:                             #   in Loop: Header=BB12_26 Depth=3
	xststdcsp 6, 0, 112
	xststdcsp 1, 1, 112
                                        # implicit-def: $f0
	xststdcsp 0, 2, 112
	xststdcsp 5, 3, 112
                                        # implicit-def: $cr5lt
	lwz 8, 332(1)
	mfocrf 7, 4
	rlwimi 7, 8, 12, 20, 20
	mtocrf 4, 7
	bc 4, 20, .Lp9_int8_BB12_149
# %bb.148:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 480(1)
	stfs 0, -12(3)
	stw 28, -12(4)
.Lp9_int8_BB12_149:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 8, 328(1)
                                        # implicit-def: $cr5lt
	mfocrf 7, 4
                                        # implicit-def: $f1
	rlwimi 7, 8, 12, 20, 20
	mtocrf 4, 7
	bc 4, 20, .Lp9_int8_BB12_151
# %bb.150:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 1, 484(1)
	stfs 1, -8(3)
	stw 28, -8(4)
.Lp9_int8_BB12_151:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 8, 324(1)
                                        # implicit-def: $cr5lt
	mfocrf 7, 4
                                        # implicit-def: $f2
	rlwimi 7, 8, 12, 20, 20
	mtocrf 4, 7
	bc 4, 20, .Lp9_int8_BB12_153
# %bb.152:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 2, 488(1)
	stfs 2, -4(3)
	stw 28, -4(4)
.Lp9_int8_BB12_153:                             #   in Loop: Header=BB12_26 Depth=3
	lwz 4, 320(1)
                                        # implicit-def: $cr5lt
	mfocrf 3, 4
                                        # implicit-def: $f3
	rlwimi 3, 4, 12, 20, 20
	mtocrf 4, 3
	bc 4, 20, .Lp9_int8_BB12_24
# %bb.154:                              #   in Loop: Header=BB12_26 Depth=3
	lfs 3, 492(1)
	stfsx 3, 6, 14
	stwx 28, 5, 14
	b .Lp9_int8_BB12_24
.Lp9_int8_BB12_155:                             #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 384(1)
	add 3, 6, 14
	stfs 0, -60(3)
	add 3, 5, 14
	stw 28, -60(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
	bc 4, 13, .Lp9_int8_BB12_95
.Lp9_int8_BB12_156:                             #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 388(1)
	add 3, 6, 14
	stfs 0, -56(3)
	add 3, 5, 14
	stw 28, -56(3)
	xststdcsp 0, 0, 112
	iseleq	24, 22, 24
	bc 12, 15, .Lp9_int8_BB12_96
	b .Lp9_int8_BB12_97
.Lp9_int8_BB12_157:                             #   in Loop: Header=BB12_26 Depth=3
	lfs 0, 384(1)
	stfs 0, -60(3)
	stw 28, -60(4)
                                        # implicit-def: $f1
	bc 4, 13, .Lp9_int8_BB12_127
.Lp9_int8_BB12_158:                             #   in Loop: Header=BB12_26 Depth=3
	lfs 1, 388(1)
	stfs 1, -56(3)
	stw 28, -56(4)
                                        # implicit-def: $f2
	bc 12, 15, .Lp9_int8_BB12_128
	b .Lp9_int8_BB12_129
	.long	0
	.quad	0
.Lp9_int8_func_end12:
	.size	pq8_shared_project_packed_power9, .Lp9_int8_func_end12-.Lp9_int8_func_begin12
	.cfi_endproc
                                        # -- End function
