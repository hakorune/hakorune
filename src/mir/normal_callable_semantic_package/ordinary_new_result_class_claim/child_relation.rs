//! Exact returned-construction child provenance. Candidates carry no Home permission.
use super::call_witness::CallWitnessSourceV1;
use super::*;
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_view::ExprChildRoleV1;
use crate::mir::normal_callable_semantic_package::{
    instance_construction::ConstructionStoreRhsV1,
    instance_constructor_semantic::VerifiedInstanceConstructorSemanticBatchV1,
    BirthFormalUseCoverageV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, OwnedExprSiteV1};
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct SourceConstructionChildRelationV1 {
    outer: OwnedExprSiteV1,
    field: CanonicalFieldRefV1,
    child: CanonicalObjectIdV1,
    unqualified_dynamic: bool,
    constructor_source: crate::parser::ConstructorSourceIdV1,
    constructor_owner: FunctionOwnerIdV1,
    formal_binding: BindingRefV1,
    formal_ordinal: u32,
    actual: OwnedExprSiteV1,
    witnesses: Box<[Rc<ResultOriginWitnessV1>]>,
    store: crate::mir::resolved_semantics::ResolvedAssignmentSourceV1,
    store_source: Result<Option<super::super::field_write_claim::OriginalFieldWriteSourceLoanV1>, &'static str>,
}
// This source-issued immutable product has identity equality. Structurally
// equal facts from another issuance never replace the original relation loan.
impl PartialEq for SourceConstructionChildRelationV1 {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self, other) }
}
impl Eq for SourceConstructionChildRelationV1 {}
impl SourceConstructionChildRelationV1 {
    pub(crate) fn outer(&self) -> &OwnedExprSiteV1 {
        &self.outer
    }
    pub(crate) fn field(&self) -> CanonicalFieldRefV1 {
        self.field
    }
    pub(crate) fn child(&self) -> CanonicalObjectIdV1 {
        self.child
    }
    pub(crate) fn is_unqualified_dynamic_v1(&self) -> bool {
        self.unqualified_dynamic
    }
    pub(crate) fn constructor_source(&self) -> &crate::parser::ConstructorSourceIdV1 {
        &self.constructor_source
    }
    pub(crate) fn constructor_owner(&self) -> FunctionOwnerIdV1 {
        self.constructor_owner
    }
    pub(crate) fn formal_binding(&self) -> BindingRefV1 {
        self.formal_binding
    }
    pub(crate) fn formal_ordinal(&self) -> u32 {
        self.formal_ordinal
    }
    pub(crate) fn actual(&self) -> &OwnedExprSiteV1 {
        &self.actual
    }
    pub(crate) fn witnesses(&self) -> &[Rc<ResultOriginWitnessV1>] {
        &self.witnesses
    }

