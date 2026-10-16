//! Encoded row-tile projections, portable C F64 reductions, grouped experts.
//! Rust owns reads/tiling/routing; C decodes bounded scratch and computes.
use super::*;
use crate::engine::execution::{Compute, MatrixKernels, Workers};
use crate::kernels::{
    encoded::Encoding,
    quantized::{self, EncodedRows, int8::QuantizedInput},
};
use std::collections::BTreeMap;

fn cells(rows: usize, cols: usize, limit: usize) -> Result<usize, Error> {
    rows.checked_mul(cols)
        .filter(|&n| n > 0 && n <= limit)
        .ok_or(Error::ResourceLimit)
}
fn inputs(input: &[Vec<f32>], cols: usize) -> Result<(), Error> {
    inputs_with(input, cols, None)
}
/// [`inputs`], scanning prompt-sized batches for nonfinite values on the
/// workers (serially the scan cost about 4% of a long prefill).
fn inputs_with<R: AsRef<[f32]> + Sync>(
    input: &[R],
    cols: usize,
    workers: Option<&Workers>,
) -> Result<(), Error> {
    if cols == 0 || input.is_empty() || input.iter().any(|r| r.as_ref().len() != cols) {
        return Err(Error::Shape("batch input dimensions"));
    }
    let nonfinite = |rows: &[R]| !rows.iter().all(|r| math::all_finite(r.as_ref()));
    let found = match workers {
        Some(w) if input.len() >= PARALLEL_CHECK_POSITIONS => w
            .ranges(input.len(), |first, end| {
                Ok::<_, Error>(nonfinite(&input[first..end]))
            })?
            .contains(&true),
        _ => nonfinite(input),
    };
    if found {
        return Err(Error::Parameter("nonfinite batch input"));
    }
    Ok(())
}
const PARALLEL_CHECK_POSITIONS: usize = 32;
fn zeros<T: Default + Clone>(count: usize) -> Result<Vec<T>, Error> {
    let mut out = Vec::new();
    out.try_reserve_exact(count)
        .map_err(|_| Error::ResourceLimit)?;
    out.resize(count, T::default());
    Ok(out)
}
fn rows(count: usize, width: usize, limit: usize) -> Result<Vec<Vec<f32>>, Error> {
    cells(count, width, limit)?;
    let mut result = Vec::new();
    result
        .try_reserve_exact(count)
        .map_err(|_| Error::ResourceLimit)?;
    for _ in 0..count {
        let mut row = Vec::new();
        row.try_reserve_exact(width)
            .map_err(|_| Error::ResourceLimit)?;
        result.push(row);
    }
    Ok(result)
}

/// Run `run` on every item, split into contiguous chunks across workers.
fn for_each_split<T: Send>(
    items: Vec<T>,
    compute: Compute<'_>,
    run: impl Fn(T) -> Result<(), Error> + Sync,
) -> Result<(), Error> {
    match compute.workers.filter(|_| items.len() > 1) {
        Some(workers) => {
            let n = workers.count().min(items.len());
            let total = items.len();
            let mut chunks: Vec<Vec<T>> = (0..n).map(|_| Vec::new()).collect();
            for (i, item) in items.into_iter().enumerate() {
                chunks[i * n / total].push(item);
            }
            workers.tasks(chunks, |chunk| chunk.into_iter().try_for_each(&run))?;
            Ok(())
        }
        None => items.into_iter().try_for_each(run),
    }
}

/// Integer-dot input: the quantized rows plus, for two-plane kernels, the
/// position offset of the residual plane (output = plane 1 + plane 2).
struct Quantized {
    input: QuantizedInput,
    second_plane: Option<usize>,
    /// Activation rows represented (before padding / the second plane).
    positions: usize,
}

/// The quantized input of `tokens` (token, weight) of `all`: plane 1 then,
/// with two planes, plane 2 (the layout [`quantize_input`] gives those rows).
fn gather_quantized(all: &Quantized, tokens: &[(usize, f32)]) -> Result<Quantized, Error> {
    let mut positions: Vec<usize> = tokens.iter().map(|&(t, _)| t).collect();
    if let Some(second) = all.second_plane {
        positions.extend(tokens.iter().map(|&(t, _)| second + t));
    }
    Ok(Quantized {
        input: all.input.gather(&positions)?,
        second_plane: all.second_plane.map(|_| tokens.len()),
        positions: tokens.len(),
    })
}

/// Quantize `rows` (each `cols` long) in groups of 4 positions split across
/// workers; with `residual`, also write each row's residual `x - da * a`
/// (row-major, `rows.len() * cols`). Each group is computed identically
/// however the groups are split.
fn quantize_rows(
    encoding: Encoding,
    rows: &[&[f32]],
    cols: usize,
    compute: Compute<'_>,
    residual: Option<&mut [f32]>,
) -> Result<QuantizedInput, Error> {
    let mut quantized = QuantizedInput::allocate(encoding, rows.len(), cols)?;
    {
        let groups = quantized.groups_mut();
        let mut outs: Vec<Option<&mut [f32]>> = match residual {
            Some(buffer) => buffer.chunks_mut(cols).map(Some).collect(),
            None => Vec::new(),
        };
        outs.resize_with(groups.len() * 4, || None);
        let mut outs = outs.into_iter();
        let items: Vec<_> = groups
            .into_iter()
            .enumerate()
            .map(|(g, group)| {
                let o: [Option<&mut [f32]>; 4] = std::array::from_fn(|_| outs.next().flatten());
                (g, group, o)
            })
            .collect();
        for_each_split(items, compute, |(g, mut group, o)| {
            let these = std::array::from_fn(|l| rows.get(4 * g + l).copied());
            if o.iter().any(Option::is_some) {
                Ok(group.quantize_with_residual(these, o)?)
            } else {
                Ok(group.quantize(these)?)
            }
        })?;
    }
    Ok(quantized)
}

/// Quantize `input` rows for integer-dot kernels. Two-plane kernels get the
/// residual rows as positions `P..2P` right after the first plane (packed:
/// one token needs one 4-position group, not two); output = plane 1 + plane 2.
fn quantize_input<R: AsRef<[f32]> + Sync>(
    encoding: Encoding,
    input: &[R],
    cols: usize,
    compute: Compute<'_>,
) -> Result<Quantized, Error> {
    let positions = input.len();
    let mut rows: Vec<&[f32]> = input.iter().map(AsRef::as_ref).collect();
    if compute.kernels.planes() != 2 {
        return Ok(Quantized {
            input: quantize_rows(encoding, &rows, cols, compute, None)?,
            second_plane: None,
            positions,
        });
    }
    static SHARED_SCALE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    let shared = positions > 2
        && encoding == Encoding::Q4_0
        && *SHARED_SCALE
            .get_or_init(|| std::env::var("POWER_LLM_SHARED_SCALE").is_ok_and(|v| v == "1"));
    if positions > 2 && (positions.is_multiple_of(4) || shared) {
        let padded = positions.div_ceil(4) * 4;
        let zero = if padded > positions {
            vec![0.0; cols]
        } else {
            Vec::new()
        };
        rows.resize(padded, &zero);
        // One pass per block range of a plane pair. The normal writer keeps
        // residuals in scratch and matches the two passes below; the opt-in
        // shared writer instead splits one rounded integer into both planes.
        let mut input = QuantizedInput::allocate(encoding, 2 * padded, cols)?;
        {
            let writer = if shared {
                input.shared_planes_writer(&rows)?
            } else {
                input.planes_writer(&rows)?
            };
            let (groups, blocks) = (writer.groups(), writer.blocks());
            let workers = compute.workers.map_or(1, Workers::count);
            let per_group = (2 * workers).div_ceil(groups).clamp(1, blocks);
            let items: Vec<(usize, usize, usize)> = (0..groups)
                .flat_map(|g| {
                    (0..per_group)
                        .map(move |j| (g, blocks * j / per_group, blocks * (j + 1) / per_group))
                })
                .collect();
            let writer = &writer;
            for_each_split(items, compute, |(g, b0, b1)| {
                let mut scratch = zeros::<f32>(4 * cols)?;
                // SAFETY: items are distinct (group, block range) pairs; the
                // writer outlives this call.
                Ok(unsafe { writer.quantize(g, b0, b1, &mut scratch) }?)
            })?;
        }
        let input = if padded == positions {
            input
        } else {
            let selected: Vec<usize> = (0..positions).chain(padded..padded + positions).collect();
            input.gather(&selected)?
        };
        return Ok(Quantized {
            input,
            second_plane: Some(positions),
            positions,
        });
    }
    // Residuals of the first plane (a first-plane quantization, then discarded
    // so that both planes can share packed groups).
    let mut residual = zeros::<f32>(cells(positions, cols, MAX_REFERENCE_MATRIX_ELEMENTS)?)?;
    if positions <= 2 {
        // Decode: both planes fit one group; one quantizer call does plane 1,
        // its residuals and plane 2 (the same bytes as the two passes below).
        let mut input = QuantizedInput::allocate(encoding, 2 * positions, cols)?;
        input
            .groups_mut()
            .into_iter()
            .next()
            .ok_or(Error::Shape("quantized group"))?
            .quantize_two_planes(&rows, &mut residual)?;
        return Ok(Quantized {
            input,
            second_plane: Some(positions),
            positions,
        });
    }
    quantize_rows(encoding, &rows, cols, compute, Some(&mut residual))?;
    let both: Vec<&[f32]> = rows
        .iter()
        .copied()
        .chain(residual.chunks_exact(cols))
        .collect();
    Ok(Quantized {
        input: quantize_rows(encoding, &both, cols, compute, None)?,
        second_plane: Some(positions),
        positions,
    })
}

/// Rows per packed unit. Backends that pack ([`MatrixKernels::packs`]) unpack
/// each (tensor, unit) once into the integer-dot layout and keep it for the
/// file's lifetime (about 1.25 bytes per weight); results are bit-identical
/// to unpacking per call. `POWER_LLM_NO_PACK=1` disables this.
const PACK_UNIT_ROWS: u64 = 128;
/// Rows per work chunk unit: whole POWER9 int8 tiles. With at most two
/// kernel positions (one token, two planes) the tile is a packed
/// super-group (8 row groups, 32 rows; one memory stream); otherwise 16.
fn align_rows(kernel_positions: usize) -> usize {
    if kernel_positions <= 2 { 32 } else { 16 }
}

/// One lock-free slot per packed unit of a tensor; the slot vector is
/// cached once per tensor, so the hot path takes no lock or key formatting.
type PackedUnits = Vec<std::sync::OnceLock<crate::kernels::quantized::int8::PackedRows>>;

fn packed_units(
    file: &ModelFile,
    tensor: &TensorInfo,
    available: u64,
) -> Result<std::sync::Arc<PackedUnits>, Error> {
    // Hot path (every projection chunk on every worker): a per-thread map
    // keyed by (file id, tensor address, replica), so lookups take no lock,
    // format no string and allocate nothing. Ids are never reused, so an
    // address reused by a later file cannot alias.
    thread_local! {
        static UNITS: std::cell::RefCell<
            std::collections::HashMap<(u64, usize, usize), std::sync::Arc<PackedUnits>>,
        > = std::cell::RefCell::new(std::collections::HashMap::new());
    }
    let key = (
        file.id(),
        std::ptr::from_ref(tensor) as usize,
        crate::engine::execution::replica(),
    );
    if let Some(found) = UNITS.with(|u| u.borrow().get(&key).cloned()) {
        return Ok(found);
    }
    let units = shared_packed_units(file, tensor, available)?;
    UNITS.with(|u| u.borrow_mut().insert(key, units.clone()));
    Ok(units)
}
fn shared_packed_units(
    file: &ModelFile,
    tensor: &TensorInfo,
    available: u64,
) -> Result<std::sync::Arc<PackedUnits>, Error> {
    let key = match crate::engine::execution::replica() {
        0 => format!("int8-pack/{}", tensor.name),
        r => format!("int8-pack{r}/{}", tensor.name),
    };
    file.cached(&key, || {
        let n = available.div_ceil(PACK_UNIT_ROWS);
        let n = usize::try_from(n).map_err(|_| Error::ResourceLimit)?;
        Ok::<_, Error>(
            (0..n)
                .map(|_| std::sync::OnceLock::new())
                .collect::<PackedUnits>(),
        )
    })
}

