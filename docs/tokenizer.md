# Gemma4 text tokenizer

`src/engine/tokenizer.rs` is a pure Rust tokenizer for the Gemma 4 vocabulary (Gemma 4 and DiffusionGemma share it). It is built from the GGUF `Vocabulary` only. It has no Python, subprocess, download, or new dependency. Chat-template rendering is out of scope.

## Reference and provenance

| Item | Value |
|---|---|
| Reference tokenizer | `google/diffusiongemma-26B-A4B-it` `tokenizer.json` at revision `f7f5b7f5fa82ffc52addd066915886d497f5517b` (sha256 `cc8d3a0ce36466ccc1278bf987df5f71db1719b9ca6b4118264f45cb627bfe0f`). Not gated. |
| Oracle | Hugging Face `tokenizers` 0.23.2 (`Tokenizer.from_file`). Encode IDs were cross-checked with `transformers` 5.18.0 `AutoTokenizer`, which loads it as `GemmaTokenizer` with `add_bos_token = False`. |
| Target GGUF | `diffusiongemma-26B-A4B-it-Q4_K_M.gguf` (Unsloth revision `f4183a2c7a354128d02545752303c4354d165bf0`), `tokenizer.ggml.model = "gemma4"` |
| Fixtures | `gemma4_reference.json`, produced with Hugging Face `tokenizers` by a generator script that records versions, policies and the sha256. The fixture and its contract test need the real GGUF and are not part of the repository. |

**GGUF ↔ reference equivalence (verified):**

- All 262144 GGUF token strings equal the `tokenizer.json` vocab by ID.
- All 514906 GGUF merges equal the `tokenizer.json` merges in the same order. No merge part contains a space, so the `"left right"` GGUF encoding is unambiguous.
- No vocabulary entry is duplicated.
- The 24 `special: true` added tokens are exactly the GGUF tokens of type CONTROL (3, 17 tokens) and USER_DEFINED (4, 7 tokens). The converter marks `<|channel>`, `<channel|>`, `<|tool_call>`, `<tool_call|>`, `<|tool_response>`, `<tool_response|>` and `<|"|>` as USER_DEFINED. The reference still treats them as special.
- The 256 BYTE tokens (type 6) are `<0x00>`…`<0xFF>` at IDs 238–493. All other tokens are NORMAL (type 1).
- Scores are unused, because the model is BPE, not unigram.

## Algorithm (pinned `tokenizer.json`)

1. **Added tokens.** With `SpecialTokens::Recognize`, the 24 special tokens are matched in the *raw* text, leftmost-longest (`normalized: false`, no lstrip/rstrip/single_word). With `SpecialTokens::Plain`, nothing is matched and special-token text is ordinary text (HF `encode_special_tokens = True`).
2. **Normalizer.** In each remaining segment, U+0020 is replaced by U+2581 `▁`. That is the only normalization: no NFC/NFKC, no prefix space, and tabs, newlines, CR, NBSP, etc. are untouched. A literal `▁` in the input is therefore indistinguishable from a space.
3. **Pre-tokenizer.** The reference `Split(" ", MergedWithPrevious)` runs *after* normalization, so it never matches. Each segment is BPE-merged as **one word**. This is not whitespace splitting. Note that llama.cpp's `gemma4` pre-tokenizer splits on newline runs; that is a heuristic and is not used here.
4. **BPE.** Each character is an initial symbol if its UTF-8 string is in the vocabulary. Otherwise its UTF-8 bytes become `<0xXX>` byte tokens (byte fallback; `<unk>` is never produced). Merges are then applied repeatedly: lowest rank first, and leftmost among equal ranks (heap + linked list, the same order as HF `merge_word`).
5. **Post-processor.** Adds nothing: no BOS, no EOS.

## Public API

```rust
let tok = Tokenizer::from_vocabulary(&model.vocabulary)?;      // validates, owns its data
tok.encode(text, EncodeOptions::REFERENCE)?;                   // Vec<u32>
tok.encode(user_text, EncodeOptions::PLAIN_TEXT)?;
tok.decode(&ids, DecodeOptions::REFERENCE)?;                    // String, HF-identical
tok.decode_bytes(&ids, DecodeOptions::SKIP_SPECIAL)?;           // Vec<u8>, exact bytes
let mut s = Utf8Stream::new(); s.push(&bytes); s.finish();      // streaming UTF-8
```

`EncodeOptions { add_bos, special_tokens }` and `DecodeOptions { skip_special_tokens }` have no `Default`. Callers must choose.

