#!/usr/bin/env python3
"""Produce data-only fixtures for the committed-output parser (src/engine/output.rs).

    python -m venv oracle && oracle/bin/pip install "transformers==5.0.0" tokenizers==0.23.2 jinja2==3.1.6
    oracle/bin/python produce.py <dir with pinned tokenizer.json + tokenizer_config.json + chat_template.jinja> \
        <gguf_chat_template.jinja> ../../../fixtures/output/gemma4_output_cases.json

Expected values never come from the Rust parser. Two sources, labelled per case:

* "oracle": transformers 5.0.0 `utils.chat_parsing_utils.recursive_parse` applied to the PINNED
  `response_schema` of google/diffusiongemma-26B-A4B-it@f7f5b7f5 (tokenizer_config.json), on
  `decode(ids, skip_special_tokens=False)` of the ids BEFORE the final terminator. Generation stops at
  generation_config `eos_token_id = [1, 106, 50]`, so the terminator is recorded separately.
* "policy": malformed / truncated / unsupported / marker-spelling inputs, where the text regex is
  unsafe (it leaks headers into content or reads ordinary text as structure). The expected value is the
  documented parser policy (docs/output.md); the regex result is recorded as `oracle_text_regex` for contrast.
"""
import hashlib
import json
import sys

import tokenizers
import transformers
from transformers import AutoTokenizer
from transformers.utils.chat_parsing_utils import recursive_parse

REVISION = "f7f5b7f5fa82ffc52addd066915886d497f5517b"
TERMINATORS = {1: "eos", 106: "turn_end"}  # 50 (<|tool_response>) is a generation stop but an unsupported tool structure


