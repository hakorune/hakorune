use crate::box_trait::{BoolBox, BoxBase, BoxCore, NyashBox, StringBox, VoidBox};
use crate::boxes::array::ArrayBox;
use crate::boxes::future::FutureTerminal;
use crate::boxes::result::ResultBox;
use std::any::Any;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

#[derive(Debug)]
pub(crate) struct TaskGroupInner {
    pub strong: Mutex<Vec<crate::boxes::future::FutureBox>>,
    pub first_failure: Mutex<Option<Box<dyn NyashBox>>>,
    pub additional_failures: Mutex<Vec<Box<dyn NyashBox>>>,
    pub closed_reason: Mutex<Option<String>>,
    pub sibling_failure_seen: AtomicBool,
}

impl Default for TaskGroupInner {
    fn default() -> Self {
        Self {
            strong: Mutex::new(Vec::new()),
            first_failure: Mutex::new(None),
            additional_failures: Mutex::new(Vec::new()),
            closed_reason: Mutex::new(None),
            sibling_failure_seen: AtomicBool::new(false),
        }
    }
}

impl TaskGroupInner {
    pub(crate) fn closed_reason(&self) -> Option<String> {
        self.closed_reason.lock().ok().and_then(|slot| slot.clone())
    }

    fn effective_closed_reason(&self, reason: &str) -> String {
        if let Ok(mut slot) = self.closed_reason.lock() {
            let effective = slot.clone().unwrap_or_else(|| {
                let reason = reason.to_string();
                *slot = Some(reason.clone());
                reason
            });
            return effective;
        }
        reason.to_string()
    }

    pub(crate) fn bind_future(&self, fut: &crate::boxes::future::FutureBox, owner: &Arc<Self>) {
        fut.bind_sibling_failure_scope(owner);
    }

    pub(crate) fn register_future(&self, fut: &crate::boxes::future::FutureBox, owner: &Arc<Self>) {
        if let Some(reason) = self.closed_reason() {
            if reason == "sibling-failed" {
                if let Some(FutureTerminal::Failed(error)) = fut.terminal_snapshot() {
                    self.note_failure_and_cancel_siblings(error);
                    return;
                }
            }
            fut.cancel_with_reason(reason);
            return;
        }
        if let Some(FutureTerminal::Failed(error)) = fut.terminal_snapshot() {
            self.note_failure_and_cancel_siblings(error);
            return;
        }
        if let Ok(mut v) = self.strong.lock() {
            v.push(fut.clone());
        }
        self.bind_future(fut, owner);
    }

    pub(crate) fn cancel_pending_with_reason(&self, reason: &str) {
        let effective_reason = self.effective_closed_reason(reason);
        if let Ok(list) = self.strong.lock() {
            for fut in list.iter() {
                if !fut.ready() {
                    fut.cancel_with_reason(effective_reason.clone());
                }
            }
        }
    }

