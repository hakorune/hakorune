//! One bounded selected Stage-B instance-function session.
//!
//! F6-2 owns only the body schedule. It reuses the existing legacy block
//! driver once and keeps prefix, selected carrier publication, and suffix
//! descent behind one monotonic owner. Function preparation/finalization and
//! pending-session capture remain F6-3 responsibilities.

mod body_schedule;
mod collector_terminal;
mod rejection;
mod session;
mod session_rejection;

pub(in crate::mir::builder) use collector_terminal::{
    collect_preloop_stageb_instance_function_v1, CollectedPreloopStageBInstanceFunctionV1,
    RejectedPreloopStageBInstanceFunctionCollectionV1,
};
pub(in crate::mir::builder) use session::{
    capture_preloop_stageb_instance_function_from_ingress_v1,
    CompletedPreloopStageBInstanceFunctionPayloadV1, CompletedPreloopStageBInstanceFunctionV1,
    PendingPreloopStageBInstanceFunctionSessionV1,
};
pub(in crate::mir::builder) use session_rejection::RejectedPreloopStageBInstanceFunctionSessionV1;

#[cfg(test)]
mod body_schedule_tests;
#[cfg(test)]
mod session_tests;