def main():
    src, gguf_template, dst = sys.argv[1], sys.argv[2], sys.argv[3]
    tok = AutoTokenizer.from_pretrained(src)
    config = json.load(open(f"{src}/tokenizer_config.json"))
    schema = config["response_schema"]
    plain = tokenizers.Tokenizer.from_file(f"{src}/tokenizer.json")
    plain.encode_special_tokens = True
    enc = lambda s: tok.encode(s, add_special_tokens=False)            # noqa: E731  specials recognized
    enc_plain = lambda s: plain.encode(s).ids                           # noqa: E731  specials as text
    ids_of = lambda *names: [tok.convert_tokens_to_ids(n) for n in names]  # noqa: E731
    CH, CC, EOS, TE, TS = ids_of("<|channel>", "<channel|>", "<eos>", "<turn|>", "<|turn>")
    HEADER = enc("thought\n")

    def oracle(ids):
        body, term = ids, None
        if ids and ids[-1] in TERMINATORS:
            body, term = ids[:-1], TERMINATORS[ids[-1]]
        parsed = recursive_parse(tok.decode(body, skip_special_tokens=False), schema)
        return {"content": parsed.get("content", ""), "reasoning": parsed.get("thinking"),
                "status": "complete" if term else "incomplete_truncated_content",
                "terminator": term}

    def regex_only(ids):
        try:
            return recursive_parse(tok.decode(ids, skip_special_tokens=False), schema)
        except ValueError as e:  # e.g. the schema's 'gemma4-tool-call' sub-parser is not implemented in 5.0.0
            return {"oracle_error": str(e)}

    observed = [100, 45518, 107, 101, 50429, 1]
    W = lambda s: enc_plain(s)  # noqa: E731  ordinary text with no special recognition
    cases = []

    def case(name, context, ids, source, expected=None, note=""):
        entry = {"name": name, "context": context, "ids": ids, "source": source, "note": note}
        if source == "oracle":
            entry["expected"] = oracle(ids)
        else:
            entry["expected"] = expected
            entry["oracle_text_regex"] = regex_only(ids)
        cases.append(entry)

    # ---- well-formed (oracle) ----
    case("observed_full_width_run", "model_turn", observed, "oracle",
         note="real full-width/default-step run: empty thought block, content 'Paris', <eos>")
    case("thinking_then_answer_turn_end", "model_turn", [CH] + HEADER + W("Let me think.\nIt is Paris.") + [CC] + W("Paris.") + [TE], "oracle")
    case("empty_thought_turn_end", "model_turn", [CH] + HEADER + [CC] + W("Hello") + [TE], "oracle")
    case("plain_no_channel", "model_turn", W("Just text, no channel.") + [EOS], "oracle",
         note="no channel markers at all: everything is content")
    case("empty_content_after_thought", "model_turn", [CH] + HEADER + W("only thinking") + [CC] + [EOS], "oracle")
    case("empty_everything", "model_turn", [EOS], "oracle")
    case("unicode_and_byte_fallback", "model_turn", [CH] + HEADER + W("日本語 \U000E0001") + [CC] + W("👨‍👩‍👧 ok") + [TE], "oracle")
    case("reasoning_with_newlines_and_spaces", "model_turn", [CH] + HEADER + W("  a\n\n b  ") + [CC] + W("\n x \n") + [EOS], "oracle")

    # ---- policy cases (regex unsafe or out of scope) ----
    P = lambda content, reasoning, status, terminator=None, error=None: {  # noqa: E731
        "content": content, "reasoning": reasoning, "status": status, "terminator": terminator, "error": error}
    case("truncated_inside_reasoning", "model_turn", [CH] + HEADER + W("still thinking"), "policy",
         P("", "still thinking", "incomplete_unclosed_reasoning"),
         "length-truncated: reasoning stays reasoning; never closed, never content")
    case("truncated_inside_header", "model_turn", [CH] + W("thou"), "policy",
         P("", None, "incomplete_unclosed_reasoning"), "header bytes withheld; nothing emitted")
    case("truncated_content_no_terminator", "model_turn", [CH] + HEADER + [CC] + W("Par"), "policy",
         P("Par", "", "incomplete_truncated_content"))
    case("eos_inside_reasoning", "model_turn", [CH] + HEADER + W("x") + [EOS], "policy",
         P("", "x", "error", error="terminator_inside_reasoning"), "no invented <channel|>")
    case("marker_spelled_as_ordinary_text", "model_turn", W("<|channel>thought\n<channel|>Paris") + [EOS], "policy",
         P("<|channel>thought\n<channel|>Paris", None, "complete", "eos"),
         "ordinary tokens spelling a marker are content; the text regex wrongly reads them as structure")
    case("thought_word_in_content", "model_turn", [CH] + HEADER + [CC] + W("thought\nis fine") + [EOS], "policy",
         P("thought\nis fine", "", "complete", "eos"))
    case("unknown_channel_name", "model_turn", [CH] + W("analysis\n") + W("x") + [CC] + W("y") + [EOS], "policy",
         P("", None, "error", error="unknown_channel_header"))
    case("adjacent_thought_spans", "model_turn", [CH] + HEADER + W("a") + [CC] + [CH] + HEADER + W("b") + [CC] + W("c") + [EOS], "policy",
         P("c", "ab", "complete", "eos"), "consecutive spans before any content: reasoning concatenated")
    case("channel_after_content", "model_turn", [CH] + HEADER + [CC] + W("c") + [CH] + HEADER + W("x") + [CC] + [EOS], "policy",
         P("c", "", "error", error="channel_after_content"))
    case("close_without_open", "model_turn", W("a") + [CC] + W("b") + [EOS], "policy",
         P("a", None, "error", error="unexpected_control"))
    case("new_turn_open_in_output", "model_turn", [CH] + HEADER + [CC] + W("a") + [TS] + W("user") + [EOS], "policy",
         P("a", "", "error", error="unexpected_control"))
    for name, special in (("bos", "<bos>"), ("pad", "<pad>"), ("think", "<|think|>"), ("mask", "<mask>")):
        case(f"unexpected_{name}", "model_turn", [CH] + HEADER + [CC] + W("a") + ids_of(special) + W("b") + [EOS], "policy",
             P("a", "", "error", error="unexpected_control"))
    case("tool_call_unsupported", "model_turn", [CH] + HEADER + [CC] + ids_of("<|tool_call>") + W("call:f{}") + ids_of("<tool_call|>") + ids_of("<|tool_response>"), "policy",
         P("", "", "error", error="unsupported_tool_structure"), "text-only engine: never content, never executed")
    case("tool_response_stop_unsupported", "model_turn", [CH] + HEADER + [CC] + W("a") + ids_of("<|tool_response>"), "policy",
         P("a", "", "error", error="unsupported_tool_structure"))
    case("quote_escape_unsupported", "model_turn", W("a") + ids_of('<|"|>') + [EOS], "policy",
         P("a", None, "error", error="unsupported_tool_structure"))
    case("media_unsupported", "model_turn", W("a") + ids_of("<|image|>") + [EOS], "policy",
         P("a", None, "error", error="unsupported_media"))
    case("invalid_utf8_bytes", "model_turn", [CH] + HEADER + [CC] + W("a") + [238 + 0xC3] + W("b") + [EOS], "policy",
         P("a�b", "", "complete", "eos"), "invalid byte sequence -> one U+FFFD per invalid sequence (lossy streaming)")
    case("truncated_utf8_at_end", "model_turn", [CH] + HEADER + [CC] + W("a") + [238 + 0xE2, 238 + 0x82], "policy",
         P("a�", "", "incomplete_truncated_content"))
    case("inside_thought_context", "inside_thought", W("prefilled reasoning") + [CC] + W("answer") + [EOS], "policy",
         P("answer", "prefilled reasoning", "complete", "eos"),
         "canonical template prefix ending '<|channel>thought\\n' (tool response + thinking): parser starts in reasoning")
    case("inside_thought_truncated", "inside_thought", W("x"), "policy",
         P("", "x", "incomplete_unclosed_reasoning"))

    # Prefix contexts from the two pinned templates (thinking on/off), rendered by transformers 5.0.0.
    canonical = open(f"{src}/chat_template.jinja").read()
    gguf = open(gguf_template).read()
    prefixes = []
    msgs = [{"role": "user", "content": "Hi"}]
    for template_name, template in (("canonical", canonical), ("gguf", gguf)):
        for thinking in (False, True):
            text = tok.apply_chat_template(msgs, chat_template=template, add_generation_prompt=True, tokenize=False,
                                           enable_thinking=thinking)
            ids = tok.encode(text, add_special_tokens=False)
            prefixes.append({"template": template_name, "enable_thinking": thinking, "tail_ids": ids[-6:],
                             "expected_context": "model_turn"})
        text = tok.apply_chat_template(msgs, chat_template=template, add_generation_prompt=False, tokenize=False)
        prefixes.append({"template": template_name, "enable_thinking": False, "add_generation_prompt": False,
                         "tail_ids": tok.encode(text, add_special_tokens=False)[-6:], "expected_context": "unsupported"})
    canonical_tool = tok.apply_chat_template(
        [{"role": "user", "content": "w?"}, {"role": "assistant", "tool_calls": [{"id": "c", "type": "function",
          "function": {"name": "f", "arguments": {}}}]}, {"role": "tool", "tool_call_id": "c", "content": "r"}],
        chat_template=canonical, add_generation_prompt=True, tokenize=False, enable_thinking=True)
    prefixes.append({"template": "canonical", "enable_thinking": True, "after_tool_response": True,
                     "tail_ids": tok.encode(canonical_tool, add_special_tokens=False)[-6:],
                     "expected_context": "inside_thought"})

    used = sorted({i for c in cases for i in c["ids"]} | {i for p in prefixes for i in p["tail_ids"]})
    out = {
        "provenance": {
            "model": "google/diffusiongemma-26B-A4B-it", "revision": REVISION,
            "tokenizer_json_sha256": hashlib.sha256(open(f"{src}/tokenizer.json", "rb").read()).hexdigest(),
            "response_schema": schema,
            "oracle": f"transformers {transformers.__version__} recursive_parse (pinned response_schema); "
                      f"tokenizers {tokenizers.__version__}",
            "model_card": "README @ revision: '<|channel>thought\\n[reasoning]<channel|>[final answer]'; disabled "
                          "thinking still emits '<|channel>thought\\n<channel|>'",
            "generation_config_eos_token_id": [1, 106, 50],
            "procedure": "tools/oracle-output/produce.py",
        },
        "vocab_subset": {str(i): tok.convert_ids_to_tokens(i) for i in used},
        "cases": cases,
        "prefixes": prefixes,
    }
    with open(dst, "w", encoding="utf-8") as f:
        json.dump(out, f, ensure_ascii=True, indent=1)
        f.write("\n")
    print(len(cases), "cases,", len(prefixes), "prefixes,", len(used), "token ids")


if __name__ == "__main__":
    main()
