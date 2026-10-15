use super::*;
#[cfg(feature = "reference-oracle")]
use crate::kernels::{ActivationBlock, Backend, Kernels, Q8WeightBlock};

#[cfg(feature = "reference-oracle")]
#[test]
fn random_encoded_blocks_match_pinned_ggml_oracle() {
    let mut rng = 0x12345678u32;
    for encoding in [
        Encoding::Q4_0,
        Encoding::Q5_0,
        Encoding::Q8_0,
        Encoding::Q4K,
        Encoding::Q6K,
    ] {
        let (elements, block_bytes) = encoding.block_layout();
        for _ in 0..1000 {
            let mut bytes: Vec<u8> = (0..block_bytes * 3)
                .map(|_| {
                    rng ^= rng << 13;
                    rng ^= rng >> 17;
                    rng ^= rng << 5;
                    rng as u8
                })
                .collect();
            for block in bytes.chunks_exact_mut(block_bytes) {
                let d_offset = if encoding == Encoding::Q6K { 208 } else { 0 };
                block[d_offset..d_offset + 2].copy_from_slice(&0x3800_u16.to_le_bytes());
                if encoding == Encoding::Q4K {
                    block[2..4].copy_from_slice(&0x3400_u16.to_le_bytes());
                }
            }
            let ours = encoding.decode(&bytes, elements * 3).unwrap();
            let oracle =
                crate::gguf::reference::decode(encoding as i32, &bytes, elements * 3).unwrap();
            for (a, b) in ours.iter().zip(oracle) {
                assert!(
                    (a - b).abs() <= 1e-6 * (1.0 + b.abs()),
                    "{encoding:?}: {a} != {b}"
                );
            }
        }
    }
}

#[test]
fn contiguous_encoded_tiles_validate_ranges_and_budget() {
    let fixture = crate::engine::attention::tests::fixture::Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let name = "token_embd.weight";
    let row = file.read_row(name, 2).unwrap();
    let size = row.bytes.len() * 3;
    let tile = file.read_rows(name, 2, 3, size).unwrap();
    assert_eq!(tile.rows, 3);
    assert_eq!(tile.elements, row.elements);
    assert_eq!(tile.encoding, row.encoding);
    let expected: Vec<_> = (2..5)
        .flat_map(|i| file.read_row(name, i).unwrap().bytes)
        .collect();
    assert_eq!(tile.bytes, expected);
    assert!(matches!(
        file.read_rows(name, 2, 3, size - 1),
        Err(Error::ReadLimit)
    ));
    assert!(matches!(
        file.read_rows(name, 7, 2, size),
        Err(Error::RowOutOfRange)
    ));
    assert!(matches!(
        file.read_rows(name, u64::MAX, 1, size),
        Err(Error::RowOutOfRange)
    ));
    assert!(matches!(
        file.read_rows(name, 0, 0, size),
        Err(Error::RowOutOfRange)
    ));
    assert!(matches!(
        file.read_rows("missing", 0, 1, size),
        Err(Error::TensorMissing(_))
    ));
}

#[test]
fn invalid_tensor_layouts_are_rejected() {
    let mut tensor = TensorInfo {
        name: "test".into(),
        type_id: 8,
        type_name: "q8_0".into(),
        shape: [32, 2, 1, 1],
        file_offset: 32,
        size_bytes: 68,
    };
    assert!(validate_layout(&tensor).is_ok());
    tensor.size_bytes = 67;
    assert!(validate_layout(&tensor).is_err());
    tensor.size_bytes = 68;
    tensor.shape[0] = 31;
    assert!(validate_layout(&tensor).is_err());
    tensor.shape = [32, u64::MAX, 2, 1];
    assert!(validate_layout(&tensor).is_err());
}

/// Optional actual-model test, never downloads files or runs inference.
/// Samples first/middle/last rows of EVERY tensor (including flattened expert
/// rows), compares decoded values to ggml, and checks native Q8 block dots.
#[cfg(feature = "reference-oracle")]
#[test]
#[ignore = "set POWER_LLM_TEST_MODEL to an immutable completed target GGUF"]
fn actual_diffusiongemma_rows_match_oracle_and_q8_kernels() {
    let path = std::env::var_os("POWER_LLM_TEST_MODEL").expect("set POWER_LLM_TEST_MODEL");
    let model = ModelFile::open(path).unwrap();
    for name in [
        "self_cond_down.weight",
        "self_cond_gate.weight",
        "self_cond_up.weight",
        "self_cond_pre_norm.weight",
        "token_embd.weight",
    ] {
        assert!(model.tensor(name).is_some(), "missing {name}");
    }
    let native = Kernels::new(Backend::Auto).unwrap();
    let scalar = Kernels::new(Backend::Scalar).unwrap();
    let mut checked = 0;
    let mut q8_blocks = 0;
    for tensor in &model.inventory().tensors {
        let nrows = row_count(tensor).unwrap();
        let mut indices = vec![0, nrows / 2, nrows - 1];
        indices.sort_unstable();
        indices.dedup();
        for row_id in indices {
            let row = model.read_row(&tensor.name, row_id).unwrap();
            let decoded = row.decode().unwrap();
            let oracle =
                crate::gguf::reference::decode(tensor.type_id, &row.bytes, row.elements).unwrap();
            for (index, (a, b)) in decoded.iter().zip(&oracle).enumerate() {
                assert!(a.is_finite() && b.is_finite());
                assert!(
                    (a - b).abs() <= 1e-6 * (1.0 + b.abs()),
                    "{} row {row_id} element {index}: {a} != {b}",
                    tensor.name
                );
            }
            if row.encoding == Encoding::Q8_0 {
                for (block_index, block) in row.bytes.chunks_exact(34).enumerate() {
                    let q: [i8; 32] = std::array::from_fn(|i| block[i + 2] as i8);
                    let a: [i8; 32] =
                        std::array::from_fn(|i| ((i + block_index) as i8).wrapping_mul(13));
                    let weights = Q8WeightBlock::from_quantized(&q, 1.0).unwrap();
                    let activation = ActivationBlock::from_quantized(&a, 1.0).unwrap();
                    let expected: i32 = q
                        .iter()
                        .zip(a)
                        .map(|(&w, a)| i32::from(w) * i32::from(a))
                        .sum();
                    assert_eq!(native.q8_block_dot(&weights, &activation), expected);
                    assert_eq!(scalar.q8_block_dot(&weights, &activation), expected);
                    q8_blocks += 1;
                }
            }
            checked += 1;
        }
        assert!(matches!(
            model.read_row(&tensor.name, nrows),
            Err(Error::RowOutOfRange)
        ));
    }
    println!(
        "Validated {checked} real tensor rows, {q8_blocks} Q8 blocks; selected backend {}",
        native.name()
    );
    assert!(checked > 1000);
    assert!(q8_blocks > 1000);
}
