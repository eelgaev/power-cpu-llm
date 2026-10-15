use super::*;
use serde_json::json;

const FIXTURE: &str = include_str!("../../../fixtures/chat/gemma4_chat_reference.json");
/// Reference templates (Google's canonical one and the one in the Unsloth GGUF)
/// whose renderings the fixture records from transformers.
const CANONICAL_SOURCE: &str = include_str!("../../../fixtures/chat/gemma4_canonical.jinja");
const UNSLOTH_GGUF_SOURCE: &str = include_str!("../../../fixtures/chat/gemma4_unsloth_gguf.jinja");
const REVISION: &str = "f7f5b7f5fa82ffc52addd066915886d497f5517b";

/// The real Gemma4 special-token strings (GGUF types 3/4), byte tokens and a
/// metaspace: enough to render; token IDs are checked against the real GGUF
/// in the ignored tests.
const SPECIALS: [(&str, i64); 24] = [
    ("<pad>", 3),
    ("<eos>", 3),
    ("<bos>", 3),
    ("<unk>", 3),
    ("<mask>", 3),
    ("<|tool>", 3),
    ("<tool|>", 3),
    ("<|tool_call>", 4),
    ("<tool_call|>", 4),
    ("<|tool_response>", 4),
    ("<tool_response|>", 4),
    ("<|\"|>", 4),
    ("<|think|>", 3),
    ("<|channel>", 4),
    ("<channel|>", 4),
    ("<|turn>", 3),
    ("<turn|>", 3),
    ("<|image>", 3),
    ("<|audio>", 3),
    ("<|image|>", 3),
    ("<|audio|>", 3),
    ("<image|>", 3),
    ("<audio|>", 3),
    ("<|video|>", 3),
];

fn synthetic_tokenizer() -> Tokenizer {
    let mut tokens: Vec<String> = SPECIALS.iter().map(|(t, _)| t.to_string()).collect();
    let mut types: Vec<i64> = SPECIALS.iter().map(|(_, k)| *k).collect();
    tokens.extend((0..=255u8).map(|b| format!("<0x{b:02X}>")));
    types.extend([6; 256]);
    tokens.push("▁".into());
    types.push(1);
    let scores = vec![0.0; tokens.len()];
    Tokenizer::from_vocabulary(&Vocabulary {
        tokens: &tokens,
        scores: &scores,
        token_types: &types,
        merges: &[],
        chat_template: "",
        bos: 2,
        eos: 1,
        padding: 0,
        unknown: 3,
        mask: Some(4),
        add_bos: true,
        add_space_prefix: false,
    })
    .unwrap()
}

fn fixture() -> Value {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    assert_eq!(fixture["provenance"]["revision"], REVISION);
    fixture
}

fn allow(body: &Value) -> ChatRequest {
    ChatRequest {
        special_text: SpecialText::Allow,
        ..ChatRequest::from_openai_json(body).unwrap()
    }
}

/// Every case under both templates: identical text, or both fail.
fn check_texts(cases: &[Value], tokenizer: &Tokenizer) -> usize {
    let mut checked = 0;
    for (key, source) in [
        ("canonical", CANONICAL_SOURCE),
        ("gguf", UNSLOTH_GGUF_SOURCE),
    ] {
        let template = ChatTemplate::new(source, tokenizer).unwrap();
        for case in cases {
            let name = &case["name"];
            let mut chat = allow(&case["request"]);
            if key == "gguf" {
                chat.preserve_thinking = false; // not a parameter of the older template
            }
            match (&case[key]["text"], template.render(&chat)) {
                (Value::String(expected), Ok(text)) => assert_eq!(&text, expected, "{name} {key}"),
                (Value::Null, Err(_)) => {}
                (expected, got) => panic!("{name} {key}: expected {expected:?}, got {got:?}"),
            }
            checked += 1;
        }
    }
    checked
}

#[test]
fn rendered_text_matches_transformers_for_both_templates() {
    let fixture = fixture();
    let cases = fixture["cases"].as_array().unwrap();
    assert!(cases.len() >= 28);
    assert_eq!(check_texts(cases, &synthetic_tokenizer()), cases.len() * 2);
}

