# attention/pa_attention_f32_power9_heads.s: hand-written kernel.
# Query heads [h_first, h_end) of one position, binary32. Same results as the portable twin
# (src/kernels/portable/attention.rs) bit for bit:
#   score[t]  = tree-reduced 16-lane fma dot product (lane l = fma chain over d % 16 == l),
#   softmax   = p9_attention_paf_softmax (unchanged, shared with the other attention kernels),
#   out[d]    = fma chain over t of weight[t] * V[t][d] starting from +0.0.
# Only the schedule differs from the former Clang output: the score loop takes four key rows per
# pass over the query (16 independent fma chains instead of 4), the weighted-V sum keeps four
# 16-float output tiles (16 vectors) in flight per pass over the rows, and the final finiteness
# test is vectorised. No addition is reordered or regrouped.
#
# Two query heads of one KV group (h, h + 1 in the range, same h / group) run as a pair
# (.Lp9_attention_h_pair): each key row and value row is loaded once for both heads, and
# value rows are prefetched ahead; a single head (range or group edge) takes the plain
# path. Per head the arithmetic and its order are identical in both paths. Scratch holds
# 2 * nkeys floats (the pair's two score rows).
#
# Arguments: r3 query, r4 keys, r5 values, r6 qheads, r7 kvheads, r8 dim, r9 nkeys, r10 allowed,
#   then (after our 288-byte frame) 384(r1) scratch, 392(r1) out, 400(r1) h_first, 408(r1) h_end.
# Returns r3 = 1 on success, 0 on bad geometry or a nonfinite allowed score / output value.
# Registers: r14 query, r15 keys, r16 values, r17 allowed, r18 scratch, r19 out, r20 dim,
#   r21 nkeys, r22 qheads/kvheads, r23 key row stride (bytes), r24 head, r25 h_end, r26 out row,
#   r27 dim/16, r28 kv-head byte offset, r29 query row, r30 key block pointer, r31 4*stride.
# VSX: vs32-47 accumulators, vs48-51 query chunk / weight splat / permute control, vs0-13 loads.
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lp9_attention_CPI0_0:
	.byte	31
	.byte	30
	.byte	29
	.byte	28
	.byte	19
	.byte	18
	.byte	17
	.byte	16
	.byte	31
	.byte	31
	.byte	31
	.byte	31
	.byte	31
	.byte	31
	.byte	31
	.byte	31
	.section	.text.pa_attention_f32_power9_heads,"ax",@progbits
	.globl	pa_attention_f32_power9_heads
	.p2align 6
	.type	pa_attention_f32_power9_heads,@function
pa_attention_f32_power9_heads:
.Lp9_attention_func_begin0:
	.cfi_startproc
.Lp9_attention_func_gep0:
	addis 2, 12, .TOC.-.Lp9_attention_func_gep0@ha
	addi 2, 2, .TOC.-.Lp9_attention_func_gep0@l
.Lp9_attention_func_lep0:
	.localentry	pa_attention_f32_power9_heads, .Lp9_attention_func_lep0-.Lp9_attention_func_gep0
	mflr 0
	stdu 1, -288(1)
	std 0, 304(1)
	.cfi_def_cfa_offset 288
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
	std 14, 144(1)
	std 15, 152(1)
	std 16, 160(1)
	std 17, 168(1)
	std 18, 176(1)
	std 19, 184(1)
	std 20, 192(1)
	std 21, 200(1)
	std 22, 208(1)
	std 23, 216(1)
	std 24, 224(1)
	std 25, 232(1)
	std 26, 240(1)
	std 27, 248(1)
	std 28, 256(1)
	std 29, 264(1)
	std 30, 272(1)
	std 31, 280(1)
	ld 12, 408(1)
	ld 11, 400(1)
	cmpld 11, 12
	bgt .Lp9_attention_h_fail
	cmpld 12, 6
	bgt .Lp9_attention_h_fail
	cmpdi 7, 0
	beq .Lp9_attention_h_fail
	andi. 0, 8, 15
	bne .Lp9_attention_h_fail
	divdu 22, 6, 7
	mulld 0, 22, 7
	cmpld 0, 6
	bne .Lp9_attention_h_fail
	cmpld 11, 12
	beq .Lp9_attention_h_ok
	mr 14, 3
	mr 15, 4
	mr 16, 5
	mr 17, 10
	mr 20, 8
	mr 21, 9
	ld 18, 384(1)
	ld 19, 392(1)
	mr 24, 11
	mr 25, 12
	mulld 23, 7, 8
	sldi 23, 23, 2
	sldi 31, 23, 2
	srdi 27, 8, 4
	mr 26, 19
