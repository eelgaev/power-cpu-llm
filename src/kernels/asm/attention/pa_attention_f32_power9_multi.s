# attention/pa_attention_f32_power9_multi.s: kernel.
# Canonical POWER9 schedule, originally Clang 20 output of the former crates/power-kernels/native/attention_f32_power9.c
# clang -O3 -ffunction-sections -fdata-sections -fPIC -m64 -mcpu=power9 -std=c11 -ffp-contract=off.
# (instruction-identical to it). Included by src/kernels/*_asm.rs.
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function pa_attention_f32_power9_multi
.Lp9_attention_CPI3_0:
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
.Lp9_attention_CPI3_1:
	.long	0xff800000                      # float -Inf
	.section	.text.pa_attention_f32_power9_multi,"ax",@progbits
	.globl	pa_attention_f32_power9_multi
	.p2align 6
	.type	pa_attention_f32_power9_multi,@function
pa_attention_f32_power9_multi:        # @pa_attention_f32_power9_multi
.Lp9_attention_func_begin3:
	.cfi_startproc
.Lp9_attention_func_gep3:
	addis 2, 12, .TOC.-.Lp9_attention_func_gep3@ha
	addi 2, 2, .TOC.-.Lp9_attention_func_gep3@l
.Lp9_attention_func_lep3:
	.localentry	pa_attention_f32_power9_multi, .Lp9_attention_func_lep3-.Lp9_attention_func_gep3
# %bb.0:
	cmpdi 1, 9, 0
	mr	11, 3
	andi. 3, 10, 15
	li 3, 0
	crorc 20, 6, 2
	bclr 12, 20, 0
# %bb.1:
	mfcr 12
	mflr 0
	stw 12, 8(1)
	stdu 1, -784(1)
	std 0, 800(1)
	.cfi_def_cfa_offset 784
	.cfi_offset lr, 16
	.cfi_offset r14, -240
	.cfi_offset r15, -232
	.cfi_offset r16, -224
	.cfi_offset r17, -216
	.cfi_offset r18, -208
	.cfi_offset r19, -200
	.cfi_offset r20, -192
	.cfi_offset r21, -184
	.cfi_offset r22, -176
	.cfi_offset r23, -168
	.cfi_offset r24, -160
	.cfi_offset r25, -152
	.cfi_offset r26, -144
	.cfi_offset r27, -136
	.cfi_offset r28, -128
	.cfi_offset r29, -120
	.cfi_offset r30, -112
	.cfi_offset r31, -104
	.cfi_offset f20, -96
	.cfi_offset f21, -88
	.cfi_offset f22, -80
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
	.cfi_offset v31, -256
	divdu 12, 8, 9
	std 29, 664(1)                          # 8-byte Folded Spill
	std 31, 680(1)                          # 8-byte Folded Spill
	mr	29, 9
	ld 31, 880(1)
	std 14, 544(1)                          # 8-byte Folded Spill
	std 15, 552(1)                          # 8-byte Folded Spill
	std 16, 560(1)                          # 8-byte Folded Spill
	std 17, 568(1)                          # 8-byte Folded Spill
	std 18, 576(1)                          # 8-byte Folded Spill
	stxv 63, 528(1)                         # 16-byte Folded Spill
	cmpdi 1, 31, 0
	std 19, 584(1)                          # 8-byte Folded Spill
	std 20, 592(1)                          # 8-byte Folded Spill
	std 21, 600(1)                          # 8-byte Folded Spill
	std 22, 608(1)                          # 8-byte Folded Spill
	std 23, 616(1)                          # 8-byte Folded Spill
	std 24, 624(1)                          # 8-byte Folded Spill
	std 25, 632(1)                          # 8-byte Folded Spill
	std 26, 640(1)                          # 8-byte Folded Spill
	std 27, 648(1)                          # 8-byte Folded Spill
	std 28, 656(1)                          # 8-byte Folded Spill
	std 30, 672(1)                          # 8-byte Folded Spill
	stfd 20, 688(1)                         # 8-byte Folded Spill
	stfd 21, 696(1)                         # 8-byte Folded Spill
	stfd 22, 704(1)                         # 8-byte Folded Spill
	stfd 23, 712(1)                         # 8-byte Folded Spill
	stfd 24, 720(1)                         # 8-byte Folded Spill
	stfd 25, 728(1)                         # 8-byte Folded Spill
	stfd 26, 736(1)                         # 8-byte Folded Spill
	stfd 27, 744(1)                         # 8-byte Folded Spill
	stfd 28, 752(1)                         # 8-byte Folded Spill
	stfd 29, 760(1)                         # 8-byte Folded Spill
	stfd 30, 768(1)                         # 8-byte Folded Spill
	stfd 31, 776(1)                         # 8-byte Folded Spill
	mulld 9, 12, 9
	sub.	8, 8, 9
	crorc 20, 6, 2
	bc 12, 20, .Lp9_attention_BB3_138
# %bb.2:
	mulld. 0, 12, 4
	beq	0, .Lp9_attention_BB3_137
