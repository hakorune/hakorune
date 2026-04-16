//! Compatibility wrapper while loop_cond break/continue facts types live under `facts/`.

pub(in crate::mir::builder) use crate::mir::builder::control_flow::facts::loop_cond_break_continue::{
    ContinueBranchSig, LoopCondBreakAcceptKind, LoopCondBreakContinueFacts,
};

/// Internal classification of if statements within loop bodies.
#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) enum IfStmtKind {
    ExitIf,
    ContinueIf { continue_in_then: bool },
    ConditionalUpdate,
    GeneralIf,
}

/// Maximum number of nested loops allowed.
pub(in crate::mir::builder) const MAX_NESTED_LOOPS: usize = 8;
