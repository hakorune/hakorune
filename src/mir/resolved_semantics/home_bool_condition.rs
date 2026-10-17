//! Whole Bool condition qualification on the same immutable expression source.
//! Conditional reach is retained; this owner never emits or eagerly runs RHS.
use super::*;
use super::leaves::{ExpressionLeafProfileV1, QualifiedExpressionLeavesV1};
use super::super::{BindingRefV1, LocalCallObservationV1, LocalCallResultClassV1, OwnedExprSiteV1, SourceExprSiteV1, SourceScalarKind};
use crate::mir::dynamic_operator_contract::{
    issue_dynamic_operator_execution_envelope_v1, DynamicOperatorDomainV1,
    DynamicOperatorFamilyV1, DynamicOperatorValueClassV1,
};
use crate::mir::resolved_value_profile::{
    expression_children::{ExpressionChildReachV1, ExpressionChildrenV1},
    expression_class::ExpressionValueClassV1,
    expression_plan::ExpressionPlanRootV1,
    expression_projection::ExpressionLeafClassV1,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedBoolConditionFrameV1 {
    source: Rc<ExpressionSourcePlanV1>,
    prior_homes: Box<[BindingRefV1]>,
    order: Box<[usize]>,
    classes: Box<[ExpressionLeafClassV1]>,
    calls: Box<[LocalCallObservationV1]>,
    operations: Box<[Rc<CheckedExpressionOperationLoanV1>]>,
    logical: Box<[usize]>,
    references: BTreeMap<usize, (BindingRefV1, SourceScalarKind)>,
    transferred_inputs: BTreeSet<usize>,
}
impl CheckedBoolConditionFrameV1 {
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> { &self.source }
    pub(crate) fn prior_homes(&self) -> &[BindingRefV1] { &self.prior_homes }
    pub(crate) fn ordered_nodes(&self) -> &[usize] { &self.order }
    pub(crate) fn calls(&self) -> &[LocalCallObservationV1] { &self.calls }
    pub(crate) fn operations(&self) -> &[Rc<CheckedExpressionOperationLoanV1>] { &self.operations }
    pub(crate) fn logical_nodes(&self) -> &[usize] { &self.logical }
    pub(crate) fn references(&self) -> &BTreeMap<usize, (BindingRefV1, SourceScalarKind)> { &self.references }
    pub(crate) fn class(&self, node: usize) -> Option<ExpressionLeafClassV1> { self.classes.get(node).copied() }
    pub(crate) fn transfers_node(&self, node: usize) -> bool { self.transferred_inputs.contains(&node) }
}

/// Every source/operator/call/refusal check precedes the last strict field
/// batch. Only a complete candidate may enter the existing Home inventory.
pub(crate) fn qualify_bool_condition_v1<E>(
    source: Rc<ExpressionSourcePlanV1>, prior_homes: &[BindingRefV1],
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<(BindingRefV1, SourceScalarKind)>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    source_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<LocalCallObservationV1>, E>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(&[LocalFieldReadRequestV1], super::super::LocalFieldReadDemandV1) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    use ExpressionLeafClassV1 as Leaf;
    use ExpressionValueClassV1 as Class;
    use ResolvedBinaryOperatorV1 as Op;
    let mut homes = BTreeSet::new();
    if prior_homes.iter().any(|home| home.owner() != source.owner() || !homes.insert(*home)) { return Ok(None); }
    let ExpressionPlanRootV1::IfCondition { condition, .. } = source.root() else { return Ok(None) };
    if source.nodes().first().is_none_or(|node| node.site() != condition) { return Ok(None); }
    let Ok(order) = source.ordered_postorder_v1() else { return Ok(None) };
    let Some(QualifiedExpressionLeavesV1 { classes, mut calls, fields, transferred, references }) =
        super::leaves::qualify_expression_leaves_v1(&source, ExpressionLeafProfileV1::BoolCondition,
            scalar_reference, prior_homes, &order, field_request, source_call)? else { return Ok(None) };
    if classes[0] != Leaf::Value(Class::Bool) { return Ok(None); }
    let mut operations = Vec::new();
    let mut logical = Vec::new();
    for &index in &order {
        if transferred.contains(&index) { continue; }
        let node = &source.nodes()[index];
        let ExpressionChildrenV1::Known(children) = node.children() else { return Ok(None) };
        let Some(binary) = node.binary() else {
            if children.iter().any(|child| child.reach != ExpressionChildReachV1::Always) { return Ok(None); }
            continue;
        };
        let Some([lhs, rhs]) = crate::mir::resolved_value_profile::expression_children::binary_children_v1(node.site(), binary)
            else { return Ok(None) };
        if children.len() != 2 || children[0].site != *lhs || children[1].site != *rhs
            || children[0].reach != ExpressionChildReachV1::Always { return Ok(None); }
        if matches!(binary.operator(), Op::And | Op::Or) {
            if children[1].reach != (ExpressionChildReachV1::When {
                condition: lhs.clone(), value: binary.operator() == Op::And,
            }) || classes[index] != Leaf::Value(Class::Bool) { return Ok(None); }
            logical.push(index);
            continue;
        }
        if children[1].reach != ExpressionChildReachV1::Always { return Ok(None); }
        let family = match binary.operator() {
            Op::Equal => DynamicOperatorFamilyV1::Equal,
            Op::NotEqual => DynamicOperatorFamilyV1::NotEqual,
            Op::Less => DynamicOperatorFamilyV1::Less,
            Op::Greater => DynamicOperatorFamilyV1::Greater,
            Op::LessEqual => DynamicOperatorFamilyV1::LessEqual,
            Op::GreaterEqual => DynamicOperatorFamilyV1::GreaterEqual,
            Op::Add => DynamicOperatorFamilyV1::Add,
            _ => return Ok(None),
        };
        let integer = |site| source.nodes().iter().position(|node| node.site() == site)
            .is_some_and(|index| classes[index] == Leaf::Value(Class::I64));
        if !integer(lhs) || !integer(rhs) { return Ok(None); }
        let Ok(envelope) = issue_dynamic_operator_execution_envelope_v1(DynamicOperatorDomainV1::new(
            family, DynamicOperatorValueClassV1::NormalInteger, DynamicOperatorValueClassV1::NormalInteger,
        )) else { return Ok(None) };
        operations.push(Rc::new(CheckedExpressionOperationLoanV1 { plan: Rc::clone(&source), node: index, envelope }));
    }
    // Primitive result loans cannot introduce a conditional owning temporary.
    if calls.values().any(|call| !matches!(call.result(), LocalCallResultClassV1::I64 | LocalCallResultClassV1::Bool)) {
        return Ok(None);
    }
    let frame = Rc::new(CheckedBoolConditionFrameV1 {
        source: Rc::clone(&source), prior_homes: prior_homes.into(), order: order.clone().into_boxed_slice(),
        classes: classes.into_boxed_slice(), calls: order.iter().filter_map(|node| calls.remove(node)).collect(),
        operations: operations.into_boxed_slice(), logical: logical.into_boxed_slice(), references, transferred_inputs: transferred,
    });
    let candidate = HomeExpressionPlanV1 {
        source: Rc::clone(&source), scalar_operations: Vec::new(), source_fields: Vec::new(),
        print_frame: None, condition_frame: Some(frame), borrowed_field_write: None,
    };
    let Some(candidate) = candidate.with_source_field_requirements(&fields) else { return Ok(None) };
    if !can_install(&candidate) { return Ok(None); }
    if !field_batch(&fields, super::super::LocalFieldReadDemandV1::SourceExpression { plan: source })? { return Ok(None); }
    Ok(Some(candidate))
}
