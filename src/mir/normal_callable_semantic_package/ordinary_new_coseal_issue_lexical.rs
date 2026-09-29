//! Claim-local `recv.m(...)` lexical handle-result probe for the co-seal walk.
use super::*;

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
    use crate::mir::resolved_semantics::{
        ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1,
        ResolvedAssignmentTargetV1,
    };
    if site.owner() != input.owner() {
        return false;
    }
    let Some((observed_site, call)) = input
        .function()
        .method_calls()
        .find(|(call_site, _)| *call_site == site.site())
    else {
        return false;
    };
    if observed_site != site.site() {
        return false;
    }
    let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
        call.receiver()
    else {
        return false;
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
        return false;
    }
    let mut initializers = input
        .function()
        .expression_source()
        .initializers()
        .filter(|initializer| initializer.binding() == binding);
    if initializers.next().is_none() || initializers.next().is_some() {
        return false;
    }
    let mut claim_rows = candidates
        .iter()
        .filter(|candidate| candidate.destination == binding);
    let Some(candidate) = claim_rows.next() else {
        return false;
    };
    if claim_rows.next().is_some() {
        return false;
    }
    let Some((_key, target_batch_slot)) =
        super::super::lexical_instance_call::unique_instance_target(
            selected,
            candidate.class.as_ref(),
            call.selector(),
            call.arity(),
        )
    else {
        return false;
    };
    let Some(callee_owner) = batch
        .declarations()
        .find(|declaration| declaration.batch_slot() == target_batch_slot)
        .map(|declaration| declaration.owner())
    else {
        return false;
    };
    let mut contracts = parameter_contracts
        .iter()
        .filter(|row| row.owner == callee_owner);
    let Some(contract) = contracts.next() else {
        return false;
    };
    if contracts.next().is_some()
        || contract.parameters.len() != call.arity() as usize
    {
        return false;
    }
    let formals_exact = batch
        .with_lowering_input(contract.batch_slot, |callee_input| {
            callee_input.owner() == contract.owner
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
                    })
        })
        .unwrap_or(false);
    formals_exact
        && crate::mir::normal_callable_semantic_package::direct_call_loan::lifecycle::construction_result_callee(
            batch,
            callee_owner,
        )
}
