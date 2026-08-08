//! Structural coherence checks for the bounded variable recurrence Facts issuer.
//!
//! This module owns only source-site/role validation.  It does not issue
//! Recipe keys, inspect AST nodes, or select a family.  Keeping these checks
//! outside the 800-line Facts owner preserves one semantic owner without
//! growing that file into a second policy surface.

use crate::mir::resolved_semantics::{
    SourceBindingSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
    VerifiedResolvedLoopSourceV1,
};

use super::variable_accum_recurrence::{
    VariableAccumRecurrenceAccumulatorUpdateV1, VariableAccumRecurrenceBindingObservationV1,
    VariableAccumRecurrenceConditionObservationV1, VariableAccumRecurrenceCoverageV1,
    VariableAccumRecurrenceInductionStepV1, VariableAccumRecurrenceInputObservationV1,
};

pub(crate) fn source_coherence_is_exact(
    source: &VerifiedResolvedLoopSourceV1,
    bindings: &[VariableAccumRecurrenceBindingObservationV1; 2],
    inputs: &[VariableAccumRecurrenceInputObservationV1; 2],
    condition: &VariableAccumRecurrenceConditionObservationV1,
    accumulator_update: &VariableAccumRecurrenceAccumulatorUpdateV1,
    induction_step: &VariableAccumRecurrenceInductionStepV1,
    coverage: &VariableAccumRecurrenceCoverageV1,
) -> bool {
    let checks = [
        declarations_are_canonical(bindings, inputs),
        coverage_is_canonical(
            source,
            condition,
            accumulator_update,
            induction_step,
            coverage,
        ),
        condition_paths_are_canonical(source, condition),
        assignment_paths_are_canonical(source, accumulator_update, 0),
        assignment_paths_are_canonical(source, induction_step, 1),
    ];
    checks.iter().all(|check| *check)
}

fn declarations_are_canonical(
    bindings: &[VariableAccumRecurrenceBindingObservationV1; 2],
    inputs: &[VariableAccumRecurrenceInputObservationV1; 2],
) -> bool {
    bindings.iter().all(|binding| {
        matches!(
            binding.declaration(),
            SourceBindingSiteV1::Local { ordinal: 0, .. }
        )
    }) && inputs.iter().all(|input| {
        matches!(
            input.declaration(),
            SourceBindingSiteV1::Local { ordinal: 0, .. }
        ) && matches!(
            input.initializer().node().segments().last(),
            Some(SourcePathSegmentV1::Initializer(0))
        )
    })
}

fn coverage_is_canonical(
    source: &VerifiedResolvedLoopSourceV1,
    condition: &VariableAccumRecurrenceConditionObservationV1,
    accumulator_update: &VariableAccumRecurrenceAccumulatorUpdateV1,
    induction_step: &VariableAccumRecurrenceInductionStepV1,
    coverage: &VariableAccumRecurrenceCoverageV1,
) -> bool {
    let result = coverage.root_statement_count() == 5
        && coverage.body_statement_sites().len() == 2
        && coverage.body_statement_sites()[0] == *accumulator_update.statement()
        && coverage.body_statement_sites()[1] == *induction_step.statement()
        && distinct_stmt_sites(&[accumulator_update.statement(), induction_step.statement()])
        && under_stmt_expr(
            source.site(),
            condition.site(),
            &[SourcePathSegmentV1::LoopCondition],
        )
        && under_stmt(
            source.site(),
            accumulator_update.statement(),
            &[SourcePathSegmentV1::LoopBody(0)],
        )
        && under_stmt(
            source.site(),
            induction_step.statement(),
            &[SourcePathSegmentV1::LoopBody(1)],
        );
    result
}

