# Matrix backends

Select with `--matrix-backend` (`serve`, `probe-generation`, `trace-prompt`, `trace-full-canvas`, `profile-decode`). `--prefill-backend` picks a different backend for multi-token prompt chunks only.

| Backend | Arithmetic | Use |
|---|---|---|
| `power9-int8x2` | int8 activations in two planes, int32 dots, binary32 scale products | **serving default** |
| `power9-int8` | int8 activations in one plane | faster prompt processing, noisier |
| `power9-fma` | decoded weights × binary32 activations, binary64 accumulation with explicit FMA | exact reference, fast enough to run real prompts |
| `portable` | the same as `power9-fma`, in plain Rust | exact reference on any CPU |
| `portable-int8x2`, `portable-int8` | plain-Rust twins of the integer backends | tests, non-POWER9 machines |
| `auto` | `power9-fma` when available, else `portable` | |

Every POWER9 backend produces the same bits as its portable twin; the tests check this for every encoding, shape and position count.

## Exact backends (`portable`, `power9-fma`)

Each output is a binary64 sum over the columns in ascending order of `(double)w · (double)x`, where `w` is the decoded weight (binary32) and `x` the binary32 activation. `power9-fma` computes the same sums with fused multiply-adds and is still bit-identical:

For binary32 `a`, `b` and a finite binary64 `s`, `fma((double)a, (double)b, s) == s + (double)a * (double)b` bit for bit. The exact product of two binary32 values has at most 48 significant bits and a magnitude in [2^-298, 2^256), so it is exactly representable in binary64 (never subnormal, never overflowing). The separate expression therefore rounds once, at the addition, and `fma` rounds the same exact value once. This holds in every IEEE rounding mode, and signed zeros agree because IEEE 754 defines the sign of an exact zero sum identically for both.

FMA is applied only to such products. The dequantizer arithmetic, GELU and every other expression keep their portable operation order.

## Integer backends (`power9-int8x2`, `power9-int8`)

**Activations** are quantized once per projection, per block of the weight format (256 values for Q4_K/Q6_K, 32 for Q4_0/Q5_0/Q8_0/TQ2_0): `da = max|x| / 127`, `a = round_half_even(x / da)`.

**Weights** are unpacked once into unsigned codes `u = w + bias` (Q4_0: 8, Q5_0: 16, Q8_0: 128, TQ2_0: 1), interleaved so that one 16-byte vector holds a 4-element chunk of four rows. `vmsummbm` then multiplies 16 signed activation bytes by 16 unsigned weight codes into four int32 lanes, one per row, with no horizontal reduction. The bias is removed per position as `bias × Σa`, seeded into the integer accumulators.

**Per output and block**, in ascending block order: `acc = fmaf((float)T, d · da, acc)`, where `T` is the exact int32 dot. Q4_K adds its minimum term the same way.

**Two planes** (`int8x2`): the quantizer also emits the residual `x − da · a`, which is quantized as a second plane and projected with the same weights. The activations become about 254× finer for twice the integer work. Attention and the self-conditioning accumulation use binary32 kernels with a fixed order of operations.

**Error bound** (tested): `|y − exact| ≤ Σ_c |w_c| · da(c) / 2`, plus the binary32 rounding of the scale products, conversions and block accumulation.

**Kernels:** the Q4_0 tiles are hand-written assembly: a 4-row-group × 4-position tile for prompts and an 8 × 2 tile for one-token decode (the two planes of one token). Other formats use the frozen Clang-generated kernels in `src/kernels/asm/int8/`. See [kernels.md](kernels.md).

## Accuracy

Approximate backends fail the exact oracle contract by design (tolerance 1e-4, exact expert sets). What they change, measured against the independent NumPy oracle ([validation.md](validation.md)):

| Model, prompt | Backend | Expert sets differing from exact | Largest hidden-state error | Next-token argmax and top-16 |
|---|---|---|---|---|
| Gemma 4 26B-A4B, 25-token chat | `power9-fma` | 0 / 750 | 1.5e-5 | identical |
| Gemma 4 26B-A4B, 25-token chat | `power9-int8x2` | 0 / 750 | 4.1e-3 | identical |
| Gemma 4 E2B, 16-token chat | `power9-int8x2` | (dense model) | 1.0e-2 (final hidden) | identical |
| DiffusionGemma, 256-position canvases | `power9-int8x2` | 9,477 / 30,720 | — | argmax 98.4–99.6% per canvas |
| DiffusionGemma, 256-position canvases | `power9-int8` | 20,370 / 30,720 | — | argmax 98.4–99.6% per canvas |

In the MoE models, the error of each projection is small; the expert sets differ because the router sees near-ties (probability gaps down to 1e-7) and one flipped expert changes everything after it. On the short benchmark prompts, `power9-int8x2` generates the same tokens as the exact backends; `power9-int8` does not.

With DiffusionGemma, single-plane noise raises the per-step entropy, so adaptive stopping takes more denoising steps: a short request needed 6 decoder passes with `power9-int8` against 3 with the exact and int8x2 backends, and was slower overall.

**Prefill precision guard:** a single-plane prefill under a two-plane runtime (`--matrix-backend power9-int8x2 --prefill-backend power9-int8`) is rejected before any work. It was 1.65× faster on prompts but changed prompt K/V and expert routing substantially (26B: 71 of 750 expert sets; full-logit RMS error 0.0003 → 0.21).

**Experimental:** `POWER_LLM_SHARED_SCALE=1` uses a different Q4_0 prompt approximation (one 15-bit activation split into two bytes sharing a scale). It gave no meaningful prefill gain and is off by default.