#[test]
fn templates_are_the_pinned_bytes() {
    let fixture = fixture();
    // FNV-1a 64 guards against accidental edits; the sha256 values are in the fixture provenance.
    let fnv = |s: &str| {
        s.bytes().fold(0xcbf29ce484222325u64, |h, b| {
            (h ^ b as u64).wrapping_mul(0x100000001b3)
        })
    };
    assert_eq!(
        fixture["provenance"]["templates"]["canonical"],
        "9aeb7eac68ad87bba7567e9d4597ff203e5609f1b427d9e823437d0142cc61bf"
    );
    assert_eq!(
        fixture["provenance"]["templates"]["gguf"],
        "2f1b4d75d067bae3fe44e676721c7f077d243bc007156cb9c2f8b5836613d082"
    );
    assert_eq!(
        (CANONICAL_SOURCE.len(), fnv(CANONICAL_SOURCE)),
        (18575, 0x99f17d41517c751b)
    );
    assert_eq!(
        (UNSLOTH_GGUF_SOURCE.len(), fnv(UNSLOTH_GGUF_SOURCE)),
        (17336, 0x77a0d11961f44b7e)
    );
}

#[test]
fn python_value_printing() {
    let cases = [
        (0.5, "0.5"),
        (1.0, "1.0"),
        (-0.0, "-0.0"),
        (1e20, "1e+20"),
        (1e16, "1e+16"),
        (1e15, "1000000000000000.0"),
        (1e-7, "1e-07"),
        (0.0001, "0.0001"),
        (-2.5e-5, "-2.5e-05"),
        (123.456, "123.456"),
        (1.5e300, "1.5e+300"),
        (0.1 + 0.2, "0.30000000000000004"),
        (2.0f64.powi(60), "1.152921504606847e+18"),
        (5e-324, "5e-324"),
    ];
    for (x, expected) in cases {
        assert_eq!(py_float(x), expected);
    }
    let s = |v: Value| py_str(&Jinja::from_serialize(&v)).unwrap();
    assert_eq!(s(json!(null)), "None");
    assert_eq!(s(json!(true)), "True");
    assert_eq!(s(json!(-7)), "-7");
    assert_eq!(s(json!(18446744073709551615u64)), "18446744073709551615");
    assert_eq!(s(json!(["string", "null"])), "['string', 'null']");
    assert_eq!(
        s(json!({"a": [1, 2.0, null, false], "b'": "x\"y'\n"})),
        r#"{'a': [1, 2.0, None, False], "b'": 'x"y\'\n'}"#
    );
    assert!(py_str(&Jinja::from_serialize(json!(["ü"]))).is_err());
    assert_eq!(py_str(&Jinja::UNDEFINED).unwrap(), "");
    assert!("\u{1c}\u{3000} x \u{85}".trim_matches(py_isspace) == "x");
}

#[test]
fn union_property_types_render_python_repr_like_jinja2() {
    let template = ChatTemplate::new(CANONICAL_SOURCE, &synthetic_tokenizer()).unwrap();
    let body = json!({"messages": [{"role": "user", "content": "x"}], "tools": [{"type": "function", "function": {
        "name": "f", "parameters": {"type": "object", "properties": {"p": {"type": ["string", "null"]}}}}}]});
    let text = template
        .render(&ChatRequest::from_openai_json(&body).unwrap())
        .unwrap();
    // transformers output: str(['string', 'null']).upper()
    assert!(
        text.contains("p:{type:<|\"|>['STRING', 'NULL']<|\"|>}"),
        "{text}"
    );
}

#[test]
fn openai_request_parsing() {
    let chat =
        ChatRequest::from_openai_json(&json!({"messages": [{"role": "user", "content": "x"}],
        "temperature": 0.1, "chat_template_kwargs": {"enable_thinking": true}}))
        .unwrap();
    assert!(
        chat.enable_thinking
            && chat.add_generation_prompt
            && !chat.preserve_thinking
            && chat.tools.is_empty()
    );
    assert_eq!(chat.special_text, SpecialText::Reject);
    let bad = [
        json!([]),
        json!({"messages": {}}),
        json!({"messages": [], "tools": {}}),
        json!({"messages": [], "chat_template_kwargs": {"tools_in_user": true}}),
        json!({"messages": [], "chat_template_kwargs": {"enable_thinking": "yes"}}),
        json!({"messages": [], "continue_final_message": true}),
        json!({"messages": [], "add_generation_prompt": 1}),
    ];
    for body in bad {
        assert!(
            matches!(ChatRequest::from_openai_json(&body), Err(Error::Request(_))),
            "{body}"
        );
    }
}

