use super::*;
use crate::engine::model::Vocabulary;
use serde_json::Value;

const FIXTURE: &str = include_str!("../../../fixtures/output/gemma4_output_cases.json");
const REVISION: &str = "f7f5b7f5fa82ffc52addd066915886d497f5517b";
const VOCAB: usize = 262144;

/// The 24 Gemma4 special tokens at their real IDs with their GGUF types.
const SPECIALS: [(u32, &str, i64); 24] = [
    (0, "<pad>", 3),
    (1, "<eos>", 3),
    (2, "<bos>", 3),
    (3, "<unk>", 3),
    (4, "<mask>", 3),
    (46, "<|tool>", 3),
    (47, "<tool|>", 3),
    (48, "<|tool_call>", 4),
    (49, "<tool_call|>", 4),
    (50, "<|tool_response>", 4),
    (51, "<tool_response|>", 4),
    (52, "<|\"|>", 4),
    (98, "<|think|>", 3),
    (100, "<|channel>", 4),
    (101, "<channel|>", 4),
    (105, "<|turn>", 3),
    (106, "<turn|>", 3),
    (255999, "<|image>", 3),
    (256000, "<|audio>", 3),
    (258880, "<|image|>", 3),
    (258881, "<|audio|>", 3),
    (258882, "<image|>", 3),
    (258883, "<audio|>", 3),
    (258884, "<|video|>", 3),
];

fn fixture() -> Value {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    assert_eq!(fixture["provenance"]["revision"], REVISION);
    fixture
}

/// Full-size vocabulary: real specials and byte tokens, the fixture's real
/// token strings at their real IDs, and unique private-use fillers elsewhere.
fn tokenizer() -> Tokenizer {
    let fixture = fixture();
    let mut tokens: Vec<String> = (0..VOCAB).map(|i| format!("\u{F0000}{i}")).collect();
    let mut types = vec![1i64; VOCAB];
    for b in 0..256 {
        tokens[238 + b] = format!("<0x{b:02X}>");
        types[238 + b] = 6;
    }
    for (id, text, kind) in SPECIALS {
        tokens[id as usize] = text.into();
        types[id as usize] = kind;
    }
    for (id, text) in fixture["vocab_subset"].as_object().unwrap() {
        let id: usize = id.parse().unwrap();
        assert!(
            tokens[id] == *text.as_str().unwrap() || types[id] == 1,
            "fixture disagrees with special/byte id {id}"
        );
        tokens[id] = text.as_str().unwrap().into();
    }
    let scores = vec![0.0; VOCAB];
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

fn ids(value: &Value) -> Vec<u32> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect()
}

fn context(name: &str) -> InitialContext {
    match name {
        "model_turn" => InitialContext::ModelTurn,
        "inside_thought" => InitialContext::InsideThought,
        other => panic!("context {other}"),
    }
}

fn status_name(status: Status) -> (&'static str, Option<&'static str>, Option<&'static str>) {
    match status {
        Status::Complete(Terminator::Eos) => ("complete", Some("eos"), None),
        Status::Complete(Terminator::TurnEnd) => ("complete", Some("turn_end"), None),
        Status::Incomplete(Incomplete::UnclosedReasoning) => {
            ("incomplete_unclosed_reasoning", None, None)
        }
        Status::Incomplete(Incomplete::TruncatedContent) => {
            ("incomplete_truncated_content", None, None)
        }
        Status::Error(e) => (
            "error",
            None,
            Some(match e {
                ParseError::TerminatorInsideReasoning => "terminator_inside_reasoning",
                ParseError::UnknownChannelHeader => "unknown_channel_header",
                ParseError::ChannelAfterContent => "channel_after_content",
                ParseError::UnexpectedControl(_) => "unexpected_control",
                ParseError::UnsupportedToolStructure(_) => "unsupported_tool_structure",
                ParseError::UnsupportedMedia(_) => "unsupported_media",
                ParseError::TokenOutOfRange(_) => "token_out_of_range",
            }),
        ),
    }
}

