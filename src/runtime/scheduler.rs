//! Minimal scheduler abstraction (Phase 10.6b prep)
//!
//! Provides a pluggable interface to run tasks and yield cooperatively.

use crate::runtime::get_global_ring0;
use crate::runtime::ring0::{Ring0Context, ThreadExit, ThreadHandle, ThreadSpawnError};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Sender};

pub trait Scheduler: Send + Sync {
    /// Spawn a task/closure. Default impl may run inline.
    fn spawn(&self, _name: &str, f: Box<dyn FnOnce() + Send + 'static>);
    /// Spawn a task after given delay milliseconds.
    fn spawn_after(&self, _delay_ms: u64, _name: &str, _f: Box<dyn FnOnce() + Send + 'static>) {}
    /// Poll scheduler: run due tasks and a limited number of queued tasks.
    fn poll(&self) {}
    /// Cooperative yield point (no-op for single-thread).
    fn yield_now(&self) {}

    /// Optional: spawn with a cancellation token. Default delegates to spawn.
    fn spawn_with_token(
        &self,
        name: &str,
        _token: CancellationToken,
        f: Box<dyn FnOnce() + Send + 'static>,
    ) {
        self.spawn(name, f)
    }
}

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Single-thread scheduler with a simple queue and delayed tasks.
pub struct SingleThreadScheduler {
    queue: Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send + 'static>>>>,
    delayed: Arc<Mutex<Vec<(Instant, Box<dyn FnOnce() + Send + 'static>)>>>,
    pending_hint: AtomicUsize,
    poll_budget: usize,
    trace_enabled: bool,
}

impl SingleThreadScheduler {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            delayed: Arc::new(Mutex::new(Vec::new())),
            pending_hint: AtomicUsize::new(0),
            // Capture env-derived knobs once to keep hot poll path getenv-free.
            poll_budget: crate::config::env::sched_poll_budget(),
            trace_enabled: crate::config::env::sched_trace_enabled(),
        }
    }
}

impl Scheduler for SingleThreadScheduler {
    fn spawn(&self, _name: &str, f: Box<dyn FnOnce() + Send + 'static>) {
        self.pending_hint.fetch_add(1, Ordering::Release);
        if let Ok(mut q) = self.queue.lock() {
            q.push_back(f);
        } else {
            // Keep hint conservative-correct on lock failure.
            self.pending_hint.fetch_sub(1, Ordering::AcqRel);
        }
    }
    fn spawn_after(&self, delay_ms: u64, _name: &str, f: Box<dyn FnOnce() + Send + 'static>) {
        let when = Instant::now() + Duration::from_millis(delay_ms);
        self.pending_hint.fetch_add(1, Ordering::Release);
        if let Ok(mut d) = self.delayed.lock() {
            d.push((when, f));
        } else {
            // Keep hint conservative-correct on lock failure.
            self.pending_hint.fetch_sub(1, Ordering::AcqRel);
        }
    }
    fn poll(&self) {
        // Fast path: no pending work observed.
        if self.pending_hint.load(Ordering::Acquire) == 0 {
            return;
        }

        // Move due delayed tasks to queue
        let now = Instant::now();
        let mut moved = 0usize;
        if let Ok(mut d) = self.delayed.lock() {
            let mut i = 0;
            while i < d.len() {
                if d[i].0 <= now {
                    let (_when, task) = d.remove(i);
                    if let Ok(mut q) = self.queue.lock() {
                        q.push_back(task);
                    }
                    moved += 1;
                } else {
                    i += 1;
                }
            }
        }
        // Run up to budget queued tasks
        let budget: usize = self.poll_budget;
        let mut ran = 0usize;
        while ran < budget {
            let task_opt = {
                if let Ok(mut q) = self.queue.lock() {
                    q.pop_front()
                } else {
                    None
                }
            };
            if let Some(task) = task_opt {
                task();
                let _ = self
                    .pending_hint
                    .fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| {
                        Some(n.saturating_sub(1))
                    });
                ran += 1;
            } else {
                break;
            }
        }
        if self.trace_enabled {
            get_global_ring0().log.debug(&format!(
                "[SCHED] poll moved={} ran={} budget={}",
                moved, ran, budget
            ));
        }
    }
}

type SchedulerTask = Box<dyn FnOnce() + Send + 'static>;

enum WorkerMessage {
    Task(SchedulerTask),
    Shutdown,
}

/// Worker-pool scheduler substrate.
///
/// This is an execution route implementation only. It is not wired into
/// `.hako` source-level `nowait` semantics by default.
pub struct WorkerPoolScheduler {
    tx: Mutex<Option<Sender<WorkerMessage>>>,
    handles: Mutex<Vec<ThreadHandle>>,
    ring0: Arc<Ring0Context>,
    pending_hint: Arc<AtomicUsize>,
}

impl WorkerPoolScheduler {
    pub fn new(worker_count: usize) -> Result<Self, ThreadSpawnError> {
        Self::with_ring0(
            worker_count,
            crate::runtime::ring0::ensure_global_ring0_initialized(),
        )
    }