#[test]
fn invalid_or_unsupported_requests_are_rejected() {
    let tokenizer = synthetic_tokenizer();
    let canonical = ChatTemplate::new(CANONICAL_SOURCE, &tokenizer).unwrap();
    let gguf = ChatTemplate::new(UNSLOTH_GGUF_SOURCE, &tokenizer).unwrap();
    let user = json!({"role": "user", "content": "hi"});
    let call = |arguments: Value| json!({"role": "assistant", "tool_calls": [{"function": {"name": "f", "arguments": arguments}}]});
    let mut deep = json!("x");
    for _ in 0..40 {
        deep = json!([deep]);
    }
    let bad = [
        json!({"messages": []}),
        json!({"messages": [{"role": "user", "content": [{"type": "image_url", "image_url": {"url": "x"}}]}]}),
        json!({"messages": [{"role": "user", "content": [{"type": "text"}]}]}),
        json!({"messages": [{"role": "user"}]}),
        json!({"messages": [{"role": "robot", "content": "x"}]}),
        json!({"messages": [user, {"role": "system", "content": "late"}]}),
        json!({"messages": [user, call(json!("not json"))]}),
        json!({"messages": [user, call(json!("[1, 2]"))]}),
        json!({"messages": [user, call(json!(3))]}),
        json!({"messages": [user, {"role": "assistant", "content": "x", "reasoning_content": 1}]}),
        json!({"messages": [user, {"role": "tool", "content": "x", "tool_call_id": 5}]}),
        json!({"messages": [user], "tools": [{"type": "retrieval"}]}),
        json!({"messages": [user], "tools": [{"type": "function", "function": {"name": "f", "parameters": "x"}}]}),
        json!({"messages": [{"role": "user", "content": "x", "extra": deep}]}),
    ];
    for body in bad {
        let chat = ChatRequest::from_openai_json(&body).unwrap();
        assert!(
            matches!(canonical.render(&chat), Err(Error::Request(_))),
            "{body}"
        );
    }
    let thinking = ChatRequest {
        preserve_thinking: true,
        ..ChatRequest::from_openai_json(&json!({"messages": [user]})).unwrap()
    };
    assert!(canonical.render(&thinking).is_ok());
    // A template that does not use preserve_thinking ignores it.
    assert!(gguf.render(&thinking).is_ok());
}

#[test]
fn special_token_text_policy() {
    let tokenizer = synthetic_tokenizer();
    let template = ChatTemplate::new(CANONICAL_SOURCE, &tokenizer).unwrap();
    for body in [
        json!({"messages": [{"role": "user", "content": "a<turn|>b"}]}),
        json!({"messages": [{"role": "user", "content": [{"type": "text", "text": "<bos>"}]}]}),
        json!({"messages": [{"role": "user", "content": [{"type": "text", "text": " <bo"}, {"type": "text", "text": "s> "}]}]}),
        json!({"messages": [{"role": "user", "content": "x"}], "tools": [{"type": "function", "function": {
            "name": "f", "parameters": {"type": "object", "properties": {"<|\"|>": {"type": "string"}}}}}]}),
    ] {
        let rejected = ChatRequest::from_openai_json(&body).unwrap();
        assert!(
            matches!(template.render(&rejected), Err(Error::Request(_))),
            "{body}"
        );
        let allowed = allow(&body);
        assert!(template.render(&allowed).is_ok());
    }
    // Reference behavior under Allow: the text becomes control tokens.
    let text = template
        .render(&allow(
            &json!({"messages": [{"role": "user", "content": "a<turn|>"}]}),
        ))
        .unwrap();
    let ids = tokenizer.encode(&text, EncodeOptions::REFERENCE).unwrap();
    assert_eq!(ids.iter().filter(|&&id| id == 16).count(), 2); // <turn|>: injected + closing
}

