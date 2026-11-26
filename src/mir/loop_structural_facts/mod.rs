//! Sealed resolved-Loop source to portable recipe-source boundary.
//!
//! See `README.md` for the authority and caller-zero contract.

mod loop_cond_break_continue_source;
mod resolved_source_adapter;
// Caller-zero until the immediately following complete S6C Facts row.
mod s6c_exit_tail;
mod s6c_scan_with_init;

pub(crate) use resolved_source_adapter::{
    bind_resolved_loop_root_v1, LoopRootSourceBindingRejectV1, VerifiedLoopRootSourceV1,
};

pub(crate) use loop_cond_break_continue_source::{
    VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    VerifiedLoopCondBreakContinueSourceProjectionV1, VerifiedLoopCondBreakContinueSourceShapeV1,
    VerifiedLoopCondSourceExitV1,
};
pub(crate) use s6c_exit_tail::{
    issue_s6c_exit_tail_source_coseal_v1, S6CExitRoleV1, S6CExitTailSourceCoSealRefV1,
    S6CExitTailSourceCoSealRejectV1, VerifiedS6CExitTailSourceCoSealV1,
};
pub(crate) use s6c_scan_with_init::{
    issue_s6c_scan_with_init_facts_v1, S6CScanWithInitFactsRefV1, S6CScanWithInitFactsRejectV1,
    VerifiedS6CScanWithInitFactsV1,
};

#[cfg(test)]
mod s6c_exit_tail_tests;
#[cfg(test)]
mod tests;