fn pack_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("POWER_LLM_NO_PACK").is_none())
}

/// Unit `unit` (rows from `unit * PACK_UNIT_ROWS`) of `tensor`, packed on
/// first use.
fn packed_unit<'u>(
    file: &ModelFile,
    tensor: &TensorInfo,
    units: &'u PackedUnits,
    available: u64,
    cols: usize,
    unit: u64,
    kernels: MatrixKernels,
) -> Result<&'u crate::kernels::quantized::int8::PackedRows, Error> {
    let slot = units
        .get(unit as usize)
        .ok_or(Error::Shape("packed unit"))?;
    if let Some(packed) = slot.get() {
        return Ok(packed);
    }
    let _span = crate::engine::stats::Span::new(crate::engine::stats::Stage::ProjectionPack);
    let start = unit * PACK_UNIT_ROWS;
    let rows = PACK_UNIT_ROWS.min(available - start) as usize;
    let tile = file.view_rows(
        &tensor.name,
        start,
        rows,
        MAX_REFERENCE_MATRIX_ELEMENTS * size_of::<f32>(),
    )?;
    let view = EncodedRows::new(tile.encoding, rows, cols, &tile.bytes)?;
    let made = kernels.pack(&view)?;
    // A concurrent first use may have filled the slot meanwhile; both packs
    // are identical, the first stored one is kept.
    let _ = slot.set(made);
    slot.get().ok_or(Error::Shape("packed unit"))
}

/// Pack every unit of `tensors` (those whose encoding the kernels pack) up
/// front, in one parallel submission, so no request pays packing on first
/// use (otherwise experts first used mid-decode are packed then).
pub(crate) fn prepack(
    file: &ModelFile,
    tensors: &[&TensorInfo],
    compute: Compute<'_>,
) -> Result<(), Error> {
    if !pack_enabled() || compute.workers.is_none() {
        return Ok(());
    }
    let mut items = Vec::new();
    for (t, tensor) in tensors.iter().enumerate() {
        if !Encoding::try_from(tensor.type_id).is_ok_and(|e| compute.kernels.packs(e)) {
            continue;
        }
        matches_file(file, tensor)?;
        let available = crate::engine::weights::row_count(tensor)?;
        for unit in 0..available.div_ceil(PACK_UNIT_ROWS) {
            items.push((t, unit));
        }
    }
    let order: Vec<usize> = (0..items.len()).collect();
    compute.map_items(items.len(), &order, |i| {
        let (t, unit) = items[i];
        let tensor = tensors[t];
        let cols = usize::try_from(tensor.shape[0]).map_err(|_| Error::ResourceLimit)?;
        let available = crate::engine::weights::row_count(tensor)?;
        let units = packed_units(file, tensor, available)?;
        packed_unit(file, tensor, &units, available, cols, unit, compute.kernels).map(|_| ())
    })?;
    Ok(())
}

/// `out[p, r - first]` (stride `count`) for rows `[first, first + count)` of
/// `tensor` from its packed units, packing a unit on first use.
#[allow(clippy::too_many_arguments)]
fn project_packed_rows(
    file: &ModelFile,
    tensor: &TensorInfo,
    available: u64,
    cols: usize,
    first: u64,
    count: usize,
    input: &QuantizedInput,
    out: &mut [f32],
    kernels: MatrixKernels,
) -> Result<(), Error> {
    let units = packed_units(file, tensor, available)?;
    let end = first + count as u64;
    let mut row = first;
    while row < end {
        let unit = row / PACK_UNIT_ROWS;
        let start = unit * PACK_UNIT_ROWS;
        let rows = PACK_UNIT_ROWS.min(available - start) as usize;
        let take = (end.min(start + rows as u64) - row) as usize;
        let packed = packed_unit(file, tensor, &units, available, cols, unit, kernels)?;
        kernels.project_packed(
            packed,
            (row - start) as usize,
            take,
            input,
            out,
            count,
            (row - first) as usize,
        )?;
        row += take as u64;
    }
    Ok(())
}

/// Largest batch fused by [`project_many`]; bigger batches already give
/// each projection enough work per submission.
const FUSE_MAX_POSITIONS: usize = 8;
/// One packed unit per item: larger items cut per-item overhead in
/// one-token decode (26B: 0.111 s/token at 32 rows, 0.105 at 128).
/// Fused one-token projection items per worker (see `project_many`).
const FUSE_ITEMS_PER_WORKER: usize = 1;
/// One-token expert items per worker: finer items cut the tail when one
/// core streams slower (8 cores: Qwen3.6 experts 20.9 -> 20.0 ms per token,
/// Gemma 4 26B 16.9 -> 17.0 tok/s; 16 is equal, 32 slower).
const EXPERT_ITEMS_PER_WORKER: usize = 8;

/// Whole matrices projected from the same `input` in ONE worker submission
/// (one-token decode issues several small projections per stage, each paying
/// a submission and its tail). Packed integer-dot backends and small batches
/// only; otherwise each matrix goes through [`project_streamed_batch_with`].
/// Every value is computed exactly as there. Returns `[matrix][position][row]`.
pub(crate) fn project_many(
    file: &ModelFile,
    tensors: &[&TensorInfo],
    input: &[Vec<f32>],
    compute: Compute<'_>,
) -> Result<Vec<Vec<Vec<f32>>>, Error> {
    let fused = compute.workers.is_some()
        && pack_enabled()
        && input.len() <= FUSE_MAX_POSITIONS
        && tensors.iter().all(|t| {
            t.shape[2..] == [1, 1]
                && Encoding::try_from(t.type_id).is_ok_and(|e| compute.kernels.packs(e))
        });
    if !fused {
        // Prompt-sized batches: one projection per matrix, but the shared
        // input is quantized once per (encoding, cols), not once per matrix.
        let mut shared: Vec<(Encoding, usize, Quantized)> = Vec::new();
        return tensors
            .iter()
            .map(|t| {
                let encoding = Encoding::try_from(t.type_id).ok().filter(|&e| {
                    compute.workers.is_some()
                        && tensors.len() > 1
                        && t.shape[2..] == [1, 1]
                        && compute.kernels.quantizes(e)
                });
                let Some(encoding) = encoding else {
                    return project_streamed_batch_with(
                        file,
                        t,
                        input,
                        MAX_REFERENCE_MATRIX_ELEMENTS,
                        compute,
                    );
                };
                let cols = usize::try_from(t.shape[0]).map_err(|_| Error::ResourceLimit)?;
                let rows = usize::try_from(t.shape[1]).map_err(|_| Error::ResourceLimit)?;
                let index = match shared
                    .iter()
                    .position(|(e, c, _)| *e == encoding && *c == cols)
                {
                    Some(index) => index,
                    None => {
                        inputs_with(input, cols, compute.workers)?;
                        let _span = crate::engine::stats::Span::new(
                            crate::engine::stats::Stage::ProjectionQuantize,
                        );
                        shared.push((
                            encoding,
                            cols,
                            quantize_input(encoding, input, cols, compute)?,
                        ));
                        shared.len() - 1
                    }
                };
                project_range_with(
                    file,
                    t,
                    0,
                    rows,
                    input,
                    Some(&shared[index].2),
                    MAX_REFERENCE_MATRIX_ELEMENTS,
                    compute,
                )
            })
            .collect();
    }
    let positions = input.len();
    // Per matrix: (rows, cols, index into `keys`); one quantization per
    // distinct (encoding, cols).
    let mut keys: Vec<(Encoding, usize)> = Vec::new();
    let mut shapes = Vec::with_capacity(tensors.len());
    for t in tensors {
        matches_file(file, t)?;
        let cols = usize::try_from(t.shape[0]).map_err(|_| Error::ResourceLimit)?;
        let rows = usize::try_from(t.shape[1]).map_err(|_| Error::ResourceLimit)?;
        inputs(input, cols)?;
        let encoding = Encoding::try_from(t.type_id).map_err(|_| Error::Shape("encoding"))?;
        let q = match keys.iter().position(|&(e, c)| e == encoding && c == cols) {
            Some(i) => i,
            None => {
                keys.push((encoding, cols));
                keys.len() - 1
            }
        };
        shapes.push((rows, cols, q));
    }
    let kernel_positions = positions * compute.kernels.planes();
    // About `per_worker` items per worker over all matrices together, each
    // whole tiles (align_rows) and as equal as the tiles allow, so the
    // last round leaves no worker idle for long; larger items are pulled first.
    let per_worker: usize = std::env::var("PQ_FUSE_PER_WORKER")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(FUSE_ITEMS_PER_WORKER);
    let align = align_rows(kernel_positions);
    // Exactly `target` items (whole tiles, as equal as the tiles allow) over
    // all matrices: each matrix gets a share of the items proportional to its
    // tiles (largest remainders), so every worker pulls the same number.
    let tiles: Vec<usize> = shapes.iter().map(|s| s.0.div_ceil(align)).collect();
    let total_tiles: usize = tiles.iter().sum();
    let target = (compute.workers.map_or(1, Workers::count) * per_worker)
        .clamp(tiles.len(), total_tiles.max(tiles.len()));
    let mut counts: Vec<usize> = tiles
        .iter()
        .map(|&n| (n * target / total_tiles.max(1)).clamp(1, n))
        .collect();
    while counts.iter().sum::<usize>() < target {
        // the matrix whose items are currently largest
        let Some(m) = (0..tiles.len())
            .filter(|&m| counts[m] < tiles[m])
            .max_by_key(|&m| tiles[m] * 1024 / counts[m])
        else {
            break;
        };
        counts[m] += 1;
    }
    let mut items = Vec::new();
    for (m, &(rows, _, _)) in shapes.iter().enumerate() {
        let n = counts[m];
        let (base, extra) = (tiles[m] / n, tiles[m] % n);
        let mut first = 0;
        for i in 0..n {
            let len = ((base + usize::from(i < extra)) * align).min(rows - first);
            items.push((m, first, len));
            first += len;
        }
    }
    let mut order: Vec<usize> = (0..items.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(items[i].2));
    let project = |i: usize, input: &QuantizedInput| {
        let (m, first, n) = items[i];
        let (rows, cols, _) = shapes[m];
        let mut out = zeros::<f32>(cells(input.positions(), n, usize::MAX)?)?;
        project_packed_rows(
            file,
            tensors[m],
            rows as u64,
            cols,
            first as u64,
            n,
            input,
            &mut out,
            compute.kernels,
        )?;
        Ok::<_, Error>(out)
    };
    // second[k]: the plane-2 offset of quantization k (None: one plane).
    let (second, parts): (Vec<Option<usize>>, Vec<Vec<f32>>) =
        match (compute.workers, keys.as_slice()) {
            (Some(workers), &[(encoding, cols)])
                if compute.kernels.planes() == 2
                    && positions <= 2
                    && std::env::var_os("PQ_SERIAL_QUANT").is_none() =>
            {
                let (offset, parts) =
                    quantize_then_items(workers, encoding, input, cols, &order, project)?;
                (vec![Some(offset)], parts)
            }
            _ => {
                let quantized = keys
                    .iter()
                    .map(|&(encoding, cols)| {
                        quantize_input(encoding, input, cols, compute.serial())
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let parts = compute.map_items(items.len(), &order, |i| {
                    project(i, &quantized[shapes[items[i].0].2].input)
                })?;
                (quantized.iter().map(|q| q.second_plane).collect(), parts)
            }
        };
    let mut result: Vec<Vec<Vec<f32>>> = shapes
        .iter()
        .map(|&(rows, _, _)| (0..positions).map(|_| Vec::with_capacity(rows)).collect())
        .collect();
    for (&(m, _, n), out) in items.iter().zip(&parts) {
        let second = second[shapes[m].2];
        for (p, row) in result[m].iter_mut().enumerate() {
            let values = &out[p * n..(p + 1) * n];
            match second {
                // The same binary32 sum as project_range_with.
                Some(offset) => row.extend(
                    values
                        .iter()
                        .zip(&out[(offset + p) * n..(offset + p + 1) * n])
                        .map(|(a, b)| a + b),
                ),
                None => row.extend_from_slice(values),
            }
        }
    }
    Ok(result)
}

/// One submission in two phases: `prepare(u0, u1)` over `units` split into
/// one contiguous range per worker, then (after every worker has finished
/// its range) `item(i)` for `order`'s items, pulled dynamically. Every task
/// reaches the barrier even if its range fails or panics, so none waits
/// forever. Results are indexed by item.
fn prepare_then_items<T: Send>(
    workers: &Workers,
    units: usize,
    prepare: impl Fn(usize, usize) -> Result<(), Error> + Sync,
    order: &[usize],
    item: impl Fn(usize) -> Result<T, Error> + Sync,
) -> Result<Vec<T>, Error> {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    let n = workers.count();
    let arrived = AtomicUsize::new(0);
    let failed = AtomicBool::new(false);
    let next = AtomicUsize::new(0);
    let parts = workers.tasks((0..n).collect(), |w| {
        let (u0, u1) = (w * units / n, (w + 1) * units / n);
        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| prepare(u0, u1)));
        if !matches!(ok, Ok(Ok(()))) {
            failed.store(true, Ordering::SeqCst);
        }
        arrived.fetch_add(1, Ordering::AcqRel);
        while arrived.load(Ordering::Acquire) < n {
            std::hint::spin_loop();
        }
        if failed.load(Ordering::SeqCst) {
            return Err(Error::Parameter("prepare phase failed"));
        }
        let mut done = Vec::new();
        while let Some(&i) = order.get(next.fetch_add(1, Ordering::Relaxed)) {
            done.push((i, item(i)?));
        }
        Ok::<_, Error>(done)
    })?;
    let mut slots: Vec<Option<T>> = (0..order.len()).map(|_| None).collect();
    for (i, value) in parts.into_iter().flatten() {
        slots[i] = Some(value);
    }
    slots
        .into_iter()
        .map(|s| s.ok_or(Error::Shape("item not run")))
        .collect()
}

