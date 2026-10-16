use super::*;
use crate::gguf::MetadataValue;
use std::collections::BTreeMap;

fn array(kind: &str, values: ArrayValues) -> MetadataValue {
    let len = match &values {
        ArrayValues::Signed(v) => v.len(),
        ArrayValues::String(v) => v.len(),
        _ => unreachable!(),
    };
    MetadataValue::Array {
        element_type: kind.into(),
        len,
        values,
    }
}

const P: &str = "nemotron_h_moe.";

/// Tiny structural fixture: Mamba, MoE, attention, MoE, then an MTP block.
/// hidden 4, 2 query heads and 1 K/V head of 2, SSM inner 4 (2 heads of 2,
/// 1 group, state 2, kernel 3), 3 experts of width 2 (top 2), shared width 3.
pub(crate) fn inventory() -> Inventory {
    let mut metadata = BTreeMap::new();
    metadata.insert(
        "general.architecture".into(),
        MetadataValue::String(ARCHITECTURE.into()),
    );
    for (key, value) in [
        ("block_count", 5),
        ("nextn_predict_layers", 1),
        ("embedding_length", 4),
        ("context_length", 64),
        ("attention.head_count", 2),
        ("attention.key_length", 2),
        ("attention.value_length", 2),
        ("expert_count", 3),
        ("expert_used_count", 2),
        ("expert_feed_forward_length", 2),
        ("expert_shared_feed_forward_length", 3),
        ("expert_shared_count", 1),
        ("expert_group_count", 1),
        ("ssm.inner_size", 4),
        ("ssm.state_size", 2),
        ("ssm.group_count", 1),
        ("ssm.time_step_rank", 2),
        ("ssm.conv_kernel", 3),
    ] {
        metadata.insert(format!("{P}{key}"), MetadataValue::Unsigned(value));
    }
    for (key, value) in [
        ("attention.layer_norm_rms_epsilon", 1e-5),
        ("expert_weights_scale", 2.5),
    ] {
        metadata.insert(format!("{P}{key}"), MetadataValue::Float(value));
    }
    metadata.insert(format!("{P}expert_weights_norm"), MetadataValue::Bool(true));
    metadata.insert(
        format!("{P}attention.head_count_kv"),
        array("i32", ArrayValues::Signed(vec![0, 0, 1, 0, 1])),
    );
    metadata.insert(
        format!("{P}feed_forward_length"),
        array("i32", ArrayValues::Signed(vec![0, 2, 0, 2, 2])),
    );
    for (key, value) in [
        ("tokenizer.ggml.model", "gpt2"),
        ("tokenizer.ggml.pre", "pixtral"),
        ("tokenizer.chat_template", "fixture"),
    ] {
        metadata.insert(key.into(), MetadataValue::String(value.into()));
    }
    metadata.insert(
        "tokenizer.ggml.tokens".into(),
        array(
            "str",
            ArrayValues::String((0..8).map(|i| format!("t{i}")).collect()),
        ),
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
        ("bos_token_id", 1),
        ("eos_token_id", 2),
        ("padding_token_id", 0),
        ("unknown_token_id", 0),
    ] {
        metadata.insert(format!("tokenizer.ggml.{key}"), MetadataValue::Unsigned(id));
    }
    metadata.insert(
        "tokenizer.ggml.add_bos_token".into(),
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
        ("output.weight", [4, 8, 1, 1]),
        ("output_norm.weight", [4, 1, 1, 1]),
    ] {
        add(name.into(), shape);
    }
    let attention = [
        ("attn_q.weight", [4, 4, 1, 1]),
        ("attn_k.weight", [4, 2, 1, 1]),
        ("attn_v.weight", [4, 2, 1, 1]),
        ("attn_output.weight", [4, 4, 1, 1]),
    ];
    let moe = [
        ("ffn_gate_inp.weight", [4, 3, 1, 1]),
        ("exp_probs_b.bias", [3, 1, 1, 1]),
        ("ffn_up_exps.weight", [4, 2, 3, 1]),
        ("ffn_down_exps.weight", [2, 4, 3, 1]),
        ("ffn_up_shexp.weight", [4, 3, 1, 1]),
        ("ffn_down_shexp.weight", [3, 4, 1, 1]),
    ];
    // conv channels 4 + 2 * 1 * 2 = 8; projection 4 + 8 + 2 = 14.
    let mamba = [
        ("ssm_in.weight", [4, 14, 1, 1]),
        ("ssm_conv1d.weight", [3, 8, 1, 1]),
        ("ssm_conv1d.bias", [8, 1, 1, 1]),
        ("ssm_dt.bias", [2, 1, 1, 1]),
        ("ssm_a", [1, 2, 1, 1]),
        ("ssm_d", [1, 2, 1, 1]),
        ("ssm_norm.weight", [4, 1, 1, 1]),
        ("ssm_out.weight", [4, 4, 1, 1]),
    ];
    for i in 0..5 {
        add(format!("blk.{i}.attn_norm.weight"), [4, 1, 1, 1]);
        let parts: Vec<&[(&str, [u64; 4])]> = match i {
            0 => vec![&mamba],
            2 => vec![&attention],
            4 => vec![&attention, &moe],
            _ => vec![&moe],
        };
        for (suffix, shape) in parts.into_iter().flatten() {
            add(format!("blk.{i}.{suffix}"), *shape);
        }
    }
    for (suffix, shape) in [
        ("nextn.enorm.weight", [4, 1, 1, 1]),
        ("nextn.hnorm.weight", [4, 1, 1, 1]),
        ("nextn.eh_proj.weight", [8, 4, 1, 1]),
        ("nextn.shared_head_norm.weight", [4, 1, 1, 1]),
        ("post_attention_norm.weight", [4, 1, 1, 1]),
    ] {
        add(format!("blk.4.{suffix}"), shape);
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

fn bind_error(inventory: &Inventory) -> String {
    match ModelDefinition::bind(inventory) {
        Ok(_) => panic!("bound an invalid model"),
        Err(e) => e.0,
    }
}

#[test]
fn binds_every_tensor_and_reads_the_layer_pattern() {
    let fixture = inventory();
    let model = ModelDefinition::bind(&fixture).unwrap();
    let spec = &model.spec;
    assert_eq!(
        spec.layers,
        [
            LayerKind::Mamba,
            LayerKind::Moe,
            LayerKind::Attention,
            LayerKind::Moe
        ]
    );
    assert_eq!((spec.query_heads, spec.kv_heads, spec.head_dim), (2, 1, 2));
    assert_eq!(
        (
            spec.ssm_head_dim(),
            spec.ssm_conv_channels(),
            spec.ssm_projection()
        ),
        (2, 8, 14)
    );
    assert_eq!(
        (spec.expert_count, spec.top_k, spec.routed_scale),
        (3, 2, 2.5)
    );
    assert!(matches!(model.layers[0].mixer, Mixer::Mamba(_)));
    assert!(matches!(model.layers[2].mixer, Mixer::Attention(_)));
    assert!(model.mtp.is_some());
    assert_eq!(model.output.name, "output.weight");
    assert_eq!(model.vocabulary.pre_tokenizer, "pixtral");
}

#[test]
fn rejects_unbound_wrong_shape_and_mixed_layers() {
    let mut extra = inventory();
    let mut tensor = extra.tensors[0].clone();
    tensor.name = "blk.0.ssm_extra".into();
    extra.tensors.push(tensor);
    assert!(bind_error(&extra).contains("unbound"));

    let mut shape = inventory();
    let t = shape
        .tensors
        .iter_mut()
        .find(|t| t.name == "blk.0.ssm_conv1d.weight")
        .unwrap();
    t.shape = [4, 8, 1, 1];
    t.size_bytes = 128;
    assert!(bind_error(&shape).contains("wrong shape"));

    let mut mixed = inventory();
    mixed.metadata.insert(
        format!("{P}feed_forward_length"),
        array("i32", ArrayValues::Signed(vec![0, 2, 2, 2, 2])),
    );
    assert!(bind_error(&mixed).contains("both"));
}
