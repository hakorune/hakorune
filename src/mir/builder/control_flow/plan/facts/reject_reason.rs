//! Structured reject reasons and handoff targets for planner entry guards.
//!
//! SSOT: docs/development/current/main/design/planner-entry-guards-ssot.md

use crate::mir::builder::control_flow::verify::diagnostics::planner_reject_detail;
use std::fmt;

/// Macro to define enum with unified Display implementation.
///
/// This macro generates:
/// - The enum variants
/// - An `as_str()` method for string conversion
/// - A `Display` implementation that delegates to `as_str()`
///
/// This provides a single source of truth for variant names and their display strings.
macro_rules! reject_reasons {
    (
        $(#[$enum_meta:meta])*
        pub enum $enum_name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $display_str:literal
            ),* $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        pub enum $enum_name {
            $(
                $(#[$variant_meta])*
                $variant
            ),*
        }

        impl $enum_name {
            /// Returns the display string for this reject reason.
            pub(super) fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $display_str),*
                }
            }
        }

        impl fmt::Display for $enum_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }
    };
}

// Define RejectReason enum with unified Display implementation
reject_reasons! {
/// Reasons why a Facts extractor rejects a loop shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectReason {
    // Phase 5 (loop_cond_continue_only)
    ReturnInBody => "return_in_body",
    ReturnInBranch => "return_in_branch",
    ReturnInBothBranches => "return_in_both_branches",
    BreakPresent => "break_present",
    ContinueInIfWithElse => "continue_in_if_with_else",
    TopLevelNestedLoop => "top_level_nested_loop",
    NoContinue => "no_continue",
    UnsupportedCondition => "unsupported_condition",
    MultipleNestedLoopsInContinueIfPrelude => "multiple_nested_loops_in_continue_if_prelude",
    ControlFlowInNestedLoopPreludePostlude => "control_flow_in_nested_loop_prelude_postlude",

    // Phase 6 (loop_cond_break_continue)
    ConditionIsTrue => "condition_is_true",
    ConditionNotSupported => "condition_not_supported",
    NestedLoopNotAllowed => "nested_loop_not_allowed",
    UnsupportedStmt => "unsupported_stmt",
    NestedLoopCount => "nested_loop_count",
    NoBreakOrContinue => "no_break_or_continue",
    NoExitIf => "no_exit_if",
    ContinueOnly => "continue_only",
    ExitAllowedRecipeBuildFailed => "exit_allowed_recipe_build_failed",

    // Phase 29ca (generic_loop)
    InBodyStepWithContinue => "in_body_step_with_continue",
    NoValidLoopVarCandidates => "no_valid_loop_var_candidates",
    AmbiguousLoopVarCandidates => "ambiguous_loop_var_candidates",
    ControlFlowAfterInBodyStep => "control_flow_after_in_body_step",
    ExitAfterInBodyStep => "exit_after_in_body_step",
    LoopVarUsedAfterInBodyStep => "loop_var_used_after_in_body_step",
    UnsupportedStmtAfterInBodyStep => "unsupported_stmt_after_in_body_step",
    ContinueIfStepRequiresTrailingExit => "continue_if_step_requires_trailing_exit",
    BreakElseStepMustBeFinalStmt => "break_else_step_must_be_final_stmt",
    MultipleConditionalStepAssignments => "multiple_conditional_step_assignments",
    MultipleStepAssignments => "multiple_step_assignments",

    // Phase 29at (match_return)
    MatchReturnScrutineeNotSupported => "match_return_scrutinee_not_supported",
    MatchReturnTooFewArms => "match_return_too_few_arms",
    MatchReturnElseNotLiteral => "match_return_else_not_literal",
    MatchReturnArmLabelNotSupported => "match_return_arm_label_not_supported",
    MatchReturnArmNotLiteral => "match_return_arm_not_literal",
    MatchReturnArmLiteralTypeUnsupported => "match_return_arm_literal_type_unsupported",
}
}

