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
    constructor_source: crate::parser::ConstructorSourceIdV1,
    constructor_owner: FunctionOwnerIdV1,
    formal_binding: BindingRefV1,
    formal_ordinal: u32,
    actual: OwnedExprSiteV1,
    witnesses: Box<[Rc<ResultOriginWitnessV1>]>,
}
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
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn attach_source_child_relations_v1(
    facts: &mut OrdinaryNewResultClassClaimsV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    parameters: &[super::super::super::model::OwnedCallableParameterContractDeclarationV1],
) {
    let mut relations = BTreeMap::new();
    for (key, rows) in facts.source_rows() {
        let Some(slot) = selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key.clone()))
        else {
            continue;
        };
        let mut seen = BTreeSet::new();
        let _ = batch.with_lowering_input(slot, |input| {
            for witness in rows.iter().flat_map(|row| row.witnesses()) {
                if !matches!(witness.step(), ResultWitnessStepV1::FreshConstruction)
                    || !seen.insert(witness.site().clone())
                {
                    continue;
                }
                for relation in join_outer(
                    witness,
                    input,
                    slot,
                    key,
                    facts,
                    batch,
                    selected,
                    constructors,
                    fields,
                    parameters,
                ) {
                    relations.insert((relation.outer.clone(), relation.field), relation);
                }
            }
        });
    }
    facts.attach_child_relations(relations);
}

fn join_outer(
    witness: &ResultOriginWitnessV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    slot: u32,
    key: &CanonicalSameModuleCallableKeyV1,
    facts: &OrdinaryNewResultClassClaimsV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    parameters: &[super::super::super::model::OwnedCallableParameterContractDeclarationV1],
) -> Vec<SourceConstructionChildRelationV1> {
    let mut result = Vec::new();
    let outer = witness.site();
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
    if !field_initializers.is_empty()
        || witness.origin() != &ResultValueOriginV1::Fresh(class.clone().into_boxed_str())
    {
        return result;
    }
    let Ok(Some(parent)) = batch.ordinary_box_coverage().row_for(class) else {
        return result;
    };
    let Ok(Some(birth)) = constructors.birth_for(&parent, arguments.len()) else {
        return result;
    };
    let Ok(plan) = birth.construction() else {
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
        let ConstructionStoreRhsV1::Parameter {
            site,
            binding,
            provided: Some(child),
        } = store.rhs()
        else {
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
        let BirthFormalUseCoverageV1::ObjectFieldStores { sites } = formal.uses() else {
            continue;
        };
        if !sites.contains(site) {
            continue;
        }
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
                    *child,
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
            child: *child,
            constructor_source: source.clone(),
            constructor_owner: *owner,
            formal_binding: *binding,
            formal_ordinal: formal.ordinal(),
            actual,
            witnesses: witnesses.into_boxed_slice(),
        });
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
                resolve_forward_local(binding, function, shape, caller, batch, selected, fields)?
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

fn class_agrees(
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
