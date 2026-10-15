# Gemma4 chat template and request preparation

`src/engine/chat.rs` turns an OpenAI-shaped chat request into the prompt text and token IDs that transformers `apply_chat_template(..., tokenize=True)` produces for Gemma 4 and DiffusionGemma. It does not cover sampling, stop handling, output parsing, or HTTP. It builds on [the tokenizer](tokenizer.md).

## Templates

The engine renders whatever template the GGUF carries, or a `.jinja` file passed with `--template`. Two templates are the references the tests compare against; copies are in `fixtures/chat/`:

| Reference | Source | sha256 |
|---|---|---|
| Canonical | `chat_template.jinja` of `google/diffusiongemma-26B-A4B-it` @ `f7f5b7f5fa82ffc52addd066915886d497f5517b` ("Gemma 4 Canonical", published 2026-07-09) | `9aeb7eac68ad87bba7567e9d4597ff203e5609f1b427d9e823437d0142cc61bf` |
| Unsloth GGUF | `tokenizer.chat_template` in the Unsloth GGUF @ `f4183a2c7a354128d02545752303c4354d165bf0` (2026-07-04; what llama.cpp uses) | `2f1b4d75d067bae3fe44e676721c7f077d243bc007156cb9c2f8b5836613d082` |

The older GGUF template differs from the canonical one in several ways:

- It replays `reasoning_content` only on tool-calling turns after the last user message. Canonical replays it on any turn after the last user message, plus earlier tool-calling turns with `preserve_thinking`.
- It has no `preserve_thinking`; the option has no effect on it.
- It does not merge consecutive assistant messages the same way.
- It prints `None` where canonical prints `null`.
- After a tool response with thinking enabled, canonical opens `<|channel>thought\n`; the GGUF template doesn't.
- It renders string tool arguments verbatim. Canonical raises on them.

In the 2000-request random corpus, 915 renders differ between the two templates.

## Rendering contract (what transformers does)

The renderer is minijinja 2.24.0, configured to reproduce transformers 5.18.0 with Jinja2 3.1.6.

**Environment and inputs:**

- `trim_blocks` and `lstrip_blocks` are on. The template's trailing newline is dropped.
- The template receives:
  - `bos_token`, `eos_token`, `unk_token`, `pad_token`, `mask_token`
  - `messages`
  - `tools` (`None` when empty)
  - `documents=None`
  - `add_generation_prompt`, `enable_thinking`, `preserve_thinking`
- `raise_exception(msg)` is a template error.

**Python behavior that minijinja doesn't share by default, reimplemented here:**

- Printing uses Python `str()`: `None`, `True`/`False`, `repr(float)` (`1.0`, `1e+20`, `1e-07`), ints, and list/dict `repr`. A `repr` of a non-ASCII string is refused rather than guessed.
- `upper` and `trim` work on Python `str()`. `trim` uses Python whitespace, which includes U+001C–U+001F.
- `dictsort` is a stable sort by `key.lower()`.
- `dict.get(key[, default])` returns `None`, not undefined, for a missing key.
- `str.split(sep)`.
- Any other Python method is an error.

**Tokenization:** the rendered text is encoded with `EncodeOptions::REFERENCE`, meaning special tokens are recognized and no BOS is added. The single `<bos>` comes from the template's `bos_token`. Do **not** pass `add_bos: true`.

## Request preparation

`ChatRequest::from_openai_json(&body)` reads only the template-relevant fields of a `/v1/chat/completions` body. Sampling and other fields are left to the server.

| Field | Handling |
|---|---|
| `messages` | Required and non-empty. |
| `tools` | Optional; only `type: "function"` tools. |
| `chat_template_kwargs` | Optional; only `enable_thinking` and `preserve_thinking` (booleans). Other keys are rejected. |
| `add_generation_prompt` | Defaults to `true`. |
| `continue_final_message: true` | Rejected (unsupported). |

**Validation before rendering.** Each rule fails with `Error::Request`:

