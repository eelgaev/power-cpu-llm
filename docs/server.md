# OpenAI-compatible HTTP server

`power-cpu-llm serve` loads the model, tokenizer, chat template and weights **once**, then serves OpenAI-style chat completions over HTTP/1.1. Code: `src/server` (axum + tokio) and `src/cli/serve.rs`.

```sh
numactl --physcpubind=0-79 --membind=0 \
  power-cpu-llm serve models/gemma-4-26B-A4B-it-qat-UD-Q4_K_XL.gguf \
  [--bind 127.0.0.1:8080] [--model-id NAME] [--workers N] \
  [--matrix-backend power9-int8x2|power9-int8|power9-fma|portable|auto] [--prefill-backend B] \
  [--split-sockets] [--template gguf|FILE.jinja] [--queue N] [--max-context N] [--encoder-chunk N]
```

`numactl` chooses the CPUs and memory node; the engine pins its workers inside that set. On a two-socket machine, `--membind` keeps the weights local to the socket that runs them.

| Option | Default |
|---|---|
| `--bind` | `127.0.0.1:8080` (localhost only; there is no authentication) |
| `--model-id` | the GGUF file stem |
| `--workers` | 2 per physical core in the affinity mask. Workers beyond 2 per core join prompt attention only. |
| `--matrix-backend` | `power9-int8x2` when available, else `auto` ([backends.md](backends.md)) |
| `--prefill-backend` | none: a different backend for multi-token prompt chunks only. A single-plane prefill under a two-plane runtime is rejected. |
| `--split-sockets` | off. When the workers span two sockets, every prompt chunk of at least 64 tokens is divided between them, each socket using its own copy of the packed weights; the division adapts to the sockets' measured speed. Decode stays on the first socket. Results are identical. Run without `--membind`, so each copy is placed locally; it costs a second packed copy of the weights. |
| `--template` | `gguf`: the template stored in the model. A path to a `.jinja` file overrides it. |
| `--queue` | 4 waiting requests (at most 64) |
| `--max-context` | the model's context length (from the GGUF) |
| `--encoder-chunk` | 256 prompt tokens per encoder pass; results do not depend on it |

Startup prints the resolved backend, the worker CPUs and the context limit.

## Endpoints

- `GET /v1/models`: one model, `{"id", "object": "model", "owned_by": "power-cpu-llm"}`.
- `POST /v1/chat/completions`, below.
- Anything else: 404 in the OpenAI error shape.

### Request fields

| Field | Handling |
|---|---|
| `messages` | required; roles `system` (first message only), `user`, `assistant`; each message has only `role` and `content`, a string or text parts |
| `model` | optional; must equal the served model id |
| `max_completion_tokens` / `max_tokens` | optional, positive (default 256; give at most one) |
| `temperature`, `top_p`, `top_k` | Gemma 4 only: `temperature` in [0, 2], `top_p` in (0, 1], `top_k` ≥ 0 (default: the model's 1.0 / 0.95 / 64). DiffusionGemma rejects them: its sampler has its own temperature schedule. |
| `seed` | optional unsigned integer (default 7) |
| `stream` | optional boolean |
| `include_reasoning` | optional boolean (default false): also return the thought channel as `reasoning_content` |
| `n` | only `1` |
| `chat_template_kwargs` | `enable_thinking`, `preserve_thinking` |
| **anything else** | **400 `invalid_request`**: for example `stop`, `tools`, `logprobs`, penalties, `response_format`, image parts, tool messages and tool calls |

Unsupported fields are never ignored silently.

### Responses

- **Non-streaming:** `chat.completion` with `id` (`chatcmpl-N`), `created`, `model`, `choices[0].message.{role, content[, reasoning_content]}`, `finish_reason` (`stop` or `length`) and `usage` (`prompt_tokens`, `completion_tokens`, `total_tokens`).
- **Streaming (SSE):** a `chat.completion.chunk` per token (Gemma 4) or per committed canvas (DiffusionGemma; drafts are never sent), then a final chunk with `finish_reason` and `usage`, then `data: [DONE]`. A keep-alive comment is sent every 10 s.
- **Errors:** `{"error": {"message", "type", "param", "code"}}`. Partial content is never presented as a choice.

| Situation | Status | Code |
|---|---|---|
| prompt plus requested output exceeds the context limit (DiffusionGemma rounds the output up to whole 256-token canvases) | 400, before any model work | `context_length_exceeded` |
| invalid or unsupported request | 400 | `invalid_request` |
| queue full | 503 | `queue_full` |
| generation failure, or malformed model output | 500 | `generation_failed` |

## Scheduling

- **One generation at a time** runs on a dedicated thread; concurrent requests are not batched. HTTP runs on a small tokio runtime and never blocks on model work.
- **Queue:** waiting requests are bounded by `--queue`; excess requests get 503 at once.
- **Cancellation:** a closed connection or dropped stream is detected within 100 ms, and the generation stops at the next token or denoising step.

## Tests

`cargo test server::` covers the models list, every rejected field and role, the bounded queue, the context error without execution, the JSON and SSE shapes, cancellation on a dropped stream and TCP disconnect detection. Real generation is covered by the engine's tests and by `tools/acceptance-server.py` ([validation.md](validation.md#4-behaviour-tools)).