impl RejectReason {
    /// Returns the freeze message for this reject reason.
    ///
    /// # Contract
    /// This method is only called for reasons that have an explicit message mapping.
    /// Callers must ensure the reason is supported before calling.
    /// Unsupported reasons will panic in debug builds.
    pub fn as_freeze_message(self) -> &'static str {
        match self {
            Self::NoValidLoopVarCandidates => "no valid loop_var candidates found",
            Self::AmbiguousLoopVarCandidates => "multiple loop_var candidates matched (ambiguous)",
            Self::MultipleConditionalStepAssignments => {
                "generic loop v0.2: multiple conditional step assignments in body"
            }
            Self::MultipleStepAssignments => "generic loop v0.2: multiple step assignments in body",
            // Phase 29at (match_return)
            Self::MatchReturnScrutineeNotSupported => {
                "match return scrutinee must be var or int literal"
            }
            Self::MatchReturnTooFewArms => "match return requires >= 2 arms",
            Self::MatchReturnElseNotLiteral => "match return else must be literal",
            Self::MatchReturnArmLabelNotSupported => {
                "match return arm label must be int/bool literal"
            }
            Self::MatchReturnArmNotLiteral => "match return arm must be literal",
            Self::MatchReturnArmLiteralTypeUnsupported => {
                "match return arm literal type unsupported"
            }
            // Phase 29ca (generic_loop)
            Self::InBodyStepWithContinue => {
                "generic loop v0.2: in-body step + continue is not allowed"
            }
            Self::ControlFlowAfterInBodyStep => {
                "generic loop v0.2: control flow after in-body step"
            }
            Self::ExitAfterInBodyStep => "generic loop v0.2: exit after in-body step",
            Self::LoopVarUsedAfterInBodyStep => {
                "generic loop v0.2: loop var used after in-body step"
            }
            Self::UnsupportedStmtAfterInBodyStep => {
                "generic loop v0.2: unsupported stmt after in-body step"
            }
            Self::ContinueIfStepRequiresTrailingExit => {
                "generic loop v0.2: continue-if step requires trailing break/return"
            }
            Self::BreakElseStepMustBeFinalStmt => {
                "generic loop v0.2: break-else step must be final stmt"
            }
            _ => {
                debug_assert!(
                    false,
                    "as_freeze_message called for unsupported reason: {:?}",
                    self
                );
                "unsupported loop shape"
            }
        }
    }
}

/// Macro to define HandoffTarget enum with unified Display implementation.
macro_rules! handoff_targets {
    (
        $(#[$enum_meta:meta])*
        pub enum $enum_name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $display_str:literal
            ),* $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        pub enum $enum_name {
            $(
                $(#[$variant_meta])*
                $variant
            ),*
        }

        impl $enum_name {
            /// Returns the display string for this handoff target.
            fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $display_str),*
                }
            }
        }

        impl fmt::Display for $enum_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }
    };
}

// Define HandoffTarget enum with unified Display implementation
handoff_targets! {
/// Handoff targets when a box rejects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoffTarget {
    // Phase 5
    LoopCondBreakContinue => "loop_cond_break_continue",
    LoopCondReturnInBody => "loop_cond_return_in_body",
    LoopCondContinueWithReturn => "loop_cond_continue_with_return",
    OutOfScope => "out_of_scope",

    // Phase 6
    LoopTrueBreakContinue => "loop_true_break_continue",
    LoopCondContinueOnly => "loop_cond_continue_only",
}
}

/// Per-box handoff table (same RejectReason may have different handoff per box)
pub mod handoff_tables {
    use super::{HandoffTarget, RejectReason};

    /// Handoff table for loop_cond_continue_only
    pub fn for_loop_cond_continue_only(reason: RejectReason) -> HandoffTarget {
        match reason {
            RejectReason::ReturnInBody => HandoffTarget::LoopCondBreakContinue,
            RejectReason::ReturnInBranch => HandoffTarget::LoopCondBreakContinue,
            RejectReason::ReturnInBothBranches => HandoffTarget::LoopCondContinueWithReturn,
            RejectReason::BreakPresent => HandoffTarget::LoopCondBreakContinue,
            RejectReason::ContinueInIfWithElse => HandoffTarget::LoopCondBreakContinue,
            RejectReason::TopLevelNestedLoop => HandoffTarget::OutOfScope,
            RejectReason::NoContinue => HandoffTarget::OutOfScope,
            RejectReason::UnsupportedCondition => HandoffTarget::OutOfScope,
            RejectReason::MultipleNestedLoopsInContinueIfPrelude => HandoffTarget::OutOfScope,
            RejectReason::ControlFlowInNestedLoopPreludePostlude => HandoffTarget::OutOfScope,
            // Phase 6 reasons (fallback for exhaustiveness)
            _ => HandoffTarget::OutOfScope,
        }
    }

    /// Handoff table for loop_cond_break_continue
    ///
    /// NOTE: ReturnInBody → LoopCondReturnInBody handoff fires only when
    /// return is NOT in exit-if form. Exit-if shaped returns are accepted
    /// by loop_cond_break_continue (logged as `accept: return_in_exit_if`).
    pub fn for_loop_cond_break_continue(reason: RejectReason) -> HandoffTarget {
        match reason {
            RejectReason::ConditionIsTrue => HandoffTarget::LoopTrueBreakContinue,
            RejectReason::ConditionNotSupported => HandoffTarget::OutOfScope,
            RejectReason::NestedLoopNotAllowed => HandoffTarget::OutOfScope,
            RejectReason::ReturnInBody => HandoffTarget::LoopCondReturnInBody,
            RejectReason::UnsupportedStmt => HandoffTarget::OutOfScope,
            RejectReason::NestedLoopCount => HandoffTarget::OutOfScope,
            RejectReason::NoBreakOrContinue => HandoffTarget::OutOfScope,
            RejectReason::NoExitIf => HandoffTarget::OutOfScope,
            RejectReason::ContinueOnly => HandoffTarget::LoopCondContinueOnly,
            // Phase 5 reasons (fallback)
            _ => HandoffTarget::OutOfScope,
        }
    }

