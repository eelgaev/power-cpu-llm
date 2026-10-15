use super::*;
use crate::engine::{attention::tests::fixture::Fixture, model::ModelDefinition};

#[test]
fn projections_and_grouped_experts_match_scalar_exactly() {
    let fixture = Fixture::new();
    let file = ModelFile::open(&fixture.path).unwrap();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let workers = Workers::current(Workers::available().unwrap().len().min(5)).unwrap();
    let bitrows = |rows: &[Vec<f32>]| {
        rows.iter()
            .flatten()
            .map(|x| x.to_bits())
            .collect::<Vec<_>>()
    };
    let input = vec![
        vec![0.3, -0.2, 0.7, 0.1],
        vec![-0.8, 0.5, 0.2, -0.4],
        vec![0.1, 0.2, 0.3, 0.4],
    ];
    for tensor in [
        model.layers[0].query,
        model.layers[1].query,
        model.embedding,
    ] {
        let batch = project_streamed_batch(&file, tensor, &input, 1024).unwrap();
        let parallel =
            project_streamed_batch_with_workers(&file, tensor, &input, 1024, Some(&workers))
                .unwrap();
        assert_eq!(bitrows(&batch), bitrows(&parallel));
        for (i, row) in input.iter().enumerate() {
            assert_eq!(batch[i], project_streamed(&file, tensor, row).unwrap());
        }
        assert!(project_streamed_batch(&file, tensor, &input, 1).is_err());
    }
    let weights = vec![
        vec![0.1; 8],
        vec![0.0; 8],
        vec![-0.2, 0.5, 0.1, 0.0, 0.0, 1.0, -1.0, 0.2],
    ];
    let transposed =
        project_transposed_streamed_batch(&file, model.embedding, &weights, 64).unwrap();
    let parallel = project_transposed_streamed_batch_with_workers(
        &file,
        model.embedding,
        &weights,
        64,
        Some(&workers),
    )
    .unwrap();
    assert_eq!(bitrows(&transposed), bitrows(&parallel));
    for (i, row) in weights.iter().enumerate() {
        assert_eq!(
            transposed[i],
            project_transposed_streamed(&file, model.embedding, row).unwrap()
        );
    }
    for mode in [LayerMode::Encoder, LayerMode::Decoder] {
        for layer in &model.layers {
            let batch = feed_forward_batch(&file, layer, &model.spec, &input, mode).unwrap();
            let parallel = feed_forward_batch_with_workers(
                &file,
                layer,
                &model.spec,
                &input,
                mode,
                Some(&workers),
            )
            .unwrap();
            for (a, b) in batch.iter().zip(parallel) {
                assert_eq!(
                    a.hidden.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                    b.hidden.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
                );
                assert_eq!(a.routing.selected, b.routing.selected);
            }
            for (i, row) in input.iter().enumerate() {
                let scalar = feed_forward(&file, layer, &model.spec, row, mode).unwrap();
                assert_eq!(batch[i].hidden, scalar.hidden);
                assert_eq!(batch[i].routing.selected, scalar.routing.selected);
                assert_eq!(batch[i].routing.probabilities, scalar.routing.probabilities);
            }
        }
    }
    assert!(project_streamed_batch(&file, model.embedding, &[], 64).is_err());
    assert!(project_streamed_batch(&file, model.embedding, &[vec![1.0; 3]], 64).is_err());
    assert!(project_streamed_batch(&file, model.embedding, &[vec![f32::NAN; 4]], 64).is_err());
    assert!(
        project_transposed_streamed_batch(&file, model.embedding, &[vec![1.0; 7]], 64).is_err()
    );
    assert!(project_transposed_streamed_batch(&file, model.embedding, &[vec![1.0; 8]], 0).is_err());
    assert!(
        feed_forward_batch(
            &file,
            &model.layers[0],
            &model.spec,
            &[],
            LayerMode::Decoder
        )
        .is_err()
    );
}
