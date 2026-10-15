//! Compute workers: a persistent pool of threads, each pinned once to one
//! CPU within the calling process's affinity. Work is submitted scope-style:
//! the caller blocks until every task has finished, so tasks may borrow.
//! No allocation/memory-node policy or HTTP scheduler here.
use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

/// Selected batched matrix kernels (portable C, or POWER9 explicit-F64-FMA,
/// bit-identical by the exact-product proof in docs/backends.md).
pub use crate::kernels::quantized::power9_fma::{
    FmaKernels as MatrixKernels, Request as MatrixBackend,
};

#[derive(Debug)]
pub enum Error {
    InvalidWorkers,
    /// A forced native matrix backend is not compiled in or not supported by this CPU/OS.
    MatrixBackendUnavailable,
    Unsupported,
    Affinity(std::io::Error),
    Spawn(std::io::Error),
    Panic,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

/// Resolve a matrix backend once per execution context (never per projection).
pub fn matrix_kernels(backend: MatrixBackend) -> Result<MatrixKernels, Error> {
    MatrixKernels::new(backend).map_err(|_| Error::MatrixBackendUnavailable)
}

/// Borrowed execution context of one batched projection: optional scoped
/// workers plus the already-resolved kernel pair.
#[derive(Debug, Clone, Copy)]
pub struct Compute<'a> {
    pub workers: Option<&'a Workers>,
    pub kernels: MatrixKernels,
}
impl<'a> Compute<'a> {
    /// The established portable path (the `*_with_workers` baselines).
    pub fn portable(workers: Option<&'a Workers>) -> Self {
        Self {
            workers,
            kernels: MatrixKernels::portable(),
        }
    }
    /// The same kernels without workers (for work already split by item).
    pub fn serial(self) -> Self {
        Self {
            workers: None,
            ..self
        }
    }
    /// `f(0..count)` in order. With workers, contiguous index ranges run
    /// concurrently; every item runs the identical computation either way.
    pub fn map_positions<T: Send, E: From<Error> + Send>(
        self,
        count: usize,
        f: impl Fn(usize) -> Result<T, E> + Sync,
    ) -> Result<Vec<T>, E> {
        match self.workers {
            Some(workers) if count > 1 => Ok(workers
                .ranges(count, |first, end| {
                    (first..end).map(&f).collect::<Result<Vec<_>, E>>()
                })?
                .into_iter()
                .flatten()
                .collect()),
            _ => (0..count).map(f).collect(),
        }
    }
    /// `f(i, a, b)` over element ranges of rows with lengths `lens`, returned
    /// concatenated per row. With fewer rows than workers (one-token decode),
    /// each row is also split into ranges of at least `min_chunk` elements; for
    /// elementwise or row-independent work the values do not depend on the
    /// split.
    pub fn map_split<E: From<Error> + Send>(
        self,
        lens: &[usize],
        min_chunk: usize,
        f: impl Fn(usize, usize, usize) -> Result<Vec<f32>, E> + Sync,
    ) -> Result<Vec<Vec<f32>>, E> {
        let workers = self.workers.map_or(1, Workers::count);
        if lens.len() >= workers {
            return self.map_positions(lens.len(), |i| f(i, 0, lens[i]));
        }
        let split = (workers / lens.len().max(1)).max(1);
        let mut items = Vec::new();
        for (i, &len) in lens.iter().enumerate() {
            let chunk = len.div_ceil(split).max(min_chunk.max(1));
            let mut a = 0;
            while a < len {
                let b = (a + chunk).min(len);
                items.push((i, a, b));
                a = b;
            }
        }
        let parts = self.map_positions(items.len(), |j| {
            let (i, a, b) = items[j];
            f(i, a, b)
        })?;
        let mut out: Vec<Vec<f32>> = lens.iter().map(|&n| Vec::with_capacity(n)).collect();
        for ((i, _, _), part) in items.into_iter().zip(parts) {
            out[i].extend(part);
        }
        Ok(out)
    }
    /// Run independent items on workers that each pull the next item from a
    /// shared queue (`order` gives the pull order, e.g. largest first).
    /// Results are returned indexed by item, independent of scheduling.
    pub fn map_items<T: Send, E: From<Error> + Send>(
        self,
        count: usize,
        order: &[usize],
        f: impl Fn(usize) -> Result<T, E> + Sync,
    ) -> Result<Vec<T>, E> {
        use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};
        let Some(workers) = self.workers.filter(|_| count > 1) else {
            return (0..count).map(f).collect();
        };
        if order.len() != count || order.iter().any(|&i| i >= count) {
            return Err(Error::InvalidWorkers.into());
        }
        let next = AtomicUsize::new(0);
        let failed = AtomicBool::new(false);
        let n = workers.count().min(count);
        let parts = workers.tasks((0..n).collect(), |_| {
            let mut done = Vec::new();
            while !failed.load(Relaxed) {
                let Some(&item) = order.get(next.fetch_add(1, Relaxed)) else {
                    break;
                };
                match f(item) {
                    Ok(value) => done.push((item, value)),
                    Err(e) => {
                        failed.store(true, Relaxed);
                        return Err(e);
                    }
                }
            }
            Ok(done)
        })?;
        let mut slots: Vec<Option<T>> = (0..count).map(|_| None).collect();
        for (item, value) in parts.into_iter().flatten() {
            slots[item] = Some(value);
        }
        slots
            .into_iter()
            .map(|s| s.ok_or_else(|| Error::InvalidWorkers.into()))
            .collect()
    }
}