/// Parses `ids` pushed in the given chunk sizes; returns the output and all updates.
fn parse(
    t: &Tokenizer,
    ctx: InitialContext,
    ids: &[u32],
    chunks: &[usize],
) -> (Output, Vec<Update>) {
    let mut parser = OutputParser::new(t, ctx).unwrap();
    let mut updates = Vec::new();
    let mut start = 0;
    for &n in chunks {
        let end = (start + n).min(ids.len());
        match parser.push(&ids[start..end]) {
            Ok(u) => updates.extend(u),
            Err(PushError {
                error: Error::Failed(_),
                updates: before,
            }) => {
                updates.extend(before);
                break;
            }
            Err(e) => panic!("{e:?}"),
        }
        start = end;
    }
    let output = parser.finish();
    updates.extend(output.trailing.iter().cloned());
    (output, updates)
}

fn concat(updates: &[Update]) -> (String, String) {
    let (mut reasoning, mut content) = (String::new(), String::new());
    for u in updates {
        match u {
            Update::Reasoning(s) => reasoning.push_str(s),
            Update::Content(s) => content.push_str(s),
            _ => {}
        }
    }
    (reasoning, content)
}

#[test]
fn fixture_cases_match_under_every_split() {
    let t = tokenizer();
    let fixture = fixture();
    let cases = fixture["cases"].as_array().unwrap();
    assert!(cases.len() >= 31);
    for case in cases {
        let name = case["name"].as_str().unwrap();
        let input = ids(&case["ids"]);
        let ctx = context(case["context"].as_str().unwrap());
        let expected = &case["expected"];
        let mut splits: Vec<Vec<usize>> = vec![vec![input.len()], vec![1; input.len()]];
        for k in 0..=input.len() {
            splits.push(vec![k, input.len() - k]);
            for j in k..=input.len() {
                splits.push(vec![k, j - k, input.len() - j]); // every 3-way split
            }
        }
        let (reference, _) = parse(&t, ctx, &input, &[input.len()]);
        for chunks in splits {
            let (out, updates) = parse(&t, ctx, &input, &chunks);
            assert_eq!(
                out, reference,
                "{name}: chunking {chunks:?} changed the result"
            );
            let (reasoning, content) = concat(&updates);
            assert_eq!(content, out.content, "{name} {chunks:?}: content updates");
            assert_eq!(
                reasoning,
                out.reasoning.clone().unwrap_or_default(),
                "{name} {chunks:?}: reasoning updates"
            );
        }
        assert_eq!(
            reference.content,
            expected["content"].as_str().unwrap(),
            "{name}: content"
        );
        assert_eq!(
            reference.reasoning.as_deref(),
            expected["reasoning"].as_str(),
            "{name}: reasoning"
        );
        let (status, terminator, error) = status_name(reference.status);
        assert_eq!(
            status,
            expected["status"].as_str().unwrap(),
            "{name}: status"
        );
        assert_eq!(
            terminator,
            expected["terminator"].as_str(),
            "{name}: terminator"
        );
        assert_eq!(
            error,
            expected.get("error").and_then(Value::as_str),
            "{name}: error"
        );
    }
}

#[test]
fn observed_full_width_run_is_empty_reasoning_then_paris() {
    let t = tokenizer();
    let (out, updates) = parse(
        &t,
        InitialContext::ModelTurn,
        &[100, 45518, 107, 101, 50429, 1],
        &[6],
    );
    assert_eq!(
        out,
        Output {
            content: "Paris".into(),
            reasoning: Some(String::new()),
            status: Status::Complete(Terminator::Eos),
            replaced_invalid_utf8: 0,
            trailing: vec![]
        }
    );
    assert_eq!(
        updates,
        [
            Update::ReasoningStarted,
            Update::ReasoningEnded,
            Update::Content("Paris".into())
        ]
    );
}

