# reference/p9_slice_probe.s: kernel.
# Frozen Clang20 POWER9 schedules. Numeric labels are function-local.
# Router TOC setup omitted: the leaf body has no TOC references or calls.
cmpldi	3, 0
beq	0, 201f
mtctr 3
xxspltib 34, 3
xxspltib 35, 5
vspltisw 5, 1
vspltisw 0, 2
vspltisw 1, 3
vspltisw 6, 4
vspltisw 7, 5
vspltisw 8, 6
vspltisw 9, 7
vspltisw 10, 8
vspltisw 11, 9
vspltisw 12, 10
vspltisw 13, 11
xxlxor 36, 36, 36
.p2align	4
200:
vmsummbm 4, 2, 3, 4
vmsummbm 5, 2, 3, 5
vmsummbm 0, 2, 3, 0
vmsummbm 1, 2, 3, 1
vmsummbm 6, 2, 3, 6
vmsummbm 7, 2, 3, 7
vmsummbm 8, 2, 3, 8
vmsummbm 9, 2, 3, 9
vmsummbm 10, 2, 3, 10
vmsummbm 11, 2, 3, 11
vmsummbm 12, 2, 3, 12
vmsummbm 13, 2, 3, 13
bdnz 200b
vadduwm 2, 5, 4
vadduwm 2, 0, 2
vadduwm 2, 1, 2
vadduwm 2, 6, 2
vadduwm 2, 7, 2
vadduwm 2, 8, 2
vadduwm 2, 9, 2
vadduwm 2, 10, 2
vadduwm 2, 11, 2
vadduwm 2, 12, 2
vadduwm 2, 13, 2
b 202f
201:
li 3, 264
blr
202:
li 3, 0
li 4, 4
li 6, 12
mfvsrwz 5, 34
vextuwrx 3, 3, 2
vextuwrx 4, 4, 2
vextuwrx 6, 6, 2
add 3, 3, 4
add 3, 3, 5
add 3, 3, 6
clrldi	3, 3, 32
blr
