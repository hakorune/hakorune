//! Type definitions for loop_cond_break_continue facts.

use super::break_continue_recipe::LoopCondBreakContinueRecipe;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe;
use crate::mir::policies::BodyLoweringPolicy;

/// Accept kind for loop_cond_break_continue facts.
/// This enum represents the different patterns that can be accepted by the facts extractor.
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
    /// Loop with only nested loops + assignments (no direct if-break/continue patterns)
    NestedLoopOnly,
    /// Loop body contains program/scope blocks but no other recognized categories.
    ProgramBlockNoExit,
}

/// Facts extracted from a loop(condition) with break/continue patterns.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopCondBreakContinueFacts {
    pub accept_kind: LoopCondBreakAcceptKind,
    /// Whether nested-loop carrier propagation is enabled for this plan.
    ///
    /// This is a semantic flag carried by facts; `accept_kind` is log/contract-only.
    pub propagate_nested_carriers: bool,
    pub condition: ASTNode,
    pub recipe: LoopCondBreakContinueRecipe,
    pub has_handled_guard_break: bool,
    pub handled_var_name: Option<String>,
    pub continue_branches: Vec<ContinueBranchSig>,
    pub body_lowering_policy: BodyLoweringPolicy,
    /// Entire loop body lowered as a single exit-allowed RecipeBlock when possible.
    ///
    /// This is an optimization/structure hook: when present, the pipeline can lower the loop body
    /// via `parts::entry::lower_exit_allowed_block_verified(...)` instead of item-by-item lowering.
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

/// Signature for a continue branch (used for analysis).
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct ContinueBranchSig {
    pub stmt_count: usize,
    pub has_assignment: bool,
    pub has_local: bool,
}

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