.Lp9_attention_h_head:
	addi 4, 24, 1
	cmpld 4, 25
	bge .Lp9_attention_h_single
	divdu 0, 4, 22
	divdu 5, 24, 22
	cmpld 0, 5
	beq .Lp9_attention_h_pair
.Lp9_attention_h_single:
	divdu 0, 24, 22
	mulld 0, 0, 20
	sldi 28, 0, 2
	mulld 29, 24, 20
	sldi 29, 29, 2
	add 29, 14, 29
	add 30, 15, 28
	mr 12, 18
	mr 5, 21
	cmpdi 5, 0
	beq .Lp9_attention_h_softmax
.Lp9_attention_h_blk:
	cmpdi 5, 4
	li 11, 4
	bge .Lp9_attention_h_full
	mr 11, 5
.Lp9_attention_h_full:
	mr 6, 30
	add 7, 6, 23
	add 8, 7, 23
	add 9, 8, 23
	cmpdi 11, 4
	beq .Lp9_attention_h_go
	cmpdi 11, 1
	bgt .Lp9_attention_h_p2
	mr 7, 6
.Lp9_attention_h_p2:
	cmpdi 11, 2
	bgt .Lp9_attention_h_p3
	mr 8, 6
.Lp9_attention_h_p3:
	cmpdi 11, 3
	bgt .Lp9_attention_h_go
	mr 9, 6
.Lp9_attention_h_go:
	xxlxor 32, 32, 32
	xxlxor 33, 33, 33
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
	xxlxor 36, 36, 36
	xxlxor 37, 37, 37
	xxlxor 38, 38, 38
	xxlxor 39, 39, 39
	xxlxor 40, 40, 40
	xxlxor 41, 41, 41
	xxlxor 42, 42, 42
	xxlxor 43, 43, 43
	xxlxor 44, 44, 44
	xxlxor 45, 45, 45
	xxlxor 46, 46, 46
	xxlxor 47, 47, 47
	mr 10, 29
	srdi. 4, 27, 1
	beq .Lp9_attention_h_stail
	mtctr 4
	sldi 3, 31, 0
	.p2align 5
.Lp9_attention_h_sloop:
	lxv 48, 0(10)
	lxv 49, 16(10)
	lxv 50, 32(10)
	lxv 51, 48(10)
	lxv 0, 0(6)
	xvmaddasp 32, 48, 0
	lxv 1, 16(6)
	xvmaddasp 33, 49, 1
	lxv 2, 32(6)
	xvmaddasp 34, 50, 2
	lxv 3, 48(6)
	xvmaddasp 35, 51, 3
	lxv 4, 0(7)
	xvmaddasp 36, 48, 4
	lxv 5, 16(7)
	xvmaddasp 37, 49, 5
	lxv 6, 32(7)
	xvmaddasp 38, 50, 6
	lxv 7, 48(7)
	xvmaddasp 39, 51, 7
	lxv 8, 0(8)
	xvmaddasp 40, 48, 8
	lxv 9, 16(8)
	xvmaddasp 41, 49, 9
	lxv 10, 32(8)
	xvmaddasp 42, 50, 10
	lxv 11, 48(8)
	xvmaddasp 43, 51, 11
	lxv 0, 0(9)
	xvmaddasp 44, 48, 0
	lxv 1, 16(9)
	xvmaddasp 45, 49, 1
	lxv 2, 32(9)
	xvmaddasp 46, 50, 2
	lxv 3, 48(9)
	xvmaddasp 47, 51, 3
	lxv 48, 64(10)
	lxv 49, 80(10)
	lxv 50, 96(10)
	lxv 51, 112(10)
	lxv 4, 64(6)
	xvmaddasp 32, 48, 4
	lxv 5, 80(6)
	xvmaddasp 33, 49, 5
	lxv 6, 96(6)
	xvmaddasp 34, 50, 6
	lxv 7, 112(6)
	xvmaddasp 35, 51, 7
	lxv 8, 64(7)
	xvmaddasp 36, 48, 8
	lxv 9, 80(7)
	xvmaddasp 37, 49, 9
	lxv 10, 96(7)
	xvmaddasp 38, 50, 10
	lxv 11, 112(7)
	xvmaddasp 39, 51, 11
	lxv 0, 64(8)
	xvmaddasp 40, 48, 0
	lxv 1, 80(8)
	xvmaddasp 41, 49, 1
	lxv 2, 96(8)
	xvmaddasp 42, 50, 2
	lxv 3, 112(8)
	xvmaddasp 43, 51, 3
	lxv 4, 64(9)
	xvmaddasp 44, 48, 4
	lxv 5, 80(9)
	xvmaddasp 45, 49, 5
	lxv 6, 96(9)
	xvmaddasp 46, 50, 6
	lxv 7, 112(9)
	xvmaddasp 47, 51, 7
	dcbt 6, 3
	dcbt 7, 3
	dcbt 8, 3
	dcbt 9, 3
	addi 10, 10, 128
	addi 6, 6, 128
	addi 7, 7, 128
	addi 8, 8, 128
	addi 9, 9, 128
	bdnz .Lp9_attention_h_sloop
