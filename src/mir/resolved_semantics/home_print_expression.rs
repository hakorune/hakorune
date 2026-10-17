//! Whole Print qualification over the shared original expression plan.
//! No physical IDs, runtime tokens or independent call inventory are issued.
use super::*;
use super::super::{LocalCallObservationV1, OwnedExprSiteV1, SourceExprSiteV1};
use crate::mir::resolved_value_profile::{
    expression_children::{ExpressionChildReachV1, ExpressionChildrenV1},
    expression_class::ExpressionValueClassV1,
    expression_plan::ExpressionPlanRootV1,
    expression_projection::ExpressionLeafClassV1,
    expression_root::StatementExpressionRoleV1,
};
use std::collections::BTreeSet;

/// Print purpose authority wraps the SAME qualified expression body. The body
/// alone neither selects a Print root nor installs a Home observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedPrintExpressionFrameV1 {
    evaluation: CheckedExpressionEvaluationV1,
}
impl CheckedPrintExpressionFrameV1 {
    pub(crate) fn evaluation(&self) -> &CheckedExpressionEvaluationV1 { &self.evaluation }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> { self.evaluation.source() }
    pub(crate) fn prior_homes(&self) -> &[crate::mir::resolved_semantics::BindingRefV1] { self.evaluation.prior_homes() }
    pub(crate) fn ordered_nodes(&self) -> &[usize] { self.evaluation.ordered_nodes() }
    pub(crate) fn calls(&self) -> &[LocalCallObservationV1] { self.evaluation.calls() }
    pub(crate) fn operations(&self) -> &[Rc<CheckedExpressionOperationLoanV1>] { self.evaluation.operations() }
    pub(crate) fn owned_result_nodes(&self) -> &[usize] { self.evaluation.owned_result_nodes() }
    pub(crate) fn root_class(&self) -> ExpressionLeafClassV1 { self.evaluation.root_class() }
    pub(crate) fn transfers_node(&self, node: usize) -> bool { self.evaluation.transfers_node(node) }
    pub(crate) fn node_is_identity(&self, node: usize) -> bool { self.evaluation.node_is_identity(node) }
}

/// The caller first retains original actual facts at the pre-statement basis.
/// Call demands below are passive: none may publish Home call observations.
/// Finish every rejectable root/envelope/install check before the strict field
/// batch; after that final atomic batch succeeds, returning the frame cannot fail.
pub(crate) fn qualify_print_expression_v1<E>(
    source: Rc<ExpressionSourcePlanV1>,
    prior_homes: &[crate::mir::resolved_semantics::BindingRefV1],
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    source_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<LocalCallObservationV1>, E>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(
        &[LocalFieldReadRequestV1], super::super::LocalFieldReadDemandV1,
    ) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    let mut unique_homes = BTreeSet::new();
    if prior_homes.iter().any(|home| home.owner() != source.owner() || !unique_homes.insert(*home)) {
        return Ok(None);
    }
    use ExpressionLeafClassV1 as Leaf;
    use ExpressionValueClassV1 as Class;
    let ExpressionPlanRootV1::Statement { role: StatementExpressionRoleV1::PrintValue, value, .. } = source.root()
        else { return Ok(None) };
    let Some(root) = source.nodes().first() else { return Ok(None) };
    if root.site() != value { return Ok(None); }
    let Ok(order) = source.ordered_postorder_v1() else { return Ok(None) };
    if source.nodes().iter().any(|node| !matches!(node.children(), ExpressionChildrenV1::Known(children)
        if children.iter().all(|child| child.reach == ExpressionChildReachV1::Always))) { return Ok(None); }

    let Some(leaves) = super::leaves::qualify_expression_leaves_v1(
        &source, super::leaves::ExpressionLeafProfileV1::Print, &std::collections::BTreeMap::new(), &mut |_| None,
        prior_homes, &order, field_request, source_call,
    )? else { return Ok(None) };
    let super::leaves::QualifiedExpressionLeavesV1 { classes, mut calls, fields, transferred, .. } = leaves;
    if !matches!(classes[0], Leaf::Value(Class::String | Class::I64)) { return Ok(None); }
    let Some((operations, owned_results)) = super::operations::qualify_add_operations_v1(&source, &order, &classes, &transferred) else { return Ok(None) };
    let frame = Rc::new(CheckedPrintExpressionFrameV1 { evaluation: CheckedExpressionEvaluationV1 {
        source: Rc::clone(&source), prior_homes: prior_homes.into(), order: order.clone().into_boxed_slice(), classes: classes.into_boxed_slice(),
        calls: order.iter().filter_map(|node| calls.remove(node)).collect(), operations: operations.into_boxed_slice(),
        owned_results: owned_results.into_boxed_slice(), transferred_inputs: transferred,
    } });
    let candidate = HomeExpressionPlanV1 {
        source: Rc::clone(&source), scalar_operations: Vec::new(), source_fields: Vec::new(), initializer_homes: None, argument_frame: None, text_initializer: None, print_frame: Some(frame), condition_frame: None, field_write: None,
    };
    let Some(candidate) = candidate.with_source_field_requirements(&fields) else { return Ok(None) };
    if !can_install(&candidate) { return Ok(None); }
    if !field_batch(&fields, super::super::LocalFieldReadDemandV1::SourceExpression { plan: source })? {
        return Ok(None);
    }
    Ok(Some(candidate))
}
