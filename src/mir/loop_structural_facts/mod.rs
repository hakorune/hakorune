//! Sealed resolved-Loop source to portable recipe-source boundary.
//!
//! See `README.md` for the authority and caller-zero contract.

mod loop_cond_break_continue_source;
mod resolved_source_adapter;

pub(crate) use resolved_source_adapter::{
    bind_resolved_loop_root_v1, LoopRootSourceBindingRejectV1, VerifiedLoopRootSourceV1,
};

pub(crate) use loop_cond_break_continue_source::{
    VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    VerifiedLoopCondBreakContinueSourceProjectionV1, VerifiedLoopCondBreakContinueSourceShapeV1,
    VerifiedLoopCondSourceExitV1,
};

#[cfg(test)]
mod tests;