    pub(crate) fn join_pending_with_timeout(&self, timeout_ms: u64) {
        use std::time::{Duration, Instant};
        let deadline = Instant::now() + Duration::from_millis(timeout_ms);
        loop {
            let mut all_ready = true;
            if let Ok(mut list) = self.strong.lock() {
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

    pub(crate) fn first_failure(&self) -> Option<Box<dyn NyashBox>> {
        self.first_failure
            .lock()
            .ok()
            .and_then(|slot| slot.as_ref().map(|failure| failure.clone_or_share()))
    }

    pub(crate) fn join_outcome(&self, timeout_ms: u64) -> Result<(), Box<dyn NyashBox>> {
        self.join_pending_with_timeout(timeout_ms);
        if let Some(failure) = self.first_failure() {
            return Err(failure);
        }
        Ok(())
    }

    pub(crate) fn failure_report(&self) -> ArrayBox {
        let report = ArrayBox::new();
        if let Some(failure) = self.first_failure() {
            let _ = report.push(failure);
        }
        if let Ok(failures) = self.additional_failures.lock() {
            for failure in failures.iter() {
                let _ = report.push(failure.clone_or_share());
            }
        }
        report
    }

    pub(crate) fn scope_exit_shutdown(&self, timeout_ms: u64) -> Result<(), Box<dyn NyashBox>> {
        self.cancel_pending_with_reason("scope-exit-cancelled");
        self.join_outcome(timeout_ms)
    }

    pub(crate) fn note_failure_and_cancel_siblings(&self, failure: Box<dyn NyashBox>) {
        if self.sibling_failure_seen.swap(true, Ordering::SeqCst) {
            if let Ok(mut failures) = self.additional_failures.lock() {
                failures.push(failure);
            }
            return;
        }
        if let Ok(mut slot) = self.first_failure.lock() {
            *slot = Some(failure);
        }
        self.cancel_pending_with_reason("sibling-failed");
    }
}

/// Phase-0 runtime scaffold behind structured `task_scope` ownership.
///
/// Current responsibility is intentionally narrow:
/// - own child futures registered under the active task scope
/// - expose best-effort `cancelAll()` / `joinAll(timeout_ms)` hooks
/// - apply the current `first failure cancels siblings` rule inside explicit scope ownership
/// - stay separate from the implicit root scope used outside explicit `task_scope`
/// - avoid defining detached/failure-aggregation semantics yet
#[derive(Debug, Clone)]
pub struct TaskGroupBox {
    base: BoxBase,
    pub(crate) inner: Arc<TaskGroupInner>,
}

impl TaskGroupBox {
    pub fn new() -> Self {
        Self {
            base: BoxBase::new(),
            inner: Arc::new(TaskGroupInner::default()),
        }
    }
    pub fn cancel_all(&mut self) {
        self.cancel_owned_futures("scope-cancelled");
    }
    /// Cancel all child tasks owned by this task-scope scaffold and return void.
    pub fn cancelAll(&mut self) -> Box<dyn NyashBox> {
        self.cancel_all();
        Box::new(VoidBox::new())
    }
    /// Best-effort bounded join for child futures owned by this task-scope scaffold.
    ///
    /// Current Phase-0 surface:
    /// - `Ok(void)` when no first failure is latched
    /// - `Err(first_failure)` when explicit child failure was latched
    /// - timeout does not yet have its own public error shape
    pub fn joinAll(&self, timeout_ms: Option<i64>) -> Box<dyn NyashBox> {
        let ms = timeout_ms.unwrap_or(2000).max(0) as u64;
        match self.inner.join_outcome(ms) {
            Ok(()) => Box::new(ResultBox::new_ok(Box::new(VoidBox::new()))),
            Err(error) => Box::new(ResultBox::new_err(error)),
        }
    }
    /// Return the currently latched failure report for this structured task-scope owner.
    ///
    /// Current Phase-0 shape:
    /// - `[]` when no child failure has been observed
    /// - `[first_failure, additional_failures...]` in observation order
    /// - sibling cancellations are not appended as aggregate failures
    pub fn failureReport(&self) -> Box<dyn NyashBox> {
        Box::new(self.inner.failure_report())
    }
    pub fn is_cancelled(&self) -> bool {
        self.inner.closed_reason().is_some()
    }

    /// Register a Future into this group's ownership
    pub fn add_future(&self, fut: &crate::boxes::future::FutureBox) {
        self.inner.register_future(fut, &self.inner);
    }

    fn cancel_owned_futures(&self, reason: &str) {
        self.inner.cancel_pending_with_reason(reason);
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
        write!(f, "TaskGroup(cancelled={})", self.is_cancelled())
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
        StringBox::new(format!("TaskGroup(cancelled={})", self.is_cancelled()))
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
    use crate::boxes::basic::ErrorBox;
    use crate::boxes::result::ResultBox;

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
        let result = out
            .as_any()
            .downcast_ref::<ResultBox>()
            .expect("joinAll must return ResultBox");
        assert!(result.is_ok_bool());
        assert_eq!(result.get_value().to_string_box().value, "void");
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

    #[test]
    fn first_failed_future_cancels_pending_siblings() {
        let group = TaskGroupBox::new();
        let first = crate::boxes::future::FutureBox::new();
        let sibling = crate::boxes::future::FutureBox::new();
        group.add_future(&first);
        group.add_future(&sibling);

        first.set_failed(Box::new(StringBox::new("boom")));

        assert_eq!(
            sibling.to_string_box().value,
            "Future(cancelled: Cancelled: sibling-failed)"
        );
        assert_eq!(
            group
                .inner
                .first_failure()
                .expect("first failure must be stored")
                .to_string_box()
                .value,
            "boom"
        );
    }

    #[test]
    fn join_all_returns_err_first_failure_payload() {
        let group = TaskGroupBox::new();
        let failed = crate::boxes::future::FutureBox::new();
        group.add_future(&failed);
        failed.set_failed(Box::new(ErrorBox::new("TaskError", "boom")));

        let out = group.joinAll(Some(0));
        let result = out
            .as_any()
            .downcast_ref::<ResultBox>()
            .expect("joinAll must return ResultBox");

        assert!(result.is_err());
        assert_eq!(result.get_error().to_string_box().value, "TaskError: boom");
    }

    #[test]
    fn failure_report_is_empty_without_failed_children() {
        let group = TaskGroupBox::new();

        let out = group.failureReport();
        let report = out
            .as_any()
            .downcast_ref::<ArrayBox>()
            .expect("failureReport must return ArrayBox");

        assert_eq!(report.len(), 0);
    }

    #[test]
    fn failure_report_keeps_first_and_additional_failures_in_order() {
        let group = TaskGroupBox::new();
        let first = crate::boxes::future::FutureBox::new();
        group.add_future(&first);
        first.set_failed(Box::new(ErrorBox::new("TaskError", "boom-1")));

        let second = crate::boxes::future::FutureBox::new();
        second.set_failed(Box::new(ErrorBox::new("TaskError", "boom-2")));
        group.add_future(&second);

        let out = group.failureReport();
        let report = out
            .as_any()
            .downcast_ref::<ArrayBox>()
            .expect("failureReport must return ArrayBox");

        assert_eq!(report.len(), 2);
        assert_eq!(
            report.get_index_i64(0).to_string_box().value,
            "TaskError: boom-1"
        );
        assert_eq!(
            report.get_index_i64(1).to_string_box().value,
            "TaskError: boom-2"
        );
    }

    #[test]
    fn add_future_after_cancel_all_is_immediately_cancelled() {
        let mut group = TaskGroupBox::new();
        group.cancelAll();
        let late = crate::boxes::future::FutureBox::new();

        group.add_future(&late);

        assert_eq!(
            late.to_string_box().value,
            "Future(cancelled: Cancelled: scope-cancelled)"
        );
    }

    #[test]
    fn add_future_after_first_failure_is_immediately_cancelled() {
        let group = TaskGroupBox::new();
        let first = crate::boxes::future::FutureBox::new();
        group.add_future(&first);
        first.set_failed(Box::new(StringBox::new("boom")));
        let late = crate::boxes::future::FutureBox::new();

        group.add_future(&late);

        assert_eq!(
            late.to_string_box().value,
            "Future(cancelled: Cancelled: sibling-failed)"
        );
    }
}
