//! Engine observations for the independent 256-position / canvas-extension
//! oracle (`../fixtures/numerical-full-canvas`). Reads ONLY the
//! oracle's documented inputs (token IDs, selections) from its manifest plus the
//! frozen processed self-conditioning input; never its expected tensors.
use power_cpu_llm::engine::{
    execution::MatrixBackend,
    model::ModelDefinition,
    text::{EncoderCache, ReferenceLimits, ReferenceText, TracePoint},
    weights::ModelFile,
};
use serde_json::{Value, json};
use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::Path,
};

const TOP_LOGITS: usize = 16;
const TOP_ROUTER: usize = 10;

struct Inputs {
    case1: Vec<u32>,
    case2: Vec<u32>,
    extension: Vec<u32>,
    case3: Vec<u32>,
    selected: Vec<usize>,
    cache_layers: Vec<usize>,
    cache_positions: Vec<usize>,
    full_rows: Vec<usize>,
}

pub(crate) fn ids(v: &Value, what: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    v.as_array()
        .ok_or(format!("manifest {what} must be an array"))?
        .iter()
        .map(|x| {
            x.as_u64()
                .and_then(|n| u32::try_from(n).ok())
                .ok_or_else(|| format!("bad id in {what}").into())
        })
        .collect()
}
pub(crate) fn indices(v: &Value, what: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    Ok(ids(v, what)?.into_iter().map(|x| x as usize).collect())
}

fn inputs(fixture: &Path) -> Result<Inputs, Box<dyn Error>> {
    let m: Value = serde_json::from_slice(&fs::read(fixture.join("manifest.json"))?)?;
    if m["complete"] != true || m["schema"] != "power-llm full-canvas golden v1" {
        return Err("fixture manifest incomplete or unknown schema".into());
    }
    let c = &m["cases"];
    Ok(Inputs {
        case1: ids(&c["case1"]["decoder_ids"], "case1.decoder_ids")?,
        case2: ids(&c["case2"]["decoder_ids"], "case2.decoder_ids")?,
        extension: ids(
            &c["case3"]["encoder_extension_ids"],
            "case3.encoder_extension_ids",
        )?,
        case3: ids(&c["case3"]["decoder_ids"], "case3.decoder_ids")?,
        selected: indices(&m["selection"]["positions"], "selection.positions")?,
        cache_layers: indices(&m["selection"]["cache_layers"], "selection.cache_layers")?,
        cache_positions: indices(
            &m["selection"]["cache_positions"],
            "selection.cache_positions",
        )?,
        full_rows: indices(
            &m["selection"]["full_logit_rows"]["case1"],
            "full_logit_rows.case1",
        )?,
    })
}

/// Create-new blobs, manifest published last (same discipline as trace.rs).
pub(crate) struct Writer<'a> {
    pub(super) root: &'a Path,
    pub(super) entries: Vec<Value>,
}
impl Writer<'_> {
    fn write(
        &mut self,
        name: &str,
        shape: &[usize],
        dtype: &str,
        bytes: &[u8],
        stats: Option<(f64, f64)>,
    ) -> io::Result<()> {
        let filename = format!("{name}.bin");
        let mut file = BufWriter::new(File::create_new(self.root.join(&filename))?);
        file.write_all(bytes)?;
        file.flush()?;
        let mut entry =
            json!({"name":name, "file":filename, "offset":0, "shape":shape, "dtype":dtype});
        if let Some((rms, max_abs)) = stats {
            entry["rms"] = json!(rms);
            entry["max_abs"] = json!(max_abs);
        }
        self.entries.push(entry);
        Ok(())
    }
    pub(super) fn f32(&mut self, name: &str, shape: &[usize], values: &[f32]) -> io::Result<()> {
        if shape.iter().product::<usize>() != values.len() || values.iter().any(|x| !x.is_finite())
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{name}: shape/finite"),
            ));
        }
        let rms = (values.iter().map(|&x| f64::from(x).powi(2)).sum::<f64>() / values.len() as f64)
            .sqrt();
        let max_abs = values
            .iter()
            .fold(0.0f64, |m, &x| m.max(f64::from(x).abs()));
        let bytes: Vec<u8> = values.iter().flat_map(|x| x.to_le_bytes()).collect();
        self.write(name, shape, "<f4", &bytes, Some((rms, max_abs)))
    }
    pub(super) fn i32(&mut self, name: &str, shape: &[usize], values: &[i32]) -> io::Result<()> {
        if shape.iter().product::<usize>() != values.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{name}: shape"),
            ));
        }
        let bytes: Vec<u8> = values.iter().flat_map(|x| x.to_le_bytes()).collect();
        self.write(name, shape, "<i4", &bytes, None)
    }
}