/// One or two positions with two planes: the workers quantize disjoint block
/// ranges of the shared input, then project `order`'s items with
/// `item(i, &input)`, in one submission ([`prepare_then_items`]). The same
/// bytes as [`quantize_input`]; returns the plane-2 offset and the results.
fn quantize_then_items<T: Send>(
    workers: &Workers,
    encoding: Encoding,
    input: &[Vec<f32>],
    cols: usize,
    order: &[usize],
    item: impl Fn(usize, &QuantizedInput) -> Result<T, Error> + Sync,
) -> Result<(usize, Vec<T>), Error> {
    let positions = input.len();
    let mut quantized = QuantizedInput::allocate(encoding, 2 * positions, cols)?;
    let mut residual = zeros::<f32>(cells(positions, cols, MAX_REFERENCE_MATRIX_ELEMENTS)?)?;
    let rows: Vec<&[f32]> = input.iter().map(Vec::as_slice).collect();
    let writer = quantized.two_plane_writer(&rows, &mut residual)?;
    let results = prepare_then_items(
        workers,
        writer.blocks(),
        // SAFETY: the workers' ranges are disjoint and cover every block.
        |b0, b1| Ok(unsafe { writer.quantize_blocks(b0, b1) }?),
        order,
        // SAFETY: items run after every block was written (the barrier).
        |i| item(i, unsafe { writer.input() }),
    )?;
    Ok((positions, results))
}

fn project_range(
    file: &ModelFile,
    tensor: &TensorInfo,
    first_row: u64,
    count: usize,
    input: &[Vec<f32>],
    max_output_elements: usize,
    compute: Compute<'_>,
) -> Result<Vec<Vec<f32>>, Error> {
    project_range_with(
        file,
        tensor,
        first_row,
        count,
        input,
        None,
        max_output_elements,
        compute,
    )
}

/// True when one-position projections of `tensor` take the one-submission
/// path ([`project_one_position`]).
fn one_position_fast(tensor: &TensorInfo, compute: Compute<'_>) -> bool {
    compute.workers.is_some()
        && compute.kernels.planes() == 2
        && pack_enabled()
        && tensor.shape[2..] == [1, 1]
        && Encoding::try_from(tensor.type_id)
            .is_ok_and(|e| compute.kernels.quantizes(e) && compute.kernels.packs(e))
        && std::env::var_os("PQ_ONEPOS_OLD").is_none()
}

/// As [`project_range`], optionally with `input` already quantized by
/// [`quantize_input`] for this tensor's encoding and these kernels.
#[allow(clippy::too_many_arguments)]
fn project_range_with(
    file: &ModelFile,
    tensor: &TensorInfo,
    first_row: u64,
    count: usize,
    input: &[Vec<f32>],
    prepared: Option<&Quantized>,
    max_output_elements: usize,
    compute: Compute<'_>,
) -> Result<Vec<Vec<f32>>, Error> {
    if let (Some(workers), [x], None) = (compute.workers, input, prepared)
        && one_position_fast(tensor, compute)
    {
        matches_file(file, tensor)?;
        return Ok(vec![project_one_position(
            file,
            tensor,
            first_row,
            count,
            x,
            false,
            workers,
            compute.kernels,
        )?]);
    }
    let mut output = rows(input.len(), count, max_output_elements)?;
    for row in &mut output {
        row.resize(count, 0.0);
    }
    let parts: Vec<&mut [f32]> = output.iter_mut().map(Vec::as_mut_slice).collect();
    project_into(file, tensor, first_row, input, prepared, parts, compute)?;
    Ok(output)
}

/// [`project_range_with`] into caller-owned rows: `output[p]` (one per input
/// position, all the same length) receives rows `[first_row, first_row +
/// len)`; the same values as the allocating form.
fn project_into<R: AsRef<[f32]> + Sync>(
    file: &ModelFile,
    tensor: &TensorInfo,
    first_row: u64,
    input: &[R],
    prepared: Option<&Quantized>,
    mut output: Vec<&mut [f32]>,
    compute: Compute<'_>,
) -> Result<(), Error> {
    let count = output.first().map_or(0, |r| r.len());
    if count == 0 || output.len() != input.len() || output.iter().any(|r| r.len() != count) {
        return Err(Error::Shape("projection output rows"));
    }
    let workers = compute.workers;
    matches_file(file, tensor)?;
    let cols = usize::try_from(tensor.shape[0]).map_err(|_| Error::ResourceLimit)?;
    if prepared.is_some() {
        // Prepared inputs have already passed the quantizer's finite checks.
        // Each expert row chunk reads those bytes, so rescanning the same
        // original floats per chunk only repeats work. Keep shape checks.
        if cols == 0 || input.is_empty() || input.iter().any(|row| row.as_ref().len() != cols) {
            return Err(Error::Shape("batch input dimensions"));
        }
    } else {
        inputs_with(input, cols, workers)?;
    }
    let available = crate::engine::weights::row_count(tensor)?;
    if u64::try_from(count)
        .ok()
        .and_then(|n| first_row.checked_add(n))
        .filter(|&n| n <= available)
        .is_none()
    {
        return Err(Error::Shape("batch projection row range"));
    }
    cells(input.len(), cols, MAX_REFERENCE_MATRIX_ELEMENTS)?;
    let tile_rows = (MAX_REFERENCE_MATRIX_ELEMENTS / cols).min(128);
    if tile_rows == 0 {
        return Err(Error::ResourceLimit);
    }
    // Integer-dot backends quantize the activations once (in parallel),
    // shared by workers; the F64 kernels take the flat F32 activations.
    let owned;
    let quantized: Option<&Quantized> = match (prepared, Encoding::try_from(tensor.type_id)) {
        (Some(q), Ok(encoding)) if q.input.encoding() == encoding && q.positions == input.len() => {
            Some(q)
        }
        (Some(_), _) => return Err(Error::Shape("prepared input does not match the tensor")),
        (None, Ok(encoding)) if compute.kernels.quantizes(encoding) => {
            let _span = (compute.workers.is_some()).then(|| {
                crate::engine::stats::Span::new(crate::engine::stats::Stage::ProjectionQuantize)
            });
            owned = quantize_input(encoding, input, cols, compute)?;
            Some(&owned)
        }
        _ => None,
    };
    let flat_input = if quantized.is_some() {
        Vec::new()
    } else {
        let mut flat = Vec::with_capacity(input.len() * cols);
        for row in input {
            flat.extend_from_slice(row.as_ref());
        }
        flat
    };
    let kernels = compute.kernels;
    let packing = quantized.is_some()
        && pack_enabled()
        && Encoding::try_from(tensor.type_id).is_ok_and(|e| kernels.packs(e));
    // With workers: about 4 row chunks per worker (multiples of 8 rows),
    // pulled dynamically so faster threads take more; each chunk writes its
    // own disjoint output slices and computes exactly what a static split would.
    let n = match workers {
        Some(w) => (w.count() * 4).min(count.div_ceil(8)).max(1),
        None => 1,
    };
    // Chunk lengths: balanced, or (packed weights) whole kernel tiles of
    // align_rows rows, so no chunk but the last ends in a partial tile.
    let lengths: Vec<usize> = if packing {
        // Whole tiles spread evenly over n items (larger items first, so the
        // dynamic pull ends on small ones); the last item is cut at `count`.
        let align = align_rows(
            quantized
                .as_ref()
                .map_or(input.len(), |q| q.input.positions()),
        );
        let tiles = count.div_ceil(align);
        let n = n.min(tiles);
        let (base, extra) = (tiles / n, tiles % n);
        let mut left = count;
        (0..n)
            .map(|i| {
                let len = ((base + usize::from(i < extra)) * align).min(left);
                left -= len;
                len
            })
            .filter(|&len| len > 0)
            .collect()
    } else {
        let (base, remainder) = (count / n, count % n);
        (0..n).map(|i| base + usize::from(i < remainder)).collect()
    };
    let mut first = 0;
    let mut tasks: Vec<_> = lengths
        .iter()
        .map(|&len| {
            first += len;
            (first - len, Vec::<&mut [f32]>::with_capacity(input.len()))
        })
        .collect();
    for row in &mut output {
        let mut tail = &mut row[..];
        for ((_, parts), &len) in tasks.iter_mut().zip(&lengths) {
            let (part, rest) = tail.split_at_mut(len);
            parts.push(part);
            tail = rest;
        }
    }
    let compute = |(first, mut parts): (usize, Vec<&mut [f32]>)| -> Result<(), Error> {
        let count = parts[0].len();
        // Decode scratch only for the F64 kernels; one output tile per worker.
        let mut scratch = if quantized.is_some() {
            Vec::new()
        } else {
            zeros::<f32>(cells(
                tile_rows.min(count),
                cols,
                MAX_REFERENCE_MATRIX_ELEMENTS,
            )?)?
        };
        let kernel_positions = quantized
            .as_ref()
            .map_or(input.len(), |q| q.input.positions());
        let mut tile_out =
            zeros::<f32>(cells(kernel_positions, tile_rows.min(count), usize::MAX)?)?;
        for local in (0..count).step_by(tile_rows) {
            let n = (count - local).min(tile_rows);
            let offset = first_row
                .checked_add((first + local) as u64)
                .ok_or(Error::ResourceLimit)?;
            let projected = &mut tile_out[..kernel_positions * n];
            match quantized.as_ref() {
                Some(q) if packing => project_packed_rows(
                    file,
                    tensor,
                    available,
                    cols,
                    offset,
                    n,
                    &q.input,
                    projected,
                    compute.kernels,
                )?,
                _ => {
                    let tile = file.view_rows(
                        &tensor.name,
                        offset,
                        n,
                        MAX_REFERENCE_MATRIX_ELEMENTS * size_of::<f32>(),
                    )?;
                    let view = EncodedRows::new(tile.encoding, n, cols, &tile.bytes)?;
                    if let Some(q) = quantized.as_ref() {
                        compute
                            .kernels
                            .project_quantized(&view, &q.input, projected)?;
                    } else {
                        compute.kernels.project(
                            &view,
                            &flat_input,
                            input.len(),
                            projected,
                            &mut scratch,
                        )?;
                    }
                }
            }
            let second = quantized.as_ref().and_then(|q| q.second_plane);
            for (p, row) in parts.iter_mut().enumerate() {
                let values = &projected[p * n..(p + 1) * n];
                match second {
                    Some(offset) => {
                        let fine = &projected[(offset + p) * n..(offset + p + 1) * n];
                        for ((out, a), b) in row[local..local + n].iter_mut().zip(values).zip(fine)
                        {
                            *out = a + b;
                        }
                    }
                    None => row[local..local + n].copy_from_slice(values),
                }
            }
        }
        Ok(())
    };
    if let Some(workers) = workers {
        let _span = crate::engine::stats::Span::new(crate::engine::stats::Stage::ProjectionWorkers);
        let slots: Vec<std::sync::Mutex<Option<_>>> = tasks
            .into_iter()
            .map(|t| std::sync::Mutex::new(Some(t)))
            .collect();
        let order: Vec<usize> = (0..slots.len()).collect();
        Compute {
            workers: Some(workers),
            kernels,
        }
        .map_items(slots.len(), &order, |i| {
            let task = slots[i]
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .take()
                .expect("each chunk runs once");
            compute(task)
        })?;
    } else {
        compute(tasks.pop().expect("one serial task"))?;
    }
    Ok(())
}

