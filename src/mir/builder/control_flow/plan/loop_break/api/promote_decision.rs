//! Phase 263 P0.2: Promotion decision types (SSOT)
//!
//! PromoteDecision enum eliminates Option<_> wrapping ambiguity by making
//! the decision explicit. All loop_break promotion logic flows through this type.

use super::super::super::loop_break_prep_box::LoopBreakPrepInputs;

pub(crate) struct PromoteStepResult {
    pub inputs: LoopBreakPrepInputs,
}

/// Phase 263 P0.1: Promotion decision for loop_break LoopBodyLocal handling
///
/// Eliminates Option<_> wrapping ambiguity by making decision explicit.
pub(crate) enum PromoteDecision {
    /// Promotion succeeded - loop_break route can proceed
    Promoted(PromoteStepResult),

    /// Promotion not applicable (e.g., no LoopBodyLocal in conditions)
    /// → Continue loop_break route with unchanged inputs
    NotApplicable(PromoteStepResult),

    /// Contract violation or unimplemented behavior
    /// → Fail-Fast with error message
    Freeze(String),
}