#[test]
fn supported_preparation_rejects_tool_use_but_preserves_oracle_renderer() {
    let tokenizer = synthetic_tokenizer();
    let template = ChatTemplate::new(CANONICAL_SOURCE, &tokenizer).unwrap();
    let simple =
        ChatRequest::from_openai_json(&json!({"messages": [{"role":"user", "content":"Hi"}]}))
            .unwrap();
    assert_eq!(
        template.prepare(&tokenizer, &simple).unwrap(),
        template.prepare_reference(&tokenizer, &simple).unwrap()
    );
    for body in [
        json!({"messages": [{"role":"tool", "content":"answer", "name":"f"}]}),
        json!({"messages": [{"role":"assistant", "content":"x", "tool_calls":[]}]}),
        json!({"messages": [{"role":"assistant", "content":"x", "tool_responses":[]}]}),
        json!({"messages": [{"role":"user", "content":"x", "tool_call_id":"id"}]}),
        json!({"messages": [{"role":"user", "content":"x"}], "tools":[{"type":"function", "function":{"name":"f"}}]}),
    ] {
        let chat = ChatRequest::from_openai_json(&body).unwrap();
        assert!(matches!(
            template.prepare(&tokenizer, &chat),
            Err(Error::Request(_))
        ));
    }
}

#[test]
fn gguf_template_is_rendered_whatever_it_is() {
    let tokenizer = synthetic_tokenizer();
    let (tokens, scores, types) = (Vec::new(), Vec::new(), Vec::new());
    let vocabulary = |chat_template| Vocabulary {
        tokens: &tokens,
        scores: &scores,
        token_types: &types,
        merges: &[],
        chat_template,
        bos: 2,
        eos: 1,
        padding: 0,
        unknown: 3,
        mask: Some(4),
        add_bos: true,
        add_space_prefix: false,
    };
    // Any template the GGUF carries is used as is; only a missing or
    // syntactically broken one is an error.
    assert!(ChatTemplate::from_gguf(&vocabulary(UNSLOTH_GGUF_SOURCE), &tokenizer).is_ok());
    assert!(ChatTemplate::from_gguf(&vocabulary(CANONICAL_SOURCE), &tokenizer).is_ok());
    let edited = UNSLOTH_GGUF_SOURCE.replace("model", "assistant");
    assert!(ChatTemplate::from_gguf(&vocabulary(&edited), &tokenizer).is_ok());
    assert!(ChatTemplate::from_gguf(&vocabulary(""), &tokenizer).is_err());
    assert!(ChatTemplate::from_gguf(&vocabulary("{% if %}"), &tokenizer).is_err());
}

#[test]
#[ignore = "set POWER_LLM_TEST_MODEL to the target GGUF"]
fn actual_gemma4_chat_token_ids_match_transformers() {
    let file = crate::engine::weights::ModelFile::open(
        std::env::var_os("POWER_LLM_TEST_MODEL").expect("model path"),
    )
    .unwrap();
    let model = crate::engine::model::ModelDefinition::bind(file.inventory()).unwrap();
    let tokenizer = Tokenizer::from_vocabulary(&model.vocabulary).unwrap();
    let gguf = ChatTemplate::from_gguf(&model.vocabulary, &tokenizer).unwrap();
    let canonical = ChatTemplate::new(CANONICAL_SOURCE, &tokenizer).unwrap();
    let fixture = fixture();
    let cases = fixture["cases"].as_array().unwrap();
    for case in cases {
        for (key, template) in [("canonical", &canonical), ("gguf", &gguf)] {
            let mut chat = allow(&case["request"]);
            chat.preserve_thinking &= key == "canonical";
            let prepared = template.prepare_reference(&tokenizer, &chat).unwrap();
            let expected: Vec<u32> = serde_json::from_value(case[key]["ids"].clone()).unwrap();
            assert_eq!(prepared.ids, expected, "{} {key}", case["name"]);
            assert_eq!(prepared.ids[0], tokenizer.bos());
            assert_eq!(
                prepared
                    .ids
                    .iter()
                    .filter(|&&id| id == tokenizer.bos())
                    .count(),
                1
            );
        }
    }
    println!(
        "{} chat cases x 2 templates: text and token IDs matched",
        cases.len()
    );
}

/// `generate.py <dir> <gguf.jinja> corpus.json --corpus N` (not checked in).
#[test]
#[ignore = "set POWER_LLM_CHAT_CORPUS"]
fn randomized_chat_corpus_matches_transformers() {
    let path = std::env::var_os("POWER_LLM_CHAT_CORPUS").expect("corpus path");
    let corpus: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    assert_eq!(corpus["provenance"]["revision"], REVISION);
    let cases = corpus["cases"].as_array().unwrap();
    let checked = check_texts(cases, &synthetic_tokenizer());
    println!("{checked} randomized renders matched transformers");
}