/// Pinned worker threads (shared by clones; joined when the last is dropped).
#[derive(Clone)]
pub struct Workers {
    cpus: Vec<usize>,
    pool: std::sync::Arc<Pool>,
    /// Distinct physical cores among `cpus` (the first `cores` CPUs).
    cores: usize,
    /// Every allowed CPU of the packages `cpus` are on (see [`Self::confine`]).
    package_cpus: Vec<usize>,
    /// Physical core of each worker: (package, core id) from sysfs.
    core_keys: Vec<(String, String)>,
    /// Nonzero while one-token work runs one worker per core (see
    /// [`Workers::one_per_core`]); shared by clones.
    cap: std::sync::Arc<AtomicUsize>,
}
/// Lifts the one-worker-per-core cap when dropped.
pub struct CoreCap<'w>(&'w Workers);
impl Drop for CoreCap<'_> {
    fn drop(&mut self) {
        self.0.cap.store(0, Ordering::Relaxed);
        self.0.pool.shared.narrow.store(false, Ordering::Relaxed);
    }
}
impl fmt::Debug for Workers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Workers").field("cpus", &self.cpus).finish()
    }
}

/// A type-erased borrowed job; valid only while `Pool::run` is blocked.
#[derive(Clone, Copy)]
struct Job(*const (dyn Fn(usize) + Sync));
// SAFETY: the pointee is Sync and outlives every use (see Pool::run).
unsafe impl Send for Job {}

/// Dispatch without locks on the hot path: `run` publishes the job and an
/// epoch word `(counter << 16) | active` with one atomic store; workers spin
/// briefly on the epoch, then park. Completion is an atomic countdown; the
/// submitter likewise spins, then parks. Parking uses a mutex/condvar with a
/// SeqCst sleeper flag on each side, so a wakeup cannot be lost.
struct PoolShared {
    epoch: AtomicU64,
    /// Written only by the (single) submitter while no worker may read it:
    /// before the epoch is published and after `remaining` reaches 0.
    job: std::cell::UnsafeCell<Option<Job>>,
    remaining: AtomicUsize,
    shutdown: AtomicBool,
    /// Per worker: parked on `work`. A submission wakes parked workers only
    /// when it needs one of them, so workers outside a narrow submission (one
    /// per core for one-token steps) stay asleep instead of spinning on the
    /// SMT threads of busy cores.
    sleeping: Vec<AtomicBool>,
    /// True while one-token work runs one worker per core: workers a
    /// submission did not use park at once instead of spinning.
    narrow: AtomicBool,
    submitter_sleeping: AtomicBool,
    lock: std::sync::Mutex<()>,
    work: std::sync::Condvar,
    done: std::sync::Condvar,
}
// SAFETY: `job` follows the protocol above (writes happen-before the
// Release/SeqCst epoch publication that every reader Acquires, and a worker
// counted in `active` keeps the job alive until it decrements `remaining`);
// everything else is atomic or lock-protected.
unsafe impl Sync for PoolShared {}

const EPOCH_ACTIVE_BITS: u32 = 16;

/// How long idle threads spin before parking (`POWER_LLM_SPIN_US`, default 200).
fn spin_budget() -> std::time::Duration {
    static US: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    std::time::Duration::from_micros(*US.get_or_init(|| {
        std::env::var("POWER_LLM_SPIN_US")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(200)
    }))
}

/// Spin until `ready()` or the spin budget is spent; true if ready.
fn spin_until(ready: impl Fn() -> bool) -> bool {
    let start = std::time::Instant::now();
    let budget = spin_budget();
    loop {
        for _ in 0..64 {
            if ready() {
                return true;
            }
            std::hint::spin_loop();
        }
        if start.elapsed() >= budget {
            return ready();
        }
    }
}

struct Pool {
    shared: std::sync::Arc<PoolShared>,
    /// One submission at a time (concurrent callers queue here).
    submit: std::sync::Mutex<()>,
    threads: Vec<std::thread::JoinHandle<()>>,
}
thread_local! {
    /// True on pool threads: nested submissions run inline instead.
    static IN_POOL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// Which socket's weight replica this thread uses (see [`Split`]).
    static REPLICA: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}