/// Everything one case records from the observer.
#[derive(Default)]
pub(crate) struct Capture {
    embedding: Option<Vec<Vec<f32>>>,
    soft: Option<Vec<Vec<f32>>>,
    input: Option<Vec<Vec<f32>>>,
    pub(super) layers: Vec<Vec<Vec<f32>>>,
    pub(super) routing: Vec<Vec<Vec<f32>>>,
    pub(super) final_rows: Option<Vec<Vec<f32>>>,
}
impl Capture {
    pub(super) fn observe(
        &mut self,
        point: TracePoint,
        rows: &[Vec<f32>],
        selected: &[usize],
    ) -> io::Result<()> {
        let pick = || -> io::Result<Vec<Vec<f32>>> {
            selected
                .iter()
                .map(|&p| {
                    rows.get(p)
                        .cloned()
                        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "selection"))
                })
                .collect()
        };
        match point {
            TracePoint::DecoderEmbedding => self.embedding = Some(pick()?),
            TracePoint::DecoderSoftEmbedding => self.soft = Some(pick()?),
            TracePoint::DecoderInput => self.input = Some(pick()?),
            TracePoint::DecoderLayer(_) | TracePoint::EncoderLayer(_) => self.layers.push(pick()?),
            TracePoint::DecoderRouting(_) | TracePoint::EncoderRouting(_) => {
                self.routing.push(rows.to_vec())
            }
            TracePoint::DecoderFinal | TracePoint::EncoderFinal => self.final_rows = Some(pick()?),
            TracePoint::EncoderEmbedding => {}
        }
        Ok(())
    }
}

fn flat(rows: &[Vec<f32>]) -> Vec<f32> {
    rows.concat()
}

/// Routing for ALL positions/layers: top-8 IDs/weights in router order, and the
/// engine's own top-10 probabilities (descending, ties toward the lower ID).
fn write_routing(
    w: &mut Writer<'_>,
    prefix: &str,
    routing: &[Vec<Vec<f32>>],
    experts: usize,
) -> io::Result<()> {
    let (layers, positions) = (routing.len(), routing.first().map_or(0, Vec::len));
    let (mut ids, mut weights, mut top_ids, mut top_probs) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for layer in routing {
        for row in layer {
            let k = (row.len() - experts) / 2;
            ids.extend(row[..k].iter().map(|&x| x as i32));
            weights.extend_from_slice(&row[k..2 * k]);
            let probs = &row[2 * k..];
            let mut order: Vec<usize> = (0..probs.len()).collect();
            order.sort_by(|&a, &b| probs[b].total_cmp(&probs[a]).then(a.cmp(&b)));
            for &e in &order[..TOP_ROUTER] {
                top_ids.push(e as i32);
                top_probs.push(probs[e]);
            }
        }
    }
    let k = ids.len() / (layers * positions).max(1);
    w.i32(
        &format!("{prefix}.routing.top8_ids"),
        &[layers, positions, k],
        &ids,
    )?;
    w.f32(
        &format!("{prefix}.routing.top8_weights"),
        &[layers, positions, k],
        &weights,
    )?;
    w.i32(
        &format!("{prefix}.routing.top10_ids"),
        &[layers, positions, TOP_ROUTER],
        &top_ids,
    )?;
    w.f32(
        &format!("{prefix}.routing.top10_probs"),
        &[layers, positions, TOP_ROUTER],
        &top_probs,
    )
}

pub(crate) fn write_capture(
    w: &mut Writer<'_>,
    prefix: &str,
    c: &Capture,
    hidden: usize,
    experts: usize,
) -> io::Result<()> {
    let n = |rows: &Option<Vec<Vec<f32>>>| rows.as_ref().map_or(0, Vec::len);
    if let Some(rows) = &c.embedding {
        w.f32(
            &format!("{prefix}.embedding.selected"),
            &[rows.len(), hidden],
            &flat(rows),
        )?;
    }
    if let Some(rows) = &c.soft {
        w.f32(
            &format!("{prefix}.soft_embedding.selected"),
            &[rows.len(), hidden],
            &flat(rows),
        )?;
    }
    if let Some(rows) = &c.input {
        w.f32(
            &format!("{prefix}.self_conditioned.selected"),
            &[rows.len(), hidden],
            &flat(rows),
        )?;
    }
    let layers: Vec<f32> = c.layers.iter().flat_map(|l| flat(l)).collect();
    w.f32(
        &format!("{prefix}.layers.selected"),
        &[c.layers.len(), n(&c.final_rows), hidden],
        &layers,
    )?;
    if let Some(rows) = &c.final_rows {
        w.f32(
            &format!("{prefix}.final.selected"),
            &[rows.len(), hidden],
            &flat(rows),
        )?;
    }
    // Dense models (no routed experts) have no routing to record.
    if experts == 0 {
        return Ok(());
    }
    write_routing(w, prefix, &c.routing, experts)
}

