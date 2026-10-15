# pq8_q4_0_tile4x4_power9: the POWER9 int8 prompt tile for Q4_0 weights,
# 4 row groups (16 rows) x 4 positions, bit-identical to
# tile(2, 4, 4, ...) in the former quantized_matmul_power9_int8.c. Per block
# b, for position p and row group r (lanes = 4 rows):
#   t[r][p] = -8 * S[b][p] + sum_k vmsummbm(a_k[p] (signed), w_k[r] (unsigned codes))
#   out[r][p] = fma(float(t[r][p]), d[b][r] * da[b][p], out[r][p])
# The integer sums are exact in any order; the float update per block and
# (r, p) is the same convert, product and fused multiply-add as the C code.
#
# Hand-written assembly, originally emitted by the former schedule generator
# tools/codegen/gen-q4-0-tile.py; this file is canonical; the Rust module includes
# it directly (edit it, then cargo build).
#
# Why assembly: the C compiler places the high-nibble product of an
# accumulator 4 instructions after its low-nibble product (vmsummbm latency 7
# cycles); POWER9's issue queue then fills with dependent products (dispatch
# held about half the time). Here each chunk pair issues all 16 low-nibble
# products, then all 16 high-nibble ones, so dependent products are 16 apart
# (+7.7% single thread). The register plan needs explicit control: 16
# accumulators, 4 activations (reloaded for the high half), 4 high-nibble
# weights, 4 bias starts and the shift count fill the 32 vector registers.
#
# Instruction count, not latency, bounds this tile: measured with independent
# probes, every vector-pipe op (vmsummbm, logical, shift, convert, multiply,
# FMA, splat) costs about 0.6 cycle in a mixed stream and every scalar
# instruction about 0.65 cycle, and the same ops with no dependences run only
# ~7% faster than the schedule below (reordering, fusing the float tail into
# the next block and prefetching weights one chunk ahead were all measured
# flat). The per-block work other than the 128 products, 32 unpack ops and 48
# float ops was therefore cut:
#  * the four -8 * S[b][p] start vectors are built by vector ops (one 8-byte
#    load, two signed halfword multiplies by -8, four word splats: 7 instructions
#    instead of 16 scalar ones; v4 holds the splat of -8);
#  * the fast loop handles 4 blocks per iteration at fixed displacements
#    (the packed Q4_0 record is 640 bytes per block: 8 row groups x 64 bytes of
#    nibbles, then 8 x 16 bytes of scales; 128 bytes per activation block, 8 per
#    sums block), so the weight, scale and activation pointers are bumped once
#    per 4 blocks. It runs when rec == 640 (the scale pointers w + moff are
#    kept in r14-r17 meanwhile and turned back into offsets afterwards);
#    otherwise, and for the nb % 4 remaining blocks, the generic one-block
#    loop runs with the argument strides. Both loops carry the same float state.
# Float update per block unchanged: convert, d*da product, fused multiply-add,
# in the same order per output.
#
# Args (r3 -> pq8_q4tile_args, 8-byte fields):
#   0   w[4]     packed weights of block 0, per row group
#   32  moff[4]  metadata address minus weight address, per row group
#   64  act[4]   activations of block 0, per position
#   96  s        int16 block sums of the activations, 4 per block
#   104 da       binary32 activation scales, 4 per block
#   112 nb       blocks (>= 1)
#   120 rec      bytes between consecutive blocks of one row group
#   128 out      16 binary32 vectors (index 4r + p)
#
# Register plan:
#   v0-v3   -8 * S per position (first accumulator input; scratch while built)
#   v4      splat of halfword -8        v5,v6 low-nibble weights (alternating)
#   v7      splat 4 (vsrb count)        v8-v11 activations (low half, then high half)
#   v12-v15 high-nibble weights per row group     v16-v31 int32 accumulators (4r+p)
#   vs0 splat 15 mask; vs1-vs4 weight scales d per row group;
#   vs5-vs8 activation scale splats per position; vs9-vs12 converted sums;
#   vs13,vs30,vs31 d*da products; vs14-vs29 binary32 accumulators.
#   r4-r7 weights, r8-r11 activations, r14-r17 metadata offsets, r12 sums,
#   r18 scales, r19 record stride, r20 output, r21 nb, r22 generic-loop blocks,
#   r23 fast iterations, r25-r27 index constants.
# Frame 512: nonvolatile r14-r27, f14-f31 (the doubleword halves of vs14-vs31
# that the ABI preserves), v20-v31.
.macro BIAS sd
lxsd 0, \sd(12)
vmulesh 1, 0, 4
vmulosh 2, 0, 4
vspltw 3, 1, 0
vspltw 0, 2, 1
vspltw 1, 1, 1
vspltw 2, 2, 0
.endm
.macro CHUNK0 wd, ad
lxv 40, \ad(8)
lxv 41, \ad(9)
lxv 42, \ad(10)
lxv 43, \ad(11)
lxv 44, \wd(4)
xxland 37, 44, 0
vsrb 12, 12, 7
vmsummbm 16, 8, 5, 0
vmsummbm 17, 9, 5, 1
vmsummbm 18, 10, 5, 2
vmsummbm 19, 11, 5, 3
lxv 45, \wd(5)
xxland 38, 45, 0
vsrb 13, 13, 7
vmsummbm 20, 8, 6, 0
vmsummbm 21, 9, 6, 1
vmsummbm 22, 10, 6, 2
vmsummbm 23, 11, 6, 3
lxv 46, \wd(6)
xxland 37, 46, 0
vsrb 14, 14, 7
vmsummbm 24, 8, 5, 0
vmsummbm 25, 9, 5, 1
vmsummbm 26, 10, 5, 2
vmsummbm 27, 11, 5, 3
lxv 47, \wd(7)
xxland 38, 47, 0
vsrb 15, 15, 7
vmsummbm 28, 8, 6, 0
vmsummbm 29, 9, 6, 1
vmsummbm 30, 10, 6, 2
vmsummbm 31, 11, 6, 3
lxv 40, \ad+16(8)
lxv 41, \ad+16(9)
lxv 42, \ad+16(10)
lxv 43, \ad+16(11)
vmsummbm 16, 8, 12, 16
vmsummbm 17, 9, 12, 17
vmsummbm 18, 10, 12, 18
vmsummbm 19, 11, 12, 19
vmsummbm 20, 8, 13, 20
vmsummbm 21, 9, 13, 21
vmsummbm 22, 10, 13, 22
vmsummbm 23, 11, 13, 23
vmsummbm 24, 8, 14, 24
vmsummbm 25, 9, 14, 25
vmsummbm 26, 10, 14, 26
vmsummbm 27, 11, 14, 27
vmsummbm 28, 8, 15, 28
vmsummbm 29, 9, 15, 29
vmsummbm 30, 10, 15, 30
vmsummbm 31, 11, 15, 31
.endm
.macro CHUNK wd, ad
lxv 40, \ad(8)
lxv 41, \ad(9)
lxv 42, \ad(10)
lxv 43, \ad(11)
lxv 44, \wd(4)
xxland 37, 44, 0
vsrb 12, 12, 7
vmsummbm 16, 8, 5, 16
vmsummbm 17, 9, 5, 17
vmsummbm 18, 10, 5, 18
vmsummbm 19, 11, 5, 19
lxv 45, \wd(5)
xxland 38, 45, 0
vsrb 13, 13, 7
vmsummbm 20, 8, 6, 20
vmsummbm 21, 9, 6, 21
vmsummbm 22, 10, 6, 22
vmsummbm 23, 11, 6, 23
lxv 46, \wd(6)
xxland 37, 46, 0
vsrb 14, 14, 7
vmsummbm 24, 8, 5, 24
vmsummbm 25, 9, 5, 25
vmsummbm 26, 10, 5, 26
vmsummbm 27, 11, 5, 27
lxv 47, \wd(7)
xxland 38, 47, 0
vsrb 15, 15, 7
vmsummbm 28, 8, 6, 28
vmsummbm 29, 9, 6, 29
vmsummbm 30, 10, 6, 30
vmsummbm 31, 11, 6, 31
lxv 40, \ad+16(8)
lxv 41, \ad+16(9)
lxv 42, \ad+16(10)
lxv 43, \ad+16(11)
vmsummbm 16, 8, 12, 16
vmsummbm 17, 9, 12, 17
vmsummbm 18, 10, 12, 18
vmsummbm 19, 11, 12, 19
vmsummbm 20, 8, 13, 20
vmsummbm 21, 9, 13, 21
vmsummbm 22, 10, 13, 22
vmsummbm 23, 11, 13, 23
vmsummbm 24, 8, 14, 24
vmsummbm 25, 9, 14, 25
vmsummbm 26, 10, 14, 26
vmsummbm 27, 11, 14, 27
vmsummbm 28, 8, 15, 28
vmsummbm 29, 9, 15, 29
vmsummbm 30, 10, 15, 30
vmsummbm 31, 11, 15, 31
.endm
.macro EPI
xvcvsxwsp 9, 48
xvmulsp 13, 1, 5
xvmaddasp 14, 9, 13
xvcvsxwsp 9, 52
xvmulsp 30, 2, 5
xvmaddasp 18, 9, 30
xvcvsxwsp 9, 56
xvmulsp 31, 3, 5
xvmaddasp 22, 9, 31
xvcvsxwsp 9, 60
xvmulsp 13, 4, 5
xvmaddasp 26, 9, 13
xvcvsxwsp 10, 49
xvmulsp 13, 1, 6
xvmaddasp 15, 10, 13
xvcvsxwsp 10, 53
xvmulsp 30, 2, 6
xvmaddasp 19, 10, 30
xvcvsxwsp 10, 57
xvmulsp 31, 3, 6
xvmaddasp 23, 10, 31
xvcvsxwsp 10, 61
xvmulsp 13, 4, 6
xvmaddasp 27, 10, 13
xvcvsxwsp 11, 50
xvmulsp 13, 1, 7
xvmaddasp 16, 11, 13
xvcvsxwsp 11, 54
xvmulsp 30, 2, 7
xvmaddasp 20, 11, 30
xvcvsxwsp 11, 58
xvmulsp 31, 3, 7
xvmaddasp 24, 11, 31
xvcvsxwsp 11, 62
xvmulsp 13, 4, 7
xvmaddasp 28, 11, 13
xvcvsxwsp 12, 51
xvmulsp 13, 1, 8
xvmaddasp 17, 12, 13
xvcvsxwsp 12, 55
xvmulsp 30, 2, 8
xvmaddasp 21, 12, 30
xvcvsxwsp 12, 59
xvmulsp 31, 3, 8
xvmaddasp 25, 12, 31
xvcvsxwsp 12, 63
xvmulsp 13, 4, 8
xvmaddasp 29, 12, 13
.endm
.macro FASTBLK w, a, s, m
BIAS \s
CHUNK0 \w, \a
CHUNK \w+16, \a+32
CHUNK \w+32, \a+64
CHUNK \w+48, \a+96
lxvwsx 5, 0, 18
lxvwsx 6, 18, 25
lxvwsx 7, 18, 26
lxvwsx 8, 18, 27
lxv 1, \m(14)
lxv 2, \m(15)
lxv 3, \m(16)
lxv 4, \m(17)
EPI
addi 18, 18, 16
.endm
stdu 1, -512(1)
std 14, 32(1)
std 15, 40(1)
std 16, 48(1)
std 17, 56(1)
std 18, 64(1)
std 19, 72(1)
std 20, 80(1)
std 21, 88(1)
std 22, 96(1)
std 23, 104(1)
std 24, 112(1)
std 25, 120(1)
std 26, 128(1)
std 27, 136(1)
stfd 14, 144(1)
stfd 15, 152(1)
stfd 16, 160(1)
stfd 17, 168(1)
stfd 18, 176(1)
stfd 19, 184(1)
stfd 20, 192(1)
stfd 21, 200(1)
stfd 22, 208(1)
stfd 23, 216(1)
stfd 24, 224(1)
stfd 25, 232(1)
stfd 26, 240(1)
stfd 27, 248(1)
stfd 28, 256(1)
stfd 29, 264(1)
stfd 30, 272(1)
stfd 31, 280(1)
stxv 52, 288(1)
stxv 53, 304(1)
stxv 54, 320(1)
stxv 55, 336(1)
stxv 56, 352(1)
stxv 57, 368(1)
stxv 58, 384(1)
stxv 59, 400(1)
stxv 60, 416(1)
stxv 61, 432(1)
stxv 62, 448(1)
stxv 63, 464(1)
ld 4, 0(3)
ld 14, 32(3)
ld 8, 64(3)
ld 5, 8(3)
ld 15, 40(3)
ld 9, 72(3)
ld 6, 16(3)
ld 16, 48(3)
ld 10, 80(3)
ld 7, 24(3)
ld 17, 56(3)
ld 11, 88(3)
ld 12, 96(3)
ld 18, 104(3)
ld 21, 112(3)
ld 19, 120(3)
ld 20, 128(3)
li 25, 4
li 26, 8
li 27, 12
xxspltib 0, 15
vspltisb 7, 4
xxlxor 14, 14, 14
xxlxor 15, 15, 15
xxlxor 16, 16, 16
xxlxor 17, 17, 17
xxlxor 18, 18, 18
xxlxor 19, 19, 19
xxlxor 20, 20, 20
xxlxor 21, 21, 21
xxlxor 22, 22, 22
xxlxor 23, 23, 23
xxlxor 24, 24, 24
xxlxor 25, 25, 25
xxlxor 26, 26, 26
xxlxor 27, 27, 27
xxlxor 28, 28, 28
xxlxor 29, 29, 29
vspltish 4, -8
mr 22, 21
cmpdi 19, 640
bne 3f
srdi. 23, 21, 2
beq 3f
clrldi 22, 21, 62
mtctr 23
add 14, 14, 4
add 15, 15, 5
add 16, 16, 6
add 17, 17, 7
2:
FASTBLK 0, 0, 0, 0
FASTBLK 640, 128, 8, 640
FASTBLK 1280, 256, 16, 1280
FASTBLK 1920, 384, 24, 1920
addi 4, 4, 2560
addi 5, 5, 2560
addi 6, 6, 2560
addi 7, 7, 2560
addi 14, 14, 2560
addi 15, 15, 2560
addi 16, 16, 2560
addi 17, 17, 2560
addi 8, 8, 512
addi 9, 9, 512
addi 10, 10, 512
addi 11, 11, 512
addi 12, 12, 32
bdnz 2b
sub 14, 14, 4
sub 15, 15, 5
sub 16, 16, 6
sub 17, 17, 7
3:
cmpdi 22, 0
beq 5f
mtctr 22
4:
BIAS 0
CHUNK0 0, 0
CHUNK 16, 32
CHUNK 32, 64
CHUNK 48, 96
lxvwsx 5, 0, 18
lxvwsx 6, 18, 25
lxvwsx 7, 18, 26
lxvwsx 8, 18, 27
lxvx 1, 4, 14
lxvx 2, 5, 15
lxvx 3, 6, 16
lxvx 4, 7, 17
EPI
add 4, 4, 19
add 5, 5, 19
add 6, 6, 19
add 7, 7, 19
addi 8, 8, 128
addi 9, 9, 128
addi 10, 10, 128
addi 11, 11, 128
addi 12, 12, 8
addi 18, 18, 16
bdnz 4b
5:
stxv 14, 0(20)
stxv 15, 16(20)
stxv 16, 32(20)
stxv 17, 48(20)
stxv 18, 64(20)
stxv 19, 80(20)
stxv 20, 96(20)
stxv 21, 112(20)
stxv 22, 128(20)
stxv 23, 144(20)
stxv 24, 160(20)
stxv 25, 176(20)
stxv 26, 192(20)
stxv 27, 208(20)
stxv 28, 224(20)
stxv 29, 240(20)
ld 14, 32(1)
ld 15, 40(1)
ld 16, 48(1)
ld 17, 56(1)
ld 18, 64(1)
ld 19, 72(1)
ld 20, 80(1)
ld 21, 88(1)
ld 22, 96(1)
ld 23, 104(1)
ld 24, 112(1)
ld 25, 120(1)
ld 26, 128(1)
ld 27, 136(1)
lfd 14, 144(1)
lfd 15, 152(1)
lfd 16, 160(1)
lfd 17, 168(1)
lfd 18, 176(1)
lfd 19, 184(1)
lfd 20, 192(1)
lfd 21, 200(1)
lfd 22, 208(1)
lfd 23, 216(1)
lfd 24, 224(1)
lfd 25, 232(1)
lfd 26, 240(1)
lfd 27, 248(1)
lfd 28, 256(1)
lfd 29, 264(1)
lfd 30, 272(1)
lfd 31, 280(1)
lxv 52, 288(1)
lxv 53, 304(1)
lxv 54, 320(1)
lxv 55, 336(1)
lxv 56, 352(1)
lxv 57, 368(1)
lxv 58, 384(1)
lxv 59, 400(1)
lxv 60, 416(1)
lxv 61, 432(1)
lxv 62, 448(1)
lxv 63, 464(1)
addi 1, 1, 512
blr

.purgem BIAS
.purgem CHUNK0
.purgem CHUNK
.purgem EPI
.purgem FASTBLK
