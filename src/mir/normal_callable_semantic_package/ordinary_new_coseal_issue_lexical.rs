//! Claim-local `recv.m(...)` lexical call probes for the co-seal walk.
use super::super::lexical_instance_call::{
    PreparedBorrowedFormalIngressV1, PreparedLexicalInstanceCallSourceTargetsV1,
};
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::BorrowedResultCohortV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    BorrowedCallActualRequestV1, BorrowedCallArgumentsV1,
};
use crate::mir::resolved_semantics::BindingKindV1;

pub(super) use super::call_arguments::borrowed_call_arguments_callback_v1;

/// The shared claim-local receiver proof for a `recv.m(...)` local call:
/// the method-call inventory row at `site` carries a `Lexical(Local)`
/// receiver binding owned by this function, never rebound, initialized
/// exactly once by an ordinary-new claim candidate. On success this
/// returns the uniquely selected `InstanceBoxMethod` target, its batch
/// slot, and the callee owner — the members every lexical result lane
/// agrees on before it proves its own result class.
fn lexical_claim_local_target(
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    candidates: &[OrdinaryNewCandidate],
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
) -> Option<(
    hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    u32,
    crate::mir::resolved_semantics::FunctionOwnerIdV1,
    u32,
)> {
    use crate::mir::resolved_semantics::{
        ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1,
    };
    if site.owner() != input.owner() {
        return None;
    }
    let Some((observed_site, call)) = input
        .function()
        .method_calls()
        .find(|(call_site, _)| *call_site == site.site())
    else {
        return None;
    };
    if observed_site != site.site() {
        return None;
    }
    let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
        call.receiver()
    else {
        return None;
    };
    if binding.owner() != input.owner()
        || !matches!(
            input.function().binding(binding).map(|row| row.kind()),
            Some(BindingKindV1::Local { .. })
        )
        || input.function().assignment_targets().any(|(_, target)| {
            matches!(
                target,
                ResolvedAssignmentTargetV1::BindingRebind(rebound) if *rebound == binding
            )
        })
    {
        return None;
    }
    let mut initializers = input
        .function()
        .expression_source()
        .initializers()
        .filter(|initializer| initializer.binding() == binding);
    if initializers.next().is_none() || initializers.next().is_some() {
        return None;
    }
    let mut claim_rows = candidates
        .iter()
        .filter(|candidate| candidate.destination == binding);
    let Some(candidate) = claim_rows.next() else {
        return None;
    };
    if claim_rows.next().is_some() {
        return None;
    }
    let Some((key, target_batch_slot)) =
        super::super::lexical_instance_call::unique_instance_target(
            selected,
            candidate.class.as_ref(),
            call.selector(),
            call.arity(),
        )
    else {
        return None;
    };
    let callee_owner = batch
        .declarations()
        .find(|declaration| declaration.batch_slot() == target_batch_slot)
        .map(|declaration| declaration.owner())?;
    Some((key, target_batch_slot, callee_owner, call.arity()))
}

/// Every formal of the resolved callee must be an exact `i64` scalar in
/// declared order — the sealed contract rows carry the callee's declared
/// parameter bindings, so both sides agree on the same ordinal map. The
/// unique contract row itself comes back so callers can reuse its proven
/// binding set without re-filtering.
fn all_formals_exact_i64<'a>(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    parameter_contracts: &'a [crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    callee_owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    arity: u32,
) -> Option<&'a crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1>
{
    let mut contracts = parameter_contracts
        .iter()
        .filter(|row| row.owner == callee_owner);
    let contract = contracts.next()?;
    if contracts.next().is_some() || contract.parameters.len() != arity as usize {
        return None;
    }
    batch
        .with_lowering_input(contract.batch_slot, |callee_input| {
            (callee_input.owner() == contract.owner
                && contract
                    .parameters
                    .iter()
                    .enumerate()
                    .all(|(index, parameter)| {
                        parameter.ordinal as usize == index
                            && parameter.kind
                                == crate::mir::callable_parameter_contract::CallableParameterContractKindV1::ExactTrivial(
                                    crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64,
                                )
                            && callee_input.function().declaration_binding(
                                &SourceBindingSiteV1::Parameter {
                                    index: parameter.ordinal,
                                },
                            ) == Some(parameter.binding)
                    }))
            .then_some(contract)
        })
        .unwrap_or(None)
}

