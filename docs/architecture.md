# Architecture

power-cpu-llm is one Cargo package: the `power_cpu_llm` library and the `power-cpu-llm` executable. Everything is Rust except the POWER9 kernels, which are assembly files embedded in Rust with `global_asm!`.

## Request path

1. **GGUF reader** (`src/gguf`): parses metadata and the tensor directory. It accepts exactly what the pinned ggml reader accepts; that equivalence is tested against ggml itself (opt-in `reference-oracle` feature).
2. **Model binding** (`src/engine/model.rs`): checks every tensor's name, shape and type against the architecture (`gemma4` or `diffusion-gemma`) and builds the layer bindings. Missing, duplicate, wrongly shaped or unbound tensors are errors.
3. **Weights** (`src/engine/weights.rs`): the file is memory-mapped read-only (`POWER_LLM_NO_MMAP=1` reads instead). The integer backends repack each 128-row unit of a tensor on first use into their vector layout and keep it (about 1.25 bytes per weight; `POWER_LLM_NO_PACK=1` disables it, with identical results).
4. **Text** (`src/engine/tokenizer.rs`, `chat.rs`): the Gemma 4 BPE tokenizer built from the GGUF vocabulary, and the chat template rendered with minijinja exactly as transformers renders it. See [tokenizer.md](tokenizer.md) and [chat.md](chat.md).
5. **Generation** (`src/engine/generation.rs`): autoregressive (Gemma 4: prefill in 256-token chunks, then one causal pass per token) or diffusion (DiffusionGemma: 256-position canvases, entropy-bounded denoising). Both emit committed tokens as events. See [models.md](models.md).
6. **Forward pass** (`src/engine/text.rs`, `attention.rs`, `operators/`): embeddings, attention with sliding/global layers and RoPE, MoE routing and expert grouping, dense and per-layer feed-forward blocks, logits and soft embeddings. Matrix work goes through a `MatrixKernels` table chosen by the backend ([backends.md](backends.md)).
7. **Output** (`src/engine/output.rs`, `completion.rs`): splits committed tokens into the reasoning channel and the answer, with strict handling of malformed output ([output.md](output.md)).
8. **Server** (`src/server`): axum + tokio, one generation at a time on a dedicated thread, a bounded queue, SSE streaming and cancellation ([server.md](server.md)).

## Threads and placement

`src/engine/execution.rs` owns a pool of scoped worker threads:

- Workers are pinned to physical cores inside the process's affinity mask (set it with `numactl --physcpubind`). The default is two workers per physical core.
- Matrix projections use at most two hardware threads per core; workers beyond that join prompt attention only. One-token decode uses one worker per core, because the second thread does not stream weights any faster.
- On POWER9, the pairing of SMT threads that share a core's execution slices differs between cores and does not follow the CPU numbering, so the pool probes it at startup and orders threads accordingly (`POWER_LLM_SLICE_PROBE=0` keeps CPU order).
- Workers set the per-thread data stream prefetch depth (DSCR) to 7 when the kernel allows user access to it, which shortens decode. `POWER_LLM_DSCR=default` leaves the register alone; `POWER_LLM_DSCR=<n>` sets another depth.
- `--split-sockets` divides each long prompt chunk between the two sockets, each socket using its own packed copy of the weights in local memory. Run it without `--membind`. Decode stays on the first socket.
- Idle workers spin for 200 µs before parking (`POWER_LLM_SPIN_US`).

Never let the engine allocate or run on NUMA nodes that have no CPUs (on the AC922, the GPU memory nodes 252–255).

## Numerical policy

- The **portable Rust kernels** (`src/kernels/portable/`) are the reference. Every POWER9 kernel is tested bit for bit against its portable twin for the same backend.
- The **exact backends** (`portable`, `power9-fma`) accumulate in binary64 and produce identical bits on every machine. The **integer backends** (`power9-int8x2`, the serving default, and `power9-int8`) trade that for speed with a documented error bound ([backends.md](backends.md)).
- The transcendental functions that results depend on (`exp`, `log`, `expf`, `sincosf`, `pow`, `powf`, `tanh`, `expm1`, `fmod`) are bit-exact Rust ports of glibc 2.28 in `src/libm`, so the output does not depend on the system C library. `clippy.toml` forbids the `std` versions.
- Results never depend on the number of workers, the encoder chunk size or `--split-sockets`.
- An exactness regression is never fixed by widening a tolerance. How the engine is checked is described in [validation.md](validation.md).

## Environment variables

| Variable | Effect |
|---|---|
| `POWER_LLM_NO_MMAP=1` | Read the GGUF instead of mapping it. |
| `POWER_LLM_NO_PACK=1` | Do not keep packed weights (slower, identical results). |
| `POWER_LLM_DSCR=default\|<n>` | Prefetch depth of the worker threads (default 7). |
| `POWER_LLM_SPIN_US=<µs>` | Spin time of idle workers before parking (default 200). |
| `POWER_LLM_SLICE_PROBE=0` | Skip the SMT pairing probe. |
| `POWER_LLM_SHARED_SCALE=1` | Experimental shared-scale Q4_0 prompt path (different approximation; off by default). |
| `POWER_LLM_STATS=1` | `probe-generation` prints stage and pool timing totals. |
| `POWER_LLM_DUMP_PROMPT_IDS=<file>` | `probe-generation` writes the prompt's token IDs as JSON. |
| `POWER_LLM_TEST_MODEL=<gguf>` | Model for the ignored real-model tests. |
| `POWER_LLM_GGUF_DIRS=<dirs>` | Directories scanned by the GGUF reader's ggml comparison test (default `$HOME/models`). |
