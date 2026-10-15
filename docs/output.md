# Committed-output parser (Gemma4 reasoning channel vs answer)

`src/engine/output.rs` turns **committed** generated token IDs into assistant **content**, plus an optional, separate **reasoning** text, a final status and incremental updates. It is pure Rust over the checked tokenizer. It has no tool, HTTP or generation logic, and it does not change committed IDs, token counts, finish reasons or caches.

## What the markers mean (pinned evidence)

| Source (google/diffusiongemma-26B-A4B-it @ `f7f5b7f5`) | Evidence |
|---|---|
| Model card (README), "Thinking Mode Configuration" | Output structure is `<|channel>thought\n` **[Internal reasoning]** `<channel|>` **[Final answer]**. "If thinking is disabled, the model will still generate the tags but with an empty thought block: `<|channel>thought\n<channel|>`[Final answer]." History must not contain thoughts. |
| `tokenizer_config.json` `response_schema` (`x-regex`) | `(<|channel>thought\n(?P<thinking>.*?)<channel|>)?(?P<tool_calls><|tool_call>.*<tool_call|>)?(?P<content>…)?(?:<turn|>|<|tool_response>)?`. Names `soc_token=<|channel>`, `eoc_token=<channel|>`, `eot_token=<turn|>`. |
| Both chat templates (canonical and the Unsloth GGUF copy) | `strip_thinking` drops `<|channel>…<channel|>` spans from replayed model content. The generation prefix is `<|turn>model\n` with thinking on or off (`<|think|>` goes in the system turn). Only the canonical template, after a tool response with thinking on, ends in `<|channel>thought\n`. |
| `generation_config.json` | `eos_token_id = [1, 106, 50]`: `<eos>`, `<turn|>`, `<|tool_response>`. |

The real full-width run's committed IDs `[100, 45518, 107, 101, 50429, 1]` are `<|channel>`, `thought`, `\n`, `<channel|>`, `Paris`, `<eos>`. They parse as **reasoning `""` (an empty thought block, present), content `"Paris"`, status complete (`<eos>`)**. This matches the official schema parser (fixture `observed_full_width_run`). Generic `skip_special_tokens` decoding (`thought\nParis`) is wrong because it leaks the channel header into the answer.

## API

```rust
let markers = Markers::new(&tokenizer)?;                                  // IDs looked up by spelling, checked special
let context = InitialContext::from_prompt_ids(&tokenizer, &markers, &prepared.ids)?;
let mut parser = OutputParser::new(&tokenizer, context)?;
for canvas in committed_canvases { for update in parser.push(&canvas)? { /* Update::{ReasoningStarted, Reasoning(s), ReasoningEnded, Content(s)} */ } }
let Output { content, reasoning, status, replaced_invalid_utf8, trailing } = parser.finish();
```

**Markers.** `<|channel>`, `<channel|>`, `<|turn>`, `<turn|>` and `<eos>` are required. `<eos>` must also equal the vocabulary's EOS. The tool tokens (`<|tool>`, `<tool|>`, `<|tool_call>`, `<tool_call|>`, `<|tool_response>`, `<tool_response|>`, `<|"|>`) and media tokens (`<|image>`, `<image|>`, `<|audio>`, `<audio|>`, `<|image|>`, `<|audio|>`, `<|video|>`) are each looked up and checked to be special. Every other special token counts as an unexpected control. Nothing is an unchecked architecture-wide constant.

**Initial context.** The last special token of the prepared prompt is checked by ID, and the ordinary tokens after it by their decoded bytes:

| Prompt ends with | Context |
|---|---|
| `<|turn>` + `model\n` | `ModelTurn`: both templates, thinking on or off |
| `<|channel>` + `thought\n` | `InsideThought`: canonical template after a tool response with thinking on; the engine can't produce this (no tools), but it is parsed correctly |
| anything else (no generation prompt; text that only *spells* `<|turn>model\n`) | `Err(UnsupportedPrefix)` |

**Updates.**
- `push` returns updates for each call. On failure, `PushError { error, updates }` still carries the updates emitted earlier in that call.
- Concatenating every `Reasoning` / `Content` update, plus `Output::trailing`, reproduces the final `reasoning` / `content` exactly.
- Any chunking of the same IDs gives identical results and updates. This is tested for every split point.

## Grammar and policy

```
ModelTurn:  Start --<|channel>--> Header --"thought\n"--> Reasoning --<channel|>--> Start
            Start/Content --ordinary--> Content          (plain output with no markers is all content)
            Start/Content --<eos>|<turn|>--> Done
InsideThought starts in Reasoning (reasoning = Some("")).
```

**Header.** The bytes after `<|channel>` are withheld (at most 7 pending) until they equal `thought\n` exactly; any extra bytes in the same token are reasoning. Anything else is `UnknownChannelHeader`.

**Adjacent spans.** Several thought spans *before* any content are allowed and their reasoning is concatenated. A span *after* content has started is `ChannelAfterContent`.

**Errors and incomplete results.** These come from input, not from bugs. After an error, every `push` repeats it, and text already emitted is kept as emitted:

| Situation | Status |
|---|---|
| `<eos>` / `<turn|>` inside reasoning or its header | `Error(TerminatorInsideReasoning)`. No `<channel|>` is invented, and the reasoning stays reasoning. |
| input ends (`finish`) inside reasoning or its header, e.g. at a length limit | `Incomplete(UnclosedReasoning)`. Reasoning **never** becomes content. |
| input ends in content without a terminator | `Incomplete(TruncatedContent)` |
| stray `<channel|>`, `<|turn>`, `<bos>`, `<pad>`, `<unk>`, `<mask>`, `<|think|>` | `Error(UnexpectedControl(id))` |
| any tool token, including the `<|tool_response>` generation stop and `<|"|>` | `Error(UnsupportedToolStructure(id))`. Never content, never executed. |
| media tokens | `Error(UnsupportedMedia(id))` |
| ID ≥ vocabulary size | `Error(TokenOutOfRange(id))` |
| `push` after `<eos>`/`<turn|>`, or after `finish` | `Err(InputAfterTermination)`; the result is unchanged |

