//! StepMode pair helpers (plan-wide SSOT).

use crate::mir::builder::control_flow::plan::LoopStepMode;

#[inline]
pub(in crate::mir::builder) fn inline_in_body_no_explicit_step() -> (LoopStepMode, bool) {
    (LoopStepMode::InlineInBody, false)
}

#[inline]
pub(in crate::mir::builder) fn inline_in_body_explicit_step() -> (LoopStepMode, bool) {
    (LoopStepMode::InlineInBody, true)
}

#[inline]
pub(in crate::mir::builder) fn extract_to_step_bb_explicit_step() -> (LoopStepMode, bool) {
    (LoopStepMode::ExtractToStepBb, true)
}