pub fn project_streamed_batch(
    file: &ModelFile,
    tensor: &TensorInfo,
    input: &[Vec<f32>],
    max_output_elements: usize,
) -> Result<Vec<Vec<f32>>, Error> {
    project_streamed_batch_with_workers(file, tensor, input, max_output_elements, None)
}
pub fn project_streamed_batch_with_workers(
    file: &ModelFile,
    tensor: &TensorInfo,
    input: &[Vec<f32>],
    max_output_elements: usize,
    workers: Option<&Workers>,
) -> Result<Vec<Vec<f32>>, Error> {
    project_streamed_batch_with(
        file,
        tensor,
        input,
        max_output_elements,
        Compute::portable(workers),
    )
}
/// As [`project_streamed_batch_with_workers`] with an explicit kernel pair.
pub fn project_streamed_batch_with(
    file: &ModelFile,
    tensor: &TensorInfo,
    input: &[Vec<f32>],
    max_output_elements: usize,
    compute: Compute<'_>,
) -> Result<Vec<Vec<f32>>, Error> {
    if tensor.shape[2..] != [1, 1] {
        return Err(Error::Shape("batch matrix rank"));
    }
    project_range(
        file,
        tensor,
        0,
        usize::try_from(tensor.shape[1]).map_err(|_| Error::ResourceLimit)?,
        input,
        max_output_elements,
        compute,
    )
}

/// Tile-local coefficients avoid repeatedly validating/copying the full
/// [positions,vocabulary] matrix. F64 accumulation survives all row tiles.
pub fn project_transposed_streamed_batch(
    file: &ModelFile,
    tensor: &TensorInfo,
    weights: &[Vec<f32>],
    max_output_elements: usize,
) -> Result<Vec<Vec<f32>>, Error> {
    project_transposed_streamed_batch_with_workers(file, tensor, weights, max_output_elements, None)
}
pub fn project_transposed_streamed_batch_with_workers(
    file: &ModelFile,
    tensor: &TensorInfo,
    weights: &[Vec<f32>],
    max_output_elements: usize,
    workers: Option<&Workers>,
) -> Result<Vec<Vec<f32>>, Error> {
    project_transposed_streamed_batch_with(
        file,
        tensor,
        weights,
        max_output_elements,
        Compute::portable(workers),
    )
}
/// As [`project_transposed_streamed_batch_with_workers`] with an explicit kernel pair.
pub fn project_transposed_streamed_batch_with(
    file: &ModelFile,
    tensor: &TensorInfo,
    weights: &[Vec<f32>],
    max_output_elements: usize,
    compute: Compute<'_>,
) -> Result<Vec<Vec<f32>>, Error> {
    let (workers, kernels) = (compute.workers, compute.kernels);
    matches_file(file, tensor)?;
    if tensor.shape[2..] != [1, 1] {
        return Err(Error::Shape("transposed batch matrix rank"));
    }
    let count = usize::try_from(tensor.shape[1]).map_err(|_| Error::ResourceLimit)?;
    let cols = usize::try_from(tensor.shape[0]).map_err(|_| Error::ResourceLimit)?;
    inputs(weights, count)?;
    cells(weights.len(), cols, max_output_elements)?;
    let tile_rows = (MAX_REFERENCE_MATRIX_ELEMENTS / cols).min(128);
    if tile_rows == 0 {
        return Err(Error::ResourceLimit);
    }
    if let Some(workers) = workers.filter(|w| w.count() > 1 && kernels.approximate()) {
        // Approximate backends: workers split the ROWS (each row read and
        // decoded once), each with its own binary64 accumulator for all
        // positions; accumulators are summed in worker order. Results may
        // differ with the worker count in the last binary64 bits.
        let tiles = count.div_ceil(tile_rows);
        let parts = workers.ranges(tiles, |first, end| {
            transposed_rows(
                file,
                tensor,
                weights,
                first * tile_rows,
                (end * tile_rows).min(count),
                kernels,
            )
        })?;
        let mut total = zeros::<f64>(cells(weights.len(), cols, max_output_elements)?)?;
        for part in parts {
            for (t, v) in total.iter_mut().zip(part) {
                *t += v;
            }
        }
        let mut output = zeros::<f32>(total.len())?;
        quantized::finish(&total, &mut output)?;
        return Ok(output.chunks_exact(cols).map(<[f32]>::to_vec).collect());
    }
    if let Some(workers) = workers.filter(|w| w.count() > 1 && weights.len() > 1) {
        // Shared global skip mask preserves the serial tile visitation, even
        // when all coefficients in one worker's position subset are zero.
        let active: Vec<_> = (0..count)
            .step_by(tile_rows)
            .map(|first| {
                let end = (first + tile_rows).min(count);
                weights
                    .iter()
                    .any(|row| row[first..end].iter().any(|&v| v != 0.0))
            })
            .collect();
        let fragments = workers.ranges(weights.len(), |first, end| {
            transposed_segment(
                file,
                tensor,
                &weights[first..end],
                max_output_elements,
                Some(&active),
                kernels,
            )
        })?;
        Ok(fragments.into_iter().flatten().collect())
    } else {
        transposed_segment(file, tensor, weights, max_output_elements, None, kernels)
    }
}
/// Binary64 accumulator `[positions, cols]` over rows `first..end` (whole
/// 128-row tiles), all positions at once.
fn transposed_rows(
    file: &ModelFile,
    tensor: &TensorInfo,
    weights: &[Vec<f32>],
    first: usize,
    end: usize,
    kernels: MatrixKernels,
) -> Result<Vec<f64>, Error> {
    let cols = usize::try_from(tensor.shape[0]).map_err(|_| Error::ResourceLimit)?;
    let tile_rows = (MAX_REFERENCE_MATRIX_ELEMENTS / cols).min(128);
    let mut accumulator = zeros::<f64>(cells(weights.len(), cols, usize::MAX)?)?;
    let mut scratch = zeros::<f32>(cells(tile_rows, cols, MAX_REFERENCE_MATRIX_ELEMENTS)?)?;
    let mut coefficients = zeros::<f32>(cells(weights.len(), tile_rows, usize::MAX)?)?;
    for start in (first..end).step_by(tile_rows) {
        let n = (end - start).min(tile_rows);
        let tile = file.view_rows(
            &tensor.name,
            start as u64,
            n,
            MAX_REFERENCE_MATRIX_ELEMENTS * size_of::<f32>(),
        )?;
        let view = EncodedRows::new(tile.encoding, n, cols, &tile.bytes)?;
        for (dst, row) in coefficients.chunks_exact_mut(n).zip(weights) {
            dst.copy_from_slice(&row[start..start + n]);
        }
        kernels.accumulate_transposed(
            &view,
            0,
            &coefficients[..weights.len() * n],
            n,
            weights.len(),
            &mut accumulator,
            &mut scratch,
        )?;
    }
    Ok(accumulator)
}

fn transposed_segment(
    file: &ModelFile,
    tensor: &TensorInfo,
    weights: &[Vec<f32>],
    max_output_elements: usize,
    active: Option<&[bool]>,
    kernels: MatrixKernels,
) -> Result<Vec<Vec<f32>>, Error> {
    let count = usize::try_from(tensor.shape[1]).map_err(|_| Error::ResourceLimit)?;
    let cols = usize::try_from(tensor.shape[0]).map_err(|_| Error::ResourceLimit)?;
    inputs(weights, count)?;
    let out_cells = cells(weights.len(), cols, max_output_elements)?;
    let tile_rows = (MAX_REFERENCE_MATRIX_ELEMENTS / cols).min(128);
    if tile_rows == 0 {
        return Err(Error::ResourceLimit);
    }
    let mut accumulator = zeros::<f64>(out_cells)?;
    let mut scratch = zeros::<f32>(cells(
        tile_rows.min(count),
        cols,
        MAX_REFERENCE_MATRIX_ELEMENTS,
    )?)?;
    for first in (0..count).step_by(tile_rows) {
        let n = (count - first).min(tile_rows);
        let end = first + n;
        let skip = active.map_or_else(
            || {
                weights
                    .iter()
                    .all(|w| w[first..end].iter().all(|&v| v == 0.0))
            },
            |mask| !mask[first / tile_rows],
        );
        if skip {
            continue;
        }
        let tile = file.view_rows(
            &tensor.name,
            first as u64,
            n,
            MAX_REFERENCE_MATRIX_ELEMENTS * size_of::<f32>(),
        )?;
        let view = EncodedRows::new(tile.encoding, n, cols, &tile.bytes)?;
        let mut coefficients = Vec::new();
        coefficients
            .try_reserve_exact(cells(weights.len(), n, MAX_REFERENCE_MATRIX_ELEMENTS)?)
            .map_err(|_| Error::ResourceLimit)?;
        for row in weights {
            coefficients.extend_from_slice(&row[first..end]);
        }
        kernels.accumulate_transposed(
            &view,
            0,
            &coefficients,
            n,
            weights.len(),
            &mut accumulator,
            &mut scratch,
        )?;
    }
    let mut output = zeros::<f32>(out_cells)?;
    quantized::finish(&accumulator, &mut output)?;
    Ok(output.chunks_exact(cols).map(<[f32]>::to_vec).collect())
}

/// `gelu(gate) * up` per pair. With fewer pairs than workers (one-token
/// decode), each pair's elements are also split across workers; the product
/// is elementwise, so the values are identical either way.
fn gelu_products(pairs: &[(&[f32], &[f32])], compute: Compute<'_>) -> Result<Vec<Vec<f32>>, Error> {
    let lens: Vec<usize> = pairs.iter().map(|(g, _)| g.len()).collect();
    compute.map_split(&lens, 128, |i, a, b| {
        Ok::<_, Error>(math::gelu_product(&pairs[i].0[a..b], &pairs[i].1[a..b])?)
    })
}

fn gated_batch(
    file: &ModelFile,
    binding: &MlpBindings<'_>,
    input: &[Vec<f32>],
    compute: Compute<'_>,
) -> Result<Vec<Vec<f32>>, Error> {
    if binding.gate.shape[..2] != binding.up.shape[..2]
        || binding.down.shape[0] != binding.gate.shape[1]
        || binding.down.shape[1] != binding.gate.shape[0]
    {
        return Err(Error::Shape("batch gated MLP dimensions"));
    }
    let mut both = project_many(file, &[binding.gate, binding.up], input, compute)?.into_iter();
    let (Some(gate), Some(up)) = (both.next(), both.next()) else {
        return Err(Error::Shape("gated MLP projections"));
    };
    if let (Some(workers), [g], [u]) = (compute.workers, gate.as_slice(), up.as_slice())
        && one_position_fast(binding.down, compute)
    {
        // One token: the GELU products are formed and quantized by the
        // workers inside the down projection's submission.
        let rows = usize::try_from(binding.down.shape[1]).map_err(|_| Error::ResourceLimit)?;
        let x: Vec<f32> = g.iter().chain(u).copied().collect();
        return Ok(vec![project_one_position(
            file,
            binding.down,
            0,
            rows,
            &x,
            true,
            workers,
            compute.kernels,
        )?]);
    }
    let pairs: Vec<(&[f32], &[f32])> = gate
        .iter()
        .zip(&up)
        .map(|(g, u)| (g.as_slice(), u.as_slice()))
        .collect();
    let product = gelu_products(&pairs, compute)?;
    project_streamed_batch_with(
        file,
        binding.down,
        &product,
        MAX_REFERENCE_MATRIX_ELEMENTS,
        compute,
    )
}

