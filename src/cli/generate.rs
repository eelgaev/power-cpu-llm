//! End-to-end text-chat diagnostic. Short-canvas output is real committed
//! model output, not a validated response quality or serving demonstration.
use power_cpu_llm::engine::{
    chat::{ChatRequest, ChatTemplate},
    completion::{CompletionAssembler, CompletionFinish},
    diffusion::DenoisingConfig,
    generation::{Event, GenerationConfig, GenerationSession},
    model::ModelDefinition,
    text::{ReferenceLimits, ReferenceText},
    tokenizer::Tokenizer,
    weights::ModelFile,
};
use std::{
    error::Error,
    ffi::{OsStr, OsString},
};

struct ProbeConfig {
    generation: GenerationConfig,
    workers: usize,
    backend: Option<power_cpu_llm::engine::execution::MatrixBackend>,
    prefill_backend: Option<power_cpu_llm::engine::execution::MatrixBackend>,
    split_sockets: bool,
    emit_json: bool,
    include_reasoning: bool,
}
fn config(flags: &[OsString]) -> Result<ProbeConfig, Box<dyn Error>> {
    let mut cfg = GenerationConfig {
        max_new_tokens: 1,
        seed: 7,
        diagnostic_canvas_length: Some(1),
        denoising: DenoisingConfig {
            max_steps: 2,
            ..Default::default()
        },
        ..Default::default()
    };
    let mut workers = 1;
    let mut backend = None;
    let mut prefill_backend = None;
    let mut split_sockets = false;
    let mut emit_json = false;
    let mut include_reasoning = false;
    let mut seen = std::collections::HashSet::new();
    let mut flags = flags.iter();
    while let Some(flag) = flags.next() {
        let key = flag.to_str().ok_or("option must be UTF8")?;
        if !seen.insert(key) {
            return Err(format!("duplicate option {key}").into());
        }
        match key {
            "--emit-json" => {
                emit_json = true;
                continue;
            }
            "--include-reasoning" => {
                include_reasoning = true;
                continue;
            }
            "--split-sockets" => {
                split_sockets = true;
                continue;
            }
            _ => {}
        }
        let value = flags
            .next()
            .ok_or("option requires a value")?
            .to_str()
            .ok_or("option value must be UTF8")?;
        match key {
            "--diagnostic-canvas" => cfg.diagnostic_canvas_length = Some(value.parse()?),
            "--max-new-tokens" => cfg.max_new_tokens = value.parse()?,
            "--steps" => cfg.denoising.max_steps = value.parse()?,
            "--seed" => cfg.seed = value.parse()?,
            "--encoder-chunk" => {
                cfg.encoder_chunk_tokens = value.parse()?;
                if cfg.encoder_chunk_tokens == 0 {
                    return Err("--encoder-chunk must be positive".into());
                }
            }
            "--workers" => workers = value.parse()?,
            "--matrix-backend" => backend = Some(super::trace::matrix_backend(value)?),
            "--prefill-backend" => prefill_backend = Some(super::trace::matrix_backend(value)?),
            "--canvas-policy" if value == "model" => cfg.diagnostic_canvas_length = None,
            _ => return Err(format!("unknown probe-generation option {key}").into()),
        }
    }
    if seen.contains("--canvas-policy") && seen.contains("--diagnostic-canvas") {
        return Err("canvas-policy and diagnostic-canvas are mutually exclusive".into());
    }
    if workers == 0 {
        return Err("workers must be positive".into());
    }
    cfg.denoising.validate()?;
    if cfg.diagnostic_canvas_length == Some(0) {
        return Err("diagnostic canvas must be positive".into());
    }
    Ok(ProbeConfig {
        generation: cfg,
        workers,
        backend,
        prefill_backend,
        split_sockets,
        emit_json,
        include_reasoning,
    })
}