/// The weight replica of the calling thread: 0, or 1 on the second socket's
/// workers and on the thread that drives them.
pub(crate) fn replica() -> usize {
    REPLICA.with(std::cell::Cell::get)
}
/// Run `f` with the calling thread on weight replica `replica`.
pub(crate) fn with_replica<R>(replica: usize, f: impl FnOnce() -> R) -> R {
    let previous = REPLICA.with(|r| r.replace(replica));
    let result = f();
    REPLICA.with(|r| r.set(previous));
    result
}
impl Pool {
    fn new(cpus: &[usize], replica: usize) -> Result<Self, Error> {
        if cpus.len() >= 1 << EPOCH_ACTIVE_BITS {
            return Err(Error::InvalidWorkers);
        }
        let shared = std::sync::Arc::new(PoolShared {
            epoch: AtomicU64::new(0),
            job: std::cell::UnsafeCell::new(None),
            remaining: AtomicUsize::new(0),
            shutdown: AtomicBool::new(false),
            sleeping: cpus.iter().map(|_| AtomicBool::new(false)).collect(),
            narrow: AtomicBool::new(false),
            submitter_sleeping: AtomicBool::new(false),
            lock: std::sync::Mutex::new(()),
            work: std::sync::Condvar::new(),
            done: std::sync::Condvar::new(),
        });
        let (ready, started) = std::sync::mpsc::channel();
        let mut pool = Self {
            shared: shared.clone(),
            submit: std::sync::Mutex::new(()),
            threads: Vec::new(),
        };
        for (index, &cpu) in cpus.iter().enumerate() {
            let shared = shared.clone();
            let ready = ready.clone();
            let handle = std::thread::Builder::new()
                .name(format!("power-matrix-{cpu}"))
                .spawn(move || {
                    let pinned = pin(cpu);
                    let ok = pinned.is_ok();
                    if ok {
                        set_prefetch_depth();
                    }
                    let _ = ready.send(pinned);
                    if ok {
                        IN_POOL.with(|flag| flag.set(true));
                        REPLICA.with(|r| r.set(replica));
                        worker_loop(index, &shared);
                    }
                })
                .map_err(Error::Spawn)?;
            pool.threads.push(handle);
        }
        drop(ready);
        for _ in cpus {
            started.recv().map_err(|_| Error::Panic)??;
        }
        Ok(pool)
    }
    /// Run `f(i)` for i in 0..n on the first n workers; returns after all
    /// calls have completed. `f` must not unwind (callers catch panics).
    fn run(&self, n: usize, f: &(dyn Fn(usize) + Sync)) {
        let _submission = self.submit.lock().unwrap_or_else(|e| e.into_inner());
        if n == 0 {
            return;
        }
        let s = &*self.shared;
        // SAFETY: lifetime erasure only; this function does not return until
        // `remaining` is 0, i.e. every worker has finished calling `f`, and
        // the job is cleared before the borrow of `f` ends.
        let job = Job(unsafe {
            std::mem::transmute::<*const (dyn Fn(usize) + Sync + '_), *const (dyn Fn(usize) + Sync)>(
                f,
            )
        });
        // SAFETY: no worker reads `job` now (previous submission complete,
        // next epoch not yet published; see PoolShared).
        unsafe { *s.job.get() = Some(job) };
        s.remaining.store(n, Ordering::Relaxed);
        let counter = (s.epoch.load(Ordering::Relaxed) >> EPOCH_ACTIVE_BITS) + 1;
        s.epoch
            .store((counter << EPOCH_ACTIVE_BITS) | n as u64, Ordering::SeqCst);
        if s.sleeping[..n].iter().any(|f| f.load(Ordering::SeqCst)) {
            let _guard = s.lock.lock().unwrap_or_else(|e| e.into_inner());
            s.work.notify_all();
        }
        if !spin_until(|| s.remaining.load(Ordering::Acquire) == 0) {
            let mut guard = s.lock.lock().unwrap_or_else(|e| e.into_inner());
            s.submitter_sleeping.store(true, Ordering::SeqCst);
            while s.remaining.load(Ordering::SeqCst) > 0 {
                guard = s.done.wait(guard).unwrap_or_else(|e| e.into_inner());
            }
            s.submitter_sleeping.store(false, Ordering::SeqCst);
        }
        // SAFETY: every worker counted in this epoch has finished with the job.
        unsafe { *s.job.get() = None };
    }
}
fn worker_loop(index: usize, s: &PoolShared) {
    let active_of = |epoch: u64| (epoch & ((1 << EPOCH_ACTIVE_BITS) - 1)) as usize;
    let mut seen = 0u64;
    // Spin only while recent submissions used this worker; otherwise park.
    let mut needed = true;
    loop {
        let wanted = |seen: u64| {
            let epoch = s.epoch.load(Ordering::SeqCst);
            (epoch != seen && index < active_of(epoch)) || s.shutdown.load(Ordering::SeqCst)
        };
        let spin = needed || !s.narrow.load(Ordering::Relaxed);
        if !(spin && spin_until(|| wanted(seen))) {
            let mut guard = s.lock.lock().unwrap_or_else(|e| e.into_inner());
            s.sleeping[index].store(true, Ordering::SeqCst);
            while !wanted(seen) {
                guard = s.work.wait(guard).unwrap_or_else(|e| e.into_inner());
            }
            s.sleeping[index].store(false, Ordering::SeqCst);
        }
        if s.shutdown.load(Ordering::Acquire) {
            return;
        }
        let epoch = s.epoch.load(Ordering::Acquire);
        seen = epoch;
        needed = index < active_of(epoch);
        if !needed {
            continue;
        }
        // SAFETY: this worker is counted in `epoch`, so the submitter keeps
        // the job published (and alive) until this worker decrements
        // `remaining`; the Acquire load above orders the job write before.
        let job =
            unsafe { (*s.job.get()).as_ref().map(|j| j.0) }.expect("job published with its epoch");
        // SAFETY: valid until this worker decrements `remaining` (Pool::run).
        unsafe { (*job)(index) };
        if s.remaining.fetch_sub(1, Ordering::SeqCst) == 1
            && s.submitter_sleeping.load(Ordering::SeqCst)
        {
            let _guard = s.lock.lock().unwrap_or_else(|e| e.into_inner());
            s.done.notify_all();
        }
    }
}
impl Drop for Pool {
    fn drop(&mut self) {
        self.shared.shutdown.store(true, Ordering::SeqCst);
        {
            let _guard = self.shared.lock.lock().unwrap_or_else(|e| e.into_inner());
            self.shared.work.notify_all();
        }
        for handle in self.threads.drain(..) {
            let _ = handle.join();
        }
    }
}
impl Workers {
    /// Select the first `count` CPUs of [`Self::available`]: distinct
    /// physical cores first, then SMT siblings. Constrain process affinity
    /// first (e.g. numactl) to reserve cores.
    /// No contiguous CPU/node IDs or SMT multiplier is assumed; no widening.
    pub fn current(count: usize) -> Result<Self, Error> {
        // Bounded by the inherited affinity mask (numactl/taskset), not a fixed cap.
        if count == 0 {
            return Err(Error::InvalidWorkers);
        }
        let mut cpus = Self::available()?;
        if count > cpus.len() {
            return Err(Error::InvalidWorkers);
        }
        cpus.truncate(count);
        Self::on(cpus, 0)
    }
    /// Workers on exactly `cpus`, using weight replica `replica`.
    pub(crate) fn on(cpus: Vec<usize>, replica: usize) -> Result<Self, Error> {
        let cores = distinct_cores(&cpus)?;
        let packages = cpus
            .iter()
            .map(|&cpu| package_of(cpu))
            .collect::<Result<Vec<_>, _>>()?;
        let mut package_cpus = Vec::new();
        for cpu in allowed()? {
            if packages.contains(&package_of(cpu)?) {
                package_cpus.push(cpu);
            }
        }
        let core_keys = cpus
            .iter()
            .map(|&cpu| core_key(cpu))
            .collect::<Result<Vec<_>, _>>()?;
        let pool = std::sync::Arc::new(Pool::new(&cpus, replica)?);
        Ok(Self {
            cpus,
            pool,
            cores,
            package_cpus,
            core_keys,
            cap: std::sync::Arc::new(AtomicUsize::new(0)),
        })
    }
    /// Restrict the calling thread to the CPUs of these workers' packages, so
    /// a thread that submits to them (and waits) runs next to them.
    pub fn confine(&self) -> Result<(), Error> {
        confine(&self.package_cpus)
    }
    /// The CPUs [`Self::current`] would select, as a main set of at most
    /// `per_core` threads per physical core and, when `count` exceeds that,
    /// the remaining (higher SMT) threads as a second set. Both use weight
    /// replica 0.
    pub fn tiered(count: usize, per_core: usize) -> Result<(Self, Option<Self>), Error> {
        if count == 0 {
            return Err(Error::InvalidWorkers);
        }
        let mut cpus = Self::available()?;
        if count > cpus.len() {
            return Err(Error::InvalidWorkers);
        }
        cpus.truncate(count);
        let main = (distinct_cores(&cpus)? * per_core.max(1)).min(cpus.len());
        let extra = cpus.split_off(main);
        let extra = (!extra.is_empty())
            .then(|| Self::on(extra, 0))
            .transpose()?;
        Ok((Self::on(cpus, 0)?, extra))
    }
    /// The CPUs [`Self::current`] would select, as one worker set per CPU
    /// package (socket), in package order; set `i` uses weight replica `i`.
    pub fn per_package(count: usize) -> Result<Vec<Self>, Error> {
        if count == 0 {
            return Err(Error::InvalidWorkers);
        }
        let mut cpus = Self::available()?;
        if count > cpus.len() {
            return Err(Error::InvalidWorkers);
        }
        cpus.truncate(count);
        let mut packages: Vec<(String, Vec<usize>)> = Vec::new();
        for cpu in cpus {
            let package = package_of(cpu)?;
            match packages.iter_mut().find(|(p, _)| *p == package) {
                Some((_, list)) => list.push(cpu),
                None => packages.push((package, vec![cpu])),
            }
        }
        packages.sort_by(|a, b| a.0.cmp(&b.0));
        packages
            .into_iter()
            .enumerate()
            .map(|(replica, (_, cpus))| Self::on(cpus, replica))
            .collect()
    }
    /// Until the guard drops, work is split over one worker per physical core
    /// (the first workers): one-token decode is faster without SMT siblings
    /// (26B: 0.123 s/token on 20 workers against 0.146 on 40), while prompts
    /// gain from them. Values never depend on the worker count.
    pub fn one_per_core(&self) -> CoreCap<'_> {
        self.cap.store(self.cores, Ordering::Relaxed);
        self.pool.shared.narrow.store(true, Ordering::Relaxed);
        CoreCap(self)
    }
    /// Allowed CPUs ordered for worker selection: one hardware thread per
    /// distinct physical core first (in CPU order), then each core's second
    /// thread, and so on. A count up to the number of cores therefore uses
    /// distinct cores; larger counts add SMT siblings evenly (two threads per
    /// core nearly double POWER9 integer-dot throughput).
    pub fn available() -> Result<Vec<usize>, Error> {
        let cores = core_threads()?;
        let depth = cores.iter().map(|(_, t)| t.len()).max().unwrap_or(0);
        Ok((0..depth)
            .flat_map(|round| cores.iter().filter_map(move |(_, t)| t.get(round).copied()))
            .collect())
    }
    /// Confine the calling (submitting) thread to the third hardware thread
    /// of each of these workers' cores ([`slice_order`]: the other thread on
    /// the second worker's execution half). One-token decode runs one worker
    /// per core, so there the submitter's serial work (norms, sampling,
    /// routing) has a half to itself instead of sharing one with a busy
    /// worker (26B decode 18.1 against 20.5 tok/s when left to the
    /// scheduler). Falls back to [`Self::confine`].
    pub fn place_submitter(&self) -> Result<(), Error> {
        let third: Vec<usize> = core_threads()?
            .into_iter()
            .filter(|(key, _)| self.core_keys.contains(key))
            .filter_map(|(_, t)| t.get(2).copied())
            .filter(|cpu| !self.cpus.contains(cpu))
            .collect();
        if third.is_empty() {
            self.confine()
        } else {
            confine(&third)
        }
    }
    /// Number of distinct physical cores in the affinity mask.
    pub fn physical_cores() -> Result<usize, Error> {
        distinct_cores(&Self::available()?)
    }
    pub fn cpus(&self) -> &[usize] {
        &self.cpus
    }
    pub fn count(&self) -> usize {
        match self.cap.load(Ordering::Relaxed) {
            0 => self.cpus.len(),
            cap => cap.min(self.cpus.len()),
        }
    }
    /// Run contiguous disjoint index ranges, returning results in input order.
    /// All workers join even on error; borrowed input cannot escape the scope.
    /// Main thread waits, and its affinity is never changed.
    pub(crate) fn ranges<
        T: Send,
        E: From<Error> + Send,
        F: Fn(usize, usize) -> Result<T, E> + Sync,
    >(
        &self,
        length: usize,
        operation: F,
    ) -> Result<Vec<T>, E> {
        if length == 0 {
            return Err(Error::InvalidWorkers.into());
        }
        let n = self.count().min(length);
        if n == 1 {
            return Ok(vec![operation(0, length)?]);
        }
        let base = length / n;
        let remainder = length % n;
        let ranges = (0..n)
            .map(|i| {
                let first = i * base + i.min(remainder);
                (first, first + base + usize::from(i < remainder))
            })
            .collect();
        self.tasks(ranges, |(first, end)| operation(first, end))
    }
    pub(crate) fn tasks<
        T: Send,
        R: Send,
        E: From<Error> + Send,
        F: Fn(T) -> Result<R, E> + Sync,
    >(
        &self,
        tasks: Vec<T>,
        operation: F,
    ) -> Result<Vec<R>, E> {
        if tasks.is_empty() || tasks.len() > self.count() {
            return Err(Error::InvalidWorkers.into());
        }
        if tasks.len() == 1 || IN_POOL.with(std::cell::Cell::get) {
            // One task, or a nested submission from a pool thread: inline.
            return tasks.into_iter().map(&operation).collect();
        }
        let n = tasks.len();
        let slots: Vec<std::sync::Mutex<Option<T>>> = tasks
            .into_iter()
            .map(|t| std::sync::Mutex::new(Some(t)))
            .collect();
        let results: Vec<std::sync::Mutex<Option<Result<R, E>>>> =
            (0..n).map(|_| std::sync::Mutex::new(None)).collect();
        let busy = std::sync::atomic::AtomicU64::new(0);
        let start = std::time::Instant::now();
        self.pool.run(n, &|i| {
            let began = std::time::Instant::now();
            let task = slots[i]
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .take()
                .expect("each task runs once");
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| operation(task)))
                .unwrap_or_else(|_| Err(Error::Panic.into()));
            *results[i].lock().unwrap_or_else(|e| e.into_inner()) = Some(result);
            busy.fetch_add(
                began.elapsed().as_nanos() as u64,
                std::sync::atomic::Ordering::Relaxed,
            );
        });
        crate::engine::stats::record_pool(n, start.elapsed(), busy.into_inner());
        results
            .into_iter()
            .map(|r| {
                r.into_inner()
                    .unwrap_or_else(|e| e.into_inner())
                    .expect("pool ran every task")
            })
            .collect()
    }
}

