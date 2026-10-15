# Validation

The engine is checked at four levels: unit and twin tests, an independent NumPy reference model, byte-for-byte regression exports, and behaviour checks against a running server.

## 1. Unit and twin tests (`cargo test`)

- Every POWER9 kernel against its portable twin, bit for bit (see [kernels.md](kernels.md#tests)).
- Decoders, block dots and projections against reference rows extracted from a real DiffusionGemma GGUF (`fixtures/quantized/`).
- The chat template against transformers' rendering and tokenization (`fixtures/chat/`, 29 requests), the output parser against the official response schema parser (`fixtures/output/`), and the diffusion sampler's defaults against Google's `generation_config.json`.
- The libm ports against digests of glibc's results.

The fixtures in `fixtures/` are small (about 1 MB) and embedded into the tests at compile time. Tests that need a real model are `#[ignore]`d; run them with `POWER_LLM_TEST_MODEL=<gguf> cargo test --release -- --ignored`.

## 2. Independent reference model (`tools/oracle/`)

`tools/oracle/` is a standalone Python/NumPy implementation of the models. It shares no code with the engine and never reads engine output:

- its own GGUF reader and dequantizers (`ggufref.py`), checked bit-identical to `gguf-py` over 2,187 sampled rows of every tensor type;
- equations from the pinned transformers sources (`reference.py`, with line references), computed in binary64.

| Script | Produces |
|---|---|
| `produce.py` | DiffusionGemma short-canvas traces (encoder, cache, decoder, logits; 4.8 MiB) |
| `full_canvas.py` | DiffusionGemma full 256-position canvases, self-conditioning and a completed-canvas encoder extension (24 MiB; 22 min and 24 GiB of memory to produce) |
| `gemma4_reference.py` | Gemma 4 26B-A4B: every layer's output and expert routing at every prompt position, final hidden rows and logits |
| `gemma4_e2b_reference.py` | The same for Gemma 4 E2B (per-layer embeddings, shared K/V) |
| `long_context.py` | DiffusionGemma beyond the 1,024-token sliding window |

These golden traces are not in the repository; they are regenerated from the model file. Each script's header has the exact command. The tools need Python 3 and NumPy (on ppc64le, NumPy builds from source against OpenBLAS); the Gemma 4 scripts take the prompt's token IDs as input, which `probe-generation` writes with `POWER_LLM_DUMP_PROMPT_IDS=<file>`.

**Comparing the engine:**

```sh
# Gemma 4: export the engine's values for the golden prompt, then compare.
power-cpu-llm trace-prompt MODEL.gguf GOLDEN_DIR OUT_DIR --matrix-backend power9-fma
python3 tools/compare-full-canvas.py GOLDEN_DIR OUT_DIR --report report.json

# DiffusionGemma full canvas (the processed self-conditioning input comes from the oracle's work directory).
power-cpu-llm trace-full-canvas MODEL.gguf FULL_CANVAS_DIR WORK/case2_processed_sc.f32 OUT_DIR --matrix-backend power9-fma
python3 tools/compare-full-canvas.py FULL_CANVAS_DIR OUT_DIR
```

The contract: every value within `1e-4 + 1e-4·|expected|`, identical top-8 expert **sets** for every layer and position, and identical argmax and top-16 IDs for every logit row. The comparator exits 1 on any failure and reports the routing gap or logit margin of each mismatch. Never regenerate golden data from engine output, and never widen the tolerance to hide a difference.

**Results with the exact backends:**

| Model and case | Expert sets | Logits | Values |
|---|---|---|---|
| DiffusionGemma, four 256-position cases | 30,720 / 30,720 identical | 768 / 768 argmax and top-16 identical | 85 / 85 entries within tolerance |
| Gemma 4 26B-A4B, 25-token chat prompt | 750 / 750 identical | identical | max error 1.5e-5 |
| Gemma 4 E2B, 16-token chat prompt | (dense model) | identical | 0 of 860,160 layer values out of tolerance, max error 1.9e-5 |

What the approximate backends change is in [backends.md](backends.md#accuracy).

## 3. Regression exports

After every change to kernels or the engine, the exports above are repeated with the new build and compared **byte for byte** with the previous build's: the DiffusionGemma full-canvas export (112 blobs) and the Gemma 4 and E2B prompt exports with both `power9-fma` and `power9-int8x2`. Token IDs of fixed prompts are compared too. A speed change that alters any byte is rejected, unless it is a deliberate numerical change that passes the oracle contract again.

## 4. Behaviour (`tools/`)

Against a running `power-cpu-llm serve`:

- `acceptance-server.py`: fixed cases with pass criteria set in advance (a capital, arithmetic, a system prompt, multi-turn context, translation, primes, exact counting to 100, code, a long story, reasoning output, a long-prompt recall, and rejection of a prompt over the context limit).
- `long-context-needle.py`: recall of a code placed at the start, middle or end of a long document.

These are pass/fail checks, not quality or speed measurements.

## Not covered

- Parity with the official BF16 PyTorch models: the engine runs the quantized GGUF, and the oracle reproduces that, not the unquantized model.
- Broad generation quality. DiffusionGemma's full procedure is validated numerically per canvas, not for long multi-canvas outputs.
