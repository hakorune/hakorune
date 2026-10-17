//! Original Text initializer purpose over the shared checked expression body.
use super::*;
use super::super::{BindingRefV1, LocalCallObservationV1, OwnedExprSiteV1, SourceExprSiteV1, SourceScalarKind};
use crate::mir::normal_callable_semantic_package::InitializerTextEntryReadV1;
use crate::mir::resolved_value_profile::{expression_plan::ExpressionPlanRootV1,
    expression_children::{ExpressionChildrenV1, ExpressionChildReachV1},
    expression_class::ExpressionValueClassV1 as Class, expression_projection::ExpressionLeafClassV1 as Leaf};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub(crate) struct TextResidenceReadV1 {
    node: usize, binding: BindingRefV1, root: BindingRefV1,
    acquisition: Rc<CheckedTextInitializerFrameV1>,
}
impl PartialEq for TextResidenceReadV1 {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.binding == other.binding && self.root == other.root
            && Rc::ptr_eq(&self.acquisition, &other.acquisition)
    }
}
impl Eq for TextResidenceReadV1 {}
impl TextResidenceReadV1 {
    pub(crate) fn node(&self) -> usize { self.node }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn root(&self) -> BindingRefV1 { self.root }
    pub(crate) fn acquisition(&self) -> &Rc<CheckedTextInitializerFrameV1> { &self.acquisition }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedTextInitializerFrameV1 {
    evaluation: CheckedExpressionEvaluationV1,
    entry_reads: Box<[InitializerTextEntryReadV1]>,
    entry_refinements: BTreeMap<usize, Rc<crate::mir::resolved_semantics::home_new_prefix::EntryNullExclusionV1>>,
    residence_reads: Box<[TextResidenceReadV1]>,
    scalar_reads: BTreeMap<usize, (BindingRefV1, SourceScalarKind)>,
}
impl CheckedTextInitializerFrameV1 {
    pub(crate) fn evaluation(&self) -> &CheckedExpressionEvaluationV1 { &self.evaluation }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> { self.evaluation.source() }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.source().root().initializer().expect("original initializer").binding() }
    pub(crate) fn prior_homes(&self) -> &[BindingRefV1] { self.evaluation.prior_homes() }
    pub(crate) fn entry_reads(&self) -> &[InitializerTextEntryReadV1] { &self.entry_reads }
    pub(crate) fn entry_refinements(&self) -> &BTreeMap<usize, Rc<crate::mir::resolved_semantics::home_new_prefix::EntryNullExclusionV1>> { &self.entry_refinements }
    pub(crate) fn residence_reads(&self) -> &[TextResidenceReadV1] { &self.residence_reads }
    pub(crate) fn scalar_reads(&self) -> &BTreeMap<usize, (BindingRefV1, SourceScalarKind)> { &self.scalar_reads }
    pub(crate) fn owns_root(&self) -> bool { self.evaluation.owned_result_nodes().contains(&0) }
    pub(crate) fn transfers_node(&self, node: usize) -> bool { self.evaluation.transfers_node(node) }
}

