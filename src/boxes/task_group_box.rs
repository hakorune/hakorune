use crate::box_trait::{BoolBox, BoxBase, BoxCore, NyashBox, StringBox, VoidBox};
use std::any::Any;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub(crate) struct TaskGroupInner {
    pub strong: Mutex<Vec<crate::boxes::future::FutureBox>>,
}

/// Phase-0 runtime scaffold behind structured `task_scope` ownership.
///
/// Current responsibility is intentionally narrow:
/// - own child futures registered under the active task scope
/// - expose best-effort `cancelAll()` / `joinAll(timeout_ms)` hooks
/// - avoid defining detached/failure-aggregation semantics yet
#[derive(Debug, Clone)]
pub struct TaskGroupBox {
    base: BoxBase,
    // Skeleton: cancellation token owned by this group (future wiring)
    cancelled: bool,
    pub(crate) inner: Arc<TaskGroupInner>,
}

impl TaskGroupBox {
    pub fn new() -> Self {
        Self {
            base: BoxBase::new(),
            cancelled: false,
            inner: Arc::new(TaskGroupInner {
                strong: Mutex::new(Vec::new()),
            }),
        }
    }
    pub fn cancel_all(&mut self) {
        self.cancelled = true;
        self.cancel_owned_futures("scope-cancelled");
    }
    /// Cancel all child tasks owned by this task-scope scaffold and return void.
    pub fn cancelAll(&mut self) -> Box<dyn NyashBox> {
        self.cancel_all();
        Box::new(VoidBox::new())
    }
    /// Best-effort bounded join for child futures owned by this task-scope scaffold.
    pub fn joinAll(&self, timeout_ms: Option<i64>) -> Box<dyn NyashBox> {
        let ms = timeout_ms.unwrap_or(2000).max(0) as u64;
        self.join_all_inner(ms);
        Box::new(VoidBox::new())
    }
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    /// Register a Future into this group's ownership
    pub fn add_future(&self, fut: &crate::boxes::future::FutureBox) {
        if let Ok(mut v) = self.inner.strong.lock() {
            v.push(fut.clone());
        }
    }

    fn cancel_owned_futures(&self, reason: &str) {
        if let Ok(list) = self.inner.strong.lock() {
            for fut in list.iter() {
                if !fut.ready() {
                    fut.cancel_with_reason(reason);
                }
            }
        }
    }

    fn join_all_inner(&self, timeout_ms: u64) {
        use std::time::{Duration, Instant};
        let deadline = Instant::now() + Duration::from_millis(timeout_ms);
        loop {
            let mut all_ready = true;
            if let Ok(mut list) = self.inner.strong.lock() {
                list.retain(|f| !f.ready());
                if !list.is_empty() {
                    all_ready = false;
                }
            }
            if all_ready {
                break;
            }
            if Instant::now() >= deadline {
                break;
            }
            crate::runtime::global_hooks::safepoint_and_poll();
            std::thread::yield_now();
        }
    }
}

impl BoxCore for TaskGroupBox {
    fn box_id(&self) -> u64 {
        self.base.id
    }
    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        None
    }
    fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TaskGroup(cancelled={})", self.cancelled)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NyashBox for TaskGroupBox {
    fn to_string_box(&self) -> StringBox {
        StringBox::new(format!("TaskGroup(cancelled={})", self.cancelled))
    }
    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        if let Some(g) = other.as_any().downcast_ref::<TaskGroupBox>() {
            BoolBox::new(self.base.id == g.base.id)
        } else {
            BoolBox::new(false)
        }
    }
    fn clone_box(&self) -> Box<dyn NyashBox> {
        Box::new(self.clone())
    }
    fn share_box(&self) -> Box<dyn NyashBox> {
        self.clone_box()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancel_all_marks_group_cancelled() {
        let mut group = TaskGroupBox::new();
        assert!(!group.is_cancelled());

        let out = group.cancelAll();

        assert!(group.is_cancelled());
        assert_eq!(out.to_string_box().value, "void");
    }

    #[test]
    fn join_all_on_empty_group_returns_void() {
        let group = TaskGroupBox::new();

        let out = group.joinAll(Some(0));

        assert_eq!(out.to_string_box().value, "void");
    }

    #[test]
    fn cancel_all_cancels_owned_pending_future() {
        let mut group = TaskGroupBox::new();
        let fut = crate::boxes::future::FutureBox::new();
        group.add_future(&fut);

        group.cancelAll();

        assert_eq!(
            fut.to_string_box().value,
            "Future(cancelled: Cancelled: scope-cancelled)"
        );
    }
}