.Lp9_attention_h_stail:
	andi. 0, 27, 1
	beq .Lp9_attention_h_sred
	lxv 48, 0(10)
	lxv 49, 16(10)
	lxv 50, 32(10)
	lxv 51, 48(10)
	lxv 8, 0(6)
	xvmaddasp 32, 48, 8
	lxv 9, 16(6)
	xvmaddasp 33, 49, 9
	lxv 10, 32(6)
	xvmaddasp 34, 50, 10
	lxv 11, 48(6)
	xvmaddasp 35, 51, 11
	lxv 0, 0(7)
	xvmaddasp 36, 48, 0
	lxv 1, 16(7)
	xvmaddasp 37, 49, 1
	lxv 2, 32(7)
	xvmaddasp 38, 50, 2
	lxv 3, 48(7)
	xvmaddasp 39, 51, 3
	lxv 4, 0(8)
	xvmaddasp 40, 48, 4
	lxv 5, 16(8)
	xvmaddasp 41, 49, 5
	lxv 6, 32(8)
	xvmaddasp 42, 50, 6
	lxv 7, 48(8)
	xvmaddasp 43, 51, 7
	lxv 8, 0(9)
	xvmaddasp 44, 48, 8
	lxv 9, 16(9)
	xvmaddasp 45, 49, 9
	lxv 10, 32(9)
	xvmaddasp 46, 50, 10
	lxv 11, 48(9)
	xvmaddasp 47, 51, 11
.Lp9_attention_h_sred:
	addis 3, 2, .Lp9_attention_CPI0_0@toc@ha
	addi 3, 3, .Lp9_attention_CPI0_0@toc@l
	lxv 51, 0(3)
	xvaddsp 32, 32, 33
	xvaddsp 34, 34, 35
	xvaddsp 32, 32, 34
	vsldoi 1, 0, 0, 12
	vperm 0, 0, 0, 19
	xvaddsp 0, 33, 32
	xxspltw 1, 0, 2
	xvaddsp 0, 0, 1
	xxsldwi 0, 0, 0, 2
	stfiwx 0, 0, 12
	addi 12, 12, 4
	cmpdi 11, 1
	ble .Lp9_attention_h_sdone
	xvaddsp 36, 36, 37
	xvaddsp 38, 38, 39
	xvaddsp 36, 36, 38
	vsldoi 5, 4, 4, 12
	vperm 4, 4, 4, 19
	xvaddsp 0, 37, 36
	xxspltw 1, 0, 2
	xvaddsp 0, 0, 1
	xxsldwi 0, 0, 0, 2
	stfiwx 0, 0, 12
	addi 12, 12, 4
	cmpdi 11, 2
	ble .Lp9_attention_h_sdone
	xvaddsp 40, 40, 41
	xvaddsp 42, 42, 43
	xvaddsp 40, 40, 42
	vsldoi 9, 8, 8, 12
	vperm 8, 8, 8, 19
	xvaddsp 0, 41, 40
	xxspltw 1, 0, 2
	xvaddsp 0, 0, 1
	xxsldwi 0, 0, 0, 2
	stfiwx 0, 0, 12
	addi 12, 12, 4
	cmpdi 11, 3
	ble .Lp9_attention_h_sdone
	xvaddsp 44, 44, 45
	xvaddsp 46, 46, 47
	xvaddsp 44, 44, 46
	vsldoi 13, 12, 12, 12
	vperm 12, 12, 12, 19
	xvaddsp 0, 45, 44
	xxspltw 1, 0, 2
	xvaddsp 0, 0, 1
	xxsldwi 0, 0, 0, 2
	stfiwx 0, 0, 12
	addi 12, 12, 4