/// Exhaustive per-position top-16 / logsumexp over the full vocabulary.
pub(crate) fn write_logits(
    w: &mut Writer<'_>,
    prefix: &str,
    logits: &[f32],
    vocab: usize,
    full_rows: &[usize],
) -> io::Result<Vec<u32>> {
    let positions = logits.len() / vocab;
    let (mut top_ids, mut top_scores, mut lse, mut argmax) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for row in logits.chunks_exact(vocab) {
        let mut order: Vec<usize> = (0..vocab).collect();
        order.select_nth_unstable_by(TOP_LOGITS, |&a, &b| {
            row[b].total_cmp(&row[a]).then(a.cmp(&b))
        });
        let mut top = order[..TOP_LOGITS].to_vec();
        top.sort_by(|&a, &b| row[b].total_cmp(&row[a]).then(a.cmp(&b)));
        argmax.push(top[0] as u32);
        top_ids.extend(top.iter().map(|&i| i as i32));
        top_scores.extend(top.iter().map(|&i| row[i]));
        let max = f64::from(row[top[0]]);
        lse.push(
            (max + power_cpu_llm::libm::ln(
                row.iter()
                    .map(|&x| power_cpu_llm::libm::exp(f64::from(x) - max))
                    .sum::<f64>(),
            )) as f32,
        );
    }
    w.i32(
        &format!("{prefix}.logits.top_ids"),
        &[positions, TOP_LOGITS],
        &top_ids,
    )?;
    w.f32(
        &format!("{prefix}.logits.top_scores"),
        &[positions, TOP_LOGITS],
        &top_scores,
    )?;
    w.f32(&format!("{prefix}.logits.logsumexp"), &[positions], &lse)?;
    for &p in full_rows {
        w.f32(
            &format!("{prefix}.logits.full_row.{p}"),
            &[vocab],
            &logits[p * vocab..(p + 1) * vocab],
        )?;
    }
    Ok(argmax)
}

fn read_processed(path: &Path, cells: usize) -> Result<Vec<f32>, Box<dyn Error>> {
    let bytes = fs::read(path)?;
    if bytes.len() != cells * 4 {
        return Err(
            format!("processed self-conditioning input must be {cells} LE F32 values").into(),
        );
    }
    let values: Vec<f32> = bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|&b| f32::from_le_bytes(b))
        .collect();
    if values.iter().any(|x| x.is_nan() || *x == f32::INFINITY) {
        return Err("processed self-conditioning input contains NaN/+inf".into());
    }
    Ok(values)
}

/// (keys, values, retained tokens) of one encoder cache layer.
pub(crate) type CacheSlices<'c> = (&'c [f32], &'c [f32], usize);
pub(crate) fn cache_rows<'c>(
    cache: &'c EncoderCache<'_>,
    layer: usize,
) -> Result<CacheSlices<'c>, Box<dyn Error>> {
    let l = cache.layer(layer).ok_or("cache layer")?;
    Ok((l.keys(), l.values(), l.retained_tokens()))
}