| Constant | Meaning |
|---|---|
| `EncodeOptions::REFERENCE` | No BOS, specials recognized. This equals `tokenizers` and `transformers` defaults. |
| `EncodeOptions::PLAIN_TEXT` | No BOS, specials *not* recognized. Use for untrusted user text spliced into a rendered prompt. |
| `DecodeOptions::REFERENCE` | Specials rendered as text (`skip_special_tokens=False`). |
| `DecodeOptions::SKIP_SPECIAL` | Drops all 24 special tokens before decoding. |

**BOS.** `add_bos: true` prepends exactly one `bos` (ID 2). It does not deduplicate a `<bos>` already present in the text. The reference adds no BOS, because the chat template emits `<bos>` itself. GGUF `add_bos_token = true` is llama.cpp policy: llama.cpp even forces it for gemma4. It is exposed only through `Vocabulary::add_bos` and is **not** applied automatically. Rendering a chat template and then encoding with `add_bos: true` would double the BOS.

**EOS/padding/mask/unk.** These are never appended or produced by encoding. They are recognized only as literal special text under `Recognize`. Accessors: `bos() eos() padding() unknown() mask() is_special(id) token(id) token_id(str)`.

**Decoding.**

- Any out-of-range ID rejects the whole call with `Error::TokenOutOfRange`. The reference silently drops such IDs, which is a deliberate difference.
- `decode` reproduces HF `Replace(▁→" ") + ByteFallback + Fuse` exactly. Each maximal run of consecutive byte tokens becomes its text if the whole run is valid UTF-8. Otherwise it becomes one U+FFFD per byte token. Example: `<0xE2><0x82><0xAC><0xC3>` gives four U+FFFD, not `€` plus one U+FFFD.
- `skip_special_tokens` filters first, as in HF, so byte runs can join across a removed special token.
- `decode_bytes` is *not* an HF output. It is the streaming contract: byte tokens become raw bytes and `▁` becomes a space, so the output can end mid-character. `Utf8Stream` releases complete characters, holds an incomplete trailing sequence (≤3 bytes) across pushes (for example across canvas boundaries), and replaces invalid sequences like `String::from_utf8_lossy`. Its output can differ from `decode` only for invalid byte-token runs.

**Rejected vocabularies** (`Error::Vocabulary`; no approximate fallback):

- token/type/score length mismatch, or an empty vocabulary
- empty or duplicate token strings
- token types 0, 2 (UNKNOWN), or 5 (UNUSED)
- a byte spelling with a non-BYTE type, or a BYTE type without `<0xXX>` spelling
- any missing byte token
- malformed or duplicate merges, or merges whose parts or result are not in the vocabulary
- BOS/EOS/pad/unk/mask that are not special tokens
- `add_space_prefix = true`

## Tests and results

- **Synthetic unit tests** (`src/engine/tokenizer/tests.rs`, 8 tests, run by `cargo test`). They cover merge priority and ties, cross-space merging, byte fallback, both special-token policies including leftmost-longest overlap, BOS, HF byte-run decode rules, skip-joins-runs, out-of-range IDs, round trips, `Utf8Stream`, and every rejection rule. Expected IDs are hand-derived.
- **Reference fixtures** (real GGUF; not in the repository): 44 encode cases × 2 policies (plus BOS and decode checks for each) and 9 decode cases match exactly. They cover ASCII, leading/repeated/trailing/only spaces, tabs, newlines, CRLF, mixed Unicode whitespace, code, Japanese, Chinese, Hangul, Arabic, precomposed and decomposed accents, emoji with skin tone, ZWJ sequences, flags, embedded NUL, C0/DEL controls, byte-fallback characters (U+E0001, U+FFFF, U+10FFFD, …), literal `▁`, literal `<0x41>`, long runs, every control-token family, nested and partial specials, truncated, split, and invalid UTF-8 byte runs, and special skipping.
- **Special IDs** (real GGUF): the special-token set is exactly the 24 reference IDs; BOS/EOS/pad/unk/mask IDs are checked; `<eos> <pad> <mask> <unk> <turn|>` encode to their IDs only under `Recognize`; out-of-range IDs are rejected.
- **Oracle corpus** (not checked in, ~7 MB): 3000 random strings over mixed scripts, whitespace, specials, combining marks, and byte-fallback characters, plus 6 real documents and source files (up to ~180 KB as a single segment). All 3006 cases matched for both policies and for `decode`: 359,218 tokens in 0.53 s on one POWER9 thread (release, including decode).

**Not validated or out of scope:**

- chat-template rendering
- other Gemma4 GGUFs whose type marking differs (they would be rejected or need re-verification)
- performance tuning beyond the measurement above (there is no BPE word cache)
