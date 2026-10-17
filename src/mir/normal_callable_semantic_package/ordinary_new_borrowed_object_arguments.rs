//! Original qualified object-return source joins the shared argument lender.
//! Missing source support stays pending; no result/Normal/transport is issued.
use super::super::super::result_class_claim::{
    ObjectReturnCallQualificationV1, ResultWitnessStepV1,
};
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::ObjectCallSourceSupportV1;

pub(in crate::mir::normal_callable_semantic_package) fn project_pending_object_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    actuals: &PendingBorrowedFormalActualsV1,
    target: &LexicalInstanceCallSourceTargetV1,
    loan: &ObjectReturnCallQualificationV1,
    ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<ObjectCallSourceSupportV1, String> {
    if loan.call() != target.call_site()
        || loan.key() != target.target()
        || target.argument_sites().len() != loan.key().arity() as usize
        || target
            .object_return_sources()
            .is_some_and(|required| !required.iter().any(|original| original == loan))
        || loan.witnesses().is_empty()
        || loan.witnesses().iter().any(|witness| {
            !matches!(witness.step(),
            ResultWitnessStepV1::Call { site, key, callee, .. }
                if site == target.call_site() && key == target.target()
                && callee.site().owner() == target.callee_owner())
        })
    {
        return Err(freeze("borrowed-call/object-source-identity"));
    }
    project_pending_object_target_arguments_v1(source, contracts, actuals, target,
        ledger,
    )
}

/// Borrows the exact original target's input phase; grants no result membership.
/// Qualified callers validate their original qualification before this lender.
pub(in crate::mir::normal_callable_semantic_package) fn project_pending_object_target_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    actuals: &PendingBorrowedFormalActualsV1,
    target: &LexicalInstanceCallSourceTargetV1,
    ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<ObjectCallSourceSupportV1, String> {
    let site = target.call_site();
    let ingress = source.as_ref().map_err(Clone::clone)?;
    if let Some(Err(issue)) = actuals.get(site) {
        return Err(issue.clone());
    }
    let source_only_forward =
        super::object_source::selected_original_object_phase_v1(ingress, target.call_site())?;
    if source_only_forward
        && !matches!(actuals.get(site), Some(Ok(rows))
        if matches!(rows.phase, BorrowedCallActualEvidencePhaseV1::SourceObject(_) | BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(_)))
    {
        return Err(freeze("borrowed-object/source-phase-drift"));
    }
    if let Some(Ok(rows)) = actuals.get(site) {
        if matches!(&rows.phase, BorrowedCallActualEvidencePhaseV1::SourceInstance(_)
            | BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(_)) {
            // Original QualifiedObject membership excludes the Instance issuer.
            // Literal/executable Object rows keep their existing constructor law.
            if ingress.source_incoming.has_object_input_callee_v1(target) {
                return Err(freeze("borrowed-object/source-phase-drift"));
            }
            let mut originals = ingress.source_incoming.exact_rows().filter(|row| &row.call == site);
            let original = originals.next().ok_or_else(|| freeze("borrowed-instance/original-source-missing"))?;
            if originals.next().is_some() || original.source.require_instance()? != target
                || original.callee != target.callee_owner() {
                return Err(freeze("borrowed-call/object-source-identity"));
            }
            // SourceObject selection above is disjoint from this phase, so
            // delegation cannot retry this owner through the Instance port.
            let observed = matches!(&rows.phase, BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(_));
            let arguments = super::instance_source::project_pending_instance_source_arguments_with_array_context_v1(
                source, actuals, contracts, site, ledger,
            )?.ok_or_else(|| freeze("borrowed-instance/source-phase-drift"))?;
            return Ok(if observed { ObjectCallSourceSupportV1::Observed(arguments) }
                else { ObjectCallSourceSupportV1::SourceOnly(arguments) });
        }
        if let BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(identity) = &rows.phase {
            let mut calls = ingress
                .source_incoming
                .exact_rows()
                .filter(|row| &row.call == site);
            let call = calls
                .next()
                .ok_or_else(|| freeze("borrowed-guarded/original-call-missing"))?;
            if calls.next().is_some() || call.source.require_instance()? != target {
                return Err(freeze("borrowed-guarded/original-target"));
            }
            super::object_source::recheck_object_source_actuals_v1(
                ingress, contracts, call, rows, identity,
                ledger,
            )?;
            return Ok(ObjectCallSourceSupportV1::Observed(
                rows.ordered_arguments.clone(),
            ));
        }
        if let BorrowedCallActualEvidencePhaseV1::SourceObject(identity) = &rows.phase {
            let mut original = ingress
                .source_incoming
                .exact_rows()
                .filter(|row| &row.call == site);
            let call = original
                .next()
                .ok_or_else(|| freeze("borrowed-object/source-row-missing"))?;
            if original.next().is_some()
                || call.source.require_instance()? != target
                || call.callee != target.callee_owner()
                || identity.source != *target
                || identity.incoming_arguments != call.arguments
                || rows.ordered_arguments.len() != target.argument_sites().len()
                || !rows.opaque_actuals.is_empty()
            {
                return Err(freeze("borrowed-object/source-actual-identity"));
            }
            let rechecked = super::object_source::prepare_object_source_actuals_v1(
                ingress,
                contracts,
                site,
                &identity.candidates,
            )?
            .ok_or_else(|| freeze("borrowed-object/source-phase-drift"))?;
            if rows != &rechecked {
                return Err(freeze("borrowed-object/source-snapshot-drift"));
            }
            return Ok(ObjectCallSourceSupportV1::SourceOnly(
                rows.ordered_arguments.clone(),
            ));
        }
    }
    // A copied Executable marker cannot erase a selected original Instance
    // protocol. SourceObject and sealed Instance arms above own their checks.
    if matches!(actuals.get(site), Some(Ok(rows))
        if matches!(&rows.phase, BorrowedCallActualEvidencePhaseV1::Executable))
        && super::instance_source::selected_original_instance_forward_v1(ingress, site)? {
        return Err(freeze("borrowed-instance/source-phase-drift"));
    }
    let mut incoming = ingress.incoming.iter().filter(|row| &row.call == site);
    let Some(call) = incoming.next() else {
        return Ok(ObjectCallSourceSupportV1::Unavailable);
    };
    if incoming.next().is_some()
        || call.source.require_instance()? != target
        || call.callee != target.callee_owner()
    {
        return Err(freeze("borrowed-call/object-source-identity"));
    }
    if !ingress.definitions.contains_key(&call.callee) || !actuals.contains_key(site) {
        return Ok(ObjectCallSourceSupportV1::Unavailable);
    }
    let Some((original, arguments)) = lend_pending_borrowed_arguments_v1(source, actuals, site)?
    else {
        return Err(freeze("borrowed-call/object-source-identity"));
    };
    if !std::ptr::eq(original, call) {
        return Err(freeze("borrowed-call/object-source-identity"));
    }
    Ok(ObjectCallSourceSupportV1::Observed(
        arguments.to_vec().into_boxed_slice(),
    ))
}