/// All routed experts of one layer with workers: each expert's gate/up and
/// down projections are split into row chunks (more chunks for experts with
/// more tokens) so no single expert bounds the stage. Every (row, token)
/// value and every GELU product is computed exactly as in [`expert_batch`].
/// Returns `[group][token][hidden]`.
fn experts_chunked(
    file: &ModelFile,
    layer: &LayerBindings<'_>,
    groups: &[(usize, Vec<(usize, f32)>)],
    expert_input: &[Vec<f32>],
    compute: Compute<'_>,
) -> Result<Vec<Vec<Vec<f32>>>, Error> {
    let moe = layer
        .moe
        .as_ref()
        .ok_or(Error::Shape("layer has no routed experts"))?;
    routed_experts(
        file,
        moe.expert_gate_up,
        None,
        moe.expert_down,
        ExpertActivation::GeluProduct,
        groups,
        expert_input,
        compute,
    )
}

/// The activation between an expert's up and down projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExpertActivation {
    /// Fused gate|up rows: `gelu(gate) * up` (Gemma).
    GeluProduct,
    /// `relu(up)^2` (Nemotron).
    ReluSquared,
    /// Gate|up rows: `silu(gate) * up` (Qwen 3.5/3.6).
    SiluProduct,
}
impl ExpertActivation {
    /// Columns `[a, z)` of the down projection's input, from one up row of
    /// `inner` (the down projection's width) or `2 * inner` values.
    fn apply(self, up: &[f32], inner: usize, a: usize, z: usize) -> Result<Vec<f32>, Error> {
        match self {
            Self::GeluProduct => Ok(math::gelu_product(&up[a..z], &up[inner + a..inner + z])?),
            Self::SiluProduct => Ok(up[a..z]
                .iter()
                .zip(&up[inner + a..inner + z])
                .map(|(&g, &u)| {
                    let g = f64::from(g);
                    (g / (1.0 + crate::libm::exp(-g)) * f64::from(u)) as f32
                })
                .collect()),
            Self::ReluSquared => Ok(up[a..z]
                .iter()
                .map(|&v| {
                    let r = v.max(0.0);
                    r * r
                })
                .collect()),
        }
    }
    fn up_rows(self, inner: u64) -> Option<u64> {
        match self {
            Self::GeluProduct | Self::SiluProduct => inner.checked_mul(2),
            Self::ReluSquared => Some(inner),
        }
    }
}

/// The routed experts `groups` (expert, its `(token, weight)`s) of one
/// layer, as [`experts_chunked`] for any expert tensors: `gu` `[hidden,
/// up rows, experts]` (fused gate|up rows, or only the gate rows when the up
/// rows are the separate `up` of the same shape and encoding), `down`
/// `[inner, hidden, experts]`. Weights are not applied. Returns
/// `[group][token][hidden]`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn routed_experts(
    file: &ModelFile,
    gu: &TensorInfo,
    up: Option<&TensorInfo>,
    down: &TensorInfo,
    activation: ExpertActivation,
    groups: &[(usize, Vec<(usize, f32)>)],
    expert_input: &[Vec<f32>],
    compute: Compute<'_>,
) -> Result<Vec<Vec<Vec<f32>>>, Error> {
    // Rows of the (virtual) fused gate|up matrix of one expert.
    let fused_rows = match up {
        None => Some(gu.shape[1]),
        Some(u) if u.shape == gu.shape && u.type_id == gu.type_id => gu.shape[1].checked_mul(2),
        Some(_) => None,
    };
    if gu.shape[3] != 1
        || down.shape[3] != 1
        || gu.shape[2] != down.shape[2]
        || groups.iter().any(|(e, _)| *e as u64 >= gu.shape[2])
        || activation.up_rows(down.shape[0]) != fused_rows
        || down.shape[1] != gu.shape[0]
    {
        return Err(Error::Shape("batch fused expert dimensions/index"));
    }
    let gu_rows =
        usize::try_from(fused_rows.expect("checked")).map_err(|_| Error::ResourceLimit)?;
    let down_rows = usize::try_from(down.shape[1]).map_err(|_| Error::ResourceLimit)?;
    let half = usize::try_from(down.shape[0]).map_err(|_| Error::ResourceLimit)?;
    if let (Some(workers), [x]) = (compute.workers, expert_input) {
        let packs = |t: &TensorInfo| {
            Encoding::try_from(t.type_id)
                .is_ok_and(|e| compute.kernels.quantizes(e) && compute.kernels.packs(e))
        };
        if compute.kernels.planes() == 2
            && pack_enabled()
            && packs(gu)
            && packs(down)
            && std::env::var_os("PQ_EXPERTS_OLD").is_none()
        {
            return experts_one_token(
                file,
                gu,
                up,
                down,
                activation,
                groups,
                x,
                gu_rows,
                down_rows,
                half,
                workers,
                compute.kernels,
            );
        }
    }
    let workers = compute.workers.map_or(1, Workers::count);
    let total: usize = groups.iter().map(|(_, t)| t.len()).sum::<usize>().max(1);
    // Chunks per expert proportional to its tokens: about 4 items per worker.
    let chunks = |tokens: usize, rows: usize| -> usize {
        (tokens * workers * 4)
            .div_ceil(total)
            .clamp(1, 16)
            .min(rows.div_ceil(16))
    };
    let serial = compute.serial();
    // Largest token groups first, so the pull ends on small ones.
    let mut group_order: Vec<usize> = (0..groups.len()).collect();
    group_order.sort_by_key(|&g| std::cmp::Reverse(groups[g].1.len()));
    // Each expert's input quantized once, shared by all of its row chunks
    // (None when the kernels take F32 activations).
    let quantize_for = |tensor: &TensorInfo, rows: &[&[f32]]| -> Result<Option<Quantized>, Error> {
        let cols = usize::try_from(tensor.shape[0]).map_err(|_| Error::ResourceLimit)?;
        match Encoding::try_from(tensor.type_id) {
            Ok(encoding) if compute.kernels.quantizes(encoding) => {
                Ok(Some(quantize_input(encoding, rows, cols, serial)?))
            }
            _ => Ok(None),
        }
    };
    let gather = |g: usize| -> Vec<&[f32]> {
        groups[g]
            .1
            .iter()
            .map(|&(t, _)| expert_input[t].as_slice())
            .collect()
    };
    // One token (decode): every expert has the same input row, quantized
    // once. Otherwise each expert's rows are gathered and quantized in one
    // submission over the experts.
    let same_inputs = expert_input.len() == 1;
    let (inputs, quantized): (Vec<Vec<&[f32]>>, Vec<Quantized>) = if same_inputs {
        let inputs: Vec<Vec<&[f32]>> = (0..groups.len()).map(gather).collect();
        let quantized = quantize_for(gu, &inputs[0])?.into_iter().collect();
        (inputs, quantized)
    } else {
        // Every token's input is quantized once (each token goes to top-k
        // experts); an expert's input is a gather of its tokens' positions,
        // the same bytes as quantizing its rows again.
        let shared = match Encoding::try_from(gu.type_id) {
            Ok(encoding) if compute.kernels.quantizes(encoding) => {
                let cols = usize::try_from(gu.shape[0]).map_err(|_| Error::ResourceLimit)?;
                Some(quantize_input(encoding, expert_input, cols, compute)?)
            }
            _ => None,
        };
        let both = compute.map_items(groups.len(), &group_order, |g| {
            let rows = gather(g);
            let q = match &shared {
                Some(all) => Some(gather_quantized(all, &groups[g].1)?),
                None => quantize_for(gu, &rows)?,
            };
            Ok::<_, Error>((rows, q))
        })?;
        let (inputs, quantized): (Vec<_>, Vec<_>) = both.into_iter().unzip();
        (inputs, quantized.into_iter().flatten().collect())
    };
    // (group, first row, rows) items over a [rows]-row expert matrix.
    let items = |rows: usize| -> Vec<(usize, usize, usize)> {
        let mut items = Vec::new();
        for (g, (_, tokens)) in groups.iter().enumerate() {
            let n = chunks(tokens.len(), rows);
            // Whole kernel tiles per item (see align_rows).
            let align = align_rows(tokens.len() * compute.kernels.planes());
            let per = rows.div_ceil(n).next_multiple_of(align);
            for a in (0..rows).step_by(per) {
                items.push((g, a, per.min(rows - a)));
            }
        }
        items
    };
    let run = |tensor: &TensorInfo,
               rows: usize,
               inputs: &[Vec<&[f32]>],
               quantized: &[Quantized],
               same_inputs: bool|
     -> Result<Vec<Vec<Vec<f32>>>, Error> {
        // `quantized`: one per group, or one for all when every group has the
        // same rows (one token), or none.
        let prepared = |g: usize| quantized.get(if same_inputs { 0 } else { g });
        let items = items(rows);
        let mut order: Vec<usize> = (0..items.len()).collect();
        order.sort_by_key(|&i| std::cmp::Reverse(items[i].2 * inputs[items[i].0].len()));
        // [group][token][rows], allocated once; each item writes its own row
        // range of its group's rows directly (items of a group are contiguous
        // in `items`, in row order).
        let mut out: Vec<Vec<Vec<f32>>> = inputs
            .iter()
            .map(|input| (0..input.len()).map(|_| vec![0.0f32; rows]).collect())
            .collect();
        let mut slots: Vec<Vec<&mut [f32]>> = items.iter().map(|_| Vec::new()).collect();
        let mut next = 0;
        for (g, group) in out.iter_mut().enumerate() {
            let begin = next;
            while next < items.len() && items[next].0 == g {
                next += 1;
            }
            for row in group.iter_mut() {
                let mut tail = row.as_mut_slice();
                for (i, slot) in slots.iter_mut().enumerate().take(next).skip(begin) {
                    let (part, rest) = tail.split_at_mut(items[i].2);
                    slot.push(part);
                    tail = rest;
                }
            }
        }
        let slots: Vec<std::sync::Mutex<Option<Vec<&mut [f32]>>>> = slots
            .into_iter()
            .map(|s| std::sync::Mutex::new(Some(s)))
            .collect();
        compute.map_items(items.len(), &order, |i| {
            let (g, first, _) = items[i];
            let base = (groups[g].0 as u64)
                .checked_mul(rows as u64)
                .and_then(|n| n.checked_add(first as u64))
                .ok_or(Error::ResourceLimit)?;
            let parts = slots[i]
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .take()
                .expect("each item runs once");
            project_into(file, tensor, base, &inputs[g], prepared(g), parts, serial)
        })?;
        drop(slots);
        Ok(out)
    };
    let projected = match up {
        None => run(gu, gu_rows, &inputs, &quantized, same_inputs)?,
        Some(up) => {
            // Separate gate and up matrices (same encoding, so the same
            // quantized inputs), joined into gate|up rows.
            let gate = run(gu, half, &inputs, &quantized, same_inputs)?;
            let ups = run(up, half, &inputs, &quantized, same_inputs)?;
            gate.into_iter()
                .zip(ups)
                .map(|(g, u)| {
                    g.into_iter()
                        .zip(u)
                        .map(|(mut g, u)| {
                            g.extend(u);
                            g
                        })
                        .collect()
                })
                .collect()
        }
    };
    drop(inputs);
    drop(quantized);
    let pairs: Vec<(usize, usize)> = projected
        .iter()
        .enumerate()
        .flat_map(|(g, rows)| (0..rows.len()).map(move |t| (g, t)))
        .collect();
    let flat = match activation {
        ExpertActivation::GeluProduct => {
            let halves: Vec<(&[f32], &[f32])> = pairs
                .iter()
                .map(|&(g, t)| projected[g][t].split_at(half))
                .collect();
            gelu_products(&halves, compute)?
        }
        ExpertActivation::ReluSquared | ExpertActivation::SiluProduct => {
            compute.map_positions(pairs.len(), |i| {
                let (g, t) = pairs[i];
                activation.apply(&projected[g][t], half, 0, half)
            })?
        }
    };
    drop(projected);
    let mut products: Vec<Vec<Vec<f32>>> = groups.iter().map(|_| Vec::new()).collect();
    for ((g, _), product) in pairs.into_iter().zip(flat) {
        products[g].push(product);
    }
    let product_rows: Vec<Vec<&[f32]>> = products
        .iter()
        .map(|group| group.iter().map(Vec::as_slice).collect())
        .collect();
    let quantized = compute.map_items(groups.len(), &group_order, |g| {
        quantize_for(down, &product_rows[g])
    })?;
    let quantized: Vec<Quantized> = quantized.into_iter().flatten().collect();
    run(down, down_rows, &product_rows, &quantized, false)
}

