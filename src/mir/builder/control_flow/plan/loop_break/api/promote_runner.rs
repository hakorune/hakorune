//! Phase 263 P0.2: loop_break promotion runner (SSOT entry point)
//!
//! Single entry point for all loop_break promotion logic.
//! All callers should use `try_promote()` instead of accessing internals directly.

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;

use super::super::super::loop_break_prep_box::LoopBreakPrepInputs;

use super::promote_decision::{PromoteDecision, PromoteStepResult};
use super::promote_finalize_helpers::finalize_promoted_inputs;
use super::promote_prepare_helpers::prepare_promoted_inputs;

/// Phase 263 P0.2: Try to promote LoopBodyLocal variables for loop_break route
///
/// This is the single entry point for loop_break promotion logic.
/// Returns PromoteDecision to indicate success, applicability, or freeze.
pub(in crate::mir::builder) fn try_promote(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    inputs: LoopBreakPrepInputs,
    debug: bool,
    _verbose: bool,
) -> Result<PromoteDecision, String> {
    let mut inputs = inputs;
    let prepared = match prepare_promoted_inputs(builder, condition, body, &mut inputs) {
        Ok(result) => result,
        Err(reason) => return Ok(PromoteDecision::Freeze(reason)),
    };

    finalize_promoted_inputs(builder, &mut inputs, prepared.promoted_pairs, debug)?;

    if prepared.has_body_locals_in_conditions {
        Ok(PromoteDecision::Promoted(PromoteStepResult { inputs }))
    } else {
        Ok(PromoteDecision::NotApplicable(PromoteStepResult { inputs }))
    }
}