# %bb.3:
	lis 3, 21845
	mr	30, 10
	cmpldi	31, 2
	rldicr 27, 31, 0, 61
	ld 19, 888(1)
	ld 22, 904(1)
	addi 26, 6, 32
	ld 23, 920(1)
	sldi 20, 31, 3
	ld 21, 912(1)
	ori 3, 3, 21845
	sldi 17, 31, 2
	cmpldi 4, 30, 17
	rldic 3, 3, 33, 0
	addi 16, 22, -4
	mulld 9, 30, 29
	sldi 25, 30, 2
	oris 3, 3, 43690
	sldi 15, 9, 3
	ori 3, 3, 43691
	mulhdu 4, 31, 3
	rldicl 8, 4, 59, 5
	addi 4, 10, -1
	mulli 28, 8, 48
	rldicl 8, 4, 60, 4
	addi 24, 8, 1
	mfocrf 8, 128
	cmpdi	30, 0
	stw 8, 332(1)
	mfocrf 8, 128
	andi. 4, 4, 16
	addi 4, 27, -4
	mcrf 2, 0
	addi 10, 28, -48
	stw 8, 460(1)
	addi 8, 6, 64
	std 4, 256(1)                           # 8-byte Folded Spill
	std 19, 312(1)                          # 8-byte Folded Spill
	mulhdu 3, 10, 3
	li 10, 3
	std 8, 232(1)                           # 8-byte Folded Spill
	addi 8, 7, 32
	std 8, 168(1)                           # 8-byte Folded Spill
	addi 8, 31, -2
	std 8, 208(1)                           # 8-byte Folded Spill
	ld 8, 896(1)
	rldicl 3, 3, 59, 5
	addi 3, 3, 1
	std 8, 320(1)                           # 8-byte Folded Spill
	addis 8, 2, .Lp9_attention_CPI3_0@toc@ha
	addi 4, 8, .Lp9_attention_CPI3_0@toc@l
	andi. 8, 31, 1
	crmove	8, 2
	cmpdi	19, 0
	sldi 19, 9, 2
	lxv 63, 0(4)
	li 4, 1
	mfocrf 8, 128
	stw 8, 308(1)
	addi 8, 22, 96
	std 28, 224(1)                          # 8-byte Folded Spill
	std 9, 336(1)                           # 8-byte Folded Spill
	rldicl 9, 24, 63, 1
	std 8, 296(1)                           # 8-byte Folded Spill
	addi 8, 22, -8
	add 18, 8, 17
	std 8, 200(1)                           # 8-byte Folded Spill
	add 8, 8, 20
	std 8, 184(1)                           # 8-byte Folded Spill
	add 8, 16, 17
	std 18, 192(1)                          # 8-byte Folded Spill
	std 8, 136(1)                           # 8-byte Folded Spill
	sub.	8, 31, 28
	mfocrf 28, 128
	cmpldi	8, 4
	stw 28, 248(1)
	mfocrf 8, 128
	cmpldi	31, 4
	stw 8, 220(1)
	std 3, 240(1)                           # 8-byte Folded Spill
	addis 3, 2, .Lp9_attention_CPI3_1@toc@ha
	add 8, 26, 19
	std 26, 280(1)                          # 8-byte Folded Spill
	std 16, 144(1)                          # 8-byte Folded Spill
	lfs 31, .Lp9_attention_CPI3_1@toc@l(3)
	add 3, 16, 20
	std 8, 272(1)                           # 8-byte Folded Spill
	rldicl 8, 9, 1, 3
	li 16, 4
	std 3, 128(1)                           # 8-byte Folded Spill
	mulli 3, 31, 12
	addi 8, 8, -2
	rldicl 9, 8, 63, 1
	rldicr 8, 8, 4, 58
	addi 18, 8, 32
	addi 14, 9, 1
	li 9, 2
	li 8, 0
	std 3, 448(1)                           # 8-byte Folded Spill
	mfocrf 3, 128
	cmpldi	31, 48
	stw 3, 420(1)
	mfocrf 3, 128
	cmpd	31, 27
	stw 3, 292(1)
	mfocrf 3, 128
	std 27, 264(1)                          # 8-byte Folded Spill
	stw 3, 252(1)
	std 7, 152(1)                           # 8-byte Folded Spill
	std 6, 64(1)                            # 8-byte Folded Spill
	std 5, 112(1)                           # 8-byte Folded Spill
	std 11, 104(1)                          # 8-byte Folded Spill
	std 12, 96(1)                           # 8-byte Folded Spill
	std 0, 88(1)                            # 8-byte Folded Spill
	std 24, 80(1)                           # 8-byte Folded Spill
	std 23, 352(1)                          # 8-byte Folded Spill
	std 21, 344(1)                          # 8-byte Folded Spill
	std 20, 432(1)                          # 8-byte Folded Spill
	std 25, 56(1)                           # 8-byte Folded Spill
	std 17, 424(1)                          # 8-byte Folded Spill
.Lp9_attention_BB3_4:                                # =>This Loop Header: Depth=1
                                        #     Child Loop BB3_5 Depth 2
                                        #       Child Loop BB3_20 Depth 3
                                        #         Child Loop BB3_23 Depth 4
                                        #       Child Loop BB3_10 Depth 3
                                        #         Child Loop BB3_12 Depth 4
                                        #           Child Loop BB3_14 Depth 5
                                        #       Child Loop BB3_83 Depth 3
                                        #       Child Loop BB3_39 Depth 3
                                        #         Child Loop BB3_42 Depth 4
                                        #         Child Loop BB3_48 Depth 4
                                        #         Child Loop BB3_52 Depth 4
                                        #         Child Loop BB3_55 Depth 4
                                        #         Child Loop BB3_59 Depth 4
                                        #         Child Loop BB3_63 Depth 4
                                        #         Child Loop BB3_66 Depth 4
                                        #       Child Loop BB3_71 Depth 3
                                        #         Child Loop BB3_73 Depth 4
                                        #       Child Loop BB3_92 Depth 3
                                        #       Child Loop BB3_103 Depth 3
                                        #       Child Loop BB3_115 Depth 3
                                        #       Child Loop BB3_126 Depth 3
	mulld 3, 25, 8
	mulld 27, 8, 12
	li 26, 0
	std 8, 72(1)                            # 8-byte Folded Spill
	std 3, 160(1)                           # 8-byte Folded Spill
	mulld 3, 8, 30
	std 27, 120(1)                          # 8-byte Folded Spill
	sldi 3, 3, 2
	add 3, 6, 3
	std 3, 360(1)                           # 8-byte Folded Spill
.Lp9_attention_BB3_5:                                #   Parent Loop BB3_4 Depth=1
                                        # =>  This Loop Header: Depth=2
                                        #       Child Loop BB3_20 Depth 3
                                        #         Child Loop BB3_23 Depth 4
                                        #       Child Loop BB3_10 Depth 3
                                        #         Child Loop BB3_12 Depth 4
                                        #           Child Loop BB3_14 Depth 5
                                        #       Child Loop BB3_83 Depth 3
                                        #       Child Loop BB3_39 Depth 3
                                        #         Child Loop BB3_42 Depth 4
                                        #         Child Loop BB3_48 Depth 4
                                        #         Child Loop BB3_52 Depth 4
                                        #         Child Loop BB3_55 Depth 4
                                        #         Child Loop BB3_59 Depth 4
                                        #         Child Loop BB3_63 Depth 4
                                        #         Child Loop BB3_66 Depth 4
                                        #       Child Loop BB3_71 Depth 3
                                        #         Child Loop BB3_73 Depth 4
                                        #       Child Loop BB3_92 Depth 3
                                        #       Child Loop BB3_103 Depth 3
                                        #       Child Loop BB3_115 Depth 3
                                        #       Child Loop BB3_126 Depth 3
	sub	3, 0, 26
	divdu 21, 26, 12
	li 8, 0
	ld 0, 320(1)                            # 8-byte Folded Reload
	ld 28, 312(1)                           # 8-byte Folded Reload
	cmpldi 3, 3, 1
	cmpldi	3, 2
	std 3, 440(1)                           # 8-byte Folded Spill
	cmpldi 6, 3, 3
	isel 3, 4, 8, 13
	iselgt	4, 9, 8
	isel 6, 10, 8, 25
	or 3, 26, 3
	or 4, 4, 26
	or 10, 6, 26
	lwz 6, 308(1)
	std 26, 176(1)                          # 8-byte Folded Spill
	divdu 23, 3, 12
	divdu 25, 4, 12
	rotlwi	6, 6, 28
	mtocrf 64, 6
	divdu 6, 10, 12
	maddld 9, 21, 0, 28
	mulld 7, 21, 12
	std 21, 384(1)                          # 8-byte Folded Spill
	isel 9, 0, 9, 6
	sub	7, 26, 7
	std 9, 464(1)
	add 7, 7, 27
	mulld 9, 23, 12
	std 23, 392(1)                          # 8-byte Folded Spill
	std 25, 408(1)                          # 8-byte Folded Spill
	mulld 7, 7, 30
	std 6, 400(1)                           # 8-byte Folded Spill
	sub	3, 3, 9
	mulld 9, 25, 12
	add 3, 3, 27
	mulld 3, 3, 30
	sub	4, 4, 9
	maddld 9, 23, 0, 28
	add 4, 4, 27
	mulld 4, 4, 30
	isel 9, 0, 9, 6
	std 9, 472(1)
	mulld 9, 6, 12
	sldi 4, 4, 2
	std 4, 376(1)                           # 8-byte Folded Spill
	sub	10, 10, 9
	maddld 9, 25, 0, 28
	isel 9, 0, 9, 6
	std 9, 480(1)
	mulld 9, 21, 5
	sldi 21, 7, 2
	sldi 9, 9, 2
	add 9, 11, 9
	add 7, 9, 21
	std 7, 496(1)
	mulld 7, 23, 5
	sldi 23, 3, 2
	sldi 7, 7, 2
	add 7, 11, 7
	add 3, 7, 23
	std 3, 504(1)
	mulld 3, 25, 5
	sldi 3, 3, 2
	add 3, 11, 3
	add 3, 3, 4
	add 4, 10, 27
	std 3, 512(1)
	mulld 3, 6, 5
	mulld 4, 4, 30
	sldi 3, 3, 2
	sldi 4, 4, 2
	add 3, 11, 3
	std 4, 368(1)                           # 8-byte Folded Spill
	add 3, 3, 4
	std 3, 520(1)
	maddld 3, 6, 0, 28
	isel 3, 0, 3, 6
	std 3, 488(1)
	lwz 3, 332(1)
	rotlwi	3, 3, 28
	mtocrf 64, 3
	bc 4, 4, .Lp9_attention_BB3_18