    pub(crate) fn original_field_name_v1(&self) -> Result<&str, &'static str> {
        Ok(self.store_source.as_ref().map_err(|issue| *issue)?
            .as_ref().ok_or("returned-field/store-source-missing")?.write().field())
    }

    /// Exact immutable Provided residence; separate from the child's value class.
    pub(crate) fn provided_field_source_v1(
        &self, owner_box: &str,
    ) -> Result<&super::super::field_write_claim::OriginalFieldWriteSourceLoanV1, &'static str> {
        if self.unqualified_dynamic {
            return Err("returned-field/store-source-unqualified");
        }
        let source = self.store_source.as_ref().map_err(|issue| *issue)?
            .as_ref().ok_or("returned-field/store-source-missing")?;
        if !source.corroborates_sole_provided_birth_v1(owner_box, &self.constructor_source,
            self.constructor_owner, &self.store) {
            return Err("returned-field/store-source-incomplete");
        }
        Ok(source)
    }

    pub(crate) fn unqualified_field_source_v1(
        &self, owner_box: &str,
    ) -> Result<&super::super::field_write_claim::OriginalFieldWriteSourceLoanV1, &'static str> {
        if !self.unqualified_dynamic {
            return Err("returned-field/store-source-qualified");
        }
        let source = self.store_source.as_ref().map_err(|issue| *issue)?
            .as_ref().ok_or("returned-field/store-source-missing")?;
        if !source.corroborates_sole_provided_birth_v1(owner_box, &self.constructor_source,
            self.constructor_owner, &self.store) {
            return Err("returned-field/store-source-incomplete");
        }
        Ok(source)
    }
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn attach_source_child_relations_v1(
    facts: &mut OrdinaryNewResultClassClaimsV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    parameters: &[super::super::super::model::OwnedCallableParameterContractDeclarationV1],
) -> Result<(), super::super::OrdinaryNewCoSealIssueV1> {
    let mut dynamic_classes = BTreeMap::<CanonicalFieldRefV1, BTreeSet<CanonicalObjectIdV1>>::new();
    let mut expected_dynamic_sites = BTreeMap::<CanonicalObjectIdV1, BTreeSet<OwnedExprSiteV1>>::new();
    let mut new_sites = Vec::new();
    let result_construction_sites: BTreeSet<_> = facts.source_rows().flat_map(|(_, rows)| rows.iter())
        .flat_map(|row| row.witnesses())
        .filter_map(|witness| witness.construction_site_v1().cloned())
        .collect();
    for declaration in batch.declarations() {
        let slot = declaration.batch_slot();
        let key = match selected.key_for_batch_slot(slot) {
            Some(SelectedNormalCallableKeyV1::Cataloged(key)) => Some(key.clone()),
            _ => None,
        };
        batch.with_lowering_input(slot, |input| {
            for construction in input.function().expression_source().constructions() {
                let outer = OwnedExprSiteV1::new(input.owner(), construction.site().clone());
                let Some(parent_object) = parent_object_for_outer(&outer, input, batch, constructors) else {
                    continue;
                };
                expected_dynamic_sites.entry(parent_object).or_default().insert(outer.clone());
                if birth_owner_for_outer(&outer, input, batch, constructors).is_some() {
                    if let Some(key) = &key {
                        for (field, child) in dynamic_child_classes_for_outer(
                            &outer, input, slot, key, facts, batch, selected, constructors, fields, parameters,
                        ) {
                            dynamic_classes.entry(field).or_default().insert(child);
                        }
                    }
                }
                new_sites.push((slot, key.clone(), outer.clone(), result_construction_sites.contains(&outer)));
            }
        }).map_err(|_| super::super::OrdinaryNewCoSealIssueV1::BatchLoan)?;
    }

    let dynamic_classes: BTreeMap<_, _> = dynamic_classes.into_iter()
        .filter_map(|(field, classes)| (classes.len() == 1).then(|| (field, *classes.first().unwrap())))
        .collect();
    let mut relations = BTreeMap::new();
    for (slot, key, outer, is_result_construction) in new_sites {
        if !is_result_construction { continue; }
        let Some(key) = key else { continue; };
        batch.with_lowering_input(slot, |input| {
            for relation in join_outer(
                &outer,
                input,
                slot,
                &key,
                facts,
                batch,
                selected,
                constructors,
                fields,
                parameters,
                &dynamic_classes,
            ) {
                relations.insert((relation.outer.clone(), relation.field), relation);
            }
        }).map_err(|_| super::super::OrdinaryNewCoSealIssueV1::BatchLoan)?;
    }

    let mut dynamic_fields = BTreeMap::<
        CanonicalObjectIdV1,
        BTreeMap<OwnedExprSiteV1, BTreeMap<CanonicalFieldRefV1, Option<bool>>>,
    >::new();
    for relation in relations.values().filter(|relation| relation.unqualified_dynamic) {
        let category = relation.witnesses().first().and_then(|first| {
            let category = match first.origin() {
                ResultValueOriginV1::Null => false,
                ResultValueOriginV1::Fresh(_) => true,
                _ => return None,
            };
            relation.witnesses().iter().all(|witness| match (category, witness.origin()) {
                (false, ResultValueOriginV1::Null) => true,
                (true, ResultValueOriginV1::Fresh(class)) => batch.ordinary_box_coverage()
                    .row_for(class).ok().flatten()
                    .and_then(|source| constructors.object_for(&source).ok()) == Some(relation.child()),
                _ => false,
            }).then_some(category)
        });
        dynamic_fields.entry(relation.field.object()).or_default()
            .entry(relation.outer.clone()).or_default().insert(relation.field, category);
    }
    let rejected_dynamic_objects: BTreeSet<_> = expected_dynamic_sites.iter().filter_map(|(object, sites)| {
        let fields = dynamic_fields.get(object);
        let mut expected_fields: Option<BTreeSet<CanonicalFieldRefV1>> = None;
        let complete = !sites.is_empty() && sites.iter().all(|site| {
            let Some(current) = fields.and_then(|rows| rows.get(site)) else { return false; };
            if current.is_empty() || current.values().any(|category| category.is_none())
                || current.values().copied().flatten().collect::<BTreeSet<_>>().len() != 1 { return false; }
            let current_fields: BTreeSet<_> = current.keys().copied().collect();
            match expected_fields.as_ref() {
                Some(expected) => expected == &current_fields,
                None => { expected_fields = Some(current_fields); true }
            }
        });
        (!complete).then_some(*object)
    }).collect();
    relations.retain(|_, relation| !relation.unqualified_dynamic
        || !rejected_dynamic_objects.contains(&relation.field.object()));
    facts.attach_child_relations(relations);
    Ok(())
}