/// Observed at long context (16K-token prompt): the model dropped the
/// `<|channel>` opener and began with `thought\n`. That is an implicit,
/// unopened reasoning span; everything after `<channel|>` is the answer.
/// Leading text that is not exactly the header stays content.
#[test]
fn dropped_channel_opener_is_an_implicit_reasoning_span() {
    let t = tokenizer();
    let ids = [45518, 107, 101, 50429, 1];
    for split in 1..=ids.len() {
        let (out, updates) = parse(&t, InitialContext::ModelTurn, &ids, &vec![split; ids.len()]);
        assert_eq!(out.content, "Paris", "split {split}");
        assert_eq!(out.reasoning, Some(String::new()));
        assert_eq!(out.status, Status::Complete(Terminator::Eos));
        assert_eq!(
            updates,
            [
                Update::ReasoningStarted,
                Update::ReasoningEnded,
                Update::Content("Paris".into())
            ]
        );
    }
    // "thought" + "Paris": not the header, so plain content.
    let (out, _) = parse(
        &t,
        InitialContext::ModelTurn,
        &[45518, 50429, 1],
        &[1, 1, 1],
    );
    assert_eq!(
        (out.content.as_str(), out.reasoning),
        ("thoughtParis", None)
    );
    assert_eq!(out.status, Status::Complete(Terminator::Eos));
    // "thought" then EOS: ambiguous prefix flushed as content.
    let (out, _) = parse(&t, InitialContext::ModelTurn, &[45518, 1], &[2]);
    assert_eq!((out.content.as_str(), out.reasoning), ("thought", None));
    // Only at the start of the turn: after content, `<channel|>` is still an error.
    let (out, _) = parse(
        &t,
        InitialContext::ModelTurn,
        &[50429, 45518, 107, 101],
        &[4],
    );
    assert_eq!(
        out.status,
        Status::Error(ParseError::UnexpectedControl(101))
    );
}

/// Gemma 4 QAT template, thinking disabled: the prompt ends with an empty,
/// closed reasoning span; the output is the answer (no implicit header).
#[test]
fn prompt_ending_in_empty_closed_thought_is_detected() {
    let t = tokenizer();
    let markers = Markers::new(&t).unwrap();
    // ... <|turn> model \n <|channel> thought \n <channel|>
    let prompt = [2, 105, 4368, 107, 100, 45518, 107, 101];
    assert_eq!(
        InitialContext::from_prompt_ids(&t, &markers, &prompt).unwrap(),
        InitialContext::AfterEmptyThought
    );
    // `<channel|>` must close exactly a model-turn thought header.
    assert!(InitialContext::from_prompt_ids(&t, &markers, &[2, 100, 45518, 107, 101]).is_err());
    assert!(InitialContext::from_prompt_ids(&t, &markers, &[2, 105, 4368, 107, 101]).is_err());
    let (out, _) = parse(&t, InitialContext::AfterEmptyThought, &[50429, 1], &[1, 1]);
    assert_eq!(
        (out.content.as_str(), out.reasoning.as_deref()),
        ("Paris", Some(""))
    );
    assert_eq!(out.status, Status::Complete(Terminator::Eos));
    // No implicit header here: a leading "thought\n" is content.
    let (out, _) = parse(
        &t,
        InitialContext::AfterEmptyThought,
        &[45518, 107, 1],
        &[3],
    );
    assert_eq!(out.content, "thought\n");
}

