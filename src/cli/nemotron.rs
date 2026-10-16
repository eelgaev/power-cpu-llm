//! Hybrid recurrent models (Nemotron 3.5 `nemotron_h_moe`, Qwen 3.5/3.6
//! `qwen35moe`), chosen by the GGUF architecture. `hybrid-logits`: the
//! reference forward pass over token IDs, printing the top next-token logits
//! and optionally writing the residual stream after every layer for an
//! oracle comparison; `hybrid-chat`: greedy completion of one user message.

use super::trace::matrix_backend;
use power_cpu_llm::engine::chat::{ChatRequest, ChatTemplate, SpecialText};
use power_cpu_llm::engine::execution::{Compute, MatrixBackend, Workers, matrix_kernels};
use power_cpu_llm::engine::nemotron::{self, Vocabulary, tokenizer::Tokenizer};
use power_cpu_llm::engine::operators;
use power_cpu_llm::engine::qwen35;
use power_cpu_llm::engine::stats;
use power_cpu_llm::engine::tokenizer::Utf8Stream;
use power_cpu_llm::engine::weights::ModelFile;
use std::error::Error;
use std::ffi::OsString;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

/// A bound hybrid model.
enum Hybrid<'a> {
    Nemotron(nemotron::ModelDefinition<'a>),
    Qwen(qwen35::ModelDefinition<'a>),
}
enum HybridState {
    Nemotron(nemotron::forward::State),
    Qwen(qwen35::forward::State),
}
impl<'a> Hybrid<'a> {
    fn bind(file: &'a ModelFile) -> Result<Self, Box<dyn Error>> {
        let inventory = file.inventory();
        Ok(match inventory.architecture() {
            Some(nemotron::ARCHITECTURE) => {
                Self::Nemotron(nemotron::ModelDefinition::bind(inventory)?)
            }
            Some(qwen35::ARCHITECTURE) => Self::Qwen(qwen35::ModelDefinition::bind(inventory)?),
            other => return Err(format!("not a hybrid model: {other:?}").into()),
        })
    }
    fn vocabulary(&self) -> &Vocabulary<'a> {
        match self {
            Self::Nemotron(m) => &m.vocabulary,
            Self::Qwen(m) => &m.vocabulary,
        }
    }
    fn state(&self) -> HybridState {
        match self {
            Self::Nemotron(m) => HybridState::Nemotron(nemotron::forward::State::new(m)),
            Self::Qwen(m) => HybridState::Qwen(qwen35::forward::State::new(m)),
        }
    }
    fn prepack(&self, file: &ModelFile, compute: Compute<'_>) -> Result<(), operators::Error> {
        match self {
            Self::Nemotron(m) => nemotron::forward::prepack(file, m, compute),
            Self::Qwen(m) => qwen35::forward::prepack(file, m, compute),
        }
    }
    fn forward(
        &self,
        file: &ModelFile,
        state: &mut HybridState,
        tokens: &[u32],
        compute: Compute<'_>,
        observe: &mut dyn FnMut(usize, &[Vec<f32>]),
    ) -> Result<Vec<Vec<f32>>, operators::Error> {
        match (self, state) {
            (Self::Nemotron(m), HybridState::Nemotron(s)) => {
                nemotron::forward::forward(file, m, s, tokens, compute, observe)
            }
            (Self::Qwen(m), HybridState::Qwen(s)) => {
                qwen35::forward::forward(file, m, s, tokens, compute, observe)
            }
            _ => Err(operators::Error::Shape("state of another model")),
        }
    }
    fn logits(
        &self,
        file: &ModelFile,
        hidden: &[f32],
        compute: Compute<'_>,
    ) -> Result<Vec<f32>, operators::Error> {
        match self {
            Self::Nemotron(m) => nemotron::forward::logits(file, m, hidden, compute),
            Self::Qwen(m) => qwen35::forward::logits(file, m, hidden, compute),
        }
    }
}

pub(crate) const USAGE: &str = "hybrid-logits <model.gguf> <token-id,token-id,...|text:TEXT> [--workers N] [--matrix-backend portable|auto|power9-fma|power9-int8|power9-int8x2] [--trace NEW-directory]";