pub(crate) fn trace_full_canvas(
    model_path: &Path,
    fixture: &Path,
    processed_path: &Path,
    out: &Path,
    workers: usize,
    backend: MatrixBackend,
) -> Result<(), Box<dyn Error>> {
    let input = inputs(fixture)?;
    let file = ModelFile::open(model_path)?;
    let model = ModelDefinition::bind(file.inventory())?;
    let (vocab, hidden, canvas) = (model.spec.vocabulary, model.spec.hidden, model.spec.canvas);
    for case in [&input.case1, &input.case2, &input.extension, &input.case3] {
        if case.len() != canvas || case.iter().any(|&t| t as usize >= vocab) {
            return Err("fixture canvases must be full-width valid IDs".into());
        }
    }
    let processed = read_processed(processed_path, canvas * vocab)?;
    fs::create_dir(out)?; // never overwrite
    let runtime = ReferenceText::new_with_backend(
        &file,
        &model,
        ReferenceLimits::default(),
        workers,
        backend,
    )?;
    eprintln!(
        "Full-canvas trace: matrix backend {}; worker CPUs {:?}; inherited memory-node policy.",
        runtime.matrix_backend(),
        runtime.worker_cpus()
    );
    let mut w = Writer {
        root: out,
        entries: Vec::new(),
    };
    let experts = model.spec.expert_count;
    let selected = &input.selected;
    let start = std::time::Instant::now();
    let mut seconds = serde_json::Map::new();
    let lap = |name: &str, seconds: &mut serde_json::Map<String, Value>| {
        seconds.insert(name.into(), json!(start.elapsed().as_secs_f64()));
        eprintln!("[{:8.1}s] {name}", start.elapsed().as_secs_f64());
    };

    let mut cache = runtime.new_cache()?;
    runtime.encode(&[2], &mut cache)?;
    for layer in 0..model.layers.len() {
        let (k, v, n) = cache_rows(&cache, layer)?;
        let g = &model.spec.attention[layer];
        w.f32(
            &format!("prompt_cache.{layer}.k"),
            &[n, g.kv_heads, g.head_dim],
            k,
        )?;
        w.f32(
            &format!("prompt_cache.{layer}.v"),
            &[n, g.kv_heads, g.head_dim],
            v,
        )?;
    }
    lap("prompt encoder", &mut seconds);

    let mut argmax = Vec::new();
    for (prefix, ids, sc) in [
        ("case1", &input.case1, None),
        ("case2", &input.case2, Some(processed.as_slice())),
    ] {
        let mut capture = Capture::default();
        let final_rows = runtime
            .decode_hidden_traced(ids, &cache, sc, &mut |p, rows| {
                capture.observe(p, rows, selected)
            })
            .map_err(|e| e.to_string())?;
        write_capture(&mut w, prefix, &capture, hidden, experts)?;
        let logits = runtime.logits(&final_rows)?;
        let full = if prefix == "case1" {
            input.full_rows.clone()
        } else {
            Vec::new()
        };
        let top = write_logits(&mut w, prefix, &logits, vocab, &full)?;
        if prefix == "case1" {
            argmax = top;
        }
        lap(prefix, &mut seconds);
    }
    if cache.total_tokens() != 1 {
        return Err("decoder modified the prompt cache".into());
    }

    let mut capture = Capture::default();
    runtime
        .encode_traced(&input.extension, &mut cache, &mut |p, rows| {
            capture.observe(p, rows, selected)
        })
        .map_err(|e| e.to_string())?;
    write_capture(&mut w, "case3_encode", &capture, hidden, experts)?;
    for &layer in &input.cache_layers {
        let (k, v, n) = cache_rows(&cache, layer)?;
        let g = &model.spec.attention[layer];
        let width = g.kv_heads * g.head_dim;
        let pick = |data: &[f32]| -> Result<Vec<f32>, Box<dyn Error>> {
            input
                .cache_positions
                .iter()
                .map(|&p| {
                    (p < n)
                        .then(|| data[p * width..(p + 1) * width].to_vec())
                        .ok_or("cache position".into())
                })
                .collect::<Result<Vec<_>, _>>()
                .map(|r| r.concat())
        };
        let shape = [input.cache_positions.len(), g.kv_heads, g.head_dim];
        w.f32(
            &format!("extended_cache.{layer}.k.selected"),
            &shape,
            &pick(k)?,
        )?;
        w.f32(
            &format!("extended_cache.{layer}.v.selected"),
            &shape,
            &pick(v)?,
        )?;
    }
    let retained: Vec<usize> = (0..model.layers.len())
        .map(|l| cache_rows(&cache, l).map(|r| r.2))
        .collect::<Result<_, _>>()?;
    lap("case3 encoder extension", &mut seconds);

    let mut capture = Capture::default();
    let final_rows = runtime
        .decode_hidden_traced(&input.case3, &cache, None, &mut |p, rows| {
            capture.observe(p, rows, selected)
        })
        .map_err(|e| e.to_string())?;
    write_capture(&mut w, "case3_decode", &capture, hidden, experts)?;
    write_logits(
        &mut w,
        "case3_decode",
        &runtime.logits(&final_rows)?,
        vocab,
        &[],
    )?;
    if cache.total_tokens() != 1 + canvas {
        return Err("decoder modified the extended cache".into());
    }
    lap("case3 decoder", &mut seconds);

    let manifest = json!({
        "schema":"power-llm full-canvas engine observation v1",
        "kind":"engine-observation-NOT-independent-golden", "complete":true,
        "model":{"path":model_path.to_string_lossy(), "file_size":file.inventory().file_size},
        "fixture_inputs_from":fixture.to_string_lossy(),
        "processed_self_conditioning_input":processed_path.to_string_lossy(),
        "matrix_backend":runtime.matrix_backend(),
        "projection_worker_cpus":runtime.worker_cpus(),
        "case1_engine_argmax":argmax,
        "extended_cache":{"total_tokens":cache.total_tokens(), "retained_per_layer":retained},
        "seconds":seconds,
        "entries":w.entries,
    });
    let mut manifest_file = File::create_new(out.join("manifest.json.partial"))?;
    manifest_file.write_all(&serde_json::to_vec_pretty(&manifest)?)?;
    manifest_file.sync_all()?;
    fs::rename(out.join("manifest.json.partial"), out.join("manifest.json"))?;
    eprintln!(
        "Full-canvas engine observations exported to {}.",
        out.display()
    );
    Ok(())
}
