use crate::*;

/// A short-canvas full-model diagnostic. Token IDs are hardcoded deliberately:
/// no tokenizer, sampling, generated-text endpoint, or throughput claim.
pub(crate) fn probe_forward(path: &std::ffi::OsStr) -> Result<(), Box<dyn Error>> {
    let started = Instant::now();
    let file = weights::ModelFile::open(path)?;
    let model = model::ModelDefinition::bind(file.inventory())?;
    let runtime = text::ReferenceText::new(&file, &model, text::ReferenceLimits::default())?;
    let mut cache = runtime.new_cache()?;
    println!(
        "Full reference diagnostic: {} layers, BOS prompt, one-position canvas token ID 42",
        model.layers.len()
    );
    println!("Slow F32 correctness path; short canvas, not a generation benchmark.");
    let encoded = runtime.encode(&[model.vocabulary.bos], &mut cache)?;
    let rms =
        |x: &[f32]| (x.iter().map(|&x| f64::from(x).powi(2)).sum::<f64>() / x.len() as f64).sqrt();
    println!(
        "Encoder: {} cached token; final hidden RMS={:.6}",
        cache.total_tokens(),
        rms(&encoded[0])
    );
    let hidden = runtime.decode_hidden(&[42], &cache, None)?;
    let logits = runtime.logits(&hidden)?;
    let mut order: Vec<_> = (0..logits.len()).collect();
    order.sort_by(|&a, &b| logits[b].total_cmp(&logits[a]).then(a.cmp(&b)));
    println!(
        "Decoder final hidden RMS={:.6}; {} finite softcapped logits",
        rms(&hidden[0]),
        logits.len()
    );
    println!("Top diagnostic token IDs/scores (NOT committed generated text):");
    for &id in order.iter().take(5) {
        println!("  {id}: {:.6}", logits[id]);
    }
    println!(
        "Encoder cache remains {} token. Completed in {:.3} s.",
        cache.total_tokens(),
        started.elapsed().as_secs_f64()
    );
    println!(
        "Real-model numerical parity, trained full-canvas generation quality, and serving remain unvalidated/unimplemented."
    );
    Ok(())
}

pub(crate) fn probe_model(path: &std::ffi::OsStr) -> Result<(), Box<dyn Error>> {
    let started = Instant::now();
    let file = weights::ModelFile::open(path)?;
    let model = model::ModelDefinition::bind(file.inventory())?;
    let spec = &model.spec;
    println!(
        "Bound all {} tensors; {} layers, width {}, canvas {}",
        file.inventory().tensors.len(),
        model.layers.len(),
        spec.hidden,
        spec.canvas
    );
    let global = spec
        .attention
        .iter()
        .filter(|g| g.kind == model::AttentionKind::Global)
        .count();
    println!(
        "Attention: {} local / {global} global layers",
        model.layers.len() - global
    );
    println!(
        "Vocabulary: {} tokens / {} merges; tokenizer available separately (not used by this synthetic probe)",
        model.vocabulary.tokens.len(),
        model.vocabulary.merges.len()
    );

    let raw = file
        .read_row(&model.embedding.name, u64::from(model.vocabulary.bos))?
        .decode()?;
    let embedding = power_cpu_llm::kernels::reference::multiply_scaled(
        &raw,
        &vec![1.0; spec.hidden],
        (spec.hidden as f32).sqrt(),
    )?;
    // This is deliberately an operator smoke test, NOT a true router activation
    // (that requires attention/residuals). No inference or generated text claimed.
    if let Some(moe) = &model.layers[0].moe {
        let router = operators::Router::load(&file, &moe.router, spec.top_k, spec.rms_epsilon)?;
        let routing = router.route(&embedding)?;
        println!(
            "Layer-0 router probe on scaled BOS embedding: {:?}",
            routing.selected
        );
    } else {
        println!("Dense model: no router to probe.");
    }

    let mlp = operators::GatedMlp::load(
        &file,
        model
            .self_conditioning_mlp
            .as_ref()
            .ok_or("not a diffusion model")?,
    )?;
    let sc = operators::SelfConditioner::new(
        operators::load_vector(
            &file,
            model
                .self_conditioning_pre_norm
                .ok_or("not a diffusion model")?,
        )?,
        mlp,
        spec.rms_epsilon,
    )?;
    let zero = sc.apply(&embedding, None)?;
    let soft: Vec<_> = (0..spec.hidden)
        .map(|i| (i % 17) as f32 / 17.0 - 0.5)
        .collect();
    let nonzero = sc.apply(&embedding, Some(&soft))?;
    let rms =
        |x: &[f32]| (x.iter().map(|&v| f64::from(v).powi(2)).sum::<f64>() / x.len() as f64).sqrt();
    let difference: Vec<_> = nonzero.iter().zip(&zero).map(|(a, b)| a - b).collect();
    println!(
        "Self-conditioning probe: zero-signal RMS={:.6}, synthetic-signal RMS={:.6}, difference RMS={:.6}",
        rms(&zero),
        rms(&nonzero),
        rms(&difference)
    );
    drop(sc); // release expanded self-conditioning matrices before the FFN probe
    let ffn = operators::feed_forward(
        &file,
        &model.layers[0],
        spec,
        &embedding,
        operators::LayerMode::Decoder,
    )?;
    println!(
        "Layer-0 decoder FFN probe: {} routed experts + shared MLP, output RMS={:.6}",
        ffn.routing.selected.len(),
        rms(&ffn.hidden)
    );
    println!(
        "Completed in {:.3} s. Real weights, reference math, synthetic inputs.",
        started.elapsed().as_secs_f64()
    );
    println!("Not a complete forward pass or generation benchmark; no server yet.");
    Ok(())
}
