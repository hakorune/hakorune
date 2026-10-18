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
    source_loan: ArgumentExpressionSourceLoanV1,
}
impl CheckedArgumentExpressionFrameV1 {
    pub(crate) fn is_i64(&self) -> bool { self.evaluation.root_class() == Leaf::Value(Class::I64) }
    pub(crate) fn evaluation(&self) -> &CheckedExpressionEvaluationV1 { &self.evaluation }
    pub(crate) fn source_loan(&self) -> &ArgumentExpressionSourceLoanV1 { &self.source_loan }
    pub(crate) fn static_reads(&self) -> Vec<&super::super::StaticTextReadV1> { self.evaluation.references().values().filter_map(|read| match read { CheckedExpressionReferenceV1::StaticText(read) => Some(read), _ => None }).collect() }
    pub(crate) fn residence_reads(&self) -> Vec<&super::initializer_expression::TextResidenceReadV1> { self.evaluation.references().values().filter_map(|read| match read { CheckedExpressionReferenceV1::Residence(read) => Some(read), _ => None }).collect() }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> { self.evaluation.source() }
    pub(crate) fn prior_homes(&self) -> &[BindingRefV1] { self.evaluation.prior_homes() }
    pub(crate) fn text_reads(&self) -> Vec<&BorrowedTextEntryReadRequirementV1> { self.evaluation.references().values().filter_map(|read| match read { CheckedExpressionReferenceV1::ArgumentEntry(read) => Some(read), _ => None }).collect() }
    pub(crate) fn references(&self) -> BTreeMap<usize, (BindingRefV1, SourceScalarKind)> { self.evaluation.references().iter().filter_map(|(&node, read)| match read { CheckedExpressionReferenceV1::Scalar(read) => Some((node, (read.binding(), read.kind()))), _ => None }).collect() }
    pub(crate) fn transfers_node(&self, node: usize) -> bool { self.evaluation.transfers_node(node) }
}