/// A second socket's workers for prompt chunks: positions are split between
/// the sockets (each on its own weight replica, packed by its own workers so
/// it lives in that socket's memory). Positions are independent, so values
/// never depend on the split; the share adapts to the sockets' measured speed.
pub struct Split {
    pub workers: Workers,
    /// Fraction of positions on the first socket, as f64 bits.
    share: AtomicU64,
}
impl Split {
    pub fn new(workers: Workers, first: usize) -> Self {
        let share = first as f64 / (first + workers.count()) as f64;
        Self {
            workers,
            share: AtomicU64::new(share.to_bits()),
        }
    }
    /// Positions of an `n`-position chunk for the first socket, or `None`
    /// when the chunk is too small to split.
    pub(crate) fn first_part(&self, n: usize) -> Option<usize> {
        const MIN: usize = 32;
        (n >= 2 * MIN).then(|| {
            let share = f64::from_bits(self.share.load(Ordering::Relaxed));
            ((n as f64 * share).round() as usize).clamp(MIN, n - MIN)
        })
    }
    /// Move the share towards equal finishing times (smoothed).
    pub(crate) fn observe(&self, first: (usize, f64), second: (usize, f64)) {
        if first.1 <= 0.0 || second.1 <= 0.0 {
            return;
        }
        let (a, b) = (first.0 as f64 / first.1, second.0 as f64 / second.1);
        let target = a / (a + b);
        let old = f64::from_bits(self.share.load(Ordering::Relaxed));
        let new = (0.8 * old + 0.2 * target).clamp(0.1, 0.9);
        self.share.store(new.to_bits(), Ordering::Relaxed);
    }
    pub fn share(&self) -> f64 {
        f64::from_bits(self.share.load(Ordering::Relaxed))
    }
}
impl fmt::Debug for Split {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Split")
            .field("workers", &self.workers)
            .finish()
    }
}

