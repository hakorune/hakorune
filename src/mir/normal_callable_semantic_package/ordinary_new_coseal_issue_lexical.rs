//! Claim-local `recv.m(...)` lexical call probes for the co-seal walk.
use super::*;
use crate::mir::resolved_semantics::BindingKindV1;

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

/// A `recv.m(...)` local-call site whose receiver is a claim-local `new`
/// binding and whose uniquely selected callee returns `new` of one class.
///
/// This is the scan-side mirror of `issue_lexical_instance_call_dispositions`
/// bounded to claim-local receivers: `candidates` stands in for this
/// declaration's not-yet-committed claims, the callee's parameter contract
/// proves every formal is an exact i64, and the callee's body shape proves
/// the uniform construction result. Parameter receivers, nested
/// handle-result receivers, and rebound bindings prove nothing here — the
/// site simply stays on its existing path.
pub(super) fn lexical_handle_result_call(
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    parameter_contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    candidates: &[OrdinaryNewCandidate],
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
) -> bool {
    let Some((_key, _slot, callee_owner, arity)) =
        lexical_claim_local_target(selected, batch, candidates, input, site)
    else {
        return false;
    };
    all_formals_exact_i64(batch, parameter_contracts, callee_owner, arity).is_some()
        && crate::mir::normal_callable_semantic_package::direct_call_loan::lifecycle::construction_result_callee(
            batch,
            callee_owner,
        )
}

/// A `recv.m(...)` local-call site whose receiver is a claim-local `new`
/// binding and whose uniquely selected callee's every verified value-return
/// is a literal or an exact-`i64` formal — the sole scan-side proof that
/// the callee's co-sealed disposition mints `InvokeCallResultKind::I64`.
///
/// Why this shape only: the callee's terminal relations are minted by the
/// same cohort walk that needs this predicate, so the scan cannot re-read
/// them. A literal `return` always classifies to an i64-kind relation
/// (`IntegerLiteral`, scalar `OtherTrivial`, or a `Value` row whose
/// `String`/`Null`/`Float` source maps to `I64`), and a `return` of a
/// formal the callee's own contract proves `ExactTrivial(I64)` can never
/// resolve to a `Map`/`Handle`/`Construction` source either — while every
/// other shape can hide a relation the sealed disposition would record
/// differently. Claiming such a site and then disagreeing at emission is
/// a freeze, so the gate stays fail-closed and the unclaimed call keeps
/// its existing dynamic path. A class-claimed callee
/// (`Object`/`NullableObject`) likewise belongs to another lane. The
/// symmetric Standard-route emitter corroborates the minted row's
/// `result == Some(I64)` — a half-sealed edge freezes, never degrades.
pub(super) fn lexical_i64_result_call(
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    parameter_contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    callable_result_classes: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    candidates: &[OrdinaryNewCandidate],
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
) -> bool {
    let Some((key, target_batch_slot, callee_owner, arity)) =
        lexical_claim_local_target(selected, batch, candidates, input, site)
    else {
        return false;
    };
    let Some(contract) = all_formals_exact_i64(batch, parameter_contracts, callee_owner, arity)
    else {
        return false;
    };
    if callable_result_classes.contains_key(&key) {
        return false;
    }
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
                    function.expression_source().literal(site).is_some()
                        || matches!(
                            function.variable_ref(site),
                            Some(
                                crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(
                                    binding
                                )
                            ) if contract
                                .parameters
                                .iter()
                                .any(|parameter| parameter.binding == binding)
                        )
                })
        })
        .unwrap_or(false)
}
