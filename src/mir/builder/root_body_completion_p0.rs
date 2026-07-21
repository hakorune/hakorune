//! HEADERPORT0-I0-BODYDRAIN0-P0: disconnected closure/failure matrix.
//!
//! These fixtures exercise the S0 witness without opening a production
//! lowering route.  They keep nested child completion, header-loan closure,
//! and pending-terminal closure as independent activities.

use super::root_body_completion::{
    RootBodyCompletionErrorV1, RootBodyCompletionTrackerV1, RootBodyResultV1,
};
use crate::mir::ValueId;

#[test]
fn nested_children_close_inner_before_outer() {
    let mut tracker = RootBodyCompletionTrackerV1::new();
    let outer = tracker.begin_child();
    let inner = tracker.begin_child();
    tracker.close_child(inner).unwrap();
    tracker.close_child(outer).unwrap();

    let completed = tracker
        .complete(RootBodyResultV1::Value(ValueId::new(11)))
        .unwrap();
    assert_eq!(completed.completed_children(), 2);
}

#[test]
fn header_and_pending_tokens_close_before_root_completion() {
    let mut tracker = RootBodyCompletionTrackerV1::new();
    let header = tracker.begin_header_loan();
    let pending = tracker.begin_pending_terminal();

    tracker.close_pending_terminal(pending).unwrap();
    tracker.close_header_loan(header).unwrap();
    assert_eq!(
        tracker
            .complete(RootBodyResultV1::NoValue)
            .unwrap()
            .result(),
        RootBodyResultV1::NoValue
    );
}

#[test]
fn each_open_activity_has_a_distinct_fail_fast_disposition() {
    let mut child_tracker = RootBodyCompletionTrackerV1::new();
    let _child = child_tracker.begin_child();
    assert_eq!(
        child_tracker
            .complete(RootBodyResultV1::NoValue)
            .unwrap_err(),
        RootBodyCompletionErrorV1::OpenChildScopes { count: 1 }
    );

    let mut header_tracker = RootBodyCompletionTrackerV1::new();
    let _header = header_tracker.begin_header_loan();
    assert_eq!(
        header_tracker
            .complete(RootBodyResultV1::NoValue)
            .unwrap_err(),
        RootBodyCompletionErrorV1::OpenHeaderLoans { count: 1 }
    );

    let mut pending_tracker = RootBodyCompletionTrackerV1::new();
    let _pending = pending_tracker.begin_pending_terminal();
    assert_eq!(
        pending_tracker
            .complete(RootBodyResultV1::NoValue)
            .unwrap_err(),
        RootBodyCompletionErrorV1::OpenPendingTerminals { count: 1 }
    );
}

#[test]
fn failed_completion_consumes_the_tracker_without_a_witness() {
    let mut tracker = RootBodyCompletionTrackerV1::new();
    let _child = tracker.begin_child();
    let result = tracker.complete(RootBodyResultV1::NoValue);
    assert!(matches!(
        result,
        Err(RootBodyCompletionErrorV1::OpenChildScopes { count: 1 })
    ));
}