- **Roles:** `user`, `assistant`, `tool`, and `system`/`developer` (first message only).
- **Content:** a string, or a list of `{"type":"text","text":…}` parts. Assistant content may be `null`.
- **Non-text parts:** image, audio, and video parts are **rejected**. This path is text-only, and the template would emit `<|image|>` without its embeddings.
- **Optional string fields:** `reasoning`, `reasoning_content`, `tool_call_id`, `name`.
- **Tool calls:** `tool_calls[].function.name` must be a string. `arguments` is required: either a JSON object, or a **string containing a JSON object, which is parsed** (OpenAI sends strings, and the canonical template refuses them). This is the only normalization, and the fixture generator applies the same one.
- **Legacy `tool_responses`:** accepted.
- **Nesting** is limited to 32 levels.

**Special-token text (`SpecialText`).** The reference turns any `<|turn>`, `<bos>`, `<|"|>`, etc. inside content into real control tokens. That lets a user forge turns or break tool encoding.

- `from_openai_json` defaults to `SpecialText::Reject`. Any of the 24 special-token strings found in any request string or object key fails the request. Trimmed concatenated text parts are checked too, preventing a control token assembled across part boundaries.
- `SpecialText::Allow` reproduces the reference exactly. The fixtures use it.

```rust
let tokenizer = Tokenizer::from_vocabulary(&model.vocabulary)?;
let template = ChatTemplate::from_gguf(&model.vocabulary, &tokenizer)?;   // or ChatTemplate::new(&source, &tokenizer)?
let chat = ChatRequest::from_openai_json(&body)?;
let PreparedChat { text, ids } = template.prepare(&tokenizer, &chat)?;     // ids feed ReferenceText::encode
```

`prepare` is the supported text-chat path and rejects tool schemas, tool roles and tool call/response history. `render`/`prepare_reference` keep the templates' tool branches for oracle research only; they do not enable tool generation/execution or serving. Fixture tool cases use `prepare_reference`.

Generation config (not handled here): `eos_token_id = [1, 106, 50]`, i.e. `<eos>`, `<turn|>`, and `<|tool_response>`, the token after which the model waits for a tool result.

## Validation

Fixtures are in `fixtures/chat/gemma4_chat_reference.json`, produced by `fixtures/chat/generate.py` with transformers 5.18.0, Jinja2 3.1.6, and tokenizers 0.23.2. There are 29 requests, each rendered with both templates.

They cover:
- user-only requests, with and without a generation prompt
- system, developer, and system content parts
- thinking, with and without system
- multi-turn conversations
- `strip_thinking` of assistant content
- reasoning before and after the last user message
- Python-whitespace trimming
- text parts, Unicode, and NUL
- special text inside content
- nested, array, enum, nullable, and response tool schemas, plus union-typed properties
- pending and completed tool calls, and tool responses with and without thinking
- every JSON argument type (floats, `null`, nested, mixed-case keys)
- parallel calls resolved by `tool_call_id`, and the name fallback
- legacy `tool_responses`, both mapping and scalar
- consecutive assistant messages
- `preserve_thinking`

| Test | Result |
|---|---|
| Text parity (synthetic special-token vocabulary, no GGUF) | 29 × 2 renders byte-identical |
| Token IDs, real GGUF (ignored by default) | 29 × 2 identical to transformers; exactly one BOS; the GGUF template is byte-identical to the reference copy |
| Random corpus (`--corpus 2000`, not checked in, ignored by default) | 4000 renders byte-identical |
| Unit tests | Python float/str/repr printing, union-type repr, OpenAI parsing, every rejection rule, the special-text policy, and pinned template hashes |

The random corpus mixes system/developer messages, thinking flags, random tool schemas, tool calls with random JSON arguments (including `None`, floats, nested values, special text, quotes, and backslashes), tool responses, and consecutive assistant messages.

No oracle case raised a template error, so error-path parity is only covered by the Rust rejection tests.

```sh
cargo test chat::                                     # fixture parity and unit tests
POWER_LLM_TEST_MODEL=gemma.gguf POWER_LLM_CHAT_CORPUS=corpus.json \
  cargo test --release chat:: -- --include-ignored    # real token IDs and the random corpus
```
