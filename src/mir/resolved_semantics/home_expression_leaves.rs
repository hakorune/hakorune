//! Shared leaf qualification of one original whole-expression source plan.
//! Calls own their exact receiver/actual subtrees; fields remain batch obligations.
use super::*;
use super::super::{LocalCallObservationV1, LocalCallResultClassV1, OwnedExprSiteV1, SourceExprSiteV1};
use crate::mir::resolved_value_profile::{
    expression_class::ExpressionValueClassV1 as Class,
    expression_projection::{project_expression_nodes_v1, ExpressionLeafClassV1 as Leaf},
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::resolved_semantics::home_new_prefix) enum ExpressionLeafProfileV1 { Print, BoolCondition, Argument, LocalInitializer, BindingRebind }

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExpressionCallSourceV1 {
    Ordinary(LocalCallObservationV1),
    Core(crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1),
}

pub(super) struct QualifiedExpressionLeavesV1 {
    pub(super) cores: BTreeMap<usize, crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1>,
    pub(super) classes: Vec<Leaf>,
    pub(super) calls: BTreeMap<usize, LocalCallObservationV1>,
    pub(super) fields: Vec<LocalFieldReadRequestV1>,
    pub(super) transferred: BTreeSet<usize>,
    pub(super) references: BTreeMap<usize, (crate::mir::resolved_semantics::BindingRefV1, super::super::SourceScalarKind)>,
}

pub(super) fn qualify_expression_leaves_v1<E>(
    source: &Rc<ExpressionSourcePlanV1>,
    profile: ExpressionLeafProfileV1,
    seeded: &BTreeMap<usize, Leaf>,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<(crate::mir::resolved_semantics::BindingRefV1, super::super::SourceScalarKind)>,
    prior_homes: &[crate::mir::resolved_semantics::BindingRefV1],
    order: &[usize],
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    source_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<ExpressionCallSourceV1>, E>,
) -> Result<Option<QualifiedExpressionLeavesV1>, E> {
    let Some(prepared) = prepare_expression_calls_v1(source, profile, seeded, prior_homes, source_call)?
        else { return Ok(None) };
    qualify_expression_leaves_with_calls_v1(prepared, scalar_reference, order, field_request)
}

/// Invocation-local scope from the sole original Call/Core classification.
/// Consumed by leaf completion; no independent source, Home or physical grant.
pub(crate) struct PreparedExpressionCallsV1 {
    source: Rc<ExpressionSourcePlanV1>,
    profile: ExpressionLeafProfileV1,
    leaves: Vec<Leaf>,
    calls: BTreeMap<usize, LocalCallObservationV1>,
    cores: BTreeMap<usize, crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1>,
    transferred: BTreeSet<usize>,
    prior_homes: Box<[crate::mir::resolved_semantics::BindingRefV1]>,
}

impl PreparedExpressionCallsV1 {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn source(&self) -> &Rc<ExpressionSourcePlanV1> { &self.source }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn prior_homes(&self) -> &[crate::mir::resolved_semantics::BindingRefV1] { &self.prior_homes }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn is_local_initializer(&self) -> bool { self.profile == ExpressionLeafProfileV1::LocalInitializer }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn is_binding_rebind(&self) -> bool { self.profile == ExpressionLeafProfileV1::BindingRebind }
    pub(super) fn is_bool_condition(&self) -> bool { self.profile == ExpressionLeafProfileV1::BoolCondition }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn has_core(&self) -> bool { !self.cores.is_empty() }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn is_active_reference(&self, node: usize) -> bool {
        !self.transferred.contains(&node) && self.leaves.get(node) == Some(&Leaf::Unresolved)
            && self.source.nodes().get(node).is_some_and(|node| matches!(node.source(), ExpressionNodeSourceV1::Reference(_))
                && !self.source.nodes().iter().any(|parent| matches!(parent.source(),
                    ExpressionNodeSourceV1::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { object, .. })
                        if object == node.site())))
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn seed_reference(&mut self, node: usize, class: Leaf) -> bool {
        if !self.is_active_reference(node) || class.value_classes().is_none() { return false; }
        self.leaves[node] = class;
        true
    }
}

