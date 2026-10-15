# reference/p9_ref_matvec_multi_power9.s: kernel.
# Frozen Clang20 POWER9 schedules. Numeric labels are function-local.
# Router TOC setup omitted: the leaf body has no TOC references or calls.
mfcr 12
stw 12, 8(1)
stdu 1, -512(1)
cmpldi	6, 0
std 14, 336(1)
std 15, 344(1)
std 16, 352(1)
std 17, 360(1)
std 18, 368(1)
std 19, 376(1)
std 20, 384(1)
std 21, 392(1)
std 22, 400(1)
std 23, 408(1)
std 24, 416(1)
std 25, 424(1)
std 26, 432(1)
std 27, 440(1)
std 28, 448(1)
std 29, 456(1)
std 30, 464(1)
std 31, 472(1)
stfd 28, 480(1)
stfd 29, 488(1)
stfd 30, 496(1)
stfd 31, 504(1)
std 2, 328(1)
std 5, 128(1)
std 6, 224(1)
beq	0, 231f
cmpldi	7, 8
mr	31, 4
rldicr 4, 8, 0, 60
mr	20, 10
sldi 27, 8, 2
addi 23, 10, 16
addi 18, 10, 24
addi 19, 10, 32
sldi 6, 7, 2
addi 17, 10, 40
addi 26, 10, 48
addi 25, 10, 56
cmpdi 1, 8, 0
cmpldi 6, 8, 8
sldi 0, 8, 5
li 21, 0
mfocrf 5, 128
addi 4, 4, -8
addi 30, 20, -64
add 12, 9, 6
stw 5, 180(1)
addi 6, 6, -4
andi. 11, 8, 7
li 15, 0
li 24, 0
mulli 5, 8, 12
rldicl 4, 4, 61, 3
addi 22, 4, 1
mulli 4, 8, 28
std 30, 112(1)
addi 30, 27, -4
std 30, 104(1)
mulli 30, 8, 20
std 6, 72(1)
ld 6, 128(1)
sldi 16, 8, 3
std 8, 192(1)
std 31, 184(1)
std 23, 168(1)
std 18, 160(1)
std 17, 152(1)
std 16, 144(1)
std 5, 208(1)
sldi 5, 8, 4
std 5, 120(1)
addi 5, 31, 16
std 4, 48(1)
mulli 4, 8, 24
std 30, 80(1)
std 5, 96(1)
addi 5, 10, 256
std 5, 320(1)
addi 5, 3, 16
std 4, 56(1)
ld 4, 224(1)
std 5, 88(1)
mulli 5, 7, 28
add 5, 9, 5
std 4, 312(1)
sldi 4, 7, 5
std 4, 200(1)
std 5, 304(1)
mulli 5, 7, 24
add 5, 9, 5
std 5, 296(1)
mulli 5, 7, 20
add 5, 9, 5
std 5, 288(1)
sldi 5, 7, 4
add 5, 9, 5
std 5, 280(1)
mulli 5, 7, 12
add 5, 9, 5
std 5, 272(1)
sldi 5, 7, 3
add 10, 9, 5
addi 5, 5, -4
std 5, 64(1)
mr	5, 9
b 201f
.p2align	4
200:
ld 4, 312(1)
ld 8, 200(1)
ld 10, 256(1)
ld 12, 248(1)
addi 24, 24, 8
addi 4, 4, -8
ld 5, 240(1)
ld 6, 232(1)
ld 15, 264(1)
add 10, 10, 8
add 12, 12, 8
add 5, 5, 8
ld 31, 184(1)
ld 23, 168(1)
ld 18, 160(1)
ld 17, 152(1)
ld 16, 144(1)
add 6, 6, 0
addi 15, 15, 1
std 4, 312(1)
ld 4, 304(1)
add 4, 4, 8
std 4, 304(1)
ld 4, 296(1)
add 4, 4, 8
std 4, 296(1)
ld 4, 288(1)
add 4, 4, 8
std 4, 288(1)
ld 4, 280(1)
add 4, 4, 8
std 4, 280(1)
ld 4, 272(1)
add 4, 4, 8
ld 8, 192(1)
std 4, 272(1)
ld 4, 224(1)
cmpld 5, 24, 4
bge 5, 231f
201:
std 6, 232(1)
std 5, 240(1)
std 12, 248(1)
std 10, 256(1)
bc 12, 6, 212f
ld 4, 104(1)
ld 6, 128(1)
maddld 4, 0, 15, 4
add 5, 6, 4
maddld 4, 0, 15, 6
ld 6, 224(1)
cmpd 5, 6, 24
addi 29, 4, -4
sub	4, 6, 24
cmpldi 4, 4, 4
cmpldi 7, 4, 1
cmpldi 2, 4, 2
cmpldi 3, 4, 3
mfocrf 6, 8
cmpldi 4, 4, 5
rotlwi	6, 6, 16
stw 6, 264(1)
mfocrf 6, 8
cmpldi 4, 4, 6
rotlwi	6, 6, 16
stw 6, 220(1)
mfocrf 6, 8
cmpldi 4, 4, 7
mfocrf 4, 8
rotlwi	6, 6, 16
rotlwi	4, 4, 16
stw 6, 140(1)
li 6, 48
stw 4, 136(1)
li 4, 0
mtctr 8
ld 10, 112(1)
.p2align	5
202:
mr	12, 10
addi 10, 10, 64
mr	28, 5
addi 5, 5, 4
mr	30, 29
addi 29, 29, 4
bc 4, 22, 203f
std 21, 64(12)
b 205f
.p2align	5
203:
lfs 0, 4(30)
stfd 0, 64(12)
bc 4, 29, 205f
lfs 0, 4(28)
stfd 0, 72(12)
bc 4, 9, 211f
lfsx 0, 29, 16
addi 30, 6, -48
stfd 0, 80(12)
bc 4, 13, 207f
ld 28, 208(1)
lfsx 0, 29, 28
stfd 0, 88(12)
lwz 28, 264(1)
rotlwi	28, 28, 16
mtocrf 8, 28
bc 4, 17, 208f
ld 28, 120(1)
lfsx 0, 29, 28
stfd 0, 96(12)
lwz 28, 220(1)
rotlwi	28, 28, 16
mtocrf 8, 28
bc 4, 17, 209f
ld 28, 80(1)
lfsx 0, 29, 28
stfd 0, 104(12)
lwz 28, 140(1)
rotlwi	28, 28, 16
mtocrf 8, 28
bc 4, 17, 210f
ld 28, 56(1)
lfsx 0, 29, 28
stfd 0, 112(12)
xxlxor 0, 0, 0
lwz 12, 136(1)
rotlwi	12, 12, 16
mtocrf 8, 12
bc 4, 17, 204f
ld 12, 48(1)
lfsx 0, 29, 12
.p2align	5
204:
addi 4, 4, 1
addi 6, 6, 64
stfdx 0, 25, 30
bdnz 202b
b 212f
.p2align	5
205:
addi 30, 6, -48
std 21, 72(12)
206:
stdx 21, 23, 30
207:
stdx 21, 18, 30
208:
stdx 21, 19, 30
209:
stdx 21, 17, 30
210:
stdx 21, 26, 30
xxlxor 0, 0, 0
addi 4, 4, 1
addi 6, 6, 64
stfdx 0, 25, 30
bdnz 202b
b 212f
211:
sldi 30, 4, 6
b 206b
.p2align	4
212:
std 15, 264(1)
li 2, 0
lwz 4, 180(1)
rotlwi	4, 4, 12
mtocrf 4, 4
bc 4, 20, 222f
213:
cmpld 5, 2, 7
bge 5, 200b
bc 12, 6, 228f
ld 5, 312(1)
li 4, 8
ld 6, 88(1)
maddld 23, 27, 2, 3
li 16, 0
cmpldi 5, 5, 8
maddld 29, 27, 2, 6
isel 4, 5, 4, 20
li 5, 1
cmpldi 5, 4, 1
isel 5, 4, 5, 21
mulld 4, 27, 2
addi 18, 4, -32
addi 17, 4, -4
b 215f
.p2align	4
214:
addi 2, 2, 1
add 29, 29, 27
add 23, 23, 27
addi 16, 16, 1
cmpld 5, 2, 7
beq	5, 200b
215:
maddld 4, 27, 16, 17
mr	10, 20
li 6, 0
add 15, 3, 4
maddld 4, 27, 16, 18
add 28, 3, 4
sldi 4, 2, 2
add 31, 9, 4
ld 4, 320(1)
b 217f
.p2align	4
216:
add 8, 6, 24
xsrsp 0, 0
addi 6, 6, 1
addi 4, 4, 8
addi 10, 10, 8
mulld 8, 8, 7
cmpld 5, 6, 5
sldi 8, 8, 2
stfsx 0, 31, 8
beq	5, 214b
217:
sldi 8, 6, 3
add 8, 8, 20
bc 4, 24, 218f
xxlxor 0, 0, 0
li 12, 0
b 220f
.p2align	4
218:
mtctr 22
addi 30, 8, -512
li 12, 0
mr	14, 28
xxlxor 0, 0, 0
.p2align	5
219:
lfsu 1, 32(14)
addi 12, 12, 8
lfdu 2, 512(30)
xsmaddadp 0, 1, 2
lfd 2, 64(30)
lfs 1, 4(14)
lfs 3, 8(14)
xsmaddadp 0, 1, 2
lfd 1, 128(30)
lfd 2, 192(30)
xsmaddadp 0, 3, 1
lfs 1, 12(14)
lfs 3, 16(14)
xsmaddadp 0, 1, 2
lfd 1, 256(30)
lfd 2, 320(30)
xsmaddadp 0, 3, 1
lfs 1, 20(14)
lfs 3, 24(14)
xsmaddadp 0, 1, 2
lfd 1, 384(30)
lfd 2, 448(30)
xsmaddadp 0, 3, 1
lfs 1, 28(14)
xsmaddadp 0, 1, 2
bdnz 219b
220:
bc 12, 2, 216b
addi 30, 8, -64
sldi 14, 12, 6
sldi 8, 12, 2
add 8, 15, 8
mtctr 11
add 12, 30, 14
.p2align	5
221:
lfsu 2, 4(8)
lfdu 1, 64(12)
xsmaddadp 0, 2, 1
bdnz 221b
b 216b
.p2align	4
222:
ori 4, 24, 1
ld 28, 312(1)
mulld 5, 24, 7
li 23, 0
cmpdi 5, 28, 5
mulld 6, 4, 7
ori 4, 24, 2
mulld 17, 4, 7
ori 4, 24, 3
cmpldi 7, 28, 2
cmpdi 2, 28, 2
cmpdi 3, 28, 3
cmpdi 4, 28, 4
sldi 5, 5, 2
mulld 10, 4, 7
ori 4, 24, 4
sldi 18, 6, 2
mulld 12, 4, 7
mfocrf 4, 4
cmpdi 5, 28, 6
sldi 17, 17, 2
rotlwi	4, 4, 20
sldi 6, 10, 2
stw 4, 220(1)
ori 4, 24, 5
sldi 10, 12, 2
mulld 30, 4, 7
mfocrf 4, 4
cmpdi 5, 28, 7
rotlwi	4, 4, 20
stw 4, 140(1)
ori 4, 24, 6
sldi 12, 30, 2
mulld 29, 4, 7
mfocrf 4, 4
rotlwi	4, 4, 20
stw 4, 136(1)
ori 4, 24, 7
ld 16, 96(1)
sldi 30, 29, 2
mulld 28, 4, 7
li 4, 8
sldi 28, 28, 2
b 224f
.p2align	4
223:
addi 4, 2, 8
add 16, 16, 0
addi 23, 23, 1
cmpld 5, 4, 7
bgt	5, 213b
224:
mr	15, 2
mr	2, 4
xxlxor 0, 0, 0
xxlxor 6, 6, 6
xxlxor 34, 34, 34
xxlxor 40, 40, 40
xxlxor 1, 1, 1
xxlxor 8, 8, 8
xxlxor 36, 36, 36
xxlxor 42, 42, 42
xxlxor 2, 2, 2
xxlxor 10, 10, 10
bc 12, 6, 226f
maddld 4, 0, 23, 31
mtctr 8
xxlxor 32, 32, 32
mr	29, 19
addi 14, 4, -32
xxlxor 44, 44, 44
xxlxor 3, 3, 3
xxlxor 11, 11, 11
xxlxor 33, 33, 33
xxlxor 45, 45, 45
xxlxor 12, 12, 12
xxlxor 38, 38, 38
xxlxor 46, 46, 46
xxlxor 4, 4, 4
xxlxor 13, 13, 13
xxlxor 39, 39, 39
xxlxor 47, 47, 47
xxlxor 7, 7, 7
xxlxor 35, 35, 35
xxlxor 41, 41, 41
xxlxor 48, 48, 48
xxlxor 9, 9, 9
xxlxor 37, 37, 37
xxlxor 43, 43, 43
xxlxor 49, 49, 49
xxlxor 5, 5, 5
.p2align	5
225:
lfsu 31, 32(14)
lxv 50, -32(29)
lxv 30, 0(29)
lxv 29, 16(29)
xxspltd	51, 31, 0
lxv 31, -16(29)
lfs 28, 8(14)
addi 29, 29, 64
xvmaddadp 49, 51, 50
xvmaddadp 37, 51, 30
xvmaddadp 9, 51, 29
xvmaddadp 43, 51, 31
lxssp 19, 4(14)
xxspltd	51, 51, 0
xvmaddadp 48, 51, 50
xvmaddadp 41, 51, 31
xvmaddadp 35, 51, 30
xvmaddadp 7, 51, 29
xxspltd	51, 28, 0
lfs 28, 16(14)
xvmaddadp 47, 51, 50
xvmaddadp 39, 51, 31
xvmaddadp 13, 51, 30
xvmaddadp 4, 51, 29
lxssp 19, 12(14)
xxspltd	51, 51, 0
xvmaddadp 46, 51, 50
xvmaddadp 38, 51, 31
xvmaddadp 12, 51, 30
xvmaddadp 5, 51, 29
xxspltd	51, 28, 0
lfs 28, 24(14)
xvmaddadp 45, 51, 50
xvmaddadp 33, 51, 31
xvmaddadp 11, 51, 30
xvmaddadp 3, 51, 29
lxssp 19, 20(14)
xxspltd	51, 51, 0
xvmaddadp 44, 51, 50
xvmaddadp 32, 51, 31
xvmaddadp 10, 51, 30
xvmaddadp 2, 51, 29
xxspltd	51, 28, 0
xvmaddadp 42, 51, 50
xvmaddadp 36, 51, 31
xvmaddadp 8, 51, 30
xvmaddadp 1, 51, 29
lxssp 19, 28(14)
xxspltd	51, 51, 0
xvmaddadp 40, 51, 50
xvmaddadp 34, 51, 31
xvmaddadp 6, 51, 30
xvmaddadp 0, 51, 29
bdnz 225b
b 227f
.p2align	4
226:
xxlxor 32, 32, 32
xxlxor 44, 44, 44
xxlxor 3, 3, 3
xxlxor 11, 11, 11
xxlxor 33, 33, 33
xxlxor 45, 45, 45
xxlxor 5, 5, 5
xxlxor 12, 12, 12
xxlxor 38, 38, 38
xxlxor 46, 46, 46
xxlxor 4, 4, 4
xxlxor 13, 13, 13
xxlxor 39, 39, 39
xxlxor 47, 47, 47
xxlxor 7, 7, 7
xxlxor 35, 35, 35
xxlxor 41, 41, 41
xxlxor 48, 48, 48
xxlxor 9, 9, 9
xxlxor 37, 37, 37
xxlxor 43, 43, 43
xxlxor 49, 49, 49
227:
xxswapd	50, 49
sldi 4, 15, 2
add 15, 9, 4
xsrsp 31, 50
xxswapd	50, 48
mr	4, 15
stfsux 31, 4, 5
xsrsp 50, 50
stxssp 18, 4(4)
xxswapd	50, 47
xsrsp 50, 50
stxssp 18, 8(4)
xxswapd	50, 46
xsrsp 50, 50
stxssp 18, 12(4)
xxswapd	50, 45
xsrsp 50, 50
stxssp 18, 16(4)
xxswapd	50, 44
xsrsp 50, 50
stxssp 18, 20(4)
xxswapd	50, 42
xsrsp 50, 50
stxssp 18, 24(4)
xxswapd	50, 40
xsrsp 50, 50
stxssp 18, 28(4)
bc 12, 28, 223b
xsrsp 31, 49
mr	4, 15
stfsux 31, 4, 18
xsrsp 48, 48
stxssp 16, 4(4)
xsrsp 47, 47
stxssp 15, 8(4)
xsrsp 46, 46
stxssp 14, 12(4)
xsrsp 45, 45
stxssp 13, 16(4)
xsrsp 44, 44
stxssp 12, 20(4)
xsrsp 42, 42
stxssp 10, 24(4)
xsrsp 40, 40
stxssp 8, 28(4)
bc 12, 10, 223b
xxswapd	40, 43
mr	4, 15
xsrsp 31, 40
xxswapd	40, 41
stfsux 31, 4, 17
xsrsp 40, 40
stxssp 8, 4(4)
xxswapd	40, 39
xsrsp 40, 40
stxssp 8, 8(4)
xxswapd	40, 38
xsrsp 40, 40
stxssp 8, 12(4)
xxswapd	40, 33
xsrsp 40, 40
stxssp 8, 16(4)
xxswapd	40, 32
xsrsp 40, 40
stxssp 8, 20(4)
xxswapd	40, 36
xsrsp 40, 40
stxssp 8, 24(4)
xxswapd	40, 34
xsrsp 40, 40
stxssp 8, 28(4)
bc 12, 14, 223b
xsrsp 31, 43
mr	4, 15
stfsux 31, 4, 6
xsrsp 40, 41
stxssp 8, 4(4)
xsrsp 39, 39
stxssp 7, 8(4)
xsrsp 38, 38
stxssp 6, 12(4)
xsrsp 33, 33
stxssp 1, 16(4)
xsrsp 32, 32
stxssp 0, 20(4)
xsrsp 36, 36
stxssp 4, 24(4)
xsrsp 34, 34
stxssp 2, 28(4)
bc 12, 18, 223b
xxswapd	34, 37
mr	4, 15
xsrsp 31, 34
xxswapd	34, 35
stfsux 31, 4, 10
xsrsp 34, 34
stxssp 2, 4(4)
xxswapd	34, 13
xsrsp 34, 34
stxssp 2, 8(4)
xxswapd	34, 12
xsrsp 34, 34
stxssp 2, 12(4)
xxswapd	34, 11
xsrsp 34, 34
stxssp 2, 16(4)
xxswapd	34, 10
xsrsp 34, 34
stxssp 2, 20(4)
xxswapd	34, 8
xsrsp 34, 34
stxssp 2, 24(4)
xxswapd	34, 6
xsrsp 34, 34
stxssp 2, 28(4)
lwz 4, 220(1)
rotlwi	4, 4, 12
mtocrf 4, 4
bc 12, 22, 223b
xsrsp 31, 37
mr	4, 15
stfsux 31, 4, 12
xsrsp 34, 35
stxssp 2, 4(4)
xsrsp 13, 13
stfs 13, 8(4)
xsrsp 12, 12
stfs 12, 12(4)
xsrsp 11, 11
stfs 11, 16(4)
xsrsp 10, 10
stfs 10, 20(4)
xsrsp 8, 8
stfs 8, 24(4)
xsrsp 6, 6
stfs 6, 28(4)
lwz 4, 140(1)
rotlwi	4, 4, 12
mtocrf 4, 4
bc 12, 22, 223b
xxswapd	6, 9
mr	4, 15
xsrsp 6, 6
stfsux 6, 4, 30
xxswapd	6, 7
xsrsp 6, 6
stfs 6, 4(4)
xxswapd	6, 4
xsrsp 6, 6
stfs 6, 8(4)
xxswapd	6, 5
xsrsp 6, 6
stfs 6, 12(4)
xxswapd	6, 3
xsrsp 6, 6
stfs 6, 16(4)
xxswapd	6, 2
xsrsp 6, 6
stfs 6, 20(4)
xxswapd	6, 1
xsrsp 6, 6
stfs 6, 24(4)
xxswapd	6, 0
xsrsp 6, 6
stfs 6, 28(4)
lwz 4, 136(1)
rotlwi	4, 4, 12
mtocrf 4, 4
bc 12, 22, 223b
xsrsp 6, 9
stfsux 6, 15, 28
xsrsp 6, 7
stfs 6, 4(15)
xsrsp 4, 4
stfs 4, 8(15)
xsrsp 4, 5
stfs 4, 12(15)
xsrsp 3, 3
stfs 3, 16(15)
xsrsp 2, 2
stfs 2, 20(15)
xsrsp 1, 1
stfs 1, 24(15)
xsrsp 0, 0
stfs 0, 28(15)
b 223b
228:
ld 4, 64(1)
ld 5, 264(1)
ld 10, 200(1)
maddld 4, 10, 5, 4
add 6, 9, 4
ld 4, 72(1)
maddld 4, 10, 5, 4
add 8, 9, 4
maddld 4, 10, 5, 9
addi 5, 4, -4
ld 4, 312(1)
cmpdi 4, 4, 5
cmpldi 5, 4, 2
cmpdi 7, 4, 2
cmpdi 2, 4, 3
cmpdi 3, 4, 4
mfocrf 10, 8
cmpdi 4, 4, 6
rotlwi	10, 10, 16
stw 10, 220(1)
mfocrf 10, 8
cmpdi 4, 4, 7
mfocrf 4, 8
rotlwi	10, 10, 16
rotlwi	4, 4, 16
stw 10, 140(1)
stw 4, 136(1)
sldi 4, 2, 2
add 23, 6, 4
sub	6, 7, 2
add 29, 8, 4
add 5, 5, 4
mtctr 6
ld 6, 272(1)
ld 8, 280(1)
ld 10, 288(1)
ld 12, 296(1)
ld 30, 304(1)
b 230f
.p2align	5
229:
addi 30, 30, 4
addi 12, 12, 4
addi 10, 10, 4
addi 8, 8, 4
addi 6, 6, 4
bdz 200b
230:
mr	28, 23
addi 23, 23, 4
mr	18, 29
addi 29, 29, 4
stwu 21, 4(5)
bc 12, 20, 229b
stw 21, 4(18)
bc 12, 30, 229b
stw 21, 4(28)
bc 12, 10, 229b
stwx 21, 6, 4
bc 12, 14, 229b
stwx 21, 8, 4
lwz 28, 220(1)
rotlwi	28, 28, 16
mtocrf 8, 28
bc 12, 18, 229b
stwx 21, 10, 4
lwz 28, 140(1)
rotlwi	28, 28, 16
mtocrf 8, 28
bc 12, 18, 229b
stwx 21, 12, 4
lwz 28, 136(1)
rotlwi	28, 28, 16
mtocrf 8, 28
bc 12, 18, 229b
stwx 21, 30, 4
b 229b
231:
ld 2, 328(1)
lfd 31, 504(1)
lfd 30, 496(1)
ld 31, 472(1)
ld 30, 464(1)
ld 29, 456(1)
ld 28, 448(1)
ld 27, 440(1)
ld 26, 432(1)
lfd 29, 488(1)
ld 25, 424(1)
ld 24, 416(1)
ld 23, 408(1)
lfd 28, 480(1)
ld 22, 400(1)
ld 21, 392(1)
ld 20, 384(1)
ld 19, 376(1)
ld 18, 368(1)
ld 17, 360(1)
ld 16, 352(1)
ld 15, 344(1)
ld 14, 336(1)
addi 1, 1, 512
lwz 12, 8(1)
mtocrf 32, 12
mtocrf 16, 12
mtocrf 8, 12
blr

