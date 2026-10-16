mod cli;
use cli::probe::{probe_forward, probe_model};
use cli::{
    full_canvas, generate, long_context, nemotron as nemotron_cli, profile, prompt_trace, serve,
    trace,
};

use power_cpu_llm::engine::{diffusion, gguf, model, nemotron, operators, qwen35, text, weights};
use power_cpu_llm::kernels::{
    ActivationBlock, BLOCK_SIZE, Backend, Kernels, Q4WeightBlock, Q8WeightBlock,
};
use std::error::Error;
use std::ffi::OsString;
use std::hint::black_box;
use std::time::Instant;

/// libm's `pow`, for the two dependencies that call it (tokio's scheduler
/// statistics and minijinja's `**` operator), so the binary imports no libm
/// function. Bit-identical to the system glibc 2.28 `pow`; errno is not set.
#[unsafe(no_mangle)]
pub extern "C" fn pow(x: f64, y: f64) -> f64 {
    power_cpu_llm::libm::pow(x, y)
}

/// libm's `fmod`, which Rust's float `%` calls (minijinja's float `%` and
/// `//`), so the static binary contains no C maths function. Exact, and
/// bit-identical to the system glibc 2.28 `fmod`; errno is not set.
#[unsafe(no_mangle)]
pub extern "C" fn fmod(x: f64, y: f64) -> f64 {
    power_cpu_llm::libm::fmod(x, y)
}