pub(in crate::mir::resolved_semantics::home_new_prefix) fn prepare_expression_calls_v1<E>(
    source: &Rc<ExpressionSourcePlanV1>, profile: ExpressionLeafProfileV1,
    seeded: &BTreeMap<usize, Leaf>, prior_homes: &[crate::mir::resolved_semantics::BindingRefV1],
    source_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<ExpressionCallSourceV1>, E>,
) -> Result<Option<PreparedExpressionCallsV1>, E> {
    let statement = source.root().statement();
    let mut leaves = vec![Leaf::Unresolved; source.nodes().len()];
    for (index, class) in seeded {
        let Some(leaf) = leaves.get_mut(*index) else { return Ok(None) };
        *leaf = *class;
    }
    let mut transferred = BTreeSet::new();
    let mut calls = BTreeMap::new();
    let mut cores = BTreeMap::new();
    // Preorder discovers each call's real whole-input scope before descendants.
    // Those exact argument subtrees remain owned by the call/input recipe;
    // the enclosing frame must not emit them or acquire their owners twice.
    for (index, node) in source.nodes().iter().enumerate() {
        if transferred.contains(&index) || seeded.contains_key(&index) || !matches!(node.source(), ExpressionNodeSourceV1::MethodCall) { continue; }
        let site = OwnedExprSiteV1::new(source.owner(), node.site().clone());
        let Some(call) = source_call(&site)? else { return Ok(None) };
        let call = match call {
            ExpressionCallSourceV1::Core(core) => {
                if !core.matches_plan(source, index) { return Ok(None); }
                leaves[index] = Leaf::Value(if core.owns_result() { Class::String } else { Class::I64 });
                cores.insert(index, core);
                continue;
            }
            ExpressionCallSourceV1::Ordinary(call) => call,
        };
        let Some(loan) = call.borrowed_scalar() else { return Ok(None) };
        let (result, class) = match loan.kind() {
            super::super::SourceScalarKind::Integer => (LocalCallResultClassV1::I64, Class::I64),
            super::super::SourceScalarKind::Bool if profile == ExpressionLeafProfileV1::BoolCondition =>
                (LocalCallResultClassV1::Bool, Class::Bool),
            _ => return Ok(None),
        };
        if call.owner() != source.owner() || call.statement() != statement || call.site() != &site
            || !call.is_expression_value() || call.result() != result
            || loan.source().call_site() != &site
            || call.prior_homes() != prior_homes
            || call.arguments().len() != loan.source().argument_sites().len()
        { return Ok(None); }
        let Some(receiver) = source.nodes().iter().position(|node| node.site() == loan.source().receiver_site())
            else { return Ok(None) };
        leaves[receiver] = Leaf::Identity;
        for argument in std::iter::once(loan.source().receiver_site()).chain(loan.source().argument_sites().iter()) {
            if !source.nodes().iter().any(|node| node.site() == argument) { return Ok(None); }
            for (child, node) in source.nodes().iter().enumerate() {
                if node.site().node().segments().starts_with(argument.node().segments()) { transferred.insert(child); }
            }
        }
        leaves[index] = Leaf::Value(class);
        calls.insert(index, call);
    }
    Ok(Some(PreparedExpressionCallsV1 { source: Rc::clone(source), profile, leaves, calls, cores, transferred, prior_homes: prior_homes.into() }))
}

pub(super) fn qualify_expression_leaves_with_calls_v1<E>(
    prepared: PreparedExpressionCallsV1,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<(crate::mir::resolved_semantics::BindingRefV1, super::super::SourceScalarKind)>,
    order: &[usize],
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
) -> Result<Option<QualifiedExpressionLeavesV1>, E> {
    let PreparedExpressionCallsV1 { source, profile, mut leaves, calls, cores, transferred, prior_homes: _ } = prepared;
    let source = &source;
    let mut references = BTreeMap::new();
    let mut fields = Vec::new();
    // Establish borrowed receiver roles before postorder reaches their Local
    // reference leaves. A receiver is not an Integer/Bool operand use.
    for &index in order {
        if transferred.contains(&index) { continue; }
        let node = &source.nodes()[index];
        if matches!(node.source(), ExpressionNodeSourceV1::Shape(
            crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. })) {
                let Some(request) = field_request(node.site()) else { return Ok(None) };
                if request.node_in_plan(source) != Some(index) { return Ok(None); }
                let Some(receiver) = source.nodes().iter().position(|node| node.site() == &request.receiver_site)
                    else { return Ok(None) };
                leaves[receiver] = Leaf::Identity;
                leaves[index] = if request.path.as_ref().is_some_and(|path| path.is_intermediate()) {
                    Leaf::Identity
                } else { Leaf::Value(Class::I64) };
                fields.push(request);
        }
    }
    for &index in order {
        if transferred.contains(&index) { continue; }
        let node = &source.nodes()[index];
        match node.source() {
            ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::Integer(_)) => {},
            ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::String(_))
                if matches!(profile, ExpressionLeafProfileV1::Print | ExpressionLeafProfileV1::BoolCondition | ExpressionLeafProfileV1::Argument | ExpressionLeafProfileV1::LocalInitializer | ExpressionLeafProfileV1::BindingRebind) => {},
            ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::Bool(_))
                if profile == ExpressionLeafProfileV1::BoolCondition => {},
            ExpressionNodeSourceV1::Literal(_) => return Ok(None),
            ExpressionNodeSourceV1::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }) => {},
            ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding))
                if matches!(profile, ExpressionLeafProfileV1::BoolCondition | ExpressionLeafProfileV1::Argument | ExpressionLeafProfileV1::LocalInitializer | ExpressionLeafProfileV1::BindingRebind) && leaves[index] == Leaf::Unresolved => {
                let Some((original, kind)) = scalar_reference(node.site()) else { return Ok(None) };
                if original != *binding || binding.owner() != source.owner() { return Ok(None); }
                leaves[index] = Leaf::Value(match kind {
                    super::super::SourceScalarKind::Integer => Class::I64,
                    super::super::SourceScalarKind::Bool => Class::Bool,
                });
                references.insert(index, (original, kind));
            }
            _ => {},
        }
    }
    let Some(classes) = project_expression_nodes_v1(&source, &transferred, |index| leaves[index])
        else { return Ok(None) };
    // A Core result never erases its receiver or argument obligations. Those
    // children were evaluated in the SAME postorder, unlike transferred Calls.
    for core in cores.values() {
        let original = core.source().source();
        let class = |site| source.nodes().iter().position(|node| node.site() == site).map(|node| classes[node]);
        if class(original.receiver_site()) != Some(Leaf::Value(Class::String)) { return Ok(None); }
        for (argument, parameter) in original.arguments().iter().zip(core.source().target().parameters()) {
            let expected = match parameter {
                crate::mir::resolved_semantics::CoreMethodHomeParameterRelationV1::I64Parameter => Class::I64,
                crate::mir::resolved_semantics::CoreMethodHomeParameterRelationV1::TextParameter => Class::String,
                _ => return Ok(None),
            };
            if class(argument.site()) != Some(Leaf::Value(expected)) { return Ok(None); }
        }
    }
    Ok(Some(QualifiedExpressionLeavesV1 { classes, calls, cores, fields, transferred, references }))
}