.Lp9_attention_BB3_6:                                #   in Loop: Header=BB3_5 Depth=2
	ld 3, 440(1)                            # 8-byte Folded Reload
	ld 12, 336(1)                           # 8-byte Folded Reload
	ld 0, 360(1)                            # 8-byte Folded Reload
	ld 6, 352(1)                            # 8-byte Folded Reload
	li 11, 0
	cmpldi 1, 3, 4
	ld 28, 344(1)                           # 8-byte Folded Reload
	isel 27, 3, 16, 4
	cmpld 1, 8, 31
	bge 1, .Lp9_attention_BB3_38
# %bb.7:                                #   in Loop: Header=BB3_5 Depth=2
	lwz 3, 460(1)
	rotlwi	3, 3, 28
	mtocrf 64, 3
	bc 12, 6, .Lp9_attention_BB3_32
# %bb.8:                                #   in Loop: Header=BB3_5 Depth=2
	ld 3, 232(1)                            # 8-byte Folded Reload
	maddld 3, 19, 8, 3
	b .Lp9_attention_BB3_10
	.p2align	4
.Lp9_attention_BB3_9:                                #   in Loop: Header=BB3_10 Depth=3
	addi 8, 8, 1
	add 3, 3, 19
	cmpld	8, 31
	beq	0, .Lp9_attention_BB3_38
.Lp9_attention_BB3_10:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        # =>    This Loop Header: Depth=3
                                        #         Child Loop BB3_12 Depth 4
                                        #           Child Loop BB3_14 Depth 5
	mulld 4, 8, 12
	sldi 5, 8, 2
	li 7, 0
	add 5, 22, 5
	sldi 4, 4, 2
	add 4, 0, 4
	b .Lp9_attention_BB3_12
	.p2align	4
.Lp9_attention_BB3_11:                               #   in Loop: Header=BB3_12 Depth=4
	xvaddsp 2, 3, 2
	xvaddsp 0, 1, 0
	mulld 9, 7, 31
	addi 7, 7, 1
	cmpld	7, 27
	sldi 9, 9, 2
	xvaddsp 34, 2, 0
	vsldoi 3, 2, 2, 12
	vperm 2, 2, 2, 31
	xvaddsp 0, 35, 34
	xxspltw 1, 0, 2
	xvaddsp 0, 0, 1
	xxsldwi 0, 0, 0, 2
	stfiwx 0, 5, 9
	bge 0, .Lp9_attention_BB3_9
.Lp9_attention_BB3_12:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        #       Parent Loop BB3_10 Depth=3
                                        # =>      This Loop Header: Depth=4
                                        #           Child Loop BB3_14 Depth 5
	addi 10, 1, 496
	sldi 9, 7, 3
	xxlxor 0, 0, 0
	ldx 9, 10, 9
	li 10, 0
	xxlxor 1, 1, 1
	xxlxor 2, 2, 2
	xxlxor 3, 3, 3
	bc 12, 16, .Lp9_attention_BB3_16
# %bb.13:                               #   in Loop: Header=BB3_12 Depth=4
	mtctr 14
	addi 10, 9, 64
	mr	11, 3
	.p2align	5
.Lp9_attention_BB3_14:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        #       Parent Loop BB3_10 Depth=3
                                        #         Parent Loop BB3_12 Depth=4
                                        # =>        This Inner Loop Header: Depth=5
	lxv 4, -64(10)
	lxv 5, -64(11)
	xvmaddasp 3, 4, 5
	lxv 4, -48(10)
	lxv 5, -48(11)
	xvmaddasp 2, 4, 5
	lxv 4, -32(10)
	lxv 5, -32(11)
	xvmaddasp 1, 4, 5
	lxv 4, -16(10)
	lxv 5, -16(11)
	xvmaddasp 0, 4, 5
	lxv 4, 0(10)
	lxv 5, 0(11)
	xvmaddasp 3, 4, 5
	lxv 4, 16(10)
	lxv 5, 16(11)
	xvmaddasp 2, 4, 5
	lxv 4, 32(10)
	lxv 5, 32(11)
	xvmaddasp 1, 4, 5
	lxv 4, 48(10)
	lxv 5, 48(11)
	addi 10, 10, 128
	addi 11, 11, 128
	xvmaddasp 0, 4, 5
	bdnz .Lp9_attention_BB3_14
# %bb.15:                               #   in Loop: Header=BB3_12 Depth=4
	mr	10, 18
.Lp9_attention_BB3_16:                               #   in Loop: Header=BB3_12 Depth=4
	bc 4, 10, .Lp9_attention_BB3_11
# %bb.17:                               #   in Loop: Header=BB3_12 Depth=4
	sldi 10, 10, 2
	lxvx 4, 9, 10
	lxvx 5, 4, 10
	add 11, 9, 10
	add 9, 4, 10
	xvmaddasp 3, 4, 5
	lxv 4, 16(11)
	lxv 5, 16(9)
	xvmaddasp 2, 4, 5
	lxv 4, 32(11)
	lxv 5, 32(9)
	xvmaddasp 1, 4, 5
	lxv 4, 48(11)
	lxv 5, 48(9)
	xvmaddasp 0, 4, 5
	b .Lp9_attention_BB3_11
.Lp9_attention_BB3_18:                               #   in Loop: Header=BB3_5 Depth=2
	ld 4, 496(1)
	ld 5, 440(1)                            # 8-byte Folded Reload
	ld 3, 504(1)
	ld 26, 272(1)                           # 8-byte Folded Reload
	li 7, 2
	addi 0, 4, 32
	cmpdi 1, 5, 2
	ld 4, 512(1)
	ld 5, 520(1)
	ld 25, 280(1)                           # 8-byte Folded Reload
	b .Lp9_attention_BB3_20
	.p2align	4
.Lp9_attention_BB3_19:                               #   in Loop: Header=BB3_20 Depth=3
	addi 7, 8, 2
	add 25, 25, 15
	add 26, 26, 15
	cmpld 5, 7, 31
	bgt	5, .Lp9_attention_BB3_6