fn join_outer(
    outer: &OwnedExprSiteV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    slot: u32,
    key: &CanonicalSameModuleCallableKeyV1,
    facts: &OrdinaryNewResultClassClaimsV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    parameters: &[super::super::super::model::OwnedCallableParameterContractDeclarationV1],
    dynamic_classes: &BTreeMap<CanonicalFieldRefV1, CanonicalObjectIdV1>,
) -> Vec<SourceConstructionChildRelationV1> {
    let mut result = Vec::new();
    if outer.owner() != input.owner() {
        return result;
    }
    let Ok(located) = input.source().expr_at(outer) else {
        return result;
    };
    let ASTNode::New {
        class,
        arguments,
        field_initializers,
        ..
    } = located.node()
    else {
        return result;
    };
    if !field_initializers.is_empty() {
        return result;
    }
    let Ok(Some(parent)) = batch.ordinary_box_coverage().row_for(class) else {
        return result;
    };
    let Ok(Some(birth)) = constructors.birth_for(&parent, arguments.len()) else {
        return result;
    };
    let Ok(plan) = birth.construction_source() else {
        return result;
    };
    let Some((source, owner)) = plan.constructor() else {
        return result;
    };
    let [root] = birth.forest().roots() else {
        return result;
    };
    if !source.same_as(birth.source_id())
        || owner != root
        || owner.compilation_brand() != input.owner().compilation_brand()
        || plan.object() != birth.object()
        || constructors.object_for(&parent).ok() != Some(plan.object())
    {
        return result;
    }
    for store in plan.stores() {
        let ConstructionStoreRhsV1::Parameter { site, binding, provided } = store.rhs() else {
            continue;
        };
        if plan
            .stores()
            .iter()
            .filter(|other| other.field() == store.field())
            .count()
            != 1
        {
            continue;
        }
        if store.field().object() != plan.object() || binding.owner() != *owner {
            continue;
        }
        let Some(formal) = birth
            .formal_contracts()
            .iter()
            .find(|formal| formal.binding() == *binding)
        else {
            continue;
        };
        let (child, unqualified_dynamic) = match (provided, formal.uses()) {
            (Some(child), BirthFormalUseCoverageV1::ObjectFieldStores { sites }) if sites.contains(site) => (*child, false),
            (None, BirthFormalUseCoverageV1::UnqualifiedFieldStores { sites }) if sites.contains(site) => {
                let Some(child) = dynamic_classes.get(&store.field()).copied() else { continue; };
                (child, true)
            }
            _ => continue,
        };
        let Ok(actual) = input
            .source()
            .child_expr_from_expr(&located, ExprChildRoleV1::CallArgument(formal.ordinal()))
        else {
            continue;
        };
        let actual = OwnedExprSiteV1::new(actual.owner(), actual.site().clone());
        if actual.owner() != input.owner() {
            continue;
        }
        let Some(witnesses) = actual_witnesses(
            &actual, input, slot, key, facts, batch, selected, fields, parameters,
        ) else {
            continue;
        };
        if witnesses.is_empty()
            || !witnesses.iter().all(|witness| {
                class_agrees(
                    witness.origin(),
                    child,
                    input,
                    slot,
                    batch,
                    constructors,
                    parameters,
                )
            })
        {
            continue;
        }
        result.push(SourceConstructionChildRelationV1 {
            outer: outer.clone(),
            field: store.field(),
            child,
            unqualified_dynamic,
            constructor_source: source.clone(),
            constructor_owner: *owner,
            formal_binding: *binding,
            formal_ordinal: formal.ordinal(),
            actual,
            witnesses: witnesses.into_boxed_slice(),
            store: store.assignment().clone(),
            store_source: fields.original_write_source_v1(*owner, store.assignment().target_site()),
        });
    }
    result
}

