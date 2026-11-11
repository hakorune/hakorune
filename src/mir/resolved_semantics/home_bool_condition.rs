//! Whole Bool condition qualification on the same immutable expression source.
//! Conditional reach is retained; this owner never emits or eagerly runs RHS.
use super::super::{
    BindingRefV1, LocalCallObservationV1, LocalCallResultClassV1, OwnedExprSiteV1,
    SourceExprSiteV1, SourceScalarKind,
};
use super::leaves::ExpressionLeafProfileV1;
use super::*;
use crate::mir::dynamic_operator_contract::{DynamicOperatorFamilyV1, DynamicOperatorValueClassV1};
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_value_profile::{
    expression_class::ExpressionValueClassV1, expression_plan::ExpressionPlanRootV1,
    expression_projection::ExpressionLeafClassV1,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedBoolConditionFrameV1 {
    evaluation: CheckedExpressionEvaluationV1,
    logical: Box<[usize]>,
}
impl CheckedBoolConditionFrameV1 {
    pub(crate) fn evaluation(&self) -> &CheckedExpressionEvaluationV1 {
        &self.evaluation
    }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> {
        self.evaluation.source()
    }
    pub(crate) fn prior_homes(&self) -> &[LiveHomeV1] {
        self.evaluation.prior_homes()
    }
    pub(crate) fn ordered_nodes(&self) -> &[usize] {
        self.evaluation.ordered_nodes()
    }
    pub(crate) fn calls(&self) -> &[LocalCallObservationV1] {
        self.evaluation.calls()
    }
    pub(crate) fn operations(&self) -> &[Rc<CheckedExpressionOperationLoanV1>] {
        self.evaluation.operations()
    }
    pub(crate) fn logical_nodes(&self) -> &[usize] {
        &self.logical
    }
    pub(crate) fn references(&self) -> &BTreeMap<usize, CheckedExpressionReferenceV1> {
        self.evaluation.references()
    }
    pub(crate) fn class(&self, node: usize) -> Option<ExpressionLeafClassV1> {
        self.evaluation.classes.get(node).copied()
    }
    pub(crate) fn transfers_node(&self, node: usize) -> bool {
        self.evaluation.transfers_node(node)
    }
    /// These original leaves are observations owned by one checked comparison,
    /// not scalar-producing nodes or separate Local acquisitions.
    pub(crate) fn comparison_input_children_v1(&self, node: usize) -> Option<[usize; 2]> {
        let plan = self.evaluation.comparison_plan_v1(node).ok()??;
        (matches!(
            plan.kind(),
            ComparisonExecutionKindV1::Text | ComparisonExecutionKindV1::Checked { .. }
        ) || self
            .evaluation
            .bool_condition_comparison_input_order_v1(node)
            .is_some())
        .then_some(plan.operands()?)
    }

    pub(crate) fn scalar_invocation_input_nodes_v1(&self) -> BTreeSet<usize> {
        self.ordered_nodes()
            .iter()
            .filter_map(|node| self.evaluation.scalar_invoke_input_order_v1(*node))
            .flatten()
            .collect()
    }
}

/// Every source/operator/call/refusal check precedes the last strict field
/// batch. Only a complete candidate may enter the existing Home inventory.
pub(crate) fn qualify_bool_condition_v1<E>(
    source: Rc<ExpressionSourcePlanV1>,
    prior_homes: &[LiveHomeV1],
    static_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::StaticTextReadV1>,
    residence_reference: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<(
        BindingRefV1,
        BindingRefV1,
        TextResidenceAcquisitionV1,
    )>,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<(BindingRefV1, SourceScalarKind)>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    source_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<LocalCallObservationV1>, E>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        super::super::LocalFieldReadDemandV1,
    ) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    if !matches!(source.root(), ExpressionPlanRootV1::IfCondition { .. }) {
        return Ok(None);
    }
    let Some(prepared) = super::leaves::prepare_expression_calls_v1(
        &source,
        ExpressionLeafProfileV1::BoolCondition,
        &BTreeMap::new(),
        prior_homes,
        &mut |site| {
            source_call(site).map(|call| call.map(super::leaves::ExpressionCallSourceV1::Ordinary))
        },
    )?
    else {
        return Ok(None);
    };
    qualify_condition_expression_v1(
        prepared,
        &[],
        static_reference,
        residence_reference,
        &mut |site| {
            scalar_reference(site)
                .map(|(binding, kind)| CheckedScalarReferenceV1::Obligation { binding, kind })
        },
        field_request,
        can_install,
        field_batch,
    )
}