.Lp9_attention_BB3_20:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        # =>    This Loop Header: Depth=3
                                        #         Child Loop BB3_23 Depth 4
	lwz 6, 460(1)
	mr	27, 8
	mr	8, 7
	xxlxor 0, 0, 0
	rotlwi	6, 6, 12
	xxlxor 1, 1, 1
	xxlxor 2, 2, 2
	xxlxor 3, 3, 3
	xxlxor 4, 4, 4
	xxlxor 5, 5, 5
	xxlxor 6, 6, 6
	xxlxor 7, 7, 7
	xxlxor 8, 8, 8
	mtocrf 4, 6
	xxlxor 9, 9, 9
	xxlxor 10, 10, 10
	xxlxor 11, 11, 11
	xxlxor 12, 12, 12
	xxlxor 13, 13, 13
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
	xxlxor 36, 36, 36
	xxlxor 37, 37, 37
	xxlxor 32, 32, 32
	xxlxor 33, 33, 33
	xxlxor 38, 38, 38
	xxlxor 39, 39, 39
	xxlxor 40, 40, 40
	bc 12, 22, .Lp9_attention_BB3_27
# %bb.21:                               #   in Loop: Header=BB3_20 Depth=3
	mtctr 24
	mr	28, 0
	mr	10, 26
	mr	11, 25
	mr	12, 3
	mr	7, 4
	mr	9, 5
	xxlxor 41, 41, 41
	xxlxor 42, 42, 42
	xxlxor 43, 43, 43
	xxlxor 44, 44, 44
	xxlxor 45, 45, 45
	xxlxor 46, 46, 46
	xxlxor 47, 47, 47
	xxlxor 48, 48, 48
	xxlxor 49, 49, 49
	b .Lp9_attention_BB3_23
	.p2align	5
.Lp9_attention_BB3_22:                               #   in Loop: Header=BB3_23 Depth=4
	xvmaddasp 49, 21, 29
	xvmaddasp 45, 21, 25
	addi 11, 11, 64
	addi 10, 10, 64
	addi 28, 28, 64
	addi 12, 12, 64
	addi 7, 7, 64
	addi 9, 9, 64
	xvmaddasp 48, 22, 30
	xvmaddasp 44, 22, 26
	xvmaddasp 47, 23, 51
	xvmaddasp 43, 23, 27
	xvmaddasp 46, 24, 50
	xvmaddasp 42, 24, 28
	bdz .Lp9_attention_BB3_28
.Lp9_attention_BB3_23:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        #       Parent Loop BB3_20 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lxv 29, -32(11)
	lxv 30, -16(11)
	lxv 51, 0(11)
	lxv 50, 16(11)
	lxv 25, -32(10)
	lxv 26, -16(10)
	lxv 27, 0(10)
	lxv 28, 16(10)
	lxv 21, -32(28)
	lxv 22, -16(28)
	lxv 23, 0(28)
	lxv 24, 16(28)
	bc 4, 13, .Lp9_attention_BB3_22
# %bb.24:                               #   in Loop: Header=BB3_23 Depth=4
	lxv 20, 0(12)
	xvmaddasp 41, 20, 29
	xvmaddasp 33, 20, 25
	lxv 20, 16(12)
	xvmaddasp 40, 20, 30
	xvmaddasp 32, 20, 26
	lxv 20, 32(12)
	xvmaddasp 39, 20, 51
	xvmaddasp 37, 20, 27
	lxv 20, 48(12)
	xvmaddasp 38, 20, 50
	xvmaddasp 36, 20, 28
	bc 12, 6, .Lp9_attention_BB3_22
# %bb.25:                               #   in Loop: Header=BB3_23 Depth=4
	lxv 20, 0(7)
	xvmaddasp 35, 20, 29
	xvmaddasp 11, 20, 25
	lxv 20, 16(7)
	xvmaddasp 34, 20, 30
	xvmaddasp 10, 20, 26
	lxv 20, 32(7)
	xvmaddasp 13, 20, 51
	xvmaddasp 9, 20, 27
	lxv 20, 48(7)
	xvmaddasp 12, 20, 50
	xvmaddasp 8, 20, 28
	bc 4, 25, .Lp9_attention_BB3_22
# %bb.26:                               #   in Loop: Header=BB3_23 Depth=4
	lxv 20, 0(9)
	xvmaddasp 7, 20, 29
	xvmaddasp 3, 20, 25
	lxv 20, 16(9)
	xvmaddasp 6, 20, 30
	xvmaddasp 2, 20, 26
	lxv 20, 32(9)
	xvmaddasp 5, 20, 51
	xvmaddasp 1, 20, 27
	lxv 20, 48(9)
	xvmaddasp 4, 20, 50
	xvmaddasp 0, 20, 28
	b .Lp9_attention_BB3_22
	.p2align	4
.Lp9_attention_BB3_27:                               #   in Loop: Header=BB3_20 Depth=3
	xxlxor 41, 41, 41
	xxlxor 42, 42, 42
	xxlxor 43, 43, 43
	xxlxor 44, 44, 44
	xxlxor 45, 45, 45
	xxlxor 46, 46, 46
	xxlxor 47, 47, 47
	xxlxor 48, 48, 48
	xxlxor 49, 49, 49
.Lp9_attention_BB3_28:                               #   in Loop: Header=BB3_20 Depth=3
	xvaddsp 48, 49, 48
	xvaddsp 46, 47, 46
	sldi 9, 27, 2
	mr	7, 22
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
	xxsldwi 45, 45, 45, 3
	xvaddsp 42, 43, 42
	xscvspdpn 30, 45
	xxsldwi 42, 42, 42, 2
	stfsux 30, 7, 9
	stxsiwx 42, 7, 16
	bc 4, 13, .Lp9_attention_BB3_19
# %bb.29:                               #   in Loop: Header=BB3_20 Depth=3
	xvaddsp 40, 41, 40
	xvaddsp 38, 39, 38
	mr	9, 7
	xvaddsp 32, 33, 32
	xvaddsp 36, 37, 36
	xvaddsp 38, 40, 38
	xvaddsp 36, 32, 36
	xxspltw 39, 38, 2
	xxspltw 40, 38, 1
	xxspltw 37, 36, 2
	xxspltw 32, 36, 1
	xvaddsp 39, 38, 39
	xxspltw 38, 38, 0
	xvaddsp 37, 36, 37
	xxspltw 36, 36, 0
	xvaddsp 38, 40, 38
	xvaddsp 36, 32, 36
	xvaddsp 38, 39, 38
	xvaddsp 36, 37, 36
	xxsldwi 38, 38, 38, 3
	xxsldwi 36, 36, 36, 2
	xscvspdpn 30, 38
	stfsux 30, 9, 17
	addi 9, 9, 4
	stxsiwx 36, 0, 9
	bc 12, 6, .Lp9_attention_BB3_19
# %bb.30:                               #   in Loop: Header=BB3_20 Depth=3
	xvaddsp 34, 35, 34
	xvaddsp 12, 13, 12
	mr	9, 7
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
	stfsux 12, 9, 20
	addi 9, 9, 4
	stfiwx 8, 0, 9
	bc 4, 25, .Lp9_attention_BB3_19