fn birth_owner_for_outer(
    outer: &OwnedExprSiteV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
) -> Option<FunctionOwnerIdV1> {
    if outer.owner() != input.owner() { return None; }
    let located = input.source().expr_at(outer).ok()?;
    let ASTNode::New { class, arguments, field_initializers, .. } = located.node() else { return None; };
    if !field_initializers.is_empty() { return None; }
    let parent = batch.ordinary_box_coverage().row_for(class).ok().flatten()?;
    let birth = constructors.birth_for(&parent, arguments.len()).ok().flatten()?;
    let source = birth.construction_source().as_ref().ok()?;
    let (constructor, owner) = source.constructor()?;
    let [root] = birth.forest().roots() else { return None; };
    (constructor.same_as(birth.source_id()) && *owner == *root && source.object() == birth.object()
        && constructors.object_for(&parent).ok() == Some(source.object())).then_some(*owner)
}

fn parent_object_for_outer(
    outer: &OwnedExprSiteV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
) -> Option<CanonicalObjectIdV1> {
    if outer.owner() != input.owner() { return None; }
    let located = input.source().expr_at(outer).ok()?;
    let ASTNode::New { class, .. } = located.node() else { return None; };
    let parent = batch.ordinary_box_coverage().row_for(class).ok().flatten()?;
    constructors.object_for(&parent).ok()
}

/// Gather exact non-null class anchors from all direct result constructions.
/// Null-only sites are completed by the same field's source-ordered Births
/// after this cohort fold; each site still retains its own argument witness.
#[allow(clippy::too_many_arguments)]
fn dynamic_child_classes_for_outer(
    outer: &OwnedExprSiteV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    slot: u32,
    key: &CanonicalSameModuleCallableKeyV1,
    facts: &OrdinaryNewResultClassClaimsV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    parameters: &[super::super::super::model::OwnedCallableParameterContractDeclarationV1],
) -> Vec<(CanonicalFieldRefV1, CanonicalObjectIdV1)> {
    let mut result = Vec::new();
    let Some(_owner) = birth_owner_for_outer(outer, input, batch, constructors) else { return result; };
    let Ok(located) = input.source().expr_at(outer) else { return result; };
    let ASTNode::New { class, arguments, .. } = located.node() else { return result; };
    let Ok(Some(parent)) = batch.ordinary_box_coverage().row_for(class) else { return result; };
    let Ok(Some(birth)) = constructors.birth_for(&parent, arguments.len()) else { return result; };
    let Ok(source) = birth.construction_source() else { return result; };
    if source.constructor().map(|(constructor, source_owner)| (constructor.same_as(birth.source_id()), *source_owner))
        != Some((true, _owner)) { return result; }

    for store in source.stores() {
        let ConstructionStoreRhsV1::Parameter { site, binding, provided: None } = store.rhs() else { continue; };
        if store.field().object() != source.object() || binding.owner() != _owner
            || source.stores().iter().filter(|other| other.field() == store.field()).count() != 1 { continue; }
        let Some(formal) = birth.formal_contracts().iter().find(|formal| formal.binding() == *binding) else { continue; };
        let BirthFormalUseCoverageV1::UnqualifiedFieldStores { sites } = formal.uses() else { continue; };
        if !sites.contains(site) { continue; }
        let Ok(actual) = input.source().child_expr_from_expr(
            &located, ExprChildRoleV1::CallArgument(formal.ordinal()),
        ) else { continue; };
        let actual = OwnedExprSiteV1::new(actual.owner(), actual.site().clone());
        if actual.owner() != input.owner() { continue; }
        let Some(witnesses) = actual_witnesses(
            &actual, input, slot, key, facts, batch, selected, fields, parameters,
        ) else { continue; };
        let mut children = BTreeSet::new();
        let valid = !witnesses.is_empty() && witnesses.iter().all(|witness| match witness.origin() {
            ResultValueOriginV1::Null => true,
            ResultValueOriginV1::Fresh(child_class) => batch.ordinary_box_coverage().row_for(child_class)
                .ok().flatten().and_then(|source| constructors.object_for(&source).ok())
                .is_some_and(|child| { children.insert(child); true }),
            _ => false,
        });
        if valid && children.len() == 1 {
            result.push((store.field(), *children.first().unwrap()));
        }
    }
    result
}