/// Allowed CPUs grouped by physical core (cores in CPU order), each core's
/// threads ordered by [`slice_order`].
/// A physical core's (package, core id) and its hardware threads.
type CoreThreads = ((String, String), Vec<usize>);
fn core_threads() -> Result<Vec<CoreThreads>, Error> {
    let mut cores: Vec<CoreThreads> = Vec::new();
    for cpu in allowed()? {
        let root = format!("/sys/devices/system/cpu/cpu{cpu}/topology");
        let package = std::fs::read_to_string(format!("{root}/physical_package_id"))
            .map_err(Error::Affinity)?;
        let core = std::fs::read_to_string(format!("{root}/core_id")).map_err(Error::Affinity)?;
        let key = (package.trim().to_owned(), core.trim().to_owned());
        match cores.iter_mut().find(|(k, _)| *k == key) {
            Some((_, threads)) => threads.push(cpu),
            None => cores.push((key, vec![cpu])),
        }
    }
    slice_order(&mut cores);
    Ok(cores)
}

/// Put a thread on the other execution half of each core second.
///
/// A POWER9 SMT4 core has two execution halves (superslices), two hardware
/// threads each. Two busy threads on one half run at about half speed (int8
/// kernel: 12.5 against 23 GMAC/s per thread). Which threads share a half
/// differs between cores and does not follow the CPU numbering (AC922
/// socket 0: CPUs 4c and 4c+1 share one on 6 of 20 cores, and no fixed
/// sibling offset avoids it), so a short probe (about 10 ms, once per CPU)
/// checks whether a core's first two threads slow each other down; if so,
/// the third thread moves to second place. With four threads, the third
/// place goes to the second thread's half-partner (one more probe where the
/// first two do not collide): order [a, b, partner of b, partner of a].
/// `POWER_LLM_SLICE_PROBE=0` keeps CPU order.
fn slice_order(cores: &mut [CoreThreads]) {
    use std::collections::BTreeMap;
    use std::sync::{Mutex, OnceLock};
    static SHARED: OnceLock<Mutex<BTreeMap<(usize, usize), bool>>> = OnceLock::new();
    if std::env::var("POWER_LLM_SLICE_PROBE").is_ok_and(|v| v == "0") {
        return;
    }
    let cache = SHARED.get_or_init(|| Mutex::new(BTreeMap::new()));
    let mut cache = cache.lock().unwrap_or_else(|e| e.into_inner());
    // Probe the pairs a selection needs, all cores at once.
    let measure =
        |cache: &mut BTreeMap<(usize, usize), bool>, pairs: Vec<(usize, usize)>| -> bool {
            let pairs: Vec<(usize, usize)> = pairs
                .into_iter()
                .filter(|p| !cache.contains_key(p))
                .collect();
            if pairs.is_empty() {
                return true;
            }
            let Some(shared) = probe_shared_halves(&pairs) else {
                return false;
            };
            for (pair, shared) in pairs.into_iter().zip(shared) {
                cache.insert(pair, shared);
            }
            true
        };
    let first: Vec<(usize, usize)> = cores
        .iter()
        .filter(|(_, t)| t.len() >= 3)
        .map(|(_, t)| (t[0], t[1]))
        .collect();
    if !measure(&mut cache, first) {
        return;
    }
    let second: Vec<(usize, usize)> = cores
        .iter()
        .filter(|(_, t)| t.len() >= 4 && cache.get(&(t[0], t[1])) == Some(&false))
        .map(|(_, t)| (t[1], t[2]))
        .collect();
    let have_second = measure(&mut cache, second);
    for (_, threads) in cores.iter_mut() {
        if threads.len() < 3 {
            continue;
        }
        let (a, b, c) = (threads[0], threads[1], threads[2]);
        if cache.get(&(a, b)) == Some(&true) {
            // Halves {a, b} and {c, d}: [a, c, d, b].
            threads.remove(1);
            threads.push(b);
        } else if have_second && threads.len() >= 4 && cache.get(&(b, c)) == Some(&false) {
            // b's partner is d: [a, b, d, c].
            threads.swap(2, 3);
        }
    }
}