/// Received self calls require the original entry receiver and covered local row.
/// A missing observation remains pending; contradictory identity is corruption.
pub(in crate::mir::normal_callable_semantic_package) fn corroborate_received_object_receiver_v1(
    target: &LexicalInstanceCallSourceTargetV1,
    loan: &ObjectReturnCallQualificationV1,
    destination: BindingRefV1,
    receiver: Option<BindingRefV1>,
    rows: &BTreeMap<
        OwnedExprSiteV1,
        super::super::super::receiver_call_observation::ReceiverCallClassObservationV1,
    >,
) -> Result<bool, String> {
    if !target.is_self_receiver() {
        return Ok(true);
    }
    if receiver.is_none() || Some(target.receiver_binding()?) != receiver {
        return Err(freeze("borrowed-call/object-receiver-identity"));
    }
    let Some(row) = rows.get(target.call_site()) else {
        return Ok(false);
    };
    if row.callee() != loan.key()
        || row.class() != loan.class()
        || row.destination() != destination
        || row.arguments().len() != target.argument_sites().len()
        || row
            .arguments()
            .iter()
            .zip(target.argument_sites())
            .enumerate()
            .any(|(ordinal, (arg, original))| {
                arg.ordinal() as usize != ordinal || arg.site() != original
            })
    {
        return Err(freeze("borrowed-call/object-receiver-identity"));
    }
    Ok(true)
}

#[cfg(test)]
#[path = "ordinary_new_object_argument_identity_tests.rs"]
mod identity_tests;
