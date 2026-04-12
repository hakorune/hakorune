//! Lightweight global hooks for JIT/extern to reach GC/scheduler without owning NyashRuntime.

use once_cell::sync::OnceCell;
use std::sync::{
    atomic::{AtomicBool, AtomicU8, Ordering},
    Arc, RwLock,
};

use super::scheduler::CancellationToken;
use super::{gc::BarrierKind, gc::GcHooks, scheduler::Scheduler};

const SAFEPOINT_FLAG_GC: u8 = 1 << 0;
const SAFEPOINT_FLAG_POLL: u8 = 1 << 1;
static SAFEPOINT_FAST_FLAGS: AtomicU8 = AtomicU8::new(0);
static GC_ALLOC_FAST_ENABLED: AtomicBool = AtomicBool::new(false);

// Unified global runtime hooks state (single lock for consistency)
struct GlobalHooksState {
    gc: Option<Arc<dyn GcHooks>>,
    sched: Option<Arc<dyn Scheduler>>,
    // When false, safepoint bridge skips scheduler polling.
    // Controlled by explicit env policy (NYASH_SCHED_POLL_IN_SAFEPOINT).
    poll_in_safepoint: bool,
    cur_token: Option<CancellationToken>,
    futures: Vec<crate::boxes::future::FutureWeak>,
    strong: Vec<crate::boxes::future::FutureBox>,
    root_cancel_reason: Option<String>,
    scope_depth: usize,
    group_stack: Vec<std::sync::Arc<crate::boxes::task_group_box::TaskGroupInner>>,
}

impl GlobalHooksState {
    fn new() -> Self {
        Self {
            gc: None,
            sched: None,
            poll_in_safepoint: true,
            cur_token: None,
            futures: Vec::new(),
            strong: Vec::new(),
            root_cancel_reason: None,
            scope_depth: 0,
            group_stack: Vec::new(),
        }
    }
}

static GLOBAL_STATE: OnceCell<RwLock<GlobalHooksState>> = OnceCell::new();

fn state() -> &'static RwLock<GlobalHooksState> {
    GLOBAL_STATE.get_or_init(|| RwLock::new(GlobalHooksState::new()))
}

#[inline(always)]
fn gc_runtime_active(gc: &Arc<dyn GcHooks>) -> bool {
    gc.is_active()
}

#[inline(always)]
fn gc_safepoint_enabled(gc: &Arc<dyn GcHooks>) -> bool {
    gc_runtime_active(gc)
}

#[inline(always)]
fn recompute_safepoint_flags(st: &GlobalHooksState) -> u8 {
    let mut flags = 0u8;
    if st.gc.as_ref().is_some_and(gc_safepoint_enabled) {
        flags |= SAFEPOINT_FLAG_GC;
    }
    if st.poll_in_safepoint && st.sched.is_some() {
        flags |= SAFEPOINT_FLAG_POLL;
    }
    flags
}

#[inline(always)]
fn publish_runtime_fast_flags(st: &GlobalHooksState) {
    SAFEPOINT_FAST_FLAGS.store(recompute_safepoint_flags(st), Ordering::Relaxed);
    let gc_alloc_enabled = st.gc.as_ref().is_some_and(gc_runtime_active);
    GC_ALLOC_FAST_ENABLED.store(gc_alloc_enabled, Ordering::Relaxed);
}

pub fn set_from_runtime(rt: &crate::runtime::nyash_runtime::NyashRuntime) {
    if let Ok(mut st) = state().write() {
        st.gc = Some(rt.gc.clone());
        st.sched = rt.scheduler.as_ref().cloned();
        st.poll_in_safepoint = crate::config::env::sched_poll_in_safepoint();
        if st.cur_token.is_none() {
            st.cur_token = Some(CancellationToken::new());
        }
        st.futures.clear();
        st.strong.clear();
        st.root_cancel_reason = None;
        st.scope_depth = 0;
        st.group_stack.clear();
        publish_runtime_fast_flags(&st);
    }
}

