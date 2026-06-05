//! Minimal scheduler abstraction (Phase 10.6b prep)
//!
//! Provides a pluggable interface to run tasks and yield cooperatively.

use crate::runtime::get_global_ring0;
use crate::runtime::ring0::{Ring0Context, ThreadExit, ThreadHandle, ThreadSpawnError};
use crate::runtime::thread_registry::{global_thread_registry, ThreadRegistry, ThreadRegistryRole};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};

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

enum DelayMessage {
    Task {
        when: Instant,
        name: String,
        task: SchedulerTask,
    },
    Shutdown,
}

struct WorkerThreadRegistration {
    registry: Arc<ThreadRegistry>,
    host_thread_id: crate::runtime::ring0::HostThreadId,
}

impl Drop for WorkerThreadRegistration {
    fn drop(&mut self) {
        let _ = self.registry.unregister_current_thread(self.host_thread_id);
    }
}

struct PendingHintGuard {
    pending: Arc<AtomicUsize>,
}

impl Drop for PendingHintGuard {
    fn drop(&mut self) {
        let _ = self
            .pending
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| {
                Some(n.saturating_sub(1))
            });
    }
}

/// Worker-pool scheduler substrate.
///
/// This is an execution route implementation only. It is not wired into
/// `.hako` source-level `nowait` semantics by default.
pub struct WorkerPoolScheduler {
    tx: Mutex<Option<Sender<WorkerMessage>>>,
    delay_tx: Mutex<Option<Sender<DelayMessage>>>,
    handles: Mutex<Vec<ThreadHandle>>,
    timer_handle: Mutex<Option<ThreadHandle>>,
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
        let pending_hint = Arc::new(AtomicUsize::new(0));