/// For each (a, b): whether running the probe on a and b together takes
/// over 1.4 times as long as on a alone (best of three; all cores at once).
/// `None` without POWER9 kernels.
fn probe_shared_halves(pairs: &[(usize, usize)]) -> Option<Vec<bool>> {
    const ROUNDS: u64 = 200_000;
    if !crate::kernels::slice_probe(1) {
        return None;
    }
    let run = |cpus: Vec<usize>| -> Vec<f64> {
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(cpus.len()));
        let handles: Vec<_> = cpus
            .into_iter()
            .map(|cpu| {
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    let pinned = pin(cpu).is_ok();
                    barrier.wait();
                    let start = std::time::Instant::now();
                    crate::kernels::slice_probe(ROUNDS);
                    if pinned {
                        start.elapsed().as_secs_f64()
                    } else {
                        f64::NAN
                    }
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().unwrap_or(f64::NAN))
            .collect()
    };
    let mut alone = vec![f64::INFINITY; pairs.len()];
    let mut together = vec![f64::INFINITY; pairs.len()];
    for _ in 0..3 {
        let a = run(pairs.iter().map(|p| p.0).collect());
        let both = run(pairs.iter().flat_map(|p| [p.0, p.1]).collect());
        for i in 0..pairs.len() {
            alone[i] = alone[i].min(a[i]);
            together[i] = together[i].min(both[2 * i].max(both[2 * i + 1]));
        }
    }
    Some(
        alone
            .iter()
            .zip(&together)
            .map(|(a, t)| a.is_finite() && t.is_finite() && *t > 1.4 * a)
            .collect(),
    )
}