    pub fn with_ring0(
        worker_count: usize,
        ring0: Arc<Ring0Context>,
    ) -> Result<Self, ThreadSpawnError> {
        let worker_count = worker_count.max(1);
        let (tx, rx) = mpsc::channel::<WorkerMessage>();
        let rx = Arc::new(Mutex::new(rx));
        let mut handles = Vec::with_capacity(worker_count);

        for idx in 0..worker_count {
            let rx = rx.clone();
            let handle = match ring0.thread.spawn(
                crate::runtime::ring0::ThreadSpawnSpec::named(format!("hako-worker-pool-{idx}")),
                Box::new(move || loop {
                    let message = {
                        let Ok(rx) = rx.lock() else {
                            return ThreadExit::Panic(
                                "worker pool receiver lock poisoned".to_string(),
                            );
                        };
                        rx.recv()
                    };
                    match message {
                        Ok(WorkerMessage::Task(task)) => task(),
                        Ok(WorkerMessage::Shutdown) | Err(_) => return ThreadExit::Ok,
                    }
                }),
            ) {
                Ok(handle) => handle,
                Err(err) => {
                    for _ in 0..handles.len() {
                        let _ = tx.send(WorkerMessage::Shutdown);
                    }
                    for handle in handles {
                        let _ = ring0.thread.join(handle);
                    }
                    return Err(err);
                }
            };
            handles.push(handle);
        }

        Ok(Self {
            tx: Mutex::new(Some(tx)),
            handles: Mutex::new(handles),
            ring0,
            pending_hint: Arc::new(AtomicUsize::new(0)),
        })
    }

    pub fn pending_hint(&self) -> usize {
        self.pending_hint.load(Ordering::Acquire)
    }

    fn send_message(&self, message: WorkerMessage) -> Result<(), WorkerMessage> {
        let Ok(tx) = self.tx.lock() else {
            return Err(message);
        };
        let Some(tx) = tx.as_ref() else {
            return Err(message);
        };
        tx.send(message).map_err(|err| err.0)
    }
}

impl Scheduler for WorkerPoolScheduler {
    fn spawn(&self, name: &str, f: SchedulerTask) {
        self.pending_hint.fetch_add(1, Ordering::Release);
        let pending = self.pending_hint.clone();
        let task = Box::new(move || {
            f();
            let _ = pending.fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| {
                Some(n.saturating_sub(1))
            });
        });
        if self.send_message(WorkerMessage::Task(task)).is_err() {
            let _ = self
                .pending_hint
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| {
                    Some(n.saturating_sub(1))
                });
            self.ring0.log.error(&format!(
                "WorkerPoolScheduler failed to enqueue task {name}"
            ));
        }
    }

    fn spawn_after(&self, delay_ms: u64, name: &str, f: SchedulerTask) {
        let name = name.to_string();
        self.spawn(
            &name,
            Box::new({
                let ring0 = self.ring0.clone();
                move || {
                    ring0.thread.sleep(Duration::from_millis(delay_ms));
                    f();
                }
            }),
        );
    }

    fn poll(&self) {}

    fn yield_now(&self) {
        self.ring0.thread.yield_now();
    }
}

impl Drop for WorkerPoolScheduler {
    fn drop(&mut self) {
        let worker_count = self
            .handles
            .lock()
            .map(|handles| handles.len())
            .unwrap_or(0);
        if let Ok(mut tx) = self.tx.lock() {
            if let Some(tx) = tx.take() {
                for _ in 0..worker_count {
                    let _ = tx.send(WorkerMessage::Shutdown);
                }
            }
        }
        if let Ok(mut handles) = self.handles.lock() {
            for handle in handles.drain(..) {
                let _ = self.ring0.thread.join(handle);
            }
        }
    }
}

use std::sync::atomic::AtomicBool;

/// Simple idempotent cancellation token for structured concurrency (skeleton)
#[derive(Clone, Debug)]
pub struct CancellationToken(Arc<AtomicBool>);

impl CancellationToken {
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }
    pub fn cancel(&self) {
        self.0.store(true, Ordering::SeqCst);
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    fn wait_for_no_pending(scheduler: &WorkerPoolScheduler) {
        let deadline = Instant::now() + Duration::from_secs(2);
        while scheduler.pending_hint() != 0 {
            assert!(
                Instant::now() < deadline,
                "worker pool pending hint did not drain"
            );
            scheduler.yield_now();
        }
    }

    #[test]
    fn worker_pool_scheduler_runs_task() {
        let scheduler = WorkerPoolScheduler::new(2).expect("worker pool should start");
        let (tx, rx) = mpsc::channel();

        scheduler.spawn(
            "worker-pool-test",
            Box::new(move || {
                tx.send(42).expect("test receiver should be alive");
            }),
        );

        assert_eq!(rx.recv_timeout(Duration::from_secs(2)), Ok(42));
        wait_for_no_pending(&scheduler);
    }

    #[test]
    fn worker_pool_scheduler_spawn_after_runs_task() {
        let scheduler = WorkerPoolScheduler::new(1).expect("worker pool should start");
        let (tx, rx) = mpsc::channel();

        scheduler.spawn_after(
            1,
            "worker-pool-delay-test",
            Box::new(move || {
                tx.send("done").expect("test receiver should be alive");
            }),
        );

        assert_eq!(rx.recv_timeout(Duration::from_secs(2)), Ok("done"));
        wait_for_no_pending(&scheduler);
    }
}
