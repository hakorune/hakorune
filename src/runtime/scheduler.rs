//! Minimal scheduler abstraction (Phase 10.6b prep)
//!
//! Provides a pluggable interface to run tasks and yield cooperatively.

use crate::runtime::get_global_ring0;
use std::sync::atomic::{AtomicUsize, Ordering};

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