pub(crate) fn logits(args: &[OsString]) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 || !args[2..].len().is_multiple_of(2) {
        return Err(USAGE.into());
    }
    let (mut workers, mut backend, mut trace) = (None, None, None);
    for pair in args[2..].as_chunks::<2>().0 {
        let value = pair[1].to_str().ok_or("option value must be UTF-8")?;
        match pair[0].to_str() {
            Some("--workers") if workers.is_none() => workers = Some(value.parse::<usize>()?),
            Some("--matrix-backend") if backend.is_none() => backend = Some(matrix_backend(value)?),
            Some("--trace") if trace.is_none() => trace = Some(Path::new(&pair[1])),
            _ => return Err(USAGE.into()),
        }
    }
    let file = ModelFile::open(&args[0])?;
    let model = Hybrid::bind(&file)?;
    let tokenizer = Tokenizer::from_vocabulary(model.vocabulary())?;
    let input = args[1].to_str().ok_or("input must be UTF-8")?;
    let tokens = match input.strip_prefix("text:") {
        Some(text) => tokenizer.encode(text, true),
        None => input
            .split(',')
            .map(|t| t.trim().parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?,
    };
    if tokens.is_empty() {
        return Err("no input tokens".into());
    }
    println!("Input {tokens:?}");
    let pool = workers.map(Workers::current).transpose()?;
    let compute = Compute {
        workers: pool.as_ref(),
        kernels: matrix_kernels(backend.unwrap_or(MatrixBackend::Portable))?,
    };
    if let Some(dir) = trace {
        // Never overwrite an existing trace.
        std::fs::create_dir(dir)?;
    }
    let mut failure = None;
    let mut observe = |layer: usize, rows: &[Vec<f32>]| {
        let Some(dir) = trace else { return };
        let result =
            std::fs::File::create(dir.join(format!("layer.{layer}.f32"))).and_then(|mut f| {
                rows.iter()
                    .flatten()
                    .try_for_each(|x| f.write_all(&x.to_le_bytes()))
            });
        if let Err(e) = result {
            failure.get_or_insert(e);
        }
    };
    let start = Instant::now();
    let mut state = model.state();
    let hidden = model.forward(&file, &mut state, &tokens, compute, &mut observe)?;
    if let Some(e) = failure {
        return Err(e.into());
    }
    let last = hidden.last().expect("nonempty prompt");
    let logits = model.logits(&file, last, compute)?;
    let elapsed = start.elapsed().as_secs_f64();
    if let Some(dir) = trace {
        let bytes: Vec<u8> = logits.iter().flat_map(|x| x.to_le_bytes()).collect();
        std::fs::write(dir.join("logits.f32"), bytes)?;
    }
    let mut order: Vec<usize> = (0..logits.len()).collect();
    order.sort_by(|&a, &b| logits[b].total_cmp(&logits[a]));
    println!(
        "{} positions in {elapsed:.2} s; top next tokens after the last position:",
        tokens.len()
    );
    for &id in &order[..10] {
        println!(
            "  {id:6} {:12.6} {:?}",
            logits[id],
            tokenizer.decode(&[id as u32], false)?
        );
    }
    Ok(())
}

pub(crate) const CHAT_USAGE: &str = "hybrid-chat <model.gguf> <user-text|@file> [--max-new-tokens N] [--think on|off] [--workers N] [--matrix-backend portable|auto|power9-fma|power9-int8|power9-int8x2]";

