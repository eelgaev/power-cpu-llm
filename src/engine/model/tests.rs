use super::*;

fn array(kind: &str, values: ArrayValues) -> MetadataValue {
    let len = match &values {
        ArrayValues::Unsigned(v) => v.len(),
        ArrayValues::Signed(v) => v.len(),
        ArrayValues::Float(v) => v.len(),
        ArrayValues::Bool(v) => v.len(),
        ArrayValues::String(v) => v.len(),
    };
    MetadataValue::Array {
        element_type: kind.into(),
        len,
        values,
    }
}

// Tiny structural fixture: metadata/tensor records, no actual weight blob.
pub(crate) fn inventory() -> Inventory {
    let mut metadata = BTreeMap::new();
    metadata.insert(
        "general.architecture".into(),
        MetadataValue::String("diffusion-gemma".into()),
    );
    for (key, value) in [
        ("block_count", 2),
        ("embedding_length", 4),
        ("attention.head_count", 2),
        ("attention.key_length", 4),
        ("attention.value_length", 4),
        ("attention.key_length_swa", 2),
        ("attention.value_length_swa", 2),
        ("rope.dimension_count", 4),
        ("rope.dimension_count_swa", 2),
        ("attention.shared_kv_layers", 0),
        ("embedding_length_per_layer_input", 0),
        ("expert_count", 3),
        ("expert_used_count", 2),
        ("feed_forward_length", 3),
        ("expert_feed_forward_length", 2),
        ("attention.sliding_window", 8),
        ("context_length", 128),
    ] {
        metadata.insert(
            format!("diffusion-gemma.{key}"),
            MetadataValue::Unsigned(value),
        );
    }
    for (key, value) in [
        ("attention.layer_norm_rms_epsilon", 1e-6),
        ("final_logit_softcapping", 30.0),
        ("rope.freq_base", 1e6),
        ("rope.freq_base_swa", 1e4),
    ] {
        metadata.insert(
            format!("diffusion-gemma.{key}"),
            MetadataValue::Float(value),
        );
    }
    metadata.insert(
        "diffusion-gemma.attention.causal".into(),
        MetadataValue::Bool(false),
    );
    metadata.insert("diffusion.canvas_length".into(), MetadataValue::Unsigned(4));
    metadata.insert(
        "diffusion-gemma.attention.sliding_window_pattern".into(),
        array("bool", ArrayValues::Bool(vec![true, false])),
    );
    metadata.insert(
        "diffusion-gemma.attention.head_count_kv".into(),
        array("i32", ArrayValues::Signed(vec![1, 1])),
    );
    metadata.insert(
        "tokenizer.ggml.model".into(),
        MetadataValue::String("gemma4".into()),
    );
    metadata.insert(
        "tokenizer.chat_template".into(),
        MetadataValue::String("fixture".into()),
    );
    metadata.insert(
        "tokenizer.ggml.tokens".into(),
        array(
            "str",
            ArrayValues::String((0..8).map(|i| format!("t{i}")).collect()),
        ),
    );
    metadata.insert(
        "tokenizer.ggml.scores".into(),
        array("f32", ArrayValues::Float(vec![0.0; 8])),
    );
    metadata.insert(
        "tokenizer.ggml.token_type".into(),
        array("i32", ArrayValues::Signed(vec![1; 8])),
    );
    metadata.insert(
        "tokenizer.ggml.merges".into(),
        array("str", ArrayValues::String(vec![])),
    );
    for (key, id) in [
        ("bos_token_id", 2),
        ("eos_token_id", 1),
        ("padding_token_id", 0),
        ("unknown_token_id", 3),
    ] {
        metadata.insert(format!("tokenizer.ggml.{key}"), MetadataValue::Unsigned(id));
    }
    metadata.insert(
        "tokenizer.ggml.add_bos_token".into(),
        MetadataValue::Bool(true),
    );
    metadata.insert(
        "tokenizer.ggml.add_space_prefix".into(),
        MetadataValue::Bool(false),
    );
    let mut tensors = Vec::new();
    let mut add = |name: String, shape: [u64; 4]| {
        tensors.push(TensorInfo {
            name,
            type_id: 0,
            type_name: "f32".into(),
            shape,
            file_offset: 32,
            size_bytes: shape.iter().product::<u64>() * 4,
        });
    };
    for (name, shape) in [
        ("token_embd.weight", [4, 8, 1, 1]),
        ("output_norm.weight", [4, 1, 1, 1]),
        ("rope_freqs.weight", [2, 1, 1, 1]),
        ("self_cond_pre_norm.weight", [4, 1, 1, 1]),
        ("self_cond_gate.weight", [4, 3, 1, 1]),
        ("self_cond_up.weight", [4, 3, 1, 1]),
        ("self_cond_down.weight", [3, 4, 1, 1]),
    ] {
        add(name.into(), shape);
    }
    for i in 0..2 {
        let d = if i == 0 { 2 } else { 4 };
        for (suffix, shape) in [
            ("attn_q.weight", [4, 2 * d, 1, 1]),
            ("attn_k.weight", [4, d, 1, 1]),
            ("attn_output.weight", [2 * d, 4, 1, 1]),
            ("attn_q_norm.weight", [d, 1, 1, 1]),
            ("attn_k_norm.weight", [d, 1, 1, 1]),
            ("ffn_gate.weight", [4, 3, 1, 1]),
            ("ffn_up.weight", [4, 3, 1, 1]),
            ("ffn_down.weight", [3, 4, 1, 1]),
            ("ffn_gate_inp.weight", [4, 3, 1, 1]),
            ("ffn_gate_inp.scale", [4, 1, 1, 1]),
            ("ffn_down_exps.scale", [3, 1, 1, 1]),
            ("ffn_gate_up_exps.weight", [4, 4, 3, 1]),
            ("ffn_down_exps.weight", [2, 4, 3, 1]),
            ("enc_layer_output_scale.weight", [1, 1, 1, 1]),
            ("layer_output_scale.weight", [1, 1, 1, 1]),
        ] {
            add(format!("blk.{i}.{suffix}"), shape);
        }
        for suffix in [
            "attn_norm.weight",
            "post_attention_norm.weight",
            "ffn_norm.weight",
            "post_ffw_norm_1.weight",
            "post_ffw_norm_2.weight",
            "pre_ffw_norm_2.weight",
            "post_ffw_norm.weight",
        ] {
            add(format!("blk.{i}.{suffix}"), [4, 1, 1, 1]);
        }
        if i == 0 {
            add("blk.0.attn_v.weight".into(), [4, 2, 1, 1]);
        }
    }
    Inventory {
        version: 3,
        alignment: 32,
        data_offset: 32,
        file_size: 10_000,
        metadata,
        tensors,
    }
}

