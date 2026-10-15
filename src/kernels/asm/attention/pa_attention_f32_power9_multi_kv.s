# attention/pa_attention_f32_power9_multi_kv.s: kernel.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/attention_f32_power9.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function pa_attention_f32_power9_multi_kv
.Lp9_attention_CPI4_0:
	.byte	31                              # 0x1f
	.byte	30                              # 0x1e
	.byte	29                              # 0x1d
	.byte	28                              # 0x1c
	.byte	19                              # 0x13
	.byte	18                              # 0x12
	.byte	17                              # 0x11
	.byte	16                              # 0x10
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.byte	31                              # 0x1f
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lp9_attention_CPI4_1:
	.long	0xff800000                      # float -Inf
	.section	.text.pa_attention_f32_power9_multi_kv,"ax",@progbits
	.globl	pa_attention_f32_power9_multi_kv
	.p2align 6
	.type	pa_attention_f32_power9_multi_kv,@function
pa_attention_f32_power9_multi_kv:     # @pa_attention_f32_power9_multi_kv
.Lp9_attention_func_begin4:
	.cfi_startproc
.Lp9_attention_func_gep4:
	addis 2, 12, .TOC.-.Lp9_attention_func_gep4@ha
	addi 2, 2, .TOC.-.Lp9_attention_func_gep4@l
.Lp9_attention_func_lep4:
	.localentry	pa_attention_f32_power9_multi_kv, .Lp9_attention_func_lep4-.Lp9_attention_func_gep4
# %bb.0:
	mfcr 12
	mflr 0
	stw 12, 8(1)
	stdu 1, -1264(1)
	std 0, 1280(1)
	.cfi_def_cfa_offset 1264
	.cfi_offset lr, 16
	.cfi_offset r14, -216
	.cfi_offset r15, -208
	.cfi_offset r16, -200
	.cfi_offset r17, -192
	.cfi_offset r18, -184
	.cfi_offset r19, -176
	.cfi_offset r20, -168
	.cfi_offset r21, -160
	.cfi_offset r22, -152
	.cfi_offset r23, -144
	.cfi_offset r24, -136
	.cfi_offset r25, -128
	.cfi_offset r26, -120
	.cfi_offset r27, -112
	.cfi_offset r28, -104
	.cfi_offset r29, -96
	.cfi_offset r30, -88
	.cfi_offset r31, -80
	.cfi_offset f23, -72
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
	.cfi_offset v30, -256
	.cfi_offset v31, -240
	std 30, 1176(1)                         # 8-byte Folded Spill
	mr	30, 10
	cmpdi 1, 9, 0
	mr	10, 3
	stxv 62, 1008(1)                        # 16-byte Folded Spill
	std 14, 1048(1)                         # 8-byte Folded Spill
	std 15, 1056(1)                         # 8-byte Folded Spill
	stxv 63, 1024(1)                        # 16-byte Folded Spill
	std 16, 1064(1)                         # 8-byte Folded Spill
	andi. 3, 30, 15
	li 3, 0
	std 17, 1072(1)                         # 8-byte Folded Spill
	std 18, 1080(1)                         # 8-byte Folded Spill
	std 19, 1088(1)                         # 8-byte Folded Spill
	std 20, 1096(1)                         # 8-byte Folded Spill
	std 21, 1104(1)                         # 8-byte Folded Spill
	std 22, 1112(1)                         # 8-byte Folded Spill
	std 23, 1120(1)                         # 8-byte Folded Spill
	std 24, 1128(1)                         # 8-byte Folded Spill
	std 25, 1136(1)                         # 8-byte Folded Spill
	std 26, 1144(1)                         # 8-byte Folded Spill
	std 27, 1152(1)                         # 8-byte Folded Spill
	std 28, 1160(1)                         # 8-byte Folded Spill
	std 29, 1168(1)                         # 8-byte Folded Spill
	crorc 20, 6, 2
	std 31, 1184(1)                         # 8-byte Folded Spill
	stfd 23, 1192(1)                        # 8-byte Folded Spill
	stfd 24, 1200(1)                        # 8-byte Folded Spill
	stfd 25, 1208(1)                        # 8-byte Folded Spill
	stfd 26, 1216(1)                        # 8-byte Folded Spill
	stfd 27, 1224(1)                        # 8-byte Folded Spill
	stfd 28, 1232(1)                        # 8-byte Folded Spill
	stfd 29, 1240(1)                        # 8-byte Folded Spill
	stfd 30, 1248(1)                        # 8-byte Folded Spill
	stfd 31, 1256(1)                        # 8-byte Folded Spill
	std 5, 496(1)                           # 8-byte Folded Spill
	bc 4, 20, .Lp9_attention_BB4_2
.Lp9_attention_BB4_1:
	lxv 63, 1024(1)                         # 16-byte Folded Reload
	lxv 62, 1008(1)                         # 16-byte Folded Reload
	lfd 31, 1256(1)                         # 8-byte Folded Reload
	lfd 30, 1248(1)                         # 8-byte Folded Reload
	ld 31, 1184(1)                          # 8-byte Folded Reload
	ld 30, 1176(1)                          # 8-byte Folded Reload
	ld 29, 1168(1)                          # 8-byte Folded Reload
	ld 28, 1160(1)                          # 8-byte Folded Reload
	ld 27, 1152(1)                          # 8-byte Folded Reload
	lfd 29, 1240(1)                         # 8-byte Folded Reload
	ld 26, 1144(1)                          # 8-byte Folded Reload
	ld 25, 1136(1)                          # 8-byte Folded Reload
	ld 24, 1128(1)                          # 8-byte Folded Reload
	lfd 28, 1232(1)                         # 8-byte Folded Reload
	ld 23, 1120(1)                          # 8-byte Folded Reload
	ld 22, 1112(1)                          # 8-byte Folded Reload
	ld 21, 1104(1)                          # 8-byte Folded Reload
	lfd 27, 1224(1)                         # 8-byte Folded Reload
	ld 20, 1096(1)                          # 8-byte Folded Reload
	ld 19, 1088(1)                          # 8-byte Folded Reload
	ld 18, 1080(1)                          # 8-byte Folded Reload
	lfd 26, 1216(1)                         # 8-byte Folded Reload
	ld 17, 1072(1)                          # 8-byte Folded Reload
	ld 16, 1064(1)                          # 8-byte Folded Reload
	ld 15, 1056(1)                          # 8-byte Folded Reload
	lfd 25, 1208(1)                         # 8-byte Folded Reload
	ld 14, 1048(1)                          # 8-byte Folded Reload
	lfd 24, 1200(1)                         # 8-byte Folded Reload
	lfd 23, 1192(1)                         # 8-byte Folded Reload
	addi 1, 1, 1264
	ld 0, 16(1)
	lwz 12, 8(1)
	mtlr 0
	mtocrf 32, 12
	mtocrf 16, 12
	mtocrf 8, 12
	blr
.Lp9_attention_BB4_2:
	divdu 26, 8, 9
	ld 15, 1360(1)
	ld 0, 1416(1)
	ld 29, 1408(1)
	cmpdi 1, 15, 0
	mulld 5, 26, 9
	sub.	5, 8, 5
	crorc 20, 6, 2
	bc 12, 20, .Lp9_attention_BB4_1
# %bb.3:
	cmpld	29, 0
	bc 4, 0, .Lp9_attention_BB4_1
# %bb.4:
	cmpld	0, 9
	bc 12, 1, .Lp9_attention_BB4_1
# %bb.5:
	ld 3, 1376(1)
	cmpldi	30, 257
	lis 5, 2
	sldi 27, 30, 2
	ld 8, 1368(1)
	ld 28, 1384(1)
	mr	25, 15
	mulld 31, 30, 9
	mulld 9, 26, 4
	std 10, 488(1)                          # 8-byte Folded Spill
	std 3, 400(1)                           # 8-byte Folded Spill
	lis 3, 1
	ori 3, 3, 32768
	isellt	3, 3, 5
	li 5, 16
	divdu 3, 3, 27
	cmpldi	3, 16
	iselgt	3, 3, 5
	cmpdi	8, 0
	std 3, 336(1)                           # 8-byte Folded Spill
	mfocrf 3, 128
	cmpldi	8, 0
	stw 3, 484(1)
	li 3, 0
	std 8, 384(1)                           # 8-byte Folded Spill
	std 9, 528(1)                           # 8-byte Folded Spill
	beq	0, .Lp9_attention_BB4_22
# %bb.6:
	cmpldi	4, 0
	beq	0, .Lp9_attention_BB4_19
# %bb.7:
	ld 12, 384(1)                           # 8-byte Folded Reload
	li 25, 0
	mr	3, 15
	li 11, 0
	addi 5, 12, -1
	b .Lp9_attention_BB4_10
	.p2align	4
.Lp9_attention_BB4_8:                                #   in Loop: Header=BB4_10 Depth=1
	cmpld	8, 3
	isellt	3, 8, 3
	cmpld	9, 25
	iselgt	25, 9, 25
	xor 8, 25, 15
	or. 8, 3, 8
	beq	0, .Lp9_attention_BB4_20
.Lp9_attention_BB4_9:                                #   in Loop: Header=BB4_10 Depth=1
	ld 8, 400(1)                            # 8-byte Folded Reload
	addi 11, 11, 1
	cmpld	11, 4
	add 12, 12, 8
	add 5, 5, 8
	beq	0, .Lp9_attention_BB4_21
.Lp9_attention_BB4_10:                               # =>This Loop Header: Depth=1
                                        #     Child Loop BB4_11 Depth 2
                                        #     Child Loop BB4_16 Depth 2
	mtctr 15
	li 8, 0
	.p2align	5
.Lp9_attention_BB4_11:                               #   Parent Loop BB4_10 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lbzx 9, 12, 8
	cmplwi	9, 0
	bne	0, .Lp9_attention_BB4_13
# %bb.12:                               #   in Loop: Header=BB4_11 Depth=2
	addi 8, 8, 1
	bdnz .Lp9_attention_BB4_11
	b .Lp9_attention_BB4_9
	.p2align	4
.Lp9_attention_BB4_13:                               #   in Loop: Header=BB4_10 Depth=1
	cmpld	8, 15
	beq	0, .Lp9_attention_BB4_9
# %bb.14:                               #   in Loop: Header=BB4_10 Depth=1
	mr	9, 15
	bge 0, .Lp9_attention_BB4_8
# %bb.15:                               #   in Loop: Header=BB4_10 Depth=1
	sub	9, 15, 8
	mtctr 9
	mr	9, 15
	.p2align	5
.Lp9_attention_BB4_16:                               #   Parent Loop BB4_10 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lbzx 10, 5, 9
	cmplwi	10, 0
	bne	0, .Lp9_attention_BB4_8
# %bb.17:                               #   in Loop: Header=BB4_16 Depth=2
	addi 9, 9, -1
	bdnz .Lp9_attention_BB4_16
# %bb.18:                               #   in Loop: Header=BB4_10 Depth=1
	mr	9, 8
	b .Lp9_attention_BB4_8
.Lp9_attention_BB4_19:
	li 25, 0
	mr	3, 15
	b .Lp9_attention_BB4_22
.Lp9_attention_BB4_20:
	li 3, 0
	mr	25, 15
.Lp9_attention_BB4_21:
	ld 9, 528(1)                            # 8-byte Folded Reload
.Lp9_attention_BB4_22:
	lis 4, 21845
	addi 5, 30, -1
	cmpdi	9, 0
	sldi 24, 15, 2
	mulli 21, 15, 12
	cmpldi 4, 30, 17
	xxlxor 62, 62, 62
	sldi 10, 31, 4
	ori 4, 4, 21845
	rldicl 8, 5, 60, 4
	rldic 4, 4, 33, 0
	addi 8, 8, 1
	sldi 18, 15, 3
	add 20, 28, 18
	rldicr 17, 15, 0, 61
	oris 4, 4, 43690
	std 8, 616(1)                           # 8-byte Folded Spill
	rldicl 11, 8, 63, 1
	mulld 8, 9, 15
	mfocrf 9, 128
	cmpdi	30, 0
	ori 23, 4, 43691
	mulhdu 4, 15, 23
	rldicl 4, 4, 59, 5
	mulli 22, 4, 48
	ld 4, 336(1)                            # 8-byte Folded Reload
	stw 9, 324(1)
	addi 9, 1, 920
	std 9, 352(1)                           # 8-byte Folded Spill
	mfocrf 9, 128
	modud 4, 3, 4
	stw 9, 644(1)
	andi. 9, 30, 16
	mfocrf 9, 128
	andi. 5, 5, 16
	add 5, 24, 28
	addi 12, 22, -48
	mcrf 3, 0
	stw 9, 164(1)
	addi 9, 1, 720
	addi 5, 5, 4
	std 10, 360(1)                          # 8-byte Folded Spill
	sldi 10, 15, 4
	std 20, 288(1)                          # 8-byte Folded Spill
	addi 20, 24, -8
	std 21, 632(1)                          # 8-byte Folded Spill
	std 28, 392(1)                          # 8-byte Folded Spill
	std 7, 168(1)                           # 8-byte Folded Spill
	std 22, 128(1)                          # 8-byte Folded Spill
	std 9, 544(1)                           # 8-byte Folded Spill
	addi 9, 1, 784
	std 5, 280(1)                           # 8-byte Folded Spill
	sldi 5, 8, 2
	std 10, 472(1)                          # 8-byte Folded Spill
	addi 10, 28, 96
	std 20, 312(1)                          # 8-byte Folded Spill
	addi 20, 18, -8
	std 9, 600(1)                           # 8-byte Folded Spill
	addi 9, 1, 848
	add 5, 28, 5
	std 10, 216(1)                          # 8-byte Folded Spill
	sldi 10, 31, 3
	std 20, 304(1)                          # 8-byte Folded Spill
	std 9, 592(1)                           # 8-byte Folded Spill
	sldi 9, 31, 2
	std 5, 648(1)                           # 8-byte Folded Spill
	std 10, 296(1)                          # 8-byte Folded Spill
	addi 10, 1, 904
	std 9, 256(1)                           # 8-byte Folded Spill
	mulld 9, 29, 30
	std 10, 376(1)                          # 8-byte Folded Spill
	addis 10, 2, .Lp9_attention_CPI4_0@toc@ha
	addi 10, 10, .Lp9_attention_CPI4_0@toc@l
	lxv 63, 0(10)
	sldi 8, 9, 2
	add 9, 28, 21
	add 5, 6, 8
	std 9, 272(1)                           # 8-byte Folded Spill
	std 5, 248(1)                           # 8-byte Folded Spill
	add 5, 7, 8
	ld 8, 1400(1)
	std 5, 368(1)                           # 8-byte Folded Spill
	rldicl 5, 11, 1, 3
	sub	19, 3, 4
	sub.	3, 15, 22
	addi 5, 5, -2
	mfocrf 4, 128
	cmpldi	3, 4
	cmpld 1, 19, 25
	stw 4, 160(1)
	rldicl 4, 5, 63, 1
	rldicr 5, 5, 4, 58
	mfocrf 3, 128
	cmpldi	15, 4
	addi 21, 5, 32
	ld 5, 1392(1)
	addi 20, 4, 1
	stw 3, 124(1)
	mulhdu 3, 12, 23
	std 23, 112(1)                          # 8-byte Folded Spill
	std 25, 328(1)                          # 8-byte Folded Spill
	std 29, 232(1)                          # 8-byte Folded Spill
	std 29, 240(1)                          # 8-byte Folded Spill
	rldicl 3, 3, 59, 5
	addi 3, 3, 1
	std 3, 152(1)                           # 8-byte Folded Spill
	addi 3, 17, -4
	std 3, 184(1)                           # 8-byte Folded Spill
	mfocrf 3, 128
	cmpldi	15, 48
	stw 3, 228(1)
	mfocrf 3, 128
	cmpd	15, 17
	stw 3, 212(1)
	mfocrf 3, 128
	std 17, 192(1)                          # 8-byte Folded Spill
	stw 3, 180(1)
	std 26, 536(1)                          # 8-byte Folded Spill
	std 18, 608(1)                          # 8-byte Folded Spill
	std 8, 512(1)                           # 8-byte Folded Spill
	std 5, 504(1)                           # 8-byte Folded Spill
	std 6, 144(1)                           # 8-byte Folded Spill
	std 0, 136(1)                           # 8-byte Folded Spill
	std 27, 624(1)                          # 8-byte Folded Spill
	std 19, 200(1)                          # 8-byte Folded Spill
