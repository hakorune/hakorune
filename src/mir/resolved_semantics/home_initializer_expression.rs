//! Original Local initializer purpose over the shared checked expression body.
use super::*;
use super::super::{BindingRefV1, SourceExprSiteV1, SourceScalarKind};
use crate::mir::normal_callable_semantic_package::InitializerTextEntryReadV1;
use crate::mir::resolved_value_profile::{expression_plan::ExpressionPlanRootV1,
    expression_children::{ExpressionChildrenV1, ExpressionChildReachV1},
    expression_class::ExpressionValueClassV1 as Class, expression_projection::ExpressionLeafClassV1 as Leaf};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub(crate) struct TextResidenceReadV1 {
    node: usize, binding: BindingRefV1, root: BindingRefV1,
    acquisition: Rc<CheckedInitializerExpressionFrameV1>,
}
impl PartialEq for TextResidenceReadV1 {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.binding == other.binding && self.root == other.root
            && Rc::ptr_eq(&self.acquisition, &other.acquisition)
    }
}
impl Eq for TextResidenceReadV1 {}
impl TextResidenceReadV1 {
    pub(super) fn new(node: usize, binding: BindingRefV1, root: BindingRefV1, acquisition: Rc<CheckedInitializerExpressionFrameV1>) -> Self {
        Self { node, binding, root, acquisition }
    }
    pub(crate) fn node(&self) -> usize { self.node }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn root(&self) -> BindingRefV1 { self.root }
    pub(crate) fn acquisition(&self) -> &Rc<CheckedInitializerExpressionFrameV1> { &self.acquisition }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedInitializerExpressionFrameV1 {
    evaluation: CheckedExpressionEvaluationV1,
}
impl CheckedInitializerExpressionFrameV1 {
    pub(crate) fn evaluation(&self) -> &CheckedExpressionEvaluationV1 { &self.evaluation }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> { self.evaluation.source() }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.source().root().initializer().expect("original initializer").binding() }
    pub(crate) fn prior_homes(&self) -> &[BindingRefV1] { self.evaluation.prior_homes() }
    pub(crate) fn entry_reads(&self) -> Vec<&InitializerTextEntryReadV1> { self.evaluation.references().values().filter_map(|read| match read { CheckedExpressionReferenceV1::InitializerEntry { read, .. } => Some(read), _ => None }).collect() }
    pub(crate) fn entry_refinements(&self) -> BTreeMap<usize, &Rc<crate::mir::resolved_semantics::home_new_prefix::EntryNullExclusionV1>> { self.evaluation.references().iter().filter_map(|(&node, read)| match read { CheckedExpressionReferenceV1::InitializerEntry { exclusion: Some(proof), .. } => Some((node, proof)), _ => None }).collect() }
    pub(crate) fn residence_reads(&self) -> Vec<&TextResidenceReadV1> { self.evaluation.references().values().filter_map(|read| match read { CheckedExpressionReferenceV1::Residence(read) => Some(read), _ => None }).collect() }
    pub(crate) fn scalar_reads(&self) -> BTreeMap<usize, (BindingRefV1, SourceScalarKind)> { self.evaluation.references().iter().filter_map(|(&node, read)| match read { CheckedExpressionReferenceV1::Scalar(read) => Some((node, (read.binding(), read.kind()))), _ => None }).collect() }
    pub(crate) fn is_i64(&self) -> bool { self.evaluation.root_class() == Leaf::Value(Class::I64) }
    pub(crate) fn is_text_residence(&self) -> bool { self.evaluation.root_class() == Leaf::Value(Class::String) && self.owns_root() }
    pub(crate) fn owns_root(&self) -> bool { self.evaluation.owned_result_nodes().contains(&0) }
    pub(crate) fn transfers_node(&self, node: usize) -> bool { self.evaluation.transfers_node(node) }
}