# %bb.31:                               #   in Loop: Header=BB3_20 Depth=3
	xvaddsp 6, 7, 6
	xvaddsp 4, 5, 4
	ld 9, 448(1)                            # 8-byte Folded Reload
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
	stfsux 4, 7, 9
	addi 7, 7, 4
	stfiwx 0, 0, 7
	b .Lp9_attention_BB3_19
.Lp9_attention_BB3_32:                               #   in Loop: Header=BB3_5 Depth=2
	ori 3, 8, 1
	bc 12, 8, .Lp9_attention_BB3_37
# %bb.33:                               #   in Loop: Header=BB3_5 Depth=2
	sldi 5, 8, 2
	mr	4, 22
	mr	8, 3
	stwux 11, 4, 5
	bc 4, 13, .Lp9_attention_BB3_37
# %bb.34:                               #   in Loop: Header=BB3_5 Depth=2
	mr	8, 3
	stwx 11, 4, 17
	beq	0, .Lp9_attention_BB3_37
# %bb.35:                               #   in Loop: Header=BB3_5 Depth=2
	mr	8, 3
	stwx 11, 4, 20
	bc 4, 25, .Lp9_attention_BB3_37
# %bb.36:                               #   in Loop: Header=BB3_5 Depth=2
	ld 5, 448(1)                            # 8-byte Folded Reload
	mr	8, 3
	stwx 11, 4, 5
.Lp9_attention_BB3_37:                               #   in Loop: Header=BB3_5 Depth=2
	cmpld	31, 3
	bne	0, .Lp9_attention_BB3_81
	.p2align	4
.Lp9_attention_BB3_38:                               #   in Loop: Header=BB3_5 Depth=2
	ld 3, 384(1)                            # 8-byte Folded Reload
	ld 4, 376(1)                            # 8-byte Folded Reload
	ld 26, 296(1)                           # 8-byte Folded Reload
	mr	25, 22
	crmove	9, 25
	mulld 3, 3, 6
	sldi 3, 3, 2
	add 3, 28, 3
	add 3, 3, 21
	std 3, 384(1)                           # 8-byte Folded Spill
	ld 3, 392(1)                            # 8-byte Folded Reload
	mulld 3, 3, 6
	sldi 3, 3, 2
	add 3, 28, 3
	add 3, 3, 23
	std 3, 392(1)                           # 8-byte Folded Spill
	ld 3, 408(1)                            # 8-byte Folded Reload
	mulld 3, 3, 6
	sldi 3, 3, 2
	add 3, 28, 3
	add 3, 3, 4
	ld 4, 368(1)                            # 8-byte Folded Reload
	std 3, 408(1)                           # 8-byte Folded Spill
	ld 3, 400(1)                            # 8-byte Folded Reload
	mulld 3, 3, 6
	sldi 3, 3, 2
	add 3, 28, 3
	li 28, 0
	add 3, 3, 4
	std 3, 400(1)                           # 8-byte Folded Spill
.Lp9_attention_BB3_39:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        # =>    This Loop Header: Depth=3
                                        #         Child Loop BB3_42 Depth 4
                                        #         Child Loop BB3_48 Depth 4
                                        #         Child Loop BB3_52 Depth 4
                                        #         Child Loop BB3_55 Depth 4
                                        #         Child Loop BB3_59 Depth 4
                                        #         Child Loop BB3_63 Depth 4
                                        #         Child Loop BB3_66 Depth 4
	addi 5, 1, 464
	sldi 4, 28, 3
	maddld 3, 17, 28, 22
	ldx 4, 5, 4
	addi 21, 3, -4
	cmpldi	4, 0
	beq	0, .Lp9_attention_BB3_50
# %bb.40:                               #   in Loop: Header=BB3_39 Depth=3
	addi 20, 4, -1
	addis 4, 2, .Lp9_attention_CPI3_1@toc@ha
	mtctr 31
	addi 17, 3, -4
	addi 3, 3, -4
	lfs 29, .Lp9_attention_CPI3_1@toc@l(4)
	mr	4, 20
	b .Lp9_attention_BB3_42
	.p2align	5
.Lp9_attention_BB3_41:                               #   in Loop: Header=BB3_42 Depth=4
	bdz .Lp9_attention_BB3_46
.Lp9_attention_BB3_42:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        #       Parent Loop BB3_39 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lbzu 6, 1(4)
	mr	5, 3
	addi 3, 3, 4
	cmplwi	6, 0
	beq	0, .Lp9_attention_BB3_41
