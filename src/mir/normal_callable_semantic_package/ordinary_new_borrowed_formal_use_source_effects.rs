//! Original nullable-forward Return and field-write source uses.
//! These retain existing facts; execution, field class and ABI remain separate.
use super::*;
use crate::mir::resolved_semantics::{BodyExpressionShapeV1, BodyStatementShapeV1};

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) struct BorrowedFormalSourceUseContextV1<'a> {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) key: &'a hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) results: &'a result_class_claim::OrdinaryNewResultClassClaimsV1,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) writes: &'a field_write_claim::OrdinaryNewFieldWriteClaimsV1,
}

pub(super) fn source_effect_kind_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    contract: &OwnedCallableParameterContractDeclarationV1,
    context: Option<&BorrowedFormalSourceUseContextV1<'_>>,
    formal: BindingRefV1,
    binding: BindingRefV1,
    site: &SourceExprSiteV1,
    null_guards: &BTreeMap<BindingRefV1, Vec<SourceNodeSiteV1>>,
    integer_guards: &BTreeMap<BindingRefV1, Vec<Rc<guarded_actual::CheckedIntegerGuardV1>>>,
) -> Result<Option<BorrowedFormalUseDraftKindV1>, BorrowedFormalUseDraftErrorV1> {
    let Some(context) = context else { return Ok(None); };
    if contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod {
        return Ok(None);
    }
    // Only this original result responsibility supplies these source fragments.
    // The complete source/input/physical consumers still owe their own checks.
    if !matches!(context.results.get(context.key),
        Some(result_class_claim::OrdinaryNewResultClassV1::NullableForwarded { .. }))
    { return Ok(None); }
    let shape = input.body_shape().ok_or(BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
    for assignment in shape.assignment_sources() {
        let Some(BodyExpressionShapeV1::FieldAccess { object, .. }) =
            shape.expression_shape(assignment.target_site()) else { continue; };
        let receiver = object == site;
        let value = assignment.value_site() == site;
        if !receiver && !value { continue; }
        if receiver && value || !matches!(input.function().assignment_target(assignment.target_site()),
            Some(ResolvedAssignmentTargetV1::FieldWrite { receiver }) if receiver == object)
        { return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity); }
        let source = context.writes.original_write_source_v1(input.owner(), assignment.target_site())
            .map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)?
            .ok_or(BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
        let original = source.write();
        if original.assignment() != assignment || original.receiver_site() != object {
            return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
        }
        if receiver {
            let Some(guard) = null_guards.get(&formal).into_iter().flatten()
                .find(|guard| use_dominated_by_if(assignment.target_site().node(), guard))
            else { return Ok(None); };
            return Ok(Some(BorrowedFormalUseDraftKindV1::FieldWriteReceiver {
                source, guard: guard.clone(),
            }));
        }
        let Some(reach) = integer_guards.get(&formal).into_iter().flatten()
            .find_map(|guard| guard.normal_path_for_operand(input, formal, binding, site))
        else { return Ok(None); };
        return Ok(Some(BorrowedFormalUseDraftKindV1::FieldWriteValue { source, reach }));
    }
    let mut exits = shape.statements().iter().filter_map(|row| match row {
        BodyStatementShapeV1::Return { site: exit, value: Some(value) } if value == site => Some(exit),
        _ => None,
    });
    let Some(exit) = exits.next() else { return Ok(None); };
    if exits.next().is_some() { return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity); }
    let parameter = contract.parameters.iter().find(|row| row.binding == formal)
        .ok_or(BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
    let source = context.results.borrowed_formal_return_source_v1(
        context.key, &OwnedExprSiteV1::new(input.owner(), site.clone()), formal, parameter.ordinal,
    ).map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
    Ok(source.map(|source| BorrowedFormalUseDraftKindV1::BorrowedFormalReturn {
        exit: exit.clone(), source: Box::new(source),
    }))
}
