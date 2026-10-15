# int8/pq8_project_packed_power9.s: kernel.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/quantized_matmul_power9_int8.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it, except the hand-written epilogue fast path
# marked below). Included by src/kernels/*_asm.rs.
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function pq8_project_packed_power9
.Lp9_int8_CPI4_0:
	.quad	6                               # 0x6
	.quad	6                               # 0x6
.Lp9_int8_CPI4_1:
	.quad	4                               # 0x4
	.quad	4                               # 0x4
.Lp9_int8_CPI4_2:
	.quad	512                             # 0x200
	.quad	512                             # 0x200
	.section	.text.pq8_project_packed_power9,"ax",@progbits
	.globl	pq8_project_packed_power9
	.p2align 6
	.type	pq8_project_packed_power9,@function
pq8_project_packed_power9:            # @pq8_project_packed_power9
.Lp9_int8_func_begin4:
	.cfi_startproc
.Lp9_int8_func_gep4:
	addis 2, 12, .TOC.-.Lp9_int8_func_gep4@ha
	addi 2, 2, .TOC.-.Lp9_int8_func_gep4@l
.Lp9_int8_func_lep4:
	.localentry	pq8_project_packed_power9, .Lp9_int8_func_lep4-.Lp9_int8_func_gep4
# %bb.0:
	mfcr 12
	mflr 0
	stw 12, 8(1)
	stdu 1, -2224(1)
	std 0, 2240(1)
	.cfi_def_cfa_offset 2224
	.cfi_offset lr, 16
	.cfi_offset r14, -208
	.cfi_offset r15, -200
	.cfi_offset r16, -192
	.cfi_offset r17, -184
	.cfi_offset r18, -176
	.cfi_offset r19, -168
	.cfi_offset r20, -160
	.cfi_offset r21, -152
	.cfi_offset r22, -144
	.cfi_offset r23, -136
	.cfi_offset r24, -128
	.cfi_offset r25, -120
	.cfi_offset r26, -112
	.cfi_offset r27, -104
	.cfi_offset r28, -96
	.cfi_offset r29, -88
	.cfi_offset r30, -80
	.cfi_offset r31, -72
	.cfi_offset f24, -64
	.cfi_offset f25, -56
	.cfi_offset f26, -48
	.cfi_offset f27, -40
	.cfi_offset f28, -32
	.cfi_offset f29, -24
	.cfi_offset f30, -16
	.cfi_offset f31, -8
	.cfi_offset cr2, 8
	.cfi_offset cr3, 8
	.cfi_offset cr4, 8
	.cfi_offset v20, -400
	.cfi_offset v21, -384
	.cfi_offset v22, -368
	.cfi_offset v23, -352
	.cfi_offset v24, -336
	.cfi_offset v25, -320
	.cfi_offset v26, -304
	.cfi_offset v27, -288
	.cfi_offset v28, -272
	.cfi_offset v29, -256
	.cfi_offset v30, -240
	.cfi_offset v31, -224
	std 17, 2040(1)                         # 8-byte Folded Spill
	std 29, 2136(1)                         # 8-byte Folded Spill
	ld 29, 2336(1)
	ld 17, 2328(1)
	cmpdi	4, 0
	stxv 52, 1824(1)                        # 16-byte Folded Spill
	stxv 53, 1840(1)                        # 16-byte Folded Spill
	stxv 54, 1856(1)                        # 16-byte Folded Spill
	std 14, 2016(1)                         # 8-byte Folded Spill
	li 14, 1
	std 15, 2024(1)                         # 8-byte Folded Spill
	stxv 55, 1872(1)                        # 16-byte Folded Spill
	std 16, 2032(1)                         # 8-byte Folded Spill
	stxv 56, 1888(1)                        # 16-byte Folded Spill
	std 18, 2048(1)                         # 8-byte Folded Spill
	stxv 57, 1904(1)                        # 16-byte Folded Spill
	std 19, 2056(1)                         # 8-byte Folded Spill
	std 20, 2064(1)                         # 8-byte Folded Spill
	stxv 58, 1920(1)                        # 16-byte Folded Spill
	std 21, 2072(1)                         # 8-byte Folded Spill
	stxv 59, 1936(1)                        # 16-byte Folded Spill
	std 22, 2080(1)                         # 8-byte Folded Spill
	stxv 60, 1952(1)                        # 16-byte Folded Spill
	std 23, 2088(1)                         # 8-byte Folded Spill
	std 24, 2096(1)                         # 8-byte Folded Spill
	stxv 61, 1968(1)                        # 16-byte Folded Spill
	std 25, 2104(1)                         # 8-byte Folded Spill
	stxv 62, 1984(1)                        # 16-byte Folded Spill
	std 26, 2112(1)                         # 8-byte Folded Spill
	stxv 63, 2000(1)                        # 16-byte Folded Spill
	std 27, 2120(1)                         # 8-byte Folded Spill
	std 28, 2128(1)                         # 8-byte Folded Spill
	std 30, 2144(1)                         # 8-byte Folded Spill
	std 31, 2152(1)                         # 8-byte Folded Spill
	stfd 24, 2160(1)                        # 8-byte Folded Spill
	stfd 25, 2168(1)                        # 8-byte Folded Spill
	stfd 26, 2176(1)                        # 8-byte Folded Spill
	stfd 27, 2184(1)                        # 8-byte Folded Spill
	stfd 28, 2192(1)                        # 8-byte Folded Spill
	stfd 29, 2200(1)                        # 8-byte Folded Spill
	stfd 30, 2208(1)                        # 8-byte Folded Spill
	stfd 31, 2216(1)                        # 8-byte Folded Spill
	bc 12, 2, .Lp9_int8_BB4_25
# %bb.1:
	cmpdi	10, 0
	bc 12, 2, .Lp9_int8_BB4_25
# %bb.2:
	cmpdi	29, 0
	bc 12, 2, .Lp9_int8_BB4_25
# %bb.3:
	cmpdi	9, 0
	bc 12, 2, .Lp9_int8_BB4_25
# %bb.4:
	cmpdi	17, 0
	bc 12, 2, .Lp9_int8_BB4_25
# %bb.5:
	mr	30, 8
	or 8, 10, 4
	andi. 8, 8, 15
	bne	0, .Lp9_int8_BB4_25
# %bb.6:
	cmplwi 4, 3, 35
	std 4, 448(1)                           # 8-byte Folded Spill
	bgt	4, .Lp9_int8_BB4_21
# %bb.7:
	clrldi	4, 3, 32
	li 11, 1
	li 8, 256
	sld 4, 11, 4
	lis 11, 128
	ori 11, 11, 5
	rldic 11, 11, 12, 28
	and. 4, 4, 11
	beq	0, .Lp9_int8_BB4_21
# %bb.8:
	cmpdi	8, 0
	bc 12, 2, .Lp9_int8_BB4_22
.Lp9_int8_BB4_9:
	cmpdi	6, 0
	bc 12, 2, .Lp9_int8_BB4_22
# %bb.10:
	cmpdi	7, 0
	bc 12, 2, .Lp9_int8_BB4_22
# %bb.11:
	addi 4, 8, -1
	li 11, -1
	rldimi 4, 11, 61, 0
	and. 4, 4, 7
	bne	0, .Lp9_int8_BB4_22
# %bb.12:
	cnttzd 8, 8
	addi 4, 6, 3
	rlwinm 11, 3, 0, 31, 29
	rldicl 4, 4, 62, 2
	cmpwi 2, 11, 12
	srd 8, 7, 8
	bgt	4, .Lp9_int8_BB4_14
# %bb.13:
	clrldi	12, 3, 32
	li 0, 1
	li 11, 256
	sld 12, 0, 12
	lis 0, 128
	ori 0, 0, 5
	rldic 0, 0, 12, 28
	and. 12, 12, 0
	bne	0, .Lp9_int8_BB4_15
.Lp9_int8_BB4_14:
	rlwinm 11, 3, 0, 30, 28
	cmpwi 1, 3, 8
	cmpwi	11, 2
	li 11, 32
	crnor 20, 6, 2
	isel 11, 0, 11, 20
.Lp9_int8_BB4_15:
	cmpwi	3, 12
	cmpwi 3, 3, 2
	li 12, 1
	li 28, 2
	crnor 20, 14, 2
	cmplwi	3, 14
	isel 12, 28, 12, 20
	li 28, 288
	sld 11, 11, 12
	li 12, 16
	iseleq	12, 28, 12
	cmplwi	3, 12
	li 28, 224
	iseleq	12, 28, 12
	li 28, 2
	add 11, 11, 12
	li 12, 3
	isel 21, 28, 12, 10
	sld 11, 11, 21
	mulld 8, 11, 8
	li 11, 7
	isel 11, 12, 11, 10
	add 4, 4, 11
	mr	18, 11
	srd 4, 4, 21
	mulhdu. 11, 8, 4
	mcrf 1, 0
	mulld. 4, 8, 4
	crandc 20, 6, 2
	bc 4, 20, .Lp9_int8_BB4_22
# %bb.16:
	li 14, 3
	cmpld	5, 4
	blt	0, .Lp9_int8_BB4_25
# %bb.17:
	cmpld	6, 30
	sub	4, 6, 30
	li 14, 1
	bc 12, 0, .Lp9_int8_BB4_25
# %bb.18:
	cmpld	9, 4
	bc 12, 1, .Lp9_int8_BB4_25
# %bb.19:
	ld 27, 2352(1)
	not	4, 9
	cmpld	27, 4
	ble 0, .Lp9_int8_BB4_26
# %bb.20:
	li 14, 4
	b .Lp9_int8_BB4_25
.Lp9_int8_BB4_21:
	rlwinm 4, 3, 0, 30, 28
	cmpwi 1, 3, 8
	cmpwi	4, 2
	li 4, 32
	crnor 20, 6, 2
	isel 8, 0, 4, 20
	cmpdi	8, 0
	bc 4, 2, .Lp9_int8_BB4_9
.Lp9_int8_BB4_22:
	li 14, 2
	bgt	4, .Lp9_int8_BB4_25
# %bb.23:
	clrldi	3, 3, 32
	li 4, 1
	sld 3, 4, 3
	rldic 4, 4, 35, 28
	ori 4, 4, 20804
	and. 3, 3, 4
	beq	0, .Lp9_int8_BB4_25
# %bb.24:
	li 14, 3
.Lp9_int8_BB4_25:
	lxv 63, 2000(1)                         # 16-byte Folded Reload
	lxv 62, 1984(1)                         # 16-byte Folded Reload
	lxv 61, 1968(1)                         # 16-byte Folded Reload
	lxv 60, 1952(1)                         # 16-byte Folded Reload
	clrldi	3, 14, 32
	lxv 59, 1936(1)                         # 16-byte Folded Reload
	lxv 58, 1920(1)                         # 16-byte Folded Reload
	lxv 57, 1904(1)                         # 16-byte Folded Reload
	lxv 56, 1888(1)                         # 16-byte Folded Reload
	lxv 55, 1872(1)                         # 16-byte Folded Reload
	lxv 54, 1856(1)                         # 16-byte Folded Reload
	lxv 53, 1840(1)                         # 16-byte Folded Reload
	lxv 52, 1824(1)                         # 16-byte Folded Reload
	lfd 31, 2216(1)                         # 8-byte Folded Reload
	lfd 30, 2208(1)                         # 8-byte Folded Reload
	ld 31, 2152(1)                          # 8-byte Folded Reload
	lfd 29, 2200(1)                         # 8-byte Folded Reload
	ld 30, 2144(1)                          # 8-byte Folded Reload
	ld 29, 2136(1)                          # 8-byte Folded Reload
	ld 28, 2128(1)                          # 8-byte Folded Reload
	lfd 28, 2192(1)                         # 8-byte Folded Reload
	ld 27, 2120(1)                          # 8-byte Folded Reload
	ld 26, 2112(1)                          # 8-byte Folded Reload
	ld 25, 2104(1)                          # 8-byte Folded Reload
	lfd 27, 2184(1)                         # 8-byte Folded Reload
	ld 24, 2096(1)                          # 8-byte Folded Reload
	ld 23, 2088(1)                          # 8-byte Folded Reload
	ld 22, 2080(1)                          # 8-byte Folded Reload
	lfd 26, 2176(1)                         # 8-byte Folded Reload
	ld 21, 2072(1)                          # 8-byte Folded Reload
	ld 20, 2064(1)                          # 8-byte Folded Reload
	ld 19, 2056(1)                          # 8-byte Folded Reload
	lfd 25, 2168(1)                         # 8-byte Folded Reload
	ld 18, 2048(1)                          # 8-byte Folded Reload
	ld 17, 2040(1)                          # 8-byte Folded Reload
	ld 16, 2032(1)                          # 8-byte Folded Reload
	lfd 24, 2160(1)                         # 8-byte Folded Reload
	ld 15, 2024(1)                          # 8-byte Folded Reload
	ld 14, 2016(1)                          # 8-byte Folded Reload
	addi 1, 1, 2224
	ld 0, 16(1)
	lwz 12, 8(1)
	mtlr 0
	mtocrf 32, 12
	mtocrf 16, 12
	mtocrf 8, 12
	blr
.Lp9_int8_BB4_26:
	ld 28, 2344(1)
	add 4, 27, 9
	cmpld	4, 28
	bgt	0, .Lp9_int8_BB4_25
# %bb.27:
	li 14, 4
	mulhdu. 4, 28, 17
	bne	0, .Lp9_int8_BB4_25
# %bb.28:
	mr	23, 9
	ld 9, 2320(1)
	addi 8, 1, 1600
	mr	26, 7
	mr	4, 7
	mr	5, 17
	mr	6, 10
	mr	25, 3
	mr	7, 9
	bl p9_int8_i8c_input_view
	cmplwi	3, 0
	beq	0, .Lp9_int8_BB4_158
# %bb.29:
	mr	3, 25
	bgt	4, .Lp9_int8_BB4_31
# %bb.30:
	clrldi	4, 3, 32
	li 5, 1
	li 24, 256
	sld 4, 5, 4
	lis 5, 128
	ori 5, 5, 5
	rldic 5, 5, 12, 28
	and. 4, 4, 5
	bne	0, .Lp9_int8_BB4_32
.Lp9_int8_BB4_31:
	rlwinm 4, 3, 0, 30, 28
	cmpwi 1, 3, 8
	cmpwi	4, 2
	li 4, 32
	crnor 20, 6, 2
	isel 24, 0, 4, 20
.Lp9_int8_BB4_32:
	bl p9_int8_record_bytes
	add 23, 23, 30
	rldicl 19, 30, 62, 2
	li 14, 0
	addi 4, 23, -1
	rldicl 22, 4, 62, 2
	cmpld	19, 22
	bgt	0, .Lp9_int8_BB4_25
# %bb.33:
	cmpldi	17, 3
	li 5, 1
	li 7, 2
	sldi 20, 26, 2
	mr	4, 26
	slwi 6, 25, 5
	cnttzd 10, 24
	cmpldi 1, 17, 2
	li 11, 1
	li 24, 4
	li 26, 2
	li 8, 16
	li 9, 4
	isellt	12, 7, 5
	isel 7, 5, 7, 10
	addi 5, 1, 968
	std 5, 512(1)                           # 8-byte Folded Spill
	addi 5, 20, 64
	sldi 25, 4, 3
	xxlxor 60, 60, 60
	crand 20, 14, 5
	isellt	31, 11, 14
	li 14, 0
	isellt	16, 26, 24
	isel 9, 9, 8, 20
	std 5, 320(1)                           # 8-byte Folded Spill
	addi 5, 20, 32
	std 5, 312(1)                           # 8-byte Folded Spill
	addi 5, 20, 112
	std 5, 304(1)                           # 8-byte Folded Spill
	addi 5, 25, 112
	std 5, 296(1)                           # 8-byte Folded Spill
	srd 5, 4, 10
	slw 10, 12, 7
	cmplwi	10, 8
	slwi 7, 10, 1
	rlwinm 11, 10, 2, 0, 29
	mulld 3, 5, 3
	mfocrf 8, 128
	andi. 12, 10, 2
	add 6, 7, 6
	stw 8, 340(1)
	rlwinm 8, 10, 0, 27, 28
	std 12, 144(1)                          # 8-byte Folded Spill
	mfocrf 12, 128
	cmplwi	10, 4
	rlwimi 31, 6, 0, 0, 30
	mulli 6, 4, 12
	stw 12, 188(1)
	mfocrf 12, 128
	cmpdi	8, 8
	cmpwi 4, 31, 387
	cmpwi 2, 31, 199
	mfocrf 7, 128
	cmpdi	8, 16
	stw 12, 184(1)
	stw 7, 180(1)
	mfocrf 7, 128
	cmplwi	31, 72
	stw 7, 140(1)
	addi 7, 6, 64
	std 6, 288(1)                           # 8-byte Folded Spill
	std 11, 192(1)                          # 8-byte Folded Spill
	std 7, 280(1)                           # 8-byte Folded Spill
	addi 7, 25, 64
	std 7, 272(1)                           # 8-byte Folded Spill
	addi 7, 6, 48
	addi 6, 6, 112
	std 7, 264(1)                           # 8-byte Folded Spill
	addi 7, 25, 48
	std 6, 248(1)                           # 8-byte Folded Spill
	sldi 6, 19, 4
	sldi 25, 28, 2
	std 7, 256(1)                           # 8-byte Folded Spill
	sldi 7, 27, 2
	add 6, 6, 7
	sldi 7, 30, 2
	sub	6, 6, 7
	addi 7, 1, 1504
	add 6, 6, 29
	std 7, 128(1)                           # 8-byte Folded Spill
	addi 7, 1, 1440
	addi 6, 6, 8
	std 7, 120(1)                           # 8-byte Folded Spill
	addi 7, 1, 1528
	std 6, 96(1)                            # 8-byte Folded Spill
	mulld 6, 9, 4
	sldi 4, 4, 4
	std 7, 112(1)                           # 8-byte Folded Spill
	addi 7, 11, -4
	rldicl 7, 7, 62, 2
	addi 15, 7, 1
	sldi 6, 6, 2
	std 6, 64(1)                            # 8-byte Folded Spill
	mfocrf 6, 128
	cmplwi	31, 81
	stw 6, 440(1)
	std 4, 504(1)                           # 8-byte Folded Spill
	mulld 4, 9, 28
	addis 6, 2, .Lp9_int8_CPI4_0@toc@ha
	mfocrf 5, 128
	cmplwi	31, 200
	std 9, 72(1)                            # 8-byte Folded Spill
	std 20, 424(1)                          # 8-byte Folded Spill
	std 20, 216(1)                          # 8-byte Folded Spill
	std 10, 344(1)                          # 8-byte Folded Spill
	mr	20, 21
	addi 6, 6, .Lp9_int8_CPI4_0@toc@l
	lxv 52, 0(6)
	addis 6, 2, .Lp9_int8_CPI4_1@toc@ha
	sldi 4, 4, 2
	addi 6, 6, .Lp9_int8_CPI4_1@toc@l
	std 4, 56(1)                            # 8-byte Folded Spill
	sldi 4, 28, 4
	lxv 0, 0(6)
	addis 6, 2, .Lp9_int8_CPI4_2@toc@ha
	std 4, 496(1)                           # 8-byte Folded Spill
	addi 4, 1, 1464
	addi 6, 6, .Lp9_int8_CPI4_2@toc@l
	std 4, 104(1)                           # 8-byte Folded Spill
	li 4, 64
	std 4, 240(1)                           # 8-byte Folded Spill
	li 4, 32
	stxv 0, 400(1)                          # 16-byte Folded Spill
	lxv 0, 0(6)
	std 4, 232(1)                           # 8-byte Folded Spill
	li 4, 0
	std 4, 224(1)                           # 8-byte Folded Spill
	rlwinm 4, 10, 0, 29, 29
	stxv 0, 384(1)                          # 16-byte Folded Spill
	std 4, 160(1)                           # 8-byte Folded Spill
	mr	4, 3
	sldi 3, 19, 2
	std 3, 48(1)                            # 8-byte Folded Spill
	sldi 3, 10, 4
	std 3, 152(1)                           # 8-byte Folded Spill
	stw 5, 372(1)
	mfocrf 5, 128
	cmplwi	31, 209
	li 3, 0
	stw 5, 444(1)
	mfocrf 5, 128
	stw 5, 436(1)
	std 21, 200(1)                          # 8-byte Folded Spill
	mr	21, 18
	std 17, 520(1)                          # 8-byte Folded Spill
	mr	18, 4
	std 19, 80(1)                           # 8-byte Folded Spill
	stw 31, 460(1)                          # 4-byte Folded Spill
	std 16, 472(1)                          # 8-byte Folded Spill
	std 15, 464(1)                          # 8-byte Folded Spill
	std 4, 168(1)                           # 8-byte Folded Spill
	std 25, 488(1)                          # 8-byte Folded Spill
	ld 27, 448(1)                           # 8-byte Folded Reload
	std 21, 328(1)                          # 8-byte Folded Spill
	b .Lp9_int8_BB4_35