/// The route of a `recv.m(...)` local-call site whose receiver is a
/// claim-local `new` binding: the callee's result class, decided once from the
/// callee's own class source.
///
/// * a sealed `NullableObject` claim whose every verified value-return is
///   `return null` or a construction -> `LexicalNullable` (the received binding
///   joins the owned-Home ledger for a checked release);
/// * exact-`i64` formals and a one-class construction result -> `LexicalHandle`
///   (the callee's co-sealed disposition mints a Handle result);
/// * exact-`i64` formals, no result claim and a normal exact-`i64` result in the
///   callee's result cohort -> `LexicalI64`. A return annotation alone does not
///   verify the literal type: Float, Bool, String and Null must not enter this
///   lane. The emitter separately corroborates the minted row's
///   `result == Some(I64)`; disagreement freezes without fallback.
///
/// Parameter receivers, nested handle-result receivers and rebound bindings
/// prove nothing here — the site simply has no route.
pub(super) fn lexical_local_route(
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    parameter_contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    callable_result_classes: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    borrowed_i64_results: &super::super::lexical_instance_call::BorrowedResultCohortV1,
    candidates: &[OrdinaryNewCandidate],
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
) -> Option<crate::mir::resolved_semantics::home_new_prefix::LocalCallRouteV1> {
    use crate::mir::resolved_semantics::home_new_prefix::LocalCallRouteV1;
    let (key, target_batch_slot, callee_owner, arity) =
        lexical_claim_local_target(selected, batch, candidates, input, site)?;
    let claim = callable_result_classes.get(&key);
    // K3: the lexical lane decides the callee's result class here, and the one route table turns
    // the class into the lane's route.
    use crate::mir::resolved_semantics::home_new_prefix::{CallLaneV1, LocalCallResultClassV1 as Class};
    let class = if matches!(
        claim,
        Some(crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1::NullableObject(_))
    ) {
        nullable_exits_hold(batch, target_batch_slot).then_some(Class::Nullable)?
    } else {
        all_formals_exact_i64(batch, parameter_contracts, callee_owner, arity)?;
        if crate::mir::normal_callable_semantic_package::direct_call_loan::lifecycle::construction_result_callee(
            batch,
            callee_owner,
        ) {
            Class::Handle
        } else {
            (claim.is_none() && borrowed_i64_results.is_normal_i64_result_v1(&callee_owner))
                .then_some(Class::I64)?
        }
    };
    LocalCallRouteV1::for_result(CallLaneV1::Lexical, class)
}

