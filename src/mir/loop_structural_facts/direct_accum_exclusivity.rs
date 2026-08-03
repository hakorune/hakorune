//! Source-side exclusivity proof for the DirectAccum pilot.
//!
//! The projector issues `DirectAccumStructuralShapeV1` only after navigating
//! the resolved source and checking the exact two-assignment grammar. This
//! module turns that closed grammar into a separate proof so policy admission
//! cannot mistake a shape payload for an exclusive route certificate.

use super::types::DirectAccumStructuralShapeV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedDirectAccumDisjointnessV1 {
    grammar: DirectAccumTerminalGrammarV1,
    _seal: DirectAccumDisjointnessSealV1,
}

#[derive(Debug, PartialEq, Eq)]
enum DirectAccumTerminalGrammarV1 {
    TwoAssignmentsWithoutControlFlow,
}

#[derive(Debug, PartialEq, Eq)]
struct DirectAccumDisjointnessSealV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectAccumDisjointnessRejectV1 {
    InductionAccumulatorCollision,
    ConditionBindingMismatch,
    UpdateBindingMismatch,
    StepBindingMismatch,
    DuplicateStatementSite,
    DuplicateExpressionSite,
}

/// Issues the source-side proof after the resolved projector has established
/// the exact two-assignment grammar. No route id, raw cursor, legacy facts, or
/// physical identity can be used to mint the proof.
pub(crate) fn issue_direct_accum_disjointness_v1(
    shape: &DirectAccumStructuralShapeV1,
) -> Result<VerifiedDirectAccumDisjointnessV1, DirectAccumDisjointnessRejectV1> {
    if shape.induction == shape.accumulator {
        return Err(DirectAccumDisjointnessRejectV1::InductionAccumulatorCollision);
    }
    if shape.condition_binding != shape.induction {
        return Err(DirectAccumDisjointnessRejectV1::ConditionBindingMismatch);
    }
    if shape.update.binding != shape.accumulator {
        return Err(DirectAccumDisjointnessRejectV1::UpdateBindingMismatch);
    }
    if shape.step.binding != shape.induction {
        return Err(DirectAccumDisjointnessRejectV1::StepBindingMismatch);
    }
    if shape.update.statement_site == shape.step.statement_site {
        return Err(DirectAccumDisjointnessRejectV1::DuplicateStatementSite);
    }
    let expression_sites = [
        &shape.condition_site,
        &shape.condition_lhs_site,
        &shape.update.target_site,
        &shape.update.value_site,
        &shape.update.lhs_site,
        &shape.update.rhs_site,
        &shape.step.target_site,
        &shape.step.value_site,
        &shape.step.lhs_site,
        &shape.step.rhs_site,
    ];
    if has_duplicate_expression_site(&expression_sites) {
        return Err(DirectAccumDisjointnessRejectV1::DuplicateExpressionSite);
    }
    Ok(VerifiedDirectAccumDisjointnessV1 {
        grammar: DirectAccumTerminalGrammarV1::TwoAssignmentsWithoutControlFlow,
        _seal: DirectAccumDisjointnessSealV1,
    })
}

impl VerifiedDirectAccumDisjointnessV1 {
    pub(crate) fn grammar_is_terminal(&self) -> bool {
        matches!(
            self.grammar,
            DirectAccumTerminalGrammarV1::TwoAssignmentsWithoutControlFlow
        )
    }
}

fn has_duplicate_expression_site(sites: &[&SourceExprSiteV1]) -> bool {
    sites
        .iter()
        .enumerate()
        .any(|(index, site)| sites[index + 1..].contains(site))
}