.Lp9_attention_h_sdone:
	add 30, 30, 31
	sub. 5, 5, 11
	bgt .Lp9_attention_h_blk
.Lp9_attention_h_softmax:
	mr 3, 18
	mr 4, 21
	mr 5, 17
	bl p9_attention_paf_softmax
	andi. 3, 3, 1
	beq .Lp9_attention_h_fail
	add 7, 16, 28
	cmpdi 21, 0
	bne .Lp9_attention_h_vsum
	cmpdi 27, 0
	beq .Lp9_attention_h_next
	xxlxor 0, 0, 0
	sldi 4, 27, 2
	mtctr 4
	mr 8, 26
.Lp9_attention_h_zero:
	stxv 0, 0(8)
	addi 8, 8, 16
	bdnz .Lp9_attention_h_zero
	b .Lp9_attention_h_next
.Lp9_attention_h_vsum:
	li 10, 0
	srdi. 9, 27, 2
	beq .Lp9_attention_h_vtiles
.Lp9_attention_h_vblock:
	xxlxor 32, 32, 32
	xxlxor 33, 33, 33
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
	xxlxor 36, 36, 36
	xxlxor 37, 37, 37
	xxlxor 38, 38, 38
	xxlxor 39, 39, 39
	xxlxor 40, 40, 40
	xxlxor 41, 41, 41
	xxlxor 42, 42, 42
	xxlxor 43, 43, 43
	xxlxor 44, 44, 44
	xxlxor 45, 45, 45
	xxlxor 46, 46, 46
	xxlxor 47, 47, 47
	add 6, 7, 10
	mr 8, 18
	mtctr 21
	.p2align 5
.Lp9_attention_h_vloop:
	lxvwsx 48, 0, 8
	lxv 0, 0(6)
	xvmaddasp 32, 48, 0
	lxv 1, 16(6)
	xvmaddasp 33, 48, 1
	lxv 2, 32(6)
	xvmaddasp 34, 48, 2
	lxv 3, 48(6)
	xvmaddasp 35, 48, 3
	lxv 4, 64(6)
	xvmaddasp 36, 48, 4
	lxv 5, 80(6)
	xvmaddasp 37, 48, 5
	lxv 6, 96(6)
	xvmaddasp 38, 48, 6
	lxv 7, 112(6)
	xvmaddasp 39, 48, 7
	lxv 8, 128(6)
	xvmaddasp 40, 48, 8
	lxv 9, 144(6)
	xvmaddasp 41, 48, 9
	lxv 10, 160(6)
	xvmaddasp 42, 48, 10
	lxv 11, 176(6)
	xvmaddasp 43, 48, 11
	lxv 0, 192(6)
	xvmaddasp 44, 48, 0
	lxv 1, 208(6)
	xvmaddasp 45, 48, 1
	lxv 2, 224(6)
	xvmaddasp 46, 48, 2
	lxv 3, 240(6)
	xvmaddasp 47, 48, 3
	add 6, 6, 23
	addi 8, 8, 4
	bdnz .Lp9_attention_h_vloop
	add 8, 26, 10
	stxv 32, 0(8)
	stxv 33, 16(8)
	stxv 34, 32(8)
	stxv 35, 48(8)
	stxv 36, 64(8)
	stxv 37, 80(8)
	stxv 38, 96(8)
	stxv 39, 112(8)
	stxv 40, 128(8)
	stxv 41, 144(8)
	stxv 42, 160(8)
	stxv 43, 176(8)
	stxv 44, 192(8)
	stxv 45, 208(8)
	stxv 46, 224(8)
	stxv 47, 240(8)
	addi 10, 10, 256
	addi 9, 9, -1
	cmpdi 9, 0
	bne .Lp9_attention_h_vblock
.Lp9_attention_h_vtiles:
	andi. 9, 27, 3
	beq .Lp9_attention_h_vend
.Lp9_attention_h_vtile:
	xxlxor 32, 32, 32
	xxlxor 33, 33, 33
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
	add 6, 7, 10
	mr 8, 18
	mtctr 21
	.p2align 5
