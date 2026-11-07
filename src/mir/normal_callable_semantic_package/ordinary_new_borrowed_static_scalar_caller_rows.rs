//! Caller rows of a static scalar input: exact-usize caller check, ready rows and the
//! original scalar source. Split out of the scalar input finish owner by responsibility.
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::{PhysicalCallableLaneRoleV1, VerifiedCallablePhysicalSignatureCohortV1},
    qualified_static_call_claim::incoming_source::StaticIncomingSourceV1,
    result_contract::VerifiedCallableResultContractCohortV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1,
    OrdinaryNewClaimLedgerV1,
};
use crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use std::collections::BTreeSet;
use std::rc::Rc;

pub(in super::super) fn check_exact_usize_scalar_caller_v1(
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    original: &Rc<StaticIncomingSourceV1>,
    binding: BindingRefV1,
) -> Result<(), String> {
    let mut callers = contracts
        .iter()
        .filter(|row| row.owner == original.call_site().owner());
    let caller = callers
        .next()
        .ok_or_else(|| freeze("static-scalar/caller-contract-missing"))?;
    let caller_signature = signatures
        .row(caller.batch_slot)
        .ok_or_else(|| freeze("static-scalar/caller-signature-missing"))?;
    if callers.next().is_some()
        || caller.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
        || caller.parameters.len() != 1
        || caller.parameters[0].ordinal != 0
        || caller.parameters[0].binding != binding
        || caller.parameters[0].kind
            != CallableParameterContractKindV1::ExactTrivial(ExactTrivialParameterAbiV1::USIZE)
        || !matches!(selected.key_for_batch_slot(caller.batch_slot),
            Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key == original.caller())
        || caller_signature.owner() != caller.owner
        || caller_signature.mode() != caller.mode
        || caller_signature.source_logical_arity() != 1
        || caller_signature.lanes().len() != 1
        || caller_signature.lanes()[0].role() != PhysicalCallableLaneRoleV1::OrdinaryScalar
        || caller_signature.lanes()[0].binding() != binding
    {
        return Err(freeze("static-scalar/caller-formal-drift"));
    }
    Ok(())
}

/// Keep qualified callers in their already-issued execution protocol. The
/// existing qualified projection checks original inventory, payload and required
/// Integer domains; this cannot recurse because only Executable is selected.
pub(in super::super) fn qualified_scalar_actual_v1(
    source: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    original: &Rc<StaticIncomingSourceV1>,
    completion: &Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
) -> Result<bool, String> {
    if original.qualified_scalar_result_v1().is_none() {
        return Ok(false);
    }
    let Some(actual) = pending.get(original.call_site()) else {
        return Ok(false);
    };
    let actual = actual.as_ref().map_err(Clone::clone)?;
    if let BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(proof) = &actual.phase {
        if let StaticScalarActualV1::QualifiedExecutable(snapshot) = &proof.actual {
            if !proof.corroborates(original, completion)
                || !source.is_compare_only_static_input_v1(original.callee_owner())
                || snapshot.opaque_actuals != actual.opaque_actuals
                || snapshot.ordered_arguments != actual.ordered_arguments
            {
                return Err(freeze("static-scalar/qualified-snapshot-drift"));
            }
            let call = source
                .scalar_qualified_source_call_v1(original.call_site())?
                .ok_or_else(|| freeze("static-scalar/qualified-source-missing"))?;
            super::static_source::checked_executable_qualified_arguments_v1(
                source,
                call,
                snapshot,
                original.required_i64_arguments(),
            )?;
            return Ok(true);
        }
        let StaticScalarActualV1::QualifiedLiteral(identity) = &proof.actual else {
            return Ok(false);
        };
        if !proof.corroborates(original, completion) {
            return Err(freeze("static-scalar/qualified-completion-drift"));
        }
        let Some(value) = super::static_tagged_input_finish::literal_identity_v1(
            source,
            original,
            identity,
            actual,
            original.parameters()[0].binding,
        )?
        else {
            return Ok(false);
        };
        if actual.opaque_actuals.as_ref()
            != [PreparedBorrowedFormalActualV1 {
                ordinal: 0,
                site: original.argument_sites()[0].clone(),
                formal: original.parameters()[0].binding,
                source: BorrowedFormalActualSourceV1::Integer(value),
            }]
        {
            return Err(freeze("static-scalar/qualified-payload-drift"));
        }
        return Ok(true);
    }
    if !matches!(actual.phase, BorrowedCallActualEvidencePhaseV1::Executable) {
        return Ok(false);
    }
    let mut originals = source
        .source_incoming
        .exact_rows()
        .filter(|row| &row.call == original.call_site() && row.callee == original.callee_owner());
    let call = originals
        .next()
        .ok_or_else(|| freeze("static-scalar/incoming-missing"))?;
    if originals.next().is_some() {
        return Err(freeze("static-scalar/incoming-identity"));
    }
    super::static_source::checked_executable_qualified_arguments_v1(
        source,
        call,
        actual,
        original.required_i64_arguments(),
    )?;
    Ok(true)
}

