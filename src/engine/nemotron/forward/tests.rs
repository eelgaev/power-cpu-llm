use super::*;
use crate::engine::attention::tests::fixture::{pattern, write_gguf};
use crate::engine::nemotron::tests::inventory;

struct TempModel(std::path::PathBuf);
impl Drop for TempModel {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// The tiny structural fixture as a GGUF file, with stable (negative) A.
fn model_file() -> (TempModel, ModelFile) {
    let (path, _) = write_gguf(&inventory(), |name, i| {
        if name.ends_with("ssm_a") {
            -0.5 - i as f32 * 0.25
        } else if name.contains("norm") {
            0.6 + (i % 7) as f32 * 0.11
        } else {
            pattern(name, i)
        }
    });
    let file = ModelFile::open(&path).unwrap();
    (TempModel(path), file)
}

#[test]
fn one_batch_equals_one_position_at_a_time() {
    let (_guard, file) = model_file();
    let model = ModelDefinition::bind(file.inventory()).unwrap();
    let tokens = [1, 3, 5, 2, 7];
    let compute = Compute::portable(None);
    let mut state = State::new(&model);
    let mut layers = Vec::new();
    let batch = forward(
        &file,
        &model,
        &mut state,
        &tokens,
        compute,
        &mut |i, rows| layers.push((i, rows.len())),
    )
    .unwrap();
    assert_eq!(layers, [(0, 5), (1, 5), (2, 5), (3, 5)]);
    assert_eq!(state.positions(), 5);
    let mut state = State::new(&model);
    for (i, &t) in tokens.iter().enumerate() {
        let one = forward(&file, &model, &mut state, &[t], compute, &mut |_, _| {}).unwrap();
        assert_eq!(one[0], batch[i], "position {i}");
    }
    let a = logits(&file, &model, &batch[4], compute).unwrap();
    assert_eq!(a.len(), 8);
    assert!(a.iter().all(|x| x.is_finite()));
    // The recurrent state carries information: a different history changes
    // the last position.
    let mut other = State::new(&model);
    let changed = forward(
        &file,
        &model,
        &mut other,
        &[4, 3, 5, 2, 7],
        compute,
        &mut |_, _| {},
    )
    .unwrap();
    assert_ne!(changed[4], batch[4]);
}

#[test]
fn routing_chooses_with_the_bias_and_weights_without_it() {
    // Scores: sigmoid(0) = 0.5, sigmoid(ln 3) = 0.75, sigmoid(-ln 3) = 0.25.
    let ln3 = 3f32.ln();
    let logits = [0.0, ln3, -ln3, 0.0];
    let bias = [0.0, 0.0, 0.3, 0.0];
    // The bias makes expert 2 a choice, but its weight uses its score 0.25.
    let r = route(&logits, &bias, 2, 2.0);
    assert_eq!(r.iter().map(|p| p.0).collect::<Vec<_>>(), [1, 2]);
    assert!((r[0].1 - 1.5).abs() < 1e-6 && (r[1].1 - 0.5).abs() < 1e-6);
    // Ties (experts 0 and 3) keep the lower expert.
    let r = route(&logits, &bias, 3, 2.0);
    assert_eq!(r.iter().map(|p| p.0).collect::<Vec<_>>(), [1, 2, 0]);
    assert!((r.iter().map(|p| p.1).sum::<f32>() - 2.0).abs() < 1e-6);
}
