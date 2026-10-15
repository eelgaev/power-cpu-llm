# pq8_q4_0_shared_tile4x2_power9: experimental Q4_0 shared-scale tile, bit-identical
# to the portable Rust reference (pq8_shared_project_rows). Four row groups x
# two logical positions, with exact integer high/low dots and one float tail:
#   T = 256*high + low;  acc = fmaf(float(T), weight_d * fine_d, acc)
#
# Hand-written assembly, originally emitted by the former schedule generator
# tools/codegen/gen-q4-0-shared-tile.py; this file is canonical; the Rust module includes
# it directly (edit it, then cargo build).
#
# Args (r3 -> 8-byte argument fields):
#   w[4]:0, moff[4]:32, high[2]:64, low[2]:80, high_sums[2]:96,
#   low_sums[2]:112, fine_scales[2]:128, nb:144, record_stride:152,
#   output:160.
# Registers: v0..v7 high, v8..v15 low, v16..v17 activations, v18 shift 4,
#   v19 shift 8, v20..v23 low-nibble weights, v24..v27 high-nibble weights,
#   v28..v29 bias starts. vs0..vs7 F32 outputs; vs8..vs29 float tail;
#   vs31 nibble mask.
# High/low products are interleaved in groups of eight to hide dependency
# latency. Eight logical positions per outer block touch sixteen physical
# positions, keeping the activation working set equal to the existing tile.
# The float tail is split into separate conversion, product and FMA waves: no
# immediately dependent float instructions; the arithmetic of each output is
# unchanged.
# Frame 512: nonvolatile r14-r25, f14-f31, v20-v31.
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
ld 5, 8(3)
ld 6, 16(3)
ld 7, 24(3)
ld 8, 32(3)
ld 9, 40(3)
ld 10, 48(3)
ld 11, 56(3)
ld 12, 64(3)
ld 14, 72(3)
ld 15, 80(3)
ld 16, 88(3)
ld 17, 96(3)
ld 18, 104(3)
ld 19, 112(3)
ld 20, 120(3)
ld 21, 128(3)
ld 22, 136(3)
ld 23, 144(3)
ld 24, 152(3)
ld 25, 160(3)
mtctr 23
xxspltib 31, 15
vspltisb 18, 4
vspltisw 19, 8
xxlxor 0, 0, 0
xxlxor 1, 1, 1
xxlxor 2, 2, 2
xxlxor 3, 3, 3
xxlxor 4, 4, 4
xxlxor 5, 5, 5
xxlxor 6, 6, 6
xxlxor 7, 7, 7
2:
lha 0, 0(17)
slwi 0, 0, 3
neg 0, 0
mtvsrws 60, 0
lha 0, 0(18)
slwi 0, 0, 3
neg 0, 0
mtvsrws 61, 0
lxv 56, 0(4)
xxland 52, 56, 31
vsrb 24, 24, 18
lxv 57, 0(5)
xxland 53, 57, 31
vsrb 25, 25, 18
lxv 58, 0(6)
xxland 54, 58, 31
vsrb 26, 26, 18
lxv 59, 0(7)
xxland 55, 59, 31
vsrb 27, 27, 18
lxv 48, 0(12)
lxv 49, 0(14)
vmsummbm 0, 16, 20, 28
vmsummbm 1, 17, 20, 29
vmsummbm 2, 16, 21, 28
vmsummbm 3, 17, 21, 29
vmsummbm 4, 16, 22, 28
vmsummbm 5, 17, 22, 29
vmsummbm 6, 16, 23, 28
vmsummbm 7, 17, 23, 29
lha 0, 0(19)
slwi 0, 0, 3
neg 0, 0
mtvsrws 60, 0
lha 0, 0(20)
slwi 0, 0, 3
neg 0, 0
mtvsrws 61, 0
lxv 48, 0(15)
lxv 49, 0(16)
vmsummbm 8, 16, 20, 28
vmsummbm 9, 17, 20, 29
vmsummbm 10, 16, 21, 28
vmsummbm 11, 17, 21, 29
vmsummbm 12, 16, 22, 28
vmsummbm 13, 17, 22, 29
vmsummbm 14, 16, 23, 28
vmsummbm 15, 17, 23, 29
lxv 48, 16(12)
lxv 49, 16(14)
vmsummbm 0, 16, 24, 0
vmsummbm 1, 17, 24, 1
vmsummbm 2, 16, 25, 2
vmsummbm 3, 17, 25, 3
vmsummbm 4, 16, 26, 4
vmsummbm 5, 17, 26, 5
vmsummbm 6, 16, 27, 6
vmsummbm 7, 17, 27, 7
lxv 48, 16(15)
lxv 49, 16(16)
vmsummbm 8, 16, 24, 8
vmsummbm 9, 17, 24, 9
vmsummbm 10, 16, 25, 10
vmsummbm 11, 17, 25, 11
vmsummbm 12, 16, 26, 12
vmsummbm 13, 17, 26, 13
vmsummbm 14, 16, 27, 14
vmsummbm 15, 17, 27, 15
lxv 56, 16(4)
xxland 52, 56, 31
vsrb 24, 24, 18
lxv 57, 16(5)
xxland 53, 57, 31
vsrb 25, 25, 18
lxv 58, 16(6)
xxland 54, 58, 31
vsrb 26, 26, 18
lxv 59, 16(7)
xxland 55, 59, 31
vsrb 27, 27, 18
lxv 48, 32(12)
lxv 49, 32(14)
vmsummbm 0, 16, 20, 0
vmsummbm 1, 17, 20, 1
vmsummbm 2, 16, 21, 2
vmsummbm 3, 17, 21, 3
vmsummbm 4, 16, 22, 4
vmsummbm 5, 17, 22, 5
vmsummbm 6, 16, 23, 6
vmsummbm 7, 17, 23, 7
lxv 48, 32(15)
lxv 49, 32(16)
vmsummbm 8, 16, 20, 8
vmsummbm 9, 17, 20, 9
vmsummbm 10, 16, 21, 10
vmsummbm 11, 17, 21, 11
vmsummbm 12, 16, 22, 12
vmsummbm 13, 17, 22, 13
vmsummbm 14, 16, 23, 14
vmsummbm 15, 17, 23, 15
lxv 48, 48(12)
lxv 49, 48(14)
vmsummbm 0, 16, 24, 0
vmsummbm 1, 17, 24, 1
vmsummbm 2, 16, 25, 2
vmsummbm 3, 17, 25, 3
vmsummbm 4, 16, 26, 4
vmsummbm 5, 17, 26, 5
vmsummbm 6, 16, 27, 6
vmsummbm 7, 17, 27, 7
lxv 48, 48(15)
lxv 49, 48(16)
vmsummbm 8, 16, 24, 8
vmsummbm 9, 17, 24, 9
vmsummbm 10, 16, 25, 10
vmsummbm 11, 17, 25, 11
vmsummbm 12, 16, 26, 12
vmsummbm 13, 17, 26, 13
vmsummbm 14, 16, 27, 14
vmsummbm 15, 17, 27, 15
lxv 56, 32(4)
xxland 52, 56, 31
vsrb 24, 24, 18
lxv 57, 32(5)
xxland 53, 57, 31
vsrb 25, 25, 18
lxv 58, 32(6)
xxland 54, 58, 31
vsrb 26, 26, 18
lxv 59, 32(7)
xxland 55, 59, 31
vsrb 27, 27, 18
lxv 48, 64(12)
lxv 49, 64(14)
vmsummbm 0, 16, 20, 0
vmsummbm 1, 17, 20, 1
vmsummbm 2, 16, 21, 2
vmsummbm 3, 17, 21, 3
vmsummbm 4, 16, 22, 4
vmsummbm 5, 17, 22, 5
vmsummbm 6, 16, 23, 6
vmsummbm 7, 17, 23, 7
lxv 48, 64(15)
lxv 49, 64(16)
vmsummbm 8, 16, 20, 8
vmsummbm 9, 17, 20, 9
vmsummbm 10, 16, 21, 10
vmsummbm 11, 17, 21, 11
vmsummbm 12, 16, 22, 12
vmsummbm 13, 17, 22, 13
vmsummbm 14, 16, 23, 14
vmsummbm 15, 17, 23, 15
lxv 48, 80(12)
lxv 49, 80(14)
vmsummbm 0, 16, 24, 0
vmsummbm 1, 17, 24, 1
vmsummbm 2, 16, 25, 2
vmsummbm 3, 17, 25, 3
vmsummbm 4, 16, 26, 4
vmsummbm 5, 17, 26, 5
vmsummbm 6, 16, 27, 6
vmsummbm 7, 17, 27, 7
lxv 48, 80(15)
lxv 49, 80(16)
vmsummbm 8, 16, 24, 8
vmsummbm 9, 17, 24, 9
vmsummbm 10, 16, 25, 10
vmsummbm 11, 17, 25, 11
vmsummbm 12, 16, 26, 12
vmsummbm 13, 17, 26, 13
vmsummbm 14, 16, 27, 14
vmsummbm 15, 17, 27, 15
lxv 56, 48(4)
xxland 52, 56, 31
vsrb 24, 24, 18
lxv 57, 48(5)
xxland 53, 57, 31
vsrb 25, 25, 18
lxv 58, 48(6)
xxland 54, 58, 31
vsrb 26, 26, 18
lxv 59, 48(7)
xxland 55, 59, 31
vsrb 27, 27, 18
lxv 48, 96(12)
lxv 49, 96(14)
vmsummbm 0, 16, 20, 0
vmsummbm 1, 17, 20, 1
vmsummbm 2, 16, 21, 2
vmsummbm 3, 17, 21, 3
vmsummbm 4, 16, 22, 4
vmsummbm 5, 17, 22, 5
vmsummbm 6, 16, 23, 6
vmsummbm 7, 17, 23, 7
lxv 48, 96(15)
lxv 49, 96(16)
vmsummbm 8, 16, 20, 8
vmsummbm 9, 17, 20, 9
vmsummbm 10, 16, 21, 10
vmsummbm 11, 17, 21, 11
vmsummbm 12, 16, 22, 12
vmsummbm 13, 17, 22, 13
vmsummbm 14, 16, 23, 14
vmsummbm 15, 17, 23, 15
lxv 48, 112(12)
lxv 49, 112(14)
vmsummbm 0, 16, 24, 0
vmsummbm 1, 17, 24, 1
vmsummbm 2, 16, 25, 2
vmsummbm 3, 17, 25, 3
vmsummbm 4, 16, 26, 4
vmsummbm 5, 17, 26, 5
vmsummbm 6, 16, 27, 6
vmsummbm 7, 17, 27, 7
lxv 48, 112(15)
lxv 49, 112(16)
vmsummbm 8, 16, 24, 8
vmsummbm 9, 17, 24, 9
vmsummbm 10, 16, 25, 10
vmsummbm 11, 17, 25, 11
vmsummbm 12, 16, 26, 12
vmsummbm 13, 17, 26, 13
vmsummbm 14, 16, 27, 14
vmsummbm 15, 17, 27, 15
lxvwsx 20, 0, 21
lxvwsx 21, 0, 22
lxvx 16, 4, 8
lxvx 17, 5, 9
lxvx 18, 6, 10
lxvx 19, 7, 11
vslw 0, 0, 19
vadduwm 0, 0, 8
vslw 1, 1, 19
vadduwm 1, 1, 9
vslw 2, 2, 19
vadduwm 2, 2, 10
vslw 3, 3, 19
vadduwm 3, 3, 11
vslw 4, 4, 19
vadduwm 4, 4, 12
vslw 5, 5, 19
vadduwm 5, 5, 13
vslw 6, 6, 19
vadduwm 6, 6, 14
vslw 7, 7, 19
vadduwm 7, 7, 15
xvcvsxwsp 8, 32
xvcvsxwsp 9, 33
xvcvsxwsp 10, 34
xvcvsxwsp 11, 35
xvcvsxwsp 12, 36
xvcvsxwsp 13, 37
xvcvsxwsp 14, 38
xvcvsxwsp 15, 39
xvmulsp 22, 16, 20
xvmulsp 23, 16, 21
xvmulsp 24, 17, 20
xvmulsp 25, 17, 21
xvmulsp 26, 18, 20
xvmulsp 27, 18, 21
xvmulsp 28, 19, 20
xvmulsp 29, 19, 21
xvmaddasp 0, 8, 22
xvmaddasp 1, 9, 23
xvmaddasp 2, 10, 24
xvmaddasp 3, 11, 25
xvmaddasp 4, 12, 26
xvmaddasp 5, 13, 27
xvmaddasp 6, 14, 28
xvmaddasp 7, 15, 29
add 4, 4, 24
add 5, 5, 24
add 6, 6, 24
add 7, 7, 24
addi 12, 12, 128
addi 14, 14, 128
addi 15, 15, 128
addi 16, 16, 128
addi 17, 17, 8
addi 18, 18, 8
addi 19, 19, 8
addi 20, 20, 8
addi 21, 21, 16
addi 22, 22, 16
bdnz 2b
stxv 0, 0(25)
stxv 1, 16(25)
stxv 2, 32(25)
stxv 3, 48(25)
stxv 4, 64(25)
stxv 5, 80(25)
stxv 6, 96(25)
stxv 7, 112(25)
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