#[test]
fn binds_every_tensor_and_distinguishes_local_and_global_heads() {
    let fixture = inventory();
    let model = ModelDefinition::bind(&fixture).unwrap();
    assert_eq!(model.spec.hidden, 4);
    assert_eq!(model.spec.canvas, 4);
    assert_eq!(model.spec.attention[0].kind, AttentionKind::Sliding);
    assert_eq!(model.spec.attention[1].kind, AttentionKind::Global);
    assert!(model.layers[0].value.is_some());
    assert!(model.layers[1].value.is_none());
    assert_eq!(model.vocabulary.tokens.len(), 8);
    assert_eq!(model.embedding.name, "token_embd.weight");
}

#[test]
fn rejects_missing_wrong_shape_extra_and_duplicate_tensors() {
    let mut missing = inventory();
    missing
        .tensors
        .retain(|t| t.name != "self_cond_down.weight");
    assert!(ModelDefinition::bind(&missing).is_err());
    let mut shape = inventory();
    shape
        .tensors
        .iter_mut()
        .find(|t| t.name == "blk.0.attn_q.weight")
        .unwrap()
        .shape[1] = 5;
    assert!(ModelDefinition::bind(&shape).is_err());
    let mut extra = inventory();
    let mut t = extra.tensors[0].clone();
    t.name = "unsupported.weight".into();
    extra.tensors.push(t);
    assert!(ModelDefinition::bind(&extra).is_err());
    let mut duplicate = inventory();
    duplicate.tensors.push(duplicate.tensors[0].clone());
    assert!(ModelDefinition::bind(&duplicate).is_err());
}