/// Greedy chat completion of one user message, streamed to stdout.
pub(crate) fn chat(args: &[OsString]) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 || !args[2..].len().is_multiple_of(2) {
        return Err(CHAT_USAGE.into());
    }
    let text = args[1].to_str().ok_or("user text must be UTF-8")?;
    let text = match text.strip_prefix('@') {
        Some(path) => std::fs::read_to_string(path)?,
        None => text.to_owned(),
    };
    let (mut max_new, mut think, mut workers, mut backend) = (256usize, true, None, None);
    for pair in args[2..].as_chunks::<2>().0 {
        let value = pair[1].to_str().ok_or("option value must be UTF-8")?;
        match pair[0].to_str() {
            Some("--max-new-tokens") => max_new = value.parse()?,
            Some("--think") => {
                think = match value {
                    "on" => true,
                    "off" => false,
                    _ => return Err(CHAT_USAGE.into()),
                }
            }
            Some("--workers") => workers = Some(value.parse::<usize>()?),
            Some("--matrix-backend") => backend = Some(matrix_backend(value)?),
            _ => return Err(CHAT_USAGE.into()),
        }
    }
    let file = ModelFile::open(&args[0])?;
    let model = Hybrid::bind(&file)?;
    let tokenizer = Tokenizer::from_vocabulary(model.vocabulary())?;
    let vocabulary = model.vocabulary();
    let token = |id: u32| tokenizer.token(id).unwrap_or_default().to_owned();
    let template = ChatTemplate::with_tokens(
        vocabulary.chat_template,
        vec![
            ("bos_token", token(vocabulary.bos)),
            ("eos_token", token(vocabulary.eos)),
            (
                "unk_token",
                vocabulary.unknown.map(token).unwrap_or_default(),
            ),
            ("pad_token", token(vocabulary.padding)),
        ],
        tokenizer.added_tokens().map(str::to_owned).collect(),
    )?;
    let request = ChatRequest {
        messages: vec![serde_json::json!({"role": "user", "content": text})],
        tools: Vec::new(),
        enable_thinking: think,
        preserve_thinking: false,
        add_generation_prompt: true,
        special_text: SpecialText::Reject,
    };
    let prompt = template.render(&request)?;
    let ids = tokenizer.encode(&prompt, true);
    let pool = workers.map(Workers::current).transpose()?;
    let compute = Compute {
        workers: pool.as_ref(),
        kernels: matrix_kernels(backend.unwrap_or(MatrixBackend::Power9Fma))?,
    };
    let start = Instant::now();
    model.prepack(&file, compute)?;
    eprintln!("Weights packed in {:.1} s", start.elapsed().as_secs_f64());
    // EOS, and `</s>` / `<|endoftext|>` where they are special tokens
    // (Nemotron's generation_config.json stops on `</s>` too).
    let stops: Vec<u32> = std::iter::once(vocabulary.eos)
        .chain(
            ["</s>", "<|endoftext|>"]
                .iter()
                .filter_map(|t| tokenizer.token_id(t))
                .filter(|&id| tokenizer.is_special(id)),
        )
        .collect();
    let mut state = model.state();
    let start = Instant::now();
    let hidden = model.forward(&file, &mut state, &ids, compute, &mut |_, _| {})?;
    let mut logits = model.logits(&file, hidden.last().expect("prompt"), compute)?;
    let prefill = start.elapsed().as_secs_f64();
    let stats = std::env::var_os("POWER_LLM_STATS").is_some();
    // The prefill's stage totals (printed with POWER_LLM_STATS), then the
    // decode's alone.
    let prompt_stages = stats::take();
    let prompt_pool = stats::take_stage_pool();
    if stats {
        eprintln!("Prompt stages, s (pool submissions, wall s, summed task s):");
        for (name, seconds) in prompt_stages {
            if seconds > 0.0 {
                let pool = prompt_pool.iter().find(|p| p.0 == name);
                match pool {
                    Some((_, n, wall, busy)) => {
                        eprintln!("  {seconds:7.3}  {name}  ({n}, {wall:.3}, {busy:.3})")
                    }
                    None => eprintln!("  {seconds:7.3}  {name}"),
                }
            }
        }
    }
    eprintln!(
        "Prompt: {} tokens in {prefill:.2} s ({:.1} tok/s)",
        ids.len(),
        ids.len() as f64 / prefill
    );
    print!(
        "{}",
        prompt.rsplit("<|im_start|>").next().unwrap_or_default()
    );
    let mut stream = Utf8Stream::new();
    let mut out = std::io::stdout();
    let start = Instant::now();
    let mut generated = 0;
    while generated < max_new {
        let next = argmax(&logits) as u32;
        generated += 1;
        if stops.contains(&next) {
            break;
        }
        write!(
            out,
            "{}",
            stream.push(&tokenizer.decode_bytes(&[next], false)?)
        )?;
        out.flush()?;
        // One-token steps run one worker per core (as Gemma's decode; see
        // Workers::one_per_core). POWER_LLM_DECODE_SMT=1 keeps every worker.
        let _cap = pool
            .as_ref()
            .filter(|_| std::env::var_os("POWER_LLM_DECODE_SMT").is_none())
            .map(Workers::one_per_core);
        let hidden = model.forward(&file, &mut state, &[next], compute, &mut |_, _| {})?;
        logits = model.logits(&file, &hidden[0], compute)?;
    }
    writeln!(out, "{}", stream.finish())?;
    let decode = start.elapsed().as_secs_f64();
    eprintln!(
        "Generated {generated} tokens in {decode:.2} s ({:.2} tok/s)",
        generated as f64 / decode
    );
    if stats {
        // Every generated token but the last ran one forward pass.
        let passes = generated.saturating_sub(1).max(1) as f64;
        eprintln!("Decode stages, ms per token:");
        let mut total = 0.0;
        for (name, seconds) in stats::take() {
            if seconds > 0.0 {
                if !name.starts_with('(') {
                    total += seconds;
                }
                eprintln!("  {:8.2}  {name}", seconds * 1e3 / passes);
            }
        }
        eprintln!(
            "  {:8.2}  (not in a stage: norms, embedding, sampling)",
            (decode - total) * 1e3 / passes
        );
        eprintln!("Worker submissions per token, pool wall ms, summed task ms:");
        for (name, submissions, wall, busy) in stats::take_stage_pool() {
            if submissions > 0 {
                eprintln!(
                    "  {:6.1} {:8.2} {:8.2}  {name}",
                    submissions as f64 / passes,
                    wall * 1e3 / passes,
                    busy * 1e3 / passes
                );
            }
        }
    }
    Ok(())
}

/// Index of the greatest logit in `f32::total_cmp` order, the lowest index on
/// ties (two vectorizable passes over the order-preserving integer keys).
fn argmax(logits: &[f32]) -> usize {
    let key = |v: f32| {
        let bits = v.to_bits() as i32;
        bits ^ (((bits >> 31) as u32) >> 1) as i32
    };
    let best = logits.iter().fold(i32::MIN, |m, &v| m.max(key(v)));
    logits
        .iter()
        .position(|&v| key(v) == best)
        .expect("nonempty vocabulary")
}

#[cfg(test)]
mod tests {
    #[test]
    fn argmax_matches_max_by() {
        let cases: [&[f32]; 5] = [
            &[1.0, 3.0, 3.0, -2.0],
            &[-0.0, 0.0, -0.0],
            &[-5.0, -1.0, -1.0, -7.0],
            &[f32::NEG_INFINITY, -3.0e38, 2.5],
            &[0.5],
        ];
        for logits in cases {
            let reference = logits
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.total_cmp(b.1).then(b.0.cmp(&a.0)))
                .unwrap()
                .0;
            assert_eq!(super::argmax(logits), reference, "{logits:?}");
        }
    }
}