    /// Handoff table for generic_loop
    ///
    /// NOTE: generic_loop rejects are currently all OutOfScope
    /// (no specialized receiver yet).
    pub fn for_generic_loop(reason: RejectReason) -> HandoffTarget {
        match reason {
            RejectReason::InBodyStepWithContinue => HandoffTarget::OutOfScope,
            RejectReason::NoValidLoopVarCandidates => HandoffTarget::OutOfScope,
            RejectReason::AmbiguousLoopVarCandidates => HandoffTarget::OutOfScope,
            RejectReason::ControlFlowAfterInBodyStep => HandoffTarget::OutOfScope,
            RejectReason::ExitAfterInBodyStep => HandoffTarget::OutOfScope,
            RejectReason::LoopVarUsedAfterInBodyStep => HandoffTarget::OutOfScope,
            RejectReason::UnsupportedStmtAfterInBodyStep => HandoffTarget::OutOfScope,
            RejectReason::ContinueIfStepRequiresTrailingExit => HandoffTarget::OutOfScope,
            RejectReason::BreakElseStepMustBeFinalStmt => HandoffTarget::OutOfScope,
            RejectReason::MultipleConditionalStepAssignments => HandoffTarget::OutOfScope,
            RejectReason::MultipleStepAssignments => HandoffTarget::OutOfScope,
            // Future: add more generic_loop reasons here
            _ => HandoffTarget::OutOfScope,
        }
    }

    /// Handoff table for match_return_facts
    ///
    /// NOTE: match_return_facts rejects are currently all OutOfScope
    /// (no specialized receiver yet).
    pub fn for_match_return_facts(reason: RejectReason) -> HandoffTarget {
        match reason {
            RejectReason::MatchReturnScrutineeNotSupported => HandoffTarget::OutOfScope,
            RejectReason::MatchReturnTooFewArms => HandoffTarget::OutOfScope,
            RejectReason::MatchReturnElseNotLiteral => HandoffTarget::OutOfScope,
            RejectReason::MatchReturnArmLabelNotSupported => HandoffTarget::OutOfScope,
            RejectReason::MatchReturnArmNotLiteral => HandoffTarget::OutOfScope,
            RejectReason::MatchReturnArmLiteralTypeUnsupported => HandoffTarget::OutOfScope,
            _ => HandoffTarget::OutOfScope,
        }
    }
}

/// Emit structured reject log (only when HAKO_JOINIR_DEBUG=1)
///
/// # Arguments
/// * `box_name` - Name of the box doing the rejection
/// * `reason` - Why the shape was rejected
/// * `handoff_fn` - Per-box handoff table function (e.g., `handoff_tables::for_loop_cond_continue_only`)
pub fn log_reject<F>(box_name: &str, reason: RejectReason, handoff_fn: F)
where
    F: Fn(RejectReason) -> HandoffTarget,
{
    let handoff = handoff_fn(reason);
    planner_reject_detail::set_last_plan_reject_detail(format!(
        "box={} reason={} handoff={}",
        box_name, reason, handoff
    ));

    if !crate::config::env::is_joinir_debug() {
        return;
    }
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[plan/reject] box={} reason={} handoff={}",
        box_name, reason, handoff
    ));
}

/// Emit structured accept log (only when HAKO_JOINIR_DEBUG=1)
///
/// # Arguments
/// * `box_name` - Name of the box accepting the shape
/// * `accept_tag` - Fixed vocabulary tag describing what was accepted
pub fn log_accept(box_name: &str, accept_tag: &'static str) {
    if !crate::config::env::is_joinir_debug() {
        return;
    }
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[plan/accept] box={} accept={}",
        box_name, accept_tag
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_reject_sets_diagnostic_detail() {
        planner_reject_detail::clear_last_plan_reject_detail();
        log_reject(
            "test_box",
            RejectReason::NoContinue,
            handoff_tables::for_loop_cond_continue_only,
        );
        assert_eq!(
            planner_reject_detail::take_last_plan_reject_detail().as_deref(),
            Some("box=test_box reason=no_continue handoff=out_of_scope")
        );
    }
}