.Lp9_attention_h_vloop1:
	lxvwsx 48, 0, 8
	lxv 0, 0(6)
	xvmaddasp 32, 48, 0
	lxv 1, 16(6)
	xvmaddasp 33, 48, 1
	lxv 2, 32(6)
	xvmaddasp 34, 48, 2
	lxv 3, 48(6)
	xvmaddasp 35, 48, 3
	add 6, 6, 23
	addi 8, 8, 4
	bdnz .Lp9_attention_h_vloop1
	add 8, 26, 10
	stxv 32, 0(8)
	stxv 33, 16(8)
	stxv 34, 32(8)
	stxv 35, 48(8)
	addi 10, 10, 64
	addi 9, 9, -1
	cmpdi 9, 0
	bne .Lp9_attention_h_vtile
.Lp9_attention_h_vend:
	cmpdi 27, 0
	beq .Lp9_attention_h_next
	xxlxor 32, 32, 32
	xxlxor 33, 33, 33
	mtctr 27
	mr 8, 26
.Lp9_attention_h_fin:
	lxv 0, 0(8)
	lxv 1, 16(8)
	lxv 2, 32(8)
	lxv 3, 48(8)
	xvtstdcsp 4, 0, 112
	xvtstdcsp 5, 1, 112
	xvtstdcsp 6, 2, 112
	xvtstdcsp 7, 3, 112
	xxlor 32, 32, 4
	xxlor 32, 32, 5
	xxlor 32, 32, 6
	xxlor 32, 32, 7
	addi 8, 8, 64
	bdnz .Lp9_attention_h_fin
	vcmpequw. 2, 0, 1
	bge 6, .Lp9_attention_h_fail
.Lp9_attention_h_next:
	sldi 0, 20, 2
	add 26, 26, 0
	addi 24, 24, 1
	cmpld 24, 25
	blt .Lp9_attention_h_head
	b .Lp9_attention_h_ok
# Two query heads of one KV group per pass over the keys and values: every key row
# and value row is loaded once and feeds both heads. Per head the arithmetic is the
# single-head path's, in the same order (score = 16 fma chains over d % 16, summed by
# the same tree; out[d] = fma chain over t starting from +0.0). Two key rows x two
# heads = 16 accumulators vs32-vs47 (vs32-35 key 0/head 0, vs36-39 key 1/head 0,
# vs40-43 key 0/head 1, vs44-47 key 1/head 1); head 1's scores follow head 0's in
# the scratch (2 * nkeys floats). r8 = head 1's query row, r3 = nkeys * 4.
.Lp9_attention_h_pair:
	divdu 0, 24, 22
	mulld 0, 0, 20
	sldi 28, 0, 2
	mulld 29, 24, 20
	sldi 29, 29, 2
	add 29, 14, 29
	sldi 4, 20, 2
	add 8, 29, 4
	add 30, 15, 28
	mr 12, 18
	sldi 3, 21, 2
	mr 5, 21
	cmpdi 5, 0
	beq .Lp9_attention_h_psoftmax
.Lp9_attention_h_pblk:
	li 11, 2
	cmpdi 5, 2
	bge .Lp9_attention_h_pfull
	li 11, 1
.Lp9_attention_h_pfull:
	mr 6, 30
	add 7, 6, 23
	cmpdi 11, 2
	beq .Lp9_attention_h_pgo
	mr 7, 6
.Lp9_attention_h_pgo:
	mulli 0, 23, 4
	xxlxor 32, 32, 32
	xxlxor 33, 33, 33
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
	xxlxor 36, 36, 36
	xxlxor 37, 37, 37
	xxlxor 38, 38, 38
	xxlxor 39, 39, 39
	xxlxor 40, 40, 40
	xxlxor 41, 41, 41
	xxlxor 42, 42, 42
	xxlxor 43, 43, 43
	xxlxor 44, 44, 44
	xxlxor 45, 45, 45
	xxlxor 46, 46, 46
	xxlxor 47, 47, 47
	mr 10, 29
	mr 9, 8
	srdi. 4, 27, 1
	beq .Lp9_attention_h_ptail
	mtctr 4
	.p2align 5
