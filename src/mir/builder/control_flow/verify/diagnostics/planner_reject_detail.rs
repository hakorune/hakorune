//! Planner reject-detail state shared by route and freeze diagnostics.

use std::cell::RefCell;

thread_local! {
    static LAST_PLAN_REJECT_DETAIL: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Clear last recorded planner/reject detail.
pub(in crate::mir::builder) fn clear_last_plan_reject_detail() {
    LAST_PLAN_REJECT_DETAIL.with(|slot| {
        *slot.borrow_mut() = None;
    });
}

/// Set last planner/reject detail.
pub(in crate::mir::builder) fn set_last_plan_reject_detail(detail: String) {
    LAST_PLAN_REJECT_DETAIL.with(|slot| {
        *slot.borrow_mut() = Some(detail);
    });
}

/// Set planner/reject detail only when no detail has been recorded yet.
pub(in crate::mir::builder) fn set_last_plan_reject_detail_if_absent(detail: String) {
    LAST_PLAN_REJECT_DETAIL.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.is_none() {
            *slot = Some(detail);
        }
    });
}

/// Take (consume) last recorded planner/reject detail.
pub(in crate::mir::builder) fn take_last_plan_reject_detail() -> Option<String> {
    LAST_PLAN_REJECT_DETAIL.with(|slot| slot.borrow_mut().take())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planner_reject_detail_roundtrip() {
        clear_last_plan_reject_detail();
        assert!(take_last_plan_reject_detail().is_none());

        set_last_plan_reject_detail("x".to_string());
        assert_eq!(take_last_plan_reject_detail().as_deref(), Some("x"));
        assert!(take_last_plan_reject_detail().is_none());
    }

    #[test]
    fn planner_reject_detail_if_absent_preserves_first() {
        clear_last_plan_reject_detail();
        set_last_plan_reject_detail_if_absent("first".to_string());
        set_last_plan_reject_detail_if_absent("second".to_string());
        assert_eq!(take_last_plan_reject_detail().as_deref(), Some("first"));
    }
}