/// Every verified value-return of the callee is `return null` or a
/// construction; the claim names the class and the same sealed return sites
/// re-prove its shape, so the lane never infers nullability from MIR types or a
/// backend role.
fn nullable_exits_hold(batch: &VerifiedResolvedCallableSemanticBatchV1, target_batch_slot: u32) -> bool {
    batch
        .with_lowering_input(target_batch_slot, |callee_input| {
            let Some(shape) = callee_input.body_shape() else {
                return false;
            };
            let Some(sites) = super::super::verified_value_return_sites(callee_input, shape) else {
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

#[path = "ordinary_new_nullable_received_class.rs"]
mod nullable_received_class;
pub(in crate::mir::normal_callable_semantic_package) use nullable_received_class::nullable_received_result_class;



#[cfg(test)]
#[path = "ordinary_new_borrowed_callback_tests.rs"]
mod borrowed_callback_tests;

/// Preserve the existing source preflight order and retained per-slot errors.
pub(super) fn prepare_source_preflight_v1(
    brands: Option<&crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    omitted_static_callers: &BTreeMap<
        FunctionOwnerIdV1,
        crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    >,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    parameter_contracts: &[super::super::super::model::OwnedCallableParameterContractDeclarationV1],
    static_call_claims: &super::super::super::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1,
    app_main_batch_slot: Option<u32>,
    app_main: Option<&super::super::lexical_instance_call::BorrowedAppMainSourceLoanV1<'_>>,
    dynamic_slot: Option<u32>,
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    field_write_claims: &field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    callable_result_classes: &result_class_claim::OrdinaryNewResultClassClaimsV1,
    residences: &field_write_claim::OwnedFieldResidencesV1,
    array_i64_fields: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    array_field_write_candidates: &[array_i64_fields::ArrayFieldWriteCandidateV1],
    array_field_get_candidates: &[array_i64_fields::ArrayFieldGetCandidateV1],
    owned: &mut BTreeMap<hakorune_mir_defs::CanonicalObjectIdV1, Option<Box<[OwnedFieldChildV1]>>>,
) -> Result<
    (
        Box<[Box<str>]>,
        BTreeMap<u32, Result<Vec<OrdinaryNewCandidate>, OrdinaryNewCoSealIssueV1>>,
        PreparedLexicalInstanceCallSourceTargetsV1,
        Result<PreparedBorrowedFormalIngressV1, String>,
        Result<std::collections::BTreeSet<OwnedExprSiteV1>, String>,
        BorrowedResultCohortV1,
    ),
    OrdinaryNewCoSealIssueV1,
> {
    let names: Box<[Box<str>]> = batch
        .ordinary_box_coverage()
        .rows()
        .iter()
        .map(|row| row.name().to_owned().into_boxed_str())
        .collect();
    let local_candidates = source_claims::prepare_local_candidates_by_slot_v1(
        batch,
        selected,
        omitted_static_callers,
        instance_constructors,
        app_main_batch_slot,
        dynamic_slot,
    );
    let new_classes = local_candidates
        .values()
        .filter_map(|rows| rows.as_ref().ok())
        .flatten()
        .map(|candidate| (candidate.site.clone(), candidate.class.clone()))
        .collect();
    let (lexical_source_targets, borrowed_formal_source, static_source_sites, borrowed_i64_results) =
        super::super::lexical_instance_call::prepare_borrowed_profile_v1(
            brands,
            batch,
            selected,
            omitted_static_callers,
            instance_constructors,
            parameter_contracts,
            static_call_claims,
            app_main_batch_slot,
            app_main,
            dynamic_slot,
            entry_home_loans,
            &local_candidates,
            &new_classes,
            &names,
            field_write_claims,
            callable_result_classes,
            array_i64_fields,
            array_field_write_candidates,
            array_field_get_candidates,
            &mut |caller, input, call| {
                let Some(slot) = caller.declaration_slot() else {
                    return source_claims::stored_constructor_child_source_v1(caller, input, call);
                };
                source_claims::stored_child_source_v1(
                    batch,
                    instance_constructors,
                    selected,
                    entry_home_loans.for_batch_slot(slot),
                    slot,
                    input,
                    call,
                    residences,
                    callable_result_classes,
                    &local_candidates,
                )
            },
            &mut |reference, receiver, original| {
                source_claims::stored_child_receiver_v1(
                    batch,
                    instance_constructors,
                    callable_result_classes,
                    reference,
                    receiver,
                    original,
                    field_write_claims,
                    residences,
                    owned,
                )
            },
        )?;
    Ok((
        names,
        local_candidates,
        lexical_source_targets,
        borrowed_formal_source,
        static_source_sites,
        borrowed_i64_results,
    ))
}

/// Stored scalar source rows demand sealed I64/Bool; execution stays separate.
pub(super) fn has_stored_terminal_v1(
    rows: &PreparedLexicalInstanceCallSourceTargetsV1,
    results: &BorrowedResultCohortV1,
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
) -> Result<bool, OrdinaryNewCoSealIssueV1> {
    let owner = input.owner();
    let Some(rows) = rows.as_ref().ok() else {
        return Ok(false);
    };
    let candidates = || {
        rows.iter()
            .filter_map(|row| row.as_ref().ok()?.as_ref())
            .filter(|row| {
                row.call_site().owner() == owner
                    && row.stored_receiver().is_some()
                    && row.uses_existing_borrowed_result_v1()
            })
    };
    let Some(first) = candidates().next() else {
        return Ok(false);
    };
    let shape = input
        .body_shape()
        .filter(|shape| shape.owner() == owner)
        .ok_or_else(|| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: first.call_site().clone(),
            issue: "[freeze:contract][stored-child/original-return-shape]".into(),
        })?;
    // Source membership selects this obligation, never result-proof availability
    // or Completion success. Print/local operands keep their own consumers.
    let Some(row) = candidates().find(|row| {
        shape.statements().iter().any(|statement| {
            matches!(statement, crate::mir::resolved_semantics::BodyStatementShapeV1::Return {
            value: Some(value), .. } if value == row.call_site().site())
        })
    }) else {
        return Ok(false);
    };
    let demand = results
        .get(&owner)
        .ok_or_else(|| "[freeze:contract][stored-child/result-source-missing]".to_owned())
        .and_then(|proof| proof.as_ref().map_err(Clone::clone))
        .and_then(|proof| proof.require_source_scalar_v1());
    demand.map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
        site: row.call_site().clone(),
        issue,
    })?;
    Ok(true)
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_dispatch_tests.rs"]
mod static_dispatch_tests;

pub(super) use super::super::receiver_call_observation::has_object_receiver_call_at_v1;

#[cfg(test)]
#[path = "ordinary_new_receiver_source_role_tests.rs"]
mod receiver_source_role_tests;

#[cfg(test)]
#[path = "ordinary_new_object_arguments_port_tests.rs"]
mod object_arguments_port_tests;
