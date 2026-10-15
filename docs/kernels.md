# Kernels: portable Rust and POWER9 assembly

There is no C in the build. Every kernel is either portable Rust (`src/kernels/portable/`) or POWER9 assembly embedded in Rust. The portable kernels are the numerical reference: each POWER9 kernel is tested against its portable twin bit for bit.

## Where the assembly comes from

Most of `src/kernels/asm/` is **frozen compiler output**: the kernels were first written in C, compiled with Clang 20 (`-O3 -mcpu=power9 -ffp-contract=off`), and the assembly was kept once a disassembly check showed every function instruction-identical to its C original and twin tests showed identical bits. The C was then deleted. These files are canonical now and can be edited like any other source.

The kernels where the schedule matters most are **hand-written**:

| File | What | Why |
|---|---|---|
| `q4_0/pq8_q4_0_tile4x4_power9.s` | Q4_0 prompt tile: 4 row groups (16 rows) × 4 positions | Spaces dependent `vmsummbm` products 16 apart, keeps 16 accumulators in registers, and has a 4-block fast loop for the common record size. Instruction count bounds it (about 0.6 cycle per vector operation), not latency. |
| `q4_0/pq8_q4_0_tile8x2_power9.s` | Q4_0 decode tile: 8 row groups × 2 positions (the two planes of one token) | One-token decode streams every weight once; sixteen low-nibble products precede the sixteen high-nibble ones, spacing dependent accumulators. |
| `q4_0/pq8_q4_0_shared_tile4x2_power9.s` | The experimental shared-scale tile | |
| `attention/pa_attention_f32_power9_heads.s` | Decode attention | Four key rows per pass over the query (16 independent FMA chains), four 16-float output tiles in flight for the weighted sum, and two query heads of one KV group per pass, so each key and value row is loaded once for both. |
| `int8/pq8_project_packed_power9.s` (epilogue) | Fast path for the output of a projection tile | Replaces the compiler's per-element epilogue. |

The Q4_0 tiles began as the output of Python schedule generators; the generators are gone and the `.s` files are the source. Every hand-written kernel performs the same operations in the same order as its portable twin; only the schedule differs.

## Layout

- `src/kernels/asm/<family>/<kernel>.s`: one file per kernel or static helper, in families `int8`, `attention`, `fma`, `dequant`, `reference` and `q4_0`.
- `src/kernels/{int8,attention,fma,dequant}_asm.rs` include their family's files with `include_str!` inside one `global_asm!` block. The include order is the code layout, so keep it when adding a file.
- `src/kernels/power9_asm.rs` and `quantized/tile_q4_0*.rs` wrap single kernels as `#[unsafe(naked)]` functions with `naked_asm!(include_str!(...))`.
- An edited `.s` takes effect on the next `cargo build`; nothing is generated.
- `src/kernels/portable/`: the reference kernels (`dequant.rs`, `matmul.rs`, `reference.rs`, `attention.rs`, `int8.rs`, `quant.rs`). They keep C ABI names (`#[unsafe(no_mangle)] extern "C"`) because the assembly calls some of them (`p9_decode_row`, `pq_row_bytes`, `pq8_input_bytes`).

## Assembly conventions

- ELFv2 ABI, little-endian POWER9: TOC setup and `.localentry` where constants are loaded, saves of nonvolatile registers (including the scalar halves of VSX registers), unwind directives, ordinary `bl`/`nop` calls to Rust (`power_expf` in the softmax) and libc (`memcpy`, `memset`, `malloc`, `free`, `aligned_alloc`).
- All `global_asm!` blocks share one object, so local labels and static helpers carry a per-file prefix (`.Lp9_<tag>_`, `p9_<tag>_`).
- Entry alignment (64 bytes, 128 for the finite scan) comes before the label, so no padding executes. Smaller alignment measurably slowed identical loops.
- Never reorder a floating-point accumulation and never change a tolerance. To change a kernel's arithmetic, change the portable reference too and keep the twin tests passing.

## Transcendentals (`src/libm`)

Results depend on the last bits of `exp`, `log`, `expf` and `sincosf` (softmax, sampler entropy, RoPE), and of `tanh`, `expm1` and `powf` (GELU, softcap, RoPE frequencies). Different libm builds round some inputs differently: a static link against another glibc changed 33 of the 112 full-canvas comparison blobs. `src/libm` therefore ports these functions to Rust, bit-identical to glibc 2.28 as shipped in RHEL 8 for POWER9:

- `exp`, `ln`: IBM Accurate Mathematical Library (`e_exp.c`, `e_log.c`);
- `expf`: the POWER8 assembly that glibc selects on POWER8 and later (`e_expf.S`);
- `sincosf`: generic `s_sincosf.c` (LLVM merges an `f32` `sin` and `cos` of one angle into this call);
- `pow`: IBM `e_pow.c` with `__exp1`; `powf`: generic `e_powf.c`;
- `tanh`, `expm1`: fdlibm; `fmod`: `e_fmod.c`, with NaN results built from bits because LLVM need not preserve NaN payloads.

That libm was compiled with floating-point contraction, so the operation order and every fused multiply-add follow its disassembly, not only its C source. The tables are generated from the glibc source by `tools/codegen/gen-libm-tables.py`, which also checks them byte for byte against the system library. All 2^32 inputs of `expf` and `sincosf`, and billions of samples of the others (random bit patterns, special values, subnormals and the ranges the engine uses), were identical to glibc. The tests keep digests of the system library's results; the full comparison is an ignored test that needs that library.

`libm::tanh_block` (`src/libm/tanh_batch.rs`) is the batched `tanh` behind GELU. It performs for every element exactly the operations of the scalar `tanh`, but first sorts the elements into classes with identical control flow and runs each class as straight-line code on four elements at a time (about 25 ns instead of 41 ns per GELU element). The ignored test `gelu_exhaustive` compares it with the scalar formula for all 2^32 binary32 inputs.

`clippy.toml` forbids the `std` versions of these functions. Two dependencies call libm themselves (tokio's statistics and minijinja's `**` call `pow`; float `%` calls `fmod`), so the executable defines `pow` and `fmod` (`src/main.rs`), forwarding to these ports. It links no C maths library, and a static build contains no glibc maths function.

## Tests

- POWER9 against portable, bit for bit: block dots, the exact router, the decoder for every format (all 65,536 binary16 Q4_0 scales), the finite scan (every NaN and infinity class), the exact FMA projections in every rounding mode, the int8 quantizers, packing and projections, and binary32 attention including head ranges and masks.
- The int8 error bound against exact dequantized arithmetic.
- With `--features reference-oracle`, the decoders against ggml's dequantizers.

For speed comparisons, alternate the two builds (A/B/B/A) on the same cores with nothing else running.