/// One whole original cohort, with each row's own issued argument protocol.
/// Qualified rows are borrowed, never reissued as current-owner Scalar rows.
pub(in super::super) fn checked_ready_scalar_rows_v1<'a>(
    source: &'a PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    original: &Rc<StaticIncomingSourceV1>,
    completion: &Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
) -> Result<
    Option<Box<[&'a super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1]>>,
    String,
> {
    let rows = source
        .source_incoming
        .project(&BTreeSet::from([original.callee_owner()]))
        .map_err(|error| format!("{}: {error:?}", freeze("static-scalar/incoming-veto")))?;
    let cohort = source.static_incoming_cohort_v1(original)?;
    let owner = original.callee_owner();
    let compare_only = source.is_compare_only_static_input_v1(owner);
    if !cohort
        .iter()
        .any(|row| row.is_current_owner_i64_source_v1())
        && !cohort.iter().all(|row| {
            (source.is_static_terminal_source_v1(row) || compare_only)
                && source.checked_static_input(row.parameters()[0].binding)
                && matches!(pending.get(row.call_site()), Some(Ok(actual))
                    if matches!(&actual.phase,
                        BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(proof)
                            if matches!(proof.actual, StaticScalarActualV1::QualifiedLiteral(_)
                                | StaticScalarActualV1::QualifiedExecutable(_))))
        })
    {
        return Ok(None);
    }
    if cohort.len() != rows.len() {
        return Err(freeze("static-scalar/whole-cohort"));
    }
    let mut retained = Vec::with_capacity(rows.len());
    for original in &cohort {
        let mut incoming = source
            .source_incoming
            .exact_rows()
            .filter(|call| call.callee == owner && call.call == *original.call_site());
        let call = incoming
            .next()
            .ok_or_else(|| freeze("static-scalar/incoming-missing"))?;
        if incoming.next().is_some()
            || !matches!(&call.source,
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(source)
                    if Rc::ptr_eq(source, original))
            || !(original.is_current_owner_i64_source_v1() || original.qualified_scalar_result_v1().is_some())
            || !(original.required_i64_arguments() == [0]
                || original.required_i64_arguments().is_empty())
            || original.parameters().len() != 1
            || !original.parameters()[0].kind.is_ordinary_borrowed_handle()
            || call.arguments.as_ref()
                != [(
                    0,
                    original.argument_sites()[0].clone(),
                    original.parameters()[0].binding,
                )]
        {
            return Err(freeze("static-scalar/incoming-identity"));
        }
        let Some(actual) = pending.get(&call.call) else {
            return Ok(None);
        };
        let actual = actual.as_ref().map_err(Clone::clone)?;
        if original.qualified_scalar_result_v1().is_some() {
            if !qualified_scalar_actual_v1(source, pending, original, completion)? {
                return Ok(None);
            }
            retained.push(call);
            continue;
        }
        let BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(proof) = &actual.phase else {
            return Ok(None);
        };
        let StaticScalarActualV1::CurrentOwner(proof_binding) = &proof.actual else {
            return Err(freeze("static-scalar/actual-source-kind"));
        };
        if !proof.corroborates(original, completion)
            || actual.ordered_arguments.as_ref()
                != [LocalCallArgumentV1::BorrowedActual {
                    ordinal: 0,
                    site: original.argument_sites()[0].clone(),
                }]
            || actual.opaque_actuals.len() != 1
            || actual.opaque_actuals[0].ordinal != 0
            || actual.opaque_actuals[0].site != original.argument_sites()[0]
            || actual.opaque_actuals[0].formal != original.parameters()[0].binding
            || !matches!(&actual.opaque_actuals[0].source,
                BorrowedFormalActualSourceV1::Scalar { binding, kind: SourceScalarKind::Integer }
                    if binding == proof_binding && binding.owner() == call.call.owner())
        {
            return Err(freeze("static-scalar/actual-identity"));
        }
        retained.push(call);
    }
    Ok(Some(retained.into_boxed_slice()))
}

/// Original scalar identity survives input finishing. This is only a source
/// loan; callers still demand the whole cohort before executable projection.
pub(in super::super) fn original_scalar_source_v1(
    actual: &PreparedBorrowedCallActualsV1,
) -> Result<Option<(&Rc<StaticIncomingSourceV1>, BindingRefV1, &SourceExprSiteV1)>, String> {
    let (original, binding, site) = match &actual.phase {
        BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) => {
            let [candidate] = identity.candidates.as_ref() else {
                return Ok(None);
            };
            let BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer) =
                candidate.value
            else {
                return Ok(None);
            };
            if candidate.ordinal != 0
                || identity.integer_evidence.as_ref() != [true]
                || !actual.opaque_actuals.is_empty()
            {
                return Err(freeze("borrowed-static/loop-scalar-source-drift"));
            }
            (&identity.source, binding, &candidate.site)
        }
        BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(proof) => {
            let StaticScalarActualV1::CurrentOwner(scalar_binding) = &proof.actual else {
                return Ok(None);
            };
            let [payload] = actual.opaque_actuals.as_ref() else {
                return Ok(None);
            };
            if !proof.corroborates(&proof.source, &proof.completion)
                || payload.ordinal != 0
                || proof.source.parameters().first().map(|p| p.binding) != Some(payload.formal)
                || !matches!(payload.source,
                    BorrowedFormalActualSourceV1::Scalar { binding, kind: SourceScalarKind::Integer }
                    if binding == *scalar_binding)
            {
                return Err(freeze("borrowed-static/loop-scalar-source-drift"));
            }
            (&proof.source, *scalar_binding, &payload.site)
        }
        _ => return Ok(None),
    };
    if original.call_site().owner() != binding.owner()
        || original.argument_sites() != [site.clone()]
        || actual.ordered_arguments.as_ref()
            != [LocalCallArgumentV1::BorrowedActual {
                ordinal: 0,
                site: site.clone(),
            }]
    {
        return Err(freeze("borrowed-static/loop-scalar-source-drift"));
    }
    Ok(Some((original, binding, site)))
}