fn condition_paths_are_canonical(
    source: &VerifiedResolvedLoopSourceV1,
    condition: &VariableAccumRecurrenceConditionObservationV1,
) -> bool {
    under_stmt_expr(
        source.site(),
        condition.site(),
        &[SourcePathSegmentV1::LoopCondition],
    ) && under_expr(
        condition.site(),
        condition.lhs(),
        &[SourcePathSegmentV1::Lhs],
    ) && under_expr(
        condition.site(),
        condition.rhs(),
        &[SourcePathSegmentV1::Rhs],
    ) && distinct_expr_sites(&[condition.site(), condition.lhs(), condition.rhs()])
}

fn assignment_paths_are_canonical(
    source: &VerifiedResolvedLoopSourceV1,
    assignment: &impl AssignmentSourceSites,
    body_index: u32,
) -> bool {
    let result = under_stmt(
        source.site(),
        assignment.statement(),
        &[SourcePathSegmentV1::LoopBody(body_index)],
    ) && under_stmt_expr(
        assignment.statement(),
        assignment.target(),
        &[SourcePathSegmentV1::Target],
    ) && under_stmt_expr(
        assignment.statement(),
        assignment.value(),
        &[SourcePathSegmentV1::Value],
    ) && under_expr(
        assignment.value(),
        assignment.lhs(),
        &[SourcePathSegmentV1::Lhs],
    ) && under_expr(
        assignment.value(),
        assignment.rhs(),
        &[SourcePathSegmentV1::Rhs],
    ) && distinct_expr_sites(&[
        assignment.target(),
        assignment.value(),
        assignment.lhs(),
        assignment.rhs(),
    ]);
    result
}

trait AssignmentSourceSites {
    fn statement(&self) -> &SourceStmtSiteV1;
    fn target(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1;
    fn value(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1;
    fn lhs(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1;
    fn rhs(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1;
}

impl AssignmentSourceSites for VariableAccumRecurrenceAccumulatorUpdateV1 {
    fn statement(&self) -> &SourceStmtSiteV1 {
        self.statement()
    }
    fn target(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.target()
    }
    fn value(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.value()
    }
    fn lhs(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.lhs()
    }
    fn rhs(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.rhs()
    }
}

impl AssignmentSourceSites for VariableAccumRecurrenceInductionStepV1 {
    fn statement(&self) -> &SourceStmtSiteV1 {
        self.statement()
    }
    fn target(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.target()
    }
    fn value(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.value()
    }
    fn lhs(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.lhs()
    }
    fn rhs(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.rhs()
    }
}

fn distinct_stmt_sites(sites: &[&SourceStmtSiteV1]) -> bool {
    sites[0] != sites[1]
}

fn distinct_expr_sites(sites: &[&crate::mir::resolved_semantics::SourceExprSiteV1]) -> bool {
    sites
        .iter()
        .enumerate()
        .all(|(index, site)| sites.iter().skip(index + 1).all(|other| *site != *other))
}

fn under_stmt_expr(
    parent: &SourceStmtSiteV1,
    child: &crate::mir::resolved_semantics::SourceExprSiteV1,
    suffix: &[SourcePathSegmentV1],
) -> bool {
    under_nodes(parent.node(), child.node(), suffix)
}

fn under_expr(
    parent: &crate::mir::resolved_semantics::SourceExprSiteV1,
    child: &crate::mir::resolved_semantics::SourceExprSiteV1,
    suffix: &[SourcePathSegmentV1],
) -> bool {
    under_nodes(parent.node(), child.node(), suffix)
}

fn under_stmt(
    parent: &SourceStmtSiteV1,
    child: &SourceStmtSiteV1,
    suffix: &[SourcePathSegmentV1],
) -> bool {
    under_nodes(parent.node(), child.node(), suffix)
}

fn under_nodes(
    parent: &SourceNodeSiteV1,
    child: &SourceNodeSiteV1,
    suffix: &[SourcePathSegmentV1],
) -> bool {
    let parent_segments = parent.segments();
    let child_segments = child.segments();
    child_segments.len() == parent_segments.len() + suffix.len()
        && child_segments.starts_with(parent_segments)
        && child_segments[parent_segments.len()..] == *suffix
}