.Lp9_attention_BB4_23:                               # =>This Loop Header: Depth=1
                                        #     Child Loop BB4_27 Depth 2
                                        #       Child Loop BB4_30 Depth 3
                                        #       Child Loop BB4_33 Depth 3
                                        #       Child Loop BB4_37 Depth 3
                                        #         Child Loop BB4_59 Depth 4
                                        #         Child Loop BB4_44 Depth 4
                                        #         Child Loop BB4_49 Depth 4
                                        #           Child Loop BB4_51 Depth 5
                                        #           Child Loop BB4_56 Depth 5
                                        #         Child Loop BB4_96 Depth 4
                                        #           Child Loop BB4_98 Depth 5
                                        #         Child Loop BB4_77 Depth 4
                                        #           Child Loop BB4_79 Depth 5
                                        #             Child Loop BB4_81 Depth 6
                                        #         Child Loop BB4_86 Depth 4
                                        #     Child Loop BB4_109 Depth 2
                                        #       Child Loop BB4_111 Depth 3
                                        #         Child Loop BB4_113 Depth 4
                                        #         Child Loop BB4_159 Depth 4
                                        #         Child Loop BB4_165 Depth 4
                                        #         Child Loop BB4_171 Depth 4
                                        #         Child Loop BB4_176 Depth 4
                                        #         Child Loop BB4_180 Depth 4
                                        #         Child Loop BB4_183 Depth 4
                                        #         Child Loop BB4_122 Depth 4
                                        #         Child Loop BB4_128 Depth 4
                                        #         Child Loop BB4_133 Depth 4
                                        #         Child Loop BB4_137 Depth 4
                                        #         Child Loop BB4_140 Depth 4
                                        #         Child Loop BB4_117 Depth 4
                                        #         Child Loop BB4_142 Depth 4
                                        #         Child Loop BB4_146 Depth 4
                                        #         Child Loop BB4_150 Depth 4
                                        #         Child Loop BB4_153 Depth 4
                                        #     Child Loop BB4_237 Depth 2
                                        #       Child Loop BB4_239 Depth 3
                                        #         Child Loop BB4_250 Depth 4
                                        #         Child Loop BB4_244 Depth 4
                                        #           Child Loop BB4_246 Depth 5
                                        #         Child Loop BB4_255 Depth 4
                                        #           Child Loop BB4_266 Depth 5
                                        #     Child Loop BB4_192 Depth 2
                                        #       Child Loop BB4_196 Depth 3
                                        #       Child Loop BB4_200 Depth 3
                                        #         Child Loop BB4_201 Depth 4
                                        #         Child Loop BB4_203 Depth 4
                                        #       Child Loop BB4_209 Depth 3
                                        #         Child Loop BB4_220 Depth 4
                                        #         Child Loop BB4_214 Depth 4
                                        #           Child Loop BB4_216 Depth 5
                                        #         Child Loop BB4_225 Depth 4
                                        #           Child Loop BB4_227 Depth 5
                                        #     Child Loop BB4_279 Depth 2
                                        #       Child Loop BB4_281 Depth 3
                                        #       Child Loop BB4_285 Depth 3
                                        #       Child Loop BB4_289 Depth 3
                                        #       Child Loop BB4_293 Depth 3
	ld 28, 944(1)
	addi 27, 1, 976
	addi 4, 1, 912
	crmove	12, 4
	std 4, 520(1)                           # 8-byte Folded Spill
	bc 4, 4, .Lp9_attention_BB4_105
# %bb.24:                               #   in Loop: Header=BB4_23 Depth=1
	ld 4, 240(1)                            # 8-byte Folded Reload
	ld 7, 648(1)                            # 8-byte Folded Reload
	mulld 3, 4, 30
	mulld 18, 4, 26
	sldi 3, 3, 2
	std 18, 464(1)                          # 8-byte Folded Spill
	add 3, 6, 3
	std 3, 264(1)                           # 8-byte Folded Spill
	ld 3, 232(1)                            # 8-byte Folded Reload
	sub	3, 4, 3
	mulld 3, 3, 26
	std 3, 448(1)                           # 8-byte Folded Spill
	ld 3, 952(1)
	std 3, 416(1)                           # 8-byte Folded Spill
	ld 3, 960(1)
	std 3, 424(1)                           # 8-byte Folded Spill
	ld 3, 968(1)
	std 3, 408(1)                           # 8-byte Folded Spill
	b .Lp9_attention_BB4_27
	.p2align	4
.Lp9_attention_BB4_25:                               #   in Loop: Header=BB4_27 Depth=2
	ld 3, 512(1)                            # 8-byte Folded Reload
	ld 4, 432(1)                            # 8-byte Folded Reload
	ld 5, 440(1)                            # 8-byte Folded Reload
	mulld 3, 4, 3
	ld 4, 504(1)                            # 8-byte Folded Reload
	sldi 3, 3, 2
	add 3, 4, 3
	ld 4, 448(1)                            # 8-byte Folded Reload
	add 4, 5, 4
	mulld 4, 4, 30
	sldi 4, 4, 2
	add 28, 3, 4
.Lp9_attention_BB4_26:                               #   in Loop: Header=BB4_27 Depth=2
	ld 3, 328(1)                            # 8-byte Folded Reload
	ld 19, 344(1)                           # 8-byte Folded Reload
	cmpld	19, 3
	bge 0, .Lp9_attention_BB4_106
.Lp9_attention_BB4_27:                               #   Parent Loop BB4_23 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB4_30 Depth 3
                                        #       Child Loop BB4_33 Depth 3
                                        #       Child Loop BB4_37 Depth 3
                                        #         Child Loop BB4_59 Depth 4
                                        #         Child Loop BB4_44 Depth 4
                                        #         Child Loop BB4_49 Depth 4
                                        #           Child Loop BB4_51 Depth 5
                                        #           Child Loop BB4_56 Depth 5
                                        #         Child Loop BB4_96 Depth 4
                                        #           Child Loop BB4_98 Depth 5
                                        #         Child Loop BB4_77 Depth 4
                                        #           Child Loop BB4_79 Depth 5
                                        #             Child Loop BB4_81 Depth 6
                                        #         Child Loop BB4_86 Depth 4
	ld 4, 336(1)                            # 8-byte Folded Reload
	sub	3, 15, 19
	ld 14, 624(1)                           # 8-byte Folded Reload
	add 5, 19, 4
	cmpld	3, 4
	isellt	29, 15, 5
	std 5, 344(1)                           # 8-byte Folded Spill
	cmpld	19, 29
	mfocrf 4, 128
	stw 4, 456(1)
	bge 0, .Lp9_attention_BB4_34
# %bb.28:                               #   in Loop: Header=BB4_27 Depth=2
	ld 4, 336(1)                            # 8-byte Folded Reload
	ld 18, 264(1)                           # 8-byte Folded Reload
	mr	26, 19
	cmpld	4, 3
	isellt	25, 4, 3
	andi. 23, 25, 3
	beq	0, .Lp9_attention_BB4_31
# %bb.29:                               #   in Loop: Header=BB4_27 Depth=2
	li 22, 0
	mr	26, 19
	.p2align	5
.Lp9_attention_BB4_30:                               #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_27 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	sub	3, 26, 19
	mulld 4, 26, 31
	mr	5, 14
	mulld 3, 3, 30
	sldi 4, 4, 2
	sldi 3, 3, 2
	add 4, 18, 4
	add 3, 7, 3
	bl memcpy
	nop
	addi 22, 22, 1
	ld 7, 648(1)                            # 8-byte Folded Reload
	addi 26, 26, 1
	cmpld	22, 23
	bne	0, .Lp9_attention_BB4_30
.Lp9_attention_BB4_31:                               #   in Loop: Header=BB4_27 Depth=2
	addi 3, 25, -1
	cmpldi	3, 3
	blt	0, .Lp9_attention_BB4_34
# %bb.32:                               #   in Loop: Header=BB4_27 Depth=2
	ld 4, 256(1)                            # 8-byte Folded Reload
	addi 3, 26, 3
	ld 17, 248(1)                           # 8-byte Folded Reload
	mulld 25, 4, 3
	addi 3, 26, 2
	mulld 18, 4, 26
	mulld 23, 4, 3
	addi 3, 26, 1
	mulld 22, 4, 3
	.p2align	5
.Lp9_attention_BB4_33:                               #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_27 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	sub	16, 26, 19
	add 4, 17, 18
	mr	5, 14
	mulld 3, 16, 30
	sldi 3, 3, 2
	add 3, 7, 3
	bl memcpy
	nop
	addi 3, 16, 1
	ld 4, 648(1)                            # 8-byte Folded Reload
	mr	5, 14
	mulld 3, 3, 30
	sldi 3, 3, 2
	add 3, 4, 3
	add 4, 17, 22
	bl memcpy
	nop
	addi 3, 16, 2
	ld 4, 648(1)                            # 8-byte Folded Reload
	mr	5, 14
	mulld 3, 3, 30
	sldi 3, 3, 2
	add 3, 4, 3
	add 4, 17, 23
	bl memcpy
	nop
	addi 3, 16, 3
	ld 4, 648(1)                            # 8-byte Folded Reload
	mr	5, 14
	mulld 3, 3, 30
	sldi 3, 3, 2
	add 3, 4, 3
	add 4, 17, 25
	bl memcpy
	nop
	ld 3, 360(1)                            # 8-byte Folded Reload
	addi 26, 26, 4
	ld 7, 648(1)                            # 8-byte Folded Reload
	cmpld	26, 29
	add 17, 17, 3
	blt	0, .Lp9_attention_BB4_33
.Lp9_attention_BB4_34:                               #   in Loop: Header=BB4_27 Depth=2
	ld 26, 536(1)                           # 8-byte Folded Reload
	ld 6, 528(1)                            # 8-byte Folded Reload
	lwz 3, 324(1)
	ld 18, 464(1)                           # 8-byte Folded Reload
	mtocrf 128, 3
	bc 12, 2, .Lp9_attention_BB4_26
# %bb.35:                               #   in Loop: Header=BB4_27 Depth=2
	li 3, 0
	ld 5, 288(1)                            # 8-byte Folded Reload
	ld 9, 280(1)                            # 8-byte Folded Reload
	ld 10, 392(1)                           # 8-byte Folded Reload
	cmpd 1, 19, 29
	li 8, 0
	std 3, 584(1)                           # 8-byte Folded Spill
	ld 3, 272(1)                            # 8-byte Folded Reload
	std 3, 576(1)                           # 8-byte Folded Spill
	b .Lp9_attention_BB4_37
	.p2align	4
.Lp9_attention_BB4_36:                               #   in Loop: Header=BB4_37 Depth=3
	ld 3, 472(1)                            # 8-byte Folded Reload
	ld 10, 552(1)                           # 8-byte Folded Reload
	ld 9, 560(1)                            # 8-byte Folded Reload
	ld 5, 568(1)                            # 8-byte Folded Reload
	addi 8, 8, 4
	add 10, 10, 3
	ld 4, 576(1)                            # 8-byte Folded Reload
	add 9, 9, 3
	add 5, 5, 3
	add 4, 4, 3
	ld 3, 584(1)                            # 8-byte Folded Reload
	ld 6, 528(1)                            # 8-byte Folded Reload
	ld 26, 536(1)                           # 8-byte Folded Reload
	ld 18, 464(1)                           # 8-byte Folded Reload
	cmpld	8, 6
	std 4, 576(1)                           # 8-byte Folded Spill
	addi 3, 3, 1
	std 3, 584(1)                           # 8-byte Folded Spill
	bge 0, .Lp9_attention_BB4_25
.Lp9_attention_BB4_37:                               #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_27 Depth=2
                                        # =>    This Loop Header: Depth=3
                                        #         Child Loop BB4_59 Depth 4
                                        #         Child Loop BB4_44 Depth 4
                                        #         Child Loop BB4_49 Depth 4
                                        #           Child Loop BB4_51 Depth 5
                                        #           Child Loop BB4_56 Depth 5
                                        #         Child Loop BB4_96 Depth 4
                                        #           Child Loop BB4_98 Depth 5
                                        #         Child Loop BB4_77 Depth 4
                                        #           Child Loop BB4_79 Depth 5
                                        #             Child Loop BB4_81 Depth 6
                                        #         Child Loop BB4_86 Depth 4
	std 10, 552(1)                          # 8-byte Folded Spill
	divdu 10, 8, 26
	std 5, 568(1)                           # 8-byte Folded Spill
	ld 3, 496(1)                            # 8-byte Folded Reload
	sub	11, 6, 8
	ld 4, 488(1)                            # 8-byte Folded Reload
	std 9, 560(1)                           # 8-byte Folded Spill
	mulld 5, 10, 26
	sub	6, 8, 5
	mulld 5, 10, 3
	std 6, 440(1)                           # 8-byte Folded Spill
	add 6, 6, 18
	mulld 6, 6, 30
	sldi 5, 5, 2
	add 5, 4, 5
	sldi 6, 6, 2
	add 12, 5, 6
	std 12, 976(1)
	lwz 5, 484(1)
	std 10, 432(1)                          # 8-byte Folded Spill
	mtocrf 128, 5
	subfic 5, 11, 1
	bc 12, 2, .Lp9_attention_BB4_39
# %bb.38:                               #   in Loop: Header=BB4_37 Depth=3
	ld 17, 400(1)                           # 8-byte Folded Reload
	ld 16, 384(1)                           # 8-byte Folded Reload
	cmpldi	11, 2
	li 7, 0
	li 5, 2
	li 9, 3
	maddld 10, 10, 17, 16
	iselgt	5, 5, 7
	or 5, 5, 8
	cmpldi	11, 3
	iselgt	9, 9, 7
	li 6, 1
	subfe 6, 6, 6
	mr	7, 26
	neg 6, 6
	or 9, 9, 8
	or 6, 8, 6
	divdu 0, 9, 26
	std 10, 912(1)
	divdu 10, 5, 26
	mulld 25, 0, 7
	mulld 28, 10, 26
	maddld 26, 10, 17, 16
	sub	9, 9, 25
	mulld 25, 0, 3
	std 26, 928(1)
	divdu 26, 6, 7
	sub	5, 5, 28
	mulld 28, 10, 3
	sldi 25, 25, 2
	add 25, 4, 25
	maddld 23, 26, 17, 16
	std 23, 920(1)
	mulld 23, 26, 7
	ld 7, 512(1)                            # 8-byte Folded Reload
	mulld 22, 10, 7
	sldi 10, 28, 2
	add 28, 5, 18
	sub	6, 6, 23
	mulld 23, 26, 3
	ld 3, 504(1)                            # 8-byte Folded Reload
	mulld 28, 28, 30
	add 10, 4, 10
	sldi 28, 28, 2
	add 10, 10, 28
	sldi 28, 22, 2
	ld 22, 448(1)                           # 8-byte Folded Reload
	add 28, 3, 28
	std 10, 992(1)
	add 5, 5, 22
	mulld 5, 5, 30
	sldi 5, 5, 2
	add 5, 28, 5
	std 5, 424(1)                           # 8-byte Folded Spill
	add 5, 9, 18
	mulld 5, 5, 30
	sldi 5, 5, 2
	add 28, 25, 5
	add 5, 6, 18
	sldi 25, 23, 2
	add 6, 6, 22
	mulld 5, 5, 30
	add 25, 4, 25
	mulld 6, 6, 30
	std 28, 1000(1)
	sldi 5, 5, 2
	sldi 6, 6, 2
	add 18, 25, 5
	mulld 5, 26, 7
	std 18, 984(1)
	sldi 5, 5, 2
	add 5, 3, 5
	add 4, 5, 6
	mulld 5, 0, 7
	add 6, 9, 22
	mulld 6, 6, 30
	std 4, 416(1)                           # 8-byte Folded Spill
	ld 4, 520(1)                            # 8-byte Folded Reload
	sldi 5, 5, 2
	add 5, 3, 5
	sldi 6, 6, 2
	add 26, 5, 6
	maddld 5, 0, 17, 16
	b .Lp9_attention_BB4_40
	.p2align	4
.Lp9_attention_BB4_39:                               #   in Loop: Header=BB4_37 Depth=3
	cmpldi	11, 2
	li 5, 0
	li 6, 2
	li 7, 1
	ld 16, 448(1)                           # 8-byte Folded Reload
	std 5, 928(1)
	iselgt	6, 6, 5
	cmpldi	11, 3
	subfe 9, 7, 7
	li 7, 3
	mr	17, 18
	or 6, 6, 8
	iselgt	10, 7, 5
	neg 9, 9
	mr	7, 26
	divdu 0, 6, 26
	or 10, 10, 8
	or 9, 8, 9
	divdu 26, 10, 26
	divdu 25, 9, 7
	mulld 28, 0, 7
	mulld 23, 26, 7
	mulld 22, 25, 7
	ld 7, 512(1)                            # 8-byte Folded Reload
	mulld 18, 26, 3
	sub	6, 6, 28
	mulld 28, 0, 3
	mulld 0, 0, 7
	sub	23, 10, 23
	sub	9, 9, 22
	mulld 22, 25, 3
	ld 3, 504(1)                            # 8-byte Folded Reload
	sldi 10, 28, 2
	add 28, 6, 17
	add 6, 6, 16
	sldi 0, 0, 2
	mulld 6, 6, 30
	mulld 28, 28, 30
	add 10, 4, 10
	add 0, 3, 0
	sldi 6, 6, 2
	sldi 28, 28, 2
	add 6, 0, 6
	add 10, 10, 28
	sldi 28, 18, 2
	sldi 0, 22, 2
	std 6, 424(1)                           # 8-byte Folded Spill
	add 6, 23, 17
	add 28, 4, 28
	add 0, 4, 0
	ld 4, 520(1)                            # 8-byte Folded Reload
	std 10, 992(1)
	mulld 6, 6, 30
	stxvx 62, 0, 4
	sldi 6, 6, 2
	add 28, 28, 6
	add 6, 9, 17
	add 9, 9, 16
	mulld 6, 6, 30
	mulld 9, 9, 30
	std 28, 1000(1)
	sldi 6, 6, 2
	sldi 9, 9, 2
	add 18, 0, 6
	mulld 6, 25, 7
	std 18, 984(1)
	sldi 6, 6, 2
	add 6, 3, 6
	add 6, 6, 9
	add 9, 23, 16
	std 6, 416(1)                           # 8-byte Folded Spill
	mulld 6, 26, 7
	mulld 9, 9, 30
	sldi 6, 6, 2
	sldi 9, 9, 2
	add 6, 3, 6
	add 26, 6, 9
.Lp9_attention_BB4_40:                               #   in Loop: Header=BB4_37 Depth=3
	cmpldi 5, 11, 4
	li 3, 4
	std 5, 936(1)
	ld 7, 648(1)                            # 8-byte Folded Reload
	li 5, 0
	isel 0, 11, 3, 20
	lwz 3, 456(1)
	std 26, 408(1)                          # 8-byte Folded Spill
	mtocrf 128, 3
	bc 4, 0, .Lp9_attention_BB4_42
