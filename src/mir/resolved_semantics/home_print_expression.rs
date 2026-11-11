//! Whole Print qualification over the shared original expression plan.
//! No physical IDs, runtime tokens or independent call inventory are issued.
use super::super::{LocalCallObservationV1, OwnedExprSiteV1, SourceExprSiteV1};
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_value_profile::{
    expression_class::ExpressionValueClassV1, expression_plan::ExpressionPlanRootV1,
    expression_projection::ExpressionLeafClassV1, expression_root::StatementExpressionRoleV1,
};
use std::collections::BTreeMap;

/// Print purpose authority wraps the SAME qualified expression body. The body
/// alone neither selects a Print root nor installs a Home observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedPrintExpressionFrameV1 {
    evaluation: CheckedExpressionEvaluationV1,
}
impl CheckedPrintExpressionFrameV1 {
    /// A bare statement call whose owned result ends with the statement.
    pub(crate) fn is_discard(&self) -> bool {
        matches!(
            self.source().root(),
            ExpressionPlanRootV1::Statement {
                role: StatementExpressionRoleV1::DiscardValue,
                ..
            }
        )
    }
    pub(crate) fn evaluation(&self) -> &CheckedExpressionEvaluationV1 {
        &self.evaluation
    }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> {
        self.evaluation.source()
    }
    pub(crate) fn prior_homes(&self) -> &[LiveHomeV1] {
        self.evaluation.prior_homes()
    }
    pub(crate) fn calls(&self) -> &[LocalCallObservationV1] {
        self.evaluation.calls()
    }
    pub(crate) fn operations(&self) -> &[Rc<CheckedExpressionOperationLoanV1>] {
        self.evaluation.operations()
    }
    pub(crate) fn owned_result_nodes(&self) -> &[usize] {
        self.evaluation.owned_result_nodes()
    }
}

/// The caller first retains original actual facts at the pre-statement basis.
/// Call demands below are passive: none may publish Home call observations.
/// Finish every rejectable root/envelope/install check before the strict field
/// batch; after that final atomic batch succeeds, returning the frame cannot fail.
pub(crate) fn qualify_print_expression_v1<E>(
    source: Rc<ExpressionSourcePlanV1>,
    prior_homes: &[LiveHomeV1],
    static_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::StaticTextReadV1>,
    residence_reference: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<(
        super::super::BindingRefV1,
        super::super::BindingRefV1,
        TextResidenceAcquisitionV1,
    )>,
    loop_text_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::LoopTextReadV1>,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::CurrentScalarReadV1>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    field_seeds: &BTreeMap<usize, ExpressionLeafClassV1>,
    source_call: &mut impl FnMut(
        &OwnedExprSiteV1,
    ) -> Result<Option<super::leaves::ExpressionCallSourceV1>, E>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        super::super::LocalFieldReadDemandV1,
    ) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    if !source.admits_prior_homes_v1(prior_homes) {
        return Ok(None);
    }
    let ExpressionPlanRootV1::Statement {
        role: StatementExpressionRoleV1::PrintValue | StatementExpressionRoleV1::DiscardValue,
        value,
        ..
    } = source.root()
    else {
        return Ok(None);
    };
    let Some(root) = source.nodes().first() else {
        return Ok(None);
    };
    if root.site() != value {
        return Ok(None);
    }
    if !source.children_always_known_v1() {
        return Ok(None);
    }

    let Some(prepared) = super::leaves::prepare_expression_calls_v1(
        &source,
        super::leaves::ExpressionLeafProfileV1::Print,
        field_seeds,
        prior_homes,
        source_call,
    )?
    else {
        return Ok(None);
    };
    let Ok((evaluation, fields)) = super::qualification::qualify_expression_evaluation_v1(
        prepared,
        Vec::new(),
        static_reference,
        residence_reference,
        loop_text_reference,
        &mut |site| {
            scalar_reference(site).map(super::evaluation::CheckedScalarReferenceV1::Original)
        },
        field_request,
    )?
    else {
        return Ok(None);
    };
    if !super::qualification::admits_statement_value_v1(&evaluation) {
        return Ok(None);
    }
    let frame = Rc::new(CheckedPrintExpressionFrameV1 { evaluation });
    if !frame.evaluation().reference_coverage_is_exact() {
        return Ok(None);
    }
    let candidate = HomeExpressionPlanV1 {
        print_frame: Some(frame),
        ..HomeExpressionPlanV1::for_source(Rc::clone(&source))
    };
    let Some(candidate) = candidate.with_source_field_requirements(&fields) else {
        return Ok(None);
    };
    if !can_install(&candidate) {
        return Ok(None);
    }
    if !field_batch(
        &fields,
        super::super::LocalFieldReadDemandV1::SourceExpression {
            probe: false,
            plan: source,
        },
    )? {
        return Ok(None);
    }
    Ok(Some(candidate))
}