pub fn set_gc(gc: Arc<dyn GcHooks>) {
    if let Ok(mut st) = state().write() {
        st.gc = Some(gc);
        publish_runtime_fast_flags(&st);
    }
}
pub fn set_scheduler(s: Arc<dyn Scheduler>) {
    if let Ok(mut st) = state().write() {
        st.sched = Some(s);
        publish_runtime_fast_flags(&st);
    }
}
/// Set the current task group's cancellation token (scaffold).
pub fn set_current_group_token(tok: CancellationToken) {
    if let Ok(mut st) = state().write() {
        st.cur_token = Some(tok);
    }
}

/// Get the current task group's cancellation token (no-op default).
pub fn current_group_token() -> CancellationToken {
    if let Ok(st) = state().read() {
        if let Some(t) = st.cur_token.as_ref() {
            return t.clone();
        }
    }
    CancellationToken::new()
}

/// Cancel the current structured task scope and mark owned pending futures as cancelled.
///
/// If no explicit `task_scope` is active, this falls back to the implicit root
/// scope that owns top-level futures registered through `register_future_to_current_group`.
pub fn cancel_current_group_with_reason(reason: &str) {
    if let Ok(mut st) = state().write() {
        if let Some(tok) = st.cur_token.as_ref() {
            tok.cancel();
        }
        if let Some(inner) = st.group_stack.last() {
            inner.cancel_pending_with_reason(reason);
            return;
        }
        if st.root_cancel_reason.is_none() {
            st.root_cancel_reason = Some(reason.to_string());
        }
        for fut in st.strong.iter() {
            if !fut.ready() {
                fut.cancel_with_reason(reason);
            }
        }
    }
}

/// Register a Future into the current scope registry.
///
/// Policy:
/// - if an explicit `task_scope` is active, the future belongs to that scope
/// - otherwise it falls back to the implicit root scope
/// - this path does not imply detached-task semantics
pub fn register_future_to_current_group(fut: &crate::boxes::future::FutureBox) {
    if let Ok(mut st) = state().write() {
        // Prefer explicit current TaskGroup at top of stack
        if let Some(inner) = st.group_stack.last() {
            inner.register_future(fut, inner);
            return;
        }
        if let Some(reason) = st.root_cancel_reason.clone() {
            fut.cancel_with_reason(reason);
            return;
        }
        // Fallback to implicit global group
        st.futures.push(fut.downgrade());
        st.strong.push(fut.clone());
    }
}

/// Join all currently registered futures with a coarse timeout guard.
pub fn join_all_registered_futures(timeout_ms: u64) {
    use std::time::{Duration, Instant};
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    loop {
        let mut all_ready = true;
        // purge + readiness check under single state lock (short critical sections)
        if let Ok(mut st) = state().write() {
            st.futures.retain(|fw| fw.is_ready().is_some());
            st.strong.retain(|f| !f.ready());
            for fw in st.futures.iter() {
                if let Some(ready) = fw.is_ready() {
                    if !ready {
                        all_ready = false;
                        break;
                    }
                }
            }
        }
        if all_ready {
            break;
        }
        if Instant::now() >= deadline {
            break;
        }
        safepoint_and_poll();
        std::thread::yield_now();
    }
    // Final sweep
    if let Ok(mut st) = state().write() {
        st.strong.retain(|f| !f.ready());
        st.futures.retain(|fw| matches!(fw.is_ready(), Some(false)));
    }
}

/// Push the current structured `task_scope` scaffold.
///
/// This is the Phase-0 ownership boundary for child futures only; it does not
/// imply detached-task semantics or a finalized failure contract.
pub fn push_task_scope() {
    if let Ok(mut st) = state().write() {
        st.scope_depth += 1;
        // Push a new explicit TaskGroup for this scope
        st.group_stack.push(std::sync::Arc::new(
            crate::boxes::task_group_box::TaskGroupInner::default(),
        ));
    }
    // Set a fresh cancellation token for this scope (best-effort)
    set_current_group_token(CancellationToken::new());
}