.Lp9_attention_h_ploop:
	lxv 48, 0(10)
	lxv 49, 16(10)
	lxv 50, 32(10)
	lxv 51, 48(10)
	lxv 0, 0(9)
	lxv 1, 16(9)
	lxv 2, 32(9)
	lxv 3, 48(9)
	lxv 4, 0(6)
	lxv 5, 16(6)
	lxv 6, 32(6)
	lxv 7, 48(6)
	lxv 8, 0(7)
	lxv 9, 16(7)
	lxv 10, 32(7)
	lxv 11, 48(7)
	xvmaddasp 32, 48, 4
	xvmaddasp 40, 0, 4
	xvmaddasp 33, 49, 5
	xvmaddasp 41, 1, 5
	xvmaddasp 34, 50, 6
	xvmaddasp 42, 2, 6
	xvmaddasp 35, 51, 7
	xvmaddasp 43, 3, 7
	xvmaddasp 36, 48, 8
	xvmaddasp 44, 0, 8
	xvmaddasp 37, 49, 9
	xvmaddasp 45, 1, 9
	xvmaddasp 38, 50, 10
	xvmaddasp 46, 2, 10
	xvmaddasp 39, 51, 11
	xvmaddasp 47, 3, 11
	lxv 48, 64(10)
	lxv 49, 80(10)
	lxv 50, 96(10)
	lxv 51, 112(10)
	lxv 0, 64(9)
	lxv 1, 80(9)
	lxv 2, 96(9)
	lxv 3, 112(9)
	lxv 4, 64(6)
	lxv 5, 80(6)
	lxv 6, 96(6)
	lxv 7, 112(6)
	lxv 8, 64(7)
	lxv 9, 80(7)
	lxv 10, 96(7)
	lxv 11, 112(7)
	xvmaddasp 32, 48, 4
	xvmaddasp 40, 0, 4
	xvmaddasp 33, 49, 5
	xvmaddasp 41, 1, 5
	xvmaddasp 34, 50, 6
	xvmaddasp 42, 2, 6
	xvmaddasp 35, 51, 7
	xvmaddasp 43, 3, 7
	xvmaddasp 36, 48, 8
	xvmaddasp 44, 0, 8
	xvmaddasp 37, 49, 9
	xvmaddasp 45, 1, 9
	xvmaddasp 38, 50, 10
	xvmaddasp 46, 2, 10
	xvmaddasp 39, 51, 11
	xvmaddasp 47, 3, 11
	dcbt 6, 0
	dcbt 7, 0
	addi 10, 10, 128
	addi 9, 9, 128
	addi 6, 6, 128
	addi 7, 7, 128
	bdnz .Lp9_attention_h_ploop
.Lp9_attention_h_ptail:
	andi. 0, 27, 1
	beq .Lp9_attention_h_pred
	lxv 48, 0(10)
	lxv 49, 16(10)
	lxv 50, 32(10)
	lxv 51, 48(10)
	lxv 0, 0(9)
	lxv 1, 16(9)
	lxv 2, 32(9)
	lxv 3, 48(9)
	lxv 4, 0(6)
	lxv 5, 16(6)
	lxv 6, 32(6)
	lxv 7, 48(6)
	lxv 8, 0(7)
	lxv 9, 16(7)
	lxv 10, 32(7)
	lxv 11, 48(7)
	xvmaddasp 32, 48, 4
	xvmaddasp 40, 0, 4
	xvmaddasp 33, 49, 5
	xvmaddasp 41, 1, 5
	xvmaddasp 34, 50, 6
	xvmaddasp 42, 2, 6
	xvmaddasp 35, 51, 7
	xvmaddasp 43, 3, 7
	xvmaddasp 36, 48, 8
	xvmaddasp 44, 0, 8
	xvmaddasp 37, 49, 9
	xvmaddasp 45, 1, 9
	xvmaddasp 38, 50, 10
	xvmaddasp 46, 2, 10
	xvmaddasp 39, 51, 11
	xvmaddasp 47, 3, 11
.Lp9_attention_h_pred:
	addis 4, 2, .Lp9_attention_CPI0_0@toc@ha
	addi 4, 4, .Lp9_attention_CPI0_0@toc@l
	lxv 51, 0(4)
	xvaddsp 32, 32, 33
	xvaddsp 34, 34, 35
	xvaddsp 32, 32, 34
	vsldoi 1, 0, 0, 12
	vperm 0, 0, 0, 19
	xvaddsp 0, 33, 32
	xxspltw 1, 0, 2
	xvaddsp 0, 0, 1
	xxsldwi 0, 0, 0, 2
	stfiwx 0, 0, 12
	xvaddsp 40, 40, 41
	xvaddsp 42, 42, 43
	xvaddsp 40, 40, 42
	vsldoi 9, 8, 8, 12
	vperm 8, 8, 8, 19
	xvaddsp 0, 41, 40
	xxspltw 1, 0, 2
	xvaddsp 0, 0, 1
	xxsldwi 0, 0, 0, 2
	add 4, 12, 3
	stfiwx 0, 0, 4
	addi 12, 12, 4
	cmpdi 11, 1
	ble .Lp9_attention_h_pdone
	xvaddsp 36, 36, 37
	xvaddsp 38, 38, 39
	xvaddsp 36, 36, 38
	vsldoi 5, 4, 4, 12
	vperm 4, 4, 4, 19
	xvaddsp 0, 37, 36
	xxspltw 1, 0, 2
	xvaddsp 0, 0, 1
	xxsldwi 0, 0, 0, 2
	stfiwx 0, 0, 12
	xvaddsp 44, 44, 45
	xvaddsp 46, 46, 47
	xvaddsp 44, 44, 46
	vsldoi 13, 12, 12, 12
	vperm 12, 12, 12, 19
	xvaddsp 0, 45, 44
	xxspltw 1, 0, 2
	xvaddsp 0, 0, 1
	xxsldwi 0, 0, 0, 2
	add 4, 12, 3
	stfiwx 0, 0, 4
	addi 12, 12, 4