/// Rows `[first_row, first_row + count)` of `tensor` times one position, on
/// the workers with two-plane integer kernels and packed weights, in one
/// submission: the input (or, with `gelu`, the GELU products of its two
/// halves `x[..h]`, `x[h..]`, formed block by block) is quantized by the
/// workers, then balanced row items are projected. The same functions on the
/// same values as quantize_input / gelu_products and project_into.
#[allow(clippy::too_many_arguments)]
fn project_one_position(
    file: &ModelFile,
    tensor: &TensorInfo,
    first_row: u64,
    count: usize,
    x: &[f32],
    gelu: bool,
    workers: &Workers,
    kernels: MatrixKernels,
) -> Result<Vec<f32>, Error> {
    let encoding = Encoding::try_from(tensor.type_id).map_err(|_| Error::Shape("encoding"))?;
    let cols = usize::try_from(tensor.shape[0]).map_err(|_| Error::ResourceLimit)?;
    let available = crate::engine::weights::row_count(tensor)?;
    if first_row + count as u64 > available || (if gelu { 2 * cols } else { cols }) != x.len() {
        return Err(Error::Shape("one-position projection"));
    }
    let mut source = zeros::<f32>(cols)?;
    let mut residual = zeros::<f32>(cols)?;
    let mut quantized = QuantizedInput::allocate(encoding, 2, cols)?;
    let source_ptr = SharedOut(source.as_mut_ptr());
    if !gelu {
        source.copy_from_slice(x);
        if !math::all_finite(&source) {
            return Err(Error::Parameter("nonfinite batch input"));
        }
    }
    // SAFETY: `source` is complete (no GELU) or written block by block by
    // the thread that then quantizes those blocks; buffers outlive `writer`.
    let writer = unsafe {
        quantized.two_plane_writer_from_ptrs(&[source_ptr.0.cast_const()], residual.as_mut_ptr())
    }?;
    let blocks = writer.blocks();
    let element = cols / blocks.max(1);
    let mut out = zeros::<f32>(count)?;
    let out_ptr = SharedOut(out.as_mut_ptr());
    let items = even_items(
        1,
        count,
        workers.count(),
        FUSE_ITEMS_PER_WORKER,
        align_rows(2),
    );
    let mut order: Vec<usize> = (0..items.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(items[i].2));
    prepare_then_items(
        workers,
        blocks,
        |b0, b1| {
            if gelu {
                let (a, z) = (b0 * element, b1 * element);
                let product = math::gelu_product(&x[a..z], &x[cols + a..cols + z])?;
                // SAFETY: this block range is this worker's alone.
                unsafe { source_ptr.slice(a, z - a) }.copy_from_slice(&product);
            }
            // SAFETY: disjoint ranges; the blocks' source is written.
            Ok(unsafe { writer.quantize_blocks(b0, b1) }?)
        },
        &order,
        |i| {
            let (_, first, len) = items[i];
            let mut tmp = zeros::<f32>(cells(2, len, usize::MAX)?)?;
            // SAFETY: after the barrier.
            let input = unsafe { writer.input() };
            project_packed_rows(
                file,
                tensor,
                available,
                cols,
                first_row + first as u64,
                len,
                input,
                &mut tmp,
                kernels,
            )?;
            // SAFETY: items' output ranges are disjoint.
            let dst = unsafe { out_ptr.slice(first, len) };
            for ((d, a), b) in dst.iter_mut().zip(&tmp[..len]).zip(&tmp[len..]) {
                *d = a + b; // plane 1 + plane 2, as project_into
            }
            Ok(())
        },
    )?;
    Ok(out)
}

/// A raw output pointer shared by tasks that write disjoint ranges.
#[derive(Clone, Copy)]
struct SharedOut(*mut f32);
// SAFETY: users write disjoint ranges only, within the buffer's lifetime.
unsafe impl Send for SharedOut {}
unsafe impl Sync for SharedOut {}
impl SharedOut {
    /// # Safety
    /// `[offset, offset + len)` is in bounds and no other live slice
    /// overlaps it.
    unsafe fn slice<'s>(self, offset: usize, len: usize) -> &'s mut [f32] {
        // SAFETY: per the contract.
        unsafe { std::slice::from_raw_parts_mut(self.0.add(offset), len) }
    }
}

/// Exactly `per_worker` x workers items (as equal as whole tiles allow) over
/// `experts` matrices of `rows` rows each: (expert, first row, rows).
fn even_items(
    experts: usize,
    rows: usize,
    workers: usize,
    per_worker: usize,
    align: usize,
) -> Vec<(usize, usize, usize)> {
    let tiles = rows.div_ceil(align);
    let target = (workers * per_worker).clamp(experts, experts * tiles);
    let mut items = Vec::with_capacity(target);
    for g in 0..experts {
        let n = (target / experts + usize::from(g < target % experts)).clamp(1, tiles);
        let (base, extra) = (tiles / n, tiles % n);
        let mut first = 0;
        for i in 0..n {
            let len = ((base + usize::from(i < extra)) * align).min(rows - first);
            items.push((g, first, len));
            first += len;
        }
    }
    items
}

