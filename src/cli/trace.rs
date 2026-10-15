//! Diagnostic trace exporter, never an expected-result/golden producer.
use power_cpu_llm::engine::execution::MatrixBackend;
use power_cpu_llm::engine::{
    model::ModelDefinition,
    text::{ReferenceLimits, ReferenceText, TracePoint},
    weights::ModelFile,
};
use serde_json::{Value, json};
use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::Path,
};

struct Writer<'a> {
    root: &'a Path,
    entries: Vec<Value>,
}
impl Writer<'_> {
    fn blob(&mut self, name: &str, shape: &[usize], values: &[f32]) -> io::Result<()> {
        let cells = shape.iter().try_fold(1usize, |n, &d| n.checked_mul(d));
        if cells != Some(values.len()) || values.is_empty() || values.iter().any(|x| !x.is_finite())
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "trace shape/finite values",
            ));
        }
        // Names are internal constants/indices, never user-provided paths.
        let filename = format!("{name}.f32");
        let mut file = BufWriter::new(File::create_new(self.root.join(&filename))?);
        let mut rms = 0.0f64;
        let mut max_abs = 0.0f64;
        for &x in values {
            file.write_all(&x.to_le_bytes())?;
            rms += f64::from(x).powi(2);
            max_abs = max_abs.max(f64::from(x).abs());
        }
        file.flush()?;
        self.entries.push(
            json!({"name":name, "file":filename, "offset":0, "shape":shape,
            "dtype":"<f4", "rms":(rms/values.len() as f64).sqrt(), "max_abs":max_abs}),
        );
        Ok(())
    }
    fn rows(&mut self, prefix: &str, point: TracePoint, rows: &[Vec<f32>]) -> io::Result<()> {
        if point.is_routing() {
            return Ok(()); // short-canvas export keeps its established blob set
        }
        let point = point.name();
        let suffix = point.split_once('.').expect("internal trace point").1;
        let suffix = if suffix == "input" {
            "self_conditioned"
        } else {
            suffix
        };
        let width = rows.first().map_or(0, Vec::len);
        if rows.iter().any(|r| r.len() != width) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "ragged trace"));
        }
        self.blob(
            &format!("{prefix}.{suffix}"),
            &[rows.len(), width],
            &rows.concat(),
        )
    }
}
fn read_logits(path: &Path, vocabulary: usize) -> io::Result<Vec<f32>> {
    let size = vocabulary
        .checked_mul(4)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "logit dimensions"))?;
    if fs::metadata(path)?.len() != size as u64 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected one full vocabulary of little-endian F32 raw logits",
        ));
    }
    let bytes = fs::read(path)?;
    if bytes.len() != size {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "logit file changed length",
        ));
    }
    let logits: Vec<_> = bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|&b| f32::from_le_bytes(b))
        .collect();
    if logits.iter().any(|x| !x.is_finite()) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "nonfinite input logits",
        ));
    }
    Ok(logits)
}
fn best(logits: &[f32]) -> usize {
    logits
        .iter()
        .enumerate()
        .max_by(|(i, a), (j, b)| a.total_cmp(b).then(j.cmp(i)))
        .expect("nonempty vocabulary")
        .0
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Options<'a> {
    pub previous: Option<&'a Path>,
    pub workers: Option<usize>,
    pub backend: Option<MatrixBackend>,
}

/// `portable` | `auto` | `power9-fma` (explicit; no environment variable).
pub(crate) fn matrix_backend(value: &str) -> Result<MatrixBackend, Box<dyn Error>> {
    match value {
        "portable" => Ok(MatrixBackend::Portable),
        "auto" => Ok(MatrixBackend::Auto),
        "power9-fma" => Ok(MatrixBackend::Power9Fma),
        "power9-int8" => Ok(MatrixBackend::Power9Int8),
        "portable-int8" => Ok(MatrixBackend::PortableInt8),
        "power9-int8x2" => Ok(MatrixBackend::Power9Int8x2),
        "portable-int8x2" => Ok(MatrixBackend::PortableInt8x2),
        _ => Err(
            "matrix backend must be portable, auto, power9-fma, power9-int8, power9-int8x2, portable-int8 or portable-int8x2"
                .into(),
        ),
    }
}
pub(crate) fn options(
    args: &[std::ffi::OsString],
    batched: bool,
) -> Result<Options<'_>, Box<dyn Error>> {
    if !args.as_chunks::<2>().1.is_empty() {
        return Err("trace options require name/value pairs".into());
    }
    let mut previous = None;
    let mut workers = None;
    let mut backend = None;
    for pair in args.as_chunks::<2>().0 {
        match pair[0].to_str() {
            Some("--self-condition-from") if previous.is_none() => {
                previous = Some(Path::new(&pair[1]))
            }
            Some("--workers") if batched && workers.is_none() => {
                let count = pair[1]
                    .to_str()
                    .ok_or("worker count must be UTF-8")?
                    .parse::<usize>()?;
                if count == 0 {
                    return Err("workers must be at least one".into());
                }
                workers = Some(count);
            }
            Some("--matrix-backend") if batched && backend.is_none() => {
                backend = Some(matrix_backend(
                    pair[1].to_str().ok_or("matrix backend must be UTF-8")?,
                )?);
            }
            _ => return Err("unknown, duplicate, or incompatible trace option".into()),
        }
    }
    Ok(Options {
        previous,
        workers,
        backend,
    })
}

