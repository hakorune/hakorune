//! Phase 263 P0.2: Promotion decision types (SSOT)
//!
//! PromoteDecision enum eliminates Option<_> wrapping ambiguity by making
//! the decision explicit. All Pattern2 promotion logic flows through this type.

use super::super::super::pattern2_inputs_facts_box::Pattern2Inputs;

pub(crate) struct PromoteStepResult {
    pub inputs: Pattern2Inputs,
}

/// Phase 263 P0.1: Promotion decision for Pattern2 LoopBodyLocal handling
///
/// Eliminates Option<_> wrapping ambiguity by making decision explicit.
pub(crate) enum PromoteDecision {
    /// Promotion succeeded - Pattern2 can proceed
    Promoted(PromoteStepResult),

    /// Promotion not applicable (e.g., no LoopBodyLocal in conditions)
    /// → Continue Pattern2 with unchanged inputs
    NotApplicable(PromoteStepResult),

    /// Contract violation or unimplemented behavior
    /// → Fail-Fast with error message
    Freeze(String),
}