pub(crate) fn qualify_argument_expression_v1<E>(
    core: ArgumentExpressionSourceLoanV1, statement: &SourceStmtSiteV1, prior_homes: &[BindingRefV1],
    text_reads: &[BorrowedTextEntryReadRequirementV1],
    text_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<(BindingRefV1, BindingRefV1)>,
    static_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::StaticTextReadV1>,
    residence_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<(BindingRefV1, BindingRefV1, Rc<CheckedInitializerExpressionFrameV1>)>,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::CurrentScalarReadV1>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    source_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<super::leaves::ExpressionCallSourceV1>, E>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(&[LocalFieldReadRequestV1], super::super::LocalFieldReadDemandV1) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    let source = Rc::clone(core.plan());
    let ExpressionPlanRootV1::CallArgument { statement: original, call, ordinal } = source.root() else { return Ok(None) };
    let Some(root) = source.nodes().first() else { return Ok(None) };
    if original != statement || call.owner() != source.owner()
        || call.arguments().get(*ordinal as usize).is_none_or(|arg| arg.ordinal() != *ordinal || arg.site() != root.site()) {
        return Ok(None);
    }
    if let Some(resolver) = core.resolver_source() {
        let method = resolver.source();
        if method.owner() != source.owner() || root.site() != method.site()
            || !matches!(root.source(), ExpressionNodeSourceV1::MethodCall) { return Ok(None); }
    } else if !core.requires_i64_v1() && !matches!(root.source(), ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::String(_))
        | ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(_))) { return Ok(None); }
    let mut homes = BTreeSet::new();
    if prior_homes.iter().any(|home| home.owner() != source.owner() || !homes.insert(*home)) { return Ok(None); }
    let Ok(order) = source.ordered_postorder_v1() else { return Ok(None) };
    if source.nodes().iter().any(|node| !matches!(node.children(), ExpressionChildrenV1::Known(children)
        if children.iter().all(|child| child.reach == ExpressionChildReachV1::Always))) { return Ok(None); }
    let mut seeded = BTreeMap::new();

    let mut static_reads = Vec::new();
    let mut residence_reads = Vec::new();
    for (index, node) in source.nodes().iter().enumerate() {
        let ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) = node.source() else { continue };
        if !core.text_operand_is(node.site()) { continue; }
        if let Some(read) = static_reference(node.site()) {
            if read.read() != node.site() || read.binding() != *binding
                || seeded.insert(index, Leaf::Value(Class::String)).is_some() { return Ok(None); }
            static_reads.push(read);
        } else if let Some((current, root, acquisition)) = residence_reference(node.site()) {
            if current != *binding || root != acquisition.binding() || acquisition.source().owner() != source.owner()
                || (acquisition.owns_root() && !homes.contains(&root))
                || seeded.insert(index, Leaf::Value(Class::String)).is_some() { return Ok(None); }
            residence_reads.push(super::initializer_expression::TextResidenceReadV1::new(index, current, root, acquisition));
        }
    }
    for read in text_reads {
        if !Rc::ptr_eq(read.source_plan(), &source) || !core.text_operand_is(read.read())
            || text_reference(read.read()) != Some((read.binding(), read.formal().1)) { return Ok(None); }
        let index = read.node();
        if source.nodes().get(index).is_none_or(|node| node.site() != read.read()
            || !matches!(node.source(), ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if *binding == read.binding()))
            || seeded.insert(index, Leaf::Value(Class::String)).is_some() { return Ok(None); }
    }
    let Some(mut prepared) = super::leaves::prepare_expression_calls_v1(&source,
        super::leaves::ExpressionLeafProfileV1::Argument, &seeded, prior_homes, source_call)? else { return Ok(None) };
    let mut scalar_proofs = BTreeMap::new();
    for (node, original) in source.nodes().iter().enumerate() {
        if !prepared.is_active_reference(node) || !core.signed_operand_contains(original.site()) { continue; }
        let Some(read) = scalar_reference(original.site()) else { continue };
        let proof = CheckedExpressionReferenceV1::Scalar(CheckedScalarReferenceV1::Original(read));
        if !proof.matches_node(&source, node) || !prepared.seed_reference(node, Leaf::Value(Class::I64))
            || scalar_proofs.insert(node, proof).is_some() { return Ok(None); }
    }
    let Some(super::leaves::QualifiedExpressionLeavesV1 { classes, mut calls, cores, fields, transferred, references }) =
        super::leaves::qualify_expression_leaves_with_calls_v1::<E>(prepared,
            &mut |_| None, &order, field_request)? else { return Ok(None) };
    let expected = if core.requires_i64_v1() { Class::I64 } else { Class::String };
    if classes[0] != Leaf::Value(expected) || transferred.contains(&0) || !references.is_empty() { return Ok(None); }
    let Some((operations, owned_results)) = super::operations::qualify_add_operations_v1(&source, &order, &classes, &transferred) else { return Ok(None) };
    if core.resolver_source().is_some_and(|original| !cores.get(&0)
        .is_some_and(|retained| std::ptr::eq(retained.source(), original))) { return Ok(None); }
    let mut reference_proofs = scalar_proofs;
    for read in static_reads {
        let Some(node) = source.nodes().iter().position(|node| node.site() == read.read()) else { return Ok(None) };
        if !transferred.contains(&node) && reference_proofs.insert(node, CheckedExpressionReferenceV1::StaticText(read)).is_some() { return Ok(None); }
    }
    for read in residence_reads {
        if !transferred.contains(&read.node()) && reference_proofs.insert(read.node(), CheckedExpressionReferenceV1::Residence(read)).is_some() { return Ok(None); }
    }
    for read in text_reads {
        if !transferred.contains(&read.node()) && reference_proofs.insert(read.node(), CheckedExpressionReferenceV1::ArgumentEntry(read.clone())).is_some() { return Ok(None); }
    }
    reference_proofs.retain(|node, _| !transferred.contains(node));
    if reference_proofs.iter().any(|(&node, proof)| !proof.matches_node(&source, node)) { return Ok(None); }
    let frame = Rc::new(CheckedArgumentExpressionFrameV1 {
        evaluation: CheckedExpressionEvaluationV1 { source: Rc::clone(&source), prior_homes: prior_homes.into(), order: order.clone().into_boxed_slice(),
            classes: classes.into_boxed_slice(), references: reference_proofs, calls: order.iter().filter_map(|node| calls.remove(node)).collect(),
            operations: operations.into_boxed_slice(), owned_results: super::evaluation::owned_result_nodes(&order, &owned_results, &cores), cores, transferred_inputs: transferred },
        source_loan: core,
    });
    if !frame.evaluation().reference_coverage_is_exact() { return Ok(None); }
    let candidate = HomeExpressionPlanV1 { source: Rc::clone(&source), scalar_operations: Vec::new(), source_fields: Vec::new(),
        print_frame: None, condition_frame: None, argument_frame: Some(frame), initializer_frame: None, rebind_frame: None, field_write: None, initializer_homes: None };
    let Some(candidate) = candidate.with_source_field_requirements(&fields) else { return Ok(None) };
    if !can_install(&candidate) { return Ok(None); }
    if !field_batch(&fields, super::super::LocalFieldReadDemandV1::SourceExpression { plan: source })? { return Ok(None); }
    Ok(Some(candidate))
}