#[test]
fn rejects_invalid_metadata_geometry_vocabulary_and_special_tokens() {
    for (key, value) in [
        ("diffusion.canvas_length", MetadataValue::Unsigned(0)),
        (
            "diffusion-gemma.expert_used_count",
            MetadataValue::Unsigned(4),
        ),
        (
            "diffusion-gemma.attention.head_count_kv",
            array("i32", ArrayValues::Signed(vec![0, 1])),
        ),
        (
            "diffusion-gemma.attention.sliding_window_pattern",
            array("bool", ArrayValues::Bool(vec![true])),
        ),
        (
            "diffusion-gemma.attention.layer_norm_rms_epsilon",
            MetadataValue::Float(f64::NAN),
        ),
        ("tokenizer.ggml.bos_token_id", MetadataValue::Unsigned(8)),
        (
            "tokenizer.ggml.tokens",
            array("str", ArrayValues::String(vec![])),
        ),
        (
            "diffusion-gemma.attention.causal",
            MetadataValue::Bool(true),
        ),
    ] {
        let mut fixture = inventory();
        fixture.metadata.insert(key.into(), value);
        assert!(
            ModelDefinition::bind(&fixture).is_err(),
            "accepted invalid {key}"
        );
    }
}

#[test]
fn vocabulary_validates_score_type_and_optional_mask() {
    for (key, value) in [
        (
            "tokenizer.ggml.scores",
            array("f32", ArrayValues::Float(vec![f64::NAN; 8])),
        ),
        (
            "tokenizer.ggml.scores",
            array("f32", ArrayValues::Float(vec![f64::INFINITY; 8])),
        ),
        (
            "tokenizer.ggml.token_type",
            array("i32", ArrayValues::Signed(vec![7; 8])),
        ),
        ("tokenizer.ggml.mask_token_id", MetadataValue::Unsigned(8)),
    ] {
        let mut fixture = inventory();
        fixture.metadata.insert(key.into(), value);
        assert!(ModelDefinition::bind(&fixture).is_err());
    }
    let mut fixture = inventory();
    fixture.metadata.insert(
        "tokenizer.ggml.scores".into(),
        array("f32", ArrayValues::Float(vec![f64::NEG_INFINITY; 8])),
    );
    fixture.metadata.insert(
        "tokenizer.ggml.mask_token_id".into(),
        MetadataValue::Unsigned(4),
    );
    assert_eq!(
        ModelDefinition::bind(&fixture).unwrap().vocabulary.mask,
        Some(4)
    );
    assert_eq!(
        ModelDefinition::bind(&inventory()).unwrap().vocabulary.mask,
        None
    );
}

#[test]
#[ignore = "set POWER_LLM_TEST_MODEL to the completed target GGUF"]
fn actual_diffusiongemma_all_bindings_and_metadata() {
    let file = crate::engine::weights::ModelFile::open(
        std::env::var_os("POWER_LLM_TEST_MODEL").expect("model path"),
    )
    .unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    assert_eq!(model.layers.len(), 30);
    assert_eq!(model.spec.hidden, 2816);
    assert_eq!(model.spec.canvas, 256);
    assert_eq!(model.vocabulary.tokens.len(), 262144);
    assert_eq!(model.vocabulary.mask, Some(4));
    assert_eq!(
        (
            model.vocabulary.bos,
            model.vocabulary.eos,
            model.vocabulary.padding,
            model.vocabulary.unknown
        ),
        (2, 1, 0, 3)
    );
    assert_eq!(model.vocabulary.merges.len(), 514906);
    assert_eq!(
        model
            .spec
            .attention
            .iter()
            .filter(|g| g.kind == AttentionKind::Global)
            .count(),
        5
    );
    for (i, g) in model.spec.attention.iter().enumerate() {
        assert_eq!(g.kv_heads, if i % 6 == 5 { 2 } else { 8 });
        assert_eq!(g.head_dim, if i % 6 == 5 { 512 } else { 256 });
    }
    println!(
        "Bound all {} tensors: {} layers, {} tokens, {} merges",
        file.inventory().tensors.len(),
        model.layers.len(),
        model.vocabulary.tokens.len(),
        model.vocabulary.merges.len()
    );
}