/// The routed experts of one token (two-plane integer kernels, packed
/// weights) in two submissions instead of five: (1) the workers quantize the
/// token's input, then project every expert's gate/up rows; (2) each worker
/// forms the activations (GELU products) of a block range of one expert and quantizes
/// those blocks, then all project the down rows. The same functions on the
/// same values as `experts_chunked`; outputs written in place.
#[allow(clippy::too_many_arguments)]
fn experts_one_token(
    file: &ModelFile,
    gu: &TensorInfo,
    up: Option<&TensorInfo>,
    down: &TensorInfo,
    activation: ExpertActivation,
    groups: &[(usize, Vec<(usize, f32)>)],
    x: &[f32],
    gu_rows: usize,
    down_rows: usize,
    half: usize,
    workers: &Workers,
    kernels: MatrixKernels,
) -> Result<Vec<Vec<Vec<f32>>>, Error> {
    let experts = groups.len();
    let cols = x.len();
    let gu_encoding = Encoding::try_from(gu.type_id).map_err(|_| Error::Shape("encoding"))?;
    let down_encoding = Encoding::try_from(down.type_id).map_err(|_| Error::Shape("encoding"))?;
    let (gu_available, down_available) = (
        crate::engine::weights::row_count(gu)?,
        crate::engine::weights::row_count(down)?,
    );
    let align = align_rows(2);
    let order_of = |items: &[(usize, usize, usize)]| {
        let mut order: Vec<usize> = (0..items.len()).collect();
        order.sort_by_key(|&i| std::cmp::Reverse(items[i].2));
        order
    };
    // Plane 1 + plane 2 of one item's projection into `dst`.
    let project = |tensor: &TensorInfo,
                   available: u64,
                   cols: usize,
                   row0: u64,
                   len: usize,
                   input: &QuantizedInput,
                   dst: &mut [f32]|
     -> Result<(), Error> {
        let mut out = zeros::<f32>(cells(input.positions(), len, usize::MAX)?)?;
        project_packed_rows(
            file, tensor, available, cols, row0, len, input, &mut out, kernels,
        )?;
        for ((d, a), b) in dst.iter_mut().zip(&out[..len]).zip(&out[len..2 * len]) {
            *d = a + b;
        }
        Ok(())
    };
    // (1) gate/up.
    let mut qx = QuantizedInput::allocate(gu_encoding, 2, cols)?;
    let mut rx = zeros::<f32>(cols)?;
    let rows_x = [x];
    let wx = qx.two_plane_writer(&rows_x, &mut rx)?;
    let mut gu_out = zeros::<f32>(cells(experts, gu_rows, usize::MAX)?)?;
    let gu_ptr = SharedOut(gu_out.as_mut_ptr());
    // Matrices of phase 1: each expert's fused gate|up rows, or (separate
    // `up`) its gate rows (matrices 0..experts) and its up rows (experts..),
    // written to the gate and up halves of the expert's output row.
    let (matrices, rows_per) = match up {
        None => (experts, gu_rows),
        Some(_) => (2 * experts, half),
    };
    let items = even_items(
        matrices,
        rows_per,
        workers.count(),
        EXPERT_ITEMS_PER_WORKER,
        align,
    );
    let order = order_of(&items);
    let span = crate::engine::stats::Span::new(crate::engine::stats::Stage::ExpertsUp);
    prepare_then_items(
        workers,
        wx.blocks(),
        // SAFETY: disjoint block ranges covering every block.
        |b0, b1| Ok(unsafe { wx.quantize_blocks(b0, b1) }?),
        &order,
        |i| {
            let (m, first, len) = items[i];
            let (g, part) = (m % experts, m / experts);
            let tensor = if part == 0 {
                gu
            } else {
                up.expect("separate up")
            };
            let row0 = (groups[g].0 * rows_per + first) as u64;
            let at = g * gu_rows + part * half + first;
            // SAFETY: after the barrier; items' output ranges are disjoint.
            let (input, dst) = unsafe { (wx.input(), gu_ptr.slice(at, len)) };
            project(tensor, gu_available, cols, row0, len, input, dst)
        },
    )?;
    drop(span);
    let _span = crate::engine::stats::Span::new(crate::engine::stats::Stage::ExpertsDown);
    // (2) GELU products, their quantization, and down.
    let mut products = zeros::<f32>(cells(experts, half, usize::MAX)?)?;
    let mut residual = zeros::<f32>(cells(experts, half, usize::MAX)?)?;
    let mut quantized = (0..experts)
        .map(|_| QuantizedInput::allocate(down_encoding, 2, half))
        .collect::<Result<Vec<_>, _>>()?;
    let product_ptr = SharedOut(products.as_mut_ptr());
    let residual_ptr = residual.as_mut_ptr();
    let writers = quantized
        .iter_mut()
        .enumerate()
        .map(|(g, q)| {
            // SAFETY: each expert's product row is written block by block
            // by the thread that then quantizes those blocks (below); the
            // buffers outlive the writers and are not otherwise accessed.
            unsafe {
                q.two_plane_writer_from_ptrs(
                    &[product_ptr.0.add(g * half).cast_const()],
                    residual_ptr.add(g * half),
                )
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let blocks = writers.first().map_or(0, |w| w.blocks());
    let element = half / blocks.max(1);
    let mut out: Vec<Vec<f32>> = (0..experts).map(|_| vec![0.0f32; down_rows]).collect();
    let out_ptrs: Vec<SharedOut> = out.iter_mut().map(|o| SharedOut(o.as_mut_ptr())).collect();
    let items = even_items(
        experts,
        down_rows,
        workers.count(),
        EXPERT_ITEMS_PER_WORKER,
        align,
    );
    let order = order_of(&items);
    prepare_then_items(
        workers,
        experts * blocks,
        |u0, u1| {
            let mut u = u0;
            while u < u1 {
                let (g, b0) = (u / blocks, u % blocks);
                let b1 = blocks.min(b0 + (u1 - u));
                let (a, z) = (b0 * element, b1 * element);
                let row = &gu_out[g * gu_rows..(g + 1) * gu_rows];
                let product = activation.apply(row, half, a, z)?;
                // SAFETY: this range of this expert's products belongs to
                // this worker alone; then its blocks are quantized here.
                unsafe {
                    product_ptr
                        .slice(g * half + a, z - a)
                        .copy_from_slice(&product);
                    writers[g].quantize_blocks(b0, b1)?;
                }
                u += b1 - b0;
            }
            Ok(())
        },
        &order,
        |i| {
            let (g, first, len) = items[i];
            let row0 = (groups[g].0 * down_rows + first) as u64;
            // SAFETY: after the barrier; items' output ranges are disjoint.
            let (input, dst) = unsafe { (writers[g].input(), out_ptrs[g].slice(first, len)) };
            project(down, down_available, half, row0, len, input, dst)
        },
    )?;
    drop(writers);
    Ok(out.into_iter().map(|row| vec![row]).collect())
}

fn expert_batch(
    file: &ModelFile,
    layer: &LayerBindings<'_>,
    expert: usize,
    input: &[Vec<f32>],
    compute: Compute<'_>,
) -> Result<Vec<Vec<f32>>, Error> {
    let moe = layer
        .moe
        .as_ref()
        .ok_or(Error::Shape("layer has no routed experts"))?;
    let gu = moe.expert_gate_up;
    let down = moe.expert_down;
    if gu.shape[3] != 1
        || down.shape[3] != 1
        || gu.shape[2] != down.shape[2]
        || expert as u64 >= gu.shape[2]
        || down.shape[0].checked_mul(2) != Some(gu.shape[1])
        || down.shape[1] != gu.shape[0]
    {
        return Err(Error::Shape("batch fused expert dimensions/index"));
    }
    let gu_rows = usize::try_from(gu.shape[1]).map_err(|_| Error::ResourceLimit)?;
    let down_rows = usize::try_from(down.shape[1]).map_err(|_| Error::ResourceLimit)?;
    let half = usize::try_from(down.shape[0]).map_err(|_| Error::ResourceLimit)?;
    let first_gu = (expert as u64)
        .checked_mul(gu.shape[1])
        .ok_or(Error::ResourceLimit)?;
    let first_down = (expert as u64)
        .checked_mul(down.shape[1])
        .ok_or(Error::ResourceLimit)?;
    let projected = project_range(
        file,
        gu,
        first_gu,
        gu_rows,
        input,
        MAX_REFERENCE_MATRIX_ELEMENTS,
        compute,
    )?;
    let product: Vec<_> = projected
        .iter()
        .map(|r| {
            let (gate, up) = r.split_at(half);
            math::gelu_product(gate, up)
        })
        .collect::<Result<_, _>>()?;
    project_range(
        file,
        down,
        first_down,
        down_rows,
        &product,
        MAX_REFERENCE_MATRIX_ELEMENTS,
        compute,
    )
}

pub fn self_condition_batch(
    file: &ModelFile,
    binding: &MlpBindings<'_>,
    pre_scale: &[f32],
    embeddings: &[Vec<f32>],
    soft: Option<&[Vec<f32>]>,
    epsilon: f32,
) -> Result<Vec<Vec<f32>>, Error> {
    self_condition_batch_with_workers(file, binding, pre_scale, embeddings, soft, epsilon, None)
}
#[allow(clippy::too_many_arguments)]
pub fn self_condition_batch_with_workers(
    file: &ModelFile,
    binding: &MlpBindings<'_>,
    pre_scale: &[f32],
    embeddings: &[Vec<f32>],
    soft: Option<&[Vec<f32>]>,
    epsilon: f32,
    workers: Option<&Workers>,
) -> Result<Vec<Vec<f32>>, Error> {
    self_condition_batch_with(
        file,
        binding,
        pre_scale,
        embeddings,
        soft,
        epsilon,
        Compute::portable(workers),
    )
}
/// As [`self_condition_batch_with_workers`] with an explicit kernel pair.
#[allow(clippy::too_many_arguments)]
pub fn self_condition_batch_with(
    file: &ModelFile,
    binding: &MlpBindings<'_>,
    pre_scale: &[f32],
    embeddings: &[Vec<f32>],
    soft: Option<&[Vec<f32>]>,
    epsilon: f32,
    compute: Compute<'_>,
) -> Result<Vec<Vec<f32>>, Error> {
    inputs(embeddings, pre_scale.len())?;
    cells(
        embeddings.len(),
        pre_scale.len(),
        MAX_REFERENCE_MATRIX_ELEMENTS,
    )?;
    if let Some(rows) = soft {
        inputs(rows, pre_scale.len())?;
        if rows.len() != embeddings.len() {
            return Err(Error::Shape("self-conditioning positions"));
        }
    }
    let zero = vec![0.0; pre_scale.len()];
    let normalized: Vec<_> = (0..embeddings.len())
        .map(|i| {
            math::rms_norm(
                soft.map_or(zero.as_slice(), |rows| rows[i].as_slice()),
                Some(pre_scale),
                epsilon,
            )
        })
        .collect::<Result<_, _>>()?;
    let signal = gated_batch(file, binding, &normalized, compute)?;
    embeddings
        .iter()
        .zip(signal)
        .map(|(embedding, signal)| {
            Ok(math::rms_norm(
                &math::add(embedding, &signal)?,
                None,
                epsilon,
            )?)
        })
        .collect()
}

/// Group selected tokens by ascending expert ID. Stream only that expert's
/// encoded slices, never expand the entire expert or all experts to F32.
/// Each token retains the scalar path's ascending-expert accumulation order.
pub fn feed_forward_batch(
    file: &ModelFile,
    layer: &LayerBindings<'_>,
    spec: &Specification,
    residual: &[Vec<f32>],
    mode: LayerMode,
) -> Result<Vec<FeedForwardResult>, Error> {
    feed_forward_batch_with_workers(file, layer, spec, residual, mode, None)
}
pub fn feed_forward_batch_with_workers(
    file: &ModelFile,
    layer: &LayerBindings<'_>,
    spec: &Specification,
    residual: &[Vec<f32>],
    mode: LayerMode,
    workers: Option<&Workers>,
) -> Result<Vec<FeedForwardResult>, Error> {
    feed_forward_batch_with(
        file,
        layer,
        spec,
        residual,
        mode,
        Compute::portable(workers),
    )
}
/// As [`feed_forward_batch_with_workers`] with an explicit kernel pair.
pub fn feed_forward_batch_with(
    file: &ModelFile,
    layer: &LayerBindings<'_>,
    spec: &Specification,
    residual: &[Vec<f32>],
    mode: LayerMode,
    compute: Compute<'_>,
) -> Result<Vec<FeedForwardResult>, Error> {
    feed_forward_layer(file, layer, spec, residual, mode, None, compute)
}

/// As [`feed_forward_batch_with`], with this layer's per-layer embeddings
/// (`per_layer[position]`, models with per-layer inputs only). Dense layers
/// (no routed experts) run `residual + post_ffw_norm(mlp(ffn_norm(x)))`.
/// The per-layer block (Gemma4TextDecoderLayer) follows the feed-forward
/// residual and precedes the layer scalar:
/// `x + post_norm(projection(gelu(gate(x)) * per_layer))`.
pub(crate) fn feed_forward_layer(
    file: &ModelFile,
    layer: &LayerBindings<'_>,
    spec: &Specification,
    residual: &[Vec<f32>],
    mode: LayerMode,
    per_layer: Option<&[Vec<f32>]>,
    compute: Compute<'_>,
) -> Result<Vec<FeedForwardResult>, Error> {
    inputs_with(residual, spec.hidden, compute.workers)?;
    cells(residual.len(), spec.hidden, MAX_REFERENCE_MATRIX_ELEMENTS)?;
    if layer.per_layer.is_some() != per_layer.is_some()
        || per_layer.is_some_and(|p| p.len() != residual.len())
    {
        return Err(Error::Shape("per-layer inputs do not match the layer"));
    }
    let shared_pre = load_vector(file, layer.shared_pre_norm)?;
    let combined_post = load_vector(file, layer.combined_ff_norm)?;
    let scale = load_vector(
        file,
        match mode {
            LayerMode::Encoder => layer.encoder_scale,
            LayerMode::Decoder => layer.decoder_scale,
        },
    )?;
    if scale.len() != 1 {
        return Err(Error::Shape("layer scalar must have one element"));
    }
    use crate::engine::stats::{Span, Stage};
    let positions = residual.len();
    let span = Span::new(Stage::SharedMlp);
    let shared_input = compute.map_positions(positions, |i| {
        Ok::<_, Error>(math::rms_norm(
            &residual[i],
            Some(&shared_pre),
            spec.rms_epsilon,
        )?)
    })?;
    let gated = gated_batch(file, &layer.shared_mlp, &shared_input, compute)?;
    let Some(moe) = layer.moe.as_ref() else {
        // Dense layer: residual + post_ffw_norm(mlp).
        let after = compute.map_positions(positions, |i| {
            let normalized = math::rms_norm(&gated[i], Some(&combined_post), spec.rms_epsilon)?;
            Ok::<_, Error>(math::add(&residual[i], &normalized)?)
        })?;
        drop(span);
        let hidden = finish_layer(file, layer, spec, after, per_layer, scale[0], compute)?;
        return Ok(hidden
            .into_iter()
            .map(|hidden| FeedForwardResult {
                hidden,
                routing: Routing {
                    probabilities: Vec::new(),
                    selected: Vec::new(),
                },
            })
            .collect());
    };
    let shared_post = load_vector(file, moe.shared_post_norm)?;
    let expert_pre = load_vector(file, moe.expert_pre_norm)?;
    let expert_post = load_vector(file, moe.expert_post_norm)?;
    let shared = compute.map_positions(positions, |i| {
        Ok::<_, Error>(math::rms_norm(
            &gated[i],
            Some(&shared_post),
            spec.rms_epsilon,
        )?)
    })?;
    drop(span);
    let span = Span::new(Stage::Router);
    let router = cached_router(file, &moe.router, spec.top_k, spec.rms_epsilon)?;
    // Router rows are independent F64 dot products: with fewer positions than
    // workers they are split across workers (identical values).
    let routing = if positions < compute.workers.map_or(1, Workers::count) {
        (0..positions)
            .map(|i| {
                router.route_with(&residual[i], |m, x| {
                    let rows = compute.map_split(&[m.rows()], 8, |_, a, b| {
                        Ok::<_, Error>(m.project_rows(x, a, b - a)?)
                    })?;
                    Ok(rows.into_iter().next().unwrap_or_default())
                })
            })
            .collect::<Result<Vec<_>, Error>>()?
    } else {
        // One contiguous range per worker; within it, every pass over the
        // router matrix serves 4 positions.
        match compute.workers {
            Some(workers) => workers
                .ranges(positions, |a, b| router.route_many(&residual[a..b]))?
                .into_iter()
                .flatten()
                .collect(),
            None => router.route_many(residual)?,
        }
    };
    drop(router);
    let expert_input = compute.map_positions(positions, |i| {
        Ok::<_, Error>(math::rms_norm(
            &residual[i],
            Some(&expert_pre),
            spec.rms_epsilon,
        )?)
    })?;
    let mut groups: BTreeMap<usize, Vec<(usize, f32)>> = BTreeMap::new();
    for (token, r) in routing.iter().enumerate() {
        for selected in &r.selected {
            groups
                .entry(selected.expert)
                .or_default()
                .push((token, selected.weight));
        }
    }
    // Experts are independent: with workers, each expert runs whole on one
    // worker (largest token groups first). Contributions are then added per
    // token in ascending expert order, exactly as in the serial loop.
    drop(span);
    let span = Span::new(Stage::Experts);
    let groups: Vec<(usize, Vec<(usize, f32)>)> = groups.into_iter().collect();
    let mut order: Vec<usize> = (0..groups.len()).collect();
    order.sort_by_key(|&g| std::cmp::Reverse(groups[g].1.len()));
    let contributions = if compute.workers.is_some() {
        experts_chunked(file, layer, &groups, &expert_input, compute)?
    } else {
        let serial = compute.serial();
        compute.map_items(groups.len(), &order, |g| {
            let (expert, tokens) = &groups[g];
            let selected_input: Vec<_> = tokens
                .iter()
                .map(|&(token, _)| expert_input[token].clone())
                .collect();
            expert_batch(file, layer, *expert, &selected_input, serial)
        })?
    };
    drop(span);
    let _span = Span::new(Stage::ExpertCombine);
    // Per token, its contributions in ascending expert order (groups are
    // ascending), each added exactly as in the serial loop.
    let mut per_token: Vec<Vec<(usize, usize, f32)>> = vec![Vec::new(); positions];
    for (g, (_, tokens)) in groups.iter().enumerate() {
        for (slot, &(token, weight)) in tokens.iter().enumerate() {
            per_token[token].push((g, slot, weight));
        }
    }
    let hidden = compute.map_positions(positions, |i| {
        // In place, the same binary32 operations as
        // add(sum, multiply_scaled(contribution, ones, weight)): (x * 1) * w
        // is x * w exactly, then one add; finiteness is checked by rms_norm.
        let mut sum = vec![0.0f32; spec.hidden];
        for &(g, slot, weight) in &per_token[i] {
            let contribution = &contributions[g][slot];
            if contribution.len() != sum.len() {
                return Err(Error::Shape("expert contribution width"));
            }
            for (s, &x) in sum.iter_mut().zip(contribution) {
                *s += x * weight;
            }
        }
        let experts = math::rms_norm(&sum, Some(&expert_post), spec.rms_epsilon)?;
        let combined = math::rms_norm(
            &math::add(&shared[i], &experts)?,
            Some(&combined_post),
            spec.rms_epsilon,
        )?;
        Ok::<_, Error>(math::add(&residual[i], &combined)?)
    })?;
    let hidden = finish_layer(file, layer, spec, hidden, per_layer, scale[0], compute)?;
    Ok(hidden
        .into_iter()
        .zip(routing)
        .map(|(hidden, routing)| FeedForwardResult { hidden, routing })
        .collect())
}

/// The per-layer-embedding block (if any), then the layer scalar.
fn finish_layer(
    file: &ModelFile,
    layer: &LayerBindings<'_>,
    spec: &Specification,
    after: Vec<Vec<f32>>,
    per_layer: Option<&[Vec<f32>]>,
    scale: f32,
    compute: Compute<'_>,
) -> Result<Vec<Vec<f32>>, Error> {
    let after = match (&layer.per_layer, per_layer) {
        (Some(b), Some(inputs)) => {
            let gate = project_streamed_batch_with(
                file,
                b.gate,
                &after,
                MAX_REFERENCE_MATRIX_ELEMENTS,
                compute,
            )?;
            if gate.iter().zip(inputs).any(|(g, p)| g.len() != p.len()) {
                return Err(Error::Shape("per-layer input width"));
            }
            let pairs: Vec<(&[f32], &[f32])> = gate
                .iter()
                .zip(inputs)
                .map(|(g, p)| (g.as_slice(), p.as_slice()))
                .collect();
            let gated = gelu_products(&pairs, compute)?;
            let projected = project_streamed_batch_with(
                file,
                b.projection,
                &gated,
                MAX_REFERENCE_MATRIX_ELEMENTS,
                compute,
            )?;
            let post = load_vector(file, b.post_norm)?;
            compute.map_positions(after.len(), |i| {
                let normalized = math::rms_norm(&projected[i], Some(&post), spec.rms_epsilon)?;
                Ok::<_, Error>(math::add(&after[i], &normalized)?)
            })?
        }
        (None, None) => after,
        _ => return Err(Error::Shape("per-layer inputs do not match the layer")),
    };
    let ones = vec![1.0; spec.hidden];
    compute.map_positions(after.len(), |i| {
        Ok::<_, Error>(math::multiply_scaled(&after[i], &ones, scale)?)
    })
}

#[cfg(test)]
mod parallel_tests;
#[cfg(test)]
mod tests;

#[cfg(test)]
mod two_plane_tests {
    use super::*;
    /// The one-call decode quantization equals the two passes (plane 1 with
    /// residuals, then [rows, residuals]) byte for byte.
    #[test]
    fn one_call_two_planes_match_two_passes() {
        // Two-plane kernels only exist natively; elsewhere there is nothing to check.
        let Ok(kernels) = crate::engine::execution::matrix_kernels(
            crate::engine::execution::MatrixBackend::Power9Int8x2,
        ) else {
            return;
        };
        assert_eq!(kernels.planes(), 2);
        {
            let compute = Compute {
                workers: None,
                kernels,
            };
            for (encoding, cols) in [
                (Encoding::Q4_0, 2816),
                (Encoding::Q8_0, 64),
                (Encoding::Q4K, 512),
                (Encoding::Q6K, 256),
            ] {
                for positions in [1usize, 2] {
                    let input: Vec<Vec<f32>> = (0..positions)
                        .map(|p| {
                            (0..cols)
                                .map(|i| {
                                    (((i * 7919 + p * 31) % 2003) as f32 / 977.0 - 1.0)
                                        * (1.0 + (i % 13) as f32)
                                })
                                .collect()
                        })
                        .collect();
                    let fast = quantize_input(encoding, &input, cols, compute).unwrap();
                    let rows: Vec<&[f32]> = input.iter().map(Vec::as_slice).collect();
                    let mut residual = vec![0.0f32; positions * cols];
                    quantize_rows(encoding, &rows, cols, compute, Some(&mut residual)).unwrap();
                    let both: Vec<&[f32]> = rows
                        .iter()
                        .copied()
                        .chain(residual.chunks_exact(cols))
                        .collect();
                    let slow = quantize_rows(encoding, &both, cols, compute, None).unwrap();
                    assert_eq!(fast.second_plane, Some(positions));
                    assert!(
                        fast.input.as_bytes() == slow.as_bytes(),
                        "{encoding:?} cols {cols} positions {positions}"
                    );
                }
            }
        }
    }
}
#[cfg(test)]
mod timing {
    use super::*;
    /// Diagnostic: cost of one-position activation quantization (run with
    /// --ignored --nocapture).
    #[test]
    #[ignore]
    fn math_wrapper_cost() {
        let x: Vec<f32> = (0..2816)
            .map(|i| ((i * 37) % 101) as f32 / 50.0 - 1.0)
            .collect();
        let s: Vec<f32> = (0..2816).map(|i| 0.5 + (i % 7) as f32 * 0.1).collect();
        let n = 20000;
        let time = |name: &str, f: &dyn Fn()| {
            for _ in 0..200 {
                f();
            }
            let start = std::time::Instant::now();
            for _ in 0..n {
                f();
            }
            eprintln!(
                "{name}: {:.2} us",
                start.elapsed().as_secs_f64() * 1e6 / n as f64
            );
        };
        time("rms_norm 2816", &|| {
            std::hint::black_box(math::rms_norm(&x, Some(&s), 1e-6).unwrap());
        });
        time("add 2816", &|| {
            std::hint::black_box(math::add(&x, &s).unwrap());
        });
        time("finite scan 2816", &|| {
            std::hint::black_box(x.iter().all(|v| v.is_finite()));
        });
    }
    #[test]
    #[ignore]
    fn decode_projection_bandwidth() {
        // Real model (POWER_LLM_TEST_MODEL): fused Q/K/V of every layer at one
        // position, cold across layers, on the engine pool.
        let Some(path) = std::env::var_os("POWER_LLM_TEST_MODEL") else {
            return;
        };
        let file = ModelFile::open(&path).unwrap();
        let model = crate::engine::model::ModelDefinition::bind(file.inventory()).unwrap();
        let kernels = crate::engine::execution::matrix_kernels(
            crate::engine::execution::MatrixBackend::Power9Int8x2,
        )
        .unwrap();
        let counts: Vec<usize> = std::env::var("PQ_BW_WORKERS")
            .map(|v| v.split(',').map(|x| x.parse().unwrap()).collect())
            .unwrap_or(vec![10, 20, 40]);
        for count in counts {
            let workers = Workers::current(count).unwrap();
            let compute = Compute {
                workers: Some(&workers),
                kernels,
            };
            let tensors: Vec<Vec<&TensorInfo>> = model
                .layers
                .iter()
                .map(|l| {
                    let mut t = vec![l.query];
                    t.extend(l.key);
                    t.extend(l.value);
                    t
                })
                .collect();
            let all: Vec<&TensorInfo> = tensors.iter().flatten().copied().collect();
            prepack(&file, &all, compute).unwrap();
            let x = vec![
                (0..2816)
                    .map(|i| ((i * 37) % 101) as f32 / 50.0 - 1.0)
                    .collect::<Vec<f32>>(),
            ];
            let bytes: u64 = all.iter().map(|t| t.size_bytes).sum();
            for _ in 0..2 {
                for t in &tensors {
                    project_many(&file, t, &x, compute).unwrap();
                }
            }
            let reps: usize = std::env::var("PQ_BW_REPS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10);
            let start = std::time::Instant::now();
            for _ in 0..reps {
                for t in &tensors {
                    std::hint::black_box(project_many(&file, t, &x, compute).unwrap());
                }
            }
            let secs = start.elapsed().as_secs_f64() / reps as f64;
            eprintln!(
                "{count} workers: Q/K/V of all layers {:.2} ms, {:.1} GB/s of GGUF bytes",
                secs * 1e3,
                bytes as f64 / secs / 1e9
            );
            // Overhead parts per layer call.
            let enc = Encoding::try_from(tensors[0][0].type_id).unwrap();
            let start = std::time::Instant::now();
            for _ in 0..reps {
                for _ in &tensors {
                    std::hint::black_box(quantize_input(enc, &x, 2816, compute.serial()).unwrap());
                }
            }
            let q = start.elapsed().as_secs_f64() / (reps * tensors.len()) as f64;
            let order: Vec<usize> = (0..2 * count).collect();
            let start = std::time::Instant::now();
            for _ in 0..reps {
                for _ in &tensors {
                    compute
                        .map_items(order.len(), &order, Ok::<_, Error>)
                        .unwrap();
                }
            }
            let s = start.elapsed().as_secs_f64() / (reps * tensors.len()) as f64;
            eprintln!(
                "  per layer: total {:.1} us, quantize {:.1} us, empty submission {:.1} us",
                secs * 1e6 / tensors.len() as f64,
                q * 1e6,
                s * 1e6
            );
        }
    }
    #[test]
    #[ignore]
    fn one_position_quantize_cost() {
        let Ok(kernels) = crate::engine::execution::matrix_kernels(
            crate::engine::execution::MatrixBackend::Power9Int8x2,
        ) else {
            return;
        };
        let compute = Compute {
            workers: None,
            kernels,
        };
        let x: Vec<Vec<f32>> = vec![
            (0..2816)
                .map(|i| ((i * 37) % 101) as f32 / 50.0 - 1.0)
                .collect(),
        ];
        for _ in 0..100 {
            quantize_input(Encoding::Q4_0, &x, 2816, compute).unwrap();
        }
        let n = 5000;
        let start = std::time::Instant::now();
        for _ in 0..n {
            std::hint::black_box(quantize_input(Encoding::Q4_0, &x, 2816, compute).unwrap());
        }
        eprintln!(
            "quantize_input, 1 position x 2816, two planes: {:.1} us",
            start.elapsed().as_secs_f64() * 1e6 / n as f64
        );
    }
}

#[cfg(test)]
mod expert_one_token_tests {
    use super::*;
    /// The two-submission one-token experts equal the general path bitwise
    /// (real model, POWER_LLM_TEST_MODEL; run with --ignored).
    #[test]
    #[ignore]
    fn one_token_experts_match_general_path() {
        let Some(path) = std::env::var_os("POWER_LLM_TEST_MODEL") else {
            return;
        };
        let Ok(kernels) = crate::engine::execution::matrix_kernels(
            crate::engine::execution::MatrixBackend::Power9Int8x2,
        ) else {
            return;
        };
        let file = ModelFile::open(&path).unwrap();
        let model = crate::engine::model::ModelDefinition::bind(file.inventory()).unwrap();
        let workers = Workers::current(Workers::available().unwrap().len().min(20)).unwrap();
        let compute = Compute {
            workers: Some(&workers),
            kernels,
        };
        let x = vec![
            (0..model.spec.hidden)
                .map(|i| ((i * 7919) % 2003) as f32 / 977.0 - 1.0)
                .collect::<Vec<f32>>(),
        ];
        for layer in model.layers.iter().filter(|l| l.moe.is_some()).take(3) {
            let groups: Vec<(usize, Vec<(usize, f32)>)> = [3usize, 17, 40, 41, 77, 90, 101, 127]
                .iter()
                .map(|&e| (e, vec![(0, 0.125)]))
                .collect();
            let fast = experts_chunked(&file, layer, &groups, &x, compute).unwrap();
            // SAFETY of the switch: test-only, single-threaded env change.
            unsafe { std::env::set_var("PQ_EXPERTS_OLD", "1") };
            let slow = experts_chunked(&file, layer, &groups, &x, compute).unwrap();
            unsafe { std::env::remove_var("PQ_EXPERTS_OLD") };
            for (a, b) in fast.iter().zip(&slow) {
                assert_eq!(a.len(), b.len());
                for (p, q) in a.iter().zip(b) {
                    assert!(p.iter().zip(q).all(|(u, v)| u.to_bits() == v.to_bits()));
                }
            }
        }
    }
}