.Lp9_attention_h_pdone:
	add 30, 30, 23
	add 30, 30, 23
	sub. 5, 5, 11
	bgt .Lp9_attention_h_pblk
.Lp9_attention_h_psoftmax:
	mr 3, 18
	mr 4, 21
	mr 5, 17
	bl p9_attention_paf_softmax
	andi. 3, 3, 1
	beq .Lp9_attention_h_fail
	sldi 3, 21, 2
	add 3, 3, 18
	mr 4, 21
	mr 5, 17
	bl p9_attention_paf_softmax
	andi. 3, 3, 1
	beq .Lp9_attention_h_fail
	add 7, 16, 28
	cmpdi 21, 0
	bne .Lp9_attention_h_pvsum
	cmpdi 27, 0
	beq .Lp9_attention_h_pnext
	xxlxor 0, 0, 0
	sldi 4, 27, 3
	mtctr 4
	mr 8, 26
.Lp9_attention_h_pzero:
	stxv 0, 0(8)
	addi 8, 8, 16
	bdnz .Lp9_attention_h_pzero
	b .Lp9_attention_h_pnext
.Lp9_attention_h_pvsum:
	sldi 4, 20, 2
	sldi 30, 21, 2
	add 30, 30, 18
	mulli 3, 23, 12
	addi 11, 3, 112
	li 10, 0
	srdi. 5, 27, 1
	beq .Lp9_attention_h_pvtile
.Lp9_attention_h_pvblock:
	xxlxor 32, 32, 32
	xxlxor 33, 33, 33
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
	xxlxor 36, 36, 36
	xxlxor 37, 37, 37
	xxlxor 38, 38, 38
	xxlxor 39, 39, 39
	xxlxor 40, 40, 40
	xxlxor 41, 41, 41
	xxlxor 42, 42, 42
	xxlxor 43, 43, 43
	xxlxor 44, 44, 44
	xxlxor 45, 45, 45
	xxlxor 46, 46, 46
	xxlxor 47, 47, 47
	add 6, 7, 10
	mr 8, 18
	mr 9, 30
	mtctr 21
	.p2align 5
.Lp9_attention_h_pvloop:
	lxvwsx 48, 0, 8
	lxvwsx 49, 0, 9
	lxv 0, 0(6)
	lxv 1, 16(6)
	lxv 2, 32(6)
	lxv 3, 48(6)
	lxv 4, 64(6)
	lxv 5, 80(6)
	lxv 6, 96(6)
	lxv 7, 112(6)
	xvmaddasp 32, 48, 0
	xvmaddasp 40, 49, 0
	xvmaddasp 33, 48, 1
	xvmaddasp 41, 49, 1
	xvmaddasp 34, 48, 2
	xvmaddasp 42, 49, 2
	xvmaddasp 35, 48, 3
	xvmaddasp 43, 49, 3
	xvmaddasp 36, 48, 4
	xvmaddasp 44, 49, 4
	xvmaddasp 37, 48, 5
	xvmaddasp 45, 49, 5
	xvmaddasp 38, 48, 6
	xvmaddasp 46, 49, 6
	xvmaddasp 39, 48, 7
	xvmaddasp 47, 49, 7
	dcbt 6, 3
	dcbt 6, 11
	add 6, 6, 23
	addi 8, 8, 4
	addi 9, 9, 4
	bdnz .Lp9_attention_h_pvloop
	add 8, 26, 10
	add 9, 8, 4
	stxv 32, 0(8)
	stxv 33, 16(8)
	stxv 34, 32(8)
	stxv 35, 48(8)
	stxv 36, 64(8)
	stxv 37, 80(8)
	stxv 38, 96(8)
	stxv 39, 112(8)
	stxv 40, 0(9)
	stxv 41, 16(9)
	stxv 42, 32(9)
	stxv 43, 48(9)
	stxv 44, 64(9)
	stxv 45, 80(9)
	stxv 46, 96(9)
	stxv 47, 112(9)
	addi 10, 10, 128
	addi 5, 5, -1
	cmpdi 5, 0
	bne .Lp9_attention_h_pvblock