        for idx in 0..worker_count {
            let rx = rx.clone();
            let ring0_for_worker = ring0.clone();
            let worker_name = format!("hako-worker-pool-{idx}");
            let handle = match ring0.thread.spawn(
                crate::runtime::ring0::ThreadSpawnSpec::named(worker_name.clone()),
                Box::new(move || {
                    let registry = global_thread_registry();
                    let host_thread_id = ring0_for_worker.thread.current_thread_id();
                    if registry
                        .register_current_thread(
                            host_thread_id,
                            ThreadRegistryRole::RuntimeWorker,
                            Some(worker_name),
                        )
                        .is_err()
                    {
                        return ThreadExit::Panic(
                            "worker pool thread registry register failed".to_string(),
                        );
                    }
                    let _registration = WorkerThreadRegistration {
                        registry,
                        host_thread_id,
                    };

                    let exit = loop {
                        let message = {
                            let Ok(rx) = rx.lock() else {
                                break ThreadExit::Panic(
                                    "worker pool receiver lock poisoned".to_string(),
                                );
                            };
                            rx.recv()
                        };
                        match message {
                            Ok(WorkerMessage::Task(task)) => task(),
                            Ok(WorkerMessage::Shutdown) | Err(_) => break ThreadExit::Ok,
                        }
                    };
                    exit
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

        let (delay_tx, delay_rx) = mpsc::channel::<DelayMessage>();
        let timer_worker_tx = tx.clone();
        let timer_pending = pending_hint.clone();
        let ring0_for_timer = ring0.clone();
        let timer_handle = match ring0.thread.spawn(
            crate::runtime::ring0::ThreadSpawnSpec::named("hako-worker-delay-timer"),
            Box::new(move || {
                WorkerPoolScheduler::run_delay_timer(
                    delay_rx,
                    timer_worker_tx,
                    timer_pending,
                    ring0_for_timer,
                )
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

        Ok(Self {
            tx: Mutex::new(Some(tx)),
            delay_tx: Mutex::new(Some(delay_tx)),
            handles: Mutex::new(handles),
            timer_handle: Mutex::new(Some(timer_handle)),
            ring0,
            pending_hint,
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

    fn delay_sender(&self) -> Option<Sender<DelayMessage>> {
        let tx = self.delay_tx.lock().ok()?;
        tx.as_ref().cloned()
    }

    fn wrap_counted_task(&self, f: SchedulerTask) -> SchedulerTask {
        let pending = self.pending_hint.clone();
        Box::new(move || {
            let _pending_guard = PendingHintGuard { pending };
            f();
        })
    }

    fn decrement_pending_hint(&self) {
        Self::decrement_pending_hint_for(&self.pending_hint);
    }

    fn decrement_pending_hint_for(pending_hint: &AtomicUsize) {
        let _ = pending_hint.fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| {
            Some(n.saturating_sub(1))
        });
    }

    fn enqueue_counted_task(&self, name: &str, task: SchedulerTask) {
        if self.send_message(WorkerMessage::Task(task)).is_err() {
            self.decrement_pending_hint();
            self.ring0.log.error(&format!(
                "WorkerPoolScheduler failed to enqueue task {name}"
            ));
        }
    }

    fn drain_due_delayed_tasks(
        delayed: &mut Vec<(Instant, String, SchedulerTask)>,
        tx: &Sender<WorkerMessage>,
        pending_hint: &AtomicUsize,
        ring0: &Ring0Context,
    ) {
        let now = Instant::now();
        let mut index = 0usize;
        while index < delayed.len() {
            if delayed[index].0 <= now {
                let (_when, name, task) = delayed.remove(index);
                if tx.send(WorkerMessage::Task(task)).is_err() {
                    Self::decrement_pending_hint_for(pending_hint);
                    ring0.log.error(&format!(
                        "WorkerPoolScheduler failed to enqueue delayed task {name}"
                    ));
                }
            } else {
                index += 1;
            }
        }
    }

    fn next_delay_timeout(delayed: &[(Instant, String, SchedulerTask)]) -> Option<Duration> {
        let next_when = delayed.iter().map(|(when, _, _)| *when).min()?;
        Some(next_when.saturating_duration_since(Instant::now()))
    }

    fn cancel_delayed_tasks(
        delayed: &mut Vec<(Instant, String, SchedulerTask)>,
        pending_hint: &AtomicUsize,
    ) {
        for _ in delayed.drain(..) {
            Self::decrement_pending_hint_for(pending_hint);
        }
    }

    fn run_delay_timer(
        rx: Receiver<DelayMessage>,
        tx: Sender<WorkerMessage>,
        pending_hint: Arc<AtomicUsize>,
        ring0: Arc<Ring0Context>,
    ) -> ThreadExit {
        let mut delayed: Vec<(Instant, String, SchedulerTask)> = Vec::new();
        loop {
            Self::drain_due_delayed_tasks(&mut delayed, &tx, &pending_hint, &ring0);
            let message = match Self::next_delay_timeout(&delayed) {
                Some(timeout) => match rx.recv_timeout(timeout) {
                    Ok(message) => Some(message),
                    Err(RecvTimeoutError::Timeout) => None,
                    Err(RecvTimeoutError::Disconnected) => {
                        Self::cancel_delayed_tasks(&mut delayed, &pending_hint);
                        return ThreadExit::Ok;
                    }
                },
                None => match rx.recv() {
                    Ok(message) => Some(message),
                    Err(_) => {
                        Self::cancel_delayed_tasks(&mut delayed, &pending_hint);
                        return ThreadExit::Ok;
                    }
                },
            };

            match message {
                Some(DelayMessage::Task { when, name, task }) => {
                    delayed.push((when, name, task));
                }
                Some(DelayMessage::Shutdown) => {
                    Self::cancel_delayed_tasks(&mut delayed, &pending_hint);
                    return ThreadExit::Ok;
                }
                None => {}
            }
        }
    }
}

impl Scheduler for WorkerPoolScheduler {
    fn spawn(&self, name: &str, f: SchedulerTask) {
        self.pending_hint.fetch_add(1, Ordering::Release);
        let task = self.wrap_counted_task(f);
        self.enqueue_counted_task(name, task);
    }

    fn spawn_after(&self, delay_ms: u64, name: &str, f: SchedulerTask) {
        self.pending_hint.fetch_add(1, Ordering::Release);
        let task = self.wrap_counted_task(f);

        let Some(tx) = self.delay_sender() else {
            self.decrement_pending_hint();
            self.ring0.log.error(&format!(
                "WorkerPoolScheduler failed to schedule delayed task {name}"
            ));
            return;
        };

        let when = Instant::now() + Duration::from_millis(delay_ms);
        if tx
            .send(DelayMessage::Task {
                when,
                name: name.to_string(),
                task,
            })
            .is_err()
        {
            self.decrement_pending_hint();
            self.ring0.log.error(&format!(
                "WorkerPoolScheduler failed to schedule delayed task {name}"
            ));
        }
    }

    fn yield_now(&self) {
        self.ring0.thread.yield_now();
    }
}

impl Drop for WorkerPoolScheduler {
    fn drop(&mut self) {
        if let Ok(mut delay_tx) = self.delay_tx.lock() {
            if let Some(delay_tx) = delay_tx.take() {
                let _ = delay_tx.send(DelayMessage::Shutdown);
            }
        }
        if let Ok(mut timer_handle) = self.timer_handle.lock() {
            if let Some(handle) = timer_handle.take() {
                let _ = self.ring0.thread.join(handle);
            }
        }

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
    use crate::runtime::thread_registry::{
        global_thread_registry, reset_global_thread_registry_for_tests, ThreadRegistryRole,
    };
    use std::sync::{mpsc, Mutex};
    use std::time::Duration;

    static WORKER_REGISTRY_TEST_GUARD: Mutex<()> = Mutex::new(());

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
        let _guard = WORKER_REGISTRY_TEST_GUARD.lock().unwrap();
        reset_global_thread_registry_for_tests();
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
    fn worker_pool_scheduler_spawn_after_runs_task_without_external_poll() {
        let _guard = WORKER_REGISTRY_TEST_GUARD.lock().unwrap();
        reset_global_thread_registry_for_tests();
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

    #[test]
    fn worker_pool_scheduler_spawn_after_handles_many_tasks_without_external_poll() {
        let _guard = WORKER_REGISTRY_TEST_GUARD.lock().unwrap();
        reset_global_thread_registry_for_tests();
        let scheduler = WorkerPoolScheduler::new(2).expect("worker pool should start");
        let (tx, rx) = mpsc::channel();

        for value in 0..32 {
            let tx = tx.clone();
            scheduler.spawn_after(
                1,
                "worker-pool-many-delay-test",
                Box::new(move || {
                    tx.send(value).expect("test receiver should be alive");
                }),
            );
        }
        drop(tx);

        let mut values = Vec::new();
        for _ in 0..32 {
            values.push(
                rx.recv_timeout(Duration::from_secs(2))
                    .expect("delayed task should run"),
            );
        }
        values.sort_unstable();
        assert_eq!(values, (0..32).collect::<Vec<_>>());
        wait_for_no_pending(&scheduler);
    }

    #[test]
    fn worker_pool_scheduler_registers_and_unregisters_workers() {
        let _guard = WORKER_REGISTRY_TEST_GUARD.lock().unwrap();
        reset_global_thread_registry_for_tests();
        let registry = global_thread_registry();

        {
            let scheduler = WorkerPoolScheduler::new(2).expect("worker pool should start");
            let deadline = Instant::now() + Duration::from_secs(2);
            loop {
                let snapshot = registry.snapshot().expect("snapshot should succeed");
                if snapshot.len() == 2 {
                    assert!(
                        snapshot
                            .registrations
                            .iter()
                            .all(|registration| registration.role
                                == ThreadRegistryRole::RuntimeWorker)
                    );
                    break;
                }
                assert!(
                    Instant::now() < deadline,
                    "worker pool threads did not register"
                );
                scheduler.yield_now();
            }
        }

        assert!(registry
            .snapshot()
            .expect("snapshot should succeed")
            .is_empty());
    }

    #[test]
    fn worker_pool_scheduler_unregisters_worker_after_task_panic() {
        let _guard = WORKER_REGISTRY_TEST_GUARD.lock().unwrap();
        reset_global_thread_registry_for_tests();
        let registry = global_thread_registry();
        let scheduler = WorkerPoolScheduler::new(1).expect("worker pool should start");

        let deadline = Instant::now() + Duration::from_secs(2);
        while registry.snapshot().expect("snapshot should succeed").len() != 1 {
            assert!(
                Instant::now() < deadline,
                "worker pool thread did not register"
            );
            scheduler.yield_now();
        }

        scheduler.spawn(
            "worker-pool-panic-test",
            Box::new(move || {
                panic!("worker pool panic cleanup test");
            }),
        );

        let deadline = Instant::now() + Duration::from_secs(2);
        while !registry
            .snapshot()
            .expect("snapshot should succeed")
            .is_empty()
        {
            assert!(
                Instant::now() < deadline,
                "worker pool thread did not unregister after panic"
            );
            scheduler.yield_now();
        }
        wait_for_no_pending(&scheduler);
    }
}
