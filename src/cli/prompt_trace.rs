//! Engine observations for a causal prompt (`tools/oracle/
//! gemma4_reference.py`): every layer's output and routing at every position,
//! the final normed rows and the last position's logits. Reads only the prompt
//! IDs from the golden manifest. Works for both model kinds (the causal
//! encoder is shared); intended for the autoregressive Gemma 4 (MoE, and the
//! dense E2B with `gemma4_e2b_reference.py`, which records no routing).
use super::full_canvas::{Capture, Writer, ids, write_capture, write_logits};
use power_cpu_llm::engine::{
    execution::MatrixBackend,
    model::ModelDefinition,
    text::{ReferenceLimits, ReferenceText, TracePoint},
    weights::ModelFile,
};
use serde_json::{Value, json};
use std::{error::Error, fs, path::Path, time::Instant};

const CHUNK: usize = 256;

pub(crate) fn trace_prompt(
    model_path: &Path,
    fixture: &Path,
    out: &Path,
    workers: usize,
    backend: MatrixBackend,
    prefill_backend: Option<MatrixBackend>,
) -> Result<(), Box<dyn Error>> {
    let m: Value = serde_json::from_slice(&fs::read(fixture.join("manifest.json"))?)?;
    let known = [
        "power-llm gemma4 prompt golden v1",
        "power-llm gemma4 dense/PLE prompt golden v1",
    ];
    if m["complete"] != true || !known.iter().any(|s| m["schema"] == *s) {
        return Err("fixture manifest incomplete or unknown schema".into());
    }
    let prompt = ids(&m["cases"]["prompt"]["ids"], "prompt.ids")?;
    fs::create_dir(out)?;
    let file = ModelFile::open(model_path)?;
    let model = ModelDefinition::bind(file.inventory())?;
    let spec = model.spec.clone();
    let limits = ReferenceLimits::for_context(&spec, prompt.len());
    let mut runtime = ReferenceText::new_with_backend(&file, &model, limits, workers, backend)?;
    runtime.set_prefill_backend(prefill_backend)?;
    let start = Instant::now();
    let mut cache = runtime.new_cache()?;
    // Per-chunk captures concatenated per layer (all positions selected).
    let mut chunks = Vec::new();
    let mut last = None;
    for first in (0..prompt.len()).step_by(CHUNK) {
        let chunk = &prompt[first..(first + CHUNK).min(prompt.len())];
        let all: Vec<usize> = (0..chunk.len()).collect();
        let mut capture = Capture::default();
        let hidden = runtime.encode_traced(
            chunk,
            &mut cache,
            &mut |p: TracePoint, rows: &[Vec<f32>]| capture.observe(p, rows, &all),
        )?;
        last = hidden.last().cloned();
        chunks.push(capture);
    }
    let mut merged = Capture::default();
    for (index, capture) in chunks.into_iter().enumerate() {
        if index == 0 {
            merged = capture;
            continue;
        }
        for (layer, rows) in merged.layers.iter_mut().zip(capture.layers) {
            layer.extend(rows);
        }
        for (layer, rows) in merged.routing.iter_mut().zip(capture.routing) {
            layer.extend(rows);
        }
        if let (Some(a), Some(b)) = (merged.final_rows.as_mut(), capture.final_rows) {
            a.extend(b);
        }
    }
    let mut w = Writer {
        root: out,
        entries: Vec::new(),
    };
    write_capture(&mut w, "prompt", &merged, spec.hidden, spec.expert_count)?;
    let logits = runtime.logits(&[last.ok_or("empty prompt")?])?;
    write_logits(&mut w, "prompt", &logits, spec.vocabulary, &[0])?;
    let mut manifest = json!({
        "schema": "power-llm gemma4 prompt engine observation v1",
        "kind": "engine-observation-NOT-independent-golden", "complete": true,
        "matrix_backend": runtime.matrix_backend(), "prompt_tokens": prompt.len(),
        "seconds": start.elapsed().as_secs_f64(), "entries": w.entries,
    });
    if let Some(backend) = prefill_backend {
        manifest["prefill_backend"] =
            json!(power_cpu_llm::engine::execution::matrix_kernels(backend)?.name());
    }
    fs::write(
        out.join("manifest.json"),
        serde_json::to_vec_pretty(&manifest)?,
    )?;
    eprintln!("Prompt observations exported to {}.", out.display());
    Ok(())
}