pub(crate) fn probe_generation(
    path: &OsStr,
    prompt: &OsStr,
    flags: &[OsString],
) -> Result<(), Box<dyn Error>> {
    let options = config(flags)?;
    let cfg = options.generation;
    let prompt = prompt.to_str().ok_or("user text must be UTF8")?;
    // `@path` reads the user text from a file (long-context diagnostics).
    let from_file;
    let prompt = match prompt.strip_prefix('@') {
        Some(path) => {
            from_file = std::fs::read_to_string(path)?;
            from_file.as_str()
        }
        None => prompt,
    };
    let file = ModelFile::open(path)?;
    let model = ModelDefinition::bind(file.inventory())?;
    let tokenizer = Tokenizer::from_vocabulary(&model.vocabulary)?;
    let template = ChatTemplate::from_gguf(&model.vocabulary, &tokenizer)?;
    let request = ChatRequest::from_openai_json(
        &serde_json::json!({"messages":[{"role":"user","content":prompt}]}),
    )?;
    let prepared = template.prepare(&tokenizer, &request)?;
    // Diagnostics: POWER_LLM_DUMP_PROMPT_IDS=<file> writes the prompt IDs as JSON.
    if let Some(path) = std::env::var_os("POWER_LLM_DUMP_PROMPT_IDS") {
        std::fs::write(path, serde_json::to_string(&prepared.ids)?)?;
    }
    let mut assembler = CompletionAssembler::new(&tokenizer, &prepared)?;
    eprintln!(
        "Text-chat diagnostic: stored GGUF template, {} prompt tokens; canvas {:?}, max output {}, max steps {}, seed {}.",
        prepared.ids.len(),
        cfg.diagnostic_canvas_length,
        cfg.max_new_tokens,
        cfg.denoising.max_steps,
        cfg.seed
    );
    eprintln!(
        "Slow batched reference path. Short/reduced-step diagnostics do NOT establish trained full-canvas response quality."
    );
    let mut runtime = if let (Some(backend), true) = (options.backend, options.split_sockets) {
        ReferenceText::new_split_sockets(
            &file,
            &model,
            ReferenceLimits::for_context(&model.spec, model.spec.context),
            options.workers,
            backend,
        )?
    } else if let Some(backend) = options.backend {
        ReferenceText::new_with_backend(
            &file,
            &model,
            // A bound, not a reservation: admits any request the model's
            // context admits.
            ReferenceLimits::for_context(&model.spec, model.spec.context),
            options.workers,
            backend,
        )?
    } else if options.workers > 1 {
        ReferenceText::new_parallel(&file, &model, ReferenceLimits::default(), options.workers)?
    } else {
        ReferenceText::new_batched(&file, &model, ReferenceLimits::default())?
    };
    runtime.set_prefill_backend(options.prefill_backend)?;
    if let Some(cpus) = runtime.worker_cpus() {
        eprintln!("Pinned projection worker CPUs {cpus:?}; inherited memory-node policy.");
    }
    eprintln!(
        "Matrix backend: {} (requested {:?}).",
        runtime.matrix_backend(),
        options.backend
    );
    let mut session = GenerationSession::new(&runtime, &prepared.ids, cfg)?;
    let mut generation_error = None;
    while !session.is_finished() {
        let event = match session.advance() {
            Ok(event) => event,
            Err(error) => {
                generation_error = Some(error);
                break;
            }
        };
        assembler.observe(&event);
        match event {
            Event::Encoded {
                kind,
                tokens,
                cached_tokens,
            } => eprintln!("Encoded {kind:?}: {tokens} tokens; cache prefix {cached_tokens}."),
            Event::Step {
                canvas_index,
                report,
            } => eprintln!(
                "Canvas {canvas_index}, step {}: temperature {:.6}, mean entropy {:.6}, stop {:?}.",
                report.step, report.temperature, report.mean_entropy, report.finish_reason
            ),
            Event::Commit {
                canvas_index,
                tokens,
                finish_reason,
            } => eprintln!(
                "Committed canvas {canvas_index}: {} tokens; finish {finish_reason:?}.",
                tokens.len()
            ),
        }
    }
    let output = session.result().expect("finished session");
    let completion = assembler.finish(output, options.include_reasoning);
    eprintln!(
        "Committed diagnostic token IDs: {:?}",
        completion.committed_ids
    );
    eprintln!("Committed diagnostic content: {:?}", completion.content);
    if options.include_reasoning {
        eprintln!("Committed diagnostic reasoning: {:?}", completion.reasoning);
    }
    eprintln!(
        "Parse status: {:?}; UTF8 replacements: {}.",
        completion.parse_status, completion.replaced_invalid_utf8
    );
    eprintln!(
        "Completion finish: {:?}; generation finish: {:?}; usage: {:?}",
        completion.finish, completion.generation_finish, completion.usage
    );
    eprintln!("NOT a full-canvas quality/throughput benchmark or an HTTP endpoint.");
    // Diagnostics: POWER_LLM_STATS=1 prints engine stage and pool timing totals.
    if std::env::var_os("POWER_LLM_STATS").is_some() {
        for (stage, seconds) in power_cpu_llm::engine::stats::take() {
            eprintln!("  stage {stage:<40} {seconds:>8.2}s");
        }
        for (stage, runs, wall, busy) in power_cpu_llm::engine::stats::take_stage_pool() {
            if runs > 0 {
                eprintln!(
                    "  pool in {stage:44} {runs:7} submissions {wall:8.2}s wall {busy:9.2}s task"
                );
            }
        }
        let (runs, wall, capacity, busy) = power_cpu_llm::engine::stats::take_pool();
        eprintln!(
            "  pool: {runs} submissions, {wall:.2}s wall, busy {:.0}% of submitted worker time",
            100.0 * busy / capacity.max(1e-9)
        );
    }
    if options.emit_json {
        println!("{}", completion.to_json());
    }
    if let Some(error) = generation_error {
        return Err(error.into());
    }
    if let CompletionFinish::Error(error) = completion.finish {
        return Err(error.into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defaults_and_checked_options_are_explicitly_diagnostic() {
        let options = config(&[]).unwrap();
        assert!(!options.emit_json);
        assert!(!options.include_reasoning);
        let json_flags: Vec<OsString> = ["--emit-json", "--workers", "5", "--include-reasoning"]
            .into_iter()
            .map(Into::into)
            .collect();
        let json_options = config(&json_flags).unwrap();
        assert!(json_options.emit_json);
        assert!(json_options.include_reasoning);
        assert_eq!(json_options.workers, 5);
        let many_workers: Vec<OsString> = ["--workers", "20"].into_iter().map(Into::into).collect();
        assert_eq!(config(&many_workers).unwrap().workers, 20);
        for flags in [
            vec!["--emit-json", "--emit-json"],
            vec!["--include-reasoning", "--include-reasoning"],
            vec!["--emit-json", "true"],
        ] {
            assert!(config(&flags.into_iter().map(Into::into).collect::<Vec<_>>()).is_err());
        }
        assert_eq!(options.workers, 1);
        let cfg = options.generation;
        assert_eq!(cfg.diagnostic_canvas_length, Some(1));
        assert_eq!(cfg.max_new_tokens, 1);
        assert_eq!(cfg.denoising.max_steps, 2);
        let args: Vec<OsString> = [
            "--diagnostic-canvas",
            "4",
            "--max-new-tokens",
            "8",
            "--steps",
            "3",
            "--seed",
            "1",
        ]
        .into_iter()
        .map(Into::into)
        .collect();
        let cfg = config(&args).unwrap().generation;
        assert_eq!(cfg.diagnostic_canvas_length, Some(4));
        assert_eq!(cfg.max_new_tokens, 8);
        assert_eq!(cfg.seed, 1);
        let full: Vec<OsString> = [
            "--canvas-policy",
            "model",
            "--steps",
            "48",
            "--workers",
            "5",
        ]
        .into_iter()
        .map(Into::into)
        .collect();
        let options = config(&full).unwrap();
        assert_eq!(options.generation.diagnostic_canvas_length, None);
        assert_eq!(options.generation.denoising.max_steps, 48);
        assert_eq!(options.workers, 5);
        assert_eq!(options.backend, None);
        let fma: Vec<OsString> = ["--matrix-backend", "power9-fma"]
            .into_iter()
            .map(Into::into)
            .collect();
        assert_eq!(
            config(&fma).unwrap().backend,
            Some(power_cpu_llm::engine::execution::MatrixBackend::Power9Fma)
        );
        for flags in [
            vec!["--matrix-backend", "native"],
            vec!["--matrix-backend", "auto", "--matrix-backend", "auto"],
            vec!["--workers", "0"],
            vec!["--workers", "2", "--workers", "2"],
            vec!["--canvas-policy", "bad"],
            vec!["--canvas-policy", "model", "--diagnostic-canvas", "256"],
            vec!["--steps"],
            vec!["--steps", "0"],
            vec!["--steps", "2", "--steps", "3"],
            vec!["--diagnostic-canvas", "0"],
            vec!["--unknown", "1"],
            vec!["--seed", "-1"],
        ] {
            assert!(config(&flags.into_iter().map(Into::into).collect::<Vec<_>>()).is_err());
        }
    }
}
