//! Engine observations for the independent long-context oracle
//! (`tools/oracle/long_context.py`). Reads ONLY the
//! oracle's documented inputs (prompt IDs, canvas IDs, selections) from its
//! manifest; never its expected tensors. Entry names match the oracle's.
use super::full_canvas::{Capture, Writer, cache_rows, ids, indices, write_capture, write_logits};
use power_cpu_llm::engine::{
    execution::MatrixBackend,
    model::ModelDefinition,
    text::{ReferenceLimits, ReferenceText},
    weights::ModelFile,
};
use serde_json::{Value, json};
use std::{error::Error, fs, path::Path, time::Instant};

const CHUNK: usize = 256;

pub(crate) fn trace_long_context(
    model_path: &Path,
    fixture: &Path,
    out: &Path,
    workers: usize,
    backend: MatrixBackend,
) -> Result<(), Box<dyn Error>> {
    let m: Value = serde_json::from_slice(&fs::read(fixture.join("manifest.json"))?)?;
    if m["complete"] != true || m["schema"] != "power-llm long-context golden v1" {
        return Err("fixture manifest incomplete or unknown schema".into());
    }
    let c = &m["cases"];
    let prompt = ids(&c["prompt"]["ids"], "prompt.ids")?;
    let canvas = ids(&c["long_decode"]["decoder_ids"], "long_decode.decoder_ids")?;
    let selected = indices(&m["selection"]["positions"], "selection.positions")?;
    let enc_selected = indices(
        &c["long_encode"]["selected_chunk_indices"],
        "long_encode.selected",
    )?;
    let last_start = c["long_encode"]["last_chunk_start"]
        .as_u64()
        .ok_or("last_chunk_start")? as usize;
    if c["prompt"]["encoder_chunk"].as_u64() != Some(CHUNK as u64)
        || last_start != (prompt.len() - 1) / CHUNK * CHUNK
    {
        return Err("fixture chunking differs from this exporter".into());
    }
    fs::create_dir(out)?;
    let file = ModelFile::open(model_path)?;
    let model = ModelDefinition::bind(file.inventory())?;
    let spec = model.spec.clone();
    let limits = ReferenceLimits::for_context(&spec, prompt.len() + canvas.len());
    let runtime = ReferenceText::new_with_backend(&file, &model, limits, workers, backend)?;
    eprintln!(
        "Long-context trace: {} prompt tokens; backend {}; worker CPUs {:?}.",
        prompt.len(),
        runtime.matrix_backend(),
        runtime.worker_cpus()
    );
    let start = Instant::now();
    let mut w = Writer {
        root: out,
        entries: Vec::new(),
    };
    let mut cache = runtime.new_cache()?;
    let mut encode = Capture::default();
    for first in (0..prompt.len()).step_by(CHUNK) {
        let chunk = &prompt[first..(first + CHUNK).min(prompt.len())];
        if first == last_start {
            runtime.encode_traced(chunk, &mut cache, &mut |p, rows| {
                encode.observe(p, rows, &enc_selected)
            })?;
        } else {
            runtime.encode(chunk, &mut cache)?;
        }
        if (first / CHUNK).is_multiple_of(8) {
            eprintln!(
                "[{:>8.1}s] encoded {}",
                start.elapsed().as_secs_f64(),
                first + chunk.len()
            );
        }
    }
    write_capture(
        &mut w,
        "long_encode",
        &encode,
        spec.hidden,
        spec.expert_count,
    )?;
    // Selected cache rows by absolute position (sliding layers retain the last window-1).
    let total = prompt.len();
    let positions = &c["cache"]["positions"];
    for layer in indices(&c["cache"]["layers"], "cache.layers")? {
        let (keys, values, retained) = cache_rows(&cache, layer)?;
        let width = keys.len() / retained;
        // The oracle's layout: [tokens, kv_heads, head_dim].
        let g = &spec.attention[layer];
        if g.kv_heads * g.head_dim != width {
            return Err("cache row width does not match the layer geometry".into());
        }
        let wanted = indices(&positions[layer.to_string()], "cache.positions")?;
        let (mut k, mut v) = (Vec::new(), Vec::new());
        for p in &wanted {
            let index = p
                .checked_sub(total - retained)
                .filter(|&i| i < retained)
                .ok_or("cache position not retained")?;
            k.extend_from_slice(&keys[index * width..(index + 1) * width]);
            v.extend_from_slice(&values[index * width..(index + 1) * width]);
        }
        w.f32(
            &format!("long_cache.{layer}.k.selected"),
            &[wanted.len(), g.kv_heads, g.head_dim],
            &k,
        )?;
        w.f32(
            &format!("long_cache.{layer}.v.selected"),
            &[wanted.len(), g.kv_heads, g.head_dim],
            &v,
        )?;
    }
    eprintln!("[{:>8.1}s] decoder canvas", start.elapsed().as_secs_f64());
    let mut decode = Capture::default();
    let hidden = runtime.decode_hidden_traced(&canvas, &cache, None, &mut |p, rows| {
        decode.observe(p, rows, &selected)
    })?;
    write_capture(
        &mut w,
        "long_decode",
        &decode,
        spec.hidden,
        spec.expert_count,
    )?;
    let logits = runtime.logits(&hidden)?;
    write_logits(&mut w, "long_decode", &logits, spec.vocabulary, &[])?;
    let manifest = json!({
        "schema": "power-llm long-context engine observation v1",
        "kind": "engine-observation-NOT-independent-golden", "complete": true,
        "matrix_backend": runtime.matrix_backend(), "prompt_tokens": total,
        "seconds": start.elapsed().as_secs_f64(), "entries": w.entries,
    });
    fs::write(
        out.join("manifest.json"),
        serde_json::to_vec_pretty(&manifest)?,
    )?;
    eprintln!("Long-context observations exported to {}.", out.display());
    Ok(())
}