/// Standalone POWER9 inference engine for Gemma 4 and DiffusionGemma GGUF models.
#[derive(clap::Parser)]
#[command(
    name = "power-cpu-llm",
    version,
    after_help = "Docs: docs/server.md (serving), docs/models.md (models), docs/benchmarks.md (speed)."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Serve OpenAI-compatible chat completions over HTTP
    Serve(serve::ServeArgs),
    /// Text-chat generation diagnostic (prints stage times, token IDs and text)
    #[command(
        name = "probe-generation",
        after_help = "Usage: power-cpu-llm probe-generation <model.gguf> <user-text|@file> [--diagnostic-canvas N | --canvas-policy model] [--max-new-tokens N] [--steps N] [--seed N] [--workers N] [--matrix-backend portable|auto|power9-fma|power9-int8|power9-int8x2] [--prefill-backend B] [--split-sockets] [--encoder-chunk N] [--emit-json] [--include-reasoning] [--draft-model mtp.gguf] [--draft-max N]"
    )]
    ProbeGeneration(Diagnostic),
    /// Print GGUF metadata and tensor inventory
    #[command(
        name = "inspect",
        after_help = "Usage: power-cpu-llm inspect <model.gguf> [--tensors] [--metadata]"
    )]
    Inspect(Diagnostic),
    /// Check layouts and decode sample rows of every tensor
    #[command(
        name = "check-weights",
        after_help = "Usage: power-cpu-llm check-weights <model.gguf>"
    )]
    CheckWeights(Diagnostic),
    /// Operator smoke test with real weights and synthetic activations
    #[command(
        name = "probe-model",
        after_help = "Usage: power-cpu-llm probe-model <model.gguf>"
    )]
    ProbeModel(Diagnostic),
    /// Slow 30-layer reference forward diagnostic (DiffusionGemma)
    #[command(
        name = "probe-forward",
        after_help = "Usage: power-cpu-llm probe-forward <model.gguf>"
    )]
    ProbeForward(Diagnostic),
    /// Stage and pool timing of a DiffusionGemma decoder pass
    #[command(
        name = "profile-decode",
        after_help = "Usage: power-cpu-llm profile-decode <model.gguf> [--workers N] [--matrix-backend portable|auto|power9-fma|power9-int8|power9-int8x2]"
    )]
    ProfileDecode(Diagnostic),
    /// Export a rowwise reference trace
    #[command(
        name = "trace-forward",
        after_help = "Usage: power-cpu-llm trace-forward <model.gguf> <NEW-output-directory> [--self-condition-from <raw-logits.f32>]"
    )]
    TraceForward(Diagnostic),
    /// Export a batched trace
    #[command(
        name = "trace-batched",
        after_help = "Usage: power-cpu-llm trace-batched <model.gguf> <NEW-output-directory> [--self-condition-from <raw-logits.f32>] [--workers N] [--matrix-backend portable|auto|power9-fma|power9-int8|power9-int8x2]"
    )]
    TraceBatched(Diagnostic),
    /// Export a full-canvas trace for the oracle comparison
    #[command(
        name = "trace-full-canvas",
        after_help = "Usage: power-cpu-llm trace-full-canvas <model.gguf> <full-canvas-fixture-dir> <processed-self-conditioning.f32> <NEW-output-directory> [--workers N] [--matrix-backend portable|auto|power9-fma|power9-int8|power9-int8x2]"
    )]
    TraceFullCanvas(Diagnostic),
    /// Export a Gemma 4 prompt trace for the oracle comparison
    #[command(
        name = "trace-prompt",
        after_help = "Usage: power-cpu-llm trace-prompt <model.gguf> <prompt-golden-dir> <NEW-output-directory> [--workers N] [--matrix-backend portable|auto|power9-fma|power9-int8|power9-int8x2] [--prefill-backend B]"
    )]
    TracePrompt(Diagnostic),
    /// Export a long-context trace for the oracle comparison
    #[command(
        name = "trace-long-context",
        after_help = "Usage: power-cpu-llm trace-long-context <model.gguf> <long-context-fixture-dir> <NEW-output-directory> [--workers N] [--matrix-backend portable|auto|power9-fma|power9-int8|power9-int8x2]"
    )]
    TraceLongContext(Diagnostic),
    /// Reference hybrid-model forward pass (Nemotron 3.5, Qwen 3.5/3.6) over token IDs (top next-token logits)
    #[command(
        name = "hybrid-logits",
        after_help = "Usage: power-cpu-llm hybrid-logits <model.gguf> <token-id,token-id,...|text:TEXT> [--workers N] [--matrix-backend portable|auto|power9-fma|power9-int8|power9-int8x2] [--trace NEW-directory]"
    )]
    HybridLogits(Diagnostic),
    /// Greedy hybrid-model chat completion of one user message (Nemotron 3.5, Qwen 3.5/3.6)
    #[command(
        name = "hybrid-chat",
        after_help = "Usage: power-cpu-llm hybrid-chat <model.gguf> <user-text|@file> [--max-new-tokens N] [--think on|off] [--workers N] [--matrix-backend portable|auto|power9-fma|power9-int8|power9-int8x2]"
    )]
    HybridChat(Diagnostic),
    /// Print the DiffusionGemma denoising configuration
    #[command(
        name = "diffusion-config",
        after_help = "Usage: power-cpu-llm diffusion-config (no arguments)"
    )]
    DiffusionConfig(Diagnostic),
    /// Row-dot kernel microbenchmark
    #[command(
        name = "bench",
        after_help = "Usage: power-cpu-llm bench [q8|q4] [auto|scalar|power9] [iterations] [threads]"
    )]
    Bench(Diagnostic),
}

/// Arguments of a diagnostic command, passed to its own parser.
#[derive(clap::Args)]
struct Diagnostic {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0..)]
    args: Vec<OsString>,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let (name, rest) = match <Cli as clap::Parser>::parse().command {
        Command::Serve(args) => return serve::serve(args),
        Command::ProbeGeneration(d) => ("probe-generation", d.args),
        Command::Inspect(d) => ("inspect", d.args),
        Command::CheckWeights(d) => ("check-weights", d.args),
        Command::ProbeModel(d) => ("probe-model", d.args),
        Command::ProbeForward(d) => ("probe-forward", d.args),
        Command::ProfileDecode(d) => ("profile-decode", d.args),
        Command::TraceForward(d) => ("trace-forward", d.args),
        Command::TraceBatched(d) => ("trace-batched", d.args),
        Command::TraceFullCanvas(d) => ("trace-full-canvas", d.args),
        Command::TracePrompt(d) => ("trace-prompt", d.args),
        Command::TraceLongContext(d) => ("trace-long-context", d.args),
        Command::HybridLogits(d) => ("hybrid-logits", d.args),
        Command::HybridChat(d) => ("hybrid-chat", d.args),
        Command::DiffusionConfig(d) => ("diffusion-config", d.args),
        Command::Bench(d) => ("bench", d.args),
    };
    let mut args = vec![OsString::from(name)];
    args.extend(rest);
    diagnostic(args)
}