.Lp9_attention_h_pvtile:
	andi. 0, 27, 1
	beq .Lp9_attention_h_pvend
	xxlxor 32, 32, 32
	xxlxor 33, 33, 33
	xxlxor 34, 34, 34
	xxlxor 35, 35, 35
	xxlxor 36, 36, 36
	xxlxor 37, 37, 37
	xxlxor 38, 38, 38
	xxlxor 39, 39, 39
	add 6, 7, 10
	mr 8, 18
	mr 9, 30
	mtctr 21
	.p2align 5
.Lp9_attention_h_pvloop1:
	lxvwsx 48, 0, 8
	lxvwsx 49, 0, 9
	lxv 0, 0(6)
	lxv 1, 16(6)
	lxv 2, 32(6)
	lxv 3, 48(6)
	xvmaddasp 32, 48, 0
	xvmaddasp 36, 49, 0
	xvmaddasp 33, 48, 1
	xvmaddasp 37, 49, 1
	xvmaddasp 34, 48, 2
	xvmaddasp 38, 49, 2
	xvmaddasp 35, 48, 3
	xvmaddasp 39, 49, 3
	add 6, 6, 23
	addi 8, 8, 4
	addi 9, 9, 4
	bdnz .Lp9_attention_h_pvloop1
	add 8, 26, 10
	add 9, 8, 4
	stxv 32, 0(8)
	stxv 33, 16(8)
	stxv 34, 32(8)
	stxv 35, 48(8)
	stxv 36, 0(9)
	stxv 37, 16(9)
	stxv 38, 32(9)
	stxv 39, 48(9)
.Lp9_attention_h_pvend:
	cmpdi 27, 0
	beq .Lp9_attention_h_pnext
	xxlxor 32, 32, 32
	xxlxor 33, 33, 33
	sldi 4, 27, 1
	mtctr 4
	mr 8, 26
.Lp9_attention_h_pfin:
	lxv 0, 0(8)
	lxv 1, 16(8)
	lxv 2, 32(8)
	lxv 3, 48(8)
	xvtstdcsp 4, 0, 112
	xvtstdcsp 5, 1, 112
	xvtstdcsp 6, 2, 112
	xvtstdcsp 7, 3, 112
	xxlor 32, 32, 4
	xxlor 32, 32, 5
	xxlor 32, 32, 6
	xxlor 32, 32, 7
	addi 8, 8, 64
	bdnz .Lp9_attention_h_pfin
	vcmpequw. 2, 0, 1
	bge 6, .Lp9_attention_h_fail
.Lp9_attention_h_pnext:
	sldi 0, 20, 3
	add 26, 26, 0
	addi 24, 24, 2
	cmpld 24, 25
	blt .Lp9_attention_h_head
	b .Lp9_attention_h_ok
.Lp9_attention_h_ok:
	li 3, 1
	b .Lp9_attention_h_exit
.Lp9_attention_h_fail:
	li 3, 0
.Lp9_attention_h_exit:
	ld 14, 144(1)
	ld 15, 152(1)
	ld 16, 160(1)
	ld 17, 168(1)
	ld 18, 176(1)
	ld 19, 184(1)
	ld 20, 192(1)
	ld 21, 200(1)
	ld 22, 208(1)
	ld 23, 216(1)
	ld 24, 224(1)
	ld 25, 232(1)
	ld 26, 240(1)
	ld 27, 248(1)
	ld 28, 256(1)
	ld 29, 264(1)
	ld 30, 272(1)
	ld 31, 280(1)
	ld 0, 304(1)
	addi 1, 1, 288
	mtlr 0
	blr
	.long	0
	.quad	0
.Lp9_attention_func_end0:
	.size	pa_attention_f32_power9_heads, .Lp9_attention_func_end0-.Lp9_attention_func_begin0
	.cfi_endproc