# %bb.43:                               #   in Loop: Header=BB3_42 Depth=4
	lfs 0, 4(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.44:                               #   in Loop: Header=BB3_42 Depth=4
	fcmpu 0, 0, 29
	ble 0, .Lp9_attention_BB3_41
# %bb.45:                               #   in Loop: Header=BB3_42 Depth=4
	fmr 29, 0
	b .Lp9_attention_BB3_41
	.p2align	4
.Lp9_attention_BB3_46:                               #   in Loop: Header=BB3_39 Depth=3
	xxlxor 30, 30, 30
	mr	24, 31
	b .Lp9_attention_BB3_48
	.p2align	5
.Lp9_attention_BB3_47:                               #   in Loop: Header=BB3_48 Depth=4
	xsaddsp 30, 30, 1
	addi 24, 24, -1
	stfs 1, 4(17)
	mr	17, 23
	cmpldi	24, 0
	bc 4, 1, .Lp9_attention_BB3_56
.Lp9_attention_BB3_48:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        #       Parent Loop BB3_39 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lbzu 3, 1(20)
	addi 23, 17, 4
	xxlxor 1, 1, 1
	cmplwi	3, 0
	beq	0, .Lp9_attention_BB3_47
# %bb.49:                               #   in Loop: Header=BB3_48 Depth=4
	lfs 0, 4(17)
	xssubsp 1, 0, 29
	bl power_expf
	nop
	crmove	25, 9
	b .Lp9_attention_BB3_47
	.p2align	4
.Lp9_attention_BB3_50:                               #   in Loop: Header=BB3_39 Depth=3
	mulld 3, 28, 31
	sldi 3, 3, 2
	add 3, 22, 3
	lfs 0, 0(3)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.51:                               #   in Loop: Header=BB3_39 Depth=3
	mtctr 31
	fmr 29, 31
	mr	3, 25
	.p2align	5
.Lp9_attention_BB3_52:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        #       Parent Loop BB3_39 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	xsmaxcdp 29, 0, 29
	bdz .Lp9_attention_BB3_54
# %bb.53:                               #   in Loop: Header=BB3_52 Depth=4
	lfs 0, 4(3)
	addi 3, 3, 4
	xststdcsp 0, 0, 112
	bc 4, 2, .Lp9_attention_BB3_52
	b .Lp9_attention_BB3_136
.Lp9_attention_BB3_54:                               #   in Loop: Header=BB3_39 Depth=3
	xxlxor 30, 30, 30
	mr	24, 21
	mr	23, 31
	.p2align	5
.Lp9_attention_BB3_55:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        #       Parent Loop BB3_39 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfsu 0, 4(24)
	xssubsp 1, 0, 29
	bl power_expf
	nop
	xsaddsp 30, 30, 1
	addi 23, 23, -1
	crmove	25, 9
	stfs 1, 0(24)
	cmpldi	23, 0
	bc 12, 1, .Lp9_attention_BB3_55
.Lp9_attention_BB3_56:                               #   in Loop: Header=BB3_39 Depth=3
	ld 20, 432(1)                           # 8-byte Folded Reload
	ld 17, 424(1)                           # 8-byte Folded Reload
	lwz 3, 420(1)
	li 5, 0
	mtocrf 128, 3
	bc 12, 0, .Lp9_attention_BB3_65
# %bb.57:                               #   in Loop: Header=BB3_39 Depth=3
	xscvdpspn 0, 30
	lwz 3, 292(1)
	li 4, 0
	mtocrf 128, 3
	bc 12, 0, .Lp9_attention_BB3_62
# %bb.58:                               #   in Loop: Header=BB3_39 Depth=3
	ld 4, 240(1)                            # 8-byte Folded Reload
	xxspltw 1, 0, 0
	mr	3, 26
	mtctr 4
	.p2align	5
.Lp9_attention_BB3_59:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        #       Parent Loop BB3_39 Depth=3
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
	bdnz .Lp9_attention_BB3_59
# %bb.60:                               #   in Loop: Header=BB3_39 Depth=3
	lwz 3, 248(1)
	mtocrf 128, 3
	bc 12, 2, .Lp9_attention_BB3_67
# %bb.61:                               #   in Loop: Header=BB3_39 Depth=3
	ld 5, 224(1)                            # 8-byte Folded Reload
	lwz 3, 220(1)
	mr	4, 5
	mtocrf 128, 3
	bc 12, 0, .Lp9_attention_BB3_65
.Lp9_attention_BB3_62:                               #   in Loop: Header=BB3_39 Depth=3
	ld 5, 256(1)                            # 8-byte Folded Reload
	sldi 3, 4, 2
	xxspltw 0, 0, 0
	add 3, 25, 3
	sub	4, 5, 4
	rldicl 4, 4, 62, 2
	addi 4, 4, 1
	mtctr 4
	.p2align	5
.Lp9_attention_BB3_63:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        #       Parent Loop BB3_39 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lxv 1, 0(3)
	xvdivsp 1, 1, 0
	stxv 1, 0(3)
	addi 3, 3, 16
	bdnz .Lp9_attention_BB3_63
# %bb.64:                               #   in Loop: Header=BB3_39 Depth=3
	ld 5, 264(1)                            # 8-byte Folded Reload
	lwz 3, 252(1)
	mtocrf 128, 3
	bc 12, 2, .Lp9_attention_BB3_67
.Lp9_attention_BB3_65:                               #   in Loop: Header=BB3_39 Depth=3
	sub	4, 31, 5
	sldi 3, 5, 2
	mtctr 4
	add 3, 21, 3
	.p2align	5
.Lp9_attention_BB3_66:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        #       Parent Loop BB3_39 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lfsu 0, 4(3)
	xsdivsp 0, 0, 30
	stfs 0, 0(3)
	bdnz .Lp9_attention_BB3_66
.Lp9_attention_BB3_67:                               #   in Loop: Header=BB3_39 Depth=3
	addi 28, 28, 1
	add 25, 25, 17
	add 26, 26, 17
	cmpld	28, 27
	blt	0, .Lp9_attention_BB3_39
# %bb.68:                               #   in Loop: Header=BB3_5 Depth=2
	lwz 3, 460(1)
	mtocrf 128, 3
	bc 12, 2, .Lp9_attention_BB3_113
# %bb.69:                               #   in Loop: Header=BB3_5 Depth=2
	ld 3, 440(1)                            # 8-byte Folded Reload
	ld 4, 168(1)                            # 8-byte Folded Reload
	ld 10, 152(1)                           # 8-byte Folded Reload
	ld 11, 144(1)                           # 8-byte Folded Reload
	li 5, 0
	cmpdi	3, 2
	ld 12, 136(1)                           # 8-byte Folded Reload
	ld 0, 128(1)                            # 8-byte Folded Reload
	ld 28, 448(1)                           # 8-byte Folded Reload
	ld 27, 160(1)                           # 8-byte Folded Reload
	ld 26, 400(1)                           # 8-byte Folded Reload
	ld 25, 408(1)                           # 8-byte Folded Reload
	ld 24, 392(1)                           # 8-byte Folded Reload
	ld 23, 384(1)                           # 8-byte Folded Reload
	li 3, 0
	b .Lp9_attention_BB3_71
	.p2align	4
.Lp9_attention_BB3_70:                               #   in Loop: Header=BB3_71 Depth=3
	addi 5, 5, 16
	addi 4, 4, 64
	addi 3, 3, 1
	cmpld 1, 5, 30
	bge 1, .Lp9_attention_BB3_91
.Lp9_attention_BB3_71:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        # =>    This Loop Header: Depth=3
                                        #         Child Loop BB3_73 Depth 4
	sldi 7, 3, 6
	mtctr 31
	xxlxor 0, 0, 0
	mr	9, 11
	mr	6, 12
	mr	8, 0
	add 7, 27, 7
	xxlxor 1, 1, 1
	xxlxor 2, 2, 2
	add 7, 10, 7
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
	b .Lp9_attention_BB3_73
	.p2align	5
.Lp9_attention_BB3_72:                               #   in Loop: Header=BB3_73 Depth=4
	xvmaddasp 35, 36, 38
	xvmaddasp 34, 36, 37
	add 7, 7, 19
	xvmaddasp 13, 36, 32
	xvmaddasp 12, 36, 33
	bdz .Lp9_attention_BB3_77
.Lp9_attention_BB3_73:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        #       Parent Loop BB3_71 Depth=3
                                        # =>      This Inner Loop Header: Depth=4
	lxvwsx 36, 9, 16
	lxv 38, 0(7)
	lxv 37, 16(7)
	lxv 32, 32(7)
	addi 8, 8, 4
	addi 6, 6, 4
	addi 9, 9, 4
	lxv 33, 48(7)
	bc 4, 13, .Lp9_attention_BB3_72
# %bb.74:                               #   in Loop: Header=BB3_73 Depth=4
	lxvwsx 39, 0, 6
	xvmaddasp 11, 39, 38
	xvmaddasp 10, 39, 37
	xvmaddasp 9, 39, 32
	xvmaddasp 8, 39, 33
	bc 12, 2, .Lp9_attention_BB3_72
# %bb.75:                               #   in Loop: Header=BB3_73 Depth=4
	lxvwsx 39, 0, 8
	xvmaddasp 7, 39, 38
	xvmaddasp 6, 39, 37
	xvmaddasp 5, 39, 32
	xvmaddasp 4, 39, 33
	bc 4, 25, .Lp9_attention_BB3_72
# %bb.76:                               #   in Loop: Header=BB3_73 Depth=4
	lxvwsx 39, 9, 28
	xvmaddasp 3, 39, 38
	xvmaddasp 2, 39, 37
	xvmaddasp 1, 39, 32
	xvmaddasp 0, 39, 33
	b .Lp9_attention_BB3_72
	.p2align	4
.Lp9_attention_BB3_77:                               #   in Loop: Header=BB3_71 Depth=3
	sldi 6, 5, 2
	add 7, 23, 6
	stxvx 35, 23, 6
	stxv 34, 16(7)
	stxv 13, 32(7)
	stxv 12, 48(7)
	bc 4, 13, .Lp9_attention_BB3_70
# %bb.78:                               #   in Loop: Header=BB3_71 Depth=3
	add 7, 24, 6
	stxvx 11, 24, 6
	stxv 10, 16(7)
	stxv 9, 32(7)
	stxv 8, 48(7)
	bc 12, 2, .Lp9_attention_BB3_70
# %bb.79:                               #   in Loop: Header=BB3_71 Depth=3
	add 7, 25, 6
	stxvx 7, 25, 6
	stxv 6, 16(7)
	stxv 5, 32(7)
	stxv 4, 48(7)
	bc 4, 25, .Lp9_attention_BB3_70
# %bb.80:                               #   in Loop: Header=BB3_71 Depth=3
	add 7, 26, 6
	stxvx 3, 26, 6
	stxv 2, 16(7)
	stxv 1, 32(7)
	stxv 0, 48(7)
	b .Lp9_attention_BB3_70
.Lp9_attention_BB3_81:                               #   in Loop: Header=BB3_5 Depth=2
	ld 3, 440(1)                            # 8-byte Folded Reload
	ld 4, 192(1)                            # 8-byte Folded Reload
	ld 7, 184(1)                            # 8-byte Folded Reload
	sldi 5, 8, 2
	cmpdi	3, 2
	ld 3, 200(1)                            # 8-byte Folded Reload
	add 4, 4, 5
	add 3, 3, 5
	add 5, 7, 5
	ld 7, 208(1)                            # 8-byte Folded Reload
	sub	7, 7, 8
	rldicl 7, 7, 63, 1
	addi 7, 7, 1
	mtctr 7
	b .Lp9_attention_BB3_83
	.p2align	5
.Lp9_attention_BB3_82:                               #   in Loop: Header=BB3_83 Depth=3
	bdz .Lp9_attention_BB3_38
.Lp9_attention_BB3_83:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	stwu 11, 8(3)
	mr	7, 5
	addi 5, 5, 8
	mr	9, 4
	addi 4, 4, 8
	addi 8, 3, 4
	bc 4, 13, .Lp9_attention_BB3_87
# %bb.84:                               #   in Loop: Header=BB3_83 Depth=3
	stw 11, 8(9)
	bc 12, 2, .Lp9_attention_BB3_87
# %bb.85:                               #   in Loop: Header=BB3_83 Depth=3
	stw 11, 8(7)
	bc 4, 25, .Lp9_attention_BB3_87
# %bb.86:                               #   in Loop: Header=BB3_83 Depth=3
	ld 10, 448(1)                           # 8-byte Folded Reload
	add 10, 8, 10
	stw 11, -4(10)
	.p2align	5
.Lp9_attention_BB3_87:                               #   in Loop: Header=BB3_83 Depth=3
	stw 11, 4(3)
	bc 4, 13, .Lp9_attention_BB3_82
# %bb.88:                               #   in Loop: Header=BB3_83 Depth=3
	stw 11, 12(9)
	bc 12, 2, .Lp9_attention_BB3_82
# %bb.89:                               #   in Loop: Header=BB3_83 Depth=3
	stw 11, 12(7)
	bc 4, 25, .Lp9_attention_BB3_82
# %bb.90:                               #   in Loop: Header=BB3_83 Depth=3
	ld 7, 448(1)                            # 8-byte Folded Reload
	stwx 11, 8, 7
	b .Lp9_attention_BB3_82
.Lp9_attention_BB3_91:                               #   in Loop: Header=BB3_5 Depth=2
	li 3, 0
	mr	4, 30
.Lp9_attention_BB3_92:                               #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	sldi 5, 3, 2
	lfsx 0, 23, 5
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.93:                               #   in Loop: Header=BB3_92 Depth=3
	ld 6, 384(1)                            # 8-byte Folded Reload
	add 5, 6, 5
	lfs 0, 4(5)
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.94:                               #   in Loop: Header=BB3_92 Depth=3
	lfs 0, 8(5)
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.95:                               #   in Loop: Header=BB3_92 Depth=3
	lfs 0, 12(5)
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.96:                               #   in Loop: Header=BB3_92 Depth=3
	lfs 0, 16(5)
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.97:                               #   in Loop: Header=BB3_92 Depth=3
	lfs 0, 20(5)
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.98:                               #   in Loop: Header=BB3_92 Depth=3
	lfs 0, 24(5)
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.99:                               #   in Loop: Header=BB3_92 Depth=3
	lfs 0, 28(5)
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.100:                              #   in Loop: Header=BB3_92 Depth=3
	addi 4, 4, -8
	ld 23, 384(1)                           # 8-byte Folded Reload
	addi 3, 3, 8
	cmpldi 1, 4, 0
	bne	1, .Lp9_attention_BB3_92
# %bb.101:                              #   in Loop: Header=BB3_5 Depth=2
	ld 20, 432(1)                           # 8-byte Folded Reload
	ld 17, 424(1)                           # 8-byte Folded Reload
	bc 4, 13, .Lp9_attention_BB3_113
# %bb.102:                              #   in Loop: Header=BB3_5 Depth=2
	li 3, 0
	mr	4, 30
.Lp9_attention_BB3_103:                              #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	ld 6, 392(1)                            # 8-byte Folded Reload
	sldi 5, 3, 2
	lfsx 0, 6, 5
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.104:                              #   in Loop: Header=BB3_103 Depth=3
	ld 6, 392(1)                            # 8-byte Folded Reload
	add 5, 6, 5
	lfs 0, 4(5)
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.105:                              #   in Loop: Header=BB3_103 Depth=3
	lfs 0, 8(5)
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.106:                              #   in Loop: Header=BB3_103 Depth=3
	lfs 0, 12(5)
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.107:                              #   in Loop: Header=BB3_103 Depth=3
	lfs 0, 16(5)
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.108:                              #   in Loop: Header=BB3_103 Depth=3
	lfs 0, 20(5)
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.109:                              #   in Loop: Header=BB3_103 Depth=3
	lfs 0, 24(5)
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.110:                              #   in Loop: Header=BB3_103 Depth=3
	lfs 0, 28(5)
	xststdcsp 1, 0, 112
	bc 12, 6, .Lp9_attention_BB3_136
# %bb.111:                              #   in Loop: Header=BB3_103 Depth=3
	addi 4, 4, -8
	addi 3, 3, 8
	cmpldi 1, 4, 0
	bne	1, .Lp9_attention_BB3_103
# %bb.112:                              #   in Loop: Header=BB3_5 Depth=2
	ld 20, 432(1)                           # 8-byte Folded Reload
	ld 17, 424(1)                           # 8-byte Folded Reload
	bc 4, 2, .Lp9_attention_BB3_114
.Lp9_attention_BB3_113:                              #   in Loop: Header=BB3_5 Depth=2
	ld 26, 176(1)                           # 8-byte Folded Reload
	ld 0, 88(1)                             # 8-byte Folded Reload
	ld 5, 112(1)                            # 8-byte Folded Reload
	ld 11, 104(1)                           # 8-byte Folded Reload
	li 4, 1
	li 9, 2
	li 10, 3
	ld 12, 96(1)                            # 8-byte Folded Reload
	ld 24, 80(1)                            # 8-byte Folded Reload
	ld 27, 120(1)                           # 8-byte Folded Reload
	addi 26, 26, 4
	cmpld	26, 0
	blt	0, .Lp9_attention_BB3_5
	b .Lp9_attention_BB3_135
.Lp9_attention_BB3_114:                              #   in Loop: Header=BB3_5 Depth=2
	li 3, 0
	mr	4, 30
.Lp9_attention_BB3_115:                              #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	ld 6, 408(1)                            # 8-byte Folded Reload
	sldi 5, 3, 2
	lfsx 0, 6, 5
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.116:                              #   in Loop: Header=BB3_115 Depth=3
	ld 6, 408(1)                            # 8-byte Folded Reload
	add 5, 6, 5
	lfs 0, 4(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.117:                              #   in Loop: Header=BB3_115 Depth=3
	lfs 0, 8(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.118:                              #   in Loop: Header=BB3_115 Depth=3
	lfs 0, 12(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.119:                              #   in Loop: Header=BB3_115 Depth=3
	lfs 0, 16(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.120:                              #   in Loop: Header=BB3_115 Depth=3
	lfs 0, 20(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.121:                              #   in Loop: Header=BB3_115 Depth=3
	lfs 0, 24(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.122:                              #   in Loop: Header=BB3_115 Depth=3
	lfs 0, 28(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.123:                              #   in Loop: Header=BB3_115 Depth=3
	addi 4, 4, -8
	addi 3, 3, 8
	cmpldi	4, 0
	bne	0, .Lp9_attention_BB3_115
# %bb.124:                              #   in Loop: Header=BB3_5 Depth=2
	ld 20, 432(1)                           # 8-byte Folded Reload
	ld 17, 424(1)                           # 8-byte Folded Reload
	bc 4, 9, .Lp9_attention_BB3_113
# %bb.125:                              #   in Loop: Header=BB3_5 Depth=2
	li 3, 0
	mr	4, 30
.Lp9_attention_BB3_126:                              #   Parent Loop BB3_4 Depth=1
                                        #     Parent Loop BB3_5 Depth=2
                                        # =>    This Inner Loop Header: Depth=3
	ld 6, 400(1)                            # 8-byte Folded Reload
	sldi 5, 3, 2
	lfsx 0, 6, 5
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.127:                              #   in Loop: Header=BB3_126 Depth=3
	ld 6, 400(1)                            # 8-byte Folded Reload
	add 5, 6, 5
	lfs 0, 4(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.128:                              #   in Loop: Header=BB3_126 Depth=3
	lfs 0, 8(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.129:                              #   in Loop: Header=BB3_126 Depth=3
	lfs 0, 12(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.130:                              #   in Loop: Header=BB3_126 Depth=3
	lfs 0, 16(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.131:                              #   in Loop: Header=BB3_126 Depth=3
	lfs 0, 20(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.132:                              #   in Loop: Header=BB3_126 Depth=3
	lfs 0, 24(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.133:                              #   in Loop: Header=BB3_126 Depth=3
	lfs 0, 28(5)
	xststdcsp 0, 0, 112
	bc 12, 2, .Lp9_attention_BB3_136
# %bb.134:                              #   in Loop: Header=BB3_126 Depth=3
	addi 4, 4, -8
	ld 20, 432(1)                           # 8-byte Folded Reload
	ld 17, 424(1)                           # 8-byte Folded Reload
	addi 3, 3, 8
	cmpldi	4, 0
	bne	0, .Lp9_attention_BB3_126
	b .Lp9_attention_BB3_113
.Lp9_attention_BB3_135:                              #   in Loop: Header=BB3_4 Depth=1
	ld 3, 280(1)                            # 8-byte Folded Reload
	ld 25, 56(1)                            # 8-byte Folded Reload
	ld 8, 72(1)                             # 8-byte Folded Reload
	ld 6, 64(1)                             # 8-byte Folded Reload
	add 3, 3, 25
	addi 8, 8, 1
	std 3, 280(1)                           # 8-byte Folded Spill
	ld 3, 272(1)                            # 8-byte Folded Reload
	cmpld	8, 29
	add 3, 3, 25
	std 3, 272(1)                           # 8-byte Folded Spill
	ld 3, 232(1)                            # 8-byte Folded Reload
	add 3, 3, 25
	std 3, 232(1)                           # 8-byte Folded Spill
	ld 3, 168(1)                            # 8-byte Folded Reload
	add 3, 3, 25
	std 3, 168(1)                           # 8-byte Folded Spill
	li 3, 1
	bne	0, .Lp9_attention_BB3_4
	b .Lp9_attention_BB3_138
.Lp9_attention_BB3_136:
	li 3, 0
	b .Lp9_attention_BB3_138
.Lp9_attention_BB3_137:
	li 3, 1
.Lp9_attention_BB3_138:
	lxv 63, 528(1)                          # 16-byte Folded Reload
	lfd 31, 776(1)                          # 8-byte Folded Reload
	lfd 30, 768(1)                          # 8-byte Folded Reload
	ld 31, 680(1)                           # 8-byte Folded Reload
	ld 30, 672(1)                           # 8-byte Folded Reload
	ld 29, 664(1)                           # 8-byte Folded Reload
	ld 28, 656(1)                           # 8-byte Folded Reload
	ld 27, 648(1)                           # 8-byte Folded Reload
	ld 26, 640(1)                           # 8-byte Folded Reload
	lfd 29, 760(1)                          # 8-byte Folded Reload
	ld 25, 632(1)                           # 8-byte Folded Reload
	ld 24, 624(1)                           # 8-byte Folded Reload
	ld 23, 616(1)                           # 8-byte Folded Reload
	lfd 28, 752(1)                          # 8-byte Folded Reload
	ld 22, 608(1)                           # 8-byte Folded Reload
	ld 21, 600(1)                           # 8-byte Folded Reload
	ld 20, 592(1)                           # 8-byte Folded Reload
	lfd 27, 744(1)                          # 8-byte Folded Reload
	ld 19, 584(1)                           # 8-byte Folded Reload
	ld 18, 576(1)                           # 8-byte Folded Reload
	ld 17, 568(1)                           # 8-byte Folded Reload
	lfd 26, 736(1)                          # 8-byte Folded Reload
	ld 16, 560(1)                           # 8-byte Folded Reload
	ld 15, 552(1)                           # 8-byte Folded Reload
	ld 14, 544(1)                           # 8-byte Folded Reload
	lfd 25, 728(1)                          # 8-byte Folded Reload
	lfd 24, 720(1)                          # 8-byte Folded Reload
	lfd 23, 712(1)                          # 8-byte Folded Reload
	lfd 22, 704(1)                          # 8-byte Folded Reload
	lfd 21, 696(1)                          # 8-byte Folded Reload
	lfd 20, 688(1)                          # 8-byte Folded Reload
	addi 1, 1, 784
	ld 0, 16(1)
	lwz 12, 8(1)
	mtlr 0
	mtocrf 32, 12
	mtocrf 16, 12
	mtocrf 8, 12
	blr
	.long	0
	.quad	0
.Lp9_attention_func_end3:
	.size	pa_attention_f32_power9_multi, .Lp9_attention_func_end3-.Lp9_attention_func_begin3
	.cfi_endproc
                                        # -- End function
