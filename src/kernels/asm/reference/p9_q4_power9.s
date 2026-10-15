# reference/p9_q4_power9.s: kernel.
# Frozen Clang20 POWER9 schedules. Numeric labels are function-local.
# Router TOC setup omitted: the leaf body has no TOC references or calls.
lxv 34, 0(3)
xxspltib 0, 15
xxspltib 36, 4
lxv 37, 0(4)
li 3, 0
li 5, 4
li 7, 12
xxland 35, 34, 0
vsrb 2, 2, 4
xxlxor 36, 36, 36
vmsummbm 3, 5, 3, 4
lxv 37, 16(4)
lwz 4, 32(4)
slwi 4, 4, 3
vmsummbm 2, 5, 2, 4
vadduwm 2, 2, 3
vextuwrx 3, 3, 2
vextuwrx 5, 5, 2
mfvsrwz 6, 34
vextuwrx 7, 7, 2
add 3, 3, 5
add 3, 3, 6
add 3, 3, 7
sub	3, 3, 4
extsw 3, 3
blr