# %bb.41:                               #   in Loop: Header=BB4_37 Depth=3
	mr	22, 19
	mr	26, 29
	b .Lp9_attention_BB4_49
	.p2align	4
.Lp9_attention_BB4_42:                               #   in Loop: Header=BB4_37 Depth=3
	bc 4, 6, .Lp9_attention_BB4_58
# %bb.43:                               #   in Loop: Header=BB4_37 Depth=3
	ld 6, 376(1)                            # 8-byte Folded Reload
	.p2align	5
.Lp9_attention_BB4_44:                               #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_27 Depth=2
                                        #       Parent Loop BB4_37 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	ldu 9, 8(6)
	cmpldi	9, 0
	beq	0, .Lp9_attention_BB4_64
# %bb.45:                               #   in Loop: Header=BB4_44 Depth=4
	addi 5, 5, 1
	cmpld	5, 0
	bne	0, .Lp9_attention_BB4_44
	b .Lp9_attention_BB4_36
.Lp9_attention_BB4_46:                               #   in Loop: Header=BB4_49 Depth=4
	mr	25, 6
.Lp9_attention_BB4_47:                               #   in Loop: Header=BB4_49 Depth=4
	cmpld	6, 26
	isellt	26, 6, 26
	cmpld	25, 22
	iselgt	22, 25, 22
	xor 6, 26, 19
	xor 9, 22, 29
	or. 6, 6, 9
	beq	0, .Lp9_attention_BB4_104
.Lp9_attention_BB4_48:                               #   in Loop: Header=BB4_49 Depth=4
	addi 5, 5, 1
	cmpld	5, 0
	beq	0, .Lp9_attention_BB4_62
.Lp9_attention_BB4_49:                               #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_27 Depth=2
                                        #       Parent Loop BB4_37 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB4_51 Depth 5
                                        #           Child Loop BB4_56 Depth 5
	sldi 6, 5, 3
	ldx 9, 4, 6
	cmpldi	9, 0
	beq	0, .Lp9_attention_BB4_63
# %bb.50:                               #   in Loop: Header=BB4_49 Depth=4
	mr	6, 19
	.p2align	5
.Lp9_attention_BB4_51:                               #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_27 Depth=2
                                        #       Parent Loop BB4_37 Depth=3
                                        #         Parent Loop BB4_49 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	lbzx 25, 9, 6
	cmplwi	25, 0
	bne	0, .Lp9_attention_BB4_53
# %bb.52:                               #   in Loop: Header=BB4_51 Depth=5
	addi 6, 6, 1
	cmpld	6, 29
	bne	0, .Lp9_attention_BB4_51
	b .Lp9_attention_BB4_48
	.p2align	4
.Lp9_attention_BB4_53:                               #   in Loop: Header=BB4_49 Depth=4
	cmpld	6, 29
	beq	0, .Lp9_attention_BB4_48
# %bb.54:                               #   in Loop: Header=BB4_49 Depth=4
	mr	25, 29
	bge 0, .Lp9_attention_BB4_47
# %bb.55:                               #   in Loop: Header=BB4_49 Depth=4
	addi 9, 9, -1
	mr	25, 29
	.p2align	5
.Lp9_attention_BB4_56:                               #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_27 Depth=2
                                        #       Parent Loop BB4_37 Depth=3
                                        #         Parent Loop BB4_49 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	lbzx 23, 9, 25
	cmplwi	23, 0
	bne	0, .Lp9_attention_BB4_47
# %bb.57:                               #   in Loop: Header=BB4_56 Depth=5
	addi 25, 25, -1
	cmpld	25, 6
	bgt	0, .Lp9_attention_BB4_56
	b .Lp9_attention_BB4_46
	.p2align	4
.Lp9_attention_BB4_58:                               #   in Loop: Header=BB4_37 Depth=3
	ld 6, 376(1)                            # 8-byte Folded Reload
	.p2align	5
.Lp9_attention_BB4_59:                               #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_27 Depth=2
                                        #       Parent Loop BB4_37 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	ldu 9, 8(6)
	cmpldi	9, 0
	beq	0, .Lp9_attention_BB4_63
# %bb.60:                               #   in Loop: Header=BB4_59 Depth=4
	addi 5, 5, 1
	cmpld	5, 0
	bne	0, .Lp9_attention_BB4_59
# %bb.61:                               #   in Loop: Header=BB4_37 Depth=3
	mr	26, 29
	mr	22, 19
.Lp9_attention_BB4_62:                               #   in Loop: Header=BB4_37 Depth=3
	cmpld	26, 22
	bge 0, .Lp9_attention_BB4_36
	b .Lp9_attention_BB4_66
	.p2align	4
.Lp9_attention_BB4_63:                               #   in Loop: Header=BB4_37 Depth=3
	mr	22, 29
	b .Lp9_attention_BB4_65
.Lp9_attention_BB4_64:                               #   in Loop: Header=BB4_37 Depth=3
	mr	22, 19
.Lp9_attention_BB4_65:                               #   in Loop: Header=BB4_37 Depth=3
	mr	26, 19
.Lp9_attention_BB4_66:                               #   in Loop: Header=BB4_37 Depth=3
	mulld 5, 8, 15
	ld 3, 392(1)                            # 8-byte Folded Reload
	sldi 5, 5, 2
	add 9, 3, 5
	addi 5, 26, 2
	cmpld	5, 22
	ble 0, .Lp9_attention_BB4_94
.Lp9_attention_BB4_67:                               #   in Loop: Header=BB4_37 Depth=3
	cmpld	22, 26
	ble 0, .Lp9_attention_BB4_36
# %bb.68:                               #   in Loop: Header=BB4_37 Depth=3
	lwz 3, 644(1)
	mtocrf 128, 3
	bc 4, 2, .Lp9_attention_BB4_77
# %bb.69:                               #   in Loop: Header=BB4_37 Depth=3
	sub	6, 22, 26
	addi 5, 26, 1
	andi. 6, 6, 1
	bc 4, 1, .Lp9_attention_BB4_74
# %bb.70:                               #   in Loop: Header=BB4_37 Depth=3
	sldi 6, 26, 2
	li 3, 0
	cmpldi	11, 2
	mr	26, 5
	stwux 3, 9, 6
	blt	0, .Lp9_attention_BB4_74
# %bb.71:                               #   in Loop: Header=BB4_37 Depth=3
	mr	26, 5
	stwx 3, 9, 24
	beq	0, .Lp9_attention_BB4_74
# %bb.72:                               #   in Loop: Header=BB4_37 Depth=3
	ld 4, 608(1)                            # 8-byte Folded Reload
	mr	26, 5
	stwx 3, 9, 4
	blt	5, .Lp9_attention_BB4_74
# %bb.73:                               #   in Loop: Header=BB4_37 Depth=3
	ld 4, 632(1)                            # 8-byte Folded Reload
	mr	26, 5
	stwx 3, 9, 4
.Lp9_attention_BB4_74:                               #   in Loop: Header=BB4_37 Depth=3
	cmpld	22, 5
	beq	0, .Lp9_attention_BB4_36
# %bb.75:                               #   in Loop: Header=BB4_37 Depth=3
	ld 4, 472(1)                            # 8-byte Folded Reload
	ld 3, 304(1)                            # 8-byte Folded Reload
	ld 6, 584(1)                            # 8-byte Folded Reload
	cmpldi	11, 1
	cmpdi 5, 11, 2
	cmpldi 6, 11, 3
	sldi 11, 26, 2
	maddld 5, 4, 6, 3
	ld 3, 392(1)                            # 8-byte Folded Reload
	add 10, 3, 5
	ld 5, 312(1)                            # 8-byte Folded Reload
	add 10, 10, 11
	maddld 5, 4, 6, 5
	add 9, 3, 5
	maddld 5, 4, 6, 3
	ld 3, 576(1)                            # 8-byte Folded Reload
	add 9, 9, 11
	addi 6, 5, -8
	add 5, 3, 11
	add 6, 6, 11
	sub	11, 22, 26
	addi 11, 11, -2
	rldicl 11, 11, 63, 1
	addi 11, 11, 1
	mtctr 11
	b .Lp9_attention_BB4_86
	.p2align	4
.Lp9_attention_BB4_76:                               #   in Loop: Header=BB4_77 Depth=4
	addi 26, 26, 1
	cmpld	26, 22
	beq	0, .Lp9_attention_BB4_36
.Lp9_attention_BB4_77:                               #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_27 Depth=2
                                        #       Parent Loop BB4_37 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB4_79 Depth 5
                                        #             Child Loop BB4_81 Depth 6
	sub	5, 26, 19
	sldi 6, 26, 2
	li 11, 0
	mulld 5, 5, 30
	add 6, 9, 6
	sldi 5, 5, 2
	add 5, 7, 5
	addi 10, 5, 64
	b .Lp9_attention_BB4_79
	.p2align	4
.Lp9_attention_BB4_78:                               #   in Loop: Header=BB4_79 Depth=5
	xvaddsp 2, 3, 2
	xvaddsp 0, 1, 0
	mulld 12, 11, 15
	addi 11, 11, 1
	cmpld	11, 0
	sldi 12, 12, 2
	xvaddsp 34, 2, 0
	vsldoi 3, 2, 2, 12
	vperm 2, 2, 2, 31
	xvaddsp 0, 35, 34
	xxspltw 1, 0, 2
	xvaddsp 0, 0, 1
	xxsldwi 0, 0, 0, 2
	stfiwx 0, 6, 12
	bge 0, .Lp9_attention_BB4_76
.Lp9_attention_BB4_79:                               #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_27 Depth=2
                                        #       Parent Loop BB4_37 Depth=3
                                        #         Parent Loop BB4_77 Depth=4
                                        # =>        This Loop Header: Depth=5
                                        #             Child Loop BB4_81 Depth 6
	sldi 12, 11, 3
	xxlxor 0, 0, 0
	li 28, 0
	ldx 12, 27, 12
	xxlxor 1, 1, 1
	xxlxor 2, 2, 2
	xxlxor 3, 3, 3
	bc 12, 16, .Lp9_attention_BB4_83
# %bb.80:                               #   in Loop: Header=BB4_79 Depth=5
	mtctr 20
	addi 28, 12, 64
	li 25, 0
	.p2align	5
.Lp9_attention_BB4_81:                               #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_27 Depth=2
                                        #       Parent Loop BB4_37 Depth=3
                                        #         Parent Loop BB4_77 Depth=4
                                        #           Parent Loop BB4_79 Depth=5
                                        # =>          This Inner Loop Header: Depth=6
	sldi 23, 25, 2
	lxv 4, -64(28)
	lxv 6, 32(28)
	addi 25, 25, 32
	lxvx 5, 5, 23
	add 18, 5, 23
	xvmaddasp 3, 4, 5
	lxv 4, -48(28)
	lxv 5, 16(18)
	xvmaddasp 2, 4, 5
	lxv 4, -32(28)
	lxv 5, 32(18)
	xvmaddasp 1, 4, 5
	lxv 4, -16(28)
	lxv 5, 48(18)
	add 18, 10, 23
	lxv 7, 32(18)
	xvmaddasp 0, 4, 5
	lxv 4, 0(28)
	lxvx 5, 10, 23
	xvmaddasp 1, 6, 7
	xvmaddasp 3, 4, 5
	lxv 4, 16(28)
	lxv 5, 16(18)
	xvmaddasp 2, 4, 5
	lxv 4, 48(28)
	lxv 5, 48(18)
	addi 28, 28, 128
	xvmaddasp 0, 4, 5
	bdnz .Lp9_attention_BB4_81
# %bb.82:                               #   in Loop: Header=BB4_79 Depth=5
	mr	28, 21
.Lp9_attention_BB4_83:                               #   in Loop: Header=BB4_79 Depth=5
	bc 4, 14, .Lp9_attention_BB4_78
# %bb.84:                               #   in Loop: Header=BB4_79 Depth=5
	sldi 28, 28, 2
	lxvx 4, 12, 28
	lxvx 5, 5, 28
	add 25, 12, 28
	add 12, 5, 28
	xvmaddasp 3, 4, 5
	lxv 4, 16(25)
	lxv 5, 16(12)
	xvmaddasp 2, 4, 5
	lxv 4, 32(25)
	lxv 5, 32(12)
	xvmaddasp 1, 4, 5
	lxv 4, 48(25)
	lxv 5, 48(12)
	xvmaddasp 0, 4, 5
	b .Lp9_attention_BB4_78
	.p2align	5
.Lp9_attention_BB4_85:                               #   in Loop: Header=BB4_86 Depth=4
	addi 5, 5, 8
	bdz .Lp9_attention_BB4_36
.Lp9_attention_BB4_86:                               #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_27 Depth=2
                                        #       Parent Loop BB4_37 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	li 3, 0
	mr	11, 10
	addi 10, 10, 8
	mr	12, 9
	addi 9, 9, 8
	stwu 3, 8(6)
	bc 4, 1, .Lp9_attention_BB4_90
# %bb.87:                               #   in Loop: Header=BB4_86 Depth=4
	stw 3, 8(12)
	bc 12, 22, .Lp9_attention_BB4_90
# %bb.88:                               #   in Loop: Header=BB4_86 Depth=4
	stw 3, 8(11)
	bc 4, 25, .Lp9_attention_BB4_90
# %bb.89:                               #   in Loop: Header=BB4_86 Depth=4
	stw 3, 0(5)
	.p2align	5
.Lp9_attention_BB4_90:                               #   in Loop: Header=BB4_86 Depth=4
	stw 3, 4(6)
	bc 4, 1, .Lp9_attention_BB4_85
# %bb.91:                               #   in Loop: Header=BB4_86 Depth=4
	stw 3, 12(12)
	bc 12, 22, .Lp9_attention_BB4_85
# %bb.92:                               #   in Loop: Header=BB4_86 Depth=4
	stw 3, 12(11)
	bc 4, 25, .Lp9_attention_BB4_85
# %bb.93:                               #   in Loop: Header=BB4_86 Depth=4
	stw 3, 4(5)
	b .Lp9_attention_BB4_85
	.p2align	4
.Lp9_attention_BB4_94:                               #   in Loop: Header=BB4_37 Depth=3
	addi 6, 28, 32
	addi 23, 10, 32
	addi 18, 18, 32
	cmpldi	11, 1
	cmpdi 6, 11, 2
	cmpldi 7, 11, 3
	b .Lp9_attention_BB4_96
	.p2align	4
.Lp9_attention_BB4_95:                               #   in Loop: Header=BB4_96 Depth=4
	addi 5, 26, 2
	ld 7, 648(1)                            # 8-byte Folded Reload
	cmpld 2, 5, 22
	bgt	2, .Lp9_attention_BB4_67
.Lp9_attention_BB4_96:                               #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_27 Depth=2
                                        #       Parent Loop BB4_37 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB4_98 Depth 5
	lwz 3, 644(1)
	mr	17, 26
	mr	26, 5
	rotlwi	3, 3, 24
	mtocrf 32, 3
	bc 12, 10, .Lp9_attention_BB4_99
# %bb.97:                               #   in Loop: Header=BB4_96 Depth=4
	sub	5, 17, 19
	ld 3, 624(1)                            # 8-byte Folded Reload
	li 14, 0
	xxlxor 0, 0, 0
	mulld 5, 5, 30
	mr	10, 18
	mr	28, 23
	xxlxor 4, 4, 4
	xxlxor 1, 1, 1
	xxlxor 5, 5, 5
	mr	25, 6
	sldi 5, 5, 2
	xxlxor 2, 2, 2
	xxlxor 6, 6, 6
	xxlxor 3, 3, 3
	xxlxor 7, 7, 7
	xxlxor 8, 8, 8
	xxlxor 12, 12, 12
	xxlxor 9, 9, 9
	xxlxor 13, 13, 13
	xxlxor 10, 10, 10
	add 16, 7, 5
	xxlxor 34, 34, 34
	xxlxor 11, 11, 11
	add 5, 16, 3
	ld 3, 616(1)                            # 8-byte Folded Reload
	xxlxor 35, 35, 35
	xxlxor 33, 33, 33
	xxlxor 36, 36, 36
	mtctr 3
	xxlxor 39, 39, 39
	xxlxor 37, 37, 37
	xxlxor 40, 40, 40
	xxlxor 42, 42, 42
	xxlxor 46, 46, 46
	xxlxor 43, 43, 43
	xxlxor 47, 47, 47
	xxlxor 44, 44, 44
	xxlxor 48, 48, 48
	xxlxor 45, 45, 45
	xxlxor 49, 49, 49
	xxlxor 32, 32, 32
	xxlxor 41, 41, 41
	xxlxor 38, 38, 38
	.p2align	5