#[test]
#[ignore = "real vocabulary; set POWER_LLM_TEST_MODEL"]
fn actual_long_context_dropped_opener_output() {
    use crate::engine::{model::ModelDefinition, weights::ModelFile};
    let file =
        ModelFile::open(std::env::var_os("POWER_LLM_TEST_MODEL").expect("model path")).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let t = Tokenizer::from_vocabulary(&model.vocabulary).unwrap();
    // Committed IDs of an observed 12,948-token needle request.
    let ids = [
        45518, 107, 107, 101, 1579, 5635, 236772, 236812, 236771, 236771, 236771, 236772, 236917,
        106,
    ];
    let (out, _) = parse(&t, InitialContext::ModelTurn, &ids, &[ids.len()]);
    assert_eq!(out.content, "ORCA-4000-X");
    assert_eq!(out.reasoning.as_deref(), Some("\n"));
    assert!(matches!(out.status, Status::Complete(_)));
}

#[test]
fn prefix_contexts_from_both_templates() {
    let t = tokenizer();
    let markers = Markers::new(&t).unwrap();
    for prefix in fixture()["prefixes"].as_array().unwrap() {
        let tail = ids(&prefix["tail_ids"]);
        let got = InitialContext::from_prompt_ids(&t, &markers, &tail);
        match prefix["expected_context"].as_str().unwrap() {
            "unsupported" => assert_eq!(got, Err(Error::UnsupportedPrefix), "{prefix}"),
            name => assert_eq!(got, Ok(context(name)), "{prefix}"),
        }
    }
    let from = |ids: &[u32]| InitialContext::from_prompt_ids(&t, &markers, ids);
    assert_eq!(from(&[]), Err(Error::UnsupportedPrefix));
    assert_eq!(from(&[4368, 107]), Err(Error::UnsupportedPrefix)); // no special token
    assert_eq!(from(&[105, 4368]), Err(Error::UnsupportedPrefix)); // "model" without newline
    assert_eq!(from(&[105, 4368, 107, 4368]), Err(Error::UnsupportedPrefix)); // extra text after the header
    // Spelling "<|turn>model\n" as ORDINARY text is not a generation prefix.
    assert_eq!(
        from(&[106, 236820, 236909, 887, 236813, 4368, 107]),
        Err(Error::UnsupportedPrefix)
    );
}

#[test]
fn termination_errors_and_idempotent_finish() {
    let t = tokenizer();
    let err = |error, updates: Vec<Update>| Err(PushError { error, updates });
    let mut p = OutputParser::new(&t, InitialContext::ModelTurn).unwrap();
    p.push(&[50429, 1]).unwrap();
    let first = p.finish();
    assert_eq!(p.push(&[50429]), err(Error::InputAfterTermination, vec![]));
    assert_eq!(p.push(&[]), Ok(vec![]));
    assert_eq!(p.finish(), first);
    // Input after a terminator inside the SAME push is rejected; earlier text is returned and kept.
    let mut p = OutputParser::new(&t, InitialContext::ModelTurn).unwrap();
    assert_eq!(
        p.push(&[50429, 106, 50429]),
        err(
            Error::InputAfterTermination,
            vec![Update::Content("Paris".into())]
        )
    );
    assert_eq!(p.finish().content, "Paris");
    // A truncated stream: finish, then late input is rejected and the result is stable.
    let mut p = OutputParser::new(&t, InitialContext::ModelTurn).unwrap();
    p.push(&[50429]).unwrap();
    let truncated = p.finish();
    assert_eq!(
        truncated.status,
        Status::Incomplete(Incomplete::TruncatedContent)
    );
    assert_eq!(p.push(&[1]), err(Error::InputAfterTermination, vec![]));
    assert_eq!(p.finish(), truncated);
    // After a parse error every push repeats it.
    let mut p = OutputParser::new(&t, InitialContext::ModelTurn).unwrap();
    let failed = Error::Failed(ParseError::UnexpectedControl(2));
    assert_eq!(
        p.push(&[50429, 2]),
        err(failed.clone(), vec![Update::Content("Paris".into())])
    );
    assert_eq!(p.push(&[50429]), err(failed, vec![]));
    assert_eq!(
        p.finish().status,
        Status::Error(ParseError::UnexpectedControl(2))
    );
    // Out-of-range IDs.
    let mut p = OutputParser::new(&t, InitialContext::ModelTurn).unwrap();
    assert_eq!(
        p.push(&[VOCAB as u32]),
        err(
            Error::Failed(ParseError::TokenOutOfRange(VOCAB as u32)),
            vec![]
        )
    );
}