/// The diagnostic commands, with their own argument parsing.
fn diagnostic(args: Vec<OsString>) -> Result<(), Box<dyn Error>> {
    match args.first().and_then(|a| a.to_str()) {
        Some("inspect") => {
            let path = args.get(1).ok_or("inspect requires a GGUF file path")?;
            let mut tensors = false;
            let mut metadata = false;
            for arg in &args[2..] {
                match arg.to_str() {
                    Some("--tensors") => tensors = true,
                    Some("--metadata") => metadata = true,
                    _ => return Err("unknown inspect option".into()),
                }
            }
            let model = gguf::inspect(path)?;
            println!("File: {}", std::path::Path::new(path).display());
            println!(
                "GGUF v{}, architecture: {}",
                model.version,
                model.architecture().unwrap_or("<missing>")
            );
            println!(
                "{} tensors, {} metadata keys, {:.3} GiB file",
                model.tensors.len(),
                model.metadata.len(),
                model.file_size as f64 / (1u64 << 30) as f64
            );
            println!(
                "Alignment: {}, tensor data begins at byte {}",
                model.alignment, model.data_offset
            );
            println!("Tensor types (parsing support, not execution support):");
            for (kind, (count, bytes)) in model.quantization_summary() {
                println!(
                    "  {kind:12} {count:5} tensors, {:8.3} GiB",
                    bytes as f64 / (1u64 << 30) as f64
                );
            }
            let prefix = model.architecture().map(|a| format!("{a}."));
            for (key, value) in &model.metadata {
                if metadata
                    || key == "general.name"
                    || key.starts_with("diffusion.")
                    || prefix.as_ref().is_some_and(|p| key.starts_with(p))
                {
                    println!("  {key} = {}", value.summary());
                }
            }
            if tensors {
                for t in &model.tensors {
                    println!(
                        "  {}: {} {:?}, offset={}, bytes={}",
                        t.name, t.type_name, t.shape, t.file_offset, t.size_bytes
                    );
                }
            }
            Ok(())
        }
        Some("check-weights") => {
            if args.len() != 2 {
                return Err("check-weights requires one GGUF file path".into());
            }
            let model = weights::ModelFile::open(&args[1])?;
            let inventory = model.inventory();
            if inventory.architecture() == Some(nemotron::ARCHITECTURE) {
                let definition = nemotron::ModelDefinition::bind(inventory)?;
                let count = |kind| {
                    definition
                        .spec
                        .layers
                        .iter()
                        .filter(|&&k| k == kind)
                        .count()
                };
                println!(
                    "Bound {}: {} Mamba-2, {} attention and {} MoE layers, {} MTP block(s)",
                    nemotron::ARCHITECTURE,
                    count(nemotron::LayerKind::Mamba),
                    count(nemotron::LayerKind::Attention),
                    count(nemotron::LayerKind::Moe),
                    definition.spec.mtp_layers
                );
            } else if inventory.architecture() == Some(qwen35::ARCHITECTURE) {
                let definition = qwen35::ModelDefinition::bind(inventory)?;
                let count = |kind| {
                    definition
                        .spec
                        .layers
                        .iter()
                        .filter(|&&k| k == kind)
                        .count()
                };
                println!(
                    "Bound {}: {} Gated DeltaNet and {} attention layers (each with MoE), {} MTP block(s)",
                    qwen35::ARCHITECTURE,
                    count(qwen35::LayerKind::Linear),
                    count(qwen35::LayerKind::Attention),
                    definition.spec.mtp_layers
                );
            } else {
                model::ModelDefinition::bind(inventory)?;
                println!("Bound {}", inventory.architecture().unwrap_or("?"));
            }
            let mut checked = 0;
            let mut checksum = 0.0_f64;
            for tensor in &model.inventory().tensors {
                let count = weights::row_count(tensor)?;
                let mut indices = vec![0, count / 2, count - 1];
                indices.sort_unstable();
                indices.dedup();
                for row in indices {
                    let decoded = model.read_row(&tensor.name, row)?.decode()?;
                    checksum += decoded.iter().map(|&x| f64::from(x)).sum::<f64>();
                    checked += 1;
                }
            }
            println!(
                "{} tensors: checked layouts and decoded {checked} first/middle/last rows",
                model.inventory().tensors.len()
            );
            println!("All sampled weights finite; diagnostic sum={checksum:.9e}");
            println!("Sampling check only, not a complete file hash or forward-pass validation.");
            Ok(())
        }
        Some("probe-model") => {
            if args.len() != 2 {
                return Err("probe-model requires one GGUF file path".into());
            }
            probe_model(&args[1])
        }
        Some("probe-forward") => {
            if args.len() != 2 {
                return Err("probe-forward requires one GGUF file path".into());
            }
            probe_forward(&args[1])
        }
        Some("trace-forward" | "trace-batched") => {
            if args.len() < 3 {
                return Err("trace-forward|trace-batched <model.gguf> <NEW-output-directory> [--self-condition-from <raw-logits.f32>] [--workers N] [--matrix-backend portable|auto|power9-fma] (workers/backend: batched only)".into());
            }
            let options = trace::options(&args[3..], args[0] == "trace-batched")?;
            trace::trace_forward(
                std::path::Path::new(&args[1]),
                std::path::Path::new(&args[2]),
                options.previous,
                args[0] == "trace-batched",
                options.workers,
                options.backend,
            )
        }
        Some("trace-full-canvas") => {
            const USAGE: &str = "trace-full-canvas <model.gguf> <full-canvas-fixture-dir> <processed-self-conditioning.f32> <NEW-output-directory> [--workers N] [--matrix-backend portable|auto|power9-fma|power9-int8|power9-int8x2]";
            if args.len() < 5 || !args[5..].len().is_multiple_of(2) {
                return Err(USAGE.into());
            }
            let (mut workers, mut backend) = (None, None);
            for pair in args[5..].as_chunks::<2>().0 {
                let value = pair[1].to_str().ok_or("option value must be UTF-8")?;
                match pair[0].to_str() {
                    Some("--workers") if workers.is_none() => {
                        let n: usize = value.parse()?;
                        if n == 0 {
                            return Err("workers must be at least one".into());
                        }
                        workers = Some(n);
                    }
                    Some("--matrix-backend") if backend.is_none() => {
                        backend = Some(trace::matrix_backend(value)?)
                    }
                    _ => return Err(USAGE.into()),
                }
            }
            full_canvas::trace_full_canvas(
                std::path::Path::new(&args[1]),
                std::path::Path::new(&args[2]),
                std::path::Path::new(&args[3]),
                std::path::Path::new(&args[4]),
                workers.unwrap_or(1),
                backend.unwrap_or(power_cpu_llm::engine::execution::MatrixBackend::Portable),
            )
        }
        Some("trace-prompt") => {
            const USAGE: &str = "trace-prompt <model.gguf> <prompt-golden-dir> <NEW-output-directory> [--workers N] [--matrix-backend portable|auto|power9-fma|power9-int8|power9-int8x2] [--prefill-backend B]";
            if args.len() < 4 || !args[4..].len().is_multiple_of(2) {
                return Err(USAGE.into());
            }
            let (mut workers, mut backend, mut prefill_backend) = (None, None, None);
            for pair in args[4..].as_chunks::<2>().0 {
                let value = pair[1].to_str().ok_or("option value must be UTF-8")?;
                match pair[0].to_str() {
                    Some("--workers") if workers.is_none() => {
                        workers = Some(value.parse::<usize>()?)
                    }
                    Some("--matrix-backend") if backend.is_none() => {
                        backend = Some(trace::matrix_backend(value)?)
                    }
                    Some("--prefill-backend") if prefill_backend.is_none() => {
                        prefill_backend = Some(trace::matrix_backend(value)?)
                    }
                    _ => return Err(USAGE.into()),
                }
            }
            prompt_trace::trace_prompt(
                std::path::Path::new(&args[1]),
                std::path::Path::new(&args[2]),
                std::path::Path::new(&args[3]),
                workers.unwrap_or(1),
                backend.unwrap_or(power_cpu_llm::engine::execution::MatrixBackend::Portable),
                prefill_backend,
            )
        }
        Some("trace-long-context") => {
            const USAGE: &str = "trace-long-context <model.gguf> <long-context-fixture-dir> <NEW-output-directory> [--workers N] [--matrix-backend portable|auto|power9-fma|power9-int8|power9-int8x2]";
            if args.len() < 4 || !args[4..].len().is_multiple_of(2) {
                return Err(USAGE.into());
            }
            let (mut workers, mut backend) = (None, None);
            for pair in args[4..].as_chunks::<2>().0 {
                let value = pair[1].to_str().ok_or("option value must be UTF-8")?;
                match pair[0].to_str() {
                    Some("--workers") if workers.is_none() => {
                        workers = Some(value.parse::<usize>()?)
                    }
                    Some("--matrix-backend") if backend.is_none() => {
                        backend = Some(trace::matrix_backend(value)?)
                    }
                    _ => return Err(USAGE.into()),
                }
            }
            long_context::trace_long_context(
                std::path::Path::new(&args[1]),
                std::path::Path::new(&args[2]),
                std::path::Path::new(&args[3]),
                workers.unwrap_or(1),
                backend.unwrap_or(power_cpu_llm::engine::execution::MatrixBackend::Portable),
            )
        }
        Some("profile-decode") => {
            const USAGE: &str = "profile-decode <model.gguf> [--workers N] [--matrix-backend portable|auto|power9-fma|power9-int8|power9-int8x2]";
            if args.len() < 2 || !args[2..].len().is_multiple_of(2) {
                return Err(USAGE.into());
            }
            let (mut workers, mut backend) = (None, None);
            for pair in args[2..].as_chunks::<2>().0 {
                let value = pair[1].to_str().ok_or("option value must be UTF-8")?;
                match pair[0].to_str() {
                    Some("--workers") if workers.is_none() => {
                        workers = Some(value.parse::<usize>()?)
                    }
                    Some("--matrix-backend") if backend.is_none() => {
                        backend = Some(trace::matrix_backend(value)?)
                    }
                    _ => return Err(USAGE.into()),
                }
            }
            profile::profile_decode(
                std::path::Path::new(&args[1]),
                workers.unwrap_or(1),
                backend.unwrap_or(power_cpu_llm::engine::execution::MatrixBackend::Portable),
            )
        }
        Some("probe-generation") => {
            if args.len() < 3 {
                return Err("probe-generation <model.gguf> <user-text|@file> [--diagnostic-canvas N | --canvas-policy model] [--max-new-tokens N] [--steps N] [--seed N] [--workers N] [--matrix-backend portable|auto|power9-fma|power9-int8|power9-int8x2]".into());
            }
            generate::probe_generation(&args[1], &args[2], &args[3..])
        }
        Some("hybrid-logits") => nemotron_cli::logits(&args[1..]),
        Some("hybrid-chat") => nemotron_cli::chat(&args[1..]),
        Some("diffusion-config") => {
            if args.len() != 1 {
                return Err("diffusion-config takes no arguments".into());
            }
            let config = diffusion::DenoisingConfig::default();
            println!("Initial model target: DiffusionGemma-26B-A4B-it (text only)");
            println!(
                "Canvas: {} positions, vocabulary: {}",
                diffusion::CANVAS_LENGTH,
                diffusion::VOCAB_SIZE
            );
            println!(
                "Sampler: entropy-bound, max {} denoising steps",
                config.max_steps
            );
            println!(
                "Temperature: {:.6} first, {:.6} last (remaining-step schedule)",
                config.temperature(config.max_steps)?,
                config.temperature(1)?
            );
            println!(
                "Entropy bound: {}, mean-entropy stopping threshold: {}",
                config.entropy_bound, config.confidence_threshold
            );
            println!(
                "Required matching previous argmax canvases: {}",
                config.stability_steps
            );
            println!("EOS token IDs: {:?}", diffusion::EOS_TOKEN_IDS);
            println!(
                "Reference forward/sampler/multi-canvas path implemented; full-canvas quality/parity and serving remain unvalidated/unimplemented."
            );
            Ok(())
        }
        Some("bench") => {
            if args.len() > 5 {
                return Err("too many bench arguments".into());
            }
            let quant = args.get(1).and_then(|s| s.to_str()).unwrap_or("q8");
            if !matches!(quant, "q8" | "q4") {
                return Err("quant must be q8 or q4".into());
            }
            let backend = match args.get(2).and_then(|s| s.to_str()).unwrap_or("auto") {
                "auto" => Backend::Auto,
                "scalar" => Backend::Scalar,
                "power9" => Backend::Power9,
                _ => return Err("backend must be auto, scalar, or power9".into()),
            };
            let iterations: u32 = args
                .get(3)
                .map(|s| s.to_str().ok_or("iterations must be UTF-8"))
                .transpose()?
                .unwrap_or("100")
                .parse()?;
            if iterations == 0 {
                return Err("iterations must be positive".into());
            }
            let threads: usize = args
                .get(4)
                .map(|s| s.to_str().ok_or("threads must be UTF-8"))
                .transpose()?
                .unwrap_or("1")
                .parse()?;
            let available = std::thread::available_parallelism()?.get();
            if threads == 0 || threads > available || threads > 256 {
                return Err(format!(
                    "threads must be in 1..={} for this process",
                    available.min(256)
                )
                .into());
            }
            bench(quant, Kernels::new(backend)?, iterations, threads)
        }
        _ => Err("unknown command; use --help".into()),
    }
}