.Lp9_attention_BB4_98:                               #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_27 Depth=2
                                        #       Parent Loop BB4_37 Depth=3
                                        #         Parent Loop BB4_96 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	sldi 7, 14, 2
	lxv 24, -32(10)
	addi 14, 14, 16
	lxvx 50, 16, 7
	lxvx 51, 12, 7
	lxvx 31, 5, 7
	add 4, 16, 7
	add 3, 5, 7
	add 7, 12, 7
	lxv 30, 16(7)
	lxv 29, 32(4)
	lxv 28, 16(3)
	lxv 27, 32(7)
	lxv 26, 32(3)
	lxv 25, 48(3)
	xvmaddasp 49, 51, 50
	xvmaddasp 45, 51, 31
	lxv 51, 16(4)
	xvmaddasp 40, 24, 50
	xvmaddasp 37, 24, 31
	lxv 24, -32(28)
	xvmaddasp 44, 30, 28
	xvmaddasp 47, 27, 29
	xvmaddasp 43, 27, 26
	lxv 27, 48(7)
	xvmaddasp 48, 30, 51
	lxv 30, 48(4)
	xvmaddasp 35, 24, 50
	xvmaddasp 11, 24, 31
	xvmaddasp 42, 27, 25
	xvmaddasp 46, 27, 30
	lxv 27, -16(10)
	xvmaddasp 39, 27, 51
	xvmaddasp 36, 27, 28
	lxv 27, 0(10)
	xvmaddasp 33, 27, 29
	xvmaddasp 38, 27, 26
	lxv 27, 16(10)
	addi 10, 10, 64
	xvmaddasp 41, 27, 30
	xvmaddasp 32, 27, 25
	lxv 27, -16(28)
	xvmaddasp 34, 27, 51
	xvmaddasp 10, 27, 28
	lxv 27, 0(28)
	xvmaddasp 13, 27, 29
	xvmaddasp 9, 27, 26
	lxv 27, 16(28)
	addi 28, 28, 64
	xvmaddasp 12, 27, 30
	xvmaddasp 8, 27, 25
	lxv 27, -32(25)
	xvmaddasp 7, 27, 50
	lxv 50, -16(25)
	xvmaddasp 3, 27, 31
	xvmaddasp 6, 50, 51
	xvmaddasp 2, 50, 28
	lxv 50, 0(25)
	lxv 51, 16(25)
	addi 25, 25, 64
	xvmaddasp 5, 50, 29
	xvmaddasp 1, 50, 26
	xvmaddasp 4, 51, 30
	xvmaddasp 0, 51, 25
	bdnz .Lp9_attention_BB4_98
	b .Lp9_attention_BB4_100
	.p2align	4
.Lp9_attention_BB4_99:                               #   in Loop: Header=BB4_96 Depth=4
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
	xxlxor 32, 32, 32
	xxlxor 38, 38, 38
	xxlxor 36, 36, 36
	xxlxor 37, 37, 37
	xxlxor 41, 41, 41
	xxlxor 33, 33, 33
	xxlxor 39, 39, 39
	xxlxor 40, 40, 40
	xxlxor 42, 42, 42
	xxlxor 43, 43, 43
	xxlxor 44, 44, 44
	xxlxor 45, 45, 45
	xxlxor 46, 46, 46
	xxlxor 47, 47, 47
	xxlxor 48, 48, 48
	xxlxor 49, 49, 49
.Lp9_attention_BB4_100:                              #   in Loop: Header=BB4_96 Depth=4
	xvaddsp 48, 49, 48
	xvaddsp 46, 47, 46
	sldi 10, 17, 2
	mr	5, 9
	li 3, 4
	xvaddsp 44, 45, 44
	xvaddsp 42, 43, 42
	xvaddsp 45, 48, 46
	xvaddsp 42, 44, 42
	xxspltw 46, 45, 2
	xxspltw 47, 45, 1
	xxspltw 48, 45, 0
	xxspltw 43, 42, 2
	xxspltw 44, 42, 1
	xvaddsp 45, 45, 46
	xvaddsp 46, 47, 48
	xvaddsp 43, 42, 43
	xxspltw 42, 42, 0
	xvaddsp 42, 44, 42
	xvaddsp 45, 45, 46
	xvaddsp 42, 43, 42
	xxsldwi 45, 45, 45, 3
	xscvspdpn 31, 45
	xxsldwi 42, 42, 42, 2
	stfsux 31, 5, 10
	stxsiwx 42, 5, 3
	bc 4, 1, .Lp9_attention_BB4_95
# %bb.101:                              #   in Loop: Header=BB4_96 Depth=4
	xvaddsp 39, 40, 39
	xvaddsp 33, 33, 41
	mr	3, 5
	xvaddsp 36, 37, 36
	xvaddsp 37, 38, 32
	xvaddsp 33, 39, 33
	xvaddsp 36, 36, 37
	xxspltw 39, 33, 2
	xxspltw 40, 33, 1
	xxspltw 37, 36, 2
	xxspltw 32, 36, 1
	xvaddsp 39, 33, 39
	xxspltw 33, 33, 0
	xvaddsp 37, 36, 37
	xxspltw 36, 36, 0
	xvaddsp 33, 40, 33
	xvaddsp 36, 32, 36
	xvaddsp 33, 39, 33
	xvaddsp 36, 37, 36
	xxsldwi 33, 33, 33, 3
	xxsldwi 36, 36, 36, 2
	xscvspdpn 31, 33
	stfsux 31, 3, 24
	addi 3, 3, 4
	stxsiwx 36, 0, 3
	bc 12, 26, .Lp9_attention_BB4_95
# %bb.102:                              #   in Loop: Header=BB4_96 Depth=4
	xvaddsp 34, 35, 34
	xvaddsp 12, 13, 12
	ld 4, 608(1)                            # 8-byte Folded Reload
	mr	3, 5
	xvaddsp 10, 11, 10
	xvaddsp 8, 9, 8
	xvaddsp 12, 34, 12
	xvaddsp 8, 10, 8
	xxspltw 13, 12, 2
	xxspltw 34, 12, 1
	xxspltw 9, 8, 2
	xxspltw 10, 8, 1
	xvaddsp 13, 12, 13
	xxspltw 12, 12, 0
	xvaddsp 9, 8, 9
	xxspltw 8, 8, 0
	xvaddsp 12, 34, 12
	xvaddsp 8, 10, 8
	xvaddsp 12, 13, 12
	xvaddsp 8, 9, 8
	xxsldwi 12, 12, 12, 3
	xxsldwi 8, 8, 8, 2
	xscvspdpn 12, 12
	stfsux 12, 3, 4
	addi 3, 3, 4
	stfiwx 8, 0, 3
	bc 4, 29, .Lp9_attention_BB4_95
# %bb.103:                              #   in Loop: Header=BB4_96 Depth=4
	xvaddsp 6, 7, 6
	xvaddsp 4, 5, 4
	ld 3, 632(1)                            # 8-byte Folded Reload
	xvaddsp 2, 3, 2
	xvaddsp 0, 1, 0
	xvaddsp 4, 6, 4
	xvaddsp 0, 2, 0
	xxspltw 5, 4, 2
	xxspltw 6, 4, 1
	xxspltw 1, 0, 2
	xxspltw 2, 0, 1
	xvaddsp 5, 4, 5
	xxspltw 4, 4, 0
	xvaddsp 1, 0, 1
	xxspltw 0, 0, 0
	xvaddsp 4, 6, 4
	xvaddsp 0, 2, 0
	xvaddsp 4, 5, 4
	xvaddsp 0, 1, 0
	xxsldwi 4, 4, 4, 3
	xxsldwi 0, 0, 0, 2
	xscvspdpn 4, 4
	stfsux 4, 5, 3
	addi 3, 5, 4
	stfiwx 0, 0, 3
	b .Lp9_attention_BB4_95
.Lp9_attention_BB4_104:                              #   in Loop: Header=BB4_37 Depth=3
	mr	26, 19
	mr	22, 29
	cmpld	26, 22
	blt	0, .Lp9_attention_BB4_66
	b .Lp9_attention_BB4_36
.Lp9_attention_BB4_105:                              #   in Loop: Header=BB4_23 Depth=1
	ld 6, 528(1)                            # 8-byte Folded Reload
	b .Lp9_attention_BB4_107
.Lp9_attention_BB4_106:                              #   in Loop: Header=BB4_23 Depth=1
	ld 3, 416(1)                            # 8-byte Folded Reload
	ld 18, 608(1)                           # 8-byte Folded Reload
	ld 8, 512(1)                            # 8-byte Folded Reload
	ld 5, 504(1)                            # 8-byte Folded Reload
	ld 4, 520(1)                            # 8-byte Folded Reload
	std 3, 952(1)
	ld 3, 424(1)                            # 8-byte Folded Reload
	std 3, 960(1)
	ld 3, 408(1)                            # 8-byte Folded Reload
	std 3, 968(1)
.Lp9_attention_BB4_107:                              #   in Loop: Header=BB4_23 Depth=1
	std 28, 944(1)
	addi 29, 1, 944
	lwz 3, 324(1)
	std 29, 424(1)                          # 8-byte Folded Spill
	mtocrf 128, 3
	bc 12, 2, .Lp9_attention_BB4_187
# %bb.108:                              #   in Loop: Header=BB4_23 Depth=1
	ld 17, 216(1)                           # 8-byte Folded Reload
	ld 16, 392(1)                           # 8-byte Folded Reload
	li 14, 0
	li 22, 0
.Lp9_attention_BB4_109:                              #   Parent Loop BB4_23 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB4_111 Depth 3
                                        #         Child Loop BB4_113 Depth 4
                                        #         Child Loop BB4_159 Depth 4
                                        #         Child Loop BB4_165 Depth 4
                                        #         Child Loop BB4_171 Depth 4
                                        #         Child Loop BB4_176 Depth 4
                                        #         Child Loop BB4_180 Depth 4
                                        #         Child Loop BB4_183 Depth 4
                                        #         Child Loop BB4_122 Depth 4
                                        #         Child Loop BB4_128 Depth 4
                                        #         Child Loop BB4_133 Depth 4
                                        #         Child Loop BB4_137 Depth 4
                                        #         Child Loop BB4_140 Depth 4
                                        #         Child Loop BB4_117 Depth 4
                                        #         Child Loop BB4_142 Depth 4
                                        #         Child Loop BB4_146 Depth 4
                                        #         Child Loop BB4_150 Depth 4
                                        #         Child Loop BB4_153 Depth 4
	ld 3, 232(1)                            # 8-byte Folded Reload
	std 4, 80(1)
	std 6, 48(1)
	ld 4, 496(1)                            # 8-byte Folded Reload
	mr	6, 8
	ld 7, 384(1)                            # 8-byte Folded Reload
	ld 8, 400(1)                            # 8-byte Folded Reload
	mr	9, 26
	mr	10, 30
	std 29, 72(1)
	std 27, 64(1)
	std 22, 56(1)
	std 3, 40(1)
	ld 3, 240(1)                            # 8-byte Folded Reload
	std 3, 32(1)
	ld 3, 488(1)                            # 8-byte Folded Reload
	bl p9_attention_pafm_rows
	cmpldi	3, 0
	std 3, 584(1)                           # 8-byte Folded Spill
	beq	0, .Lp9_attention_BB4_186
# %bb.110:                              #   in Loop: Header=BB4_109 Depth=2
	ld 3, 472(1)                            # 8-byte Folded Reload
	li 19, 0
	mr	28, 17
	mr	26, 16
	std 17, 464(1)                          # 8-byte Folded Spill
	std 16, 552(1)                          # 8-byte Folded Spill
	mulld 3, 3, 14
	std 3, 568(1)                           # 8-byte Folded Spill
	addi 3, 3, -4
	std 3, 560(1)                           # 8-byte Folded Spill
.Lp9_attention_BB4_111:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        # =>    This Loop Header: Depth=3
                                        #         Child Loop BB4_113 Depth 4
                                        #         Child Loop BB4_159 Depth 4
                                        #         Child Loop BB4_165 Depth 4
                                        #         Child Loop BB4_171 Depth 4
                                        #         Child Loop BB4_176 Depth 4
                                        #         Child Loop BB4_180 Depth 4
                                        #         Child Loop BB4_183 Depth 4
                                        #         Child Loop BB4_122 Depth 4
                                        #         Child Loop BB4_128 Depth 4
                                        #         Child Loop BB4_133 Depth 4
                                        #         Child Loop BB4_137 Depth 4
                                        #         Child Loop BB4_140 Depth 4
                                        #         Child Loop BB4_117 Depth 4
                                        #         Child Loop BB4_142 Depth 4
                                        #         Child Loop BB4_146 Depth 4
                                        #         Child Loop BB4_150 Depth 4
                                        #         Child Loop BB4_153 Depth 4
	add 3, 19, 22
	ld 5, 392(1)                            # 8-byte Folded Reload
	sldi 6, 19, 3
	mulld 3, 3, 15
	sldi 3, 3, 2
	add 4, 5, 3
	ld 3, 520(1)                            # 8-byte Folded Reload
	ldx 3, 3, 6
	cmpldi	3, 0
	beq	0, .Lp9_attention_BB4_115
# %bb.112:                              #   in Loop: Header=BB4_111 Depth=3
	std 6, 576(1)                           # 8-byte Folded Spill
	ld 6, 568(1)                            # 8-byte Folded Reload
	std 4, 456(1)                           # 8-byte Folded Spill
	ld 4, 560(1)                            # 8-byte Folded Reload
	addi 16, 3, -1
	li 25, 0
	mr	17, 28
	maddld 6, 24, 19, 6
	maddld 4, 24, 19, 4
	std 6, 448(1)                           # 8-byte Folded Spill
	add 23, 5, 4
	add 5, 5, 4
	mr	6, 16
	mtctr 15
	.p2align	5
.Lp9_attention_BB4_113:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lbzu 7, 1(6)
	cmplwi	7, 0
	bne	0, .Lp9_attention_BB4_119
# %bb.114:                              #   in Loop: Header=BB4_113 Depth=4
	addi 25, 25, 1
	addi 17, 17, 4
	bdnz .Lp9_attention_BB4_113
	b .Lp9_attention_BB4_120
.Lp9_attention_BB4_115:                              #   in Loop: Header=BB4_111 Depth=3
	lfs 0, 0(4)
	li 3, 0
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB4_1
# %bb.116:                              #   in Loop: Header=BB4_111 Depth=3
	addis 4, 2, .Lp9_attention_CPI4_1@toc@ha
	mtctr 15
	lfs 30, .Lp9_attention_CPI4_1@toc@l(4)
	mr	4, 26
	.p2align	5
.Lp9_attention_BB4_117:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	xsmaxcdp 30, 0, 30
	bdz .Lp9_attention_BB4_141
# %bb.118:                              #   in Loop: Header=BB4_117 Depth=4
	lfs 0, 4(4)
	addi 4, 4, 4
	xststdcsp 0, 0, 112
	bc 4, 2, .Lp9_attention_BB4_117
	b .Lp9_attention_BB4_1
.Lp9_attention_BB4_119:                              #   in Loop: Header=BB4_111 Depth=3
	cmpld	25, 15
	bne	0, .Lp9_attention_BB4_157
.Lp9_attention_BB4_120:                              #   in Loop: Header=BB4_111 Depth=3
	addis 3, 2, .Lp9_attention_CPI4_1@toc@ha
	mtctr 15
	lfs 30, .Lp9_attention_CPI4_1@toc@l(3)
	mr	3, 16
	b .Lp9_attention_BB4_122
	.p2align	5
.Lp9_attention_BB4_121:                              #   in Loop: Header=BB4_122 Depth=4
	bdz .Lp9_attention_BB4_126
.Lp9_attention_BB4_122:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lbzu 6, 1(3)
	mr	4, 5
	addi 5, 5, 4
	cmplwi	6, 0
	beq	0, .Lp9_attention_BB4_121
# %bb.123:                              #   in Loop: Header=BB4_122 Depth=4
	lfs 0, 4(4)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB4_297
# %bb.124:                              #   in Loop: Header=BB4_122 Depth=4
	fcmpu 0, 0, 30
	ble 0, .Lp9_attention_BB4_121
# %bb.125:                              #   in Loop: Header=BB4_122 Depth=4
	fmr 30, 0
	b .Lp9_attention_BB4_121
.Lp9_attention_BB4_126:                              #   in Loop: Header=BB4_111 Depth=3
	xxlxor 31, 31, 31
	mr	18, 23
	mr	25, 15
	b .Lp9_attention_BB4_128
	.p2align	5
.Lp9_attention_BB4_127:                              #   in Loop: Header=BB4_128 Depth=4
	xsaddsp 31, 31, 1
	addi 25, 25, -1
	stfs 1, 4(18)
	mr	18, 17
	cmpldi	25, 0
	bc 4, 1, .Lp9_attention_BB4_130
.Lp9_attention_BB4_128:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lbzu 3, 1(16)
	addi 17, 18, 4
	xxlxor 1, 1, 1
	cmplwi	3, 0
	beq	0, .Lp9_attention_BB4_127
# %bb.129:                              #   in Loop: Header=BB4_128 Depth=4
	lfs 0, 4(18)
	xssubsp 1, 0, 30
	bl power_expf
	nop
	b .Lp9_attention_BB4_127
.Lp9_attention_BB4_130:                              #   in Loop: Header=BB4_111 Depth=3
	ld 18, 608(1)                           # 8-byte Folded Reload
	lwz 3, 228(1)
	ld 17, 464(1)                           # 8-byte Folded Reload
	ld 16, 552(1)                           # 8-byte Folded Reload
	li 5, 0
	mtocrf 128, 3
	bc 12, 0, .Lp9_attention_BB4_139
# %bb.131:                              #   in Loop: Header=BB4_111 Depth=3
	xscvdpspn 0, 31
	lwz 3, 212(1)
	li 4, 0
	mtocrf 128, 3
	bc 12, 0, .Lp9_attention_BB4_136
# %bb.132:                              #   in Loop: Header=BB4_111 Depth=3
	ld 4, 152(1)                            # 8-byte Folded Reload
	xxspltw 1, 0, 0
	mr	3, 28
	mtctr 4
	.p2align	5
