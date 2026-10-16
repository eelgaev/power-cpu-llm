use clap::Args;
use power_cpu_llm::engine::execution::{MatrixBackend, MatrixKernels, Workers};
use power_cpu_llm::server::{Server, ServerConfig};
use std::{error::Error, net::SocketAddr, num::NonZeroUsize, path::PathBuf};

/// Options of `power-cpu-llm serve` (see docs/server.md).
#[derive(Args, Debug)]
pub struct ServeArgs {
    /// Model GGUF file
    model: PathBuf,
    /// Address to listen on
    #[arg(long, default_value = "127.0.0.1:8080")]
    bind: SocketAddr,
    /// Worker threads [default: 2 per physical core in the affinity mask]
    #[arg(long)]
    workers: Option<NonZeroUsize>,
    /// Matrix backend: portable, auto, power9-fma, power9-int8, power9-int8x2,
    /// portable-int8 or portable-int8x2 [default: power9-int8x2 when available,
    /// else auto]
    #[arg(long, value_parser = backend)]
    matrix_backend: Option<MatrixBackend>,
    /// Backend for multi-token prompt chunks only (same values)
    #[arg(long, value_parser = backend)]
    prefill_backend: Option<MatrixBackend>,
    /// Divide long prompt chunks between the two sockets (each with its own
    /// copy of the weights); run without --membind. Also accepts on/off.
    #[arg(long, num_args = 0..=1, default_missing_value = "on", default_value = "off",
          value_parser = clap::builder::BoolishValueParser::new())]
    split_sockets: bool,
    /// Chat template: `gguf` (the model's own) or a path to a .jinja file
    #[arg(long, default_value = "gguf")]
    template: String,
    /// Model id reported by the API [default: the GGUF file stem]
    #[arg(long)]
    model_id: Option<String>,
    /// Waiting requests allowed in the queue (at most 64)
    #[arg(long, default_value_t = 4)]
    queue: usize,
    /// Serving context limit in tokens [default: the model's context]
    #[arg(long)]
    max_context: Option<usize>,
    /// Prompt tokens per encoder pass (results do not depend on it)
    #[arg(long, default_value = "256")]
    encoder_chunk: NonZeroUsize,
    /// Require `Authorization: Bearer <KEY>` on every request
    #[arg(long, conflicts_with = "api_key_file")]
    api_key: Option<String>,
    /// As --api-key, read from a file (first line; keeps the key out of the
    /// process list)
    #[arg(long)]
    api_key_file: Option<PathBuf>,
}

fn backend(value: &str) -> Result<MatrixBackend, String> {
    super::trace::matrix_backend(value).map_err(|error| error.to_string())
}

pub fn serve(args: ServeArgs) -> Result<(), Box<dyn Error>> {
    let model_id = match args.model_id {
        Some(id) if !id.is_empty() => id,
        _ => args
            .model
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or("model filename must be UTF8 or specify --model-id")?
            .into(),
    };
    let workers = match args.workers {
        Some(workers) => workers.get(),
        // Two hardware threads per physical core (POWER9 SMT2 nearly doubles
        // integer-dot throughput; more threads per core add little).
        None => Workers::available()?
            .len()
            .min(2 * Workers::physical_cores()?),
    };
    // Serving default: the approximate two-plane integer backend
    // (docs/backends.md) when available, else the automatic choice.
    let backend = args.matrix_backend.unwrap_or_else(|| {
        if MatrixKernels::new(MatrixBackend::Power9Int8x2).is_ok() {
            MatrixBackend::Power9Int8x2
        } else {
            MatrixBackend::Auto
        }
    });
    let template = match args.template.as_str() {
        "gguf" => None,
        path => Some(
            std::fs::read_to_string(path).map_err(|error| format!("--template {path}: {error}"))?,
        ),
    };
    let api_key = match (args.api_key, args.api_key_file) {
        (Some(key), _) => Some(key),
        (None, Some(path)) => Some(
            std::fs::read_to_string(&path)
                .map_err(|error| format!("--api-key-file {}: {error}", path.display()))?
                .lines()
                .next()
                .unwrap_or_default()
                .trim()
                .to_owned(),
        ),
        (None, None) => None,
    };
    let config = ServerConfig {
        model_id,
        model_path: args.model,
        workers,
        backend,
        template,
        queue: args.queue,
        max_context: args.max_context,
        encoder_chunk: args.encoder_chunk.get(),
        prefill_backend: args.prefill_backend,
        split_sockets: args.split_sockets,
        api_key,
    };
    Server::load(config)
        .map_err(std::io::Error::other)?
        .serve(args.bind)
}
