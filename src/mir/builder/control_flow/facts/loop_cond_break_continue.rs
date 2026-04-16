//! Facts owner surface for loop_cond_break_continue types.
//!
//! The route-specific builders stay in the loop_cond family, but the extracted
//! facts contract lives here so non-family callers can depend on a top-level
//! facts owner surface.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe;
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_recipe::LoopCondBreakContinueRecipe;
use crate::mir::policies::BodyLoweringPolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopCondBreakAcceptKind {
    ExitIf,
    ContinueIf,
    ConditionalUpdate,
    ReturnInExitIf,
    ReturnOnlyBody,
    ElseOnlyReturn,
    ElseOnlyBreak,
    MixedIf,
    NestedLoopOnly,
    ProgramBlockNoExit,
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopCondBreakContinueFacts {
    pub accept_kind: LoopCondBreakAcceptKind,
    pub propagate_nested_carriers: bool,
    pub condition: ASTNode,
    pub recipe: LoopCondBreakContinueRecipe,
    pub has_handled_guard_break: bool,
    pub handled_var_name: Option<String>,
    pub continue_branches: Vec<ContinueBranchSig>,
    pub body_lowering_policy: BodyLoweringPolicy,
    pub body_exit_allowed: Option<ExitAllowedBlockRecipe>,
}

impl LoopCondBreakContinueFacts {
    pub(in crate::mir::builder) fn release_allowed(&self) -> bool {
        !matches!(
            self.accept_kind,
            LoopCondBreakAcceptKind::NestedLoopOnly | LoopCondBreakAcceptKind::ProgramBlockNoExit
        )
    }

    pub(in crate::mir::builder) fn is_return_only_body(&self) -> bool {
        matches!(self.accept_kind, LoopCondBreakAcceptKind::ReturnOnlyBody)
    }
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct ContinueBranchSig {
    pub stmt_count: usize,
    pub has_assignment: bool,
    pub has_local: bool,
}
