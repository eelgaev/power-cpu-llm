//! Process-wide wall-clock totals per engine stage, for diagnostics
//! (`power-cpu-llm profile-decode`). A [`Span`] costs two `Instant::now` calls and
//! one relaxed atomic add; stages are entered from the calling thread, so
//! nested parallel work is attributed to its enclosing stage.

use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    AttentionProjections,
    AttentionCore,
    AttentionOutput,
    SharedMlp,
    Router,
    Experts,
    ExpertCombine,
    SoftEmbedding,
    Logits,
    /// Mamba-2 input and output projections (Nemotron).
    MambaProjections,
    /// Mamba-2 convolution, scan and gated norm (Nemotron).
    MambaScan,
    /// Nested diagnostics (overlap the stages above): main-thread time in
    /// dense-projection activation quantization and in its worker phase.
    ProjectionQuantize,
    ProjectionWorkers,
    /// Weight packing on first use (summed over threads, so it can exceed
    /// wall time).
    ProjectionPack,
    /// One-token routed experts: the gate/up and the down submissions.
    ExpertsUp,
    ExpertsDown,
}
const STAGES: [(Stage, &str); 16] = [
    (
        Stage::AttentionProjections,
        "attention q/k/v projections + rope",
    ),
    (Stage::AttentionCore, "attention scores/softmax/values"),
    (
        Stage::AttentionOutput,
        "attention output projection + residual",
    ),
    (Stage::SharedMlp, "shared mlp (+ norms)"),
    (Stage::Router, "router"),
    (Stage::Experts, "routed experts"),
    (Stage::ExpertCombine, "expert accumulation + combine"),
    (Stage::SoftEmbedding, "self-conditioning soft embedding"),
    (Stage::Logits, "output head"),
    (Stage::MambaProjections, "mamba in/out projections"),
    (Stage::MambaScan, "mamba conv/scan/gated norm"),
    (
        Stage::ProjectionQuantize,
        "(nested) projection quantization",
    ),
    (Stage::ProjectionWorkers, "(nested) projection worker phase"),
    (
        Stage::ProjectionPack,
        "(nested, thread-summed) weight packing",
    ),
    (Stage::ExpertsUp, "(nested) one-token experts gate/up"),
    (Stage::ExpertsDown, "(nested) one-token experts down"),
];
static TOTALS: [AtomicU64; 16] = [const { AtomicU64::new(0) }; 16];

/// Adds its lifetime to `stage` on drop; while it lives, pool submissions
/// from this thread are attributed to it (outermost non-nested stage).
pub struct Span(Stage, Instant, Option<usize>);
impl Span {
    pub fn new(stage: Stage) -> Self {
        let previous = if (stage as usize) < NESTED {
            CURRENT.with(|c| c.replace(Some(stage as usize)))
        } else {
            None
        };
        Self(stage, Instant::now(), previous)
    }
}
impl Drop for Span {
    fn drop(&mut self) {
        TOTALS[self.0 as usize].fetch_add(self.1.elapsed().as_nanos() as u64, Relaxed);
        if (self.0 as usize) < NESTED {
            CURRENT.with(|c| c.set(self.2));
        }
    }
}
/// Stages before this index are top-level (not nested diagnostics).
const NESTED: usize = Stage::ProjectionQuantize as usize;
thread_local! {
    static CURRENT: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
}
/// Per top-level stage (index NESTED: outside every stage): submissions,
/// pool wall nanoseconds and summed task nanoseconds.
static STAGE_POOL: [[AtomicU64; 3]; NESTED + 1] =
    [const { [const { AtomicU64::new(0) }; 3] }; NESTED + 1];
/// (stage, submissions, pool wall s, task s) since the previous call.
pub fn take_stage_pool() -> Vec<(&'static str, u64, f64, f64)> {
    (0..=NESTED)
        .map(|i| {
            let name = STAGES
                .get(i)
                .filter(|_| i < NESTED)
                .map_or("(outside stages)", |s| s.1);
            let p = &STAGE_POOL[i];
            (
                name,
                p[0].swap(0, Relaxed),
                p[1].swap(0, Relaxed) as f64 * 1e-9,
                p[2].swap(0, Relaxed) as f64 * 1e-9,
            )
        })
        .collect()
}

/// (stage description, seconds) since the previous call, resetting totals.
pub fn take() -> Vec<(&'static str, f64)> {
    STAGES
        .iter()
        .map(|&(stage, name)| (name, TOTALS[stage as usize].swap(0, Relaxed) as f64 * 1e-9))
        .collect()
}

/// Worker-pool utilisation since the previous call (resetting): submissions,
/// wall seconds, wall x tasks (capacity, task-seconds) and the summed task
/// busy seconds. busy / capacity is the fraction of submitted worker time
/// spent working (the rest is imbalance and wake-up latency).
pub fn take_pool() -> (u64, f64, f64, f64) {
    (
        POOL[0].swap(0, Relaxed),
        POOL[1].swap(0, Relaxed) as f64 * 1e-9,
        POOL[2].swap(0, Relaxed) as f64 * 1e-9,
        POOL[3].swap(0, Relaxed) as f64 * 1e-9,
    )
}
static POOL: [AtomicU64; 4] = [const { AtomicU64::new(0) }; 4];
pub(crate) fn record_pool(tasks: usize, wall: std::time::Duration, busy_nanos: u64) {
    let stage = CURRENT.with(std::cell::Cell::get).unwrap_or(NESTED);
    let p = &STAGE_POOL[stage];
    p[0].fetch_add(1, Relaxed);
    p[1].fetch_add(wall.as_nanos() as u64, Relaxed);
    p[2].fetch_add(busy_nanos, Relaxed);
    POOL[0].fetch_add(1, Relaxed);
    POOL[1].fetch_add(wall.as_nanos() as u64, Relaxed);
    POOL[2].fetch_add(wall.as_nanos() as u64 * tasks as u64, Relaxed);
    POOL[3].fetch_add(busy_nanos, Relaxed);
}