/// Preserve the genuine If/Loop purpose and exact original scalar proofs.
/// This shares qualification only; it grants no outer control-flow completion.
pub(crate) fn qualify_condition_expression_v1<E>(
    prepared: super::leaves::PreparedExpressionCallsV1,
    initial_references: &[(usize, CheckedExpressionReferenceV1)],
    static_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::StaticTextReadV1>,
    residence_reference: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<(
        BindingRefV1,
        BindingRefV1,
        TextResidenceAcquisitionV1,
    )>,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<CheckedScalarReferenceV1>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        super::super::LocalFieldReadDemandV1,
    ) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    if !prepared.is_bool_condition() {
        return Ok(None);
    }
    let source = Rc::clone(prepared.source());
    let prior_homes = prepared.prior_homes().to_vec();
    use ExpressionLeafClassV1 as Leaf;
    use ExpressionValueClassV1 as Class;
    if !source.admits_prior_homes_v1(&prior_homes) {
        return Ok(None);
    }
    let condition = match source.root() {
        ExpressionPlanRootV1::IfCondition { condition, .. }
        | ExpressionPlanRootV1::LoopCondition { condition, .. } => condition,
        _ => return Ok(None),
    };
    if source
        .nodes()
        .first()
        .is_none_or(|node| node.site() != condition)
    {
        return Ok(None);
    }
    let Ok((evaluation, fields, logical)) = super::qualification::qualify_expression_evaluation_v1(
        prepared,
        initial_references.to_vec(),
        static_reference,
        residence_reference,
        &mut |_| None,
        scalar_reference,
        field_request,
    )?
    else {
        return Ok(None);
    };
    if evaluation.root_class() != Leaf::Value(Class::Bool)
        || evaluation.calls().iter().any(|call| {
            !matches!(
                call.result(),
                LocalCallResultClassV1::I64
                    | LocalCallResultClassV1::Bool
                    | LocalCallResultClassV1::Dynamic
            )
        })
    {
        return Ok(None);
    }
    let frame = Rc::new(CheckedBoolConditionFrameV1 {
        evaluation,
        logical: logical.into_boxed_slice(),
    });
    let mut input_owners = BTreeMap::new();
    for &node in frame.ordered_nodes() {
        let required_carrier_comparison = frame
            .operations()
            .iter()
            .find(|loan| loan.node() == node)
            .and_then(|loan| loan.single_envelope())
            .is_some_and(|envelope| {
                envelope.domain().family() == DynamicOperatorFamilyV1::Greater
                    && envelope.domain().left() == DynamicOperatorValueClassV1::Dynamic
                    && envelope.domain().right() == DynamicOperatorValueClassV1::I64
            });
        if required_carrier_comparison
            && !matches!(frame.evaluation().comparison_plan_v1(node), Ok(Some(_)))
        {
            return Ok(None);
        }
        let scalar_core = frame.evaluation().cores().get(&node).is_some_and(|core| {
            core.field_map().is_some() && core.source().target().checked_map_presence_law_v1()
        });
        if frame.comparison_input_children_v1(node).is_none() && !scalar_core {
            continue;
        }
        let Some(inputs) = frame.evaluation().scalar_invoke_input_order_v1(node) else {
            return Ok(None);
        };
        for input in inputs {
            if input_owners.insert(input, node).is_some() {
                return Ok(None);
            }
        }
    }
    if frame.evaluation().cores().iter().any(|(node, core)| {
        core.source().target().frame_free_read_law_v1().is_none()
            && !input_owners.contains_key(node)
            && !(core.field_map().is_some()
                && core.source().target().checked_map_presence_law_v1()
                && frame
                    .evaluation()
                    .scalar_invoke_input_order_v1(*node)
                    .is_some())
            // A direct-receiver field Array Count is an Integer operand of its
            // comparison; it is read by the checked field-array writer.
            && !core.field_array_access_v1().is_some_and(|access| access.is_count() && access.receiver_alias_v1().is_none())
    }) || frame
        .evaluation()
        .owned_result_nodes()
        .iter()
        .any(|node| !input_owners.contains_key(node))
    {
        return Ok(None);
    }

    let candidate = HomeExpressionPlanV1 {
        condition_frame: Some(super::ConditionSourceV1::Frame(frame)),
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