pub(crate) fn qualify_initializer_expression_v1<E>(
    mut prepared: super::leaves::PreparedExpressionCallsV1,
    entry_reads: &[InitializerTextEntryReadV1],
    entry_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<(BindingRefV1, BindingRefV1)>,
    entry_refinement: &mut impl FnMut(&SourceExprSiteV1, BindingRefV1, BindingRefV1) -> Option<Rc<crate::mir::resolved_semantics::home_new_prefix::EntryNullExclusionV1>>,
    residence_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<(BindingRefV1, BindingRefV1, Rc<CheckedInitializerExpressionFrameV1>)>,
    static_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::StaticTextReadV1>,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::CurrentScalarReadV1>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(&[LocalFieldReadRequestV1], super::super::LocalFieldReadDemandV1) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    if !prepared.is_local_initializer() { return Ok(None); }
    let prior_homes = prepared.prior_homes().to_vec();
    let source = Rc::clone(prepared.source());
    let has_core = prepared.has_core();
    let ExpressionPlanRootV1::LocalInitializer { initializer, .. } = source.root() else { return Ok(None) };
    if initializer.binding().owner() != source.owner() || source.nodes().first().map(|node| node.site()) != initializer.initializer_site() { return Ok(None); }
    let mut homes = BTreeSet::new();
    if prior_homes.iter().any(|home| home.owner() != source.owner() || !homes.insert(*home)) { return Ok(None); }
    let Ok(order) = source.ordered_postorder_v1() else { return Ok(None) };
    if source.nodes().iter().any(|node| !matches!(node.children(), ExpressionChildrenV1::Known(children)
        if children.iter().all(|child| child.reach == ExpressionChildReachV1::Always))) { return Ok(None); }
    let mut initial = Vec::new();
    for read in entry_reads {
        let requirement = read.requirement();
        if entry_reference(requirement.read()) != Some((requirement.binding(), requirement.formal().1)) { return Ok(None); }
        let exclusion = entry_refinement(requirement.read(), requirement.binding(), requirement.formal().1);
        initial.push((requirement.node(), CheckedExpressionReferenceV1::InitializerEntry { read: read.clone(), exclusion }));
    }
    let reference_order: Vec<_> = (0..source.nodes().len()).collect();
    let Some(mut reference_proofs) = super::evaluation::seed_current_expression_references_v1(
        &mut prepared, initial, &reference_order, static_reference, residence_reference, scalar_reference,
    ) else { return Ok(None) };
    let Some(super::leaves::QualifiedExpressionLeavesV1 { classes, mut calls, cores, fields, transferred, references }) =
        super::leaves::qualify_expression_leaves_with_calls_v1::<E>(prepared,
            &mut |_| None, &order, field_request)? else { return Ok(None) };
    if !references.is_empty() || transferred.contains(&0) || !matches!(classes[0], Leaf::Value(Class::String | Class::I64))
        || (classes[0] == Leaf::Value(Class::I64) && !has_core) { return Ok(None); }
    let Some((operations, owned_results)) = super::operations::qualify_add_operations_v1(&source, &order, &classes, &transferred) else { return Ok(None) };
    // Owned Text transfers one root residence; I64 retains no Home. Its Text
    // child acquisitions still belong to the SAME ordered evaluation/cleanup.
    if classes[0] == Leaf::Value(Class::String)
        && !owned_results.contains(&0) && !cores.get(&0).is_some_and(|core| core.owns_result()) { return Ok(None); }
    reference_proofs.retain(|node, _| !transferred.contains(node));
    if reference_proofs.iter().any(|(&node, proof)| !proof.matches_node(&source, node)) { return Ok(None); }
    let frame = Rc::new(CheckedInitializerExpressionFrameV1 {
        evaluation: CheckedExpressionEvaluationV1 { source: Rc::clone(&source), prior_homes: prior_homes.into(), order: order.clone().into_boxed_slice(),
            classes: classes.into_boxed_slice(), references: reference_proofs, calls: order.iter().filter_map(|node| calls.remove(node)).collect(),
            operations: operations.into_boxed_slice(), owned_results: super::evaluation::owned_result_nodes(&order, &owned_results, &cores), cores, transferred_inputs: transferred },
    });
    if !frame.evaluation().reference_coverage_is_exact() { return Ok(None); }
    let candidate = HomeExpressionPlanV1 { source: Rc::clone(&source), scalar_operations: Vec::new(), source_fields: Vec::new(),
        print_frame: None, condition_frame: None, argument_frame: None, initializer_frame: Some(frame), field_write: None, initializer_homes: None };
    let Some(candidate) = candidate.with_source_field_requirements(&fields) else { return Ok(None) };
    if !can_install(&candidate) { return Ok(None); }
    if !field_batch(&fields, super::super::LocalFieldReadDemandV1::SourceExpression { plan: source })? { return Ok(None); }
    Ok(Some(candidate))
}