/// (package, core id) of a CPU: equal for the SMT threads of one core.
fn core_key(cpu: usize) -> Result<(String, String), Error> {
    let core =
        std::fs::read_to_string(format!("/sys/devices/system/cpu/cpu{cpu}/topology/core_id"))
            .map_err(Error::Affinity)?;
    Ok((package_of(cpu)?, core.trim().to_owned()))
}
/// Contiguous index ranges of `0..length` for the workers of `pools`, so that
/// all threads of one physical core get adjacent ranges (cores in order of
/// first appearance; each core's share is proportional to its threads).
/// Returns, per pool, one range per worker (in worker order).
pub(crate) fn core_local_ranges(pools: &[&Workers], length: usize) -> Vec<Vec<(usize, usize)>> {
    let mut cores: Vec<&(String, String)> = Vec::new();
    for pool in pools {
        for key in &pool.core_keys {
            if !cores.contains(&key) {
                cores.push(key);
            }
        }
    }
    // Threads in core order: (core, pool, worker).
    let mut threads = Vec::new();
    for (k, core) in cores.iter().enumerate() {
        for (p, pool) in pools.iter().enumerate() {
            for (w, key) in pool.core_keys.iter().enumerate() {
                if key == *core {
                    threads.push((k, p, w));
                }
            }
        }
    }
    let total = threads.len().max(1);
    let mut out: Vec<Vec<(usize, usize)>> =
        pools.iter().map(|p| vec![(0, 0); p.cpus.len()]).collect();
    for (i, &(_, p, w)) in threads.iter().enumerate() {
        out[p][w] = (i * length / total, (i + 1) * length / total);
    }
    out
}

fn package_of(cpu: usize) -> Result<String, Error> {
    std::fs::read_to_string(format!(
        "/sys/devices/system/cpu/cpu{cpu}/topology/physical_package_id"
    ))
    .map(|p| p.trim().to_owned())
    .map_err(Error::Affinity)
}

/// The CPUs this process may use: the calling thread's affinity at the first
/// call (numactl/taskset), not its current one, which
/// [`Workers::place_submitter`] narrows for the submitting thread.
fn allowed() -> Result<Vec<usize>, Error> {
    static INHERITED: std::sync::OnceLock<Vec<usize>> = std::sync::OnceLock::new();
    if let Some(cpus) = INHERITED.get() {
        return Ok(cpus.clone());
    }
    let cpus = thread_affinity()?;
    Ok(INHERITED.get_or_init(|| cpus).clone())
}
#[cfg(target_os = "linux")]
fn thread_affinity() -> Result<Vec<usize>, Error> {
    // CPU_SETSIZE is a checked upper bound, not a hard-coded machine CPU list.
    // Kernels with a larger mask reject with EINVAL instead of truncating it.
    let mut set: libc::cpu_set_t = unsafe { std::mem::zeroed() };
    // SAFETY: correctly sized initialized libc mask, get current calling thread.
    if unsafe { libc::sched_getaffinity(0, std::mem::size_of_val(&set), &mut set) } != 0 {
        return Err(Error::Affinity(std::io::Error::last_os_error()));
    }
    Ok((0..libc::CPU_SETSIZE as usize)
        .filter(|&cpu| unsafe { libc::CPU_ISSET(cpu, &set) })
        .collect())
}
/// Distinct (package, core) pairs among `cpus`.
fn distinct_cores(cpus: &[usize]) -> Result<usize, Error> {
    let mut seen = std::collections::BTreeSet::new();
    for &cpu in cpus {
        let root = format!("/sys/devices/system/cpu/cpu{cpu}/topology");
        let package = std::fs::read_to_string(format!("{root}/physical_package_id"))
            .map_err(Error::Affinity)?;
        let core = std::fs::read_to_string(format!("{root}/core_id")).map_err(Error::Affinity)?;
        seen.insert((package.trim().to_owned(), core.trim().to_owned()));
    }
    Ok(seen.len())
}