pub(crate) fn trace_forward(
    path: &Path,
    out: &Path,
    previous_path: Option<&Path>,
    batched: bool,
    workers: Option<usize>,
    backend: Option<MatrixBackend>,
) -> Result<(), Box<dyn Error>> {
    let file = ModelFile::open(path)?;
    let model = ModelDefinition::bind(file.inventory())?;
    if model.spec.vocabulary <= 9259 || model.vocabulary.bos != 2 {
        return Err("trace cases require target vocabulary/BOS".into());
    }
    let external = previous_path
        .map(|p| read_logits(p, model.spec.vocabulary))
        .transpose()?;
    // Fail on an existing directory: never overwrite independent fixtures.
    fs::create_dir(out)?;
    let mut writer = Writer {
        root: out,
        entries: Vec::new(),
    };
    if !batched && (workers.is_some() || backend.is_some()) {
        return Err("workers/matrix backend require batched execution".into());
    }
    let runtime = if let Some(backend) = backend {
        ReferenceText::new_with_backend(
            &file,
            &model,
            ReferenceLimits::default(),
            workers.unwrap_or(1),
            backend,
        )?
    } else if let Some(count) = workers {
        ReferenceText::new_parallel(&file, &model, ReferenceLimits::default(), count)?
    } else if batched {
        ReferenceText::new_batched(&file, &model, ReferenceLimits::default())?
    } else {
        ReferenceText::new(&file, &model, ReferenceLimits::default())?
    };
    if let Some(cpus) = runtime.worker_cpus() {
        eprintln!("Trace: pinned projection worker CPUs {cpus:?}; inherited memory-node policy.");
    }
    if batched {
        eprintln!(
            "Trace: matrix backend {} (requested {backend:?}).",
            runtime.matrix_backend()
        );
    }
    let mut cache = runtime.new_cache()?;
    eprintln!("Trace: causal BOS encoder; slow reference math, not a golden result or benchmark.");
    runtime.encode_traced(&[2], &mut cache, &mut |point, rows| {
        writer.rows("encoder", point, rows)
    })?;
    for (i, g) in model.spec.attention.iter().enumerate() {
        let c = cache.layer(i).expect("bound layer");
        let shape = [c.retained_tokens(), g.kv_heads, g.head_dim];
        writer.blob(&format!("encoder.cache.{i}.k"), &shape, c.keys())?;
        writer.blob(&format!("encoder.cache.{i}.v"), &shape, c.values())?;
    }
    eprintln!("Trace: decoder_a canvas [42], zero self-conditioning.");
    writer.blob(
        "decoder_a.soft_embedding",
        &[1, model.spec.hidden],
        &vec![0.0; model.spec.hidden],
    )?;
    let a = runtime.decode_hidden_traced(&[42], &cache, None, &mut |point, rows| {
        writer.rows("decoder_a", point, rows)
    })?;
    let logits_a = runtime.logits(&a)?;
    writer.blob("decoder_a.logits", &[1, model.spec.vocabulary], &logits_a)?;
    let input = external.as_deref().unwrap_or(&logits_a);
    let canvas_b = [best(input) as u32];
    let processed: Vec<_> = input.iter().map(|&x| (f64::from(x) / 0.8) as f32).collect();
    writer.blob(
        "decoder_b.input_processed_logits",
        &[1, model.spec.vocabulary],
        &processed,
    )?;
    eprintln!("Trace: decoder_b canvas {canvas_b:?}, processed-logit self-conditioning.");
    let b =
        runtime.decode_hidden_traced(&canvas_b, &cache, Some(&processed), &mut |point, rows| {
            writer.rows("decoder_b", point, rows)
        })?;
    writer.blob(
        "decoder_b.logits",
        &[1, model.spec.vocabulary],
        &runtime.logits(&b)?,
    )?;
    eprintln!("Trace: decoder_c canvas [42,9259], bidirectional diagnostic.");
    let c = runtime.decode_hidden_traced(&[42, 9259], &cache, None, &mut |point, rows| {
        writer.rows("decoder_c", point, rows)
    })?;
    writer.blob(
        "decoder_c.logits",
        &[2, model.spec.vocabulary],
        &runtime.logits(&c)?,
    )?;
    if cache.total_tokens() != 1 {
        return Err("decoder modified encoder cache".into());
    }
    let manifest = json!({
        "schema":1, "kind":"engine-observation-NOT-independent-golden", "complete":true,
        "execution":if batched { "portable encoded row-tile projections and grouped experts" } else { "rowwise reference" },
        "projection_worker_cpus":runtime.worker_cpus(),
        "matrix_backend":if batched { Some(runtime.matrix_backend()) } else { None },
        "matrix_backend_requested":backend.map(|b| format!("{b:?}")),
        "model":{"path":path.to_string_lossy(), "file_size":file.inventory().file_size, "tensor_count":file.inventory().tensors.len()},
        "arithmetic":"F32 storage, F64 reductions; scaled token AND soft embeddings; unscaled tied output head; softcap30",
        "encoder":{"ids":[2], "positions":[0]},
        "decoder_a":{"ids":[42], "positions":[1], "self_conditioning":null},
        "decoder_b":{"ids":canvas_b, "positions":[1], "raw_input_path":previous_path.map(|p|p.to_string_lossy()),
            "self_conditioning":"F32(F64(raw-input-logits)/0.8); raw input defaults to ENGINE decoder_a.logits, NOT an independent fixture"},
        "decoder_c":{"ids":[42,9259], "positions":[1,2], "self_conditioning":null},
        "entries":writer.entries,
        "limitations":"short canvases only; no generation-quality, full-canvas, numerical-parity or throughput claim"
    });
    // Publish the completion marker LAST. Failed exports leave no manifest.
    let mut manifest_file = File::create_new(out.join("manifest.json.partial"))?;
    manifest_file.write_all(&serde_json::to_vec_pretty(&manifest)?)?;
    manifest_file.sync_all()?;
    fs::rename(out.join("manifest.json.partial"), out.join("manifest.json"))?;
    println!(
        "Engine trace exported to {}. Compare independently; this is not a golden producer.",
        out.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trace_options_reject_duplicates_and_unsupported_worker_counts() {
        let args = |a: &[&str]| a.iter().map(std::ffi::OsString::from).collect::<Vec<_>>();
        assert_eq!(
            options(&[], false).unwrap(),
            Options {
                previous: None,
                workers: None,
                backend: None
            }
        );
        for (value, backend) in [
            ("portable", MatrixBackend::Portable),
            ("auto", MatrixBackend::Auto),
            ("power9-fma", MatrixBackend::Power9Fma),
        ] {
            assert_eq!(
                options(&args(&["--matrix-backend", value]), true)
                    .unwrap()
                    .backend,
                Some(backend)
            );
        }
        for a in [
            vec!["--matrix-backend", "fma"],
            vec!["--matrix-backend", "auto", "--matrix-backend", "auto"],
        ] {
            assert!(options(&args(&a), true).is_err());
        }
        // Backend selection is a batched-execution option only.
        assert!(options(&args(&["--matrix-backend", "auto"]), false).is_err());
        assert_eq!(
            options(&args(&["--workers", "4"]), true).unwrap().workers,
            Some(4)
        );
        for a in [
            vec!["--workers", "0"],
            vec!["--workers", "-1"],
            vec!["--workers", "4", "--workers", "4"],
            vec!["--bad", "1"],
            vec!["--workers"],
            vec!["--self-condition-from", "a", "--self-condition-from", "b"],
        ] {
            assert!(options(&args(&a), true).is_err());
        }
        assert!(options(&args(&["--workers", "2"]), false).is_err());
    }
    #[test]
    fn blobs_are_little_endian_checked_and_never_overwritten() {
        let root =
            std::env::temp_dir().join(format!("power-llm-trace-writer-{}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let mut w = Writer {
            root: &root,
            entries: Vec::new(),
        };
        w.blob("a", &[1, 2], &[1.0, -2.5]).unwrap();
        let bytes = fs::read(root.join("a.f32")).unwrap();
        assert_eq!(
            bytes,
            [1.0f32.to_le_bytes(), (-2.5f32).to_le_bytes()].concat()
        );
        assert!(w.blob("a", &[1], &[4.0]).is_err());
        assert_eq!(fs::read(root.join("a.f32")).unwrap(), bytes);
        assert!(w.blob("bad", &[1], &[f32::NAN]).is_err());
        assert!(w.blob("shape", &[usize::MAX, 2], &[1.0]).is_err());
        assert_eq!(read_logits(&root.join("a.f32"), 2).unwrap(), [1.0, -2.5]);
        assert!(read_logits(&root.join("a.f32"), 1).is_err());
        assert_eq!(best(&[1.0, 1.0, -1.0]), 0);
        fs::remove_file(root.join("a.f32")).unwrap();
        fs::remove_dir(root).unwrap();
    }
}
