//! Independent reconstruction and current-source validation of the SAME Instance actual row.
use super::*;
use super::instance_source::{InstanceSourceActualIdentityV1, prepare_instance_source_actuals_v1};

pub(super) fn reconstruct_instance_source_actuals_v1(
    ingress: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    identity: &InstanceSourceActualIdentityV1,
    array_ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<Option<PreparedBorrowedCallActualsV1>, String> {
    if identity.source != *call.source.require_instance()?
        || identity.incoming_arguments != call.arguments
    {
        return Err(freeze("borrowed-instance/original-input"));
    }
    let mut original =
        prepare_instance_source_actuals_v1(ingress, contracts, &call.call, &identity.candidates)?
            .ok_or_else(|| freeze("borrowed-instance/source-unavailable"))?;
    model::retain_instance_argument_sources_v1(&mut original, identity)?;
    if original.phase != BorrowedCallActualEvidencePhaseV1::SourceInstance(identity.clone()) {
        return Err(freeze("borrowed-instance/source-identity"));
    }
    if !identity.field_loans.is_empty() && array_ledger.is_none() {
        return Err(freeze("borrowed-instance/field-execution-unconnected"));
    }
    if !identity.array_loans.is_empty() && array_ledger.is_none() {
        return Err(freeze("borrowed-instance/array-execution-unconnected"));
    }
    if let Some(ledger) = array_ledger {
        if !identity.array_loans.is_empty() { ledger.require_original_array_ingress_v1(ingress)?; }
        for (_, loan) in &identity.array_loans {
            if ledger.retained_owned_array_read_source_v1(loan)?.is_none() { return Ok(None); }
        }
    }
    let Some(mut result) = super::object_source::reconstruct_original_argument_protocol_with_sources_v1(
        ingress, contracts, call, &identity.candidates,
        &mut |ordinal, site, formal| {
            let (_, expected_site, expected_formal) = identity.incoming_arguments.iter().find(|(index, _, _)| *index == ordinal)
                .ok_or_else(|| freeze("borrowed-instance/array-original-ordinal"))?;
            if expected_site != site || *expected_formal != formal {
                return Err(freeze("borrowed-instance/read-original-identity"));
            }
            let candidate = identity.candidates.get(ordinal as usize).ok_or_else(|| freeze("borrowed-instance/actual-identity"))?;
            if matches!(candidate.value, BorrowedCallActualValueV1::Expression | BorrowedCallActualValueV1::String(_)) {
                return array_ledger.map(|ledger| ledger.borrowed_expression_actual_source_v1(&call.call, ordinal, site)
                    .map(|actual| Some(actual.source))).unwrap_or(Ok(None));
            }
            if let BorrowedCallActualValueV1::MethodCall(method) = &candidate.value {
                let parameter = contracts.iter().find(|row| row.owner == call.callee)
                    .and_then(|row| row.parameters.get(ordinal as usize))
                    .filter(|row| row.ordinal == ordinal && row.binding == formal)
                    .ok_or_else(|| freeze("borrowed-instance/actual-identity"))?;
                if static_source::ordinary_text_argument_source_v1(ingress,
                    &OwnedExprSiteV1::new(call.call.owner(), site.clone()), method)?.is_some()
                    || static_source::core_argument_source_v1(ingress,
                        &OwnedExprSiteV1::new(call.call.owner(), site.clone()), method, &parameter.kind)?.is_some() {
                    let actual = array_ledger.ok_or_else(|| freeze("borrowed-instance/text-execution-unconnected"))?
                        .borrowed_expression_actual_source_v1(&call.call, ordinal, site)?;
                    if actual.formal != formal { return Err(freeze("borrowed-instance/text-formal-identity")); }
                    return Ok(Some(actual.source));
                }
            }
            if let Some((_, loan)) = identity.field_loans.iter().find(|(index, _)| *index == ordinal) {
                return array_ledger.ok_or_else(|| freeze("borrowed-instance/field-execution-unconnected"))?
                    .retained_returned_field_read_source_v1(&call.call, ordinal, loan.parent(), loan.acquisition())?
                    .map(crate::mir::normal_callable_semantic_package::ReadActualSourceLoanV1::from_field_v1).transpose()
                    .map(|row| row.map(BorrowedFormalActualSourceV1::Read));
            }
            let (_, loan) = identity.array_loans.iter().find(|(index, _)| *index == ordinal)
                .ok_or_else(|| freeze("borrowed-instance/array-original-loan"))?;
            if expected_site != site || *expected_formal != formal || loan.read().site() != site || loan.read().owner() != call.call.owner() {
                return Err(freeze("borrowed-instance/array-original-identity"));
            }
            array_ledger.ok_or_else(|| freeze("borrowed-instance/array-execution-unconnected"))?
                .retained_owned_array_read_source_v1(loan)
                .map(|source| source.map(crate::mir::normal_callable_semantic_package::ReadActualSourceLoanV1::from_array_v1))
                .map(|row| row.map(BorrowedFormalActualSourceV1::Read))
        },
        array_ledger,
    )?
    else {
        return Ok(None);
    };
    if result.ordered_arguments != original.ordered_arguments {
        return Err(freeze("borrowed-instance/ordered-input"));
    }
    result.phase = BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(identity.clone());
    Ok(Some(result))
}

pub(super) fn recheck_instance_source_actuals_v1(
    ingress: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    rows: &PreparedBorrowedCallActualsV1,
    identity: &InstanceSourceActualIdentityV1,
    array_ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<(), String> {
    let rebuilt = reconstruct_instance_source_actuals_v1(ingress, contracts, call, identity, array_ledger)?
        .ok_or_else(|| freeze("borrowed-instance/recheck-unavailable"))?;
    if &rebuilt != rows {
        return Err(freeze("borrowed-instance/reconstructed-snapshot"));
    }
    Ok(())
}

/// Canonical source-demand boundary checks current facts, not the loan's copy.
pub(in crate::mir::normal_callable_semantic_package) fn recheck_pending_array_source_actuals_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    classes: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    targets: &super::super::PreparedLexicalInstanceCallSourceTargetsV1,
    site: &OwnedExprSiteV1,
) -> Result<(), String> {
    let Some(Ok(rows)) = pending.get(site) else {
        return Ok(());
    };
    let identity = match &rows.phase {
        BorrowedCallActualEvidencePhaseV1::SourceInstance(identity) => identity,
        BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(identity) => {
            rows.require_executable_v1()?;
            identity
        }
        _ => return Ok(()),
    };
    let ingress = source.as_ref().map_err(Clone::clone)?;
    let mut current_targets = BTreeMap::new();
    if !identity.array_loans.is_empty() {
        for row in targets.as_ref().map_err(Clone::clone)? {
            if let Some(target) = row.as_ref().map_err(Clone::clone)? {
                if current_targets
                    .insert(target.call_site().clone(), target)
                    .is_some()
                {
                    return Err(freeze("array-source/producer-duplicate"));
                }
            }
        }
    }
    for (ordinal, loan) in identity.array_loans.iter() {
        if identity.source.call_site() != site
            || identity.source.argument_sites().get(*ordinal as usize) != Some(loan.read().site())
        {
            return Err(freeze("borrowed-instance/array-loan-identity"));
        }
        loan.recheck(ingress, classes, &current_targets)?;
    }
    Ok(())
}

/// Source-domain validation only, used outside qualified Static execution scope.
pub(in crate::mir::normal_callable_semantic_package) fn revalidate_passive_instance_incoming_v1(
    ingress: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    rows: &PreparedBorrowedCallActualsV1,
    classes: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
) -> Result<bool, String> {
    revalidate_passive_instance_incoming_with_array_context_v1(ingress, contracts, call, rows, classes, None)
}

pub(in crate::mir::normal_callable_semantic_package) fn revalidate_passive_instance_incoming_with_array_context_v1(
    ingress: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    rows: &PreparedBorrowedCallActualsV1,
    classes: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    array_ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<bool, String> {
    let identity = match &rows.phase {
        BorrowedCallActualEvidencePhaseV1::SourceInstance(identity) => identity,
        BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(identity) => {
            recheck_instance_source_actuals_v1(ingress, contracts, call, rows, identity, array_ledger)?;
            rows.require_executable_v1()?;
            return Ok(false);
        }
        _ => return Ok(false),
    };
    let original = crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1::original_instance_input_call_v1(ingress, contracts, &call.call)?
        .ok_or_else(|| freeze("borrowed-instance/source-unavailable"))?;
    if original.callee != call.callee
        || original.arguments != call.arguments
        || original.source.require_instance()? != call.source.require_instance()?
        || identity.source != *call.source.require_instance()?
        || identity.incoming_arguments != call.arguments
    {
        return Err(freeze("borrowed-instance/original-input"));
    }
    let mut rebuilt = prepare_instance_source_actuals_v1(
        ingress, contracts, &call.call, &identity.candidates,
    )?.ok_or_else(|| freeze("borrowed-instance/source-unavailable"))?;
    model::retain_instance_argument_sources_v1(&mut rebuilt, identity)?;
    if &rebuilt != rows || !rows.opaque_actuals.is_empty() {
        return Err(freeze("borrowed-instance/reconstructed-snapshot"));
    }
    let mut targets = BTreeMap::new();
    for (ordinal, loan) in identity.array_loans.iter() {
        if identity.source.argument_sites().get(*ordinal as usize) != Some(loan.read().site()) {
            return Err(freeze("borrowed-instance/array-loan-identity"));
        }
        for (producer, _) in loan.original_producer_inputs_v1(ingress, contracts)? {
            let current = ingress.object_source_target_at_v1(&producer.call)?
                .ok_or_else(|| freeze("array-source/producer-missing"))?;
            targets.insert(producer.call.clone(), current);
        }
        loan.recheck(ingress, classes, &targets)?;
    }
    // Passive validation skips executable actuals, so it must independently
    // retain the original whole-incoming formal class agreement.
    for (ordinal, _, formal) in call.arguments.iter() {
        let view = ingress.source_object_view_v1(*formal)?;
        let candidate = identity.candidates.get(*ordinal as usize)
            .ok_or_else(|| freeze("borrowed-instance/actual-identity"))?;
        let class = match &candidate.value {
            BorrowedCallActualValueV1::ReceivedField { parent, acquisition } => {
                let (_, loan) = identity.field_loans.iter().find(|(index, _)| index == ordinal)
                    .ok_or_else(|| freeze("borrowed-instance/field-loan-missing"))?;
                let current = ingress.original_returned_field_actual_source_v1(&call.call, *ordinal)?
                    .ok_or_else(|| freeze("borrowed-instance/field-original-source-missing"))?;
                if current != loan || loan.parent() != *parent || loan.acquisition() != acquisition {
                    return Err(freeze("borrowed-instance/field-original-source-drift"));
                }
                if let Some(ledger) = array_ledger {
                    classes.recheck_returned_field_graph_v1(loan, &ledger.field_write_claims,
                        &mut |site| ingress.object_source_target_at_v1(site).map_err(|_| "returned-field/current-target-identity"))
                        .map_err(freeze)?;
                }
                loan.class()
            }
            BorrowedCallActualValueV1::MethodCall(ref method) => {
                let parameter = contracts.iter().find(|row| row.owner == call.callee)
                    .and_then(|row| row.parameters.get(*ordinal as usize))
                    .filter(|row| row.ordinal == *ordinal && row.binding == *formal)
                    .ok_or_else(|| freeze("borrowed-instance/actual-identity"))?;
                if static_source::ordinary_text_argument_source_v1(ingress,
                    &OwnedExprSiteV1::new(call.call.owner(), candidate.site.clone()), method)?.is_some()
                    || static_source::core_argument_source_v1(ingress,
                        &OwnedExprSiteV1::new(call.call.owner(), candidate.site.clone()), method, &parameter.kind)?.is_some() {
                    if view.is_some() { return Err(freeze("borrowed-entry/object-view-drift")); }
                    continue;
                }
                let loan = identity.array_loans.iter().find(|(index, _)| index == ordinal)
                    .ok_or_else(|| freeze("borrowed-instance/array-loan-missing"))?;
                if candidate.ordinal != *ordinal || !loan.1.corroborates_method_source_v1(method) {
                    return Err(freeze("borrowed-instance/method-original-source-drift"));
                }
                let requirement = ingress.array_element_source_v1(loan.1.read())?
                    .ok_or_else(|| freeze("array-source/read-missing"))?;
                Some(requirement.class())
            }
            BorrowedCallActualValueV1::SelfRooted { root, .. } =>
                ingress.source_object_view_v1(*root)?.map(|view| view.class()),
            BorrowedCallActualValueV1::ReceivedNullable(_) | BorrowedCallActualValueV1::ReceivedMixed(_) => {
                let loan = ingress.original_mixed_input_source_v1(call.source.require_instance()?)?
                    .ok_or_else(|| freeze("borrowed-instance/mixed-request-missing"))?;
                classes.recheck_mixed_local_roots_v1(loan).map_err(|issue| freeze(issue))?;
                Some(loan.fresh_class())
            }
            BorrowedCallActualValueV1::Null => continue,
            _ => None,
        };
        if view.is_some_and(|view| class != Some(view.class()))
            || (matches!(candidate.value, BorrowedCallActualValueV1::MethodCall(_) | BorrowedCallActualValueV1::ReceivedField { .. }) && view.is_none())
        {
            return Err(freeze("borrowed-entry/object-view-drift"));
        }
    }
    Ok(true)
}
