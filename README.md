# power-cpu-llm

A standalone CPU inference engine for IBM POWER9, written in **Rust**, with its compute kernels in **POWER9 assembly embedded in Rust** (no C or C++ in the build). It runs **Gemma 4** (26B-A4B and E2B, autoregressive) and **DiffusionGemma-26B-A4B-it** (diffusion) from GGUF files, text only, and serves them through an OpenAI-compatible HTTP API.

More model architectures are planned; the engine is not tied to the Gemma family.

This is not a llama.cpp backend. The GGUF reader is Rust and accepts exactly what the pinned [ggml](https://github.com/ggml-org/ggml) reader accepts; ggml is only an opt-in test oracle, never part of the executable.

## Status

- **Models:** Gemma 4 26B-A4B-it and E2B-it (Unsloth QAT GGUFs, including E2B's `TQ2_0` mobile file) and DiffusionGemma-26B-A4B-it (`Q4_K_M`). See [docs/models.md](docs/models.md).
- **Serving:** `power-cpu-llm serve` loads one model and serves `POST /v1/chat/completions` (streaming or not) and `GET /v1/models`, up to the model's full context length. Requests run one at a time; concurrent requests are queued, not batched. See [docs/server.md](docs/server.md).
- **Matrix backends** (`--matrix-backend`, [docs/backends.md](docs/backends.md)):
  - `power9-int8x2`: the serving default, two-plane int8 activations;
  - `power9-int8`: faster prompt processing, noisier, opt-in;
  - `power9-fma` and `portable`: exact reference backends.
- **Validation** ([docs/validation.md](docs/validation.md)):
  - The exact backends pass an independent NumPy reference model within 1e-4, with every MoE expert selection and every top-16 logit set identical, on full 256-position DiffusionGemma canvases and on Gemma 4 prompts.
  - Every POWER9 kernel is bit-identical to its portable Rust twin, and every change is checked byte for byte against the previous build's outputs.
- **Pure Rust:** the GGUF reader, tokenizer, chat templates (minijinja) and the maths functions the numerics depend on (bit-exact ports of glibc 2.28's) are all Rust.
- **Not implemented:** batching of concurrent requests, vision and audio, tool calls, and the OpenAI `stop`, `logprobs`, penalty and `response_format` parameters (requests that set them get a 400). Gemma 4 accepts `temperature`, `top_p` and `top_k`; DiffusionGemma's sampler has its own schedule and rejects them.

## Quick start

Requirements: Linux on POWER9 (ppc64le), [Rust](https://rustup.rs) 1.99 or later, and `numactl` to choose CPUs. Nothing else: no C compiler, CMake or Python.

```sh
git clone <this repository> power-cpu-llm && cd power-cpu-llm
cargo build --release                                  # target/release/power-cpu-llm

pip install -U huggingface_hub
hf download unsloth/gemma-4-E2B-it-qat-GGUF gemma-4-E2B-it-qat-UD-Q4_K_XL.gguf --local-dir models

numactl --physcpubind=0-79 --membind=0 \
  target/release/power-cpu-llm serve models/gemma-4-E2B-it-qat-UD-Q4_K_XL.gguf
```

```sh
curl http://127.0.0.1:8080/v1/chat/completions -H 'Content-Type: application/json' \
  -d '{"messages": [{"role": "user", "content": "What is the capital of France?"}]}'
```

`--physcpubind` and `--membind` choose the cores and the memory node (here the first socket of a two-socket machine; check yours with `numactl --hardware`). The engine starts two workers per physical core in that set and pins them. Never place the engine on GPU-only memory nodes. Gemma 4 26B-A4B needs about 46 GB of memory with the default backend: the 14 GB file plus a packed copy of the weights.

## Build and test

```sh
cargo build --release
cargo test --release
cargo clippy --all-targets -- -D warnings
```

The tests need no model file: their reference data is in `fixtures/` (about 1 MB). Tests that need a real model are ignored by default; run them with `POWER_LLM_TEST_MODEL=<file.gguf> cargo test --release -- --ignored`.

The POWER9 assembly is built only for PowerPC64 targets, and runtime CPU detection selects it. On other machines the portable backends run (correct but slow):

```sh
cargo test --no-default-features kernels::
```

Tests against the pinned ggml (its GGUF reader and dequantizers) are opt-in, because they build `vendor/ggml` with CMake and a C++ compiler:

```sh
git submodule update --init --recursive
cargo test --features reference-oracle gguf::
cargo test --features reference-oracle engine::
```

### Optimized build (optional)

The default release build already uses `opt-level = 3`, thin LTO and one codegen unit. The `release-max` profile adds whole-program (fat) LTO, and the `RUSTFLAGS` below also compile the Rust code for POWER9 specifically:

```sh
RUSTFLAGS="-C target-cpu=pwr9" cargo build --profile release-max
# target/release-max/power-cpu-llm (requires POWER9 or newer)
```

Its results are byte-identical to the default build's, and it measured **no faster** (within 0.3%), because the time is spent in the assembly kernels, which compiler flags do not change.

### Static executable (optional)

A fully static executable links against the static glibc of IBM Advance Toolchain 15. Its results are byte-identical to the dynamic build's, because the maths functions that affect them are Rust ports ([docs/kernels.md](docs/kernels.md#transcendentals-srclibm)):

```sh
RUSTFLAGS="-C target-feature=+crt-static -C relocation-model=static -C linker=/opt/at15.0/bin/gcc" \
  CARGO_TARGET_DIR=target/static cargo build --release --target powerpc64le-unknown-linux-gnu
```

It bundles glibc (LGPL-2.1), so if you distribute it, you must let recipients relink it.

## Command line

`power-cpu-llm --help` lists the commands; each has `--help`.

| Command | Purpose |
|---|---|
| `serve` | the OpenAI-compatible server |
| `probe-generation MODEL "text"\|@file` | one chat request from the command line, with stage times, token IDs and text. For DiffusionGemma, add `--canvas-policy model --steps 48` for the model's own procedure (the default is a one-position diagnostic canvas). |
| `inspect MODEL [--tensors] [--metadata]` | GGUF metadata and tensor inventory |
| `check-weights`, `probe-model`, `probe-forward` | weight decoding checks and slow reference diagnostics |
| `trace-prompt`, `trace-full-canvas`, `trace-long-context`, `trace-forward`, `trace-batched` | export internal values for comparison with the reference model ([docs/validation.md](docs/validation.md)) |
| `profile-decode`, `diffusion-config`, `bench` | DiffusionGemma pass timing, the denoising configuration, a row-dot microbenchmark |

## Performance

Measured on an AC922 with two 20-core POWER9 sockets (SMT4), default release build. The host is shared, so treat the numbers as indicative. The prompt is about 650 tokens; prefill is the rate at which it is encoded, decode the steady-state generation rate. All rates are tokens per second. Full results and method: [docs/benchmarks.md](docs/benchmarks.md).

- **8 cores:** CPUs 40–71, `numactl --membind=0`, 16 workers (2026-10-15).
- **All cores:** CPUs 0–159, 80 workers, `--split-sockets`: prompt chunks are divided between the sockets and decode runs on socket 0 (2026-10-14, a build from before the 8-core kernel work).

| Model | Backend | Prefill, 8 cores | Decode, 8 cores | Prefill, all cores | Decode, all cores |
|---|---|---:|---:|---:|---:|
| Gemma 4 E2B | `power9-int8x2` (default) | 69.8 | 28.7 | 109 | 28–32 |
| Gemma 4 E2B | `power9-int8` | 116.8 | 27.5 | 179 | 27.0 |
| Gemma 4 E2B | `power9-fma` (exact) | 11.6 | 1.18 | 25.6 | 2.49 |
| Gemma 4 E2B | `portable` (exact) | 7.7 | 0.81 | 17.7 | 1.66 |
| Gemma 4 26B-A4B | `power9-int8x2` (default) | 44.4 | 17.5 | 114 | 19.2–19.8 |
| Gemma 4 26B-A4B | `power9-int8` | 73.4 | 16.8 | 165 | 18.8 |
| Gemma 4 26B-A4B | `power9-fma` (exact) | 6.2 | 0.68 | 20.3 | 1.39 |

The default `power9-int8x2` produced the same tokens as the exact backends on these prompts; `power9-int8` did not.

**DiffusionGemma** generates 0.79 tok/s on 8 cores and 1.43 tok/s on all cores with `power9-int8x2` (1.20 and 2.15 with `power9-int8`), using the model's own procedure: a 256-token canvas and 26–28 denoising steps. Every step processes the whole canvas, including vocabulary logits. On a CPU, which has no idle compute to absorb that, it is 12–18× slower than Gemma 4 26B-A4B's autoregressive decode.

**Against llama.cpp** (2026-10-07, with an older and slower build of this engine; about 660-token prompt, 64 decoded tokens, CPU only), in tok/s:

| Model | CPUs | Prefill: llama.cpp → power-cpu-llm | Decode: llama.cpp → power-cpu-llm |
|---|---|---:|---:|
| 26B-A4B | socket 0 (40 threads) | 17.6 → **55.8** | 8.6 → **13.7** |
| 26B-A4B | both sockets (80 threads) | 28.4 → **84.2** | 12.4 → **13.8** |
| E2B | socket 0 (40 threads) | 27.2 → **79.1** | 11.0 → **19.0** |
| E2B | both sockets (80 threads) | 42.1 → **82.2** | 9.0 → **18.8** |

## Documentation

- [docs/architecture.md](docs/architecture.md): request path, threads and placement, numerical policy, environment variables
- [docs/models.md](docs/models.md): supported models and downloads, architecture notes, the DiffusionGemma sampler
- [docs/server.md](docs/server.md): `serve` options and the HTTP API
- [docs/backends.md](docs/backends.md): exact and integer matrix backends, error bounds and accuracy
- [docs/kernels.md](docs/kernels.md): the assembly kernels and the bit-exact maths functions
- [docs/validation.md](docs/validation.md): tests, the independent reference model and the regression checks
- [docs/benchmarks.md](docs/benchmarks.md): all measurements and their method
- [docs/tokenizer.md](docs/tokenizer.md), [docs/chat.md](docs/chat.md), [docs/output.md](docs/output.md): text pipeline contracts
- [docs/power9-analysis.md](docs/power9-analysis.md): the original analysis of POWER9's int8 throughput that motivated the project

## Layout

- `src/kernels/`: the kernel API, portable Rust kernels (`portable/`) and the POWER9 assembly (`asm/<family>/*.s`, included by the Rust modules as is).
- `src/libm/`: bit-exact Rust ports of glibc 2.28's `exp`, `log`, `expf`, `sincosf`, `pow`, `powf`, `tanh`, `expm1` and `fmod`.
- `src/gguf/`: the GGUF reader.
- `src/engine/`: tokenizer, chat templates, model binding and weights, forward passes, autoregressive and diffusion generation, output parsing, the worker pool.
- `src/server/`: the HTTP server (axum + tokio).
- `src/main.rs`, `src/cli/`: the command line.
- `fixtures/`: test reference data ([fixtures/README.md](fixtures/README.md)).
- `tools/oracle/`: the independent NumPy reference model; `tools/oracle-output/`: the output-parser fixture generator; `tools/*.py`: trace comparison and server acceptance checks; `tools/codegen/`: the libm table generator.
- `vendor/ggml`: pinned submodule, used only by the opt-in oracle tests.

## License

power-cpu-llm is licensed under the GNU Lesser General Public License, version 2.1 or (at your option) any later version (LGPL-2.1-or-later); see [LICENSE](LICENSE). Third-party code, embedded material and dependency licences are listed in [THIRD-PARTY-NOTICES](THIRD-PARTY-NOTICES).

The engine's numerics depend on the last bits of `exp`, `log`, `expf`, `sincosf`, `pow`, `powf`, `tanh`, `expm1` and `fmod`, and different libm implementations round some inputs differently. To keep outputs byte-identical regardless of the system C library (including in a static build), `src/libm` contains bit-exact Rust translations of glibc 2.28's implementations; a translation is a derivative work, so that code is under glibc's licence, LGPL-2.1-or-later. Correctly rounded permissive alternatives (CORE-MATH, pxfm) differ from glibc in the last bits and would change outputs, so the project stays with glibc's behaviour and uses one licence for everything rather than mixing licences; "or later" is needed because some dependencies (e.g. minijinja) are Apache-2.0-only, which is compatible with LGPL-3.0 but not with LGPL-2.1 alone.

`vendor/ggml` (MIT) is used only as an opt-in test oracle and is never linked into the shipped binary. Model weights are not included in this repository and have their own terms.
