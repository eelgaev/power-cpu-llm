use super::*;
use crate::engine::attention::tests::fixture::Fixture;
use std::os::unix::fs::FileExt;
fn bits(rows: &[Vec<f32>]) -> Vec<u32> {
    rows.iter().flatten().map(|x| x.to_bits()).collect()
}
#[test]
fn uneven_row_position_partitions_and_global_zero_tiles_preserve_bits_and_errors() {
    let fixture = Fixture::with_embedding_shape(7, 137);
    let workers = Workers::current(Workers::available().unwrap().len().min(5)).unwrap();
    let input: Vec<Vec<f32>> = (0..9)
        .map(|p| {
            (0..7)
                .map(|c| ((p * 13 + c * 7) % 19) as f32 * 0.05 - 0.45)
                .collect()
        })
        .collect();
    let mut weights = vec![vec![0.0; 137]; 9];
    weights[0][4] = 0.3;
    weights[3][130] = -0.2;
    weights[8][136] = 0.7;
    weights[1][4] = -0.0;
    weights[6][130] = -0.0;
    let file = ModelFile::open(&fixture.path).unwrap();
    let tensor = file.tensor("token_embd.weight").unwrap();
    let offset = tensor.file_offset + 129 * 7 * 4;
    let serial = project_streamed_batch(&file, tensor, &input, 4096).unwrap();
    let parallel =
        project_streamed_batch_with_workers(&file, tensor, &input, 4096, Some(&workers)).unwrap();
    assert_eq!(bits(&serial), bits(&parallel));
    let serial = project_transposed_streamed_batch(&file, tensor, &weights, 4096).unwrap();
    let parallel = project_transposed_streamed_batch_with_workers(
        &file,
        tensor,
        &weights,
        4096,
        Some(&workers),
    )
    .unwrap();
    assert_eq!(bits(&serial), bits(&parallel));
    assert!(project_streamed_batch_with_workers(&file, tensor, &input, 1, Some(&workers)).is_err());
    drop(file);
    // Mutate only this temporary fixture BETWEEN runtime lifetimes.
    std::fs::OpenOptions::new()
        .write(true)
        .open(&fixture.path)
        .unwrap()
        .write_all_at(&f32::NAN.to_le_bytes(), offset)
        .unwrap();
    let file = ModelFile::open(&fixture.path).unwrap();
    let tensor = file.tensor("token_embd.weight").unwrap();
    // Inactive GLOBAL second tile is skipped by both paths, not merely an
    // individual worker's subset. It is visited if ANY position activates it.
    let mut sparse = vec![vec![0.0; 137]; 9];
    sparse[0][4] = 0.3;
    let a = project_transposed_streamed_batch(&file, tensor, &sparse, 4096).unwrap();
    let b = project_transposed_streamed_batch_with_workers(
        &file,
        tensor,
        &sparse,
        4096,
        Some(&workers),
    )
    .unwrap();
    assert_eq!(bits(&a), bits(&b));
    sparse[8][136] = 0.7;
    for workers in [None, Some(&workers)] {
        assert!(matches!(
            project_transposed_streamed_batch_with_workers(&file, tensor, &sparse, 4096, workers),
            Err(Error::Quantized(quantized::Error::NonFiniteWeight))
        ));
        assert!(matches!(
            project_streamed_batch_with_workers(&file, tensor, &input, 4096, workers),
            Err(Error::Quantized(quantized::Error::NonFiniteWeight))
        ));
    }
}