pub(crate) fn qualify_text_initializer_v1<E>(
    source: Rc<ExpressionSourcePlanV1>, prior_homes: &[BindingRefV1],
    entry_reads: &[InitializerTextEntryReadV1],
    entry_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<(BindingRefV1, BindingRefV1)>,
    entry_refinement: &mut impl FnMut(&SourceExprSiteV1, BindingRefV1, BindingRefV1) -> Option<Rc<crate::mir::resolved_semantics::home_new_prefix::EntryNullExclusionV1>>,
    residence_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<(BindingRefV1, BindingRefV1, Rc<CheckedTextInitializerFrameV1>)>,
    signed_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<BindingRefV1>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    source_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<LocalCallObservationV1>, E>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(&[LocalFieldReadRequestV1], super::super::LocalFieldReadDemandV1) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    let ExpressionPlanRootV1::LocalInitializer { initializer, .. } = source.root() else { return Ok(None) };
    if initializer.binding().owner() != source.owner() || source.nodes().first().map(|node| node.site()) != initializer.initializer_site() { return Ok(None); }
    let mut homes = BTreeSet::new();
    if prior_homes.iter().any(|home| home.owner() != source.owner() || !homes.insert(*home)) { return Ok(None); }
    let Ok(order) = source.ordered_postorder_v1() else { return Ok(None) };
    if source.nodes().iter().any(|node| !matches!(node.children(), ExpressionChildrenV1::Known(children)
        if children.iter().all(|child| child.reach == ExpressionChildReachV1::Always))) { return Ok(None); }
    let mut seeded = BTreeMap::new();
    let mut entry_refinements = BTreeMap::new();
    for read in entry_reads {
        let requirement = read.requirement();
        let refinement = entry_refinement(requirement.read(), requirement.binding(), requirement.formal().1);
        let Some(class) = read.leaf_class(refinement.as_deref()) else { return Ok(None) };
        if !Rc::ptr_eq(requirement.source_plan(), &source)
            || entry_reference(requirement.read()) != Some((requirement.binding(), requirement.formal().1))
            || source.nodes().get(requirement.node()).is_none_or(|node| node.site() != requirement.read()
                || !matches!(node.source(), ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if *binding == requirement.binding()))
            || seeded.insert(requirement.node(), class).is_some() { return Ok(None); }
        if let Some(refinement) = refinement { entry_refinements.insert(requirement.node(), refinement); }
    }
    let mut residence_reads = Vec::new();
    for (node, original) in source.nodes_outside_calls() {
        let ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) = original.source() else { continue };
        let Some((current, root, acquisition)) = residence_reference(original.site()) else { continue };
        if current != *binding || root != acquisition.binding() || acquisition.source().owner() != source.owner()
            || (acquisition.owns_root() && !homes.contains(&root))
            || seeded.insert(node, Leaf::Value(Class::String)).is_some() { return Ok(None); }
        residence_reads.push(TextResidenceReadV1 { node, binding: current, root, acquisition });
    }
    let Some(super::leaves::QualifiedExpressionLeavesV1 { classes, mut calls, fields, transferred, references }) =
        super::leaves::qualify_expression_leaves_v1(&source, super::leaves::ExpressionLeafProfileV1::TextInitializer, &seeded,
            &mut |site| signed_reference(site).map(|binding| (binding, SourceScalarKind::Integer)), prior_homes, &order, field_request, source_call)? else { return Ok(None) };
    if classes[0] != Leaf::Value(Class::String) || transferred.contains(&0) { return Ok(None); }
    let Some((operations, owned_results)) = super::operations::qualify_add_operations_v1(&source, &order, &classes, &transferred) else { return Ok(None) };
    // This purpose transfers an owned result into a local residence. Bare
    // literals/entry aliases keep their existing non-owning initializer owner.
    if !owned_results.contains(&0) { return Ok(None); }
    let entry_reads = entry_reads.iter().filter(|read| !transferred.contains(&read.requirement().node())).cloned().collect();
    let residence_reads = residence_reads.into_iter().filter(|read| !transferred.contains(&read.node)).collect();
    let frame = Rc::new(CheckedTextInitializerFrameV1 {
        evaluation: CheckedExpressionEvaluationV1 { source: Rc::clone(&source), prior_homes: prior_homes.into(), order: order.clone().into_boxed_slice(),
            classes: classes.into_boxed_slice(), calls: order.iter().filter_map(|node| calls.remove(node)).collect(),
            operations: operations.into_boxed_slice(), owned_results: owned_results.into_boxed_slice(), transferred_inputs: transferred },
        entry_reads, entry_refinements, residence_reads, scalar_reads: references,
    });
    let candidate = HomeExpressionPlanV1 { source: Rc::clone(&source), scalar_operations: Vec::new(), source_fields: Vec::new(),
        print_frame: None, condition_frame: None, argument_frame: None, text_initializer: Some(frame), field_write: None, initializer_homes: None };
    let Some(candidate) = candidate.with_source_field_requirements(&fields) else { return Ok(None) };
    if !can_install(&candidate) { return Ok(None); }
    if !field_batch(&fields, super::super::LocalFieldReadDemandV1::SourceExpression { plan: source })? { return Ok(None); }
    Ok(Some(candidate))
}