.Lp9_attention_BB4_133:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lxv 2, -96(3)
	lxv 12, 64(3)
	lxv 3, -80(3)
	lxv 4, -64(3)
	xvdivsp 2, 2, 1
	lxv 5, -48(3)
	lxv 6, -32(3)
	lxv 7, -16(3)
	lxv 8, 0(3)
	lxv 9, 16(3)
	lxv 10, 32(3)
	lxv 11, 48(3)
	lxv 13, 80(3)
	xvdivsp 3, 3, 1
	stxv 2, -96(3)
	xvdivsp 2, 12, 1
	xvdivsp 4, 4, 1
	xvdivsp 5, 5, 1
	stxv 3, -80(3)
	xvdivsp 6, 6, 1
	xvdivsp 7, 7, 1
	xvdivsp 8, 8, 1
	stxv 4, -64(3)
	stxv 5, -48(3)
	stxv 6, -32(3)
	stxv 7, -16(3)
	stxv 8, 0(3)
	stxv 2, 64(3)
	xvdivsp 9, 9, 1
	xvdivsp 10, 10, 1
	xvdivsp 11, 11, 1
	xvdivsp 2, 13, 1
	stxv 9, 16(3)
	stxv 10, 32(3)
	stxv 11, 48(3)
	stxv 2, 80(3)
	addi 3, 3, 192
	bdnz .Lp9_attention_BB4_133
# %bb.134:                              #   in Loop: Header=BB4_111 Depth=3
	lwz 3, 160(1)
	mtocrf 128, 3
	bc 12, 2, .Lp9_attention_BB4_154
# %bb.135:                              #   in Loop: Header=BB4_111 Depth=3
	ld 5, 128(1)                            # 8-byte Folded Reload
	lwz 3, 124(1)
	mr	4, 5
	mtocrf 128, 3
	bc 12, 0, .Lp9_attention_BB4_139
.Lp9_attention_BB4_136:                              #   in Loop: Header=BB4_111 Depth=3
	ld 5, 184(1)                            # 8-byte Folded Reload
	sldi 3, 4, 2
	xxspltw 0, 0, 0
	add 3, 26, 3
	sub	4, 5, 4
	rldicl 4, 4, 62, 2
	addi 4, 4, 1
	mtctr 4
	.p2align	5
.Lp9_attention_BB4_137:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lxv 1, 0(3)
	xvdivsp 1, 1, 0
	stxv 1, 0(3)
	addi 3, 3, 16
	bdnz .Lp9_attention_BB4_137
# %bb.138:                              #   in Loop: Header=BB4_111 Depth=3
	ld 5, 192(1)                            # 8-byte Folded Reload
	lwz 3, 180(1)
	mtocrf 128, 3
	bc 12, 2, .Lp9_attention_BB4_154
.Lp9_attention_BB4_139:                              #   in Loop: Header=BB4_111 Depth=3
	sub	4, 15, 5
	sldi 3, 5, 2
	mtctr 4
	add 3, 23, 3
	.p2align	5
.Lp9_attention_BB4_140:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfsu 0, 4(3)
	xsdivsp 0, 0, 31
	stfs 0, 0(3)
	bdnz .Lp9_attention_BB4_140
	b .Lp9_attention_BB4_154
.Lp9_attention_BB4_141:                              #   in Loop: Header=BB4_111 Depth=3
	xxlxor 31, 31, 31
	mr	29, 26
	mr	25, 15
	std 6, 576(1)                           # 8-byte Folded Spill
	.p2align	5
.Lp9_attention_BB4_142:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfs 0, 0(29)
	xssubsp 1, 0, 30
	bl power_expf
	nop
	xsaddsp 31, 31, 1
	addi 25, 25, -1
	stfs 1, 0(29)
	addi 29, 29, 4
	cmpldi	25, 0
	bc 12, 1, .Lp9_attention_BB4_142
# %bb.143:                              #   in Loop: Header=BB4_111 Depth=3
	lwz 3, 228(1)
	ld 29, 424(1)                           # 8-byte Folded Reload
	li 5, 0
	mtocrf 128, 3
	bc 12, 0, .Lp9_attention_BB4_152
# %bb.144:                              #   in Loop: Header=BB4_111 Depth=3
	xscvdpspn 0, 31
	lwz 3, 212(1)
	li 4, 0
	mtocrf 128, 3
	bc 12, 0, .Lp9_attention_BB4_149
# %bb.145:                              #   in Loop: Header=BB4_111 Depth=3
	ld 4, 152(1)                            # 8-byte Folded Reload
	xxspltw 1, 0, 0
	mr	3, 28
	mtctr 4
	.p2align	5
.Lp9_attention_BB4_146:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lxv 2, -96(3)
	lxv 12, 64(3)
	lxv 3, -80(3)
	lxv 4, -64(3)
	xvdivsp 2, 2, 1
	lxv 5, -48(3)
	lxv 6, -32(3)
	lxv 7, -16(3)
	lxv 8, 0(3)
	lxv 9, 16(3)
	lxv 10, 32(3)
	lxv 11, 48(3)
	lxv 13, 80(3)
	xvdivsp 3, 3, 1
	stxv 2, -96(3)
	xvdivsp 2, 12, 1
	xvdivsp 4, 4, 1
	xvdivsp 5, 5, 1
	stxv 3, -80(3)
	xvdivsp 6, 6, 1
	xvdivsp 7, 7, 1
	xvdivsp 8, 8, 1
	stxv 4, -64(3)
	stxv 5, -48(3)
	stxv 6, -32(3)
	stxv 7, -16(3)
	stxv 8, 0(3)
	stxv 2, 64(3)
	xvdivsp 9, 9, 1
	xvdivsp 10, 10, 1
	xvdivsp 11, 11, 1
	xvdivsp 2, 13, 1
	stxv 9, 16(3)
	stxv 10, 32(3)
	stxv 11, 48(3)
	stxv 2, 80(3)
	addi 3, 3, 192
	bdnz .Lp9_attention_BB4_146
# %bb.147:                              #   in Loop: Header=BB4_111 Depth=3
	lwz 3, 160(1)
	mtocrf 128, 3
	bc 12, 2, .Lp9_attention_BB4_154
# %bb.148:                              #   in Loop: Header=BB4_111 Depth=3
	ld 5, 128(1)                            # 8-byte Folded Reload
	lwz 3, 124(1)
	mr	4, 5
	mtocrf 128, 3
	bc 12, 0, .Lp9_attention_BB4_152
.Lp9_attention_BB4_149:                              #   in Loop: Header=BB4_111 Depth=3
	ld 5, 184(1)                            # 8-byte Folded Reload
	sldi 3, 4, 2
	xxspltw 0, 0, 0
	sub	4, 5, 4
	rldicl 4, 4, 62, 2
	addi 4, 4, 1
	mtctr 4
	.p2align	5
.Lp9_attention_BB4_150:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lxvx 1, 26, 3
	xvdivsp 1, 1, 0
	stxvx 1, 26, 3
	addi 3, 3, 16
	bdnz .Lp9_attention_BB4_150
# %bb.151:                              #   in Loop: Header=BB4_111 Depth=3
	ld 5, 192(1)                            # 8-byte Folded Reload
	lwz 3, 180(1)
	mtocrf 128, 3
	bc 12, 2, .Lp9_attention_BB4_154
.Lp9_attention_BB4_152:                              #   in Loop: Header=BB4_111 Depth=3
	sub	4, 15, 5
	sldi 3, 5, 2
	mtctr 4
	.p2align	5
.Lp9_attention_BB4_153:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfsx 0, 26, 3
	xsdivsp 0, 0, 31
	stfsx 0, 26, 3
	addi 3, 3, 4
	bdnz .Lp9_attention_BB4_153
.Lp9_attention_BB4_154:                              #   in Loop: Header=BB4_111 Depth=3
	lwz 3, 644(1)
	mtocrf 128, 3
	bc 12, 2, .Lp9_attention_BB4_156
.Lp9_attention_BB4_155:                              #   in Loop: Header=BB4_111 Depth=3
	ld 3, 576(1)                            # 8-byte Folded Reload
	ld 5, 624(1)                            # 8-byte Folded Reload
	li 4, 0
	ldx 3, 29, 3
	bl memset
	nop
.Lp9_attention_BB4_156:                              #   in Loop: Header=BB4_111 Depth=3
	ld 3, 584(1)                            # 8-byte Folded Reload
	addi 19, 19, 1
	add 26, 26, 24
	add 28, 28, 24
	cmpld	19, 3
	bne	0, .Lp9_attention_BB4_111
	b .Lp9_attention_BB4_186
.Lp9_attention_BB4_157:                              #   in Loop: Header=BB4_111 Depth=3
	ld 16, 552(1)                           # 8-byte Folded Reload
	mr	23, 15
	bge 0, .Lp9_attention_BB4_162
# %bb.158:                              #   in Loop: Header=BB4_111 Depth=3
	sub	6, 15, 25
	add 5, 3, 15
	mr	23, 15
	mtctr 6
	.p2align	5
.Lp9_attention_BB4_159:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lbzu 6, -1(5)
	cmplwi	6, 0
	bne	0, .Lp9_attention_BB4_162
# %bb.160:                              #   in Loop: Header=BB4_159 Depth=4
	addi 23, 23, -1
	bdnz .Lp9_attention_BB4_159
# %bb.161:                              #   in Loop: Header=BB4_111 Depth=3
	mr	29, 25
	b .Lp9_attention_BB4_185
.Lp9_attention_BB4_162:                              #   in Loop: Header=BB4_111 Depth=3
	cmpld	23, 25
	mr	29, 25
	beq	0, .Lp9_attention_BB4_185
# %bb.163:                              #   in Loop: Header=BB4_111 Depth=3
	add 3, 25, 3
	sldi 9, 25, 2
	sub	8, 23, 25
	addi 16, 3, -1
	add 3, 9, 4
	ld 4, 392(1)                            # 8-byte Folded Reload
	mtctr 8
	add 18, 4, 3
	addis 3, 2, .Lp9_attention_CPI4_1@toc@ha
	lfs 30, .Lp9_attention_CPI4_1@toc@l(3)
	mr	3, 16
	mr	4, 18
	b .Lp9_attention_BB4_165
	.p2align	5
.Lp9_attention_BB4_164:                              #   in Loop: Header=BB4_165 Depth=4
	bdz .Lp9_attention_BB4_169
.Lp9_attention_BB4_165:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lbzu 6, 1(3)
	mr	5, 4
	addi 4, 4, 4
	cmplwi	6, 0
	beq	0, .Lp9_attention_BB4_164