#[test]
fn headers_and_utf8_are_withheld_until_unambiguous() {
    let t = tokenizer();
    let mut p = OutputParser::new(&t, InitialContext::ModelTurn).unwrap();
    assert_eq!(p.push(&[100]).unwrap(), []); // <|channel>: nothing yet
    assert_eq!(p.push(&[45518]).unwrap(), []); // "thought": header still incomplete
    assert_eq!(p.push(&[107]).unwrap(), [Update::ReasoningStarted]); // "\n" completes the header
    assert_eq!(p.push(&[101]).unwrap(), [Update::ReasoningEnded]);
    assert_eq!(p.push(&[238 + 0xE2, 238 + 0x82]).unwrap(), []); // incomplete UTF-8 withheld
    assert_eq!(
        p.push(&[238 + 0xAC]).unwrap(),
        [Update::Content("\u{20AC}".into())]
    );
    p.push(&[1]).unwrap();
    assert_eq!(p.finish().replaced_invalid_utf8, 0);
    // An unterminated tail is released only by finish, as one U+FFFD in `trailing`.
    let mut p = OutputParser::new(&t, InitialContext::ModelTurn).unwrap();
    assert_eq!(
        p.push(&[50429, 238 + 0xE2]).unwrap(),
        [Update::Content("Paris".into())]
    );
    let out = p.finish();
    assert_eq!(out.trailing, [Update::Content("\u{FFFD}".into())]);
    assert_eq!(
        (out.content.as_str(), out.replaced_invalid_utf8),
        ("Paris\u{FFFD}", 1)
    );
    assert_eq!(p.finish(), out);
}

#[test]
fn markers_are_checked_against_the_vocabulary() {
    let t = tokenizer();
    let m = Markers::new(&t).unwrap();
    assert_eq!(
        (
            m.channel_open,
            m.channel_close,
            m.turn_open,
            m.turn_close,
            m.eos
        ),
        (100, 101, 105, 106, 1)
    );
    assert_eq!(m.tool, [46, 47, 48, 49, 50, 51, 52]);
    assert_eq!(
        m.media,
        [255999, 258882, 256000, 258883, 258880, 258881, 258884]
    );
}

#[test]
#[ignore = "set POWER_LLM_TEST_MODEL to the target GGUF"]
fn actual_vocabulary_matches_fixture_subset_and_cases() {
    let file = crate::engine::weights::ModelFile::open(
        std::env::var_os("POWER_LLM_TEST_MODEL").expect("model path"),
    )
    .unwrap();
    let model = crate::engine::model::ModelDefinition::bind(file.inventory()).unwrap();
    let real = Tokenizer::from_vocabulary(&model.vocabulary).unwrap();
    let fixture = fixture();
    for (id, text) in fixture["vocab_subset"].as_object().unwrap() {
        assert_eq!(real.token(id.parse().unwrap()), text.as_str(), "id {id}");
    }
    for (id, text, _) in SPECIALS {
        assert_eq!(real.token(id), Some(text));
        assert!(real.is_special(id));
    }
    let synthetic = tokenizer();
    for case in fixture["cases"].as_array().unwrap() {
        let input = ids(&case["ids"]);
        let ctx = context(case["context"].as_str().unwrap());
        assert_eq!(
            parse(&real, ctx, &input, &[input.len()]).0,
            parse(&synthetic, ctx, &input, &[input.len()]).0,
            "{}",
            case["name"]
        );
    }
    println!(
        "{} cases identical on the real vocabulary",
        fixture["cases"].as_array().unwrap().len()
    );
}
