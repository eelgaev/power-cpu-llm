# POWER9 INT8 LLM Inference: Decode and Prefill Analysis

> This analysis was written before the engine existed and motivated it. Its throughput figures are estimates from instruction rates, not measurements; measured results are in [benchmarks.md](benchmarks.md). The engine uses the `vmsummbm` mapping described here ([backends.md](backends.md)).

## Scope

This note summarizes the analysis of IBM POWER9 VSX/VMX integer instructions for CPU-only quantized LLM inference, with emphasis on Q8/Q4 decode and prefill for dense and MoE models.

## Key result

POWER9's old VMX `vmsummbm` instruction is a real and useful INT8 dot-product primitive:

- 16 byte products per instruction
- signed-byte × unsigned-byte inputs
- four 32-bit partial accumulators
- about 2 independent instructions/cycle/core on POWER9
- about 7-cycle latency

That gives a raw arithmetic roof of:

```text
32 INT8 MAC contributions / cycle / core
```

or 64 integer ops/cycle/core if a multiply and add are counted separately.

At 3.0 GHz:

```text
1 core   = 96 GMAC/s
20 cores = 1.92 TMAC/s
40 cores = 3.84 TMAC/s
```

## Q8 × Q8 mapping

Normal symmetric Q8 has signed weights and activations:

```text
w ∈ [-128,127]
a ∈ [-128,127]
```

`vmsummbm` wants signed × unsigned, so bias one operand:

```text
wu = w + 128
```

equivalently:

```text
wu = w ^ 0x80
```

Then:

```text
sum(a * w)
=
sum(a * wu) - 128 * sum(a)
```

The static weights can be repacked once at model load. The activation sum can be computed during quantization and reused across many output rows.

Conceptually:

```c
vi8 a0 = load_i8(activation + 0);
vi8 a1 = load_i8(activation + 16);

vu8 w0 = load_u8(repacked_weights + 0);
vu8 w1 = load_u8(repacked_weights + 16);

vi32 p0 = vec_msum(a0, w0, zero);
vi32 p1 = vec_msum(a1, w1, zero);

dot = hsum(p0 + p1) - 128 * activation_sum;
```

## Q4 × Q8 mapping

Q4_0 is especially convenient.

Stored nibble:

```text
q = 0..15
```

represents:

```text
q_signed = q - 8
```

Therefore:

```text
sum(a * (q - 8))
=
sum(a * q) - 8 * sum(a)
```

The nibble is already unsigned, exactly what `vmsummbm` wants.

A 32-weight block can look roughly like:

```text
load 16 packed Q4 bytes
AND 0x0f
shift right 4
2 × vmsummbm
apply -8 * activation_sum correction
```

So Q4 has a very plausible efficient POWER9 implementation.

## POWER9 integer vs FP32 arithmetic density

POWER9 FP32 VSX can sustain about:

```text
8 FP32 MACs/cycle/core
```

while packed-byte `vmsummbm` can contribute:

```text
32 INT8 MACs/cycle/core
```

So the packed integer path has roughly:

```text
4× arithmetic density
```

over FP32.

At 3 GHz / 20 cores:

```text
FP32 ≈ 0.48 TMAC/s ≈ 0.96 TFLOP/s
INT8 ≈ 1.92 TMAC/s ≈ 3.84 TOPS
```

## Decode

For batch-1 autoregressive decode, the model generally streams its weights once per generated token, so DRAM bandwidth dominates before the integer execution units are saturated.

Very rough optimized estimates for a ~4B model:

| Quant | 1 socket / 20 cores | 2 sockets / 40 cores |
|---|---:|---:|
| Q8 | ~20–29 tok/s | ~35–50 tok/s |
| Q4 | ~30–42 tok/s | ~50–75 tok/s |

Q4 wins decode because it reduces weight traffic.

## Prefill

Prefill is fundamentally different because many prompt tokens are processed together.

Large projection layers become GEMM rather than GEMV:

```text
weight matrix × many activation vectors
```

Weights can be reused across many prompt tokens, so arithmetic throughput matters much more.