# %bb.166:                              #   in Loop: Header=BB4_165 Depth=4
	lfs 0, 4(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB4_297
# %bb.167:                              #   in Loop: Header=BB4_165 Depth=4
	fcmpu 0, 0, 30
	ble 0, .Lp9_attention_BB4_164
# %bb.168:                              #   in Loop: Header=BB4_165 Depth=4
	fmr 30, 0
	b .Lp9_attention_BB4_164
.Lp9_attention_BB4_169:                              #   in Loop: Header=BB4_111 Depth=3
	std 14, 440(1)                          # 8-byte Folded Spill
	xxlxor 31, 31, 31
	mr	14, 8
	std 9, 408(1)                           # 8-byte Folded Spill
	std 18, 416(1)                          # 8-byte Folded Spill
	std 8, 432(1)                           # 8-byte Folded Spill
	b .Lp9_attention_BB4_171
	.p2align	5
.Lp9_attention_BB4_170:                              #   in Loop: Header=BB4_171 Depth=4
	xsaddsp 31, 31, 1
	addi 14, 14, -1
	stfs 1, 4(18)
	mr	18, 29
	cmpldi	14, 0
	bc 4, 1, .Lp9_attention_BB4_173
.Lp9_attention_BB4_171:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lbzu 3, 1(16)
	addi 29, 18, 4
	xxlxor 1, 1, 1
	cmplwi	3, 0
	beq	0, .Lp9_attention_BB4_170
# %bb.172:                              #   in Loop: Header=BB4_171 Depth=4
	lfs 0, 4(18)
	xssubsp 1, 0, 30
	bl power_expf
	nop
	b .Lp9_attention_BB4_170
.Lp9_attention_BB4_173:                              #   in Loop: Header=BB4_111 Depth=3
	ld 8, 432(1)                            # 8-byte Folded Reload
	ld 18, 608(1)                           # 8-byte Folded Reload
	ld 14, 440(1)                           # 8-byte Folded Reload
	ld 16, 552(1)                           # 8-byte Folded Reload
	li 3, 0
	cmpldi	8, 4
	blt	0, .Lp9_attention_BB4_182
# %bb.174:                              #   in Loop: Header=BB4_111 Depth=3
	xscvdpspn 0, 31
	cmpldi	8, 48
	blt	0, .Lp9_attention_BB4_179
# %bb.175:                              #   in Loop: Header=BB4_111 Depth=3
	ld 6, 112(1)                            # 8-byte Folded Reload
	xxspltw 1, 0, 0
	mulhdu 3, 8, 6
	rldicl 3, 3, 59, 5
	mulli 3, 3, 48
	addi 5, 3, -48
	sub	4, 8, 3
	mulhdu 5, 5, 6
	rldicl 5, 5, 59, 5
	addi 5, 5, 1
	mtctr 5
.Lp9_attention_BB4_176:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lxv 2, -96(17)
	lxv 12, 64(17)
	lxv 3, -80(17)
	lxv 4, -64(17)
	xvdivsp 2, 2, 1
	lxv 5, -48(17)
	lxv 6, -32(17)
	lxv 7, -16(17)
	lxv 8, 0(17)
	lxv 9, 16(17)
	lxv 10, 32(17)
	lxv 11, 48(17)
	lxv 13, 80(17)
	xvdivsp 3, 3, 1
	stxv 2, -96(17)
	xvdivsp 2, 12, 1
	xvdivsp 4, 4, 1
	xvdivsp 5, 5, 1
	stxv 3, -80(17)
	xvdivsp 6, 6, 1
	xvdivsp 7, 7, 1
	xvdivsp 8, 8, 1
	stxv 4, -64(17)
	stxv 5, -48(17)
	stxv 6, -32(17)
	stxv 7, -16(17)
	stxv 8, 0(17)
	stxv 2, 64(17)
	xvdivsp 9, 9, 1
	xvdivsp 10, 10, 1
	xvdivsp 11, 11, 1
	xvdivsp 2, 13, 1
	stxv 9, 16(17)
	stxv 10, 32(17)
	stxv 11, 48(17)
	stxv 2, 80(17)
	addi 17, 17, 192
	bdnz .Lp9_attention_BB4_176
# %bb.177:                              #   in Loop: Header=BB4_111 Depth=3
	cmpldi	4, 0
	beq	0, .Lp9_attention_BB4_184
# %bb.178:                              #   in Loop: Header=BB4_111 Depth=3
	cmpldi	4, 4
	blt	0, .Lp9_attention_BB4_182
.Lp9_attention_BB4_179:                              #   in Loop: Header=BB4_111 Depth=3
	mr	5, 3
	rldicr 3, 8, 0, 61
	ld 6, 448(1)                            # 8-byte Folded Reload
	ld 7, 408(1)                            # 8-byte Folded Reload
	xxspltw 0, 0, 0
	add 6, 7, 6
	sldi 4, 5, 2
	sub	5, 3, 5
	addi 5, 5, -4
	ld 7, 392(1)                            # 8-byte Folded Reload
	rldicl 5, 5, 62, 2
	addi 5, 5, 1
	add 6, 7, 6
	mtctr 5
	add 4, 6, 4
.Lp9_attention_BB4_180:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lxv 1, 0(4)
	xvdivsp 1, 1, 0
	stxv 1, 0(4)
	addi 4, 4, 16
	bdnz .Lp9_attention_BB4_180
# %bb.181:                              #   in Loop: Header=BB4_111 Depth=3
	cmpld	8, 3
	beq	0, .Lp9_attention_BB4_184
.Lp9_attention_BB4_182:                              #   in Loop: Header=BB4_111 Depth=3
	sldi 4, 3, 2
	sub	3, 23, 3
	ld 5, 416(1)                            # 8-byte Folded Reload
	sub	3, 3, 25
	mtctr 3
	add 4, 5, 4
	.p2align	5
.Lp9_attention_BB4_183:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_109 Depth=2
                                        #       Parent Loop BB4_111 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfsu 0, 4(4)
	xsdivsp 0, 0, 31
	stfs 0, 0(4)
	bdnz .Lp9_attention_BB4_183
.Lp9_attention_BB4_184:                              #   in Loop: Header=BB4_111 Depth=3
	mr	29, 23
.Lp9_attention_BB4_185:                              #   in Loop: Header=BB4_111 Depth=3
	sldi 5, 25, 2
	ld 25, 456(1)                           # 8-byte Folded Reload
	li 4, 0
	mr	3, 25
	bl memset
	nop
	sldi 3, 29, 2
	sub	4, 15, 29
	add 3, 25, 3
	sldi 5, 4, 2
	li 4, 0
	bl memset
	nop
	ld 29, 424(1)                           # 8-byte Folded Reload
	ld 17, 464(1)                           # 8-byte Folded Reload
	lwz 3, 644(1)
	mtocrf 128, 3
	bc 4, 2, .Lp9_attention_BB4_155
	b .Lp9_attention_BB4_156
.Lp9_attention_BB4_186:                              #   in Loop: Header=BB4_109 Depth=2
	ld 3, 472(1)                            # 8-byte Folded Reload
	ld 6, 528(1)                            # 8-byte Folded Reload
	addi 22, 22, 4
	ld 26, 536(1)                           # 8-byte Folded Reload
	ld 8, 512(1)                            # 8-byte Folded Reload
	addi 14, 14, 1
	add 16, 16, 3
	ld 5, 504(1)                            # 8-byte Folded Reload
	ld 4, 520(1)                            # 8-byte Folded Reload
	add 17, 17, 3
	cmpld	22, 6
	blt	0, .Lp9_attention_BB4_109
.Lp9_attention_BB4_187:                              #   in Loop: Header=BB4_23 Depth=1
	ld 3, 976(1)
	ld 28, 488(1)                           # 8-byte Folded Reload
	bc 4, 12, .Lp9_attention_BB4_233
# %bb.188:                              #   in Loop: Header=BB4_23 Depth=1
	ld 5, 240(1)                            # 8-byte Folded Reload
	ld 14, 984(1)
	ld 11, 1000(1)
	mulld 4, 5, 26
	std 4, 568(1)                           # 8-byte Folded Spill
	ld 4, 232(1)                            # 8-byte Folded Reload
	sub	4, 5, 4
	mulld 19, 4, 26
	mulld 4, 5, 30
	ld 5, 168(1)                            # 8-byte Folded Reload
	sldi 4, 4, 2
	add 16, 5, 4
	ld 4, 992(1)
	std 4, 560(1)                           # 8-byte Folded Spill
	lwz 4, 164(1)
	ld 22, 368(1)                           # 8-byte Folded Reload
	std 19, 576(1)                          # 8-byte Folded Spill
	std 16, 448(1)                          # 8-byte Folded Spill
	mtocrf 128, 4
	bc 4, 2, .Lp9_attention_BB4_234
# %bb.189:                              #   in Loop: Header=BB4_23 Depth=1
	ld 28, 200(1)                           # 8-byte Folded Reload
	b .Lp9_attention_BB4_192
	.p2align	4
.Lp9_attention_BB4_190:                              #   in Loop: Header=BB4_192 Depth=2
	ld 3, 496(1)                            # 8-byte Folded Reload
	ld 4, 552(1)                            # 8-byte Folded Reload
	ld 5, 584(1)                            # 8-byte Folded Reload
	ld 16, 448(1)                           # 8-byte Folded Reload
	mulld 3, 4, 3
	ld 4, 488(1)                            # 8-byte Folded Reload
	sldi 3, 3, 2
	add 3, 4, 3
	ld 4, 568(1)                            # 8-byte Folded Reload
	add 4, 5, 4
	mulld 4, 4, 30
	sldi 4, 4, 2
	add 3, 3, 4
.Lp9_attention_BB4_191:                              #   in Loop: Header=BB4_192 Depth=2
	ld 4, 328(1)                            # 8-byte Folded Reload
	ld 28, 432(1)                           # 8-byte Folded Reload
	cmpld	28, 4
	bge 0, .Lp9_attention_BB4_276
.Lp9_attention_BB4_192:                              #   Parent Loop BB4_23 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB4_196 Depth 3
                                        #       Child Loop BB4_200 Depth 3
                                        #         Child Loop BB4_201 Depth 4
                                        #         Child Loop BB4_203 Depth 4
                                        #       Child Loop BB4_209 Depth 3
                                        #         Child Loop BB4_220 Depth 4
                                        #         Child Loop BB4_214 Depth 4
                                        #           Child Loop BB4_216 Depth 5
                                        #         Child Loop BB4_225 Depth 4
                                        #           Child Loop BB4_227 Depth 5
	ld 4, 336(1)                            # 8-byte Folded Reload
	sub	5, 15, 28
	add 6, 28, 4
	cmpld	5, 4
	isellt	23, 15, 6
	std 6, 432(1)                           # 8-byte Folded Spill
	sub	12, 23, 28
	cmpld 1, 23, 28
	sldi 0, 12, 7
	ble 1, .Lp9_attention_BB4_205
# %bb.193:                              #   in Loop: Header=BB4_192 Depth=2
	lwz 4, 644(1)
	mtocrf 128, 4
	bc 12, 2, .Lp9_attention_BB4_205
# %bb.194:                              #   in Loop: Header=BB4_192 Depth=2
	ld 4, 336(1)                            # 8-byte Folded Reload
	cmpld	4, 5
	isellt	4, 4, 5
	andi. 4, 4, 1
	mr	4, 28
	bc 4, 1, .Lp9_attention_BB4_198
# %bb.195:                              #   in Loop: Header=BB4_192 Depth=2
	mulld 4, 28, 31
	ld 29, 648(1)                           # 8-byte Folded Reload
	li 6, 0
	sldi 4, 4, 2
	add 4, 16, 4
	.p2align	5
.Lp9_attention_BB4_196:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_192 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	rldicl 7, 6, 59, 5
	sldi 9, 6, 2
	addi 6, 6, 32
	lxvx 0, 4, 9
	mulld 8, 0, 7
	add 10, 4, 9
	maddld 7, 0, 7, 29
	cmpld	6, 30
	stxvx 0, 29, 8
	lxv 0, 112(10)
	stxv 0, 112(7)
	lxv 0, 96(10)
	stxv 0, 96(7)
	lxv 0, 80(10)
	stxv 0, 80(7)
	lxv 0, 64(10)
	stxv 0, 64(7)
	lxv 0, 48(10)
	stxv 0, 48(7)
	lxv 0, 32(10)
	stxv 0, 32(7)
	lxv 0, 16(10)
	stxv 0, 16(7)
	blt	0, .Lp9_attention_BB4_196
# %bb.197:                              #   in Loop: Header=BB4_192 Depth=2
	addi 4, 28, 1
.Lp9_attention_BB4_198:                              #   in Loop: Header=BB4_192 Depth=2
	cmpldi	5, 1
	beq	0, .Lp9_attention_BB4_205
# %bb.199:                              #   in Loop: Header=BB4_192 Depth=2
	ld 7, 256(1)                            # 8-byte Folded Reload
	addi 6, 4, 1
	maddld 5, 7, 4, 22
	mulld 6, 7, 6
	.p2align	4
.Lp9_attention_BB4_200:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_192 Depth=2
                                        # =>    This Loop Header: Depth=3
                                        #         Child Loop BB4_201 Depth 4
                                        #         Child Loop BB4_203 Depth 4
	sub	7, 4, 28
	ld 8, 648(1)                            # 8-byte Folded Reload
	mr	25, 28
	mr	9, 5
	sldi 7, 7, 7
	add 7, 8, 7
	li 8, 0
	.p2align	5
.Lp9_attention_BB4_201:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_192 Depth=2
                                        #       Parent Loop BB4_200 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	rldicl 10, 8, 59, 5
	lxv 0, 0(9)
	addi 8, 8, 32
	mulld 29, 0, 10
	maddld 10, 0, 10, 7
	cmpld	8, 30
	stxvx 0, 7, 29
	lxv 0, 112(9)
	stxv 0, 112(10)
	lxv 0, 96(9)
	stxv 0, 96(10)
	lxv 0, 80(9)
	stxv 0, 80(10)
	lxv 0, 64(9)
	stxv 0, 64(10)
	lxv 0, 48(9)
	stxv 0, 48(10)
	lxv 0, 32(9)
	stxv 0, 32(10)
	lxv 0, 16(9)
	addi 9, 9, 128
	stxv 0, 16(10)
	blt	0, .Lp9_attention_BB4_201
# %bb.202:                              #   in Loop: Header=BB4_200 Depth=3
	addi 7, 7, 128
	li 8, 0
	mr	9, 6
	.p2align	5
.Lp9_attention_BB4_203:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_192 Depth=2
                                        #       Parent Loop BB4_200 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	rldicl 10, 8, 59, 5
	lxvx 0, 22, 9
	add 28, 22, 9
	addi 8, 8, 32
	addi 9, 9, 128
	cmpld	8, 30
	mulld 29, 0, 10
	maddld 10, 0, 10, 7
	stxvx 0, 7, 29
	lxv 0, 112(28)
	stxv 0, 112(10)
	lxv 0, 96(28)
	stxv 0, 96(10)
	lxv 0, 80(28)
	stxv 0, 80(10)
	lxv 0, 64(28)
	stxv 0, 64(10)
	lxv 0, 48(28)
	stxv 0, 48(10)
	lxv 0, 32(28)
	stxv 0, 32(10)
	lxv 0, 16(28)
	stxv 0, 16(10)
	blt	0, .Lp9_attention_BB4_203
# %bb.204:                              #   in Loop: Header=BB4_200 Depth=3
	ld 7, 296(1)                            # 8-byte Folded Reload
	addi 4, 4, 2
	mr	28, 25
	cmpld	4, 23
	add 5, 5, 7
	add 6, 6, 7
	blt	0, .Lp9_attention_BB4_200
	.p2align	4
.Lp9_attention_BB4_205:                              #   in Loop: Header=BB4_192 Depth=2
	ld 6, 528(1)                            # 8-byte Folded Reload
	lwz 4, 324(1)
	mtocrf 128, 4
	bc 12, 2, .Lp9_attention_BB4_191
# %bb.206:                              #   in Loop: Header=BB4_192 Depth=2
	ld 4, 392(1)                            # 8-byte Folded Reload
	sldi 3, 28, 2
	std 28, 464(1)                          # 8-byte Folded Spill
	cmpd	23, 28
	li 28, 0
	add 3, 4, 3
	std 23, 456(1)                          # 8-byte Folded Spill
	std 3, 440(1)                           # 8-byte Folded Spill
	b .Lp9_attention_BB4_209
	.p2align	4
.Lp9_attention_BB4_207:                              #   in Loop: Header=BB4_209 Depth=3
	ld 26, 536(1)                           # 8-byte Folded Reload
	ld 22, 368(1)                           # 8-byte Folded Reload
.Lp9_attention_BB4_208:                              #   in Loop: Header=BB4_209 Depth=3
	ld 6, 528(1)                            # 8-byte Folded Reload
	addi 28, 28, 4
	ld 19, 576(1)                           # 8-byte Folded Reload
	cmpld 5, 28, 6
	bge 5, .Lp9_attention_BB4_190
.Lp9_attention_BB4_209:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_192 Depth=2
                                        # =>    This Loop Header: Depth=3
                                        #         Child Loop BB4_220 Depth 4
                                        #         Child Loop BB4_214 Depth 4
                                        #           Child Loop BB4_216 Depth 5
                                        #         Child Loop BB4_225 Depth 4
                                        #           Child Loop BB4_227 Depth 5
	divdu 7, 28, 26
	ld 17, 512(1)                           # 8-byte Folded Reload
	ld 16, 504(1)                           # 8-byte Folded Reload
	mr	11, 19
	sub	8, 6, 28
	li 5, 0
	li 6, 3
	cmpldi 6, 8, 2
	cmpldi 5, 8, 3
	cmpldi 7, 8, 4
	mulld 3, 7, 26
	sub	4, 28, 3
	mulld 3, 7, 17
	std 4, 584(1)                           # 8-byte Folded Spill
	add 4, 4, 19
	mulld 4, 4, 30
	sldi 3, 3, 2
	add 3, 16, 3
	sldi 4, 4, 2
	add 19, 3, 4
	li 4, 2
	std 19, 944(1)
	isel 4, 4, 5, 25
	isel 5, 6, 5, 21
	lwz 3, 484(1)
	std 7, 552(1)                           # 8-byte Folded Spill
	or 4, 4, 28
	or 5, 5, 28
	rotlwi	3, 3, 24
	mtocrf 32, 3
	subfic 3, 8, 1
	li 3, 1
	subfe 3, 3, 3
	neg 3, 3
	or 3, 28, 3
	bc 12, 10, .Lp9_attention_BB4_212
# %bb.210:                              #   in Loop: Header=BB4_209 Depth=3
	ld 22, 400(1)                           # 8-byte Folded Reload
	ld 18, 384(1)                           # 8-byte Folded Reload
	divdu 10, 5, 26
	mr	25, 26
	ld 14, 568(1)                           # 8-byte Folded Reload
	ld 23, 488(1)                           # 8-byte Folded Reload
	maddld 6, 7, 22, 18
	divdu 7, 4, 26
	std 6, 912(1)
	mulld 29, 10, 26
	mulld 6, 7, 26
	divdu 26, 3, 26
	maddld 9, 7, 22, 18
	sub	5, 5, 29
	std 9, 928(1)
	sub	4, 4, 6
	maddld 9, 26, 22, 18
	std 9, 920(1)
	mulld 9, 26, 25
	ld 25, 496(1)                           # 8-byte Folded Reload
	mulld 6, 7, 25
	mulld 7, 7, 17
	mulld 29, 10, 25
	sub	3, 3, 9
	add 9, 4, 14
	add 4, 4, 11
	mulld 9, 9, 30
	mulld 4, 4, 30
	sldi 6, 6, 2
	sldi 7, 7, 2
	add 6, 23, 6
	add 7, 16, 7
	sldi 9, 9, 2
	sldi 4, 4, 2
	add 6, 6, 9
	sldi 9, 29, 2
	add 29, 23, 9
	add 9, 7, 4
	add 4, 5, 14
	std 6, 560(1)                           # 8-byte Folded Spill
	mulld 6, 26, 25
	mulld 4, 4, 30
	std 9, 960(1)
	sldi 6, 6, 2
	sldi 4, 4, 2
	add 6, 23, 6
	add 29, 29, 4
	add 4, 3, 14
	add 3, 3, 11
	mulld 4, 4, 30
	mulld 3, 3, 30
	sldi 4, 4, 2
	sldi 3, 3, 2
	add 14, 6, 4
	mulld 4, 26, 17
	sldi 4, 4, 2
	add 4, 16, 4
	add 6, 4, 3
	mulld 3, 10, 17
	add 4, 5, 11
	mulld 4, 4, 30
	std 6, 952(1)
	sldi 3, 3, 2
	add 3, 16, 3
	sldi 4, 4, 2
	add 7, 3, 4
	maddld 3, 10, 22, 18
	std 7, 968(1)
	std 3, 936(1)
	li 3, 4
	isel 3, 8, 3, 28
	bc 4, 5, .Lp9_attention_BB4_219
# %bb.211:                              #   in Loop: Header=BB4_209 Depth=3
	ld 18, 608(1)                           # 8-byte Folded Reload
	ld 22, 368(1)                           # 8-byte Folded Reload
	ld 23, 456(1)                           # 8-byte Folded Reload
	mr	11, 29
	b .Lp9_attention_BB4_213
	.p2align	4
.Lp9_attention_BB4_212:                              #   in Loop: Header=BB4_209 Depth=3
	divdu 6, 4, 26
	ld 25, 496(1)                           # 8-byte Folded Reload
	ld 14, 568(1)                           # 8-byte Folded Reload
	ld 23, 488(1)                           # 8-byte Folded Reload
	mulld 9, 6, 26
	divdu 7, 5, 26
	divdu 10, 3, 26
	mulld 29, 7, 26
	mulld 26, 10, 26
	sub	4, 4, 9
	mulld 9, 6, 25
	sub	3, 3, 26
	mulld 6, 6, 17
	sub	5, 5, 29
	mulld 29, 7, 25
	add 26, 4, 14
	add 4, 4, 11
	mulld 26, 26, 30
	mulld 4, 4, 30
	sldi 9, 9, 2
	sldi 6, 6, 2
	add 9, 23, 9
	add 6, 16, 6
	sldi 26, 26, 2
	sldi 4, 4, 2
	add 9, 9, 26
	mulld 26, 10, 25
	std 9, 560(1)                           # 8-byte Folded Spill
	sldi 9, 29, 2
	add 29, 23, 9
	add 9, 6, 4
	add 4, 5, 14
	sldi 6, 26, 2
	mulld 4, 4, 30
	std 9, 960(1)
	add 6, 23, 6
	ld 23, 456(1)                           # 8-byte Folded Reload
	sldi 4, 4, 2
	add 29, 29, 4
	add 4, 3, 14
	add 3, 3, 11
	mulld 4, 4, 30
	mulld 3, 3, 30
	sldi 4, 4, 2
	sldi 3, 3, 2
	add 14, 6, 4
	mulld 6, 10, 17
	mulld 4, 7, 17
	sldi 6, 6, 2
	sldi 4, 4, 2
	add 6, 16, 6
	add 4, 16, 4
	add 6, 6, 3
	add 3, 5, 11
	mr	11, 29
	mulld 3, 3, 30
	std 6, 952(1)
	sldi 3, 3, 2
	add 7, 4, 3
	ld 3, 520(1)                            # 8-byte Folded Reload
	std 7, 968(1)
	stxv 62, 16(3)
	stxvx 62, 0, 3
	li 3, 4
	isel 3, 8, 3, 28
	bc 4, 5, .Lp9_attention_BB4_222
.Lp9_attention_BB4_213:                              #   in Loop: Header=BB4_209 Depth=3
	ld 26, 536(1)                           # 8-byte Folded Reload
	li 4, 0
.Lp9_attention_BB4_214:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_192 Depth=2
                                        #       Parent Loop BB4_209 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB4_216 Depth 5
	ld 10, 520(1)                           # 8-byte Folded Reload
	sldi 5, 4, 3
	ldx 5, 10, 5
	cmpldi 7, 5, 0
	beq	7, .Lp9_attention_BB4_222
# %bb.215:                              #   in Loop: Header=BB4_214 Depth=4
	ld 10, 464(1)                           # 8-byte Folded Reload
	.p2align	5
.Lp9_attention_BB4_216:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_192 Depth=2
                                        #       Parent Loop BB4_209 Depth=3
                                        #         Parent Loop BB4_214 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	lbzx 29, 5, 10
	cmplwi 7, 29, 0
	bne	7, .Lp9_attention_BB4_222
# %bb.217:                              #   in Loop: Header=BB4_216 Depth=5
	addi 10, 10, 1
	cmpld 7, 10, 23
	bne	7, .Lp9_attention_BB4_216
# %bb.218:                              #   in Loop: Header=BB4_214 Depth=4
	addi 4, 4, 1
	cmpld 7, 4, 3
	bne	7, .Lp9_attention_BB4_214
	b .Lp9_attention_BB4_208
	.p2align	4
.Lp9_attention_BB4_219:                              #   in Loop: Header=BB4_209 Depth=3
	ld 10, 352(1)                           # 8-byte Folded Reload
	ld 26, 536(1)                           # 8-byte Folded Reload
	ld 18, 608(1)                           # 8-byte Folded Reload
	ld 22, 368(1)                           # 8-byte Folded Reload
	addi 4, 3, -1
	li 5, 0
	mr	11, 29
	.p2align	5
.Lp9_attention_BB4_220:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_192 Depth=2
                                        #       Parent Loop BB4_209 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	cmpld 7, 5, 4
	beq	7, .Lp9_attention_BB4_208
# %bb.221:                              #   in Loop: Header=BB4_220 Depth=4
	ld 29, 0(10)
	addi 10, 10, 8
	addi 5, 5, 1
	cmpldi 7, 29, 0
	bne	7, .Lp9_attention_BB4_220
	.p2align	4
.Lp9_attention_BB4_222:                              #   in Loop: Header=BB4_209 Depth=3
	lwz 5, 644(1)
	subfic 4, 8, 1
	li 4, 1
	subfe 4, 4, 4
	rotlwi	5, 5, 4
	mtocrf 1, 5
	bc 12, 30, .Lp9_attention_BB4_232
# %bb.223:                              #   in Loop: Header=BB4_209 Depth=3
	neg 4, 4
	mulld 5, 28, 15
	ld 29, 440(1)                           # 8-byte Folded Reload
	cmpldi 7, 3, 3
	or 4, 28, 4
	mulld 4, 4, 15
	sldi 5, 5, 2
	add 25, 29, 5
	li 5, 0
	sldi 4, 4, 2
	add 8, 29, 4
	li 4, 2
	isel 4, 4, 5, 25
	cmpdi 6, 3, 2
	or 4, 4, 28
	mulld 4, 4, 15
	sldi 4, 4, 2
	add 10, 29, 4
	li 4, 3
	isel 4, 4, 5, 21
	cmpldi 5, 3, 1
	or 4, 4, 28
	mulld 4, 4, 15
	sldi 4, 4, 2
	add 26, 29, 4
	li 29, 0
	b .Lp9_attention_BB4_225
	.p2align	4
.Lp9_attention_BB4_224:                              #   in Loop: Header=BB4_225 Depth=4
	addi 29, 29, 32
	cmpld 2, 29, 30
	bge 2, .Lp9_attention_BB4_207
.Lp9_attention_BB4_225:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_192 Depth=2
                                        #       Parent Loop BB4_209 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB4_227 Depth 5
	sldi 23, 29, 2
	add 5, 19, 23
	add 22, 6, 23
	add 4, 9, 23
	add 3, 7, 23
	lxvx 42, 19, 23
	lxvx 41, 6, 23
	lxvx 35, 9, 23
	lxvx 7, 7, 23
	lxv 49, 16(5)
	lxv 48, 32(5)
	lxv 47, 48(5)
	lxv 46, 64(5)
	lxv 45, 80(5)
	lxv 44, 96(5)
	lxv 43, 112(5)
	lxv 40, 16(22)
	lxv 39, 32(22)
	lxv 38, 48(22)
	lxv 33, 64(22)
	lxv 32, 80(22)
	lxv 37, 96(22)
	lxv 36, 112(22)
	lxv 34, 16(4)
	lxv 13, 32(4)
	lxv 12, 48(4)
	lxv 11, 64(4)
	lxv 10, 80(4)
	lxv 9, 96(4)
	lxv 8, 112(4)
	lxv 6, 16(3)
	lxv 5, 32(3)
	lxv 4, 48(3)
	lxv 3, 64(3)
	lxv 2, 80(3)
	lxv 1, 96(3)
	lxv 0, 112(3)
	bc 12, 2, .Lp9_attention_BB4_228
# %bb.226:                              #   in Loop: Header=BB4_225 Depth=4
	ld 18, 648(1)                           # 8-byte Folded Reload
	rldicl 23, 29, 59, 5
	maddld 23, 0, 23, 18
	li 18, 0
	.p2align	5
.Lp9_attention_BB4_227:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_192 Depth=2
                                        #       Parent Loop BB4_209 Depth=3
                                        #         Parent Loop BB4_225 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	sldi 16, 18, 7
	sldi 17, 18, 2
	addi 18, 18, 1
	lxvx 50, 23, 16
	add 16, 23, 16
	lxvwsx 51, 25, 17
	lxvwsx 24, 10, 17
	lxvwsx 23, 26, 17
	cmpld 2, 18, 12
	lxv 31, 16(16)
	lxv 30, 32(16)
	lxv 29, 48(16)
	lxv 28, 64(16)
	lxv 27, 80(16)
	lxv 26, 96(16)
	lxv 25, 112(16)
	xvmaddasp 42, 51, 50
	xvmaddasp 35, 24, 50
	xvmaddasp 7, 23, 50
	xvmaddasp 49, 51, 31
	xvmaddasp 48, 51, 30
	xvmaddasp 47, 51, 29
	xvmaddasp 46, 51, 28
	xvmaddasp 45, 51, 27
	xvmaddasp 44, 51, 26
	xvmaddasp 43, 51, 25
	lxvwsx 51, 8, 17
	xvmaddasp 34, 24, 31
	xvmaddasp 13, 24, 30
	xvmaddasp 12, 24, 29
	xvmaddasp 11, 24, 28
	xvmaddasp 10, 24, 27
	xvmaddasp 9, 24, 26
	xvmaddasp 8, 24, 25
	xvmaddasp 6, 23, 31
	xvmaddasp 5, 23, 30
	xvmaddasp 4, 23, 29
	xvmaddasp 3, 23, 28
	xvmaddasp 2, 23, 27
	xvmaddasp 1, 23, 26
	xvmaddasp 0, 23, 25
	xvmaddasp 41, 51, 50
	xvmaddasp 40, 51, 31
	xvmaddasp 39, 51, 30
	xvmaddasp 38, 51, 29
	xvmaddasp 33, 51, 28
	xvmaddasp 32, 51, 27
	xvmaddasp 37, 51, 26
	xvmaddasp 36, 51, 25
	blt	2, .Lp9_attention_BB4_227
.Lp9_attention_BB4_228:                              #   in Loop: Header=BB4_225 Depth=4
	ld 18, 608(1)                           # 8-byte Folded Reload
	stxv 42, 0(5)
	stxv 49, 16(5)
	stxv 48, 32(5)
	stxv 47, 48(5)
	stxv 46, 64(5)
	stxv 45, 80(5)
	stxv 44, 96(5)
	stxv 43, 112(5)
	bc 4, 21, .Lp9_attention_BB4_224
# %bb.229:                              #   in Loop: Header=BB4_225 Depth=4
	stxv 41, 0(22)
	stxv 40, 16(22)
	stxv 39, 32(22)
	stxv 38, 48(22)
	stxv 33, 64(22)
	stxv 32, 80(22)
	stxv 37, 96(22)
	stxv 36, 112(22)
	bc 12, 26, .Lp9_attention_BB4_224
# %bb.230:                              #   in Loop: Header=BB4_225 Depth=4
	stxv 35, 0(4)
	stxv 34, 16(4)
	stxv 13, 32(4)
	stxv 12, 48(4)
	stxv 11, 64(4)
	stxv 10, 80(4)
	stxv 9, 96(4)
	stxv 8, 112(4)
	bc 4, 29, .Lp9_attention_BB4_224
# %bb.231:                              #   in Loop: Header=BB4_225 Depth=4
	stxv 7, 0(3)
	stxv 6, 16(3)
	stxv 5, 32(3)
	stxv 4, 48(3)
	stxv 3, 64(3)
	stxv 2, 80(3)
	stxv 1, 96(3)
	stxv 0, 112(3)
	b .Lp9_attention_BB4_224
	.p2align	4
.Lp9_attention_BB4_232:                              #   in Loop: Header=BB4_209 Depth=3
	ld 26, 536(1)                           # 8-byte Folded Reload
	b .Lp9_attention_BB4_208
.Lp9_attention_BB4_233:                              #   in Loop: Header=BB4_23 Depth=1
	ld 22, 368(1)                           # 8-byte Folded Reload
	std 3, 976(1)
	lwz 3, 324(1)
	mtocrf 128, 3
	bc 4, 2, .Lp9_attention_BB4_277
	b .Lp9_attention_BB4_295
.Lp9_attention_BB4_234:                              #   in Loop: Header=BB4_23 Depth=1
	lwz 4, 324(1)
	mtocrf 128, 4
	bc 12, 2, .Lp9_attention_BB4_276
# %bb.235:                              #   in Loop: Header=BB4_23 Depth=1
	ld 9, 200(1)                            # 8-byte Folded Reload
	b .Lp9_attention_BB4_237
	.p2align	4
.Lp9_attention_BB4_236:                              #   in Loop: Header=BB4_237 Depth=2
	ld 3, 328(1)                            # 8-byte Folded Reload
	ld 9, 456(1)                            # 8-byte Folded Reload
	cmpld	9, 3
	bge 0, .Lp9_attention_BB4_275
.Lp9_attention_BB4_237:                              #   Parent Loop BB4_23 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB4_239 Depth 3
                                        #         Child Loop BB4_250 Depth 4
                                        #         Child Loop BB4_244 Depth 4
                                        #           Child Loop BB4_246 Depth 5
                                        #         Child Loop BB4_255 Depth 4
                                        #           Child Loop BB4_266 Depth 5
	ld 4, 336(1)                            # 8-byte Folded Reload
	sub	3, 15, 9
	li 29, 0
	add 5, 9, 4
	cmpld	3, 4
	isellt	0, 15, 5
	std 5, 456(1)                           # 8-byte Folded Spill
	cmpld	9, 0
	mfocrf 3, 128
	stw 3, 584(1)
	ld 3, 528(1)                            # 8-byte Folded Reload
	b .Lp9_attention_BB4_239
	.p2align	4
.Lp9_attention_BB4_238:                              #   in Loop: Header=BB4_239 Depth=3
	ld 3, 528(1)                            # 8-byte Folded Reload
	addi 29, 29, 4
	ld 22, 368(1)                           # 8-byte Folded Reload
	cmpld	29, 3
	bge 0, .Lp9_attention_BB4_236
.Lp9_attention_BB4_239:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_237 Depth=2
                                        # =>    This Loop Header: Depth=3
                                        #         Child Loop BB4_250 Depth 4
                                        #         Child Loop BB4_244 Depth 4
                                        #           Child Loop BB4_246 Depth 5
                                        #         Child Loop BB4_255 Depth 4
                                        #           Child Loop BB4_266 Depth 5
	ld 26, 536(1)                           # 8-byte Folded Reload
	ld 10, 512(1)                           # 8-byte Folded Reload
	ld 14, 504(1)                           # 8-byte Folded Reload
	sub	3, 3, 29
	li 6, 0
	li 7, 3
	cmpldi 5, 3, 2
	cmpldi 6, 3, 3
	cmpldi 1, 3, 4
	divdu 8, 29, 26
	mulld 4, 8, 26
	sub	5, 29, 4
	mulld 4, 8, 10
	std 5, 552(1)                           # 8-byte Folded Spill
	add 5, 5, 19
	mulld 5, 5, 30
	sldi 4, 4, 2
	add 4, 14, 4
	sldi 5, 5, 2
	add 28, 4, 5
	lwz 4, 484(1)
	li 5, 2
	std 8, 464(1)                           # 8-byte Folded Spill
	isel 5, 5, 6, 21
	isel 6, 7, 6, 25
	mtocrf 128, 4
	subfic 4, 3, 1
	li 4, 1
	or 5, 5, 29
	or 6, 6, 29
	subfe 4, 4, 4
	neg 4, 4
	or 4, 29, 4
	bc 12, 2, .Lp9_attention_BB4_242
# %bb.240:                              #   in Loop: Header=BB4_239 Depth=3
	ld 17, 400(1)                           # 8-byte Folded Reload
	ld 16, 384(1)                           # 8-byte Folded Reload
	mr	18, 26
	divdu 25, 6, 26
	ld 11, 496(1)                           # 8-byte Folded Reload
	ld 19, 568(1)                           # 8-byte Folded Reload
	maddld 7, 8, 17, 16
	divdu 8, 5, 26
	std 7, 912(1)
	divdu 22, 4, 18
	mulld 23, 25, 18
	mulld 7, 8, 26
	maddld 26, 8, 17, 16
	sub	23, 6, 23
	mulld 6, 8, 11
	mulld 8, 8, 10
	sub	5, 5, 7
	maddld 7, 22, 17, 16
	std 26, 928(1)
	sldi 6, 6, 2
	std 7, 920(1)
	mulld 7, 22, 18
	ld 18, 488(1)                           # 8-byte Folded Reload
	add 6, 18, 6
	sub	4, 4, 7
	add 7, 5, 19
	mulld 7, 7, 30
	sldi 7, 7, 2
	add 6, 6, 7
	sldi 7, 8, 2
	ld 8, 576(1)                            # 8-byte Folded Reload
	add 7, 14, 7
	std 6, 560(1)                           # 8-byte Folded Spill
	mulld 6, 22, 11
	add 5, 5, 8
	mulld 5, 5, 30
	sldi 6, 6, 2
	add 6, 18, 6
	sldi 5, 5, 2
	add 26, 7, 5
	add 5, 4, 19
	ld 7, 576(1)                            # 8-byte Folded Reload
	mulld 5, 5, 30
	add 4, 4, 7
	mulld 4, 4, 30
	sldi 5, 5, 2
	add 12, 6, 5
	mulld 6, 22, 10
	mulld 5, 25, 11
	sldi 4, 4, 2
	sldi 6, 6, 2
	add 6, 14, 6
	add 6, 6, 4
	sldi 4, 5, 2
	add 5, 23, 19
	ld 19, 576(1)                           # 8-byte Folded Reload
	mulld 5, 5, 30
	add 4, 18, 4
	sldi 5, 5, 2
	add 11, 4, 5
	mulld 4, 25, 10
	add 5, 23, 19
	mulld 5, 5, 30
	sldi 4, 4, 2
	add 4, 14, 4
	sldi 5, 5, 2
	add 7, 4, 5
	maddld 4, 25, 17, 16
	std 4, 936(1)
	li 4, 4
	lwz 5, 584(1)
	isel 4, 3, 4, 4
	mtocrf 128, 5
	bc 4, 0, .Lp9_attention_BB4_249
# %bb.241:                              #   in Loop: Header=BB4_239 Depth=3
	ld 22, 544(1)                           # 8-byte Folded Reload
	ld 18, 608(1)                           # 8-byte Folded Reload
	ld 16, 448(1)                           # 8-byte Folded Reload
	mr	14, 12
	b .Lp9_attention_BB4_243
	.p2align	4
.Lp9_attention_BB4_242:                              #   in Loop: Header=BB4_239 Depth=3
	divdu 7, 5, 26
	mr	22, 26
	ld 11, 496(1)                           # 8-byte Folded Reload
	ld 17, 568(1)                           # 8-byte Folded Reload
	divdu 8, 6, 26
	mulld 23, 8, 22
	divdu 25, 4, 26
	mulld 26, 7, 26
	sub	5, 5, 26
	mulld 26, 25, 22
	sub	23, 6, 23
	mulld 6, 7, 11
	mulld 7, 7, 10
	ld 22, 488(1)                           # 8-byte Folded Reload
	sub	4, 4, 26
	add 26, 5, 17
	add 5, 5, 19
	sldi 6, 6, 2
	sldi 7, 7, 2
	mulld 26, 26, 30
	mulld 5, 5, 30
	add 6, 22, 6
	add 7, 14, 7
	sldi 26, 26, 2
	sldi 5, 5, 2
	add 6, 6, 26
	add 26, 7, 5
	add 5, 4, 17
	add 4, 4, 19
	std 6, 560(1)                           # 8-byte Folded Spill
	mulld 6, 25, 11
	mulld 5, 5, 30
	mulld 4, 4, 30
	sldi 6, 6, 2
	sldi 5, 5, 2
	sldi 4, 4, 2
	add 6, 22, 6
	add 7, 6, 5
	mulld 6, 25, 10
	mulld 5, 8, 11
	sldi 6, 6, 2
	add 6, 14, 6
	add 6, 6, 4
	sldi 4, 5, 2
	add 5, 23, 17
	mulld 5, 5, 30
	add 4, 22, 4
	ld 22, 544(1)                           # 8-byte Folded Reload
	sldi 5, 5, 2
	add 11, 4, 5
	mulld 4, 8, 10
	add 5, 23, 19
	mulld 5, 5, 30
	sldi 4, 4, 2
	add 4, 14, 4
	sldi 5, 5, 2
	mr	14, 7
	add 7, 4, 5
	ld 4, 520(1)                            # 8-byte Folded Reload
	stxv 62, 16(4)
	stxvx 62, 0, 4
	li 4, 4
	lwz 5, 584(1)
	isel 4, 3, 4, 4
	mtocrf 128, 5
	bc 4, 0, .Lp9_attention_BB4_252
.Lp9_attention_BB4_243:                              #   in Loop: Header=BB4_239 Depth=3
	li 5, 0
.Lp9_attention_BB4_244:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_237 Depth=2
                                        #       Parent Loop BB4_239 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB4_246 Depth 5
	ld 10, 520(1)                           # 8-byte Folded Reload
	sldi 8, 5, 3
	ldx 8, 10, 8
	cmpldi	8, 0
	beq	0, .Lp9_attention_BB4_252
# %bb.245:                              #   in Loop: Header=BB4_244 Depth=4
	mr	25, 9
	.p2align	5
.Lp9_attention_BB4_246:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_237 Depth=2
                                        #       Parent Loop BB4_239 Depth=3
                                        #         Parent Loop BB4_244 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	lbzx 23, 8, 25
	cmplwi	23, 0
	bne	0, .Lp9_attention_BB4_252
# %bb.247:                              #   in Loop: Header=BB4_246 Depth=5
	addi 25, 25, 1
	cmpld	25, 0
	bne	0, .Lp9_attention_BB4_246
# %bb.248:                              #   in Loop: Header=BB4_244 Depth=4
	addi 5, 5, 1
	cmpld	5, 4
	bne	0, .Lp9_attention_BB4_244
	b .Lp9_attention_BB4_238
	.p2align	4
.Lp9_attention_BB4_249:                              #   in Loop: Header=BB4_239 Depth=3
	ld 25, 352(1)                           # 8-byte Folded Reload
	ld 22, 544(1)                           # 8-byte Folded Reload
	ld 18, 608(1)                           # 8-byte Folded Reload
	ld 16, 448(1)                           # 8-byte Folded Reload
	addi 5, 4, -1
	li 8, 0
	mr	14, 12
	.p2align	5
.Lp9_attention_BB4_250:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_237 Depth=2
                                        #       Parent Loop BB4_239 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	cmpld	8, 5
	beq	0, .Lp9_attention_BB4_238
# %bb.251:                              #   in Loop: Header=BB4_250 Depth=4
	ld 23, 0(25)
	addi 25, 25, 8
	addi 8, 8, 1
	cmpldi	23, 0
	bne	0, .Lp9_attention_BB4_250
	.p2align	4
.Lp9_attention_BB4_252:                              #   in Loop: Header=BB4_239 Depth=3
	lwz 5, 644(1)
	mtocrf 128, 5
	bc 12, 2, .Lp9_attention_BB4_238
# %bb.253:                              #   in Loop: Header=BB4_239 Depth=3
	mulld 5, 29, 15
	ld 8, 392(1)                            # 8-byte Folded Reload
	crmove	17, 25
	cmpldi 7, 3, 1
	cmpdi 1, 4, 2
	cmpldi 2, 4, 3
	li 3, 0
	crmove	13, 21
	cmpldi 5, 4, 1
	sldi 5, 5, 2
	add 8, 8, 5
	ld 5, 528(1)                            # 8-byte Folded Reload
	cmpd 6, 5, 29
	b .Lp9_attention_BB4_255
	.p2align	4
.Lp9_attention_BB4_254:                              #   in Loop: Header=BB4_255 Depth=4
	addi 3, 3, 16
	cmpld	3, 30
	bge 0, .Lp9_attention_BB4_238
.Lp9_attention_BB4_255:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_237 Depth=2
                                        #       Parent Loop BB4_239 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB4_266 Depth 5
	sldi 4, 3, 2
	bc 4, 26, .Lp9_attention_BB4_257
# %bb.256:                              #   in Loop: Header=BB4_255 Depth=4
	stxv 62, 704(1)
	stxv 62, 688(1)
	stxv 62, 672(1)
	stxv 62, 656(1)
	b .Lp9_attention_BB4_261
	.p2align	4
.Lp9_attention_BB4_257:                              #   in Loop: Header=BB4_255 Depth=4
	lxvx 0, 28, 4
	add 5, 28, 4
	stxv 0, 656(1)
	lxv 0, 48(5)
	stxv 0, 704(1)
	lxv 0, 32(5)
	stxv 0, 688(1)
	lxv 0, 16(5)
	stxv 0, 672(1)
	bc 4, 29, .Lp9_attention_BB4_261
# %bb.258:                              #   in Loop: Header=BB4_255 Depth=4
	lxvx 0, 6, 4
	add 5, 6, 4
	stxv 0, 0(22)
	lxv 0, 48(5)
	stxv 0, 48(22)
	lxv 0, 32(5)
	stxv 0, 32(22)
	lxv 0, 16(5)
	stxv 0, 16(22)
	bc 4, 13, .Lp9_attention_BB4_262
# %bb.259:                              #   in Loop: Header=BB4_255 Depth=4
	lxvx 0, 26, 4
	ld 25, 600(1)                           # 8-byte Folded Reload
	add 5, 26, 4
	stxv 0, 0(25)
	lxv 0, 48(5)
	stxv 0, 48(25)
	lxv 0, 32(5)
	stxv 0, 32(25)
	lxv 0, 16(5)
	stxv 0, 16(25)
	bc 4, 17, .Lp9_attention_BB4_263
# %bb.260:                              #   in Loop: Header=BB4_255 Depth=4
	lxvx 0, 7, 4
	ld 25, 592(1)                           # 8-byte Folded Reload
	add 5, 7, 4
	stxv 0, 0(25)
	lxv 0, 48(5)
	stxv 0, 48(25)
	lxv 0, 32(5)
	stxv 0, 32(25)
	lxv 0, 16(5)
	stxv 0, 16(25)
	lwz 5, 584(1)
	mtocrf 128, 5
	bc 4, 0, .Lp9_attention_BB4_271
	b .Lp9_attention_BB4_264
	.p2align	4
.Lp9_attention_BB4_261:                              #   in Loop: Header=BB4_255 Depth=4
	stxv 62, 48(22)
	stxv 62, 32(22)
	stxv 62, 16(22)
	stxv 62, 0(22)
.Lp9_attention_BB4_262:                              #   in Loop: Header=BB4_255 Depth=4
	ld 5, 600(1)                            # 8-byte Folded Reload
	stxv 62, 48(5)
	stxv 62, 32(5)
	stxv 62, 16(5)
	stxv 62, 0(5)
.Lp9_attention_BB4_263:                              #   in Loop: Header=BB4_255 Depth=4
	ld 5, 592(1)                            # 8-byte Folded Reload
	stxv 62, 48(5)
	stxv 62, 32(5)
	stxv 62, 16(5)
	stxv 62, 0(5)
	lwz 5, 584(1)
	mtocrf 128, 5
	bc 4, 0, .Lp9_attention_BB4_271
.Lp9_attention_BB4_264:                              #   in Loop: Header=BB4_255 Depth=4
	lxv 3, 656(1)
	lxv 2, 672(1)
	lxv 1, 688(1)
	lxv 0, 704(1)
	add 5, 16, 4
	mr	25, 9
	lxv 34, 720(1)
	lxv 13, 736(1)
	lxv 12, 752(1)
	lxv 11, 768(1)
	lxv 10, 784(1)
	lxv 9, 800(1)
	lxv 8, 816(1)
	lxv 7, 832(1)
	lxv 6, 848(1)
	lxv 5, 864(1)
	lxv 4, 880(1)
	lxv 35, 896(1)
	b .Lp9_attention_BB4_266
	.p2align	5
.Lp9_attention_BB4_265:                              #   in Loop: Header=BB4_266 Depth=5
	lxvwsx 38, 8, 23
	addi 25, 25, 1
	cmpld	25, 0
	xvmaddasp 3, 38, 33
	xvmaddasp 2, 38, 32
	xvmaddasp 1, 38, 37
	xvmaddasp 0, 38, 36
	bge 0, .Lp9_attention_BB4_270
.Lp9_attention_BB4_266:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_237 Depth=2
                                        #       Parent Loop BB4_239 Depth=3
                                        #         Parent Loop BB4_255 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	mulld 23, 25, 31
	sldi 23, 23, 2
	add 22, 5, 23
	lxvx 33, 5, 23
	sldi 23, 25, 2
	lxv 32, 16(22)
	lxv 37, 32(22)
	lxv 36, 48(22)
	bc 4, 21, .Lp9_attention_BB4_265
# %bb.267:                              #   in Loop: Header=BB4_266 Depth=5
	add 22, 8, 23
	lxvwsx 38, 22, 24
	xvmaddasp 34, 38, 33
	xvmaddasp 13, 38, 32
	xvmaddasp 12, 38, 37
	xvmaddasp 11, 38, 36
	bc 12, 6, .Lp9_attention_BB4_265
# %bb.268:                              #   in Loop: Header=BB4_266 Depth=5
	lxvwsx 38, 22, 18
	xvmaddasp 10, 38, 33
	xvmaddasp 9, 38, 32
	xvmaddasp 8, 38, 37
	xvmaddasp 7, 38, 36
	bc 4, 9, .Lp9_attention_BB4_265
# %bb.269:                              #   in Loop: Header=BB4_266 Depth=5
	ld 17, 632(1)                           # 8-byte Folded Reload
	lxvwsx 38, 22, 17
	xvmaddasp 6, 38, 33
	xvmaddasp 5, 38, 32
	xvmaddasp 4, 38, 37
	xvmaddasp 35, 38, 36
	b .Lp9_attention_BB4_265
	.p2align	4
.Lp9_attention_BB4_270:                              #   in Loop: Header=BB4_255 Depth=4
	ld 22, 544(1)                           # 8-byte Folded Reload
	stxv 3, 656(1)
	stxv 2, 672(1)
	stxv 1, 688(1)
	stxv 0, 704(1)
	stxv 34, 720(1)
	stxv 13, 736(1)
	stxv 12, 752(1)
	stxv 11, 768(1)
	stxv 10, 784(1)
	stxv 9, 800(1)
	stxv 8, 816(1)
	stxv 7, 832(1)
	stxv 6, 848(1)
	stxv 5, 864(1)
	stxv 4, 880(1)
	stxv 35, 896(1)
.Lp9_attention_BB4_271:                              #   in Loop: Header=BB4_255 Depth=4
	lxv 0, 656(1)
	add 5, 28, 4
	stxvx 0, 28, 4
	lxv 0, 704(1)
	stxv 0, 48(5)
	lxv 0, 688(1)
	stxv 0, 32(5)
	lxv 0, 672(1)
	stxv 0, 16(5)
	bc 4, 21, .Lp9_attention_BB4_254
# %bb.272:                              #   in Loop: Header=BB4_255 Depth=4
	lxv 0, 0(22)
	add 5, 6, 4
	stxvx 0, 6, 4
	lxv 0, 48(22)
	stxv 0, 48(5)
	lxv 0, 32(22)
	stxv 0, 32(5)
	lxv 0, 16(22)
	stxv 0, 16(5)
	bc 12, 6, .Lp9_attention_BB4_254
# %bb.273:                              #   in Loop: Header=BB4_255 Depth=4
	ld 25, 600(1)                           # 8-byte Folded Reload
	add 5, 26, 4
	lxv 0, 0(25)
	stxvx 0, 26, 4
	lxv 0, 48(25)
	stxv 0, 48(5)
	lxv 0, 32(25)
	stxv 0, 32(5)
	lxv 0, 16(25)
	stxv 0, 16(5)
	bc 4, 9, .Lp9_attention_BB4_254
# %bb.274:                              #   in Loop: Header=BB4_255 Depth=4
	ld 25, 592(1)                           # 8-byte Folded Reload
	add 5, 7, 4
	lxv 0, 0(25)
	stxvx 0, 7, 4
	lxv 0, 48(25)
	stxv 0, 48(5)
	lxv 0, 32(25)
	stxv 0, 32(5)
	lxv 0, 16(25)
	stxv 0, 16(5)
	b .Lp9_attention_BB4_254
.Lp9_attention_BB4_275:                              #   in Loop: Header=BB4_23 Depth=1
	ld 3, 496(1)                            # 8-byte Folded Reload
	ld 4, 464(1)                            # 8-byte Folded Reload
	ld 5, 552(1)                            # 8-byte Folded Reload
	std 6, 952(1)
	ld 6, 528(1)                            # 8-byte Folded Reload
	mulld 3, 4, 3
	ld 4, 488(1)                            # 8-byte Folded Reload
	std 26, 960(1)
	ld 26, 536(1)                           # 8-byte Folded Reload
	std 28, 944(1)
	std 7, 968(1)
	sldi 3, 3, 2
	add 3, 4, 3
	ld 4, 568(1)                            # 8-byte Folded Reload
	add 4, 5, 4
	mulld 4, 4, 30
	sldi 4, 4, 2
	add 3, 3, 4
.Lp9_attention_BB4_276:                              #   in Loop: Header=BB4_23 Depth=1
	ld 4, 560(1)                            # 8-byte Folded Reload
	ld 28, 488(1)                           # 8-byte Folded Reload
	ld 8, 512(1)                            # 8-byte Folded Reload
	ld 5, 504(1)                            # 8-byte Folded Reload
	std 14, 984(1)
	std 11, 1000(1)
	std 4, 992(1)
	ld 4, 520(1)                            # 8-byte Folded Reload
	std 3, 976(1)
	lwz 3, 324(1)
	mtocrf 128, 3
	bc 12, 2, .Lp9_attention_BB4_295
.Lp9_attention_BB4_277:                              #   in Loop: Header=BB4_23 Depth=1
	li 29, 0
	b .Lp9_attention_BB4_279
.Lp9_attention_BB4_278:                              #   in Loop: Header=BB4_279 Depth=2
	ld 6, 528(1)                            # 8-byte Folded Reload
	addi 29, 29, 4
	ld 8, 512(1)                            # 8-byte Folded Reload
	ld 5, 504(1)                            # 8-byte Folded Reload
	ld 4, 520(1)                            # 8-byte Folded Reload
	cmpld	29, 6
	bge 0, .Lp9_attention_BB4_295
.Lp9_attention_BB4_279:                              #   Parent Loop BB4_23 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB4_281 Depth 3
                                        #       Child Loop BB4_285 Depth 3
                                        #       Child Loop BB4_289 Depth 3
                                        #       Child Loop BB4_293 Depth 3
	ld 3, 424(1)                            # 8-byte Folded Reload
	std 4, 80(1)
	std 6, 48(1)
	ld 4, 496(1)                            # 8-byte Folded Reload
	mr	6, 8
	ld 7, 384(1)                            # 8-byte Folded Reload
	ld 8, 400(1)                            # 8-byte Folded Reload
	mr	9, 26
	mr	10, 30
	std 27, 64(1)
	std 29, 56(1)
	std 3, 72(1)
	ld 3, 232(1)                            # 8-byte Folded Reload
	std 3, 40(1)
	ld 3, 240(1)                            # 8-byte Folded Reload
	std 3, 32(1)
	mr	3, 28
	bl p9_attention_pafm_rows
	lwz 4, 644(1)
	cmpdi	3, 0
	rotlwi	4, 4, 28
	mtocrf 64, 4
	cror 20, 2, 6
	bc 12, 20, .Lp9_attention_BB4_278
# %bb.280:                              #   in Loop: Header=BB4_279 Depth=2
	ld 5, 944(1)
	mtctr 30
	mr	4, 3
	.p2align	5
.Lp9_attention_BB4_281:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_279 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lfs 0, 0(5)
	li 3, 0
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB4_1
# %bb.282:                              #   in Loop: Header=BB4_281 Depth=3
	addi 5, 5, 4
	bdnz .Lp9_attention_BB4_281
# %bb.283:                              #   in Loop: Header=BB4_279 Depth=2
	cmpldi	4, 1
	beq	0, .Lp9_attention_BB4_278
# %bb.284:                              #   in Loop: Header=BB4_279 Depth=2
	ld 5, 952(1)
	mtctr 30
	.p2align	5
.Lp9_attention_BB4_285:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_279 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lfs 0, 0(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB4_1
# %bb.286:                              #   in Loop: Header=BB4_285 Depth=3
	addi 5, 5, 4
	bdnz .Lp9_attention_BB4_285
# %bb.287:                              #   in Loop: Header=BB4_279 Depth=2
	cmpldi	4, 2
	beq	0, .Lp9_attention_BB4_278
# %bb.288:                              #   in Loop: Header=BB4_279 Depth=2
	ld 5, 960(1)
	mtctr 30
.Lp9_attention_BB4_289:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_279 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lfs 0, 0(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB4_1
# %bb.290:                              #   in Loop: Header=BB4_289 Depth=3
	addi 5, 5, 4
	bdnz .Lp9_attention_BB4_289
# %bb.291:                              #   in Loop: Header=BB4_279 Depth=2
	cmpldi	4, 3
	beq	0, .Lp9_attention_BB4_278
# %bb.292:                              #   in Loop: Header=BB4_279 Depth=2
	ld 4, 968(1)
	mtctr 30
.Lp9_attention_BB4_293:                              #   Parent Loop BB4_23 Depth=1
                                        #     Parent Loop BB4_279 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	lfs 0, 0(4)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB4_1
# %bb.294:                              #   in Loop: Header=BB4_293 Depth=3
	addi 4, 4, 4
	bdnz .Lp9_attention_BB4_293
	b .Lp9_attention_BB4_278
.Lp9_attention_BB4_295:                              #   in Loop: Header=BB4_23 Depth=1
	ld 4, 240(1)                            # 8-byte Folded Reload
	ld 27, 624(1)                           # 8-byte Folded Reload
	ld 3, 248(1)                            # 8-byte Folded Reload
	ld 0, 136(1)                            # 8-byte Folded Reload
	crmove	4, 12
	ld 6, 144(1)                            # 8-byte Folded Reload
	ld 19, 200(1)                           # 8-byte Folded Reload
	addi 4, 4, 1
	add 3, 3, 27
	add 22, 22, 27
	cmpld	4, 0
	std 3, 248(1)                           # 8-byte Folded Spill
	std 22, 368(1)                          # 8-byte Folded Spill
	std 4, 240(1)                           # 8-byte Folded Spill
	bne	0, .Lp9_attention_BB4_23
# %bb.296:
	li 3, 1
	b .Lp9_attention_BB4_1
.Lp9_attention_BB4_297:
	li 3, 0
	b .Lp9_attention_BB4_1
	.long	0
	.quad	0
.Lp9_attention_func_end4:
	.size	pa_attention_f32_power9_multi_kv, .Lp9_attention_func_end4-.Lp9_attention_func_begin4
	.cfi_endproc
                                        # -- End function
