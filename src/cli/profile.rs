//! Stage timing of one real full-canvas decoder pass (diagnostic, not a
//! throughput benchmark). Timestamps come from the existing trace observer, so
//! the engine runs its normal batched path; the observer only records times.
use power_cpu_llm::engine::{
    execution::MatrixBackend,
    model::ModelDefinition,
    text::{ReferenceLimits, ReferenceText},
    weights::ModelFile,
};
use std::{error::Error, path::Path, time::Instant};

fn pass(
    runtime: &ReferenceText<'_>,
    tokens: &[u32],
    cache: &power_cpu_llm::engine::text::EncoderCache<'_>,
    previous: Option<&[f32]>,
    label: &str,
) -> Result<Vec<f32>, Box<dyn Error>> {
    let start = Instant::now();
    let mut marks: Vec<(String, f64)> = Vec::new();
    let mut last = 0.0;
    let hidden = runtime.decode_hidden_traced(tokens, cache, previous, &mut |point, _| {
        if point.is_routing() {
            return Ok::<_, std::convert::Infallible>(());
        }
        let now = start.elapsed().as_secs_f64();
        marks.push((point.name(), now - last));
        last = now;
        Ok(())
    })?;
    let t = Instant::now();
    let logits = runtime.logits(&hidden)?;
    let head = t.elapsed().as_secs_f64();
    let total = start.elapsed().as_secs_f64();
    let layers: f64 = marks
        .iter()
        .filter(|(n, _)| n.starts_with("decoder.layer."))
        .map(|(_, s)| s)
        .sum();
    println!("== {label}: total {total:.2}s");
    for (name, seconds) in &marks {
        if !name.starts_with("decoder.layer.") {
            println!("  {name:<28} {seconds:>8.2}s");
        }
    }
    let per: Vec<String> = marks
        .iter()
        .filter(|(n, _)| n.starts_with("decoder.layer."))
        .map(|(_, s)| format!("{s:.2}"))
        .collect();
    println!(
        "  {:<28} {layers:>8.2}s  per layer [{}]",
        "30 layers",
        per.join(" ")
    );
    println!("  {:<28} {head:>8.2}s", "logits (tied head)");
    for (stage, seconds) in power_cpu_llm::engine::stats::take() {
        println!("    stage {stage:<40} {seconds:>8.2}s");
    }
    let (runs, wall, capacity, busy) = power_cpu_llm::engine::stats::take_pool();
    println!(
        "    pool: {runs} submissions, {wall:.2}s wall, busy {:.0}% of submitted worker time",
        100.0 * busy / capacity.max(1e-9)
    );
    Ok(logits)
}

pub(crate) fn profile_decode(
    model_path: &Path,
    workers: usize,
    backend: MatrixBackend,
) -> Result<(), Box<dyn Error>> {
    let file = ModelFile::open(model_path)?;
    let model = ModelDefinition::bind(file.inventory())?;
    let runtime = ReferenceText::new_with_backend(
        &file,
        &model,
        ReferenceLimits::default(),
        workers,
        backend,
    )?;
    let spec = runtime.specification();
    eprintln!(
        "profile-decode: backend {}, worker CPUs {:?}",
        runtime.matrix_backend(),
        runtime.worker_cpus()
    );
    let vocab = spec.vocabulary as u64;
    let prompt: Vec<u32> = std::iter::once(2)
        .chain((1..21u64).map(|i| ((1000 + 7919 * i) % vocab) as u32))
        .collect();
    let canvas: Vec<u32> = (0..spec.canvas as u64)
        .map(|i| ((5003 + 104729 * i) % vocab) as u32)
        .collect();
    let mut cache = runtime.new_cache()?;
    let t = Instant::now();
    runtime.encode(&prompt, &mut cache)?;
    println!(
        "encode {} prompt tokens: {:.2}s",
        prompt.len(),
        t.elapsed().as_secs_f64()
    );
    let logits = pass(
        &runtime,
        &canvas,
        &cache,
        None,
        "decoder pass, no self-conditioning",
    )?;
    let processed: Vec<f32> = logits.iter().map(|x| x / 0.8).collect();
    pass(
        &runtime,
        &canvas,
        &cache,
        Some(&processed),
        "decoder pass with self-conditioning",
    )?;
    Ok(())
}