For a dense ~4B model on one 20-core 3 GHz socket:

```text
1.92 TMAC/s / ~4B MAC/token
≈ 480 tok/s
```

is the absolute matrix-arithmetic roof.

A highly optimized real implementation might be in the rough range:

```text
~180–300 prompt tok/s
```

for sufficiently large prompt batches.

With two sockets and good NUMA placement:

```text
~300–500 prompt tok/s
```

is plausible as an aggressive target.

## Q8 vs Q4 during prefill

During decode:

```text
Q4 usually wins
```

because model bandwidth dominates.

During prefill, however:

- weights are reused
- memory traffic matters less
- Q4 still pays unpack/shift/mask overhead
- Q8 can feed `vmsummbm` almost directly

So a tuned POWER9 implementation could show:

```text
Q8 prefill >= Q4 prefill
```

even though:

```text
Q4 decode > Q8 decode
```

## Qwen3.6-35B-A3B

Approximate relevant characteristics:

```text
~35B total parameters
~3B active parameters/token
256 routed experts
top-8 routing
```

At 20 cores / 3 GHz:

```text
1.92 TMAC/s / 3B
≈ 640 tok/s
```

absolute arithmetic roof.

At 40 cores / 3 GHz:

```text
3.84 TMAC/s / 3B
≈ 1280 tok/s
```

absolute arithmetic roof.

Those are impossible application-level ceilings because they assume perfect utilization and no routing/attention/normalization/state-update overhead.

### MoE fragmentation

With 256 experts and top-8 routing:

```text
average assignments/expert = 8B / 256 = B / 32
```

where B is the prompt-token batch.

| Prompt batch | Avg. assignments/expert |
|---:|---:|
| 128 | 4 |
| 256 | 8 |
| 512 | 16 |
| 1024 | 32 |
| 2048 | 64 |
| 4096 | 128 |

This means small prompt batches produce skinny expert GEMMs.

A good implementation should:

```text
route tokens
→ group/sort by expert
→ run tiled POWER9 expert GEMM
→ merge outputs
```

Aggressive one-socket Q8 prefill estimates:

```text
pp128   ~70–140 tok/s
pp512   ~150–230 tok/s
pp2048  ~190–280 tok/s
```

## Gemma 4 26B-A4B

Approximate relevant characteristics:

```text
~25B total parameters
~3.8B active parameters/token
128 experts
top-8 routing
```

At 20 cores / 3 GHz:

```text
1.92 TMAC/s / 3.8B
≈ 505 tok/s
```

absolute arithmetic roof.

At 40 cores / 3 GHz:

```text
3.84 TMAC/s / 3.8B
≈ 1010 tok/s
```

absolute arithmetic roof.

Because Gemma has 128 experts rather than 256:

```text
average assignments/expert = 8B / 128 = B / 16
```

So at the same prompt size, Gemma gets about twice as many tokens per expert as Qwen3.6.

| Prompt batch | Gemma assignments/expert | Qwen3.6 assignments/expert |
|---:|---:|---:|
| 128 | 8 | 4 |
| 512 | 32 | 16 |
| 1024 | 64 | 32 |
| 2048 | 128 | 64 |

This can partially compensate for Gemma's larger active parameter count.

## Optimized prefill estimates

At roughly 3 GHz:

| Model | Quant | 1 socket | 2 sockets |
|---|---:|---:|---:|
| Dense ~4B | Q8 | ~180–300 tok/s | ~300–500 tok/s |
| Dense ~4B | Q4 | ~130–230 tok/s | ~220–400 tok/s |
| Qwen3.6-35B-A3B | Q8 | ~150–280 tok/s | ~260–480 tok/s |
| Qwen3.6-35B-A3B | Q4 | ~110–220 tok/s | ~190–380 tok/s |
| Gemma 4 26B-A4B | Q8 | ~140–250 tok/s | ~240–420 tok/s |
| Gemma 4 26B-A4B | Q4 | ~100–200 tok/s | ~180–340 tok/s |

