//! Independent sealed-return coverage for direct and lexical lifecycle calls.
use super::*;

/// Whether the callable's sealed body proves a Map result: its own Map
/// coverage exists and every verified explicit `return` exit yields a
/// Map literal. The header annotation alone never decides this.
/// `return <map-local>` stays outside this source-level proof and fails
/// closed.
pub(in crate::mir::normal_callable_semantic_package) fn map_result_callee(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    callable: crate::mir::resolved_semantics::ResolvedCallableRefV1,
) -> bool {
    map_result_owner(batch, callable.owner())
}

pub(in crate::mir::normal_callable_semantic_package) fn map_result_owner(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> bool {
    let mut declarations = batch
        .declarations()
        .filter(|declaration| declaration.owner() == owner);
    let Some(declaration) = declarations.next() else {
        return false;
    };
    if declarations.next().is_some() {
        return false;
    }
    let body = declaration.body_shape();
    if !body
        .expressions()
        .iter()
        .any(|expression| matches!(expression, BodyExpressionShapeV1::MapLiteral { .. }))
    {
        return false;
    }
    batch
        .with_lowering_input(declaration.batch_slot(), |input| {
            let Some(shape) = input.body_shape() else {
                return false;
            };
            let Some(value_sites) =
                crate::mir::normal_callable_semantic_package::ordinary_new_coseal::verified_value_return_sites(input, shape)
            else {
                return false;
            };
            !value_sites.is_empty()
                && value_sites.iter().all(|site| {
                    matches!(
                        body.expression_shape(site),
                        Some(BodyExpressionShapeV1::MapLiteral { .. })
                    )
                })
        })
        .unwrap_or(false)
}

/// Whether the callable's sealed body proves an owned-object result:
/// every verified explicit `return` exit constructs `new` of one agreed
/// class — the same verified-Completion exit evidence the
/// `callable_result_classes` claim seals. The header annotation alone
/// never decides this, and the sealed terminal relation is re-checked
/// against this class at co-seal.
pub(in crate::mir::normal_callable_semantic_package) fn construction_result_callee(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    expected_class: &str,
) -> bool {
    let mut declarations = batch
        .declarations()
        .filter(|declaration| declaration.owner() == owner);
    let Some(declaration) = declarations.next() else {
        return false;
    };
    if declarations.next().is_some() {
        return false;
    }
    batch
        .with_lowering_input(declaration.batch_slot(), |input| {
            let Some(shape) = input.body_shape() else {
                return false;
            };
            let Some(value_sites) =
                crate::mir::normal_callable_semantic_package::ordinary_new_coseal::verified_value_return_sites(input, shape)
            else {
                return false;
            };
            !value_sites.is_empty() && value_sites.iter().all(|site|
                declaration.construction_class(site).is_some_and(|class| class.as_ref() == expected_class))
        })
        .unwrap_or(false)
}


pub(in crate::mir::normal_callable_semantic_package) fn nullable_exits_hold(batch: &VerifiedResolvedCallableSemanticBatchV1, target_batch_slot: u32) -> bool {
    batch
        .with_lowering_input(target_batch_slot, |callee_input| {
            let Some(shape) = callee_input.body_shape() else {
                return false;
            };
            let Some(sites) = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::verified_value_return_sites(callee_input, shape) else {
                return false;
            };
            !sites.is_empty()
                && sites.iter().all(|site| {
                    let function = callee_input.function();
                    matches!(
                        function.expression_source().literal(site),
                        Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Null)
                    ) || function.expression_source().construction(site).is_some()
                })
        })
        .unwrap_or(false)
}