#[cfg(target_os = "linux")]
fn confine(cpus: &[usize]) -> Result<(), Error> {
    let allowed = allowed()?;
    // SAFETY: zeroed cpu_set_t is valid; CPU_SET bounds are checked below.
    let mut set: libc::cpu_set_t = unsafe { std::mem::zeroed() };
    for &cpu in cpus {
        if cpu >= libc::CPU_SETSIZE as usize || !allowed.contains(&cpu) {
            return Err(Error::InvalidWorkers);
        }
        unsafe { libc::CPU_SET(cpu, &mut set) };
    }
    // SAFETY: valid set for the calling thread (pid 0).
    if unsafe { libc::sched_setaffinity(0, std::mem::size_of_val(&set), &set) } != 0 {
        return Err(Error::Affinity(std::io::Error::last_os_error()));
    }
    Ok(())
}
#[cfg(not(target_os = "linux"))]
fn confine(_: &[usize]) -> Result<(), Error> {
    Err(Error::Unsupported)
}
/// Deepest hardware stream prefetch on the calling thread (DSCR depth 7).
/// One-token decode streams every weight once; the default depth leaves the
/// tile waiting for memory (the head projection reads 66 against 79 GB/s on 8
/// cores, 26B decode 58.8 against 56.5 ms per token); prompt processing is
/// compute bound and unchanged. `POWER_LLM_DSCR=<value>` sets another DSCR
/// value, `POWER_LLM_DSCR=default` keeps the system's. DSCR is per thread.
#[cfg(all(target_arch = "powerpc64", target_os = "linux"))]
fn set_prefetch_depth() {
    // AT_HWCAP2 bit: the DSCR register is available to user code.
    const PPC_FEATURE2_DSCR: libc::c_ulong = 0x2000_0000;
    static VALUE: std::sync::OnceLock<Option<u64>> = std::sync::OnceLock::new();
    let value = *VALUE.get_or_init(|| {
        // SAFETY: getauxval only reads the process's auxiliary vector.
        if unsafe { libc::getauxval(libc::AT_HWCAP2) } & PPC_FEATURE2_DSCR == 0 {
            return None;
        }
        match std::env::var("POWER_LLM_DSCR") {
            Ok(v) if v == "default" => None,
            Ok(v) => v.parse().ok(),
            Err(_) => Some(7),
        }
    });
    if let Some(value) = value {
        // SAFETY: SPR 3 (DSCR) is writable in problem state when the kernel
        // reports the feature; it only tunes the prefetch hardware.
        unsafe { core::arch::asm!("mtspr 3, {0}", in(reg) value, options(nomem, nostack)) };
    }
}
#[cfg(not(all(target_arch = "powerpc64", target_os = "linux")))]
fn set_prefetch_depth() {}
#[cfg(target_os = "linux")]
fn pin(cpu: usize) -> Result<(), Error> {
    if cpu >= libc::CPU_SETSIZE as usize || !allowed()?.contains(&cpu) {
        return Err(Error::InvalidWorkers);
    }
    let mut set: libc::cpu_set_t = unsafe { std::mem::zeroed() };
    // SAFETY: index checked; setting only the current worker's allowed CPU.
    unsafe { libc::CPU_SET(cpu, &mut set) };
    if unsafe { libc::sched_setaffinity(0, std::mem::size_of_val(&set), &set) } != 0 {
        return Err(Error::Affinity(std::io::Error::last_os_error()));
    }
    Ok(())
}
#[cfg(not(target_os = "linux"))]
fn thread_affinity() -> Result<Vec<usize>, Error> {
    Err(Error::Unsupported)
}
#[cfg(not(target_os = "linux"))]
fn pin(_: usize) -> Result<(), Error> {
    Err(Error::Unsupported)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn disjoint_ranges_pin_workers_without_changing_parent() {
        let before = thread_affinity().unwrap();
        let n = Workers::available().unwrap().len().min(4);
        let workers = Workers::current(n).unwrap();
        let pieces = workers
            .ranges(11, |first, end| {
                let affinity = thread_affinity()?;
                if n > 1 {
                    assert_eq!(affinity.len(), 1);
                    assert!(workers.cpus().contains(&affinity[0]));
                }
                Ok::<_, Error>((first..end).collect::<Vec<_>>())
            })
            .unwrap();
        assert_eq!(pieces.concat(), (0..11).collect::<Vec<_>>());
        assert_eq!(thread_affinity().unwrap(), before);
        assert!(Workers::current(0).is_err());
        assert!(Workers::current(Workers::available().unwrap().len() + 1).is_err());
        assert!(workers.ranges::<(), Error, _>(0, |_, _| Ok(())).is_err());
    }
    /// Cost of one submission (diagnostic): run with --ignored --nocapture.
    #[test]
    #[ignore]
    fn submission_cost() {
        let workers = Workers::current(Workers::available().unwrap().len().min(40)).unwrap();
        for capped in [false, true] {
            let _cap = capped.then(|| workers.one_per_core());
            let n = workers.count();
            for _ in 0..1000 {
                workers.ranges(n, |a, _| Ok::<_, Error>(a)).unwrap();
            }
            let start = std::time::Instant::now();
            let rounds = 20000;
            for _ in 0..rounds {
                workers.ranges(n, |a, _| Ok::<_, Error>(a)).unwrap();
            }
            eprintln!(
                "{n} workers (capped {capped}): {:.1} us per empty submission",
                start.elapsed().as_secs_f64() * 1e6 / rounds as f64
            );
        }
    }
    #[test]
    fn error_and_panic_join_all_scoped_workers() {
        let workers = Workers::current(Workers::available().unwrap().len().min(4)).unwrap();
        assert!(
            workers
                .ranges::<(), Error, _>(8, |_, _| Err(Error::InvalidWorkers))
                .is_err()
        );
        if workers.count() > 1 {
            assert!(matches!(
                workers.ranges::<(), Error, _>(8, |_, _| panic!("injected worker panic")),
                Err(Error::Panic)
            ));
        }
    }
}
