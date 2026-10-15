# Benchmarks

Host: IBM AC922 with two 20-core POWER9 sockets (SMT4), RHEL 8.10, default release build. The host is shared, so all numbers are indicative. Prefill is timed from "model ready" to the last encoded prompt chunk, so it excludes loading; decode is the mean interval between committed tokens.

## Eight cores after the kernel work (2026-10-15)

Same setup and prompts as below, short prompt only, after the hand-written prefill tile and epilogue, decode attention (two heads per K/V pass), batched GELU and DSCR prefetch depth 7 on the workers.

| Model | Runtime backend | Prefill tok/s short | Decode ms/token | Before (2026-10-14) | Same tokens |
|---|---|---:|---:|---|---|
| E2B | `power9-int8x2` | 69.8 | 34.8 | 62.3 / 38.0 | yes |
| E2B | `power9-int8` | 116.8 | 36.4 | 104.7 / 39.7 | no |
| E2B | `power9-fma` | 11.6 | 846 | 11.6 / 848 | (exact) |
| E2B | `portable` | 7.7 | 1,233 | 7.7 / 1,236 | (exact) |
| 26B-A4B | `power9-int8x2` | 44.4 | 57.0 | 40.5 / 68.2 | yes |
| 26B-A4B | `power9-int8` | 73.4 | 59.6 | 67.1 / 69.8 | no |
| 26B-A4B | `power9-fma` | 6.2 | 1,476 | 6.1 / 1,476 | (exact) |

## Eight cores, every backend combination (2026-10-14, before the kernel work above)

**Setup:**
- `numactl --physcpubind=40-71 --membind=0`, with `probe-generation --workers 16`.
- The prompts are a technical document, chat-templated: "short" is its first 2,000 characters (647 tokens for E2B, 651 for 26B), and "long" is the whole of it (3,190 / 3,194 tokens).
- Prefill is the time from model ready to the last encoded prompt chunk.
- Decode is the mean interval between committed tokens; 33 tokens were decoded on the short prompt and 2 on the long one.
- "Same tokens" compares the short-prompt token IDs with the exact backends. The long runs decode only 2 tokens, which agree everywhere.

### Gemma 4 E2B (QAT Q4)

| Runtime backend | Prefill backend | Prefill tok/s short | Prefill tok/s long | Decode ms/token | Same tokens |
|---|---|---:|---:|---:|---|
| `power9-int8x2` | — | 62.3 | 59.3 | 38.0 | yes |
| `power9-int8` | — | 104.7 | 96.7 | 39.7 | no |
| `power9-int8` | `power9-int8x2` | 62.6 | 59.5 | 40.0 | no |
| `power9-fma` | — | 11.6 | 8.9 | 848 | (exact) |
| `power9-fma` | `power9-int8x2` | 58.7 | 57.5 | 847 | yes |
| `power9-fma` | `power9-int8` | 93.0 | 91.1 | 847 | no |
| `power9-int8x2` | `power9-fma` | 11.6 | 8.9 | 38.1 | yes |
| `portable` | — | 7.7 | — | 1,236 | (exact) |
| `power9-int8x2` (repeat) | — | 62.7 | 59.4 | 37.8 | yes |

### Gemma 4 26B-A4B (QAT UD-Q4_K_XL)

| Runtime backend | Prefill backend | Prefill tok/s short | Prefill tok/s long | Decode ms/token | Same tokens |
|---|---|---:|---:|---:|---|
| `power9-int8x2` | — | 40.5 | 37.4 | 68.2 | yes |
| `power9-int8` | — | 67.1 | 58.6 | 69.8 | no |
| `power9-int8` | `power9-int8x2` | 40.5 | 37.3 | 69.9 | no |
| `power9-fma` | — | 6.1 | 4.3 | 1,476 | (exact) |
| `power9-fma` | `power9-int8x2` | 34.9 | 35.8 | 1,476 | yes |
| `power9-fma` | `power9-int8` | 52.9 | 55.2 | 1,478 | yes, on this prompt |
| `power9-int8x2` | `power9-fma` | 6.1 | — | 68.2 | yes |

**Notes:**
- The opt-in `power9-int8` prefills 1.65–1.7× faster than the default and decodes at about the same speed, but its tokens differ from the exact ones.
- Decode speed is set by the runtime backend. An exact runtime backend with a fast prefill backend still decodes at 0.85–1.5 s per token.
- `power9-int8x2` with `power9-fma` prefill gives the default's tokens at the exact backend's prefill speed. It has no use.
- A single-plane prefill under an int8x2 runtime (`--prefill-backend power9-int8`) is rejected: it changes prompt K/V and expert routing.

### All cores (2026-10-14, before the kernel work above)

The same prompts and configurations on both sockets: `numactl --physcpubind=0-159` (no membind), `--workers 80 --split-sockets`. Prompt chunks are divided between the sockets, each with its own copy of the packed weights, and decode runs on socket 0. Socket 1 also runs other work, so these runs are noisier. Every run produced the same token IDs as its eight-core twin.