These are engineering estimates assuming a custom POWER9 tiled GEMM, expert grouping, NUMA-aware scheduling, and large enough prompt batches.

## Can dual-socket POWER9 reach 1500+ prefill tok/s?

For Qwen3.6-A3B:

```text
1500 tok/s × 3B MAC/token
= 4.5 TMAC/s
```

A 40-core dual-socket POWER9 at 3 GHz has an absolute packed-byte roof of:

```text
3.84 TMAC/s
```

So 1500 tok/s is already above the raw execution ceiling before any attention, DeltaNet, routing, scaling, normalization, cache misses, or quantization overhead.

For Gemma 4 A4B:

```text
1500 × 3.8B
= 5.7 TMAC/s
```

which is even further above the machine's CPU-only arithmetic ceiling.

Therefore:

```text
1500+ CPU-only prompt tok/s
is not realistic
for either model
on 40 POWER9 cores.
```

## Using only 10 physical POWER9 cores

At 3 GHz:

```text
10 × 3e9 × 32
= 960 GMAC/s
= 0.96 TMAC/s
```

Absolute model-MAC ceilings:

### Qwen3.6-A3B

```text
0.96T / 3B
≈ 320 tok/s
```

### Gemma 4 A4B

```text
0.96T / 3.8B
≈ 253 tok/s
```

| Clock | 10-core INT8 roof | Qwen3.6 absolute roof | Gemma 4 absolute roof |
|---:|---:|---:|---:|
| 2.4 GHz | 0.768 TMAC/s | 256 tok/s | 202 tok/s |
| 3.0 GHz | 0.960 TMAC/s | 320 tok/s | 253 tok/s |
| 3.5 GHz | 1.120 TMAC/s | 373 tok/s | 295 tok/s |

Very aggressive practical targets:

```text
Qwen3.6-A3B: ~150–230 prompt tok/s
Gemma 4 A4B: ~120–190 prompt tok/s
```

SMT4 does not multiply the physical arithmetic resources:

```text
10 physical cores × SMT4 = 40 hardware threads
```

but those threads still share the same execution pipelines.

## Most promising llama.cpp work

The most interesting POWER9 targets are:

```text
ggml_vec_dot_q8_0_q8_0
ggml_vec_dot_q4_0_q8_0
Q4_K × Q8 dot paths
POWER9 tiled GEMV/GEMM
MoE MUL_MAT_ID expert kernels
```

The largest prefill gain would likely come from a dedicated POWER9 GEMM microkernel rather than only improving individual vector-dot functions.

A useful design would:

```text
Q8 activations
+
weights prepacked as biased u8
+
vmsummbm tiled GEMM
+
14+ independent accumulators
+
expert-token grouping
+
NUMA-local expert placement
```

Then benchmark:

```text
pp128
pp512
pp1024
pp2048
pp4096
```

for:

```text
Q8_0
Q4_0
Q4_K
current llama.cpp
new POWER9 GEMM
```

Useful measurements:

```text
tokens/s
cycles/token
IPC
L2/L3 miss rate
DRAM bandwidth
vmsummbm issue rate
NUMA traffic
expert batch-size distribution
SMT1 vs SMT2 vs SMT4
```

## Bottom line

POWER9's VMX integer path is substantially more useful for quantized inference than simply dequantizing everything into FP32.

The most important facts are:

```text
vmsummbm ≈ 16 byte products/instruction
≈ 2 instructions/cycle/core
≈ 32 INT8 MAC contributions/cycle/core
```

That is enough compute to make optimized batch-1 Q4/Q8 decode primarily memory-bound.

For prefill, however, weight reuse allows the CPU to make much better use of that arithmetic throughput, so a custom POWER9 GEMM backend could plausibly reach a few hundred prompt tokens/s on dense and MoE models.

For Qwen3.6-35B-A3B and Gemma 4 26B-A4B, though, 1500+ CPU-only prefill tok/s is beyond the dual-socket POWER9 arithmetic ceiling, and even more impossible when restricted to 10 physical cores.
