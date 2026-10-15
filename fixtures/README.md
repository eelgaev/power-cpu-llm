# Test fixtures

Small reference data that `cargo test` embeds at compile time (about 1 MB). No model file is needed.

| Path | Contents | Produced by |
|---|---|---|
| `quantized/` | 4–16 rows each of six real DiffusionGemma tensors (Q4_K, Q5_0, Q6_K, Q8_0, F32), with activations and expected projections and transposed accumulations (`cases.txt` lists them) | extracted from `diffusiongemma-26B-A4B-it-Q4_K_M.gguf` |
| `chat/gemma4_chat_reference.json` | 29 chat requests rendered with both reference templates, with transformers' text and token IDs | `chat/generate.py` (transformers 5.18.0, Jinja2 3.1.6, tokenizers 0.23.2) |
| `chat/*.jinja` | The two reference chat templates ([docs/chat.md](../docs/chat.md)) | copies from the model repository and the GGUF |
| `output/gemma4_output_cases.json` | 31 output-parser cases and 7 prompt tails | `tools/oracle-output/produce.py` |
| `generation_config.json` | DiffusionGemma's generation configuration | copy from the model repository |

The larger golden traces of the independent reference model (DiffusionGemma short and full canvases, Gemma 4 prompts, long context; about 30 MB) are not in the repository. `tools/oracle/` regenerates them from the model files; see [docs/validation.md](../docs/validation.md).

The model material in these files is Apache-2.0 (see [THIRD-PARTY-NOTICES](../THIRD-PARTY-NOTICES)).