fn bench(
    quant: &str,
    kernels: Kernels,
    iterations: u32,
    threads: usize,
) -> Result<(), Box<dyn Error>> {
    const ROWS: usize = 256;
    const BLOCKS: usize = 128;
    let mut state = 0x1234_5678u32;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        state as u8
    };
    let activations = (0..BLOCKS)
        .map(|_| {
            ActivationBlock::from_quantized(&std::array::from_fn(|_| next() as i8), 1.0 / 128.0)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let scalar = Kernels::new(Backend::Scalar)?;

    // A call crosses into C once per row. The benchmark includes scale
    // application and block function calls; it is NOT a tiled GEMM roof test.
    let q8;
    let q4;
    let evaluate: Box<dyn Fn(Kernels, usize) -> f32 + Sync> = if quant == "q8" {
        q8 = (0..ROWS * BLOCKS)
            .map(|_| {
                Q8WeightBlock::from_quantized(&std::array::from_fn(|_| next() as i8), 1.0 / 16.0)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Box::new(|k: Kernels, row| {
            k.q8_dot(&q8[row * BLOCKS..(row + 1) * BLOCKS], &activations)
                .unwrap()
        })
    } else {
        q4 = (0..ROWS * BLOCKS)
            .map(|_| {
                Q4WeightBlock::from_quantized(
                    &std::array::from_fn(|_| (next() & 15) as i8 - 8),
                    1.0 / 16.0,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        Box::new(|k: Kernels, row| {
            k.q4_dot(&q4[row * BLOCKS..(row + 1) * BLOCKS], &activations)
                .unwrap()
        })
    };
    for row in 0..ROWS {
        if evaluate(kernels, row) != evaluate(scalar, row) {
            return Err(format!("kernel/reference mismatch in row {row}").into());
        }
    }
    let start = Instant::now();
    // Workers inherit the caller's CPU affinity and NUMA policy. No global
    // thread pool can escape the selected CPU set. Timings include startup.
    let checksum: f64 = std::thread::scope(|scope| {
        let mut workers = Vec::new();
        for worker in 0..threads {
            let evaluate = &evaluate;
            workers.push(scope.spawn(move || {
                let mut sum = 0.0_f64;
                for _ in 0..iterations {
                    for row in (worker..ROWS).step_by(threads) {
                        sum += f64::from(black_box(evaluate(black_box(kernels), row)));
                    }
                }
                sum
            }));
        }
        workers
            .into_iter()
            .map(|w| w.join().expect("benchmark worker panicked"))
            .sum()
    });
    let elapsed = start.elapsed().as_secs_f64();
    let macs = f64::from(iterations) * (ROWS * BLOCKS * BLOCK_SIZE) as f64;
    println!(
        "{} {}, {threads} thread(s), warm-cache row dots",
        kernels.name(),
        quant
    );
    println!(
        "{ROWS} rows x {} columns, {iterations} iterations",
        BLOCKS * BLOCK_SIZE
    );
    println!(
        "{elapsed:.6} s, {:.3} GMAC/s, checksum={checksum:.6}",
        macs / elapsed / 1e9
    );
    println!("Microbenchmark only: not model throughput or DRAM streaming performance.");
    Ok(())
}