.Lp9_int8_BB4_34:                               #   in Loop: Header=BB4_35 Depth=1
	ld 4, 64(1)                             # 8-byte Folded Reload
	ld 3, 216(1)                            # 8-byte Folded Reload
	ld 19, 80(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	std 3, 216(1)                           # 8-byte Folded Spill
	ld 3, 224(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	std 3, 224(1)                           # 8-byte Folded Spill
	ld 3, 320(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	std 3, 320(1)                           # 8-byte Folded Spill
	ld 3, 240(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	std 3, 240(1)                           # 8-byte Folded Spill
	ld 3, 232(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	std 3, 232(1)                           # 8-byte Folded Spill
	ld 3, 312(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	std 3, 312(1)                           # 8-byte Folded Spill
	ld 3, 304(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	std 3, 304(1)                           # 8-byte Folded Spill
	ld 3, 296(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	std 3, 296(1)                           # 8-byte Folded Spill
	ld 3, 288(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	std 3, 288(1)                           # 8-byte Folded Spill
	ld 3, 280(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	std 3, 280(1)                           # 8-byte Folded Spill
	ld 3, 272(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	std 3, 272(1)                           # 8-byte Folded Spill
	ld 3, 264(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	std 3, 264(1)                           # 8-byte Folded Spill
	ld 3, 256(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	std 3, 256(1)                           # 8-byte Folded Spill
	ld 3, 248(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	ld 4, 56(1)                             # 8-byte Folded Reload
	std 3, 248(1)                           # 8-byte Folded Spill
	ld 3, 96(1)                             # 8-byte Folded Reload
	add 3, 3, 4
	std 3, 96(1)                            # 8-byte Folded Spill
	ld 3, 88(1)                             # 8-byte Folded Reload
	cmpld	3, 17
	bge 0, .Lp9_int8_BB4_25
.Lp9_int8_BB4_35:                               # =>This Loop Header: Depth=1
                                        #     Child Loop BB4_38 Depth 2
                                        #       Child Loop BB4_41 Depth 3
                                        #       Child Loop BB4_44 Depth 3
                                        #     Child Loop BB4_49 Depth 2
                                        #       Child Loop BB4_52 Depth 3
                                        #       Child Loop BB4_55 Depth 3
                                        #       Child Loop BB4_63 Depth 3
                                        #         Child Loop BB4_85 Depth 4
                                        #           Child Loop BB4_86 Depth 5
                                        #         Child Loop BB4_102 Depth 4
                                        #           Child Loop BB4_103 Depth 5
                                        #         Child Loop BB4_115 Depth 4
                                        #           Child Loop BB4_116 Depth 5
                                        #         Child Loop BB4_70 Depth 4
                                        #           Child Loop BB4_71 Depth 5
                                        #         Child Loop BB4_97 Depth 4
                                        #           Child Loop BB4_98 Depth 5
                                        #         Child Loop BB4_110 Depth 4
                                        #           Child Loop BB4_111 Depth 5
                                        #         Child Loop BB4_79 Depth 4
                                        #         Child Loop BB4_94 Depth 4
                                        #         Child Loop BB4_107 Depth 4
                                        #         Child Loop BB4_120 Depth 4
                                        #         Child Loop BB4_147 Depth 4
                                        #           Child Loop BB4_150 Depth 5
	ld 4, 72(1)                             # 8-byte Folded Reload
	mr	6, 3
	sub	3, 17, 3
	std 6, 208(1)                           # 8-byte Folded Spill
	add 5, 6, 4
	cmpld	3, 4
	isellt	3, 17, 5
	std 5, 88(1)                            # 8-byte Folded Spill
	std 3, 528(1)                           # 8-byte Folded Spill
	cmpld	6, 3
	mr	3, 19
	bge 0, .Lp9_int8_BB4_38
# %bb.36:                               #   in Loop: Header=BB4_35 Depth=1
	ld 3, 96(1)                             # 8-byte Folded Reload
	ld 31, 48(1)                            # 8-byte Folded Reload
	std 19, 360(1)                          # 8-byte Folded Spill
	std 3, 352(1)                           # 8-byte Folded Spill
	b .Lp9_int8_BB4_49
.Lp9_int8_BB4_37:                               #   in Loop: Header=BB4_38 Depth=2
	ld 4, 344(1)                            # 8-byte Folded Reload
	ld 25, 488(1)                           # 8-byte Folded Reload
	add 3, 3, 4
	cmpld	3, 22
	bgt	0, .Lp9_int8_BB4_34
.Lp9_int8_BB4_38:                               #   Parent Loop BB4_35 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB4_41 Depth 3
                                        #       Child Loop BB4_44 Depth 3
	lwz 4, 340(1)
	mtocrf 128, 4
	bc 4, 0, .Lp9_int8_BB4_45
# %bb.39:                               #   in Loop: Header=BB4_38 Depth=2
	lwz 5, 184(1)
	ld 10, 160(1)                           # 8-byte Folded Reload
	li 4, 0
	mtocrf 128, 5
	bc 12, 0, .Lp9_int8_BB4_42
# %bb.40:                               #   in Loop: Header=BB4_38 Depth=2
	ld 5, 128(1)                            # 8-byte Folded Reload
	ld 6, 120(1)                            # 8-byte Folded Reload
	.p2align	5
.Lp9_int8_BB4_41:                               #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_38 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	add 7, 3, 4
	addi 4, 4, 4
	cmpld	7, 22
	iselgt	8, 3, 7
	and 9, 8, 21
	srd 8, 8, 20
	maddld 8, 18, 8, 27
	stdu 9, 32(6)
	addi 9, 7, 1
	cmpld	9, 22
	stdu 8, 32(5)
	iselgt	8, 3, 9
	srd 9, 8, 20
	and 8, 8, 21
	std 8, 8(6)
	addi 8, 7, 2
	maddld 9, 18, 9, 27
	addi 7, 7, 3
	cmpld	8, 22
	iselgt	8, 3, 8
	cmpld	7, 22
	std 9, 8(5)
	srd 9, 8, 20
	and 8, 8, 21
	iselgt	7, 3, 7
	cmpld	10, 4
	std 8, 16(6)
	srd 8, 7, 20
	maddld 9, 18, 9, 27
	and 7, 7, 21
	maddld 8, 18, 8, 27
	std 7, 24(6)
	std 9, 16(5)
	std 8, 24(5)
	bne	0, .Lp9_int8_BB4_41
.Lp9_int8_BB4_42:                               #   in Loop: Header=BB4_38 Depth=2
	lwz 5, 188(1)
	mtocrf 128, 5
	bc 12, 2, .Lp9_int8_BB4_37
# %bb.43:                               #   in Loop: Header=BB4_38 Depth=2
	ld 5, 112(1)                            # 8-byte Folded Reload
	ld 7, 104(1)                            # 8-byte Folded Reload
	sldi 6, 4, 3
	add 4, 4, 3
	add 5, 5, 6
	add 6, 7, 6
	ld 7, 144(1)                            # 8-byte Folded Reload
	mtctr 7
	.p2align	5
.Lp9_int8_BB4_44:                               #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_38 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	cmpld	4, 22
	iselgt	7, 3, 4
	addi 4, 4, 1
	and 8, 7, 21
	srd 7, 7, 20
	maddld 7, 18, 7, 27
	stdu 8, 8(6)
	stdu 7, 8(5)
	bdnz .Lp9_int8_BB4_44
	b .Lp9_int8_BB4_37
.Lp9_int8_BB4_45:                               #   in Loop: Header=BB4_38 Depth=2
	addi 4, 3, 1
	ld 24, 448(1)                           # 8-byte Folded Reload
	srd 11, 3, 20
	addi 5, 3, 2
	addi 6, 3, 3
	maddld 11, 18, 11, 24
	cmpld	4, 22
	iselgt	4, 3, 4
	ld 24, 448(1)                           # 8-byte Folded Reload
	cmpld	5, 22
	addi 7, 3, 4
	addi 8, 3, 5
	addi 9, 3, 6
	addi 10, 3, 7
	srd 12, 4, 20
	iselgt	5, 3, 5
	cmpld	6, 22
	and 4, 4, 21
	maddld 12, 18, 12, 24
	ld 24, 448(1)                           # 8-byte Folded Reload
	srd 0, 5, 20
	iselgt	6, 3, 6
	cmpld	7, 22
	and 5, 5, 21
	std 4, 1480(1)
	std 11, 1536(1)
	and 11, 3, 21
	srd 29, 6, 20
	iselgt	7, 3, 7
	cmpld	8, 22
	and 6, 6, 21
	std 5, 1488(1)
	std 11, 1472(1)
	maddld 0, 18, 0, 24
	ld 24, 448(1)                           # 8-byte Folded Reload
	srd 28, 7, 20
	iselgt	8, 3, 8
	cmpld	9, 22
	and 7, 7, 21
	std 6, 1496(1)
	std 12, 1544(1)
	srd 27, 8, 20
	iselgt	9, 3, 9
	cmpld	10, 22
	and 8, 8, 21
	std 7, 1504(1)
	maddld 29, 18, 29, 24
	ld 24, 448(1)                           # 8-byte Folded Reload
	srd 26, 9, 20
	iselgt	10, 3, 10
	and 9, 9, 21
	std 8, 1512(1)
	std 0, 1552(1)
	srd 25, 10, 20
	and 10, 10, 21
	std 9, 1520(1)
	maddld 28, 18, 28, 24
	ld 24, 448(1)                           # 8-byte Folded Reload
	std 10, 1528(1)
	std 29, 1560(1)
	maddld 27, 18, 27, 24
	ld 24, 448(1)                           # 8-byte Folded Reload
	std 28, 1568(1)
	maddld 26, 18, 26, 24
	ld 24, 448(1)                           # 8-byte Folded Reload
	std 27, 1576(1)
	ld 27, 448(1)                           # 8-byte Folded Reload
	maddld 25, 18, 25, 24
	std 26, 1584(1)
	std 25, 1592(1)
	lwz 4, 180(1)
	mtocrf 128, 4
	bc 12, 2, .Lp9_int8_BB4_37
# %bb.46:                               #   in Loop: Header=BB4_38 Depth=2
	addi 4, 3, 8
	addi 5, 3, 9
	ld 21, 448(1)                           # 8-byte Folded Reload
	addi 6, 3, 10
	addi 7, 3, 11
	addi 8, 3, 12
	addi 9, 3, 13
	addi 10, 3, 14
	addi 11, 3, 15
	cmpld	4, 22
	iselgt	4, 3, 4
	cmpld	5, 22
	srd 12, 4, 20
	iselgt	5, 3, 5
	cmpld	6, 22
	maddld 12, 18, 12, 21
	ld 21, 448(1)                           # 8-byte Folded Reload
	srd 0, 5, 20
	iselgt	6, 3, 6
	cmpld	7, 22
	srd 29, 6, 20
	iselgt	7, 3, 7
	cmpld	8, 22
	maddld 0, 18, 0, 21
	ld 21, 448(1)                           # 8-byte Folded Reload
	srd 28, 7, 20
	iselgt	8, 3, 8
	cmpld	9, 22
	std 12, 1600(1)
	srd 27, 8, 20
	iselgt	9, 3, 9
	cmpld	10, 22
	maddld 29, 18, 29, 21
	ld 21, 448(1)                           # 8-byte Folded Reload
	srd 26, 9, 20
	iselgt	10, 3, 10
	cmpld	11, 22
	std 0, 1608(1)
	srd 25, 10, 20
	iselgt	11, 3, 11
	maddld 28, 18, 28, 21
	ld 21, 448(1)                           # 8-byte Folded Reload
	srd 24, 11, 20
	std 29, 1616(1)
	maddld 27, 18, 27, 21
	ld 21, 448(1)                           # 8-byte Folded Reload
	std 28, 1624(1)
	maddld 26, 18, 26, 21
	ld 21, 448(1)                           # 8-byte Folded Reload
	std 27, 1632(1)
	ld 27, 448(1)                           # 8-byte Folded Reload
	maddld 25, 18, 25, 21
	ld 21, 448(1)                           # 8-byte Folded Reload
	std 26, 1640(1)
	maddld 24, 18, 24, 21
	ld 21, 328(1)                           # 8-byte Folded Reload
	std 25, 1648(1)
	and 4, 4, 21
	and 5, 5, 21
	and 6, 6, 21
	and 7, 7, 21
	and 8, 8, 21
	and 9, 9, 21
	and 10, 10, 21
	and 11, 11, 21
	std 24, 1656(1)
	std 4, 1536(1)
	std 5, 1544(1)
	std 6, 1552(1)
	std 7, 1560(1)
	std 8, 1568(1)
	std 9, 1576(1)
	std 10, 1584(1)
	std 11, 1592(1)
	lwz 4, 140(1)
	mtocrf 128, 4
	bc 12, 2, .Lp9_int8_BB4_37
# %bb.47:                               #   in Loop: Header=BB4_38 Depth=2
	addi 4, 3, 16
	addi 5, 3, 17
	ld 21, 448(1)                           # 8-byte Folded Reload
	addi 6, 3, 18
	addi 7, 3, 19
	addi 8, 3, 20
	addi 9, 3, 21
	addi 10, 3, 22
	addi 11, 3, 23
	cmpld	4, 22
	iselgt	4, 3, 4
	cmpld	5, 22
	srd 12, 4, 20
	iselgt	5, 3, 5
	cmpld	6, 22
	maddld 12, 18, 12, 21
	ld 21, 448(1)                           # 8-byte Folded Reload
	srd 0, 5, 20
	iselgt	6, 3, 6
	cmpld	7, 22
	srd 29, 6, 20
	iselgt	7, 3, 7
	cmpld	8, 22
	maddld 0, 18, 0, 21
	ld 21, 448(1)                           # 8-byte Folded Reload
	srd 28, 7, 20
	iselgt	8, 3, 8
	cmpld	9, 22
	std 12, 1664(1)
	srd 27, 8, 20
	iselgt	9, 3, 9
	cmpld	10, 22
	maddld 29, 18, 29, 21
	ld 21, 448(1)                           # 8-byte Folded Reload
	srd 26, 9, 20
	iselgt	10, 3, 10
	cmpld	11, 22
	std 0, 1672(1)
	srd 25, 10, 20
	iselgt	11, 3, 11
	maddld 28, 18, 28, 21
	ld 21, 448(1)                           # 8-byte Folded Reload
	srd 24, 11, 20
	std 29, 1680(1)
	maddld 27, 18, 27, 21
	ld 21, 448(1)                           # 8-byte Folded Reload
	std 28, 1688(1)
	maddld 26, 18, 26, 21
	ld 21, 448(1)                           # 8-byte Folded Reload
	std 27, 1696(1)
	ld 27, 448(1)                           # 8-byte Folded Reload
	maddld 25, 18, 25, 21
	ld 21, 448(1)                           # 8-byte Folded Reload
	std 26, 1704(1)
	maddld 24, 18, 24, 21
	ld 21, 328(1)                           # 8-byte Folded Reload
	std 25, 1712(1)
	and 4, 4, 21
	and 5, 5, 21
	and 6, 6, 21
	and 7, 7, 21
	and 8, 8, 21
	and 9, 9, 21
	and 10, 10, 21
	and 11, 11, 21
	std 24, 1720(1)
	std 4, 1600(1)
	std 5, 1608(1)
	std 6, 1616(1)
	std 7, 1624(1)
	std 8, 1632(1)
	std 9, 1640(1)
	std 10, 1648(1)
	std 11, 1656(1)
	b .Lp9_int8_BB4_37
.Lp9_int8_BB4_48:                               #   in Loop: Header=BB4_49 Depth=2
	ld 3, 344(1)                            # 8-byte Folded Reload
	ld 5, 360(1)                            # 8-byte Folded Reload
	ld 4, 352(1)                            # 8-byte Folded Reload
	ld 27, 448(1)                           # 8-byte Folded Reload
	add 5, 5, 3
	ld 3, 192(1)                            # 8-byte Folded Reload
	ld 20, 200(1)                           # 8-byte Folded Reload
	ld 21, 328(1)                           # 8-byte Folded Reload
	ld 18, 168(1)                           # 8-byte Folded Reload
	cmpld	5, 22
	std 5, 360(1)                           # 8-byte Folded Spill
	add 31, 31, 3
	ld 3, 152(1)                            # 8-byte Folded Reload
	add 4, 4, 3
	std 4, 352(1)                           # 8-byte Folded Spill
	bgt	0, .Lp9_int8_BB4_34
.Lp9_int8_BB4_49:                               #   Parent Loop BB4_35 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB4_52 Depth 3
                                        #       Child Loop BB4_55 Depth 3
                                        #       Child Loop BB4_63 Depth 3
                                        #         Child Loop BB4_85 Depth 4
                                        #           Child Loop BB4_86 Depth 5
                                        #         Child Loop BB4_102 Depth 4
                                        #           Child Loop BB4_103 Depth 5
                                        #         Child Loop BB4_115 Depth 4
                                        #           Child Loop BB4_116 Depth 5
                                        #         Child Loop BB4_70 Depth 4
                                        #           Child Loop BB4_71 Depth 5
                                        #         Child Loop BB4_97 Depth 4
                                        #           Child Loop BB4_98 Depth 5
                                        #         Child Loop BB4_110 Depth 4
                                        #           Child Loop BB4_111 Depth 5
                                        #         Child Loop BB4_79 Depth 4
                                        #         Child Loop BB4_94 Depth 4
                                        #         Child Loop BB4_107 Depth 4
                                        #         Child Loop BB4_120 Depth 4
                                        #         Child Loop BB4_147 Depth 4
                                        #           Child Loop BB4_150 Depth 5
	lwz 3, 340(1)
	std 31, 480(1)                          # 8-byte Folded Spill
	mtocrf 128, 3
	bc 4, 0, .Lp9_int8_BB4_56
# %bb.50:                               #   in Loop: Header=BB4_49 Depth=2
	lwz 4, 184(1)
	ld 9, 160(1)                            # 8-byte Folded Reload
	ld 10, 360(1)                           # 8-byte Folded Reload
	li 3, 0
	mtocrf 128, 4
	bc 12, 0, .Lp9_int8_BB4_53
# %bb.51:                               #   in Loop: Header=BB4_49 Depth=2
	ld 4, 128(1)                            # 8-byte Folded Reload
	ld 5, 120(1)                            # 8-byte Folded Reload
	.p2align	5
.Lp9_int8_BB4_52:                               #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	add 6, 10, 3
	addi 3, 3, 4
	cmpld	6, 22
	iselgt	7, 10, 6
	and 8, 7, 21
	srd 7, 7, 20
	maddld 7, 18, 7, 27
	stdu 8, 32(5)
	addi 8, 6, 1
	cmpld	8, 22
	stdu 7, 32(4)
	iselgt	7, 10, 8
	srd 8, 7, 20
	and 7, 7, 21
	std 7, 8(5)
	addi 7, 6, 2
	maddld 8, 18, 8, 27
	addi 6, 6, 3
	cmpld	7, 22
	iselgt	7, 10, 7
	cmpld	6, 22
	std 8, 8(4)
	srd 8, 7, 20
	and 7, 7, 21
	iselgt	6, 10, 6
	cmpld	9, 3
	std 7, 16(5)
	srd 7, 6, 20
	maddld 8, 18, 8, 27
	and 6, 6, 21
	maddld 7, 18, 7, 27
	std 6, 24(5)
	std 8, 16(4)
	std 7, 24(4)
	bne	0, .Lp9_int8_BB4_52
.Lp9_int8_BB4_53:                               #   in Loop: Header=BB4_49 Depth=2
	lwz 4, 188(1)
	mtocrf 128, 4
	bc 12, 2, .Lp9_int8_BB4_57
# %bb.54:                               #   in Loop: Header=BB4_49 Depth=2
	ld 4, 112(1)                            # 8-byte Folded Reload
	ld 6, 104(1)                            # 8-byte Folded Reload
	sldi 5, 3, 3
	ld 8, 360(1)                            # 8-byte Folded Reload
	add 4, 4, 5
	add 5, 6, 5
	ld 6, 144(1)                            # 8-byte Folded Reload
	add 3, 3, 8
	mtctr 6
	.p2align	5
.Lp9_int8_BB4_55:                               #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	cmpld	3, 22
	iselgt	6, 8, 3
	addi 3, 3, 1
	and 7, 6, 21
	srd 6, 6, 20
	maddld 6, 18, 6, 27
	stdu 7, 8(5)
	stdu 6, 8(4)
	bdnz .Lp9_int8_BB4_55
	b .Lp9_int8_BB4_57
.Lp9_int8_BB4_56:                               #   in Loop: Header=BB4_49 Depth=2
	mr	19, 18
	ld 18, 360(1)                           # 8-byte Folded Reload
	mr	24, 27
	addi 3, 18, 1
	addi 4, 18, 2
	addi 5, 18, 3
	addi 6, 18, 4
	addi 7, 18, 5
	addi 8, 18, 6
	addi 9, 18, 7
	srd 10, 18, 20
	cmpld	3, 22
	maddld 10, 19, 10, 24
	iselgt	3, 18, 3
	cmpld	4, 22
	iselgt	4, 18, 4
	cmpld	5, 22
	srd 11, 3, 20
	and 3, 3, 21
	std 10, 1536(1)
	and 10, 18, 21
	iselgt	5, 18, 5
	cmpld	6, 22
	srd 12, 4, 20
	maddld 11, 19, 11, 24
	and 4, 4, 21
	std 3, 1480(1)
	std 10, 1472(1)
	iselgt	6, 18, 6
	cmpld	7, 22
	srd 0, 5, 20
	maddld 12, 19, 12, 24
	and 5, 5, 21
	std 4, 1488(1)
	iselgt	7, 18, 7
	cmpld	8, 22
	srd 29, 6, 20
	maddld 0, 19, 0, 24
	and 6, 6, 21
	std 5, 1496(1)
	std 11, 1544(1)
	iselgt	8, 18, 8
	cmpld	9, 22
	srd 28, 7, 20
	maddld 29, 19, 29, 24
	and 7, 7, 21
	std 6, 1504(1)
	std 12, 1552(1)
	iselgt	9, 18, 9
	srd 27, 8, 20
	maddld 28, 19, 28, 24
	and 8, 8, 21
	std 7, 1512(1)
	std 0, 1560(1)
	srd 26, 9, 20
	maddld 27, 19, 27, 24
	and 9, 9, 21
	std 8, 1520(1)
	std 29, 1568(1)
	maddld 26, 19, 26, 24
	std 9, 1528(1)
	std 28, 1576(1)
	std 27, 1584(1)
	std 26, 1592(1)
	lwz 3, 180(1)
	mtocrf 128, 3
	bc 4, 2, .Lp9_int8_BB4_58
.Lp9_int8_BB4_57:                               #   in Loop: Header=BB4_49 Depth=2
	ld 3, 248(1)                            # 8-byte Folded Reload
	ld 18, 352(1)                           # 8-byte Folded Reload
	ld 29, 256(1)                           # 8-byte Folded Reload
	ld 28, 264(1)                           # 8-byte Folded Reload
	ld 26, 272(1)                           # 8-byte Folded Reload
	ld 24, 280(1)                           # 8-byte Folded Reload
	ld 0, 288(1)                            # 8-byte Folded Reload
	ld 11, 304(1)                           # 8-byte Folded Reload
	ld 10, 312(1)                           # 8-byte Folded Reload
	ld 12, 232(1)                           # 8-byte Folded Reload
	ld 5, 240(1)                            # 8-byte Folded Reload
	ld 19, 320(1)                           # 8-byte Folded Reload
	ld 27, 224(1)                           # 8-byte Folded Reload
	ld 20, 216(1)                           # 8-byte Folded Reload
	ld 21, 208(1)                           # 8-byte Folded Reload
	std 3, 944(1)                           # 8-byte Folded Spill
	ld 3, 296(1)                            # 8-byte Folded Reload
	std 3, 952(1)                           # 8-byte Folded Spill
	b .Lp9_int8_BB4_63
.Lp9_int8_BB4_58:                               #   in Loop: Header=BB4_49 Depth=2
	addi 3, 18, 8
	mr	27, 18
	addi 4, 27, 9
	cmpld	3, 22
	addi 5, 27, 10
	addi 6, 27, 11
	addi 7, 27, 12
	addi 8, 27, 13
	addi 9, 27, 14
	addi 10, 27, 15
	iselgt	18, 18, 3
	cmpld	4, 22
	mr	3, 27
	iselgt	4, 27, 4
	cmpld	5, 22
	srd 11, 18, 20
	iselgt	5, 27, 5
	cmpld	6, 22
	srd 12, 4, 20
	maddld 11, 19, 11, 24
	and 4, 4, 21
	iselgt	6, 27, 6
	cmpld	7, 22
	srd 0, 5, 20
	maddld 12, 19, 12, 24
	and 5, 5, 21
	std 4, 1544(1)
	iselgt	7, 27, 7
	cmpld	8, 22
	srd 29, 6, 20
	maddld 0, 19, 0, 24
	and 6, 6, 21
	std 5, 1552(1)
	std 11, 1600(1)
	and 11, 18, 21
	iselgt	8, 27, 8
	cmpld	9, 22
	srd 28, 7, 20
	maddld 29, 19, 29, 24
	and 7, 7, 21
	std 6, 1560(1)
	std 12, 1608(1)
	std 11, 1536(1)
	iselgt	9, 27, 9
	cmpld	10, 22
	maddld 28, 19, 28, 24
	std 7, 1568(1)
	std 0, 1616(1)
	iselgt	10, 27, 10
	srd 27, 8, 20
	srd 26, 9, 20
	and 8, 8, 21
	and 9, 9, 21
	std 29, 1624(1)
	srd 25, 10, 20
	maddld 27, 19, 27, 24
	maddld 26, 19, 26, 24
	and 10, 10, 21
	std 8, 1576(1)
	std 9, 1584(1)
	std 28, 1632(1)
	maddld 25, 19, 25, 24
	std 10, 1592(1)
	std 27, 1640(1)
	std 26, 1648(1)
	std 25, 1656(1)
	lwz 4, 140(1)
	mtocrf 128, 4
	bc 4, 2, .Lp9_int8_BB4_60
# %bb.59:                               #   in Loop: Header=BB4_49 Depth=2
	ld 3, 248(1)                            # 8-byte Folded Reload
	ld 18, 352(1)                           # 8-byte Folded Reload
	ld 29, 256(1)                           # 8-byte Folded Reload
	ld 28, 264(1)                           # 8-byte Folded Reload
	ld 26, 272(1)                           # 8-byte Folded Reload
	ld 24, 280(1)                           # 8-byte Folded Reload
	ld 0, 288(1)                            # 8-byte Folded Reload
	ld 11, 304(1)                           # 8-byte Folded Reload
	ld 10, 312(1)                           # 8-byte Folded Reload
	ld 12, 232(1)                           # 8-byte Folded Reload
	ld 5, 240(1)                            # 8-byte Folded Reload
	ld 19, 320(1)                           # 8-byte Folded Reload
	ld 27, 224(1)                           # 8-byte Folded Reload
	ld 20, 216(1)                           # 8-byte Folded Reload
	std 3, 944(1)                           # 8-byte Folded Spill
	ld 3, 296(1)                            # 8-byte Folded Reload
	std 3, 952(1)                           # 8-byte Folded Spill
	b .Lp9_int8_BB4_61
.Lp9_int8_BB4_60:                               #   in Loop: Header=BB4_49 Depth=2
	addi 4, 3, 16
	addi 5, 3, 18
	addi 6, 3, 19
	addi 7, 3, 20
	addi 8, 3, 21
	addi 9, 3, 22
	addi 10, 3, 23
	cmpld	4, 22
	iselgt	18, 3, 4
	addi 4, 3, 17
	cmpld	4, 22
	srd 11, 18, 20
	iselgt	4, 3, 4
	cmpld	5, 22
	maddld 11, 19, 11, 24
	iselgt	5, 3, 5
	cmpld	6, 22
	srd 12, 4, 20
	and 4, 4, 21
	iselgt	6, 3, 6
	cmpld	7, 22
	srd 0, 5, 20
	maddld 12, 19, 12, 24
	and 5, 5, 21
	std 4, 1608(1)
	std 11, 1664(1)
	ld 11, 304(1)                           # 8-byte Folded Reload
	iselgt	7, 3, 7
	cmpld	8, 22
	srd 29, 6, 20
	maddld 0, 19, 0, 24
	std 5, 1616(1)
	ld 5, 240(1)                            # 8-byte Folded Reload
	and 6, 6, 21
	iselgt	8, 3, 8
	cmpld	9, 22
	srd 28, 7, 20
	maddld 29, 19, 29, 24
	and 7, 7, 21
	std 6, 1624(1)
	std 12, 1672(1)
	ld 12, 232(1)                           # 8-byte Folded Reload
	iselgt	9, 3, 9
	cmpld	10, 22
	srd 27, 8, 20
	maddld 28, 19, 28, 24
	and 8, 8, 21
	std 7, 1632(1)
	std 0, 1680(1)
	ld 0, 288(1)                            # 8-byte Folded Reload
	iselgt	10, 3, 10
	and 3, 18, 21
	srd 26, 9, 20
	maddld 27, 19, 27, 24
	ld 18, 352(1)                           # 8-byte Folded Reload
	and 9, 9, 21
	std 8, 1640(1)
	std 29, 1688(1)
	ld 29, 256(1)                           # 8-byte Folded Reload
	std 3, 1600(1)
	ld 3, 248(1)                            # 8-byte Folded Reload
	maddld 26, 19, 26, 24
	srd 25, 10, 20
	and 10, 10, 21
	ld 20, 216(1)                           # 8-byte Folded Reload
	std 9, 1648(1)
	std 28, 1696(1)
	ld 28, 264(1)                           # 8-byte Folded Reload
	maddld 25, 19, 25, 24
	ld 24, 280(1)                           # 8-byte Folded Reload
	ld 19, 320(1)                           # 8-byte Folded Reload
	std 10, 1656(1)
	std 27, 1704(1)
	ld 27, 224(1)                           # 8-byte Folded Reload
	std 3, 944(1)                           # 8-byte Folded Spill
	ld 3, 296(1)                            # 8-byte Folded Reload
	std 26, 1712(1)
	ld 26, 272(1)                           # 8-byte Folded Reload
	std 25, 1720(1)
	std 3, 952(1)                           # 8-byte Folded Spill
	ld 3, 312(1)                            # 8-byte Folded Reload
	mr	10, 3
.Lp9_int8_BB4_61:                               #   in Loop: Header=BB4_49 Depth=2
	ld 21, 208(1)                           # 8-byte Folded Reload
	ld 25, 488(1)                           # 8-byte Folded Reload
	b .Lp9_int8_BB4_63
	.p2align	4
.Lp9_int8_BB4_62:                               #   in Loop: Header=BB4_63 Depth=3
	ld 3, 504(1)                            # 8-byte Folded Reload
	ld 4, 952(1)                            # 8-byte Folded Reload
	ld 5, 832(1)                            # 8-byte Folded Reload
	addi 21, 21, 4
	add 4, 4, 3
	add 20, 20, 3
	add 27, 27, 3
	add 19, 18, 3
	add 5, 5, 3
	add 12, 12, 3
	add 10, 10, 3
	add 11, 11, 3
	add 0, 0, 3
	add 24, 24, 3
	add 26, 26, 3
	add 28, 28, 3
	add 29, 29, 3
	ld 18, 840(1)                           # 8-byte Folded Reload
	std 4, 952(1)                           # 8-byte Folded Spill
	ld 4, 944(1)                            # 8-byte Folded Reload
	add 4, 4, 3
	ld 3, 496(1)                            # 8-byte Folded Reload
	std 4, 944(1)                           # 8-byte Folded Spill
	add 18, 18, 3
	ld 3, 528(1)                            # 8-byte Folded Reload
	cmpld	21, 3
	bge 0, .Lp9_int8_BB4_48
.Lp9_int8_BB4_63:                               #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        # =>    This Loop Header: Depth=3
                                        #         Child Loop BB4_85 Depth 4
                                        #           Child Loop BB4_86 Depth 5
                                        #         Child Loop BB4_102 Depth 4
                                        #           Child Loop BB4_103 Depth 5
                                        #         Child Loop BB4_115 Depth 4
                                        #           Child Loop BB4_116 Depth 5
                                        #         Child Loop BB4_70 Depth 4
                                        #           Child Loop BB4_71 Depth 5
                                        #         Child Loop BB4_97 Depth 4
                                        #           Child Loop BB4_98 Depth 5
                                        #         Child Loop BB4_110 Depth 4
                                        #           Child Loop BB4_111 Depth 5
                                        #         Child Loop BB4_79 Depth 4
                                        #         Child Loop BB4_94 Depth 4
                                        #         Child Loop BB4_107 Depth 4
                                        #         Child Loop BB4_120 Depth 4
                                        #         Child Loop BB4_147 Depth 4
                                        #           Child Loop BB4_150 Depth 5
	rldicl 8, 21, 62, 2
	std 11, 936(1)                          # 8-byte Folded Spill
	std 18, 840(1)                          # 8-byte Folded Spill
	std 10, 824(1)                          # 8-byte Folded Spill
	std 12, 816(1)                          # 8-byte Folded Spill
	std 5, 832(1)                           # 8-byte Folded Spill
	ble 4, .Lp9_int8_BB4_73
# %bb.64:                               #   in Loop: Header=BB4_63 Depth=3
	lwz 3, 460(1)                           # 4-byte Folded Reload
	std 14, 552(1)                          # 8-byte Folded Spill
	std 29, 800(1)                          # 8-byte Folded Spill
	cmpwi	3, 457
	std 28, 792(1)                          # 8-byte Folded Spill
	std 26, 808(1)                          # 8-byte Folded Spill
	std 24, 784(1)                          # 8-byte Folded Spill
	std 0, 776(1)                           # 8-byte Folded Spill
	std 19, 544(1)                          # 8-byte Folded Spill
	std 20, 768(1)                          # 8-byte Folded Spill
	bge 0, .Lp9_int8_BB4_80
# %bb.65:                               #   in Loop: Header=BB4_63 Depth=3
	cmplwi	3, 388
	std 21, 536(1)                          # 8-byte Folded Spill
	beq	0, .Lp9_int8_BB4_108
# %bb.66:                               #   in Loop: Header=BB4_63 Depth=3
	cmplwi	3, 393
	beq	0, .Lp9_int8_BB4_95
# %bb.67:                               #   in Loop: Header=BB4_63 Depth=3
	cmplwi	3, 452
	bne	0, .Lp9_int8_BB4_118
# %bb.68:                               #   in Loop: Header=BB4_63 Depth=3
	ld 6, 1624(1)
	cmpldi	6, 0
	beq	0, .Lp9_int8_BB4_133
# %bb.69:                               #   in Loop: Header=BB4_63 Depth=3
	ld 4, 1472(1)
	ld 7, 1480(1)
	ld 25, 1536(1)
	ld 10, 1600(1)
	mr	0, 8
	xxlxor 0, 0, 0
	li 24, 0
	ld 11, 1544(1)
	xxlxor 1, 1, 1
	xxlxor 2, 2, 2
	mulld 0, 6, 0
	xxlxor 3, 3, 3
	xxlxor 5, 5, 5
	mulli 26, 4, 288
	mulli 5, 7, 288
	sldi 4, 4, 10
	sldi 12, 7, 10
	add 7, 10, 28
	add 9, 10, 29
	ld 28, 1608(1)
	mr	3, 25
	add 8, 10, 27
	add 10, 10, 20
	xxlxor 6, 6, 6
	add 4, 25, 4
	add 29, 11, 12
	xxlxor 7, 7, 7
	addi 12, 4, 48
	addi 29, 29, 48
	xxlxor 4, 4, 4
	add 4, 11, 5
	add 25, 25, 26
	std 5, 928(1)                           # 8-byte Folded Spill
	mr	5, 26
	addi 26, 4, 4128
	addi 25, 25, 4128
	.p2align	4
.Lp9_int8_BB4_70:                               #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB4_71 Depth 5
	mulli 21, 24, 5248
	ld 4, 928(1)                            # 8-byte Folded Reload
	li 16, 0
	li 15, 0
	add 20, 3, 21
	add 21, 11, 21
	add 17, 20, 5
	add 20, 21, 4
	li 4, 16
	lxv 34, 4112(17)
	vnegw 7, 2
	lxv 34, 4112(20)
	mtctr 4
	vmr	0, 7
	vmr	4, 7
	vnegw 1, 2
	vmr	2, 7
	vmr	6, 1
	vmr	5, 1
	vmr	3, 1
	.p2align	5
.Lp9_int8_BB4_71:                               #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        #         Parent Loop BB4_70 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	add 19, 9, 16
	add 18, 7, 16
	add 14, 12, 16
	lxvx 40, 8, 16
	lxvx 41, 10, 16
	add 31, 29, 16
	lxv 42, -48(19)
	lxv 43, -48(18)
	lxv 44, -48(14)
	lxv 48, -48(31)
	add 21, 8, 16
	add 4, 10, 16
	lxv 49, 16(4)
	lxv 50, -32(19)
	lxv 51, -32(18)
	lxv 63, -32(14)
	vmsummbm 13, 12, 8, 28
	vmsummbm 14, 12, 9, 28
	vmsummbm 15, 12, 10, 28
	vmsummbm 12, 12, 11, 28
	vmsummbm 8, 16, 8, 28
	vmsummbm 9, 16, 9, 28
	vmsummbm 10, 16, 10, 28
	vmsummbm 11, 16, 11, 28
	lxv 48, 16(21)
	vmsummbm 13, 31, 16, 13
	vmsummbm 14, 31, 17, 14
	vmsummbm 15, 31, 18, 15
	vmsummbm 12, 31, 19, 12
	lxv 63, -32(31)
	vmsummbm 8, 31, 16, 8
	vmsummbm 9, 31, 17, 9
	lxv 48, 32(21)
	vmsummbm 10, 31, 18, 10
	vmsummbm 11, 31, 19, 11
	lxv 49, 32(4)
	lxv 50, -16(19)
	lxv 51, -16(18)
	lxv 63, -16(14)
	vmsummbm 13, 31, 16, 13
	vmsummbm 14, 31, 17, 14
	vmsummbm 15, 31, 18, 15
	vmsummbm 12, 31, 19, 12
	lxv 63, -16(31)
	vmsummbm 8, 31, 16, 8
	vmsummbm 9, 31, 17, 9
	vmsummbm 10, 31, 18, 10
	lxv 48, 48(21)
	vmsummbm 11, 31, 19, 11
	lxvx 49, 9, 16
	lxvx 50, 7, 16
	lxvx 51, 12, 16
	lxv 63, 48(4)
	vmsummbm 13, 19, 16, 13
	vmsummbm 14, 19, 31, 14
	vmsummbm 15, 19, 17, 15
	vmsummbm 12, 19, 18, 12
	lxvx 51, 29, 16
	addi 16, 16, 64
	vmsummbm 8, 19, 16, 8
	lxvx 48, 25, 15
	vmsummbm 9, 19, 31, 9
	vmsummbm 10, 19, 17, 10
	vmsummbm 11, 19, 18, 11
	vmuluwm 13, 13, 16
	vmuluwm 12, 12, 16
	vadduwm 7, 13, 7
	vmuluwm 13, 14, 16
	vadduwm 2, 12, 2
	vadduwm 0, 13, 0
	vmuluwm 13, 15, 16
	vadduwm 4, 13, 4
	lxvx 45, 26, 15
	addi 15, 15, 16
	vmuluwm 8, 8, 13
	vadduwm 1, 8, 1
	vmuluwm 8, 9, 13
	vmuluwm 9, 10, 13
	vadduwm 6, 8, 6
	vmuluwm 8, 11, 13
	vadduwm 5, 9, 5
	vadduwm 3, 8, 3
	bdnz .Lp9_int8_BB4_71
# %bb.72:                               #   in Loop: Header=BB4_70 Depth=4
	add 4, 24, 0
	lxv 9, 4096(17)
	vcfsx 7, 7, 0
	vcfsx 1, 1, 0
	addi 24, 24, 1
	addi 26, 26, 5248
	vcfsx 0, 0, 0
	addi 25, 25, 5248
	addi 12, 12, 5248
	addi 29, 29, 5248
	addi 7, 7, 1024
	addi 8, 8, 1024
	addi 9, 9, 1024
	sldi 4, 4, 4
	lxvwsx 8, 28, 4
	add 21, 28, 4
	li 4, 4
	vcfsx 4, 4, 0
	addi 10, 10, 1024
	cmpld	24, 6
	vcfsx 2, 2, 0
	xvmulsp 10, 8, 9
	xvmaddasp 0, 39, 10
	lxv 10, 4096(20)
	xvmulsp 8, 8, 10
	xvmaddasp 5, 33, 8
	lxvwsx 8, 21, 4
	li 4, 8
	xvmulsp 11, 8, 9
	xvmulsp 8, 8, 10
	xvmaddasp 1, 32, 11
	vcfsx 0, 6, 0
	xvmaddasp 6, 32, 8
	lxvwsx 8, 21, 4
	li 4, 12
	xvmulsp 11, 8, 9
	xvmulsp 8, 8, 10
	xvmaddasp 2, 36, 11
	vcfsx 4, 5, 0
	xvmaddasp 7, 36, 8
	lxvwsx 8, 21, 4
	xvmulsp 9, 8, 9
	xvmulsp 8, 8, 10
	xvmaddasp 3, 34, 9
	vcfsx 2, 3, 0
	xvmaddasp 4, 34, 8
	bne	0, .Lp9_int8_BB4_70
	b .Lp9_int8_BB4_134
	.p2align	4
.Lp9_int8_BB4_73:                               #   in Loop: Header=BB4_63 Depth=3
	ble 2, .Lp9_int8_BB4_88
# %bb.74:                               #   in Loop: Header=BB4_63 Depth=3
	lwz 3, 444(1)
	std 29, 800(1)                          # 8-byte Folded Spill
	std 28, 792(1)                          # 8-byte Folded Spill
	std 26, 808(1)                          # 8-byte Folded Spill
	std 24, 784(1)                          # 8-byte Folded Spill
	std 0, 776(1)                           # 8-byte Folded Spill
	std 20, 768(1)                          # 8-byte Folded Spill
	mtocrf 128, 3
	beq	0, .Lp9_int8_BB4_105
# %bb.75:                               #   in Loop: Header=BB4_63 Depth=3
	lwz 3, 436(1)
	mtocrf 128, 3
	beq	0, .Lp9_int8_BB4_92
# %bb.76:                               #   in Loop: Header=BB4_63 Depth=3
	lwz 3, 460(1)                           # 4-byte Folded Reload
	cmplwi	3, 273
	bne	0, .Lp9_int8_BB4_118
# %bb.77:                               #   in Loop: Header=BB4_63 Depth=3
	ld 24, 1624(1)
	std 21, 536(1)                          # 8-byte Folded Spill
	std 19, 544(1)                          # 8-byte Folded Spill
	cmpldi	24, 0
	std 14, 552(1)                          # 8-byte Folded Spill
	beq	0, .Lp9_int8_BB4_128
# %bb.78:                               #   in Loop: Header=BB4_63 Depth=3
	ld 3, 1472(1)
	ld 28, 1536(1)
	ld 0, 1504(1)
	xxlxor 0, 0, 0
	xxlxor 1, 1, 1
	sldi 4, 3, 7
	sldi 5, 3, 4
	ld 3, 1480(1)
	sldi 29, 0, 7
	xxlxor 2, 2, 2
	xxlxor 3, 3, 3
	sldi 6, 3, 7
	sldi 7, 3, 4
	ld 3, 1488(1)
	xxlxor 4, 4, 4
	xxlxor 5, 5, 5
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
	xxlxor 8, 8, 8
	xxlxor 9, 9, 9
	xxlxor 10, 10, 10
	xxlxor 11, 11, 11
	xxlxor 12, 12, 12
	xxlxor 13, 13, 13
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
	sldi 9, 3, 7
	sldi 10, 3, 4
	ld 3, 1496(1)
	sldi 11, 3, 7
	sldi 12, 3, 4
	add 3, 28, 4
	add 28, 28, 5
	ld 5, 1544(1)
	addi 19, 28, 1024
	ld 28, 1592(1)
	add 4, 5, 6
	add 26, 5, 7
	ld 6, 1552(1)
	ld 7, 1560(1)
	add 5, 6, 9
	add 9, 6, 10
	addi 10, 26, 1024
	add 6, 7, 11
	add 25, 7, 12
	ld 11, 1568(1)
	ld 12, 1512(1)
	ld 26, 1616(1)
	std 10, 928(1)                          # 8-byte Folded Spill
	addi 10, 9, 1024
	sldi 9, 0, 4
	ld 0, 1576(1)
	add 7, 11, 29
	add 9, 11, 9
	sldi 11, 12, 7
	sldi 12, 12, 4
	ld 29, 1584(1)
	add 11, 0, 11
	add 21, 0, 12
	ld 0, 1520(1)
	addi 17, 21, 1024
	li 21, 20
	sldi 12, 0, 7
	sldi 0, 0, 4
	add 20, 29, 0
	ld 0, 1528(1)
	add 12, 29, 12
	addi 16, 20, 1024
	sldi 29, 0, 7
	sldi 0, 0, 4
	add 14, 28, 0
	mulld 0, 24, 8
	add 29, 28, 29
	addi 14, 14, 1024
	sldi 28, 0, 3
	sldi 0, 0, 4
	add 18, 26, 28
	ld 28, 1608(1)
	addi 26, 25, 1024
	mr	25, 24
	addi 24, 9, 1024
	ld 9, 832(1)                            # 8-byte Folded Reload
	addi 31, 18, -8
	li 18, 16
	add 8, 28, 0
	ld 0, 1600(1)
	ld 28, 544(1)                           # 8-byte Folded Reload
	mtctr 25
	addi 20, 8, -16
	li 8, 0
	add 28, 0, 28
	add 15, 0, 9
	.p2align	5
.Lp9_int8_BB4_79:                               #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lhau 9, 8(31)
	lxv 32, -64(15)
	lxv 33, 0(3)
	lxv 39, -64(28)
	lxv 40, 0(4)
	lxv 42, 0(5)
	lxv 44, 0(6)
	lxv 46, 0(7)
	lxv 48, 0(11)
	add 25, 12, 8
	lxvx 50, 12, 8
	lxvx 59, 29, 8
	slwi 9, 9, 7
	lxv 58, 16(5)
	lha 0, 2(31)
	lxv 57, 48(28)
	lxvwsx 30, 20, 21
	neg 9, 9
	mtvsrws 36, 9
	slwi 9, 0, 7
	neg 9, 9
	vmsummbm 6, 0, 1, 4
	vmsummbm 9, 0, 8, 4
	vmsummbm 11, 0, 10, 4
	vmsummbm 13, 0, 12, 4
	vmsummbm 15, 0, 14, 4
	vmsummbm 17, 0, 16, 4
	vmsummbm 19, 0, 18, 4
	vmsummbm 4, 0, 27, 4
	lxv 32, -48(15)
	mtvsrws 37, 9
	add 9, 29, 8
	vmsummbm 1, 7, 1, 5
	vmsummbm 8, 7, 8, 5
	vmsummbm 10, 7, 10, 5
	vmsummbm 12, 7, 12, 5
	vmsummbm 14, 7, 14, 5
	vmsummbm 16, 7, 16, 5
	vmsummbm 18, 7, 18, 5
	vmsummbm 5, 7, 27, 5
	lxv 39, -48(28)
	lxv 59, 16(3)
	vmsummbm 11, 0, 26, 11
	vmsummbm 6, 0, 27, 6
	vmsummbm 1, 7, 27, 1
	lxv 59, 16(4)
	vmsummbm 10, 7, 26, 10
	lxv 58, 16(11)
	vmsummbm 9, 0, 27, 9
	vmsummbm 8, 7, 27, 8
	lxv 59, 16(6)
	vmsummbm 17, 0, 26, 17
	vmsummbm 16, 7, 26, 16
	lxv 58, 16(9)
	vmsummbm 13, 0, 27, 13
	vmsummbm 12, 7, 27, 12
	lxv 59, 16(7)
	vmsummbm 4, 0, 26, 4
	vmsummbm 5, 7, 26, 5
	lxv 58, 32(6)
	vmsummbm 15, 0, 27, 15
	vmsummbm 14, 7, 27, 14
	lxv 59, 16(25)
	vmsummbm 19, 0, 27, 19
	vmsummbm 18, 7, 27, 18
	lxv 32, -32(15)
	lxv 39, 32(3)
	lxv 59, -32(28)
	vmsummbm 6, 0, 7, 6
	vmsummbm 1, 27, 7, 1
	lxv 39, 32(4)
	vmsummbm 13, 0, 26, 13
	vmsummbm 9, 0, 7, 9
	vmsummbm 7, 27, 7, 8
	lxv 40, 32(5)
	vmsummbm 11, 0, 8, 11
	vmsummbm 8, 27, 8, 10
	vmsummbm 10, 27, 26, 12
	lxv 44, 32(7)
	lxv 58, 32(25)
	vmsummbm 15, 0, 12, 15
	vmsummbm 12, 27, 12, 14
	lxv 46, 32(11)
	vmsummbm 19, 0, 26, 19
	vmsummbm 17, 0, 14, 17
	vmsummbm 14, 27, 14, 16
	vmsummbm 16, 27, 26, 18
	lxv 50, 32(9)
	lxv 58, 48(5)
	vmsummbm 4, 0, 18, 4
	vmsummbm 5, 27, 18, 5
	lxv 32, -16(15)
	lxv 50, -16(28)
	lxv 59, 48(3)
	vmsummbm 6, 0, 27, 6
	vmsummbm 1, 18, 27, 1
	lxv 59, 48(4)
	vmsummbm 11, 0, 26, 11
	vmsummbm 8, 18, 26, 8
	lxv 58, 48(11)
	vmsummbm 9, 0, 27, 9
	vmsummbm 7, 18, 27, 7
	lxv 59, 48(6)
	vmsummbm 17, 0, 26, 17
	vmsummbm 14, 18, 26, 14
	lxv 58, 48(9)
	vmsummbm 13, 0, 27, 13
	vmsummbm 10, 18, 27, 10
	lxv 59, 48(7)
	vmsummbm 4, 0, 26, 4
	vmsummbm 5, 18, 26, 5
	lxv 58, 64(6)
	vmsummbm 15, 0, 27, 15
	vmsummbm 12, 18, 27, 12
	lxv 59, 48(25)
	vmsummbm 19, 0, 27, 19
	lxv 32, 0(15)
	vmsummbm 16, 18, 27, 16
	lxv 50, 64(3)
	lxv 59, 0(28)
	vmsummbm 6, 0, 18, 6
	vmsummbm 1, 27, 18, 1
	lxv 50, 64(4)
	vmsummbm 13, 0, 26, 13
	vmsummbm 10, 27, 26, 10
	lxv 58, 64(25)
	vmsummbm 9, 0, 18, 9
	vmsummbm 7, 27, 18, 7
	lxv 50, 64(5)
	vmsummbm 19, 0, 26, 19
	vmsummbm 16, 27, 26, 16
	lxv 58, 80(5)
	vmsummbm 11, 0, 18, 11
	vmsummbm 8, 27, 18, 8
	lxv 50, 64(7)
	vmsummbm 15, 0, 18, 15
	vmsummbm 12, 27, 18, 12
	lxv 50, 64(11)
	vmsummbm 17, 0, 18, 17
	vmsummbm 14, 27, 18, 14
	lxv 50, 64(9)
	vmsummbm 4, 0, 18, 4
	vmsummbm 5, 27, 18, 5
	lxv 32, 16(15)
	lxv 50, 16(28)
	lxv 59, 80(3)
	vmsummbm 6, 0, 27, 6
	vmsummbm 1, 18, 27, 1
	lxv 59, 80(4)
	vmsummbm 11, 0, 26, 11
	vmsummbm 8, 18, 26, 8
	lxv 58, 80(11)
	vmsummbm 9, 0, 27, 9
	vmsummbm 7, 18, 27, 7
	lxv 59, 80(6)
	vmsummbm 17, 0, 26, 17
	vmsummbm 14, 18, 26, 14
	lxv 58, 80(9)
	vmsummbm 13, 0, 27, 13
	vmsummbm 10, 18, 27, 10
	lxv 59, 80(7)
	vmsummbm 4, 0, 26, 4
	vmsummbm 5, 18, 26, 5
	lxv 58, 96(6)
	vmsummbm 15, 0, 27, 15
	vmsummbm 12, 18, 27, 12
	lxv 59, 80(25)
	vmsummbm 19, 0, 27, 19
	lxv 32, 32(15)
	vmsummbm 16, 18, 27, 16
	lxv 50, 96(3)
	lxv 59, 32(28)
	addi 28, 28, 128
	vmsummbm 6, 0, 18, 6
	vmsummbm 1, 27, 18, 1
	lxv 50, 96(4)
	vmsummbm 13, 0, 26, 13
	vmsummbm 10, 27, 26, 10
	lxv 58, 96(25)
	vmsummbm 9, 0, 18, 9
	vmsummbm 7, 27, 18, 7
	lxv 50, 96(5)
	vmsummbm 19, 0, 26, 19
	vmsummbm 16, 27, 26, 16
	lxv 58, 48(15)
	addi 15, 15, 128
	vmsummbm 11, 0, 18, 11
	vmsummbm 18, 27, 18, 8
	lxv 40, 96(7)
	vmsummbm 15, 0, 8, 15
	vmsummbm 12, 27, 8, 12
	lxv 40, 96(11)
	vmsummbm 17, 0, 8, 17
	vmsummbm 14, 27, 8, 14
	lxv 40, 96(9)
	vmsummbm 27, 27, 8, 5
	lxv 37, 112(3)
	vmsummbm 4, 0, 8, 4
	lxv 32, 112(5)
	addi 3, 3, 1152
	addi 5, 5, 1152
	vmsummbm 24, 26, 5, 6
	vmsummbm 8, 25, 5, 1
	lxv 37, 112(4)
	vmsummbm 11, 26, 0, 11
	vmsummbm 6, 25, 0, 18
	lxv 32, 112(11)
	lxvx 50, 19, 8
	addi 4, 4, 1152
	addi 11, 11, 1152
	vmsummbm 23, 26, 5, 9
	vmsummbm 9, 25, 5, 7
	lxv 37, 112(6)
	addi 6, 6, 1152
	vcfsx 11, 11, 0
	vcfsx 8, 8, 0
	vcfsx 6, 6, 0
	vmsummbm 13, 26, 5, 13
	vmsummbm 1, 25, 5, 10
	lxv 37, 112(7)
	addi 7, 7, 1152
	vcfsx 9, 9, 0
	vmsummbm 10, 26, 5, 15
	vmsummbm 15, 26, 0, 17
	vmsummbm 7, 25, 5, 12
	vmsummbm 0, 25, 0, 14
	lxv 37, 112(25)
	lxv 46, 112(9)
	lxvwsx 49, 20, 18
	ld 9, 928(1)                            # 8-byte Folded Reload
	addi 20, 20, 16
	vcfsx 13, 13, 0
	vcfsx 1, 1, 0
	vmsummbm 12, 26, 5, 19
	vmsummbm 5, 25, 5, 16
	vmsummbm 16, 26, 14, 4
	vmsummbm 4, 25, 14, 27
	vcfsx 14, 24, 0
	xvmulsp 51, 49, 50
	vcfsx 27, 23, 0
	vcfsx 10, 10, 0
	vcfsx 15, 15, 0
	vcfsx 7, 7, 0
	vcfsx 0, 0, 0
	xvmaddasp 35, 46, 51
	lxvx 46, 9, 8
	vcfsx 12, 12, 0
	vcfsx 16, 16, 0
	vcfsx 5, 5, 0
	vcfsx 4, 4, 0
	xvmulsp 51, 49, 46
	xvmulsp 46, 30, 46
	xvmaddasp 13, 59, 51
	lxvx 51, 10, 8
	xvmaddasp 12, 41, 46
	xvmulsp 31, 49, 51
	xvmaddasp 11, 43, 31
	lxvx 43, 26, 8
	xvmulsp 31, 49, 43
	xvmaddasp 9, 45, 31
	lxvx 45, 24, 8
	xvmulsp 31, 49, 45
	xvmaddasp 7, 42, 31
	lxvx 42, 17, 8
	xvmulsp 31, 49, 42
	xvmaddasp 5, 47, 31
	lxvx 47, 16, 8
	xvmulsp 31, 49, 47
	xvmaddasp 3, 44, 31
	lxvx 44, 14, 8
	addi 8, 8, 1152
	xvmulsp 49, 49, 44
	xvmaddasp 1, 48, 49
	xvmulsp 48, 30, 50
	xvmaddasp 34, 40, 48
	xvmulsp 40, 30, 51
	xvmaddasp 10, 38, 40
	xvmulsp 38, 30, 43
	xvmaddasp 8, 33, 38
	xvmulsp 33, 30, 45
	xvmulsp 38, 30, 42
	xvmaddasp 6, 39, 33
	xvmulsp 33, 30, 47
	xvmaddasp 4, 32, 38
	xvmulsp 32, 30, 44
	xvmaddasp 2, 37, 33
	xvmaddasp 0, 36, 32
	bdnz .Lp9_int8_BB4_79
	b .Lp9_int8_BB4_129
	.p2align	4
.Lp9_int8_BB4_80:                               #   in Loop: Header=BB4_63 Depth=3
	beq	0, .Lp9_int8_BB4_113
# %bb.81:                               #   in Loop: Header=BB4_63 Depth=3
	cmplwi	3, 1128
	beq	0, .Lp9_int8_BB4_100
# %bb.82:                               #   in Loop: Header=BB4_63 Depth=3
	cmplwi	3, 1137
	bne	0, .Lp9_int8_BB4_118
# %bb.83:                               #   in Loop: Header=BB4_63 Depth=3
	std 21, 536(1)                          # 8-byte Folded Spill
	ld 21, 1624(1)
	std 27, 376(1)                          # 8-byte Folded Spill
	cmpldi	21, 0
	beq	0, .Lp9_int8_BB4_135
# %bb.84:                               #   in Loop: Header=BB4_63 Depth=3
	ld 11, 1528(1)
	ld 12, 1592(1)
	ld 6, 1520(1)
	ld 7, 1512(1)
	mr	0, 8
	xxlxor 0, 0, 0
	sldi 9, 11, 10
	std 12, 712(1)                          # 8-byte Folded Spill
	sldi 4, 6, 10
	ld 5, 1504(1)
	sldi 29, 7, 10
	ld 3, 1496(1)
	sldi 28, 5, 10
	ld 10, 1488(1)
	sldi 26, 3, 10
	sldi 3, 3, 4
	ld 8, 1480(1)
	ld 19, 1536(1)
	xxlxor 1, 1, 1
	xxlxor 2, 2, 2
	add 9, 12, 9
	ld 12, 1584(1)
	std 29, 720(1)                          # 8-byte Folded Spill
	std 28, 728(1)                          # 8-byte Folded Spill
	xxlxor 3, 3, 3
	xxlxor 4, 4, 4
	std 26, 736(1)                          # 8-byte Folded Spill
	xxlxor 5, 5, 5
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
	xxlxor 8, 8, 8
	xxlxor 9, 9, 9
	xxlxor 10, 10, 10
	xxlxor 11, 11, 11
	xxlxor 12, 12, 12
	sldi 25, 10, 10
	std 3, 608(1)                           # 8-byte Folded Spill
	xxlxor 13, 13, 13
	sldi 3, 5, 4
	xxlxor 34, 34, 34
	std 25, 744(1)                          # 8-byte Folded Spill
	xxlxor 35, 35, 35
	sldi 24, 8, 10
	std 24, 752(1)                          # 8-byte Folded Spill
	sldi 8, 8, 4
	std 8, 632(1)                           # 8-byte Folded Spill
	sldi 8, 10, 4
	std 19, 648(1)                          # 8-byte Folded Spill
	std 3, 600(1)                           # 8-byte Folded Spill
	sldi 3, 7, 4
	std 8, 624(1)                           # 8-byte Folded Spill
	mulld 8, 21, 0
	std 21, 760(1)                          # 8-byte Folded Spill
	std 12, 704(1)                          # 8-byte Folded Spill
	add 12, 12, 4
	ld 4, 1576(1)
	addi 15, 12, 112
	ld 12, 1472(1)
	add 29, 4, 29
	addi 29, 29, 112
	std 4, 696(1)                           # 8-byte Folded Spill
	ld 4, 1568(1)
	add 28, 4, 28
	addi 28, 28, 112
	std 4, 688(1)                           # 8-byte Folded Spill
	ld 4, 1560(1)
	std 3, 592(1)                           # 8-byte Folded Spill
	sldi 3, 6, 4
	li 6, 0
	add 26, 4, 26
	std 4, 680(1)                           # 8-byte Folded Spill
	ld 4, 1552(1)
	std 3, 584(1)                           # 8-byte Folded Spill
	sldi 3, 11, 4
	addi 26, 26, 112
	std 3, 576(1)                           # 8-byte Folded Spill
	ld 3, 1616(1)
	std 3, 568(1)                           # 8-byte Folded Spill
	ld 3, 1608(1)
	std 8, 616(1)                           # 8-byte Folded Spill
	add 25, 4, 25
	std 4, 672(1)                           # 8-byte Folded Spill
	ld 4, 1544(1)
	std 3, 560(1)                           # 8-byte Folded Spill
	addi 25, 25, 112
	std 4, 664(1)                           # 8-byte Folded Spill
	add 24, 4, 24
	addi 4, 9, 112
	sldi 9, 12, 10
	std 9, 656(1)                           # 8-byte Folded Spill
	add 9, 19, 9
	addi 24, 24, 112
	addi 19, 9, 112
	ld 9, 1600(1)
	add 18, 9, 20
	add 27, 9, 27
	sldi 9, 12, 4
	std 9, 640(1)                           # 8-byte Folded Spill
	.p2align	4
.Lp9_int8_BB4_85:                               #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB4_86 Depth 5
	ld 3, 616(1)                            # 8-byte Folded Reload
	mulli 7, 6, 8320
	std 6, 880(1)                           # 8-byte Folded Spill
	ld 11, 704(1)                           # 8-byte Folded Reload
	ld 10, 648(1)                           # 8-byte Folded Reload
	ld 9, 664(1)                            # 8-byte Folded Reload
	ld 16, 688(1)                           # 8-byte Folded Reload
	ld 20, 696(1)                           # 8-byte Folded Reload
	mr	12, 18
	add 5, 6, 3
	ld 3, 568(1)                            # 8-byte Folded Reload
	ld 6, 736(1)                            # 8-byte Folded Reload
	std 26, 912(1)                          # 8-byte Folded Spill
	std 19, 888(1)                          # 8-byte Folded Spill
	std 25, 904(1)                          # 8-byte Folded Spill
	ld 25, 672(1)                           # 8-byte Folded Reload
	std 24, 896(1)                          # 8-byte Folded Spill
	std 28, 920(1)                          # 8-byte Folded Spill
	add 19, 11, 7
	ld 11, 712(1)                           # 8-byte Folded Reload
	add 31, 10, 7
	add 24, 9, 7
	add 28, 25, 7
	add 18, 20, 7
	add 26, 6, 7
	std 5, 872(1)                           # 8-byte Folded Spill
	sldi 5, 5, 3
	lhaux 5, 3, 5
	ld 6, 744(1)                            # 8-byte Folded Reload
	std 29, 928(1)                          # 8-byte Folded Spill
	li 29, 0
	add 21, 6, 7
	lha 3, 2(3)
	ld 6, 752(1)                            # 8-byte Folded Reload
	neg 5, 5
	neg 3, 3
	stw 5, 856(1)                           # 4-byte Folded Spill
	ld 5, 728(1)                            # 8-byte Folded Reload
	add 17, 6, 7
	ld 6, 656(1)                            # 8-byte Folded Reload
	stw 3, 848(1)                           # 4-byte Folded Spill
	ld 3, 720(1)                            # 8-byte Folded Reload
	add 5, 5, 7
	add 0, 6, 7
	add 6, 16, 7
	add 8, 3, 7
	mr	3, 27
	ld 27, 680(1)                           # 8-byte Folded Reload
	add 14, 27, 7
	add 7, 11, 7
	add 11, 20, 8
	add 20, 16, 5
	add 5, 9, 17
	ld 9, 640(1)                            # 8-byte Folded Reload
	add 16, 27, 26
	add 26, 25, 21
	mr	27, 3
	add 3, 10, 0
	add 8, 31, 9
	ld 9, 632(1)                            # 8-byte Folded Reload
	std 8, 864(1)                           # 8-byte Folded Spill
	ld 8, 624(1)                            # 8-byte Folded Reload
	add 31, 24, 9
	add 21, 28, 8
	ld 8, 608(1)                            # 8-byte Folded Reload
	add 24, 14, 8
	ld 8, 600(1)                            # 8-byte Folded Reload
	add 25, 6, 8
	ld 6, 592(1)                            # 8-byte Folded Reload
	mr	8, 27
	add 28, 18, 6
	ld 6, 584(1)                            # 8-byte Folded Reload
	mr	18, 12
	add 10, 19, 6
	ld 6, 576(1)                            # 8-byte Folded Reload
	add 9, 7, 6
	lwz 6, 856(1)                           # 4-byte Folded Reload
	mr	7, 12
	mtvsrws 42, 6
	lwz 6, 848(1)                           # 4-byte Folded Reload
	vmr	11, 10
	vmr	12, 10
	mtvsrws 32, 6
	li 6, 8
	vmr	13, 10
	vmr	14, 10
	vmr	15, 10
	mtctr 6
	vmr	16, 10
	vmr	17, 10
	vmr	4, 0
	vmr	5, 0
	vmr	1, 0
	vmr	6, 0
	vmr	7, 0
	vmr	8, 0
	vmr	9, 0
	.p2align	5
.Lp9_int8_BB4_86:                               #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        #         Parent Loop BB4_85 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	lxv 50, 0(8)
	lxv 51, 0(3)
	lxv 59, 0(7)
	add 12, 15, 29
	add 17, 4, 29
	lxv 58, -112(17)
	vmsummbm 17, 18, 19, 17
	vmsummbm 9, 27, 19, 9
	lxv 51, 0(5)
	vmsummbm 10, 18, 26, 10
	vmsummbm 0, 27, 26, 0
	lxv 58, -80(17)
	vmsummbm 16, 18, 19, 16
	vmsummbm 8, 27, 19, 8
	lxv 51, 0(26)
	vmsummbm 15, 18, 19, 15
	vmsummbm 7, 27, 19, 7
	lxv 51, 0(16)
	vmsummbm 14, 18, 19, 14
	vmsummbm 6, 27, 19, 6
	lxv 51, 0(20)
	vmsummbm 13, 18, 19, 13
	vmsummbm 1, 27, 19, 1
	lxv 51, 0(11)
	vmsummbm 12, 18, 19, 12
	vmsummbm 5, 27, 19, 5
	lxv 51, -112(12)
	vmsummbm 11, 18, 19, 11
	vmsummbm 4, 27, 19, 4
	lxv 50, 16(8)
	lxv 51, 16(3)
	lxv 59, 16(7)
	vmsummbm 17, 18, 19, 17
	vmsummbm 9, 27, 19, 9
	lxv 51, 16(5)
	vmsummbm 16, 18, 19, 16
	vmsummbm 8, 27, 19, 8
	lxv 51, 16(26)
	vmsummbm 15, 18, 19, 15
	vmsummbm 7, 27, 19, 7
	lxv 51, 16(16)
	vmsummbm 14, 18, 19, 14
	vmsummbm 6, 27, 19, 6
	lxv 51, 16(20)
	vmsummbm 13, 18, 19, 13
	vmsummbm 1, 27, 19, 1
	lxv 51, 16(11)
	vmsummbm 12, 18, 19, 12
	vmsummbm 5, 27, 19, 5
	lxv 51, -96(12)
	vmsummbm 11, 18, 19, 11
	vmsummbm 4, 27, 19, 4
	lxv 51, -96(17)
	vmsummbm 10, 18, 19, 10
	vmsummbm 0, 27, 19, 0
	lxv 50, 32(8)
	lxv 51, 32(7)
	lxv 59, 32(3)
	vmsummbm 17, 18, 27, 17
	vmsummbm 9, 19, 27, 9
	lxv 59, 32(5)
	vmsummbm 10, 18, 26, 10
	vmsummbm 0, 19, 26, 0
	lxv 58, -48(17)
	vmsummbm 16, 18, 27, 16
	vmsummbm 8, 19, 27, 8
	lxv 59, 32(26)
	vmsummbm 15, 18, 27, 15
	vmsummbm 7, 19, 27, 7
	lxv 59, 32(16)
	vmsummbm 14, 18, 27, 14
	vmsummbm 6, 19, 27, 6
	lxv 59, 32(20)
	vmsummbm 13, 18, 27, 13
	vmsummbm 1, 19, 27, 1
	lxv 59, 32(11)
	vmsummbm 12, 18, 27, 12
	vmsummbm 5, 19, 27, 5
	lxv 59, -80(12)
	vmsummbm 11, 18, 27, 11
	lxv 50, 48(8)
	vmsummbm 4, 19, 27, 4
	lxv 51, 48(3)
	lxv 59, 48(7)
	vmsummbm 17, 18, 19, 17
	vmsummbm 9, 27, 19, 9
	lxv 51, 48(5)
	vmsummbm 16, 18, 19, 16
	vmsummbm 8, 27, 19, 8
	lxv 51, 48(26)
	vmsummbm 15, 18, 19, 15
	vmsummbm 7, 27, 19, 7
	lxv 51, 48(16)
	vmsummbm 14, 18, 19, 14
	vmsummbm 6, 27, 19, 6
	lxv 51, 48(20)
	vmsummbm 13, 18, 19, 13
	vmsummbm 1, 27, 19, 1
	lxv 51, 48(11)
	vmsummbm 12, 18, 19, 12
	vmsummbm 5, 27, 19, 5
	lxv 51, -64(12)
	vmsummbm 11, 18, 19, 11
	vmsummbm 4, 27, 19, 4
	lxv 51, -64(17)
	vmsummbm 10, 18, 19, 10
	vmsummbm 0, 27, 19, 0
	lxv 50, 64(8)
	lxv 51, 64(7)
	lxv 59, 64(3)
	vmsummbm 17, 18, 27, 17
	vmsummbm 9, 19, 27, 9
	lxv 59, 64(5)
	vmsummbm 10, 18, 26, 10
	vmsummbm 0, 19, 26, 0
	lxv 58, -16(17)
	vmsummbm 16, 18, 27, 16
	vmsummbm 8, 19, 27, 8
	lxv 59, 64(26)
	vmsummbm 15, 18, 27, 15
	vmsummbm 7, 19, 27, 7
	lxv 59, 64(16)
	vmsummbm 14, 18, 27, 14
	vmsummbm 6, 19, 27, 6
	lxv 59, 64(20)
	vmsummbm 13, 18, 27, 13
	vmsummbm 1, 19, 27, 1
	lxv 59, 64(11)
	vmsummbm 12, 18, 27, 12
	vmsummbm 5, 19, 27, 5
	lxv 59, -48(12)
	vmsummbm 11, 18, 27, 11
	lxv 50, 80(8)
	vmsummbm 4, 19, 27, 4
	lxv 51, 80(3)
	lxv 59, 80(7)
	vmsummbm 17, 18, 19, 17
	vmsummbm 9, 27, 19, 9
	lxv 51, 80(5)
	vmsummbm 16, 18, 19, 16
	vmsummbm 8, 27, 19, 8
	lxv 51, 80(26)
	vmsummbm 15, 18, 19, 15
	vmsummbm 7, 27, 19, 7
	lxv 51, 80(16)
	vmsummbm 14, 18, 19, 14
	vmsummbm 6, 27, 19, 6
	lxv 51, 80(20)
	vmsummbm 13, 18, 19, 13
	vmsummbm 1, 27, 19, 1
	lxv 51, 80(11)
	vmsummbm 12, 18, 19, 12
	vmsummbm 5, 27, 19, 5
	lxv 51, -32(12)
	vmsummbm 11, 18, 19, 11
	vmsummbm 4, 27, 19, 4
	lxv 51, -32(17)
	vmsummbm 10, 18, 19, 10
	vmsummbm 0, 27, 19, 0
	lxv 50, 96(8)
	lxv 51, 96(7)
	lxv 59, 96(3)
	vmsummbm 17, 18, 27, 17
	vmsummbm 9, 19, 27, 9
	lxv 59, 96(5)
	vmsummbm 10, 18, 26, 10
	vmsummbm 0, 19, 26, 0
	vmsummbm 16, 18, 27, 16
	vmsummbm 8, 19, 27, 8
	lxv 59, 96(26)
	vmsummbm 15, 18, 27, 15
	vmsummbm 7, 19, 27, 7
	lxv 59, 96(16)
	vmsummbm 14, 18, 27, 14
	vmsummbm 6, 19, 27, 6
	lxv 59, 96(20)
	vmsummbm 13, 18, 27, 13
	vmsummbm 1, 19, 27, 1
	lxv 59, 96(11)
	vmsummbm 12, 18, 27, 12
	vmsummbm 5, 19, 27, 5
	lxv 59, -16(12)
	vmsummbm 11, 18, 27, 11
	lxv 50, 112(8)
	vmsummbm 4, 19, 27, 4
	lxv 51, 112(3)
	lxv 59, 112(7)
	addi 8, 8, 128
	addi 7, 7, 128
	addi 3, 3, 128
	vmsummbm 17, 18, 19, 17
	vmsummbm 9, 27, 19, 9
	lxv 51, 112(5)
	addi 5, 5, 128
	vmsummbm 16, 18, 19, 16
	vmsummbm 8, 27, 19, 8
	lxv 51, 112(26)
	addi 26, 26, 128
	vmsummbm 15, 18, 19, 15
	vmsummbm 7, 27, 19, 7
	lxv 51, 112(16)
	addi 16, 16, 128
	vmsummbm 14, 18, 19, 14
	vmsummbm 6, 27, 19, 6
	lxv 51, 112(20)
	addi 20, 20, 128
	vmsummbm 13, 18, 19, 13
	vmsummbm 1, 27, 19, 1
	lxv 51, 112(11)
	addi 11, 11, 128
	vmsummbm 12, 18, 19, 12
	vmsummbm 5, 27, 19, 5
	lxvx 51, 15, 29
	vmsummbm 11, 18, 19, 11
	vmsummbm 4, 27, 19, 4
	lxvx 51, 4, 29
	addi 29, 29, 128
	vmsummbm 10, 18, 19, 10
	vmsummbm 0, 27, 19, 0
	bdnz .Lp9_int8_BB4_86
# %bb.87:                               #   in Loop: Header=BB4_85 Depth=4
	ld 5, 864(1)                            # 8-byte Folded Reload
	ld 3, 872(1)                            # 8-byte Folded Reload
	vcfsx 17, 17, 0
	vcfsx 16, 16, 0
	ld 6, 880(1)                            # 8-byte Folded Reload
	ld 29, 928(1)                           # 8-byte Folded Reload
	vcfsx 15, 15, 0
	vcfsx 14, 14, 0
	ld 26, 912(1)                           # 8-byte Folded Reload
	ld 19, 888(1)                           # 8-byte Folded Reload
	addi 4, 4, 8320
	lxv 50, 8192(5)
	ld 5, 560(1)                            # 8-byte Folded Reload
	sldi 3, 3, 4
	vcfsx 13, 13, 0
	vcfsx 12, 12, 0
	vcfsx 11, 11, 0
	vcfsx 10, 10, 0
	vcfsx 9, 9, 0
	lxvwsx 51, 5, 3
	add 3, 5, 3
	li 5, 4
	addi 6, 6, 1
	vcfsx 8, 8, 0
	vcfsx 7, 7, 0
	vcfsx 6, 6, 0
	vcfsx 1, 1, 0
	vcfsx 5, 5, 0
	vcfsx 4, 4, 0
	addi 15, 15, 8320
	addi 29, 29, 8320
	addi 26, 26, 8320
	addi 18, 18, 1024
	addi 19, 19, 8320
	addi 27, 27, 1024
	xvmulsp 31, 51, 50
	xvmaddasp 35, 49, 31
	lxv 49, 8192(31)
	xvmulsp 31, 51, 49
	xvmaddasp 13, 48, 31
	lxv 48, 8192(21)
	ld 21, 760(1)                           # 8-byte Folded Reload
	cmpld	6, 21
	xvmulsp 31, 51, 48
	xvmaddasp 11, 47, 31
	lxv 47, 8192(24)
	ld 24, 896(1)                           # 8-byte Folded Reload
	addi 24, 24, 8320
	xvmulsp 31, 51, 47
	xvmaddasp 9, 46, 31
	lxv 46, 8192(25)
	ld 25, 904(1)                           # 8-byte Folded Reload
	addi 25, 25, 8320
	xvmulsp 31, 51, 46
	xvmaddasp 7, 45, 31
	lxv 45, 8192(28)
	ld 28, 920(1)                           # 8-byte Folded Reload
	addi 28, 28, 8320
	xvmulsp 31, 51, 45
	xvmaddasp 5, 44, 31
	lxv 44, 8192(10)
	xvmulsp 31, 51, 44
	xvmaddasp 3, 43, 31
	lxv 43, 8192(9)
	xvmulsp 51, 51, 43
	xvmaddasp 1, 42, 51
	lxvwsx 42, 3, 5
	xvmulsp 50, 42, 50
	xvmaddasp 34, 41, 50
	xvmulsp 41, 42, 49
	xvmaddasp 12, 40, 41
	xvmulsp 40, 42, 48
	xvmaddasp 10, 39, 40
	xvmulsp 39, 42, 47
	xvmaddasp 8, 38, 39
	xvmulsp 38, 42, 46
	xvmaddasp 6, 33, 38
	xvmulsp 33, 42, 45
	xvmaddasp 4, 37, 33
	xvmulsp 37, 42, 44
	xvmaddasp 2, 36, 37
	vcfsx 4, 0, 0
	xvmulsp 37, 42, 43
	xvmaddasp 0, 36, 37
	bne	0, .Lp9_int8_BB4_85
	b .Lp9_int8_BB4_136
.Lp9_int8_BB4_88:                               #   in Loop: Header=BB4_63 Depth=3
	lwz 3, 440(1)
	mtocrf 128, 3
	beq	0, .Lp9_int8_BB4_91
# %bb.89:                               #   in Loop: Header=BB4_63 Depth=3
	lwz 3, 372(1)
	mtocrf 128, 3
	bne	0, .Lp9_int8_BB4_118
# %bb.90:                               #   in Loop: Header=BB4_63 Depth=3
	addi 4, 1, 1472
	lxv 32, 400(1)                          # 16-byte Folded Reload
	ld 6, 1600(1)
	ld 12, 424(1)                           # 8-byte Folded Reload
	ori 5, 21, 1
	lxv 33, 384(1)                          # 16-byte Folded Reload
	ld 9, 1544(1)
	mr	17, 0
	addi 3, 1, 1640
	mr	18, 8
	ld 8, 1536(1)
	lxv 35, 1472(1)
	lxv 36, 16(4)
	lxv 37, 32(4)
	maddld 7, 21, 12, 6
	maddld 5, 5, 12, 6
	vsld 2, 3, 20
	vsld 3, 3, 0
	mfvsrld 10, 34
	mfvsrd 11, 34
	vsubudm 2, 3, 2
	vsld 3, 4, 20
	vsld 4, 4, 0
	std 7, 1768(1)
	std 5, 1776(1)
	ld 5, 1624(1)
	mfvsrld 6, 35
	mfvsrd 12, 35
	vsubudm 3, 4, 3
	vsld 4, 5, 20
	vsld 5, 5, 0
	add 9, 9, 11
	ld 11, 1576(1)
	vaddudm 2, 2, 1
	add 8, 8, 10
	ld 10, 1568(1)
	mfvsrld 7, 36
	mfvsrd 0, 36
	stxv 34, 64(3)
	vsubudm 4, 5, 4
	lxv 37, 48(4)
	ld 4, 1560(1)
	vaddudm 3, 3, 1
	std 9, 1648(1)
	std 5, 1800(1)
	std 8, 1640(1)
	stxv 35, 80(3)
	add 11, 11, 0
	vaddudm 4, 4, 1
	add 7, 10, 7
	add 4, 4, 12
	ld 12, 1592(1)
	std 11, 1680(1)
	stxv 36, 96(3)
	vsld 3, 5, 20
	std 7, 1672(1)
	vsld 5, 5, 0
	std 4, 1664(1)
	ld 4, 1616(1)
	mfvsrd 0, 35
	mfvsrld 10, 35
	vsubudm 2, 5, 3
	add 12, 12, 0
	ld 0, 1552(1)
	std 12, 1696(1)
	vaddudm 2, 2, 1
	stxv 34, 112(3)
	add 6, 0, 6
	ld 0, 1584(1)
	std 6, 1656(1)
	mulld 6, 5, 18
	add 10, 0, 10
	std 10, 1688(1)
	sldi 7, 6, 3
	sldi 6, 6, 4
	add 4, 4, 7
	std 4, 1784(1)
	ld 4, 1608(1)
	add 4, 4, 6
	std 4, 1792(1)
	li 4, 640
	std 4, 1808(1)
	addi 4, 1, 960
	std 4, 1816(1)
	bl pq8_q4_0_tile8x2_power9
	nop
	ld 10, 824(1)                           # 8-byte Folded Reload
	ld 12, 816(1)                           # 8-byte Folded Reload
	ld 11, 936(1)                           # 8-byte Folded Reload
	mr	0, 17
	ld 17, 520(1)                           # 8-byte Folded Reload
	li 3, 0
	mr	18, 19
	ld 4, 840(1)                            # 8-byte Folded Reload
	ld 5, 512(1)                            # 8-byte Folded Reload
	b .Lp9_int8_BB4_145
.Lp9_int8_BB4_91:                               #   in Loop: Header=BB4_63 Depth=3
	addi 3, 1, 1472
	ld 11, 424(1)                           # 8-byte Folded Reload
	ld 4, 1624(1)
	ori 7, 21, 2
	ld 9, 1544(1)
	ori 6, 21, 1
	lxv 32, 400(1)                          # 16-byte Folded Reload
	mr	17, 0
	std 21, 536(1)                          # 8-byte Folded Spill
	lxv 34, 16(3)
	lxv 35, 1472(1)
	ld 3, 1600(1)
	mulld 5, 4, 8
	ori 8, 21, 3
	std 4, 1752(1)
	li 4, 640
	maddld 10, 21, 11, 3
	maddld 7, 7, 11, 3
	maddld 6, 6, 11, 3
	maddld 3, 8, 11, 3
	std 4, 1760(1)
	addi 4, 1, 960
	mr	21, 20
	mr	20, 19
	mr	19, 12
	vsld 5, 3, 20
	vsld 4, 2, 20
	vsld 2, 2, 0
	std 4, 1768(1)
	vsld 3, 3, 0
	mfvsrld 8, 37
	vsubudm 2, 2, 4
	vsubudm 3, 3, 5
	std 10, 1704(1)
	mfvsrd 10, 37
	std 7, 1720(1)
	mfvsrd 7, 36
	std 3, 1728(1)
	sldi 3, 5, 3
	std 6, 1712(1)
	mfvsrld 6, 36
	lxv 36, 384(1)                          # 16-byte Folded Reload
	sldi 5, 5, 4
	add 9, 9, 10
	ld 10, 1560(1)
	std 9, 1648(1)
	vaddudm 3, 3, 4
	vaddudm 2, 2, 4
	add 7, 10, 7
	ld 10, 1616(1)
	std 7, 1664(1)
	add 10, 10, 3
	ld 3, 1536(1)
	std 10, 1736(1)
	add 8, 3, 8
	ld 3, 1552(1)
	std 8, 1640(1)
	add 6, 3, 6
	addi 3, 1, 1640
	std 6, 1656(1)
	ld 6, 1608(1)
	stxv 34, 48(3)
	stxv 35, 32(3)
	add 5, 6, 5
	std 5, 1744(1)
	bl pq8_q4_0_tile4x4_power9
	nop
	mr	5, 20
	ld 10, 824(1)                           # 8-byte Folded Reload
	mr	20, 21
	ld 21, 536(1)                           # 8-byte Folded Reload
	ld 11, 936(1)                           # 8-byte Folded Reload
	mr	0, 17
	ld 17, 520(1)                           # 8-byte Folded Reload
	mr	4, 18
	mr	12, 19
	li 3, 0
	mr	18, 5
	ld 5, 512(1)                            # 8-byte Folded Reload
	b .Lp9_int8_BB4_145
.Lp9_int8_BB4_92:                               #   in Loop: Header=BB4_63 Depth=3
	ld 9, 1624(1)
	xxlor 29, 52, 52
	std 14, 552(1)                          # 8-byte Folded Spill
	cmpldi	9, 0
	beq	0, .Lp9_int8_BB4_121
# %bb.93:                               #   in Loop: Header=BB4_63 Depth=3
	ld 3, 1472(1)
	mr	24, 8
	ld 28, 1536(1)
	ld 0, 1504(1)
	ld 20, 1608(1)
	mr	17, 21
	xxlxor 0, 0, 0
	xxlxor 1, 1, 1
	xxlxor 2, 2, 2
	mulld 24, 9, 24
	xxlxor 3, 3, 3
	mr	18, 19
	sldi 4, 3, 7
	sldi 5, 3, 4
	ld 3, 1480(1)
	sldi 29, 0, 7
	xxlxor 4, 4, 4
	xxlxor 5, 5, 5
	xxlxor 6, 6, 6
	sldi 21, 24, 4
	xxlxor 7, 7, 7
	xxlxor 8, 8, 8
	sldi 6, 3, 7
	sldi 7, 3, 4
	ld 3, 1488(1)
	xxlxor 9, 9, 9
	xxlxor 10, 10, 10
	xxlxor 11, 11, 11
	xxlxor 12, 12, 12
	add 21, 20, 21
	xxlxor 13, 13, 13
	xxlxor 31, 31, 31
	addi 14, 21, 4
	mr	21, 17
	xxlxor 30, 30, 30
	sldi 8, 3, 7
	sldi 10, 3, 4
	ld 3, 1496(1)
	sldi 11, 3, 7
	sldi 12, 3, 4
	add 3, 28, 4
	add 28, 28, 5
	ld 5, 1544(1)
	add 4, 5, 6
	add 26, 5, 7
	ld 6, 1552(1)
	ld 7, 1560(1)
	add 5, 6, 8
	add 8, 6, 10
	add 10, 7, 12
	addi 12, 28, 1024
	add 6, 7, 11
	ld 11, 1568(1)
	ld 28, 1528(1)
	std 12, 928(1)                          # 8-byte Folded Spill
	addi 12, 26, 1024
	addi 8, 8, 1024
	addi 10, 10, 1024
	std 12, 920(1)                          # 8-byte Folded Spill
	ld 12, 1512(1)
	std 8, 912(1)                           # 8-byte Folded Spill
	sldi 8, 0, 4
	ld 0, 1576(1)
	add 7, 11, 29
	ld 29, 1584(1)
	std 10, 904(1)                          # 8-byte Folded Spill
	add 8, 11, 8
	sldi 11, 12, 7
	sldi 12, 12, 4
	addi 8, 8, 1024
	add 11, 0, 11
	add 26, 0, 12
	ld 0, 1520(1)
	std 8, 896(1)                           # 8-byte Folded Spill
	addi 8, 26, 1024
	sldi 12, 0, 7
	sldi 0, 0, 4
	std 8, 888(1)                           # 8-byte Folded Spill
	add 25, 29, 0
	ld 0, 1592(1)
	add 12, 29, 12
	sldi 29, 28, 7
	sldi 28, 28, 4
	addi 8, 25, 1024
	add 20, 0, 28
	ld 28, 1616(1)
	add 29, 0, 29
	sldi 0, 24, 3
	std 8, 880(1)                           # 8-byte Folded Spill
	ld 8, 832(1)                            # 8-byte Folded Reload
	add 24, 28, 0
	ld 0, 1600(1)
	add 15, 0, 8
	addi 8, 20, 1024
	add 28, 0, 19
	li 19, 7
	std 8, 872(1)                           # 8-byte Folded Spill
	addi 8, 24, 2
	mtctr 9
	li 9, 0
	.p2align	5
.Lp9_int8_BB4_94:                               #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lha 10, -2(8)
	lha 0, 0(8)
	lxv 32, -64(15)
	lxvx 33, 3, 9
	lxv 39, -64(28)
	add 31, 3, 9
	lxvx 40, 4, 9
	lxvx 42, 5, 9
	lxvx 44, 6, 9
	add 16, 4, 9
	add 20, 5, 9
	lxvx 46, 7, 9
	lxvx 48, 11, 9
	lxvx 50, 12, 9
	lxvx 59, 29, 9
	add 25, 6, 9
	add 17, 7, 9
	add 24, 11, 9
	add 26, 29, 9
	lxv 58, 16(26)
	lxv 56, -16(28)
	lxv 35, 64(26)
	lxv 61, 80(25)
	lxv 25, 96(31)
	lxv 24, 112(31)
	addi 8, 8, 8
	slwi 10, 10, 4
	neg 10, 10
	mtvsrws 36, 10
	slwi 10, 0, 4
	neg 10, 10
	vmsummbm 6, 0, 1, 4
	vmsummbm 9, 0, 8, 4
	vmsummbm 11, 0, 10, 4
	mtvsrws 37, 10
	vmsummbm 13, 0, 12, 4
	vmsummbm 15, 0, 14, 4
	vmsummbm 17, 0, 16, 4
	vmsummbm 19, 0, 18, 4
	vmsummbm 4, 0, 27, 4
	lxv 32, -48(15)
	add 10, 12, 9
	lxv 62, 64(10)
	vmsummbm 1, 7, 1, 5
	vmsummbm 8, 7, 8, 5
	vmsummbm 10, 7, 10, 5
	vmsummbm 12, 7, 12, 5
	vmsummbm 14, 7, 14, 5
	vmsummbm 16, 7, 16, 5
	vmsummbm 18, 7, 18, 5
	vmsummbm 5, 7, 27, 5
	lxv 39, -48(28)
	lxv 59, 16(31)
	vmsummbm 4, 0, 26, 4
	vmsummbm 6, 0, 27, 6
	vmsummbm 1, 7, 27, 1
	lxv 59, 16(16)
	vmsummbm 5, 7, 26, 5
	vmsummbm 9, 0, 27, 9
	vmsummbm 8, 7, 27, 8
	lxv 59, 16(20)
	vmsummbm 11, 0, 27, 11
	vmsummbm 10, 7, 27, 10
	lxv 59, 16(25)
	vmsummbm 13, 0, 27, 13
	vmsummbm 12, 7, 27, 12
	lxv 59, 16(17)
	vmsummbm 15, 0, 27, 15
	vmsummbm 14, 7, 27, 14
	lxv 59, 16(24)
	vmsummbm 17, 0, 27, 17
	vmsummbm 16, 7, 27, 16
	lxv 59, 16(10)
	vmsummbm 19, 0, 27, 19
	lxv 32, -32(15)
	vmsummbm 18, 7, 27, 18
	lxv 39, 32(31)
	lxv 59, -32(28)
	vmsummbm 6, 0, 7, 6
	vmsummbm 1, 27, 7, 1
	lxv 39, 32(16)
	vmsummbm 9, 0, 7, 9
	vmsummbm 7, 27, 7, 8
	lxv 40, 32(20)
	vmsummbm 11, 0, 8, 11
	vmsummbm 8, 27, 8, 10
	lxv 42, 32(25)
	vmsummbm 13, 0, 10, 13
	vmsummbm 10, 27, 10, 12
	lxv 44, 32(17)
	vmsummbm 15, 0, 12, 15
	vmsummbm 12, 27, 12, 14
	lxv 46, 32(24)
	vmsummbm 26, 0, 14, 17
	vmsummbm 14, 27, 14, 16
	lxv 48, 32(10)
	vmsummbm 19, 0, 16, 19
	vmsummbm 25, 27, 16, 18
	lxv 48, 32(26)
	vmsummbm 4, 0, 16, 4
	vmsummbm 5, 27, 16, 5
	lxv 32, -16(15)
	lxv 48, 48(31)
	vmsummbm 23, 0, 16, 6
	lxv 38, 48(16)
	vmsummbm 1, 24, 16, 1
	vmsummbm 9, 0, 6, 9
	vmsummbm 22, 24, 6, 7
	lxv 38, 48(20)
	lxv 39, 0(28)
	vmsummbm 21, 0, 6, 11
	vmsummbm 20, 24, 6, 8
	lxv 38, 48(25)
	lxv 40, 64(31)
	vmsummbm 31, 0, 6, 13
	vmsummbm 27, 24, 6, 10
	lxv 38, 48(17)
	lxv 42, 64(16)
	vmsummbm 1, 7, 8, 1
	vmsummbm 18, 0, 6, 15
	vmsummbm 17, 24, 6, 12
	lxv 38, 48(24)
	lxv 44, 112(25)
	vmsummbm 16, 0, 6, 26
	vmsummbm 15, 24, 6, 14
	lxv 38, 48(10)
	lxv 58, 80(31)
	vmsummbm 14, 0, 6, 19
	vmsummbm 11, 24, 6, 25
	lxv 38, 48(26)
	lxv 57, 64(25)
	lxv 51, 112(20)
	vmsummbm 4, 0, 6, 4
	vmsummbm 5, 24, 6, 5
	lxv 38, 0(15)
	lxv 56, 80(16)
	vmsummbm 2, 6, 8, 23
	vmsummbm 8, 6, 10, 9
	vmsummbm 0, 6, 25, 31
	lxv 63, 64(17)
	vmsummbm 25, 7, 25, 27
	vmsummbm 14, 6, 30, 14
	vmsummbm 30, 7, 30, 11
	lxv 55, 96(16)
	lxv 59, 112(16)
	vmsummbm 4, 6, 3, 4
	vmsummbm 18, 6, 31, 18
	vmsummbm 31, 7, 31, 17
	xxlor 28, 34, 34
	vmsummbm 2, 7, 10, 22
	lxv 42, 64(20)
	lxv 54, 64(24)
	vmsummbm 13, 7, 10, 20
	vmsummbm 16, 6, 22, 16
	vmsummbm 22, 7, 22, 15
	lxv 52, 96(20)
	xxlor 27, 34, 34
	vmsummbm 2, 6, 10, 21
	lxv 53, 80(20)
	lxv 38, 16(28)
	lxv 42, 96(25)
	vmsummbm 17, 6, 21, 13
	lxv 45, 80(17)
	vmsummbm 1, 6, 26, 1
	xxlor 26, 34, 34
	vmsummbm 2, 7, 3, 5
	lxv 37, 16(15)
	xxlor 35, 28, 28
	vmsummbm 11, 5, 29, 0
	vmsummbm 29, 6, 29, 25
	lxv 57, 80(24)
	vmsummbm 18, 5, 13, 18
	vmsummbm 13, 6, 13, 31
	vmsummbm 9, 5, 26, 3
	xxlor 35, 27, 27
	vmsummbm 7, 5, 24, 8
	lxv 58, 96(17)
	vmsummbm 8, 6, 24, 3
	xxlor 35, 26, 26
	lxv 56, 112(17)
	vmsummbm 15, 5, 21, 3
	lxv 35, 112(10)
	lxv 53, 112(24)
	vmsummbm 16, 5, 25, 16
	vmsummbm 31, 6, 25, 22
	lxv 57, 80(10)
	lxv 54, 96(24)
	vmsummbm 14, 5, 25, 14
	vmsummbm 30, 6, 25, 30
	lxv 57, 80(26)
	vmsummbm 0, 5, 25, 4
	lxv 37, 96(10)
	vmsummbm 4, 6, 25, 2
	lxv 38, 32(15)
	lxv 57, 32(28)
	xxlor 34, 25, 25
	li 10, -4
	vmsummbm 7, 6, 23, 7
	vmsummbm 8, 25, 23, 8
	lxv 55, 96(26)
	vmsummbm 9, 6, 2, 9
	vmsummbm 14, 6, 5, 14
	vmsummbm 5, 25, 5, 30
	lxv 62, 48(15)
	vmsummbm 15, 6, 20, 15
	addi 15, 15, 128
	vmsummbm 11, 6, 10, 11
	vmsummbm 18, 6, 26, 18
	vmsummbm 16, 6, 22, 16
	vmsummbm 1, 25, 2, 1
	lxv 34, 112(26)
	vmsummbm 10, 25, 10, 29
	lxv 61, 48(28)
	vmsummbm 13, 25, 26, 13
	vmsummbm 17, 25, 20, 17
	vmsummbm 31, 25, 22, 31
	addi 28, 28, 128
	vmsummbm 6, 6, 23, 0
	vmsummbm 4, 25, 23, 4
	xxlor 32, 24, 24
	vmsummbm 9, 30, 0, 9
	vmsummbm 14, 30, 3, 14
	vmsummbm 5, 29, 3, 5
	vmsummbm 26, 29, 0, 1
	vmsummbm 7, 30, 27, 7
	vmsummbm 1, 29, 24, 13
	vmsummbm 13, 30, 21, 16
	vmsummbm 15, 30, 19, 15
	vmsummbm 11, 30, 12, 11
	vmsummbm 10, 29, 12, 10
	vmsummbm 12, 30, 24, 18
	vmsummbm 8, 29, 27, 8
	vmsummbm 17, 29, 19, 17
	vmsummbm 0, 29, 21, 31
	vmsummbm 3, 30, 2, 6
	vmsummbm 4, 29, 2, 4
	lxvwsx 34, 14, 10
	ld 10, 928(1)                           # 8-byte Folded Reload
	vcfsx 6, 9, 0
	vcfsx 5, 5, 0
	lxvx 41, 10, 9
	ld 10, 920(1)                           # 8-byte Folded Reload
	vcfsx 3, 3, 0
	vcfsx 4, 4, 0
	xvmulsp 48, 34, 41
	xvmaddasp 30, 38, 48
	vcfsx 6, 7, 0
	lxvx 39, 10, 9
	ld 10, 912(1)                           # 8-byte Folded Reload
	xvmulsp 48, 34, 39
	xvmaddasp 13, 38, 48
	vcfsx 6, 15, 0
	lxvx 47, 10, 9
	ld 10, 904(1)                           # 8-byte Folded Reload
	xvmulsp 48, 34, 47
	xvmaddasp 11, 38, 48
	vcfsx 6, 11, 0
	lxvx 43, 10, 9
	ld 10, 896(1)                           # 8-byte Folded Reload
	xvmulsp 48, 34, 43
	xvmaddasp 9, 38, 48
	vcfsx 6, 12, 0
	lxvx 44, 10, 9
	ld 10, 888(1)                           # 8-byte Folded Reload
	xvmulsp 48, 34, 44
	xvmaddasp 7, 38, 48
	vcfsx 6, 13, 0
	lxvx 45, 10, 9
	ld 10, 880(1)                           # 8-byte Folded Reload
	xvmulsp 48, 34, 45
	xvmaddasp 5, 38, 48
	vcfsx 6, 14, 0
	lxvx 46, 10, 9
	ld 10, 872(1)                           # 8-byte Folded Reload
	xvmulsp 48, 34, 46
	xvmaddasp 3, 38, 48
	lxvx 38, 10, 9
	addi 9, 9, 1152
	xvmulsp 34, 34, 38
	xvmaddasp 1, 35, 34
	lxvwsx 34, 0, 14
	vcfsx 3, 26, 0
	addi 14, 14, 16
	xvmulsp 41, 34, 41
	xvmulsp 39, 34, 39
	xvmaddasp 31, 35, 41
	vcfsx 3, 8, 0
	vcfsx 8, 17, 0
	xvmulsp 41, 34, 43
	xvmaddasp 12, 35, 39
	xvmulsp 35, 34, 47
	vcfsx 7, 10, 0
	xvmaddasp 10, 40, 35
	vcfsx 3, 1, 0
	xvmulsp 33, 34, 44
	xvmaddasp 8, 39, 41
	xvmaddasp 6, 35, 33
	vcfsx 3, 0, 0
	xvmulsp 32, 34, 45
	xvmaddasp 4, 35, 32
	xvmulsp 35, 34, 46
	xvmulsp 34, 34, 38
	xvmaddasp 2, 37, 35
	xvmaddasp 0, 36, 34
	bdnz .Lp9_int8_BB4_94
	b .Lp9_int8_BB4_122
.Lp9_int8_BB4_95:                               #   in Loop: Header=BB4_63 Depth=3
	ld 10, 1624(1)
	cmpldi	10, 0
	beq	0, .Lp9_int8_BB4_123
# %bb.96:                               #   in Loop: Header=BB4_63 Depth=3
	ld 7, 1616(1)
	mr	9, 8
	ld 8, 832(1)                            # 8-byte Folded Reload
	ld 3, 1472(1)
	ld 4, 1480(1)
	xxlxor 0, 0, 0
	cmpdi	7, 0
	std 7, 904(1)                           # 8-byte Folded Spill
	ld 7, 1600(1)
	ld 5, 1488(1)
	mulli 11, 3, 224
	sldi 3, 3, 9
	ld 25, 1544(1)
	mulli 12, 4, 224
	ld 6, 1496(1)
	sldi 4, 4, 9
	ld 24, 1552(1)
	mulli 29, 5, 224
	ld 0, 1560(1)
	ld 18, 1608(1)
	xxlxor 1, 1, 1
	add 4, 25, 4
	sldi 5, 5, 9
	mulli 26, 6, 224
	sldi 6, 6, 9
	add 5, 24, 5
	add 6, 0, 6
	xxlxor 2, 2, 2
	add 28, 7, 8
	ld 8, 1536(1)
	xxlxor 3, 3, 3
	xxlxor 4, 4, 4
	xxlxor 5, 5, 5
	std 25, 888(1)                          # 8-byte Folded Spill
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
	add 3, 8, 3
	std 8, 896(1)                           # 8-byte Folded Spill
	std 11, 928(1)                          # 8-byte Folded Spill
	std 12, 920(1)                          # 8-byte Folded Spill
	std 29, 912(1)                          # 8-byte Folded Spill
	std 24, 880(1)                          # 8-byte Folded Spill
	std 26, 760(1)                          # 8-byte Folded Spill
	addi 3, 3, 48
	std 3, 872(1)                           # 8-byte Folded Spill
	addi 3, 4, 48
	add 4, 25, 12
	std 3, 864(1)                           # 8-byte Folded Spill
	addi 3, 5, 48
	add 5, 24, 29
	std 3, 856(1)                           # 8-byte Folded Spill
	addi 3, 6, 48
	add 6, 0, 26
	std 3, 848(1)                           # 8-byte Folded Spill
	add 3, 8, 11
	add 8, 7, 19
	mr	19, 10
	addi 3, 3, 2144
	std 3, 752(1)                           # 8-byte Folded Spill
	addi 3, 4, 2144
	std 3, 744(1)                           # 8-byte Folded Spill
	addi 3, 5, 2144
	li 5, 0
	std 3, 736(1)                           # 8-byte Folded Spill
	addi 3, 6, 2144
	li 6, 0
	std 3, 728(1)                           # 8-byte Folded Spill
	mulld 3, 10, 9
	std 3, 720(1)                           # 8-byte Folded Spill
	.p2align	4
.Lp9_int8_BB4_97:                               #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB4_98 Depth 5
	mulli 7, 6, 2944
	ld 10, 896(1)                           # 8-byte Folded Reload
	ld 11, 888(1)                           # 8-byte Folded Reload
	ld 12, 880(1)                           # 8-byte Folded Reload
	ld 24, 848(1)                           # 8-byte Folded Reload
	ld 26, 856(1)                           # 8-byte Folded Reload
	ld 29, 864(1)                           # 8-byte Folded Reload
	ld 16, 872(1)                           # 8-byte Folded Reload
	ld 31, 728(1)                           # 8-byte Folded Reload
	mr	3, 8
	mr	9, 28
	ld 14, 736(1)                           # 8-byte Folded Reload
	ld 4, 744(1)                            # 8-byte Folded Reload
	xxlxor 34, 34, 34
	add 10, 10, 7
	add 11, 11, 7
	add 12, 12, 7
	add 21, 0, 7
	ld 7, 928(1)                            # 8-byte Folded Reload
	xxlxor 37, 37, 37
	xxlxor 35, 35, 35
	xxlxor 32, 32, 32
	xxlxor 36, 36, 36
	xxlxor 38, 38, 38
	xxlxor 33, 33, 33
	xxlxor 39, 39, 39
	add 7, 10, 7
	ld 10, 920(1)                           # 8-byte Folded Reload
	add 20, 11, 10
	ld 10, 912(1)                           # 8-byte Folded Reload
	li 11, 8
	add 25, 12, 10
	ld 10, 760(1)                           # 8-byte Folded Reload
	add 15, 21, 10
	ld 10, 752(1)                           # 8-byte Folded Reload
	mtctr 11
	.p2align	5
.Lp9_int8_BB4_98:                               #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        #         Parent Loop BB4_97 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	add 11, 16, 5
	add 12, 29, 5
	lxv 41, -64(9)
	lxv 42, -48(9)
	lxv 43, -64(3)
	lxv 44, -48(3)
	add 17, 26, 5
	add 21, 24, 5
	lxv 45, -48(11)
	xxspltib 8, 15
	xxspltib 40, 4
	lxv 48, -48(12)
	lxv 63, -48(21)
	lxv 61, -16(3)
	lxv 58, 48(3)
	xxland 46, 45, 8
	xxland 49, 48, 8
	vsrb 13, 13, 8
	vsrb 16, 16, 8
	xxland 62, 63, 8
	vmsummbm 15, 9, 14, 28
	vmsummbm 14, 11, 14, 28
	vmsummbm 18, 9, 17, 28
	vmsummbm 17, 11, 17, 28
	vmsummbm 15, 10, 13, 15
	vmsummbm 13, 12, 13, 14
	vmsummbm 14, 10, 16, 18
	lxv 50, -48(17)
	vmsummbm 16, 12, 16, 17
	xxland 51, 50, 8
	vsrb 18, 18, 8
	vmsummbm 17, 9, 19, 28
	vmsummbm 19, 11, 19, 28
	vmsummbm 9, 9, 30, 28
	vmsummbm 11, 11, 30, 28
	vmsummbm 17, 10, 18, 17
	vmsummbm 18, 12, 18, 19
	vsrb 19, 31, 8
	lxv 63, -32(11)
	vmsummbm 9, 10, 19, 9
	vmsummbm 10, 12, 19, 11
	lxv 43, -32(9)
	lxv 51, -32(3)
	lxv 44, -16(9)
	xxland 62, 63, 8
	vsrb 31, 31, 8
	vmsummbm 15, 11, 30, 15
	vmsummbm 13, 19, 30, 13
	lxv 62, -32(12)
	xxland 59, 62, 8
	vmsummbm 14, 11, 27, 14
	vmsummbm 16, 19, 27, 16
	vmsummbm 15, 12, 31, 15
	vmsummbm 13, 29, 31, 13
	vsrb 31, 30, 8
	lxv 62, -32(17)
	xxland 59, 62, 8
	vsrb 30, 30, 8
	vmsummbm 14, 12, 31, 14
	vmsummbm 16, 29, 31, 16
	lxv 63, -32(21)
	vmsummbm 17, 11, 27, 17
	vmsummbm 18, 19, 27, 18
	xxland 59, 63, 8
	vmsummbm 9, 11, 27, 9
	vmsummbm 17, 12, 30, 17
	vmsummbm 18, 29, 30, 18
	vmsummbm 10, 19, 27, 10
	vsrb 11, 31, 8
	lxv 62, -16(11)
	lxv 63, 0(3)
	lxv 51, 16(9)
	xxland 59, 62, 8
	vmsummbm 9, 12, 11, 9
	lxv 44, 0(9)
	vmsummbm 10, 29, 11, 10
	vmsummbm 13, 31, 27, 13
	lxv 61, -16(12)
	vmsummbm 11, 12, 27, 15
	vsrb 15, 30, 8
	lxv 62, 16(3)
	xxland 59, 61, 8
	vmsummbm 14, 12, 27, 14
	vmsummbm 13, 30, 15, 13
	vmsummbm 11, 19, 15, 11
	vmsummbm 15, 31, 27, 16
	vsrb 16, 29, 8
	lxv 61, -16(17)
	vmsummbm 14, 19, 16, 14
	xxland 59, 61, 8
	vmsummbm 15, 30, 16, 15
	vsrb 16, 29, 8
	lxv 61, -16(21)
	vmsummbm 17, 12, 27, 17
	vmsummbm 18, 31, 27, 18
	xxland 59, 61, 8
	vmsummbm 9, 12, 27, 9
	vmsummbm 10, 31, 27, 10
	vsrb 12, 29, 8
	lxvx 63, 16, 5
	vmsummbm 17, 19, 16, 17
	vmsummbm 16, 30, 16, 18
	lxv 50, 32(9)
	lxvx 61, 26, 5
	addi 16, 16, 64
	addi 26, 26, 64
	xxland 59, 61, 8
	vmsummbm 9, 19, 12, 9
	lxv 51, 32(3)
	vmsummbm 10, 30, 12, 10
	xxland 44, 63, 8
	vmsummbm 17, 18, 27, 17
	vsrb 31, 31, 8
	addi 3, 3, 128
	vmsummbm 11, 18, 12, 11
	vmsummbm 12, 19, 12, 13
	lxvx 45, 29, 5
	vmsummbm 16, 19, 27, 16
	lxvx 59, 24, 5
	addi 29, 29, 64
	addi 24, 24, 64
	xxland 62, 45, 8
	xxland 57, 59, 8
	vsrb 13, 13, 8
	vmsummbm 14, 18, 30, 14
	vmsummbm 15, 19, 30, 15
	lxv 62, 48(9)
	addi 9, 9, 128
	vmsummbm 9, 18, 25, 9
	vmsummbm 10, 19, 25, 10
	vmsummbm 12, 26, 31, 12
	vmsummbm 11, 30, 31, 11
	vmsummbm 14, 30, 13, 14
	vmsummbm 13, 26, 13, 15
	vsrb 15, 29, 8
	vsrb 8, 27, 8
	vmsummbm 9, 30, 8, 9
	vmsummbm 8, 26, 8, 10
	lxvx 42, 10, 5
	vmsummbm 17, 30, 15, 17
	addi 10, 10, 16
	vmsummbm 15, 26, 15, 16
	vmuluwm 11, 11, 10
	vmuluwm 10, 12, 10
	vadduwm 1, 10, 1
	lxvx 42, 4, 5
	vadduwm 7, 11, 7
	addi 4, 4, 16
	vmuluwm 11, 14, 10
	vmuluwm 10, 13, 10
	vadduwm 4, 10, 4
	lxvx 42, 14, 5
	vadduwm 6, 11, 6
	addi 14, 14, 16
	vmuluwm 11, 17, 10
	vmuluwm 10, 15, 10
	vadduwm 0, 11, 0
	lxvx 43, 31, 5
	addi 31, 31, 16
	vadduwm 3, 10, 3
	vmuluwm 9, 9, 11
	vmuluwm 8, 8, 11
	vadduwm 5, 9, 5
	vadduwm 2, 8, 2
	bdnz .Lp9_int8_BB4_98
# %bb.99:                               #   in Loop: Header=BB4_97 Depth=4
	ld 3, 720(1)                            # 8-byte Folded Reload
	lxv 12, 2064(7)
	lxv 48, 2128(7)
	lxv 49, 2112(7)
	li 10, 0
	vcfsx 1, 1, 0
	addi 5, 5, 2944
	lxv 50, 2096(7)
	lxv 51, 2080(7)
	lxv 13, 2048(7)
	ld 7, 904(1)                            # 8-byte Folded Reload
	vcfsx 7, 7, 0
	vcfsx 4, 4, 0
	lxv 31, 2048(20)
	lxv 56, 2080(20)
	vcfsx 6, 6, 0
	lxv 11, 2048(25)
	lxv 62, 2096(20)
	vcfsx 0, 0, 0
	lxv 9, 2048(15)
	lxv 57, 2112(20)
	vcfsx 3, 3, 0
	vcfsx 2, 2, 0
	lxv 43, 2080(15)
	lxv 47, 2080(25)
	add 3, 6, 3
	lxv 58, 2128(20)
	lxv 42, 2096(15)
	lxv 46, 2096(25)
	lxv 30, 2064(20)
	lxv 41, 2112(15)
	vcfsx 5, 5, 0
	lxv 45, 2112(25)
	lxv 40, 2128(15)
	lxv 44, 2128(25)
	addi 6, 6, 1
	addi 28, 28, 1024
	lxv 8, 2064(15)
	lxv 10, 2064(25)
	addi 8, 8, 1024
	sldi 4, 3, 6
	cmpld 1, 6, 19
	add 4, 7, 4
	sldi 3, 3, 4
	lxvwsx 29, 18, 3
	addi 7, 4, 4
	iseleq	9, 10, 4
	iseleq	4, 10, 7
	add 7, 18, 3
	li 3, 4
	lxvwsx 63, 0, 9
	li 10, 48
	xvmulsp 27, 29, 13
	lxvwsx 28, 7, 3
	li 7, 16
	li 3, 32
	lxvwsx 59, 0, 4
	lxvwsx 55, 9, 10
	lxvwsx 61, 9, 7
	vmsumshm 22, 19, 31, 28
	vmsumshm 21, 15, 31, 28
	xvmulsp 13, 28, 13
	vmsumshm 19, 19, 27, 28
	xvmaddasp 7, 39, 27
	xvmulsp 27, 29, 31
	lxvwsx 39, 9, 3
	vmsumshm 22, 18, 29, 22
	vmsumshm 21, 14, 29, 21
	xvmaddasp 6, 33, 13
	lxvwsx 33, 4, 7
	xvmulsp 13, 28, 31
	xvmaddasp 5, 38, 27
	xvmulsp 27, 29, 11
	xvmulsp 11, 28, 11
	xvmulsp 38, 29, 12
	xvmulsp 12, 28, 12
	vmsumshm 18, 18, 1, 19
	lxvwsx 51, 4, 3
	xvmaddasp 4, 36, 13
	vmsumshm 4, 24, 31, 28
	vmsumshm 31, 11, 31, 28
	vmsumshm 24, 24, 27, 28
	xvmaddasp 3, 32, 27
	xvmulsp 27, 29, 9
	xvmulsp 9, 28, 9
	xvmaddasp 2, 35, 11
	vmsumshm 3, 15, 27, 28
	xvmulsp 32, 29, 30
	vmsumshm 4, 30, 29, 4
	vmsumshm 31, 10, 29, 31
	vmsumshm 30, 30, 1, 24
	vmsumshm 24, 17, 7, 22
	vmsumshm 17, 17, 19, 18
	lxvwsx 61, 4, 10
	vmsumshm 22, 13, 7, 21
	xvmaddasp 0, 34, 9
	vmsumshm 2, 11, 27, 28
	vmsumshm 3, 14, 1, 3
	xvmaddasp 1, 37, 27
	xvmulsp 37, 29, 10
	xvmulsp 29, 29, 8
	xvmulsp 10, 28, 10
	xvmulsp 8, 28, 8
	vmsumshm 4, 25, 7, 4
	vmsumshm 7, 9, 7, 31
	vmsumshm 18, 16, 23, 24
	vmsumshm 16, 16, 29, 17
	vmsumshm 31, 12, 23, 22
	vmsumshm 2, 10, 1, 2
	vmsumshm 3, 13, 19, 3
	vmsumshm 4, 26, 23, 4
	vmsumshm 7, 8, 23, 7
	vcfsx 16, 16, 0
	vcfsx 17, 18, 0
	vcfsx 18, 31, 0
	vmsumshm 2, 9, 19, 2
	vmsumshm 3, 12, 29, 3
	vcfsx 4, 4, 0
	vcfsx 7, 7, 0
	xvnmsubasp 6, 48, 12
	xvmulsp 12, 28, 30
	xvnmsubasp 7, 49, 38
	xvnmsubasp 3, 50, 37
	vmsumshm 2, 8, 29, 2
	vcfsx 3, 3, 0
	xvnmsubasp 5, 36, 32
	vmsumshm 4, 25, 19, 30
	xvnmsubasp 1, 39, 29
	vcfsx 2, 2, 0
	xvnmsubasp 2, 35, 10
	vmsumshm 4, 26, 29, 4
	xvnmsubasp 0, 34, 8
	vcfsx 4, 4, 0
	xvnmsubasp 4, 36, 12
	bne	1, .Lp9_int8_BB4_97
	b .Lp9_int8_BB4_124
.Lp9_int8_BB4_100:                              #   in Loop: Header=BB4_63 Depth=3
	ld 18, 1624(1)
	xxlxor 0, 0, 0
	xxlxor 1, 1, 1
	cmpldi	18, 0
	xxlxor 2, 2, 2
	xxlxor 3, 3, 3
	xxlxor 4, 4, 4
	xxlxor 5, 5, 5
	xxlxor 6, 6, 6
	beq	0, .Lp9_int8_BB4_125
# %bb.101:                              #   in Loop: Header=BB4_63 Depth=3
	ld 3, 1472(1)
	ld 4, 1480(1)
	ld 5, 1488(1)
	ld 28, 1544(1)
	mr	25, 8
	xxlxor 7, 7, 7
	li 31, 0
	ld 7, 1600(1)
	ld 24, 1560(1)
	ld 9, 952(1)                            # 8-byte Folded Reload
	ld 26, 1552(1)
	xxlxor 8, 8, 8
	xxlxor 9, 9, 9
	xxlxor 10, 10, 10
	xxlxor 11, 11, 11
	std 24, 872(1)                          # 8-byte Folded Spill
	sldi 6, 3, 10
	sldi 3, 3, 4
	add 10, 7, 27
	add 11, 7, 11
	add 12, 7, 9
	add 29, 7, 0
	ld 0, 1536(1)
	std 26, 880(1)                          # 8-byte Folded Spill
	xxlxor 13, 13, 13
	li 19, 7
	std 28, 888(1)                          # 8-byte Folded Spill
	xxlxor 34, 34, 34
	std 3, 912(1)                           # 8-byte Folded Spill
	sldi 3, 4, 10
	xxlxor 35, 35, 35
	add 3, 28, 3
	sldi 4, 4, 4
	std 4, 904(1)                           # 8-byte Folded Spill
	sldi 4, 5, 10
	add 6, 0, 6
	xxlxor 12, 12, 12
	add 4, 26, 4
	sldi 5, 5, 4
	std 5, 896(1)                           # 8-byte Folded Spill
	ld 5, 1496(1)
	addi 15, 6, 112
	addi 26, 4, 112
	sldi 8, 5, 10
	add 7, 24, 8
	addi 24, 3, 112
	ld 3, 1616(1)
	li 8, 8
	addi 28, 7, 112
	std 3, 864(1)                           # 8-byte Folded Spill
	ld 3, 1608(1)
	std 3, 856(1)                           # 8-byte Folded Spill
	sldi 3, 5, 4
	std 3, 848(1)                           # 8-byte Folded Spill
	mulld 3, 18, 25
	std 3, 760(1)                           # 8-byte Folded Spill
	.p2align	4
.Lp9_int8_BB4_102:                              #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB4_103 Depth 5
	mulli 3, 31, 8320
	ld 5, 912(1)                            # 8-byte Folded Reload
	ld 7, 896(1)                            # 8-byte Folded Reload
	add 4, 0, 3
	add 6, 4, 5
	ld 4, 888(1)                            # 8-byte Folded Reload
	ld 5, 904(1)                            # 8-byte Folded Reload
	add 4, 4, 3
	add 5, 4, 5
	ld 4, 880(1)                            # 8-byte Folded Reload
	add 4, 4, 3
	add 4, 4, 7
	li 7, 0
	std 4, 920(1)                           # 8-byte Folded Spill
	ld 4, 872(1)                            # 8-byte Folded Reload
	add 3, 4, 3
	ld 4, 848(1)                            # 8-byte Folded Reload
	add 3, 3, 4
	ld 4, 864(1)                            # 8-byte Folded Reload
	std 3, 928(1)                           # 8-byte Folded Spill
	ld 3, 760(1)                            # 8-byte Folded Reload
	add 25, 31, 3
	sldi 3, 25, 3
	lhaux 3, 4, 3
	neg 3, 3
	mtvsrws 46, 3
	lha 3, 2(4)
	neg 3, 3
	vmr	16, 14
	vmr	15, 14
	vmr	17, 14
	mtvsrws 42, 3
	lha 3, 4(4)
	vmr	11, 10
	vmr	12, 10
	vmr	13, 10
	neg 3, 3
	mtvsrws 41, 3
	lha 3, 6(4)
	mtctr 8
	vmr	8, 9
	neg 3, 3
	vmr	6, 9
	vmr	7, 9
	mtvsrws 36, 3
	vmr	5, 4
	vmr	0, 4
	vmr	1, 4
	.p2align	5
.Lp9_int8_BB4_103:                              #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        #         Parent Loop BB4_102 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	add 9, 11, 7
	add 14, 12, 7
	add 17, 15, 7
	lxvx 50, 10, 7
	lxvx 61, 29, 7
	add 16, 24, 7
	lxv 51, -112(9)
	lxv 63, -112(17)
	lxv 62, -112(14)
	add 20, 26, 7
	add 4, 28, 7
	lxv 59, -112(20)
	add 8, 10, 7
	add 3, 29, 7
	vmsummbm 14, 18, 31, 14
	vmsummbm 10, 19, 31, 10
	vmsummbm 9, 30, 31, 9
	vmsummbm 4, 29, 31, 4
	lxv 63, -112(16)
	vmsummbm 15, 18, 27, 15
	vmsummbm 12, 19, 27, 12
	vmsummbm 6, 30, 27, 6
	vmsummbm 0, 29, 27, 0
	lxv 59, -80(4)
	vmsummbm 16, 18, 31, 16
	vmsummbm 11, 19, 31, 11
	vmsummbm 8, 30, 31, 8
	vmsummbm 5, 29, 31, 5
	lxv 63, -112(4)
	vmsummbm 17, 18, 31, 17
	vmsummbm 13, 19, 31, 13
	lxv 50, 16(8)
	vmsummbm 7, 30, 31, 7
	vmsummbm 1, 29, 31, 1
	lxv 51, -96(9)
	lxv 63, -96(14)
	lxv 62, 16(3)
	lxv 61, -96(17)
	vmsummbm 14, 18, 29, 14
	vmsummbm 10, 19, 29, 10
	vmsummbm 9, 31, 29, 9
	vmsummbm 4, 30, 29, 4
	lxv 61, -96(16)
	vmsummbm 16, 18, 29, 16
	vmsummbm 11, 19, 29, 11
	vmsummbm 8, 31, 29, 8
	vmsummbm 5, 30, 29, 5
	lxv 61, -96(20)
	vmsummbm 15, 18, 29, 15
	vmsummbm 12, 19, 29, 12
	vmsummbm 6, 31, 29, 6
	vmsummbm 0, 30, 29, 0
	lxv 61, -96(4)
	vmsummbm 17, 18, 29, 17
	lxv 50, 32(8)
	vmsummbm 13, 19, 29, 13
	vmsummbm 7, 31, 29, 7
	vmsummbm 1, 30, 29, 1
	lxv 51, -80(9)
	lxv 63, -80(14)
	lxv 62, -80(17)
	lxv 61, 32(3)
	vmsummbm 14, 18, 30, 14
	vmsummbm 10, 19, 30, 10
	vmsummbm 9, 31, 30, 9
	vmsummbm 4, 29, 30, 4
	lxv 62, -80(16)
	vmsummbm 17, 18, 27, 17
	vmsummbm 13, 19, 27, 13
	vmsummbm 7, 31, 27, 7
	vmsummbm 1, 29, 27, 1
	lxv 59, -64(20)
	vmsummbm 16, 18, 30, 16
	vmsummbm 11, 19, 30, 11
	vmsummbm 8, 31, 30, 8
	vmsummbm 5, 29, 30, 5
	lxv 62, -80(20)
	vmsummbm 15, 18, 30, 15
	vmsummbm 12, 19, 30, 12
	vmsummbm 6, 31, 30, 6
	vmsummbm 0, 29, 30, 0
	lxv 50, 48(8)
	lxv 51, -64(9)
	lxv 63, -64(14)
	lxv 62, -64(17)
	lxv 61, 48(3)
	vmsummbm 14, 18, 30, 14
	vmsummbm 10, 19, 30, 10
	vmsummbm 9, 31, 30, 9
	vmsummbm 4, 29, 30, 4
	lxv 62, -64(16)
	vmsummbm 15, 18, 27, 15
	vmsummbm 12, 19, 27, 12
	vmsummbm 6, 31, 27, 6
	vmsummbm 0, 29, 27, 0
	lxv 59, -32(4)
	vmsummbm 16, 18, 30, 16
	vmsummbm 11, 19, 30, 11
	vmsummbm 8, 31, 30, 8
	vmsummbm 5, 29, 30, 5
	lxv 62, -64(4)
	vmsummbm 17, 18, 30, 17
	vmsummbm 13, 19, 30, 13
	lxv 50, 64(8)
	vmsummbm 7, 31, 30, 7
	vmsummbm 1, 29, 30, 1
	lxv 51, -48(9)
	lxv 63, -48(14)
	lxv 62, 64(3)
	lxv 61, -48(17)
	vmsummbm 14, 18, 29, 14
	vmsummbm 10, 19, 29, 10
	vmsummbm 9, 31, 29, 9
	vmsummbm 4, 30, 29, 4
	lxv 61, -48(16)
	vmsummbm 16, 18, 29, 16
	vmsummbm 11, 19, 29, 11
	vmsummbm 8, 31, 29, 8
	vmsummbm 5, 30, 29, 5
	lxv 61, -48(20)
	vmsummbm 15, 18, 29, 15
	vmsummbm 12, 19, 29, 12
	vmsummbm 6, 31, 29, 6
	vmsummbm 0, 30, 29, 0
	lxv 61, -48(4)
	vmsummbm 17, 18, 29, 17
	lxv 50, 80(8)
	vmsummbm 13, 19, 29, 13
	vmsummbm 7, 31, 29, 7
	vmsummbm 1, 30, 29, 1
	lxv 51, -32(9)
	lxv 63, -32(14)
	lxv 62, -32(17)
	lxv 61, 80(3)
	vmsummbm 14, 18, 30, 14
	vmsummbm 10, 19, 30, 10
	vmsummbm 9, 31, 30, 9
	vmsummbm 4, 29, 30, 4
	lxv 62, -32(16)
	vmsummbm 17, 18, 27, 17
	vmsummbm 13, 19, 27, 13
	vmsummbm 7, 31, 27, 7
	vmsummbm 1, 29, 27, 1
	lxv 59, -16(20)
	vmsummbm 16, 18, 30, 16
	vmsummbm 11, 19, 30, 11
	vmsummbm 8, 31, 30, 8
	vmsummbm 5, 29, 30, 5
	lxv 62, -32(20)
	vmsummbm 15, 18, 30, 15
	vmsummbm 12, 19, 30, 12
	vmsummbm 6, 31, 30, 6
	vmsummbm 0, 29, 30, 0
	lxv 50, 96(8)
	lxv 51, -16(9)
	lxv 63, -16(14)
	lxv 62, -16(17)
	lxv 61, 96(3)
	vmsummbm 14, 18, 30, 14
	vmsummbm 10, 19, 30, 10
	vmsummbm 9, 31, 30, 9
	vmsummbm 4, 29, 30, 4
	lxv 62, -16(16)
	vmsummbm 15, 18, 27, 15
	vmsummbm 12, 19, 27, 12
	vmsummbm 6, 31, 27, 6
	vmsummbm 0, 29, 27, 0
	vmsummbm 16, 18, 30, 16
	vmsummbm 11, 19, 30, 11
	vmsummbm 8, 31, 30, 8
	vmsummbm 5, 29, 30, 5
	lxv 62, -16(4)
	vmsummbm 17, 18, 30, 17
	vmsummbm 13, 19, 30, 13
	vmsummbm 7, 31, 30, 7
	lxv 50, 112(8)
	vmsummbm 1, 29, 30, 1
	lxvx 51, 11, 7
	lxvx 63, 12, 7
	lxvx 62, 15, 7
	lxv 61, 112(3)
	vmsummbm 14, 18, 30, 14
	vmsummbm 10, 19, 30, 10
	vmsummbm 9, 31, 30, 9
	vmsummbm 4, 29, 30, 4
	lxvx 62, 24, 7
	vmsummbm 16, 18, 30, 16
	vmsummbm 11, 19, 30, 11
	vmsummbm 8, 31, 30, 8
	vmsummbm 5, 29, 30, 5
	lxvx 62, 26, 7
	vmsummbm 15, 18, 30, 15
	vmsummbm 12, 19, 30, 12
	vmsummbm 6, 31, 30, 6
	vmsummbm 0, 29, 30, 0
	lxvx 62, 28, 7
	addi 7, 7, 128
	vmsummbm 17, 18, 30, 17
	vmsummbm 13, 19, 30, 13
	vmsummbm 7, 31, 30, 7
	vmsummbm 1, 29, 30, 1
	bdnz .Lp9_int8_BB4_103
# %bb.104:                              #   in Loop: Header=BB4_102 Depth=4
	ld 7, 856(1)                            # 8-byte Folded Reload
	sldi 4, 25, 4
	lxv 50, 8192(6)
	vcfsx 14, 14, 0
	vcfsx 16, 16, 0
	lxvwsx 51, 7, 4
	add 3, 7, 4
	ld 4, 920(1)                            # 8-byte Folded Reload
	vcfsx 15, 15, 0
	vcfsx 17, 17, 0
	vcfsx 10, 10, 0
	vcfsx 11, 11, 0
	li 8, 8
	vcfsx 9, 9, 0
	vcfsx 8, 8, 0
	addi 31, 31, 1
	addi 28, 28, 8320
	addi 26, 26, 8320
	addi 24, 24, 8320
	addi 15, 15, 8320
	addi 10, 10, 1024
	addi 11, 11, 1024
	addi 12, 12, 1024
	addi 29, 29, 1024
	cmpld	31, 18
	vcfsx 6, 6, 0
	vcfsx 4, 4, 0
	vcfsx 12, 12, 0
	vcfsx 13, 13, 0
	xvmulsp 31, 51, 50
	xvmaddasp 0, 46, 31
	lxv 46, 8192(5)
	xvmulsp 31, 51, 46
	xvmaddasp 4, 48, 31
	lxv 48, 8192(4)
	ld 4, 928(1)                            # 8-byte Folded Reload
	xvmulsp 31, 51, 48
	xvmaddasp 8, 47, 31
	lxv 47, 8192(4)
	li 4, 4
	lxvwsx 31, 3, 4
	li 4, 12
	xvmulsp 51, 51, 47
	xvmaddasp 13, 49, 51
	xvmulsp 51, 31, 50
	lxvwsx 49, 3, 8
	xvmaddasp 1, 42, 51
	xvmulsp 42, 31, 46
	xvmulsp 51, 31, 48
	xvmulsp 31, 31, 47
	xvmaddasp 5, 43, 42
	xvmulsp 42, 49, 50
	xvmaddasp 9, 44, 51
	xvmaddasp 34, 45, 31
	xvmaddasp 2, 41, 42
	xvmulsp 42, 49, 46
	lxvwsx 41, 3, 4
	xvmaddasp 6, 40, 42
	xvmulsp 40, 49, 48
	xvmaddasp 10, 38, 40
	vcfsx 6, 7, 0
	xvmulsp 39, 49, 47
	xvmaddasp 35, 38, 39
	xvmulsp 38, 41, 50
	xvmaddasp 3, 36, 38
	vcfsx 4, 5, 0
	xvmulsp 37, 41, 46
	xvmaddasp 7, 36, 37
	vcfsx 4, 0, 0
	xvmulsp 37, 41, 48
	xvmaddasp 11, 36, 37
	vcfsx 4, 1, 0
	xvmulsp 37, 41, 47
	xvmaddasp 12, 36, 37
	bne	0, .Lp9_int8_BB4_102
	b .Lp9_int8_BB4_126
.Lp9_int8_BB4_105:                              #   in Loop: Header=BB4_63 Depth=3
	ld 0, 1624(1)
	xxlxor 0, 0, 0
	xxlxor 1, 1, 1
	cmpldi	0, 0
	xxlxor 2, 2, 2
	xxlxor 3, 3, 3
	xxlxor 4, 4, 4
	beq	0, .Lp9_int8_BB4_131
# %bb.106:                              #   in Loop: Header=BB4_63 Depth=3
	ld 3, 1472(1)
	mr	20, 8
	ld 25, 1600(1)
	ld 12, 1536(1)
	ld 29, 1544(1)
	xxlxor 5, 5, 5
	mr	18, 19
	ld 28, 1552(1)
	ld 26, 1560(1)
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
	li 17, 0
	li 31, -4
	xxlxor 8, 8, 8
	sldi 4, 3, 7
	sldi 5, 3, 4
	ld 3, 1480(1)
	xxlxor 9, 9, 9
	add 4, 12, 4
	add 12, 12, 5
	xxlxor 10, 10, 10
	xxlxor 11, 11, 11
	xxlxor 13, 13, 13
	sldi 6, 3, 7
	sldi 7, 3, 4
	ld 3, 1488(1)
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
	xxlxor 12, 12, 12
	add 5, 29, 6
	add 29, 29, 7
	sldi 8, 3, 7
	sldi 9, 3, 4
	ld 3, 1496(1)
	add 6, 28, 8
	add 28, 28, 9
	addi 9, 29, 1024
	ld 29, 1616(1)
	addi 8, 12, 1024
	sldi 10, 3, 7
	sldi 11, 3, 4
	add 3, 25, 24
	mulld 24, 0, 20
	add 7, 26, 10
	addi 10, 28, 1024
	ld 28, 1608(1)
	add 11, 26, 11
	add 26, 25, 19
	addi 11, 11, 1024
	sldi 12, 24, 3
	add 12, 29, 12
	sldi 29, 24, 4
	ld 24, 808(1)                           # 8-byte Folded Reload
	add 29, 28, 29
	ld 28, 832(1)                           # 8-byte Folded Reload
	mtctr 0
	addi 12, 12, 4
	addi 29, 29, 8
	add 24, 25, 24
	add 28, 25, 28
	.p2align	5
.Lp9_int8_BB4_107:                              #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lha 0, -4(12)
	lha 25, -2(12)
	lxv 33, -64(28)
	lxv 38, -64(26)
	lxvx 40, 4, 17
	add 20, 4, 17
	lxv 42, -64(24)
	lxv 45, -64(3)
	lxvx 46, 5, 17
	add 16, 6, 17
	add 15, 7, 17
	lxvx 50, 6, 17
	lxvx 61, 7, 17
	lxv 58, 48(3)
	lxv 59, 16(16)
	slwi 0, 0, 4
	lxvx 30, 9, 17
	neg 0, 0
	mtvsrws 36, 0
	slwi 0, 25, 4
	lha 25, 0(12)
	neg 0, 0
	vmsummbm 9, 1, 8, 4
	vmsummbm 15, 1, 14, 4
	vmsummbm 19, 1, 18, 4
	mtvsrws 37, 0
	lha 0, 2(12)
	vmsummbm 4, 1, 29, 4
	addi 12, 12, 8
	slwi 25, 25, 4
	vmsummbm 11, 6, 8, 5
	vmsummbm 16, 6, 14, 5
	vmsummbm 31, 6, 18, 5
	vmsummbm 5, 6, 29, 5
	lxv 38, -48(28)
	neg 25, 25
	slwi 0, 0, 4
	mtvsrws 32, 25
	add 25, 5, 17
	neg 0, 0
	vmsummbm 12, 10, 8, 0
	vmsummbm 17, 10, 14, 0
	vmsummbm 30, 10, 18, 0
	vmsummbm 0, 10, 29, 0
	lxv 42, -48(24)
	mtvsrws 39, 0
	li 0, -8
	vmsummbm 8, 13, 8, 7
	vmsummbm 14, 13, 14, 7
	vmsummbm 18, 13, 18, 7
	vmsummbm 1, 13, 29, 7
	lxv 39, -48(26)
	lxv 45, 16(20)
	lxv 61, -48(3)
	vmsummbm 9, 6, 13, 9
	vmsummbm 11, 7, 13, 11
	vmsummbm 12, 10, 13, 12
	vmsummbm 8, 29, 13, 8
	lxv 45, 16(25)
	vmsummbm 18, 29, 27, 18
	vmsummbm 15, 6, 13, 15
	vmsummbm 16, 7, 13, 16
	vmsummbm 17, 10, 13, 17
	vmsummbm 13, 29, 13, 14
	vmsummbm 14, 6, 27, 19
	vmsummbm 19, 7, 27, 31
	vmsummbm 31, 10, 27, 30
	lxv 62, 16(15)
	lxv 59, 48(15)
	vmsummbm 4, 6, 30, 4
	vmsummbm 5, 7, 30, 5
	vmsummbm 0, 10, 30, 0
	vmsummbm 1, 29, 30, 1
	lxv 38, -32(28)
	lxv 39, -32(26)
	lxv 42, -32(24)
	lxv 62, -32(3)
	lxv 61, 32(20)
	vmsummbm 9, 6, 29, 9
	vmsummbm 11, 7, 29, 11
	vmsummbm 12, 10, 29, 12
	vmsummbm 8, 30, 29, 8
	lxv 61, 32(25)
	vmsummbm 15, 6, 29, 15
	vmsummbm 16, 7, 29, 16
	vmsummbm 17, 10, 29, 17
	vmsummbm 13, 30, 29, 13
	lxv 61, 32(16)
	vmsummbm 14, 6, 29, 14
	vmsummbm 19, 7, 29, 19
	vmsummbm 31, 10, 29, 31
	vmsummbm 18, 30, 29, 18
	lxv 61, 32(15)
	vmsummbm 4, 6, 29, 4
	vmsummbm 5, 7, 29, 5
	lxv 38, -16(28)
	vmsummbm 0, 10, 29, 0
	vmsummbm 1, 30, 29, 1
	lxv 39, -16(26)
	lxv 42, -16(24)
	lxv 62, 48(20)
	lxv 61, -16(3)
	vmsummbm 9, 6, 30, 9
	vmsummbm 11, 7, 30, 11
	vmsummbm 12, 10, 30, 12
	vmsummbm 8, 29, 30, 8
	lxv 62, 48(25)
	vmsummbm 4, 6, 27, 4
	vmsummbm 5, 7, 27, 5
	vmsummbm 0, 10, 27, 0
	vmsummbm 1, 29, 27, 1
	lxv 59, 64(16)
	vmsummbm 15, 6, 30, 15
	vmsummbm 16, 7, 30, 16
	vmsummbm 17, 10, 30, 17
	vmsummbm 13, 29, 30, 13
	lxv 62, 48(16)
	vmsummbm 14, 6, 30, 14
	lxv 38, 0(28)
	vmsummbm 19, 7, 30, 19
	vmsummbm 31, 10, 30, 31
	vmsummbm 18, 29, 30, 18
	lxv 39, 0(26)
	lxv 42, 0(24)
	lxv 62, 64(20)
	lxv 61, 0(3)
	vmsummbm 9, 6, 30, 9
	vmsummbm 11, 7, 30, 11
	vmsummbm 12, 10, 30, 12
	vmsummbm 8, 29, 30, 8
	lxv 62, 64(25)
	vmsummbm 14, 6, 27, 14
	vmsummbm 19, 7, 27, 19
	vmsummbm 31, 10, 27, 31
	vmsummbm 18, 29, 27, 18
	lxv 59, 96(15)
	vmsummbm 15, 6, 30, 15
	vmsummbm 16, 7, 30, 16
	vmsummbm 17, 10, 30, 17
	vmsummbm 13, 29, 30, 13
	lxv 62, 64(15)
	vmsummbm 4, 6, 30, 4
	vmsummbm 5, 7, 30, 5
	vmsummbm 0, 10, 30, 0
	vmsummbm 1, 29, 30, 1
	lxv 38, 16(28)
	lxv 39, 16(26)
	lxv 42, 16(24)
	lxv 62, 16(3)
	lxv 61, 80(20)
	vmsummbm 9, 6, 29, 9
	vmsummbm 11, 7, 29, 11
	vmsummbm 12, 10, 29, 12
	vmsummbm 8, 30, 29, 8
	lxv 61, 80(25)
	vmsummbm 15, 6, 29, 15
	vmsummbm 16, 7, 29, 16
	vmsummbm 17, 10, 29, 17
	vmsummbm 13, 30, 29, 13
	lxv 61, 80(16)
	vmsummbm 14, 6, 29, 14
	vmsummbm 19, 7, 29, 19
	vmsummbm 31, 10, 29, 31
	vmsummbm 18, 30, 29, 18
	lxv 61, 80(15)
	vmsummbm 4, 6, 29, 4
	vmsummbm 5, 7, 29, 5
	lxv 38, 32(28)
	vmsummbm 0, 10, 29, 0
	vmsummbm 1, 30, 29, 1
	lxv 39, 32(26)
	lxv 42, 32(24)
	lxv 62, 96(20)
	lxv 61, 32(3)
	addi 3, 3, 128
	vmsummbm 9, 6, 30, 9
	vmsummbm 11, 7, 30, 11
	vmsummbm 12, 10, 30, 12
	vmsummbm 8, 29, 30, 8
	lxv 62, 96(25)
	vmsummbm 0, 10, 27, 0
	vmsummbm 15, 6, 30, 15
	vmsummbm 16, 7, 30, 16
	vmsummbm 17, 10, 30, 17
	vmsummbm 13, 29, 30, 13
	lxv 62, 96(16)
	vmsummbm 14, 6, 30, 14
	vmsummbm 19, 7, 30, 19
	vmsummbm 31, 10, 30, 31
	vmsummbm 18, 29, 30, 18
	vmsummbm 6, 6, 27, 4
	vmsummbm 7, 7, 27, 5
	vmsummbm 10, 29, 27, 1
	lxv 62, 48(28)
	lxv 61, 48(26)
	lxv 59, 48(24)
	lxv 36, 112(20)
	lxv 37, 112(25)
	li 25, 4
	addi 28, 28, 128
	addi 26, 26, 128
	addi 24, 24, 128
	vmsummbm 9, 30, 4, 9
	vmsummbm 11, 29, 4, 11
	vmsummbm 12, 27, 4, 12
	vmsummbm 4, 26, 4, 8
	vmsummbm 8, 30, 5, 15
	vmsummbm 15, 29, 5, 16
	vmsummbm 16, 27, 5, 17
	lxv 49, 112(16)
	vmsummbm 1, 26, 5, 13
	vmsummbm 13, 30, 17, 14
	vmsummbm 14, 29, 17, 19
	vmsummbm 19, 27, 17, 31
	vmsummbm 5, 26, 17, 18
	lxv 49, 112(15)
	vcfsx 9, 9, 0
	vcfsx 8, 8, 0
	vcfsx 11, 11, 0
	vcfsx 4, 4, 0
	vcfsx 1, 1, 0
	vmsummbm 6, 30, 17, 6
	vmsummbm 7, 29, 17, 7
	vmsummbm 18, 27, 17, 0
	vmsummbm 0, 26, 17, 10
	lxvwsx 42, 29, 0
	lxvx 49, 8, 17
	vcfsx 5, 5, 0
	xvmulsp 31, 42, 49
	xvmulsp 29, 42, 30
	vcfsx 6, 6, 0
	vcfsx 7, 7, 0
	xvmaddasp 0, 41, 31
	vcfsx 9, 13, 0
	lxvx 45, 10, 17
	xvmaddasp 4, 40, 29
	lxvx 40, 11, 17
	addi 17, 17, 1152
	xvmulsp 31, 42, 45
	xvmulsp 42, 42, 40
	xvmaddasp 8, 41, 31
	lxvwsx 41, 29, 31
	xvmaddasp 13, 38, 42
	vcfsx 6, 15, 0
	xvmulsp 42, 41, 30
	xvmulsp 31, 41, 49
	xvmaddasp 5, 38, 42
	vcfsx 6, 14, 0
	xvmulsp 42, 41, 45
	xvmaddasp 1, 43, 31
	xvmaddasp 9, 38, 42
	xvmulsp 38, 41, 40
	lxvwsx 41, 0, 29
	vcfsx 10, 12, 0
	xvmulsp 43, 41, 30
	xvmaddasp 34, 39, 38
	xvmulsp 38, 41, 49
	vcfsx 7, 16, 0
	xvmaddasp 2, 42, 38
	xvmaddasp 6, 39, 43
	vcfsx 6, 19, 0
	xvmulsp 39, 41, 45
	xvmaddasp 10, 38, 39
	vcfsx 6, 18, 0
	xvmulsp 39, 41, 40
	lxvwsx 41, 29, 25
	addi 29, 29, 16
	xvmaddasp 35, 38, 39
	xvmulsp 38, 41, 49
	xvmaddasp 3, 36, 38
	xvmulsp 36, 41, 30
	xvmulsp 38, 41, 45
	xvmaddasp 7, 33, 36
	vcfsx 4, 0, 0
	xvmulsp 32, 41, 40
	xvmaddasp 11, 37, 38
	xvmaddasp 12, 36, 32
	bdnz .Lp9_int8_BB4_107
	b .Lp9_int8_BB4_132
.Lp9_int8_BB4_108:                              #   in Loop: Header=BB4_63 Depth=3
	ld 3, 1624(1)
	cmpldi	3, 0
	beq	0, .Lp9_int8_BB4_137
# %bb.109:                              #   in Loop: Header=BB4_63 Depth=3
	ld 4, 1472(1)
	ld 5, 1480(1)
	ld 7, 1536(1)
	mr	0, 8
	ld 8, 1544(1)
	xxlxor 0, 0, 0
	li 26, 0
	ld 6, 1616(1)
	ld 11, 1600(1)
	ld 12, 1608(1)
	xxlxor 1, 1, 1
	xxlxor 2, 2, 2
	mulld 0, 3, 0
	xxlxor 3, 3, 3
	xxlxor 5, 5, 5
	li 14, 8
	mulli 29, 4, 224
	mulli 28, 5, 224
	sldi 4, 4, 9
	sldi 5, 5, 9
	cmpdi	6, 0
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
	std 6, 920(1)                           # 8-byte Folded Spill
	add 4, 7, 4
	add 5, 8, 5
	xxlxor 4, 4, 4
	addi 9, 4, 32
	addi 10, 5, 32
	add 4, 7, 29
	add 5, 8, 28
	std 29, 928(1)                          # 8-byte Folded Spill
	mr	6, 28
	addi 29, 4, 2144
	addi 28, 5, 2144
	.p2align	4
.Lp9_int8_BB4_110:                              #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB4_111 Depth 5
	mulli 4, 26, 2944
	ld 25, 928(1)                           # 8-byte Folded Reload
	mtctr 14
	mr	20, 11
	mr	17, 10
	mr	16, 9
	xxlxor 34, 34, 34
	ld 21, 944(1)                           # 8-byte Folded Reload
	ld 18, 952(1)                           # 8-byte Folded Reload
	ld 19, 936(1)                           # 8-byte Folded Reload
	li 15, 0
	xxlxor 36, 36, 36
	add 5, 7, 4
	add 4, 8, 4
	xxlxor 32, 32, 32
	xxlxor 38, 38, 38
	xxlxor 35, 35, 35
	xxlxor 37, 37, 37
	xxlxor 33, 33, 33
	xxlxor 39, 39, 39
	add 24, 5, 25
	add 25, 4, 6
	.p2align	5
.Lp9_int8_BB4_111:                              #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        #         Parent Loop BB4_110 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	add 14, 20, 27
	add 31, 20, 19
	add 4, 20, 18
	add 5, 20, 21
	lxv 49, -32(16)
	lxvx 41, 20, 27
	lxv 42, 16(14)
	lxv 43, -112(31)
	lxv 44, -96(31)
	lxv 45, -112(4)
	lxv 46, -96(4)
	lxv 47, -112(5)
	lxv 48, -96(5)
	xxspltib 8, 15
	xxland 40, 49, 8
	vmsummbm 18, 9, 8, 28
	vmsummbm 19, 11, 8, 28
	vmsummbm 31, 13, 8, 28
	vmsummbm 30, 15, 8, 28
	xxspltib 40, 4
	lxv 61, -32(17)
	lxv 58, -16(16)
	lxv 56, -64(5)
	vsrb 17, 17, 8
	xxland 59, 61, 8
	vmsummbm 9, 9, 27, 28
	vmsummbm 11, 11, 27, 28
	vmsummbm 13, 13, 27, 28
	vmsummbm 15, 15, 27, 28
	vmsummbm 18, 10, 17, 18
	vmsummbm 19, 12, 17, 19
	vmsummbm 31, 14, 17, 31
	vmsummbm 17, 16, 17, 30
	vsrb 30, 29, 8
	lxv 59, -80(4)
	lxv 61, -64(31)
	vmsummbm 9, 10, 30, 9
	vmsummbm 10, 12, 30, 11
	vmsummbm 11, 14, 30, 13
	lxv 44, 32(14)
	lxv 46, -80(31)
	vmsummbm 15, 16, 30, 15
	lxv 62, -80(5)
	lxv 45, 48(14)
	xxland 48, 58, 8
	vsrb 26, 26, 8
	vmsummbm 31, 27, 16, 31
	vmsummbm 18, 12, 16, 18
	vmsummbm 19, 14, 16, 19
	vmsummbm 16, 30, 16, 17
	lxv 49, -16(17)
	xxland 57, 49, 8
	vmsummbm 10, 14, 25, 10
	vmsummbm 9, 12, 25, 9
	lxv 44, -64(4)
	vmsummbm 11, 27, 25, 11
	vmsummbm 15, 30, 25, 15
	vmsummbm 14, 24, 26, 16
	vsrb 16, 17, 8
	vmsummbm 18, 13, 26, 18
	vmsummbm 19, 29, 26, 19
	lxv 57, 0(17)
	lxv 49, -48(31)
	lxv 62, -32(31)
	vmsummbm 31, 12, 26, 31
	lxv 58, -48(5)
	vmsummbm 10, 29, 16, 10
	lxv 61, 0(16)
	vmsummbm 9, 13, 16, 9
	vmsummbm 11, 12, 16, 11
	lxv 44, 64(14)
	vmsummbm 15, 24, 16, 15
	lxv 48, -48(4)
	xxland 56, 57, 8
	lxv 45, 80(14)
	xxland 59, 61, 8
	vsrb 29, 29, 8
	vmsummbm 18, 12, 27, 18
	vmsummbm 31, 16, 27, 31
	vmsummbm 19, 17, 27, 19
	vmsummbm 14, 26, 27, 14
	lxv 59, -32(4)
	vmsummbm 9, 12, 24, 9
	vmsummbm 11, 16, 24, 11
	lxv 44, -32(5)
	vmsummbm 10, 17, 24, 10
	vmsummbm 15, 26, 24, 15
	vsrb 16, 25, 8
	lxv 56, 16(16)
	lxv 57, -16(5)
	lxv 49, 112(14)
	lxvx 58, 20, 18
	addi 16, 16, 64
	xxland 55, 56, 8
	vmsummbm 18, 13, 29, 18
	vmsummbm 31, 27, 29, 31
	vmsummbm 9, 13, 16, 9
	lxv 45, 96(14)
	vmsummbm 11, 27, 16, 11
	lxv 59, -16(4)
	vmsummbm 19, 30, 29, 19
	vmsummbm 14, 12, 29, 14
	vmsummbm 10, 30, 16, 10
	lxv 62, -16(31)
	lxvx 61, 20, 19
	vmsummbm 12, 12, 16, 15
	vmsummbm 15, 13, 23, 18
	vmsummbm 18, 27, 23, 31
	vsrb 31, 24, 8
	lxv 56, 16(17)
	vmsummbm 16, 30, 23, 19
	vmsummbm 14, 25, 23, 14
	lxvx 51, 20, 21
	addi 17, 17, 64
	addi 20, 20, 128
	xxland 55, 56, 8
	vsrb 8, 24, 8
	vmsummbm 9, 13, 23, 9
	vmsummbm 10, 30, 23, 10
	vmsummbm 11, 27, 23, 11
	vmsummbm 12, 25, 23, 12
	vmsummbm 13, 17, 31, 15
	vmsummbm 15, 29, 31, 16
	vmsummbm 16, 26, 31, 18
	vmsummbm 14, 19, 31, 14
	vmsummbm 9, 17, 8, 9
	vmsummbm 10, 29, 8, 10
	vmsummbm 11, 26, 8, 11
	vmsummbm 8, 19, 8, 12
	lxvx 44, 29, 15
	vmuluwm 13, 13, 12
	vmuluwm 15, 15, 12
	vadduwm 7, 13, 7
	vmuluwm 13, 16, 12
	vmuluwm 12, 14, 12
	vadduwm 1, 15, 1
	vadduwm 3, 12, 3
	lxvx 44, 28, 15
	addi 15, 15, 16
	vadduwm 5, 13, 5
	vmuluwm 9, 9, 12
	vmuluwm 8, 8, 12
	vadduwm 6, 9, 6
	vmuluwm 9, 10, 12
	vadduwm 2, 8, 2
	vadduwm 0, 9, 0
	vmuluwm 9, 11, 12
	vadduwm 4, 9, 4
	bdnz .Lp9_int8_BB4_111
# %bb.112:                              #   in Loop: Header=BB4_110 Depth=4
	add 4, 26, 0
	ld 21, 920(1)                           # 8-byte Folded Reload
	lxv 9, 2048(25)
	vcfsx 6, 6, 0
	li 17, 4
	lxv 11, 2048(24)
	lxv 8, 2064(25)
	vcfsx 1, 1, 0
	vcfsx 7, 7, 0
	lxv 40, 2128(25)
	lxv 41, 2112(25)
	lxv 42, 2096(25)
	lxv 43, 2080(25)
	sldi 5, 4, 6
	vcfsx 0, 0, 0
	li 14, 8
	vcfsx 5, 5, 0
	lxv 46, 2080(24)
	lxv 45, 2096(24)
	vcfsx 4, 4, 0
	lxv 44, 2112(24)
	lxv 10, 2064(24)
	lxv 49, 2128(24)
	li 18, 16
	li 16, 32
	li 24, 48
	add 20, 21, 5
	sldi 5, 4, 4
	vcfsx 3, 3, 0
	lxvwsx 12, 12, 5
	add 4, 12, 5
	iseleq	5, 0, 20
	addi 25, 20, 4
	addi 21, 20, 8
	vcfsx 2, 2, 0
	lxvwsx 13, 4, 17
	li 17, 0
	addi 26, 26, 1
	addi 29, 29, 2944
	addi 28, 28, 2944
	addi 9, 9, 2944
	addi 10, 10, 2944
	addi 11, 11, 1024
	cmpld 1, 26, 3
	iseleq	25, 17, 25
	iseleq	21, 17, 21
	xvmulsp 48, 12, 9
	xvmulsp 47, 12, 11
	lxvwsx 50, 21, 16
	lxvwsx 51, 21, 24
	xvmaddasp 5, 38, 48
	xvmulsp 48, 13, 11
	xvmaddasp 0, 39, 47
	lxvwsx 39, 4, 14
	lxvwsx 47, 0, 5
	lxvwsx 38, 5, 18
	xvmaddasp 1, 33, 48
	lxvwsx 33, 0, 25
	xvmulsp 48, 13, 9
	xvmaddasp 6, 32, 48
	xvmulsp 48, 39, 11
	lxvwsx 32, 25, 18
	xvmaddasp 2, 37, 48
	xvmulsp 48, 39, 9
	lxvwsx 37, 0, 21
	xvmaddasp 7, 36, 48
	vmsumshm 4, 14, 15, 28
	vmsumshm 15, 11, 15, 28
	vmsumshm 16, 14, 1, 28
	vmsumshm 1, 11, 1, 28
	vmsumshm 4, 13, 6, 4
	vmsumshm 6, 10, 6, 15
	vmsumshm 15, 14, 5, 28
	vmsumshm 5, 11, 5, 28
	vmsumshm 16, 13, 0, 16
	vmsumshm 0, 10, 0, 1
	lxvwsx 33, 21, 18
	vmsumshm 15, 13, 1, 15
	vmsumshm 5, 10, 1, 5
	lxvwsx 33, 5, 16
	vmsumshm 4, 12, 1, 4
	vmsumshm 1, 9, 1, 6
	lxvwsx 38, 25, 16
	vmsumshm 15, 12, 18, 15
	vmsumshm 5, 9, 18, 5
	lxvwsx 50, 25, 24
	li 25, 12
	vmsumshm 16, 12, 6, 16
	vmsumshm 0, 9, 6, 0
	lxvwsx 38, 5, 24
	addi 5, 20, 12
	vmsumshm 5, 8, 19, 5
	vmsumshm 15, 17, 19, 15
	xvmulsp 51, 39, 10
	xvmulsp 39, 39, 8
	vmsumshm 1, 8, 6, 1
	vmsumshm 4, 17, 6, 4
	xvmulsp 38, 12, 10
	xvmulsp 12, 12, 8
	vmsumshm 0, 8, 18, 0
	vmsumshm 16, 17, 18, 16
	xvmulsp 50, 13, 10
	xvmulsp 13, 13, 8
	vcfsx 5, 5, 0
	vcfsx 15, 15, 0
	vcfsx 1, 1, 0
	vcfsx 4, 4, 0
	vcfsx 0, 0, 0
	vcfsx 16, 16, 0
	xvnmsubasp 7, 37, 39
	xvnmsubasp 2, 47, 51
	xvnmsubasp 5, 33, 12
	lxvwsx 12, 4, 25
	iseleq	4, 17, 5
	xvnmsubasp 0, 36, 38
	lxvwsx 37, 4, 18
	xvnmsubasp 6, 32, 13
	lxvwsx 32, 4, 16
	lxvwsx 33, 4, 24
	xvnmsubasp 1, 48, 50
	xvmulsp 11, 12, 11
	xvmulsp 9, 12, 9
	xvmulsp 10, 12, 10
	xvmulsp 8, 12, 8
	xvmaddasp 3, 35, 11
	lxvwsx 35, 0, 4
	xvmaddasp 4, 34, 9
	vmsumshm 4, 14, 3, 28
	vmsumshm 2, 11, 3, 28
	vmsumshm 4, 13, 5, 4
	vmsumshm 2, 10, 5, 2
	vmsumshm 4, 12, 0, 4
	vmsumshm 2, 9, 0, 2
	vmsumshm 4, 17, 1, 4
	vmsumshm 2, 8, 1, 2
	vcfsx 4, 4, 0
	vcfsx 2, 2, 0
	xvnmsubasp 3, 36, 10
	xvnmsubasp 4, 34, 8
	bne	1, .Lp9_int8_BB4_110
	b .Lp9_int8_BB4_138
.Lp9_int8_BB4_113:                              #   in Loop: Header=BB4_63 Depth=3
	ld 0, 1624(1)
	std 21, 536(1)                          # 8-byte Folded Spill
	cmpldi	0, 0
	beq	0, .Lp9_int8_BB4_139
# %bb.114:                              #   in Loop: Header=BB4_63 Depth=3
	ld 7, 1600(1)
	ld 3, 1472(1)
	ld 4, 1480(1)
	ld 5, 1488(1)
	mulld 14, 0, 8
	ld 25, 1544(1)
	ld 6, 1496(1)
	ld 24, 1552(1)
	ld 21, 1560(1)
	xxlxor 0, 0, 0
	xxlxor 1, 1, 1
	xxlxor 2, 2, 2
	li 31, 0
	xxlxor 3, 3, 3
	xxlxor 4, 4, 4
	add 28, 7, 12
	ld 12, 1536(1)
	mulli 9, 3, 288
	sldi 3, 3, 10
	mulli 11, 4, 288
	sldi 4, 4, 10
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
	xxlxor 5, 5, 5
	mulli 29, 5, 288
	add 4, 25, 4
	sldi 5, 5, 10
	mulli 26, 6, 288
	add 5, 24, 5
	add 3, 12, 3
	sldi 6, 6, 10
	add 6, 21, 6
	std 25, 888(1)                          # 8-byte Folded Spill
	std 21, 760(1)                          # 8-byte Folded Spill
	addi 3, 3, 48
	std 29, 912(1)                          # 8-byte Folded Spill
	std 26, 904(1)                          # 8-byte Folded Spill
	std 24, 880(1)                          # 8-byte Folded Spill
	std 11, 920(1)                          # 8-byte Folded Spill
	std 9, 928(1)                           # 8-byte Folded Spill
	std 12, 896(1)                          # 8-byte Folded Spill
	std 3, 872(1)                           # 8-byte Folded Spill
	addi 3, 4, 48
	add 4, 24, 29
	add 29, 7, 10
	std 3, 864(1)                           # 8-byte Folded Spill
	addi 3, 5, 48
	add 5, 25, 11
	ld 25, 1608(1)
	std 3, 856(1)                           # 8-byte Folded Spill
	addi 3, 6, 48
	add 6, 12, 9
	std 3, 848(1)                           # 8-byte Folded Spill
	add 3, 21, 26
	addi 3, 3, 4128
	std 3, 752(1)                           # 8-byte Folded Spill
	addi 3, 4, 4128
	std 3, 744(1)                           # 8-byte Folded Spill
	addi 3, 5, 4128
	std 3, 736(1)                           # 8-byte Folded Spill
	addi 3, 6, 4128
	std 3, 728(1)                           # 8-byte Folded Spill
	li 3, 0
	.p2align	4
.Lp9_int8_BB4_115:                              #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB4_116 Depth 5
	mulli 4, 3, 5248
	ld 5, 896(1)                            # 8-byte Folded Reload
	ld 6, 928(1)                            # 8-byte Folded Reload
	ld 8, 912(1)                            # 8-byte Folded Reload
	li 11, 16
	ld 26, 848(1)                           # 8-byte Folded Reload
	ld 9, 856(1)                            # 8-byte Folded Reload
	ld 10, 864(1)                           # 8-byte Folded Reload
	ld 24, 872(1)                           # 8-byte Folded Reload
	mr	15, 29
	add 5, 5, 4
	add 7, 5, 6
	ld 5, 888(1)                            # 8-byte Folded Reload
	ld 6, 920(1)                            # 8-byte Folded Reload
	lxv 34, 4112(7)
	add 5, 5, 4
	add 6, 5, 6
	ld 5, 880(1)                            # 8-byte Folded Reload
	vnegw 5, 2
	lxv 34, 4112(6)
	vmr	7, 5
	add 5, 5, 4
	add 5, 5, 8
	ld 8, 760(1)                            # 8-byte Folded Reload
	vnegw 4, 2
	lxv 34, 4112(5)
	vmr	6, 4
	add 4, 8, 4
	ld 8, 904(1)                            # 8-byte Folded Reload
	vnegw 3, 2
	add 4, 4, 8
	mr	8, 28
	vmr	1, 3
	lxv 34, 4112(4)
	mtctr 11
	ld 16, 728(1)                           # 8-byte Folded Reload
	ld 17, 736(1)                           # 8-byte Folded Reload
	ld 20, 744(1)                           # 8-byte Folded Reload
	ld 11, 752(1)                           # 8-byte Folded Reload
	vnegw 2, 2
	vmr	0, 2
	.p2align	5
.Lp9_int8_BB4_116:                              #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        #         Parent Loop BB4_115 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	add 21, 24, 31
	lxv 40, -32(8)
	lxv 41, -32(15)
	add 19, 10, 31
	add 18, 9, 31
	add 12, 26, 31
	lxv 49, -16(15)
	lxv 42, -48(21)
	lxv 44, -48(19)
	lxv 46, -48(18)
	lxv 48, -48(12)
	lxv 50, -32(21)
	vmsummbm 11, 10, 8, 28
	vmsummbm 10, 10, 9, 28
	vmsummbm 13, 12, 8, 28
	vmsummbm 12, 12, 9, 28
	vmsummbm 15, 14, 8, 28
	vmsummbm 14, 14, 9, 28
	vmsummbm 8, 16, 8, 28
	vmsummbm 9, 16, 9, 28
	lxv 48, -16(8)
	vmsummbm 11, 18, 16, 11
	vmsummbm 10, 18, 17, 10
	lxv 50, -32(19)
	vmsummbm 13, 18, 16, 13
	vmsummbm 12, 18, 17, 12
	lxv 50, -32(18)
	vmsummbm 15, 18, 16, 15
	vmsummbm 14, 18, 17, 14
	lxv 50, -32(12)
	vmsummbm 8, 18, 16, 8
	lxv 48, 0(8)
	vmsummbm 9, 18, 17, 9
	lxv 49, 0(15)
	lxv 50, -16(21)
	vmsummbm 11, 18, 16, 11
	vmsummbm 10, 18, 17, 10
	lxv 50, -16(19)
	vmsummbm 13, 18, 16, 13
	vmsummbm 12, 18, 17, 12
	lxv 50, -16(18)
	vmsummbm 15, 18, 16, 15
	vmsummbm 14, 18, 17, 14
	lxv 50, -16(12)
	vmsummbm 8, 18, 16, 8
	vmsummbm 9, 18, 17, 9
	lxv 48, 16(8)
	lxv 49, 16(15)
	lxvx 50, 24, 31
	addi 24, 24, 64
	addi 8, 8, 64
	addi 15, 15, 64
	vmsummbm 11, 18, 16, 11
	vmsummbm 10, 18, 17, 10
	lxvx 50, 10, 31
	addi 10, 10, 64
	vmsummbm 13, 18, 16, 13
	vmsummbm 12, 18, 17, 12
	lxvx 50, 9, 31
	addi 9, 9, 64
	vmsummbm 15, 18, 16, 15
	vmsummbm 14, 18, 17, 14
	lxvx 50, 26, 31
	addi 26, 26, 64
	vmsummbm 8, 18, 16, 8
	lxvx 48, 16, 31
	vmsummbm 9, 18, 17, 9
	addi 16, 16, 16
	vmuluwm 10, 10, 16
	vmuluwm 11, 11, 16
	vadduwm 5, 10, 5
	lxvx 42, 17, 31
	vadduwm 7, 11, 7
	addi 17, 17, 16
	vmuluwm 11, 13, 10
	vmuluwm 10, 12, 10
	vadduwm 4, 10, 4
	lxvx 42, 20, 31
	addi 20, 20, 16
	vadduwm 6, 11, 6
	vmuluwm 11, 15, 10
	vmuluwm 10, 14, 10
	vadduwm 1, 11, 1
	lxvx 43, 11, 31
	addi 11, 11, 16
	vadduwm 3, 10, 3
	vmuluwm 8, 8, 11
	vmuluwm 9, 9, 11
	vadduwm 0, 8, 0
	vadduwm 2, 9, 2
	bdnz .Lp9_int8_BB4_116
# %bb.117:                              #   in Loop: Header=BB4_115 Depth=4
	add 8, 3, 14
	lxv 9, 4096(7)
	vcfsx 7, 7, 0
	vcfsx 6, 6, 0
	addi 3, 3, 1
	addi 31, 31, 5248
	vcfsx 1, 1, 0
	addi 28, 28, 1024
	addi 29, 29, 1024
	sldi 8, 8, 4
	lxvwsx 8, 25, 8
	vcfsx 0, 0, 0
	vcfsx 5, 5, 0
	vcfsx 4, 4, 0
	vcfsx 3, 3, 0
	cmpld	3, 0
	vcfsx 2, 2, 0
	xvmulsp 10, 8, 9
	xvmaddasp 0, 39, 10
	lxv 10, 4096(6)
	xvmulsp 11, 8, 10
	xvmaddasp 2, 38, 11
	lxv 11, 4096(5)
	li 5, 4
	xvmulsp 12, 8, 11
	xvmaddasp 4, 33, 12
	lxv 12, 4096(4)
	add 4, 25, 8
	xvmulsp 8, 8, 12
	xvmaddasp 7, 32, 8
	lxvwsx 8, 4, 5
	xvmulsp 9, 8, 9
	xvmaddasp 1, 37, 9
	xvmulsp 9, 8, 10
	xvmaddasp 3, 36, 9
	xvmulsp 9, 8, 11
	xvmulsp 8, 8, 12
	xvmaddasp 6, 35, 9
	xvmaddasp 5, 34, 8
	bne	0, .Lp9_int8_BB4_115
	b .Lp9_int8_BB4_140
.Lp9_int8_BB4_118:                              #   in Loop: Header=BB4_63 Depth=3
	std 0, 776(1)                           # 8-byte Folded Spill
	ld 0, 1624(1)
	xxlxor 0, 0, 0
	xxlxor 1, 1, 1
	std 20, 768(1)                          # 8-byte Folded Spill
	cmpldi	0, 0
	xxlxor 2, 2, 2
	xxlxor 3, 3, 3
	xxlxor 4, 4, 4
	std 26, 808(1)                          # 8-byte Folded Spill
	std 28, 792(1)                          # 8-byte Folded Spill
	std 29, 800(1)                          # 8-byte Folded Spill
	std 24, 784(1)                          # 8-byte Folded Spill
	beq	0, .Lp9_int8_BB4_142
# %bb.119:                              #   in Loop: Header=BB4_63 Depth=3
	ld 3, 1472(1)
	mr	20, 8
	ld 25, 1600(1)
	ld 12, 1536(1)
	ld 29, 1544(1)
	xxlxor 5, 5, 5
	mr	18, 19
	ld 28, 1552(1)
	ld 26, 1560(1)
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
	li 17, 0
	li 31, -4
	xxlxor 8, 8, 8
	sldi 4, 3, 7
	sldi 5, 3, 4
	ld 3, 1480(1)
	xxlxor 9, 9, 9
	add 4, 12, 4
	add 12, 12, 5
	xxlxor 10, 10, 10
	xxlxor 11, 11, 11
	xxlxor 12, 12, 12
	sldi 6, 3, 7
	sldi 7, 3, 4
	ld 3, 1488(1)
	xxlxor 13, 13, 13
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
	add 5, 29, 6
	add 29, 29, 7
	sldi 8, 3, 7
	sldi 9, 3, 4
	ld 3, 1496(1)
	add 6, 28, 8
	add 28, 28, 9
	addi 9, 29, 1024
	ld 29, 1616(1)
	addi 8, 12, 1024
	sldi 10, 3, 7
	sldi 11, 3, 4
	add 3, 25, 24
	mulld 24, 0, 20
	add 7, 26, 10
	addi 10, 28, 1024
	ld 28, 1608(1)
	add 11, 26, 11
	add 26, 25, 19
	addi 11, 11, 1024
	sldi 12, 24, 3
	add 12, 29, 12
	sldi 29, 24, 4
	ld 24, 808(1)                           # 8-byte Folded Reload
	add 29, 28, 29
	ld 28, 832(1)                           # 8-byte Folded Reload
	mtctr 0
	addi 12, 12, 4
	addi 29, 29, 8
	add 24, 25, 24
	add 28, 25, 28
	.p2align	5
.Lp9_int8_BB4_120:                              #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lha 0, -4(12)
	lha 25, -2(12)
	lxv 33, -64(28)
	lxv 38, -64(26)
	lxvx 40, 4, 17
	add 20, 4, 17
	lxv 42, -64(24)
	lxv 45, -64(3)
	lxvx 46, 5, 17
	add 16, 6, 17
	add 15, 7, 17
	lxvx 50, 6, 17
	lxvx 61, 7, 17
	lxv 58, 48(3)
	lxv 59, 16(16)
	slwi 0, 0, 7
	lxvx 30, 9, 17
	neg 0, 0
	mtvsrws 36, 0
	slwi 0, 25, 7
	lha 25, 0(12)
	neg 0, 0
	vmsummbm 9, 1, 8, 4
	vmsummbm 15, 1, 14, 4
	vmsummbm 19, 1, 18, 4
	mtvsrws 37, 0
	lha 0, 2(12)
	vmsummbm 4, 1, 29, 4
	addi 12, 12, 8
	slwi 25, 25, 7
	vmsummbm 11, 6, 8, 5
	vmsummbm 16, 6, 14, 5
	vmsummbm 31, 6, 18, 5
	vmsummbm 5, 6, 29, 5
	lxv 38, -48(28)
	neg 25, 25
	slwi 0, 0, 7
	mtvsrws 32, 25
	add 25, 5, 17
	neg 0, 0
	vmsummbm 12, 10, 8, 0
	vmsummbm 17, 10, 14, 0
	vmsummbm 30, 10, 18, 0
	vmsummbm 0, 10, 29, 0
	lxv 42, -48(24)
	mtvsrws 39, 0
	li 0, -8
	vmsummbm 8, 13, 8, 7
	vmsummbm 14, 13, 14, 7
	vmsummbm 18, 13, 18, 7
	vmsummbm 1, 13, 29, 7
	lxv 39, -48(26)
	lxv 45, 16(20)
	lxv 61, -48(3)
	vmsummbm 9, 6, 13, 9
	vmsummbm 11, 7, 13, 11
	vmsummbm 12, 10, 13, 12
	vmsummbm 8, 29, 13, 8
	lxv 45, 16(25)
	vmsummbm 18, 29, 27, 18
	vmsummbm 15, 6, 13, 15
	vmsummbm 16, 7, 13, 16
	vmsummbm 17, 10, 13, 17
	vmsummbm 13, 29, 13, 14
	vmsummbm 14, 6, 27, 19
	vmsummbm 19, 7, 27, 31
	vmsummbm 31, 10, 27, 30
	lxv 62, 16(15)
	lxv 59, 48(15)
	vmsummbm 4, 6, 30, 4
	vmsummbm 5, 7, 30, 5
	vmsummbm 0, 10, 30, 0
	vmsummbm 1, 29, 30, 1
	lxv 38, -32(28)
	lxv 39, -32(26)
	lxv 42, -32(24)
	lxv 62, -32(3)
	lxv 61, 32(20)
	vmsummbm 9, 6, 29, 9
	vmsummbm 11, 7, 29, 11
	vmsummbm 12, 10, 29, 12
	vmsummbm 8, 30, 29, 8
	lxv 61, 32(25)
	vmsummbm 15, 6, 29, 15
	vmsummbm 16, 7, 29, 16
	vmsummbm 17, 10, 29, 17
	vmsummbm 13, 30, 29, 13
	lxv 61, 32(16)
	vmsummbm 14, 6, 29, 14
	vmsummbm 19, 7, 29, 19
	vmsummbm 31, 10, 29, 31
	vmsummbm 18, 30, 29, 18
	lxv 61, 32(15)
	vmsummbm 4, 6, 29, 4
	vmsummbm 5, 7, 29, 5
	lxv 38, -16(28)
	vmsummbm 0, 10, 29, 0
	vmsummbm 1, 30, 29, 1
	lxv 39, -16(26)
	lxv 42, -16(24)
	lxv 62, 48(20)
	lxv 61, -16(3)
	vmsummbm 9, 6, 30, 9
	vmsummbm 11, 7, 30, 11
	vmsummbm 12, 10, 30, 12
	vmsummbm 8, 29, 30, 8
	lxv 62, 48(25)
	vmsummbm 4, 6, 27, 4
	vmsummbm 5, 7, 27, 5
	vmsummbm 0, 10, 27, 0
	vmsummbm 1, 29, 27, 1
	lxv 59, 64(16)
	vmsummbm 15, 6, 30, 15
	vmsummbm 16, 7, 30, 16
	vmsummbm 17, 10, 30, 17
	vmsummbm 13, 29, 30, 13
	lxv 62, 48(16)
	vmsummbm 14, 6, 30, 14
	lxv 38, 0(28)
	vmsummbm 19, 7, 30, 19
	vmsummbm 31, 10, 30, 31
	vmsummbm 18, 29, 30, 18
	lxv 39, 0(26)
	lxv 42, 0(24)
	lxv 62, 64(20)
	lxv 61, 0(3)
	vmsummbm 9, 6, 30, 9
	vmsummbm 11, 7, 30, 11
	vmsummbm 12, 10, 30, 12
	vmsummbm 8, 29, 30, 8
	lxv 62, 64(25)
	vmsummbm 14, 6, 27, 14
	vmsummbm 19, 7, 27, 19
	vmsummbm 31, 10, 27, 31
	vmsummbm 18, 29, 27, 18
	lxv 59, 96(15)
	vmsummbm 15, 6, 30, 15
	vmsummbm 16, 7, 30, 16
	vmsummbm 17, 10, 30, 17
	vmsummbm 13, 29, 30, 13
	lxv 62, 64(15)
	vmsummbm 4, 6, 30, 4
	vmsummbm 5, 7, 30, 5
	vmsummbm 0, 10, 30, 0
	vmsummbm 1, 29, 30, 1
	lxv 38, 16(28)
	lxv 39, 16(26)
	lxv 42, 16(24)
	lxv 62, 16(3)
	lxv 61, 80(20)
	vmsummbm 9, 6, 29, 9
	vmsummbm 11, 7, 29, 11
	vmsummbm 12, 10, 29, 12
	vmsummbm 8, 30, 29, 8
	lxv 61, 80(25)
	vmsummbm 15, 6, 29, 15
	vmsummbm 16, 7, 29, 16
	vmsummbm 17, 10, 29, 17
	vmsummbm 13, 30, 29, 13
	lxv 61, 80(16)
	vmsummbm 14, 6, 29, 14
	vmsummbm 19, 7, 29, 19
	vmsummbm 31, 10, 29, 31
	vmsummbm 18, 30, 29, 18
	lxv 61, 80(15)
	vmsummbm 4, 6, 29, 4
	vmsummbm 5, 7, 29, 5
	lxv 38, 32(28)
	vmsummbm 0, 10, 29, 0
	vmsummbm 1, 30, 29, 1
	lxv 39, 32(26)
	lxv 42, 32(24)
	lxv 62, 96(20)
	lxv 61, 32(3)
	addi 3, 3, 128
	vmsummbm 9, 6, 30, 9
	vmsummbm 11, 7, 30, 11
	vmsummbm 12, 10, 30, 12
	vmsummbm 8, 29, 30, 8
	lxv 62, 96(25)
	vmsummbm 0, 10, 27, 0
	vmsummbm 15, 6, 30, 15
	vmsummbm 16, 7, 30, 16
	vmsummbm 17, 10, 30, 17
	vmsummbm 13, 29, 30, 13
	lxv 62, 96(16)
	vmsummbm 14, 6, 30, 14
	vmsummbm 19, 7, 30, 19
	vmsummbm 31, 10, 30, 31
	vmsummbm 18, 29, 30, 18
	vmsummbm 6, 6, 27, 4
	vmsummbm 7, 7, 27, 5
	vmsummbm 10, 29, 27, 1
	lxv 62, 48(28)
	lxv 61, 48(26)
	lxv 59, 48(24)
	lxv 36, 112(20)
	lxv 37, 112(25)
	li 25, 4
	addi 28, 28, 128
	addi 26, 26, 128
	addi 24, 24, 128
	vmsummbm 9, 30, 4, 9
	vmsummbm 11, 29, 4, 11
	vmsummbm 12, 27, 4, 12
	vmsummbm 4, 26, 4, 8
	vmsummbm 8, 30, 5, 15
	vmsummbm 15, 29, 5, 16
	vmsummbm 16, 27, 5, 17
	lxv 49, 112(16)
	vmsummbm 1, 26, 5, 13
	vmsummbm 13, 30, 17, 14
	vmsummbm 14, 29, 17, 19
	vmsummbm 19, 27, 17, 31
	vmsummbm 5, 26, 17, 18
	lxv 49, 112(15)
	vcfsx 9, 9, 0
	vcfsx 8, 8, 0
	vcfsx 11, 11, 0
	vcfsx 4, 4, 0
	vcfsx 1, 1, 0
	vmsummbm 6, 30, 17, 6
	vmsummbm 7, 29, 17, 7
	vmsummbm 18, 27, 17, 0
	vmsummbm 0, 26, 17, 10
	lxvwsx 42, 29, 0
	lxvx 49, 8, 17
	vcfsx 5, 5, 0
	xvmulsp 31, 42, 49
	xvmulsp 29, 42, 30
	vcfsx 6, 6, 0
	vcfsx 7, 7, 0
	xvmaddasp 35, 41, 31
	vcfsx 9, 13, 0
	lxvx 45, 10, 17
	xvmaddasp 11, 40, 29
	lxvx 40, 11, 17
	addi 17, 17, 1152
	xvmulsp 31, 42, 45
	xvmulsp 42, 42, 40
	xvmaddasp 7, 41, 31
	lxvwsx 41, 29, 31
	xvmaddasp 3, 38, 42
	vcfsx 6, 15, 0
	xvmulsp 42, 41, 30
	xvmulsp 31, 41, 49
	xvmaddasp 10, 38, 42
	vcfsx 6, 14, 0
	xvmulsp 42, 41, 45
	xvmaddasp 34, 43, 31
	xvmaddasp 6, 38, 42
	xvmulsp 38, 41, 40
	lxvwsx 41, 0, 29
	vcfsx 10, 12, 0
	xvmulsp 43, 41, 30
	xvmaddasp 2, 39, 38
	xvmulsp 38, 41, 49
	vcfsx 7, 16, 0
	xvmaddasp 13, 42, 38
	xvmaddasp 9, 39, 43
	vcfsx 6, 19, 0
	xvmulsp 39, 41, 45
	xvmaddasp 5, 38, 39
	vcfsx 6, 18, 0
	xvmulsp 39, 41, 40
	lxvwsx 41, 29, 25
	addi 29, 29, 16
	xvmaddasp 1, 38, 39
	xvmulsp 38, 41, 49
	xvmaddasp 12, 36, 38
	xvmulsp 36, 41, 30
	xvmulsp 38, 41, 45
	xvmaddasp 8, 33, 36
	vcfsx 4, 0, 0
	xvmulsp 32, 41, 40
	xvmaddasp 4, 37, 38
	xvmaddasp 0, 36, 32
	bdnz .Lp9_int8_BB4_120
	b .Lp9_int8_BB4_143
.Lp9_int8_BB4_121:                              #   in Loop: Header=BB4_63 Depth=3
	xxlxor 0, 0, 0
	xxlxor 1, 1, 1
	mr	18, 19
	li 19, 7
	xxlxor 2, 2, 2
	xxlxor 3, 3, 3
	xxlxor 4, 4, 4
	xxlxor 5, 5, 5
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
	xxlxor 8, 8, 8
	xxlxor 9, 9, 9
	xxlxor 10, 10, 10
	xxlxor 11, 11, 11
	xxlxor 12, 12, 12
	xxlxor 13, 13, 13
	xxlxor 31, 31, 31
	xxlxor 30, 30, 30
.Lp9_int8_BB4_122:                              #   in Loop: Header=BB4_63 Depth=3
	ld 4, 840(1)                            # 8-byte Folded Reload
	ld 5, 512(1)                            # 8-byte Folded Reload
	ld 14, 552(1)                           # 8-byte Folded Reload
	ld 16, 472(1)                           # 8-byte Folded Reload
	li 3, 0
	xxlor 52, 29, 29
	ld 15, 464(1)                           # 8-byte Folded Reload
	ld 25, 488(1)                           # 8-byte Folded Reload
	ld 31, 480(1)                           # 8-byte Folded Reload
	ld 29, 800(1)                           # 8-byte Folded Reload
	ld 28, 792(1)                           # 8-byte Folded Reload
	ld 26, 808(1)                           # 8-byte Folded Reload
	ld 24, 784(1)                           # 8-byte Folded Reload
	ld 0, 776(1)                            # 8-byte Folded Reload
	ld 11, 936(1)                           # 8-byte Folded Reload
	ld 12, 816(1)                           # 8-byte Folded Reload
	ld 20, 768(1)                           # 8-byte Folded Reload
	stxv 30, 960(1)
	stxv 31, 976(1)
	b .Lp9_int8_BB4_130
.Lp9_int8_BB4_123:                              #   in Loop: Header=BB4_63 Depth=3
	xxlxor 0, 0, 0
	xxlxor 1, 1, 1
	xxlxor 2, 2, 2
	xxlxor 3, 3, 3
	xxlxor 4, 4, 4
	xxlxor 5, 5, 5
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
.Lp9_int8_BB4_124:                              #   in Loop: Header=BB4_63 Depth=3
	ld 4, 840(1)                            # 8-byte Folded Reload
	ld 5, 512(1)                            # 8-byte Folded Reload
	ld 14, 552(1)                           # 8-byte Folded Reload
	ld 16, 472(1)                           # 8-byte Folded Reload
	li 3, 0
	li 19, 7
	ld 15, 464(1)                           # 8-byte Folded Reload
	ld 25, 488(1)                           # 8-byte Folded Reload
	ld 31, 480(1)                           # 8-byte Folded Reload
	ld 29, 800(1)                           # 8-byte Folded Reload
	ld 28, 792(1)                           # 8-byte Folded Reload
	ld 26, 808(1)                           # 8-byte Folded Reload
	ld 24, 784(1)                           # 8-byte Folded Reload
	ld 0, 776(1)                            # 8-byte Folded Reload
	ld 11, 936(1)                           # 8-byte Folded Reload
	ld 12, 816(1)                           # 8-byte Folded Reload
	ld 20, 768(1)                           # 8-byte Folded Reload
	ld 21, 536(1)                           # 8-byte Folded Reload
	stxv 7, 960(1)
	stxv 6, 976(1)
	stxv 5, 1024(1)
	stxv 4, 1040(1)
	stxv 3, 1088(1)
	stxv 2, 1104(1)
	stxv 1, 1152(1)
	stxv 0, 1168(1)
	b .Lp9_int8_BB4_127
.Lp9_int8_BB4_125:                              #   in Loop: Header=BB4_63 Depth=3
	xxlxor 7, 7, 7
	xxlxor 8, 8, 8
	li 19, 7
	xxlxor 9, 9, 9
	xxlxor 10, 10, 10
	xxlxor 11, 11, 11
	xxlxor 13, 13, 13
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
	xxlxor 12, 12, 12
.Lp9_int8_BB4_126:                              #   in Loop: Header=BB4_63 Depth=3
	ld 4, 840(1)                            # 8-byte Folded Reload
	ld 5, 512(1)                            # 8-byte Folded Reload
	ld 14, 552(1)                           # 8-byte Folded Reload
	ld 16, 472(1)                           # 8-byte Folded Reload
	li 3, 0
	ld 15, 464(1)                           # 8-byte Folded Reload
	ld 25, 488(1)                           # 8-byte Folded Reload
	ld 31, 480(1)                           # 8-byte Folded Reload
	ld 29, 800(1)                           # 8-byte Folded Reload
	ld 28, 792(1)                           # 8-byte Folded Reload
	ld 26, 808(1)                           # 8-byte Folded Reload
	ld 24, 784(1)                           # 8-byte Folded Reload
	ld 0, 776(1)                            # 8-byte Folded Reload
	ld 11, 936(1)                           # 8-byte Folded Reload
	ld 12, 816(1)                           # 8-byte Folded Reload
	ld 20, 768(1)                           # 8-byte Folded Reload
	stxv 0, 960(1)
	stxv 1, 976(1)
	stxv 2, 992(1)
	stxv 3, 1008(1)
	stxv 4, 1024(1)
	stxv 5, 1040(1)
	stxv 6, 1056(1)
	stxv 7, 1072(1)
	stxv 8, 1088(1)
	stxv 9, 1104(1)
	stxv 10, 1120(1)
	stxv 11, 1136(1)
	stxv 13, 1152(1)
	stxv 34, 1168(1)
	stxv 35, 1184(1)
	stxv 12, 1200(1)
.Lp9_int8_BB4_127:                              #   in Loop: Header=BB4_63 Depth=3
	ld 17, 520(1)                           # 8-byte Folded Reload
	ld 10, 824(1)                           # 8-byte Folded Reload
	ld 18, 544(1)                           # 8-byte Folded Reload
	b .Lp9_int8_BB4_147
.Lp9_int8_BB4_128:                              #   in Loop: Header=BB4_63 Depth=3
	xxlxor 0, 0, 0
	xxlxor 1, 1, 1
	xxlxor 2, 2, 2
	xxlxor 3, 3, 3
	xxlxor 4, 4, 4
	xxlxor 5, 5, 5
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
	xxlxor 8, 8, 8
	xxlxor 9, 9, 9
	xxlxor 10, 10, 10
	xxlxor 11, 11, 11
	xxlxor 12, 12, 12
	xxlxor 13, 13, 13
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
.Lp9_int8_BB4_129:                              #   in Loop: Header=BB4_63 Depth=3
	ld 4, 840(1)                            # 8-byte Folded Reload
	ld 5, 512(1)                            # 8-byte Folded Reload
	ld 14, 552(1)                           # 8-byte Folded Reload
	ld 16, 472(1)                           # 8-byte Folded Reload
	li 3, 0
	li 19, 7
	ld 15, 464(1)                           # 8-byte Folded Reload
	ld 25, 488(1)                           # 8-byte Folded Reload
	ld 31, 480(1)                           # 8-byte Folded Reload
	ld 29, 800(1)                           # 8-byte Folded Reload
	ld 28, 792(1)                           # 8-byte Folded Reload
	ld 26, 808(1)                           # 8-byte Folded Reload
	ld 24, 784(1)                           # 8-byte Folded Reload
	ld 0, 776(1)                            # 8-byte Folded Reload
	ld 11, 936(1)                           # 8-byte Folded Reload
	ld 12, 816(1)                           # 8-byte Folded Reload
	ld 18, 544(1)                           # 8-byte Folded Reload
	ld 20, 768(1)                           # 8-byte Folded Reload
	ld 21, 536(1)                           # 8-byte Folded Reload
	stxv 35, 960(1)
	stxv 34, 976(1)
.Lp9_int8_BB4_130:                              #   in Loop: Header=BB4_63 Depth=3
	stxv 13, 1024(1)
	stxv 12, 1040(1)
	stxv 11, 1088(1)
	stxv 10, 1104(1)
	stxv 9, 1152(1)
	stxv 8, 1168(1)
	stxv 7, 1216(1)
	stxv 6, 1232(1)
	stxv 5, 1280(1)
	stxv 4, 1296(1)
	stxv 3, 1344(1)
	stxv 2, 1360(1)
	stxv 1, 1408(1)
	stxv 0, 1424(1)
	ld 17, 520(1)                           # 8-byte Folded Reload
	ld 10, 824(1)                           # 8-byte Folded Reload
	b .Lp9_int8_BB4_147
.Lp9_int8_BB4_131:                              #   in Loop: Header=BB4_63 Depth=3
	xxlxor 5, 5, 5
	xxlxor 6, 6, 6
	mr	18, 19
	xxlxor 7, 7, 7
	xxlxor 8, 8, 8
	xxlxor 9, 9, 9
	xxlxor 10, 10, 10
	xxlxor 11, 11, 11
	xxlxor 13, 13, 13
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
	xxlxor 12, 12, 12
.Lp9_int8_BB4_132:                              #   in Loop: Header=BB4_63 Depth=3
	ld 4, 840(1)                            # 8-byte Folded Reload
	ld 5, 512(1)                            # 8-byte Folded Reload
	stxv 0, 960(1)
	stxv 1, 976(1)
	li 3, 0
	stxv 2, 992(1)
	stxv 3, 1008(1)
	stxv 4, 1024(1)
	stxv 5, 1040(1)
	stxv 6, 1056(1)
	stxv 7, 1072(1)
	stxv 8, 1088(1)
	stxv 9, 1104(1)
	stxv 10, 1120(1)
	stxv 11, 1136(1)
	stxv 13, 1152(1)
	stxv 34, 1168(1)
	stxv 35, 1184(1)
	stxv 12, 1200(1)
	b .Lp9_int8_BB4_144
.Lp9_int8_BB4_133:                              #   in Loop: Header=BB4_63 Depth=3
	xxlxor 0, 0, 0
	xxlxor 1, 1, 1
	xxlxor 2, 2, 2
	xxlxor 3, 3, 3
	xxlxor 5, 5, 5
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
	xxlxor 4, 4, 4
.Lp9_int8_BB4_134:                              #   in Loop: Header=BB4_63 Depth=3
	ld 4, 840(1)                            # 8-byte Folded Reload
	ld 5, 512(1)                            # 8-byte Folded Reload
	li 3, 0
	stxv 0, 960(1)
	stxv 1, 976(1)
	stxv 2, 992(1)
	stxv 3, 1008(1)
	stxv 5, 1024(1)
	stxv 6, 1040(1)
	stxv 7, 1056(1)
	stxv 4, 1072(1)
	b .Lp9_int8_BB4_141
.Lp9_int8_BB4_135:                              #   in Loop: Header=BB4_63 Depth=3
	xxlxor 0, 0, 0
	xxlxor 1, 1, 1
	xxlxor 2, 2, 2
	xxlxor 3, 3, 3
	xxlxor 4, 4, 4
	xxlxor 5, 5, 5
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
	xxlxor 8, 8, 8
	xxlxor 9, 9, 9
	xxlxor 10, 10, 10
	xxlxor 11, 11, 11
	xxlxor 12, 12, 12
	xxlxor 13, 13, 13
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
.Lp9_int8_BB4_136:                              #   in Loop: Header=BB4_63 Depth=3
	ld 4, 840(1)                            # 8-byte Folded Reload
	ld 5, 512(1)                            # 8-byte Folded Reload
	ld 17, 520(1)                           # 8-byte Folded Reload
	ld 14, 552(1)                           # 8-byte Folded Reload
	li 3, 0
	li 19, 7
	ld 16, 472(1)                           # 8-byte Folded Reload
	ld 15, 464(1)                           # 8-byte Folded Reload
	ld 25, 488(1)                           # 8-byte Folded Reload
	ld 31, 480(1)                           # 8-byte Folded Reload
	ld 29, 800(1)                           # 8-byte Folded Reload
	ld 28, 792(1)                           # 8-byte Folded Reload
	ld 26, 808(1)                           # 8-byte Folded Reload
	ld 24, 784(1)                           # 8-byte Folded Reload
	ld 0, 776(1)                            # 8-byte Folded Reload
	ld 11, 936(1)                           # 8-byte Folded Reload
	ld 10, 824(1)                           # 8-byte Folded Reload
	ld 12, 816(1)                           # 8-byte Folded Reload
	ld 18, 544(1)                           # 8-byte Folded Reload
	ld 20, 768(1)                           # 8-byte Folded Reload
	ld 21, 536(1)                           # 8-byte Folded Reload
	ld 27, 376(1)                           # 8-byte Folded Reload
	stxv 35, 960(1)
	stxv 34, 976(1)
	stxv 13, 1024(1)
	stxv 12, 1040(1)
	stxv 11, 1088(1)
	stxv 10, 1104(1)
	stxv 9, 1152(1)
	stxv 8, 1168(1)
	stxv 7, 1216(1)
	stxv 6, 1232(1)
	stxv 5, 1280(1)
	stxv 4, 1296(1)
	stxv 3, 1344(1)
	stxv 2, 1360(1)
	stxv 1, 1408(1)
	stxv 0, 1424(1)
	b .Lp9_int8_BB4_147
.Lp9_int8_BB4_137:                              #   in Loop: Header=BB4_63 Depth=3
	xxlxor 0, 0, 0
	xxlxor 1, 1, 1
	xxlxor 2, 2, 2
	xxlxor 3, 3, 3
	xxlxor 5, 5, 5
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
	xxlxor 4, 4, 4
.Lp9_int8_BB4_138:                              #   in Loop: Header=BB4_63 Depth=3
	ld 4, 840(1)                            # 8-byte Folded Reload
	ld 5, 512(1)                            # 8-byte Folded Reload
	ld 14, 552(1)                           # 8-byte Folded Reload
	ld 16, 472(1)                           # 8-byte Folded Reload
	li 3, 0
	li 19, 7
	ld 15, 464(1)                           # 8-byte Folded Reload
	ld 25, 488(1)                           # 8-byte Folded Reload
	ld 31, 480(1)                           # 8-byte Folded Reload
	ld 29, 800(1)                           # 8-byte Folded Reload
	ld 28, 792(1)                           # 8-byte Folded Reload
	ld 26, 808(1)                           # 8-byte Folded Reload
	ld 24, 784(1)                           # 8-byte Folded Reload
	ld 0, 776(1)                            # 8-byte Folded Reload
	ld 11, 936(1)                           # 8-byte Folded Reload
	ld 10, 824(1)                           # 8-byte Folded Reload
	ld 12, 816(1)                           # 8-byte Folded Reload
	ld 18, 544(1)                           # 8-byte Folded Reload
	ld 20, 768(1)                           # 8-byte Folded Reload
	ld 21, 536(1)                           # 8-byte Folded Reload
	ld 17, 520(1)                           # 8-byte Folded Reload
	stxv 0, 960(1)
	stxv 1, 976(1)
	stxv 2, 992(1)
	stxv 3, 1008(1)
	stxv 5, 1024(1)
	stxv 6, 1040(1)
	stxv 7, 1056(1)
	stxv 4, 1072(1)
	b .Lp9_int8_BB4_147
.Lp9_int8_BB4_139:                              #   in Loop: Header=BB4_63 Depth=3
	xxlxor 0, 0, 0
	xxlxor 1, 1, 1
	xxlxor 2, 2, 2
	xxlxor 3, 3, 3
	xxlxor 4, 4, 4
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
	xxlxor 5, 5, 5
.Lp9_int8_BB4_140:                              #   in Loop: Header=BB4_63 Depth=3
	ld 4, 840(1)                            # 8-byte Folded Reload
	ld 5, 512(1)                            # 8-byte Folded Reload
	stxv 0, 960(1)
	stxv 1, 976(1)
	li 3, 0
	stxv 2, 1024(1)
	stxv 3, 1040(1)
	stxv 4, 1088(1)
	stxv 6, 1104(1)
	stxv 7, 1152(1)
	stxv 5, 1168(1)
.Lp9_int8_BB4_141:                              #   in Loop: Header=BB4_63 Depth=3
	ld 17, 520(1)                           # 8-byte Folded Reload
	ld 14, 552(1)                           # 8-byte Folded Reload
	ld 16, 472(1)                           # 8-byte Folded Reload
	ld 15, 464(1)                           # 8-byte Folded Reload
	li 19, 7
	ld 25, 488(1)                           # 8-byte Folded Reload
	ld 31, 480(1)                           # 8-byte Folded Reload
	ld 29, 800(1)                           # 8-byte Folded Reload
	ld 28, 792(1)                           # 8-byte Folded Reload
	ld 26, 808(1)                           # 8-byte Folded Reload
	ld 24, 784(1)                           # 8-byte Folded Reload
	ld 0, 776(1)                            # 8-byte Folded Reload
	ld 11, 936(1)                           # 8-byte Folded Reload
	ld 10, 824(1)                           # 8-byte Folded Reload
	ld 12, 816(1)                           # 8-byte Folded Reload
	ld 18, 544(1)                           # 8-byte Folded Reload
	ld 20, 768(1)                           # 8-byte Folded Reload
	ld 21, 536(1)                           # 8-byte Folded Reload
	b .Lp9_int8_BB4_147
.Lp9_int8_BB4_142:                              #   in Loop: Header=BB4_63 Depth=3
	xxlxor 5, 5, 5
	xxlxor 6, 6, 6
	mr	18, 19
	xxlxor 7, 7, 7
	xxlxor 8, 8, 8
	xxlxor 9, 9, 9
	xxlxor 10, 10, 10
	xxlxor 11, 11, 11
	xxlxor 12, 12, 12
	xxlxor 13, 13, 13
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
.Lp9_int8_BB4_143:                              #   in Loop: Header=BB4_63 Depth=3
	ld 4, 840(1)                            # 8-byte Folded Reload
	ld 5, 512(1)                            # 8-byte Folded Reload
	li 3, 0
	stxv 35, 960(1)
	stxv 34, 976(1)
	stxv 13, 992(1)
	stxv 12, 1008(1)
	stxv 11, 1024(1)
	stxv 10, 1040(1)
	stxv 9, 1056(1)
	stxv 8, 1072(1)
	stxv 7, 1088(1)
	stxv 6, 1104(1)
	stxv 5, 1120(1)
	stxv 4, 1136(1)
	stxv 3, 1152(1)
	stxv 2, 1168(1)
	stxv 1, 1184(1)
	stxv 0, 1200(1)
.Lp9_int8_BB4_144:                              #   in Loop: Header=BB4_63 Depth=3
	ld 16, 472(1)                           # 8-byte Folded Reload
	ld 15, 464(1)                           # 8-byte Folded Reload
	ld 25, 488(1)                           # 8-byte Folded Reload
	ld 31, 480(1)                           # 8-byte Folded Reload
	ld 29, 800(1)                           # 8-byte Folded Reload
	ld 28, 792(1)                           # 8-byte Folded Reload
	ld 26, 808(1)                           # 8-byte Folded Reload
	ld 24, 784(1)                           # 8-byte Folded Reload
	ld 0, 776(1)                            # 8-byte Folded Reload
	ld 11, 936(1)                           # 8-byte Folded Reload
	ld 12, 816(1)                           # 8-byte Folded Reload
	ld 20, 768(1)                           # 8-byte Folded Reload
	ld 17, 520(1)                           # 8-byte Folded Reload
	ld 10, 824(1)                           # 8-byte Folded Reload
.Lp9_int8_BB4_145:                              #   in Loop: Header=BB4_63 Depth=3
	li 19, 7
	b .Lp9_int8_BB4_147
	.p2align	4
.Lp9_int8_BB4_146:                              #   in Loop: Header=BB4_147 Depth=4
	cror 20, 21, 20
	addi 3, 3, 1
	addi 5, 5, 16
	add 4, 4, 25
	cror 20, 22, 20
	cmpld	3, 16
	cror 20, 23, 20
	isel 14, 19, 14, 20
	beq	0, .Lp9_int8_BB4_62
.Lp9_int8_BB4_147:                              #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB4_150 Depth 5
	add 6, 3, 21
	cmpld	6, 17
	bge 0, .Lp9_int8_BB4_62
# Hand-written epilogue fast path.  Every element of this row is copied
# (and class-tested) iff the whole column span [r31, r31 + 4 * r15) lies in
# [r30, r23) and its last group index is <= r22; the per-element guards of the
# scalar loop below are then all true, so 16-byte vector copies with a
# NaN/Inf test over the same floats are bit-identical.  Otherwise take the
# scalar loop.
	sldi 9, 15, 2
	cmpld	31, 30
	add 9, 9, 31
	blt	0, .Lp9_int8_BB4_slow
	cmpld	9, 23
	addi 9, 9, -1
	bgt	0, .Lp9_int8_BB4_slow
	srdi 9, 9, 2
	cmpld	9, 22
	bgt	0, .Lp9_int8_BB4_slow
	addi 7, 5, -8
	addi 6, 4, -8
	srdi. 8, 15, 1
	xxlxor 3, 3, 3
	mtctr 8
	rldicl 8, 15, 0, 63
	cmpdi	1, 8, 0
	li 8, 64
	li 9, 16
	beq	0, .Lp9_int8_BB4_epi_odd
	.p2align	4
.Lp9_int8_BB4_epi_pair:
	lxvx 0, 0, 7
	lxvx 1, 7, 8
	addi 7, 7, 128
	stxvx 0, 0, 6
	stxvx 1, 6, 9
	addi 6, 6, 32
	xvtstdcsp 0, 0, 112
	xvtstdcsp 1, 1, 112
	xxlor 3, 3, 0
	xxlor 3, 3, 1
	bdnz	.Lp9_int8_BB4_epi_pair
.Lp9_int8_BB4_epi_odd:
	beq	1, .Lp9_int8_BB4_epi_done
	lxvx 0, 0, 7
	stxvx 0, 0, 6
	xvtstdcsp 0, 0, 112
	xxlor 3, 3, 0
.Lp9_int8_BB4_epi_done:
	mfvsrd 8, 3
	mfvsrld 9, 3
	li 6, 7
	or. 8, 8, 9
	addi 3, 3, 1
	addi 5, 5, 16
	add 4, 4, 25
	isel 14, 14, 6, 2
	cmpld	3, 16
	bne	0, .Lp9_int8_BB4_147
	b	.Lp9_int8_BB4_62
.Lp9_int8_BB4_slow:
# %bb.148:                              #   in Loop: Header=BB4_147 Depth=4
	mtctr 15
	crxor 20, 20, 20
	mr	6, 4
	mr	7, 5
	mr	8, 31
	crxor 21, 21, 21
	crxor 22, 22, 22
	crxor 23, 23, 23
	b .Lp9_int8_BB4_150
	.p2align	5
.Lp9_int8_BB4_149:                              #   in Loop: Header=BB4_150 Depth=5
	xststdcsp 0, 0, 112
	xststdcsp 1, 1, 112
	addi 8, 8, 4
	addi 7, 7, 64
	addi 6, 6, 16
	xststdcsp 7, 2, 112
	xststdcsp 3, 3, 112
	crand 24, 24, 2
	cror 20, 20, 24
	crand 24, 25, 6
	cror 21, 21, 24
	crand 24, 26, 30
	cror 22, 22, 24
	crand 24, 27, 14
	cror 23, 23, 24
	bdz .Lp9_int8_BB4_146
.Lp9_int8_BB4_150:                              #   Parent Loop BB4_35 Depth=1
                                        #     Parent Loop BB4_49 Depth=2
                                        #       Parent Loop BB4_63 Depth=3
                                        #         Parent Loop BB4_147 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	cmpld	8, 30
	cmpld 1, 8, 23
	rldicl 9, 8, 62, 2
                                        # implicit-def: $f0
	crandc 24, 4, 0
	cmpld	9, 22
	crandc 24, 24, 1
	bc 4, 24, .Lp9_int8_BB4_152
# %bb.151:                              #   in Loop: Header=BB4_150 Depth=5
	lfs 0, -8(7)
	stfs 0, -8(6)
.Lp9_int8_BB4_152:                              #   in Loop: Header=BB4_150 Depth=5
	addi 9, 8, 1
                                        # implicit-def: $f1
	cmpld	9, 30
	cmpld 1, 9, 23
	rldicl 9, 9, 62, 2
	crandc 25, 4, 0
	cmpld	9, 22
	crandc 25, 25, 1
	bc 4, 25, .Lp9_int8_BB4_154
# %bb.153:                              #   in Loop: Header=BB4_150 Depth=5
	lfs 1, -4(7)
	stfs 1, -4(6)
.Lp9_int8_BB4_154:                              #   in Loop: Header=BB4_150 Depth=5
	addi 9, 8, 2
                                        # implicit-def: $f2
	cmpld	9, 30
	cmpld 1, 9, 23
	rldicl 9, 9, 62, 2
	crandc 26, 4, 0
	cmpld	9, 22
	crandc 26, 26, 1
	bc 4, 26, .Lp9_int8_BB4_156
# %bb.155:                              #   in Loop: Header=BB4_150 Depth=5
	lfs 2, 0(7)
	stfs 2, 0(6)
.Lp9_int8_BB4_156:                              #   in Loop: Header=BB4_150 Depth=5
	addi 9, 8, 3
                                        # implicit-def: $f3
	cmpld	9, 30
	cmpld 1, 9, 23
	rldicl 9, 9, 62, 2
	crandc 27, 4, 0
	cmpld	9, 22
	crandc 27, 27, 1
	bc 4, 27, .Lp9_int8_BB4_149
# %bb.157:                              #   in Loop: Header=BB4_150 Depth=5
	lfs 3, 4(7)
	stfs 3, 4(6)
	b .Lp9_int8_BB4_149
.Lp9_int8_BB4_158:
	li 14, 5
	b .Lp9_int8_BB4_25
	.long	0
	.quad	0
.Lp9_int8_func_end4:
	.size	pq8_project_packed_power9, .Lp9_int8_func_end4-.Lp9_int8_func_begin4
	.cfi_endproc
                                        # -- End function
