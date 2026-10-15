# Models

The architecture is read from the GGUF; nothing else needs to be configured. Text only: image and audio inputs are rejected.

| Model | Hugging Face repository | File | Size |
|---|---|---|---|
| Gemma 4 26B-A4B-it (MoE) | `unsloth/gemma-4-26B-A4B-it-qat-GGUF` | `gemma-4-26B-A4B-it-qat-UD-Q4_K_XL.gguf` | 14.2 GB |
| Gemma 4 E2B-it (dense) | `unsloth/gemma-4-E2B-it-qat-GGUF` | `gemma-4-E2B-it-qat-UD-Q4_K_XL.gguf` | 2.6 GB |
| Gemma 4 E2B-it, mobile | `unsloth/gemma-4-E2B-it-qat-mobile-GGUF` | `gemma-4-E2B-it-qat-UD-Q2_K_XL.gguf` | 2.2 GB |
| DiffusionGemma-26B-A4B-it | `unsloth/diffusiongemma-26B-A4B-it-GGUF` | `diffusiongemma-26B-A4B-it-Q4_K_M.gguf` | 16.8 GB |

```sh
pip install -U huggingface_hub
hf download unsloth/gemma-4-E2B-it-qat-GGUF gemma-4-E2B-it-qat-UD-Q4_K_XL.gguf --local-dir models
```

Supported weight types: F32, F16 (exact backends only), Q4_0, Q4_K, Q5_0, Q6_K, Q8_0 and TQ2_0. Other files of these architectures work if they use only these types; anything else is rejected when the model is bound. Split GGUF files are not supported. Do not open a file that is still downloading.

The integer backends keep a packed copy of the weights in memory (about 1.25 bytes per weight: roughly 32 GB for either 26B model), in addition to the mapped file.

## Gemma 4 26B-A4B and E2B (autoregressive)

GGUF architecture `gemma4`.

- **26B-A4B:** 30 layers (five sliding-window layers, then one global, repeated), hidden width 2816, 128 experts with top-8 routing plus a shared MLP, 262,144-token vocabulary and context. Global layers reuse the raw keys as values.
- **E2B:** 35 layers (28 sliding, 7 global), hidden width 1536, a dense MLP, per-layer token embeddings, and shared K/V: layers 15–34 have no K/V projections and attend over the K/V of the last earlier layer of the same kind.
- **Generation:** the prompt is encoded in 256-token chunks, then one causal pass per token. Sampling defaults to Gemma's settings (temperature 1.0, top-k 64, top-p 0.95), which requests can override.
- **Chat template:** the GGUF's own, rendered as transformers renders it ([chat.md](chat.md)). Thinking is off unless the request enables it through `chat_template_kwargs`.

The equations follow transformers `models/gemma4/modeling_gemma4.py` at revision `bd970fda476f204f9852fb551aa07a4b3fbc5540`.

## DiffusionGemma-26B-A4B-it (diffusion)

GGUF architecture `diffusion-gemma`: the same 30-layer MoE as Gemma 4 26B-A4B, used as a causal encoder (prompt and completed canvases) plus a bidirectional decoder over a 256-position canvas, with additional self-conditioning weights. Encoder and decoder share the text weights.

On a CPU it is much slower than Gemma 4 26B-A4B: every denoising step is a full decoder pass over all 256 positions, including the vocabulary projection for each (one step costs about as much as 170 autoregressive tokens). See [benchmarks.md](benchmarks.md#diffusiongemma-26b-a4b-it-q4_k_m).

**Sources:** Google's [model card](https://huggingface.co/google/diffusiongemma-26B-A4B-it/blob/f7f5b7f5fa82ffc52addd066915886d497f5517b/README.md), [configuration](https://huggingface.co/google/diffusiongemma-26B-A4B-it/blob/f7f5b7f5fa82ffc52addd066915886d497f5517b/config.json) and [generation configuration](https://huggingface.co/google/diffusiongemma-26B-A4B-it/blob/f7f5b7f5fa82ffc52addd066915886d497f5517b/generation_config.json) (a copy is in `fixtures/` to test the defaults), and the transformers [generation](https://github.com/huggingface/transformers/blob/bd970fda476f204f9852fb551aa07a4b3fbc5540/src/transformers/models/diffusion_gemma/generation_diffusion_gemma.py) and [model](https://github.com/huggingface/transformers/blob/bd970fda476f204f9852fb551aa07a4b3fbc5540/src/transformers/models/diffusion_gemma/modeling_diffusion_gemma.py) code (Apache-2.0). The sampler in `src/engine/diffusion.rs` is a new implementation of that algorithm.

**Operator details** that differ from a generic decoder:

- RMS weights are direct multipliers, not `1 + weight`.
- Routing reads the residual before either MLP branch's normalization: unscaled RMS, learned hidden scale and `1/sqrt(hidden)`, projection, softmax, top-8 renormalization, then per-expert scales. Probability ties go to the lower expert ID.
- Fused expert projections hold the whole gate half, then the whole up half. Contributions accumulate in ascending expert ID.
- Shared and expert branches have separate pre/post norms; their sum is normalized, added to the residual, then scaled by the encoder- or decoder-specific layer scalar.
- Self-conditioning: the previous step's temperature-processed logits are softmaxed, multiplied by the token embedding matrix and scaled by `sqrt(2816)`; this passes through its own normalized MLP and is added to the scaled token embedding, followed by an unscaled RMS norm. The first step's zero signal does not skip that norm.
- The decoder reads the encoder cache but never appends the canvas to it.

**Sampling** (defaults from `generation_config.json`):

| Setting | Value |
|---|---:|
| Canvas positions | 256 |
| Maximum denoising steps | 48 |
| Temperature max / min | 0.8 / 0.4 |
| Entropy bound | 0.1 |
| Mean-entropy stop threshold | 0.005 |
| EOS IDs | 1, 106, 50 |

For each canvas:

1. Fill every position with uniform random tokens from the whole vocabulary (not mask tokens).
2. Run the decoder with the encoder cache, the canvas and the previous self-conditioning signal (zero at first).
3. Process the logits with `temperature = min + (max − min) · remaining / max_steps`, `remaining` counting down from 48 to 1.
4. Sample one token per position; compute argmax and entropy from the processed logits.
5. Sort positions by entropy and accept the largest prefix with `sum(entropies) − largest ≤ 0.1` (a joint bound; at least one position). Accepted positions take their sampled tokens, all others fresh noise. Acceptance is recomputed every step.
6. Stop early when the argmax canvas equals the previous one **and** the mean entropy is below 0.005; otherwise continue to the step limit.
7. Commit the latest **argmax** canvas, then truncate at EOS or the length limit. If more output is needed, encode the completed canvas and start a new one.

Random sampling uses ChaCha8 and is reproducible per seed within this engine, not equal to PyTorch's random stream.

`probe-generation` defaults to a one-position diagnostic canvas; pass `--canvas-policy model --steps 48` for the model's own procedure, which `serve` always uses.