**Ordinary text that looks like structure.** Ordinary tokens are always text, even if they spell `<|channel>thought\n<channel|>` or `thought\n` (fixtures `marker_spelled_as_ordinary_text` and `thought_word_in_content`).

**`finish()`.** It is idempotent, and it never closes a span or invents a terminator.

**Text and UTF-8.**
- Each ordinary token is decoded with the checked tokenizer (`decode_bytes`: byte-fallback tokens become raw bytes, `▁` becomes a space). The bytes are streamed through incremental UTF-8, so an incomplete character is withheld across `push` calls and commit boundaries.
- Invalid sequences become one U+FFFD each, as `String::from_utf8_lossy` does.
- An unterminated tail at `finish` becomes one U+FFFD, delivered in `Output::trailing`.
- All of these are counted in `replaced_invalid_utf8`.
- When text switches between reasoning and content, a pending incomplete character is flushed to the side it belongs to.
- Text is not trimmed, matching the official parser (for example, leading spaces are kept).

### Dropped channel opener (added 2026-10-05)

At long context (a 12,948-token prompt) the model was observed to omit the `<|channel>` opener. It began its turn with `thought\n\n<channel|>ORCA-4000-X<turn|>`. Previously this was `UnexpectedControl(101)`, so a correct answer became a 500 error in the server.

**Policy:**
- **Withholding:** at the very start of a model turn only, leading ordinary bytes are withheld while they are still a prefix of `thought\n` (state `ImplicitHeader`).
- **Recovery:** a complete `thought\n` is treated as a reasoning span whose opener was dropped. It emits `ReasoningStarted`, and its text and the following `<channel|>` behave as for an opened span.
- **Fallback:** if the bytes diverge from the header (for example `thoughtful`), or any control token or the end of input arrives first, the withheld bytes are flushed as ordinary content and parsing continues unchanged.
- **Scope:** later in the turn (after content), a stray `<channel|>` is still `UnexpectedControl`.

**Tests:**
- `dropped_channel_opener_is_an_implicit_reasoning_span`: every split, the divergent prefix, the ambiguous prefix at EOS, and the after-content error.
- `actual_long_context_dropped_opener_output`: the exact observed IDs under the real vocabulary give content `ORCA-4000-X`.

All existing fixture cases are unchanged.

## Fixtures and provenance

`fixtures/output/gemma4_output_cases.json` holds 31 cases and 7 prompt-prefix tails, produced by `tools/oracle-output/produce.py`. It records `tokenizer.json` sha256, the pinned `response_schema`, oracle versions, the model card quote, the generation EOS IDs, and the strings of all 80 token IDs used.

**Oracle cases (8)** use **transformers 5.0.0 `recursive_parse`** (the official implementation of `response_schema` parsing; it was removed in 5.18) on the pinned schema, applied to `decode(ids)` of the IDs before the final terminator. Covered: the observed six IDs, thinking + answer + `<turn|>`, an empty thought block, plain output without markers, empty content after a thought, only `<eos>`, Unicode with byte fallback, and whitespace and newlines.

**Policy cases (23)** cover truncation inside reasoning, header and content; `<eos>` inside reasoning; marker spelling and `thought\n` as ordinary text; an unknown channel; adjacent spans; a channel after content; a stray close; a new turn; `<bos>` / `<pad>` / `<|think|>` / `<mask>`; tool call, tool-response stop and `<|"|>`; media; invalid and truncated UTF-8; and the `InsideThought` context.

In policy cases the text-regex oracle is unsafe, so its result is stored as `oracle_text_regex` for contrast. For example:
- truncated reasoning parses as content `"<|channel>thought\nstill thinking"`, leaking reasoning
- marker-spelling text parses as structure
- `<eos>` ends up inside content (`"Paris<eos>"`), because the schema only stops at `<turn|>` / `<|tool_response>`

**Prefix tails** are rendered with both pinned templates, thinking on and off, with and without a generation prompt, plus the canonical tool-response case.

## Tests (`cargo test output::`)

| Test | Coverage |
|---|---|
| fixture cases under every split | All 31 cases, pushed whole, one ID at a time, and in **every 2-way and 3-way split**: identical `Output`; updates plus `trailing` equal the final text; content/reasoning/status/terminator/error equal the fixture |
| observed run | `[100,45518,107,101,50429,1]` gives updates `ReasoningStarted, ReasoningEnded, Content("Paris")` and output content `"Paris"`, reasoning `Some("")`, `Complete(Eos)` |
| prefix contexts | all 7 rendered tails, plus no special token, missing newline, extra text, and `<|turn>model\n` spelled as ordinary text |
| termination | `push` after the terminator, inside the same push and after `finish`; repeated `finish`; repeated errors with the earlier updates returned; out-of-range IDs |
| withholding | header pieces emit nothing until `\n` completes them; an incomplete UTF-8 character is withheld and then completed; a `finish`-time U+FFFD appears in `trailing` |
| markers | IDs derived from the vocabulary, including the tool and media sets |
| real vocabulary (ignored; needs `POWER_LLM_TEST_MODEL`) | the fixture's 80 token strings and all 24 specials match the real GGUF vocabulary, and every case parses identically with the real tokenizer |

The tests build a full 262,144-entry vocabulary from the fixture subset, with real special and byte tokens at their real IDs, so they need no model file.