| Model | Runtime backend | Prefill backend | Prefill tok/s short | Prefill tok/s long | Decode ms/token |
|---|---|---|---:|---:|---:|
| E2B | `power9-int8x2` | — | 109.4 | 105.6 | 31.3 |
| E2B | `power9-int8` | — | 179.4 | 169.8 | 37.1 |
| E2B | `power9-int8` | `power9-int8x2` | 105.0 | 105.5 | 36.3 |
| E2B | `power9-fma` | — | 25.6 | 20.2 | 402 |
| E2B | `power9-fma` | `power9-int8x2` | 103.7 | 103.7 | 404 |
| E2B | `power9-fma` | `power9-int8` | 160.6 | 165.4 | 402 |
| E2B | `power9-int8x2` | `power9-fma` | 25.9 | 20.1 | 32.3 |
| E2B | `portable` | — | 17.7 | — | 604 |
| E2B | `power9-int8x2` (repeat) | — | 111.3 | 106.1 | 36.0 |
| 26B-A4B | `power9-int8x2` | — | 114.0 | 104.2 | 52.1 |
| 26B-A4B | `power9-int8` | — | 164.8 | 156.7 | 53.2 |
| 26B-A4B | `power9-int8` | `power9-int8x2` | 120.0 | 107.8 | 52.8 |
| 26B-A4B | `power9-fma` | — | 20.3 | 14.0 | 719 |
| 26B-A4B | `power9-fma` | `power9-int8x2` | 88.8 | 97.6 | 721 |
| 26B-A4B | `power9-fma` | `power9-int8` | 115.8 | 132.5 | 721 |
| 26B-A4B | `power9-int8x2` | `power9-fma` | 19.3 | 14.0 | 50.2 |
| 26B-A4B | `portable` | — | 13.7 | — | 1,049 |
| 26B-A4B | `power9-int8x2` (repeat) | — | 113.2 | 106.3 | 50.4 |

DiffusionGemma, with the model's canvas policy on the short prompt:

| Backend | Prefill tok/s | Denoising steps | s/step | Time for 256 tokens | tok/s |
|---|---:|---:|---:|---:|---:|
| `power9-int8x2` | 104.3 | 28 | 6.4 | 180 s | 1.43 |
| `power9-int8` | 169.0 | 26 | 4.6 | 119 s | 2.15 |

Against eight cores, all cores speed up 26B-A4B prefill about 2.8× but E2B's only about 1.75× (its shared-K/V layers do not split between sockets). Decode runs on socket 0 only (20 cores against 8): the exact backends, which are compute-bound, decode about 2× faster, while the approximate backends gain 1.2–1.3×, as their single-token decode is limited by memory rather than compute.

### DiffusionGemma-26B-A4B-it (Q4_K_M)

The same short prompt, with `--canvas-policy model --steps 48 --max-new-tokens 256`: one 256-token canvas, adaptive stopping. Both runs stopped as stable and confident, and the text is coherent.

| Backend | Prefill | Denoising steps | s/step | Time for 256 tokens | tok/s |
|---|---:|---:|---:|---:|---:|
| `power9-int8x2` | 17.1 s | 28 | 11.6 | 324 s | 0.79 |
| `power9-int8` | 10.2 s | 26 | 8.2 | 213 s | 1.20 |

**Why it is slower:**
- Every denoising step is a full decoder pass over all 256 positions, including the vocabulary projection for each one. One step costs about 170 of Gemma 4 26B's single-token decode steps.
- Here the model committed about 9–10 tokens per step. To match autoregressive decode it would need about 1.5 steps per canvas.
- The exact backends were not timed: one canvas would take over 20 minutes.

`probe-generation` without `--canvas-policy model` uses a one-position diagnostic canvas. That commits one token per canvas at three model passes each (one encoder, two decoder), so it says nothing about diffusion speed.

## Against llama.cpp (2026-10-07)

Measured a week before the tables above, with an older build of this engine (it has become faster since; compare the eight-core rows). llama.cpp upstream `53ed051c`, CPU only, same GGUF files; about 660-token prompt, 64 decoded tokens.

| Model | Engine | CPUs | Prompt tok/s | Decode tok/s |
|---|---|---|---:|---:|
| 26B-A4B | llama.cpp | socket 0, 40 threads | 17.6 | 8.6 |
| 26B-A4B | power-cpu-llm | socket 0, 40 workers | **55.8** | **13.7** |
| 26B-A4B | llama.cpp | both sockets, 80 threads | 28.4 | 12.4 |
| 26B-A4B | power-cpu-llm `--split-sockets` | both sockets, 80 workers | **84.2** | **13.8** |
| E2B | llama.cpp | socket 0, 40 threads | 27.2 | 11.0 |
| E2B | power-cpu-llm | socket 0, 40 workers | **79.1** | **19.0** |
| E2B | llama.cpp | both sockets, 80 threads | 42.1 | 9.0 |
| E2B | power-cpu-llm `--split-sockets` | both sockets, 80 workers | **82.2** | **18.8** |