fn actual_witnesses(
    actual: &OwnedExprSiteV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    slot: u32,
    caller: &CanonicalSameModuleCallableKeyV1,
    facts: &OrdinaryNewResultClassClaimsV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    parameters: &[super::super::super::model::OwnedCallableParameterContractDeclarationV1],
) -> Option<Vec<Rc<ResultOriginWitnessV1>>> {
    let function = input.function();
    if matches!(
        function.expression_source().literal(actual.site()),
        Some(ResolvedLiteralSourceV1::Null)
    ) {
        return Some(vec![Rc::new(ResultOriginWitnessV1 {
            site: actual.clone(),
            origin: ResultValueOriginV1::Null,
            step: ResultWitnessStepV1::NullLiteral,
        })]);
    }
    let shape = input.body_shape()?;
    let forwarded = if let Some(ResolvedLexicalRefV1::Local(binding)) =
        function.variable_ref(actual.site())
    {
        match function.binding(binding)?.kind() {
            BindingKindV1::Parameter { index } => {
                let parameter = parameter_contract(parameters, slot, binding)?;
                if parameter.ordinal != index || !binding_is_unrebound(function, binding)
                    || !matches!(parameter.kind, crate::mir::callable_parameter_contract::CallableParameterContractKindV1::DeclaredObject(_)) { return None; }
                return Some(
                    [
                        ResultValueOriginV1::Null,
                        ResultValueOriginV1::ForwardFormal { ordinal: index },
                    ]
                    .into_iter()
                    .map(|origin| {
                        Rc::new(ResultOriginWitnessV1 {
                            site: actual.clone(),
                            origin,
                            step: ResultWitnessStepV1::Formal {
                                binding,
                                ordinal: index,
                            },
                        })
                    })
                    .collect(),
                );
            }
            BindingKindV1::Local { .. } => {
                let call =
                    OwnedExprSiteV1::new(input.owner(), sole_initializer_site(function, binding)?);
                let source = facts
                    .original_local_call_source_v1(batch, selected, fields, &call, binding, parameters, None)
                    .ok()??;
                return source.compose_at_v1(actual, facts, batch, parameters).ok();
            }
            _ => return None,
        }
    } else {
        let (key, actuals) = resolve_call_key(
            actual.site(),
            function,
            shape,
            caller,
            batch,
            selected,
            fields,
        )?;
        PendingExitV1::Fwd {
            call_site: actual.clone(),
            key,
            actuals,
        }
    };
    let PendingExitV1::Fwd {
        call_site,
        key,
        actuals,
    } = forwarded
    else {
        return None;
    };
    let source = CallWitnessSourceV1::verify(actual, &call_site, &key, &actuals)?;
    source.compose(facts.outcomes(&key)?, parameters, slot)
}

pub(super) fn class_agrees(
    origin: &ResultValueOriginV1,
    child: CanonicalObjectIdV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    slot: u32,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    parameters: &[super::super::super::model::OwnedCallableParameterContractDeclarationV1],
) -> bool {
    use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
    let name = match origin {
        ResultValueOriginV1::MapHeld { .. } => return false,
        ResultValueOriginV1::Null => return true,
        ResultValueOriginV1::Fresh(name) => name,
        ResultValueOriginV1::ForwardFormal { ordinal } => {
            let Some(parameter) = parameters
                .iter()
                .find(|row| row.batch_slot == slot && row.owner == input.owner())
                .and_then(|row| {
                    row.parameters
                        .iter()
                        .find(|parameter| parameter.ordinal == *ordinal)
                })
            else {
                return false;
            };
            if parameter.binding.owner() != input.owner()
                || !matches!(input.function().binding(parameter.binding).map(|record| record.kind()), Some(BindingKindV1::Parameter { index }) if index == *ordinal)
            {
                return false;
            }
            let CallableParameterContractKindV1::DeclaredObject(name) = &parameter.kind else {
                return false;
            };
            name
        }
    };
    batch
        .ordinary_box_coverage()
        .row_for(name)
        .ok()
        .flatten()
        .and_then(|source| constructors.object_for(&source).ok())
        == Some(child)
}

#[cfg(test)]
#[path = "child_relation_tests.rs"]
mod tests;
