use super::*;
use crate::engine::{
    attention::tests::fixture::Fixture,
    generation::{GenerationConfig, GenerationSession},
    model::{ModelDefinition, Vocabulary},
    text::{ReferenceLimits, ReferenceText},
    weights::ModelFile,
};

fn tokenizer() -> Tokenizer {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/output/gemma4_output_cases.json"
    ))
    .unwrap();
    let mut tokens: Vec<String> = (0..262144).map(|index| format!("unused{index}")).collect();
    let mut types = vec![1; tokens.len()];
    for (id, text) in [
        (0, "<pad>"),
        (1, "<eos>"),
        (2, "<bos>"),
        (3, "<unk>"),
        (4, "<mask>"),
        (46, "<|tool>"),
        (47, "<tool|>"),
        (48, "<|tool_call>"),
        (49, "<tool_call|>"),
        (50, "<|tool_response>"),
        (51, "<tool_response|>"),
        (52, "<|\"|>"),
        (98, "<|think|>"),
        (100, "<|channel>"),
        (101, "<channel|>"),
        (105, "<|turn>"),
        (106, "<turn|>"),
        (255999, "<|image>"),
        (256000, "<|audio>"),
        (258880, "<|image|>"),
        (258881, "<|audio|>"),
        (258882, "<image|>"),
        (258883, "<audio|>"),
        (258884, "<|video|>"),
    ] {
        tokens[id] = text.into();
        types[id] = 3;
    }
    for byte in 0..256 {
        tokens[238 + byte] = format!("<0x{byte:02X}>");
        types[238 + byte] = 6;
    }
    for (id, text) in fixture["vocab_subset"].as_object().unwrap() {
        let index: usize = id.parse().unwrap();
        let text = text.as_str().unwrap();
        tokens[index] = text.into();
        if text.starts_with('<') && text.ends_with('>') {
            types[index] = if text.starts_with("<0x") { 6 } else { 3 };
        }
    }
    Tokenizer::from_vocabulary(&Vocabulary {
        scores: &vec![0.0; tokens.len()],
        tokens: &tokens,
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

fn result(tokens: &[u32], reason: FinishReason) -> generation::Output {
    generation::Output {
        tokens: tokens.to_vec(),
        finish_reason: Some(reason),
        usage: Usage {
            prompt_tokens: 21,
            completion_tokens: tokens.len(),
            decoder_passes: 3,
            encoder_passes: 1,
            completed_canvases: 1,
        },
    }
}

fn commit(index: usize, tokens: &[u32], reason: Option<FinishReason>) -> Event {
    Event::Commit {
        canvas_index: index,
        tokens: tokens.to_vec(),
        finish_reason: reason,
    }
}

#[test]
fn finish_mapping_is_exhaustive() {
    let statuses = [
        Status::Complete(output::Terminator::Eos),
        Status::Complete(output::Terminator::TurnEnd),
        Status::Incomplete(output::Incomplete::TruncatedContent),
        Status::Incomplete(output::Incomplete::UnclosedReasoning),
        Status::Error(output::ParseError::UnsupportedToolStructure(50)),
    ];
    for reason in [
        None,
        Some(FinishReason::Eos),
        Some(FinishReason::Length),
        Some(FinishReason::Cancelled),
        Some(FinishReason::Failed),
    ] {
        for status in statuses {
            let expected = match (reason, status) {
                (None, _) => CompletionFinish::Error(CompletionError::Unfinished),
                (Some(FinishReason::Cancelled), _) => {
                    CompletionFinish::Error(CompletionError::Cancelled)
                }
                (Some(FinishReason::Failed), _) => CompletionFinish::Error(CompletionError::Failed),
                (_, Status::Error(error)) => CompletionFinish::Error(CompletionError::Parse(error)),
                (_, Status::Incomplete(output::Incomplete::UnclosedReasoning)) => {
                    CompletionFinish::Error(CompletionError::UnclosedReasoning)
                }
                (Some(FinishReason::Eos), Status::Complete(_)) => CompletionFinish::Stop,
                (
                    Some(FinishReason::Length),
                    Status::Incomplete(output::Incomplete::TruncatedContent),
                ) => CompletionFinish::Length,
                _ => CompletionFinish::Error(CompletionError::UnexpectedTermination),
            };
            assert_eq!(map_finish(reason, status), expected);
        }
    }
}

#[test]
fn observed_run_all_splits_and_json_privacy() {
    let tokenizer = tokenizer();
    let ids = [100, 45518, 107, 101, 50429, 1];
    for split in 0..=ids.len() {
        for reasoning in [false, true] {
            let mut assembler =
                CompletionAssembler::with_context(&tokenizer, InitialContext::ModelTurn).unwrap();
            assembler.observe(&commit(0, &ids[..split], None));
            assembler.observe(&commit(1, &ids[split..], Some(FinishReason::Eos)));
            let original = result(&ids, FinishReason::Eos);
            let completion = assembler.finish(&original, reasoning);
            assert_eq!(completion.content, "Paris");
            assert_eq!(completion.reasoning.as_deref(), reasoning.then_some(""));
            assert_eq!(completion.finish, CompletionFinish::Stop);
            assert_eq!(completion.committed_ids, original.tokens);
            assert_eq!(completion.usage, original.usage);
            let json = completion.to_json();
            assert_eq!(json["object"], "chat.completion");
            assert_eq!(json["choices"][0]["message"]["content"], "Paris");
            assert_eq!(
                json["choices"][0]["message"]
                    .get("reasoning_content")
                    .is_some(),
                reasoning
            );
            assert_eq!(json["choices"][0]["finish_reason"], "stop");
            assert_eq!(json["usage"]["total_tokens"], 27);
        }
    }
}

#[test]
fn length_turn_end_and_usage_overflow_have_explicit_json_results() {
    let tokenizer = tokenizer();
    for (ids, reason, expected) in [
        (vec![50429], FinishReason::Length, "length"),
        (vec![50429, 106], FinishReason::Eos, "stop"),
    ] {
        let mut assembler =
            CompletionAssembler::with_context(&tokenizer, InitialContext::ModelTurn).unwrap();
        assembler.observe(&commit(0, &ids, Some(reason)));
        let mut completion = assembler.finish(&result(&ids, reason), false);
        assert_eq!(
            completion.to_json()["choices"][0]["finish_reason"],
            expected
        );
        assert_eq!(
            completion.to_json()["choices"][0]["message"]["role"],
            "assistant"
        );
        completion.usage.prompt_tokens = usize::MAX;
        assert!(completion.to_json().get("error").is_some());
        assert!(completion.to_json().get("choices").is_none());
    }
}

#[test]
fn utf8_bytes_span_commits_without_crossing_reasoning_boundaries() {
    let tokenizer = tokenizer();
    let euro = [238 + 0xE2, 238 + 0x82, 238 + 0xAC];
    let ids = [
        100, 45518, 107, euro[0], euro[1], euro[2], 101, euro[0], euro[1], euro[2], 1,
    ];
    for split in 0..=ids.len() {
        let mut assembler =
            CompletionAssembler::with_context(&tokenizer, InitialContext::ModelTurn).unwrap();
        assembler.observe(&commit(0, &ids[..split], None));
        assembler.observe(&commit(1, &ids[split..], Some(FinishReason::Eos)));
        let completion = assembler.finish(&result(&ids, FinishReason::Eos), true);
        assert_eq!(completion.content, "€");
        assert_eq!(completion.reasoning.as_deref(), Some("€"));
        assert_eq!(completion.replaced_invalid_utf8, 0);
        assert_eq!(completion.finish, CompletionFinish::Stop);
        assert_eq!(
            completion.to_json()["choices"][0]["message"]["content"],
            "€"
        );
    }
    let ids = [100, 45518, 107, euro[0], 101, 50429, 1];
    let mut assembler =
        CompletionAssembler::with_context(&tokenizer, InitialContext::ModelTurn).unwrap();
    for (index, id) in ids.iter().enumerate() {
        assembler.observe(&commit(index, &[*id], None));
    }
    let completion = assembler.finish(&result(&ids, FinishReason::Eos), true);
    assert_eq!(completion.content, "Paris");
    assert_eq!(completion.reasoning.as_deref(), Some("\u{FFFD}"));
    assert_eq!(completion.replaced_invalid_utf8, 1);
}

#[test]
fn truncated_utf8_tail_is_finalized_on_its_original_channel() {
    let tokenizer = tokenizer();
    for reasoning in [false, true] {
        let mut ids = if reasoning {
            vec![100, 45518, 107]
        } else {
            Vec::new()
        };
        ids.extend([238 + 0xE2, 238 + 0x82]);
        let mut assembler =
            CompletionAssembler::with_context(&tokenizer, InitialContext::ModelTurn).unwrap();
        assembler.observe(&commit(0, &ids, Some(FinishReason::Length)));
        let completion = assembler.finish(&result(&ids, FinishReason::Length), true);
        assert_eq!(completion.replaced_invalid_utf8, 1);
        if reasoning {
            assert_eq!(completion.content, "");
            assert_eq!(completion.reasoning.as_deref(), Some("\u{FFFD}"));
            assert_eq!(
                completion.finish,
                CompletionFinish::Error(CompletionError::UnclosedReasoning)
            );
            assert!(completion.to_json().get("choices").is_none());
        } else {
            assert_eq!(completion.content, "\u{FFFD}");
            assert_eq!(completion.reasoning, None);
            assert_eq!(completion.finish, CompletionFinish::Length);
            assert_eq!(
                completion.to_json()["choices"][0]["message"]["content"],
                "\u{FFFD}"
            );
        }
    }
}

#[test]
fn errors_never_serialize_partial_content_as_a_choice() {
    let tokenizer = tokenizer();
    for (ids, reason) in [
        (vec![50429, 50], FinishReason::Eos),
        (vec![100, 45518, 107, 50429], FinishReason::Length),
        (vec![50429], FinishReason::Cancelled),
        (vec![50429], FinishReason::Failed),
    ] {
        let mut assembler =
            CompletionAssembler::with_context(&tokenizer, InitialContext::ModelTurn).unwrap();
        assembler.observe(&commit(0, &ids, Some(reason)));
        let completion = assembler.finish(&result(&ids, reason), true);
        assert!(matches!(completion.finish, CompletionFinish::Error(_)));
        assert!(completion.to_json().get("error").is_some());
        assert!(completion.to_json().get("choices").is_none());
    }
}

#[test]
fn commit_order_and_result_mismatch_are_errors() {
    let tokenizer = tokenizer();
    let mut assembler =
        CompletionAssembler::with_context(&tokenizer, InitialContext::ModelTurn).unwrap();
    assembler.observe(&commit(1, &[50429], None));
    assert_eq!(
        assembler
            .finish(&result(&[50429], FinishReason::Length), false)
            .finish,
        CompletionFinish::Error(CompletionError::CommitOrder)
    );
    let assembler =
        CompletionAssembler::with_context(&tokenizer, InitialContext::ModelTurn).unwrap();
    assert_eq!(
        assembler
            .finish(&result(&[50429], FinishReason::Length), false)
            .finish,
        CompletionFinish::Error(CompletionError::CommitmentMismatch)
    );
}

#[test]
fn tiny_generation_is_unchanged_and_matches_direct_parsing() {
    let fixture = Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let runtime = ReferenceText::new_batched(&file, &model, ReferenceLimits::default()).unwrap();
    let tokenizer = tokenizer();
    let config = GenerationConfig {
        max_new_tokens: 6,
        eos_ids: Vec::new(),
        encoder_chunk_tokens: 2,
        denoising: crate::engine::diffusion::DenoisingConfig {
            max_steps: 2,
            ..Default::default()
        },
        ..Default::default()
    };
    let mut baseline = GenerationSession::new(&runtime, &[2, 5], config.clone()).unwrap();
    while !baseline.is_finished() {
        baseline.advance().unwrap();
    }
    let mut session = GenerationSession::new(&runtime, &[2, 5], config).unwrap();
    let mut assembler =
        CompletionAssembler::with_context(&tokenizer, InitialContext::ModelTurn).unwrap();
    while !session.is_finished() {
        assembler.observe(&session.advance().unwrap());
    }
    let original = session.result().unwrap();
    assert_eq!(original.tokens, baseline.result().unwrap().tokens);
    assert_eq!(original.usage, baseline.result().unwrap().usage);
    assert_eq!(session.cached_tokens(), baseline.cached_tokens());
    let completion = assembler.finish(original, true);
    let mut direct = OutputParser::new(&tokenizer, InitialContext::ModelTurn).unwrap();
    let _ = direct.push(&original.tokens);
    let expected = direct.finish();
    assert_eq!(completion.content, expected.content);
    assert_eq!(completion.reasoning, expected.reasoning);
    assert_eq!(completion.parse_status, expected.status);
    assert_eq!(completion.committed_ids, original.tokens);
    assert_eq!(completion.usage, original.usage);
    for split in 0..=original.tokens.len() {
        let mut split_assembler =
            CompletionAssembler::with_context(&tokenizer, InitialContext::ModelTurn).unwrap();
        split_assembler.observe(&commit(0, &original.tokens[..split], None));
        split_assembler.observe(&commit(
            1,
            &original.tokens[split..],
            original.finish_reason,
        ));
        assert_eq!(split_assembler.finish(original, true), completion);
    }
}

#[test]
#[ignore = "set POWER_LLM_TEST_MODEL to the target GGUF"]
fn real_vocabulary_prepared_prompt_and_observed_completion() {
    let file = ModelFile::open(std::env::var_os("POWER_LLM_TEST_MODEL").unwrap()).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let tokenizer = Tokenizer::from_vocabulary(&model.vocabulary).unwrap();
    let template =
        crate::engine::chat::ChatTemplate::from_gguf(&model.vocabulary, &tokenizer).unwrap();
    let request = crate::engine::chat::ChatRequest::from_openai_json(&json!({"messages":[{"role":"user","content":"What is the capital of France? Answer with one word."}]})).unwrap();
    let prepared = template.prepare(&tokenizer, &request).unwrap();
    let mut assembler = CompletionAssembler::new(&tokenizer, &prepared).unwrap();
    let ids = [100, 45518, 107, 101, 50429, 1];
    assembler.observe(&commit(0, &ids, Some(FinishReason::Eos)));
    let completion = assembler.finish(&result(&ids, FinishReason::Eos), true);
    assert_eq!(completion.content, "Paris");
    assert_eq!(completion.reasoning.as_deref(), Some(""));
    assert_eq!(completion.finish, CompletionFinish::Stop);
}