#[cfg(test)]
pub(crate) fn reset_for_tests() {
    if let Ok(mut st) = state().write() {
        *st = GlobalHooksState::new();
        publish_runtime_fast_flags(&st);
    }
}

/// Pop the current structured `task_scope` scaffold.
///
/// When depth reaches 0, perform a best-effort bounded join for the futures
/// that were registered under this scope.
pub fn pop_task_scope() {
    let mut do_join = false;
    let mut popped: Option<std::sync::Arc<crate::boxes::task_group_box::TaskGroupInner>> = None;
    if let Ok(mut st) = state().write() {
        if st.scope_depth > 0 {
            st.scope_depth -= 1;
        }
        if st.scope_depth == 0 {
            do_join = true;
        }
        // Pop explicit group for this scope
        popped = st.group_stack.pop();
    }
    if do_join {
        let ms: u64 = std::env::var("NYASH_TASK_SCOPE_JOIN_MS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1000);
        if let Some(inner) = popped {
            // Join this group's outstanding futures
            let deadline = std::time::Instant::now() + std::time::Duration::from_millis(ms);
            loop {
                let mut all_ready = true;
                if let Ok(mut list) = inner.strong.lock() {
                    list.retain(|f| !f.ready());
                    if !list.is_empty() {
                        all_ready = false;
                    }
                }
                if all_ready {
                    break;
                }
                if std::time::Instant::now() >= deadline {
                    break;
                }
                safepoint_and_poll();
                std::thread::yield_now();
            }
        } else {
            // Fallback to implicit global group
            join_all_registered_futures(ms);
        }
    }
    // Reset token (best-effort)
    set_current_group_token(CancellationToken::new());
}

/// Perform a runtime safepoint and poll the scheduler if available.
pub fn safepoint_and_poll() {
    let flags = SAFEPOINT_FAST_FLAGS.load(Ordering::Relaxed);
    if flags == 0 {
        return;
    }
    if let Ok(st) = state().read() {
        if (flags & SAFEPOINT_FLAG_GC) != 0 {
            if let Some(gc) = st.gc.as_ref() {
                gc.safepoint();
            }
        }
        if (flags & SAFEPOINT_FLAG_POLL) != 0 {
            if let Some(sched) = st.sched.as_ref() {
                sched.poll();
            }
        }
    }
}

/// Try to schedule a task on the global scheduler. Returns true if scheduled.
pub fn spawn_task(name: &str, f: Box<dyn FnOnce() + Send + 'static>) -> bool {
    // If a scheduler is registered, enqueue the task; otherwise run inline.
    if let Ok(st) = state().read() {
        if let Some(sched) = st.sched.as_ref() {
            sched.spawn(name, f);
            return true;
        }
    }
    // Fallback inline execution
    f();
    false
}

/// Spawn a task bound to a cancellation token when available (skeleton).
pub fn spawn_task_with_token(
    name: &str,
    token: crate::runtime::scheduler::CancellationToken,
    f: Box<dyn FnOnce() + Send + 'static>,
) -> bool {
    if let Ok(st) = state().read() {
        if let Some(sched) = st.sched.as_ref() {
            sched.spawn_with_token(name, token, f);
            return true;
        }
    }
    f();
    false
}

/// Spawn a delayed task via scheduler if available; returns true if scheduled.
pub fn spawn_task_after(delay_ms: u64, name: &str, f: Box<dyn FnOnce() + Send + 'static>) -> bool {
    if let Ok(st) = state().read() {
        if let Some(sched) = st.sched.as_ref() {
            sched.spawn_after(delay_ms, name, f);
            return true;
        }
    }
    // Fallback: run inline after blocking sleep
    // Phase 90-D: thread 系移行
    let ring0 = crate::runtime::ring0::get_global_ring0();
    let ring0_clone = ring0.clone();
    std::thread::spawn(move || {
        ring0_clone
            .thread
            .sleep(std::time::Duration::from_millis(delay_ms));
        f();
    });
    false
}

/// Forward a GC barrier event to the currently registered GC hooks (if any).
pub fn gc_barrier(kind: BarrierKind) {
    if let Ok(st) = state().read() {
        if let Some(gc) = st.gc.as_ref() {
            gc.barrier(kind);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_trait::NyashBox;
    use crate::boxes::basic::ErrorBox;
    use std::sync::Mutex as TestMutex;

    static TEST_GUARD: TestMutex<()> = TestMutex::new(());

    #[test]
    fn cancel_current_group_marks_registered_future_cancelled() {
        let _guard = TEST_GUARD.lock().unwrap();
        reset_for_tests();
        push_task_scope();
        let fut = crate::boxes::future::FutureBox::new();
        register_future_to_current_group(&fut);

        cancel_current_group_with_reason("scope-cancelled");

        assert_eq!(
            fut.to_string_box().value,
            "Future(cancelled: Cancelled: scope-cancelled)"
        );
        pop_task_scope();
        reset_for_tests();
    }

    #[test]
    fn late_registration_into_cancelled_explicit_scope_is_immediately_cancelled() {
        let _guard = TEST_GUARD.lock().unwrap();
        reset_for_tests();
        push_task_scope();

        cancel_current_group_with_reason("scope-cancelled");
        let late = crate::boxes::future::FutureBox::new();
        register_future_to_current_group(&late);

        assert_eq!(
            late.to_string_box().value,
            "Future(cancelled: Cancelled: scope-cancelled)"
        );
        pop_task_scope();
        reset_for_tests();
    }

    #[test]
    fn late_registration_after_explicit_sibling_failure_is_immediately_cancelled() {
        let _guard = TEST_GUARD.lock().unwrap();
        reset_for_tests();
        push_task_scope();
        let failed = crate::boxes::future::FutureBox::new();
        register_future_to_current_group(&failed);
        failed.set_failed(Box::new(ErrorBox::new("TaskError", "boom")));
        let late = crate::boxes::future::FutureBox::new();

        register_future_to_current_group(&late);

        assert_eq!(
            late.to_string_box().value,
            "Future(cancelled: Cancelled: sibling-failed)"
        );
        pop_task_scope();
        reset_for_tests();
    }

    #[test]
    fn cancel_current_group_marks_implicit_root_future_cancelled() {
        let _guard = TEST_GUARD.lock().unwrap();
        reset_for_tests();
        let fut = crate::boxes::future::FutureBox::new();
        register_future_to_current_group(&fut);

        cancel_current_group_with_reason("scope-cancelled");

        assert_eq!(
            fut.to_string_box().value,
            "Future(cancelled: Cancelled: scope-cancelled)"
        );
        reset_for_tests();
    }

    #[test]
    fn late_registration_into_cancelled_implicit_root_is_immediately_cancelled() {
        let _guard = TEST_GUARD.lock().unwrap();
        reset_for_tests();
        cancel_current_group_with_reason("scope-cancelled");
        let late = crate::boxes::future::FutureBox::new();

        register_future_to_current_group(&late);

        assert_eq!(
            late.to_string_box().value,
            "Future(cancelled: Cancelled: scope-cancelled)"
        );
        reset_for_tests();
    }
}
/// Report an allocation to the current GC hooks (best-effort)
pub fn gc_alloc(bytes: u64) {
    if !GC_ALLOC_FAST_ENABLED.load(Ordering::Relaxed) {
        return;
    }
    if let Ok(st) = state().read() {
        if let Some(gc) = st.gc.as_ref() {
            gc.alloc(bytes);
        }
    }
}
