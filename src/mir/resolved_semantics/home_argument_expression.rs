//! Completed semantic argument qualification at the real covered Prefix point.
//! Mandatory entry requirements remain obligations, not physical descriptors.
use super::*;
use super::super::{BindingRefV1, LocalCallObservationV1, OwnedExprSiteV1, SourceExprSiteV1, SourceScalarKind};
use crate::mir::normal_callable_semantic_package::{ArgumentExpressionSourceLoanV1, BorrowedTextEntryReadRequirementV1};
use crate::mir::resolved_value_profile::{expression_plan::ExpressionPlanRootV1,
    expression_children::{ExpressionChildrenV1, ExpressionChildReachV1},
    expression_class::ExpressionValueClassV1 as Class, expression_projection::ExpressionLeafClassV1 as Leaf};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedArgumentExpressionFrameV1 {
    evaluation: CheckedExpressionEvaluationV1,
    core: ArgumentExpressionSourceLoanV1,
    text_reads: Box<[BorrowedTextEntryReadRequirementV1]>,
    references: BTreeMap<usize, (BindingRefV1, SourceScalarKind)>,
}
impl CheckedArgumentExpressionFrameV1 {
    pub(crate) fn evaluation(&self) -> &CheckedExpressionEvaluationV1 { &self.evaluation }
    pub(crate) fn core(&self) -> &ArgumentExpressionSourceLoanV1 { &self.core }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> { self.evaluation.source() }
    pub(crate) fn prior_homes(&self) -> &[BindingRefV1] { self.evaluation.prior_homes() }
    pub(crate) fn text_reads(&self) -> &[BorrowedTextEntryReadRequirementV1] { &self.text_reads }
    pub(crate) fn references(&self) -> &BTreeMap<usize, (BindingRefV1, SourceScalarKind)> { &self.references }
    pub(crate) fn transfers_node(&self, node: usize) -> bool { self.evaluation.transfers_node(node) }
}

pub(crate) fn qualify_argument_expression_v1<E>(
    core: ArgumentExpressionSourceLoanV1, statement: &SourceStmtSiteV1, prior_homes: &[BindingRefV1],
    text_reads: &[BorrowedTextEntryReadRequirementV1],
    text_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<(BindingRefV1, BindingRefV1)>,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<(BindingRefV1, SourceScalarKind)>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    source_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<LocalCallObservationV1>, E>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(&[LocalFieldReadRequestV1], super::super::LocalFieldReadDemandV1) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    let source = Rc::clone(core.plan());
    let ExpressionPlanRootV1::CallArgument { statement: original, call, ordinal } = source.root() else { return Ok(None) };
    let method = core.resolver_source().source();
    if original != statement || call.owner() != source.owner() || method.owner() != source.owner()
        || call.arguments().get(*ordinal as usize).is_none_or(|arg| arg.ordinal() != *ordinal || arg.site() != method.site())
        || source.nodes().first().is_none_or(|node| node.site() != method.site() || !matches!(node.source(), ExpressionNodeSourceV1::MethodCall)) {
        return Ok(None);
    }
    let mut homes = BTreeSet::new();
    if prior_homes.iter().any(|home| home.owner() != source.owner() || !homes.insert(*home)) { return Ok(None); }
    let Ok(order) = source.ordered_postorder_v1() else { return Ok(None) };
    if source.nodes().iter().any(|node| !matches!(node.children(), ExpressionChildrenV1::Known(children)
        if children.iter().all(|child| child.reach == ExpressionChildReachV1::Always))) { return Ok(None); }
    let mut seeded = BTreeMap::from([(0, Leaf::Value(Class::String))]);
    for read in text_reads {
        if !Rc::ptr_eq(read.source_plan(), &source) || !core.text_operand_is(read.read())
            || text_reference(read.read()) != Some((read.binding(), read.formal().1)) { return Ok(None); }
        let index = read.node();
        if source.nodes().get(index).is_none_or(|node| node.site() != read.read()
            || !matches!(node.source(), ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if *binding == read.binding()))
            || seeded.insert(index, Leaf::Value(Class::String)).is_some() { return Ok(None); }
    }
    let Some(super::leaves::QualifiedExpressionLeavesV1 { classes, mut calls, fields, transferred, references }) =
        super::leaves::qualify_expression_leaves_v1(&source, super::leaves::ExpressionLeafProfileV1::Argument, &seeded,
            &mut |site| {
                if !core.signed_operand_contains(site) { return None; }
                scalar_reference(site).filter(|(_, kind)| *kind == SourceScalarKind::Integer)
            }, prior_homes, &order, field_request, source_call)? else { return Ok(None) };
    if classes[0] != Leaf::Value(Class::String) || transferred.contains(&0) { return Ok(None); }
    let Some((operations, mut owned_results)) = super::operations::qualify_add_operations_v1(&source, &order, &classes, &transferred) else { return Ok(None) };
    // The source plan lists acquisition templates; only physical Normal acquires.
    owned_results.push(0);
    let frame = Rc::new(CheckedArgumentExpressionFrameV1 {
        evaluation: CheckedExpressionEvaluationV1 { source: Rc::clone(&source), prior_homes: prior_homes.into(), order: order.clone().into_boxed_slice(),
            classes: classes.into_boxed_slice(), calls: order.iter().filter_map(|node| calls.remove(node)).collect(),
            operations: operations.into_boxed_slice(), owned_results: owned_results.into_boxed_slice(), transferred_inputs: transferred },
        core, text_reads: text_reads.into(), references,
    });
    let candidate = HomeExpressionPlanV1 { source: Rc::clone(&source), scalar_operations: Vec::new(), source_fields: Vec::new(),
        print_frame: None, condition_frame: None, argument_frame: Some(frame), text_initializer: None, field_write: None, initializer_homes: None };
    let Some(candidate) = candidate.with_source_field_requirements(&fields) else { return Ok(None) };
    if !can_install(&candidate) { return Ok(None); }
    if !field_batch(&fields, super::super::LocalFieldReadDemandV1::SourceExpression { plan: source })? { return Ok(None); }
    Ok(Some(candidate))
}
