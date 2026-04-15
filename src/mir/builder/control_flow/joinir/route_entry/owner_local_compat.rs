//! Minimal owner-local bridge for route_entry.
//!
//! Keep the remaining owner-local plan residue here so route_entry routing code
//! does not pattern-match on keep-plan enum variants directly.

use crate::mir::builder::control_flow::plan::loop_cond::break_continue_types::{
    LoopCondBreakAcceptKind, LoopCondBreakContinueFacts,
};

pub(in crate::mir::builder::control_flow::joinir::route_entry) fn loop_cond_break_release_allowed(
    facts: &LoopCondBreakContinueFacts,
) -> bool {
    !matches!(
        facts.accept_kind,
        LoopCondBreakAcceptKind::NestedLoopOnly | LoopCondBreakAcceptKind::ProgramBlockNoExit
    )
}

pub(in crate::mir::builder::control_flow::joinir::route_entry) fn loop_cond_break_is_return_only_body(
    facts: &LoopCondBreakContinueFacts,
) -> bool {
    matches!(facts.accept_kind, LoopCondBreakAcceptKind::ReturnOnlyBody)
}

#[cfg(test)]
pub(in crate::mir::builder::control_flow::joinir::route_entry) use crate::mir::builder::control_flow::plan::loop_scan_methods_block_v0::try_extract_loop_scan_methods_block_v0_facts;
#[cfg(test)]
pub(in crate::mir::builder::control_flow::joinir::route_entry) use crate::mir::builder::control_flow::plan::loop_scan_methods_v0::try_extract_loop_scan_methods_v0_facts;
