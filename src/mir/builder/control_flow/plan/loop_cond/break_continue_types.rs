//! Compatibility wrapper while loop_cond break/continue facts types live under `facts/`.

pub(in crate::mir::builder) use crate::mir::builder::control_flow::facts::loop_cond_break_continue::{
    ContinueBranchSig, LoopCondBreakAcceptKind, LoopCondBreakContinueFacts,
};

/// Maximum number of nested loops allowed.
pub(in crate::mir::builder) const MAX_NESTED_LOOPS: usize = 8;
