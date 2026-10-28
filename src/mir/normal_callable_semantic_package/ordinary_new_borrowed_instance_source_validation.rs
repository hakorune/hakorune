//! Independent reconstruction and current-source validation of the SAME Instance actual row.
use super::instance_source::{
    prepare_instance_source_actuals_v1, selected_original_instance_forward_v1,
    InstanceSourceActualIdentityV1,
};
use super::*;

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
        if !identity.array_loans.is_empty() {
            ledger.require_original_array_ingress_v1(ingress)?;
        }
        for (_, loan) in &identity.array_loans {
            if ledger.retained_owned_array_read_source_v1(loan)?.is_none() {
                return Ok(None);
            }
        }
    }
    // An original source slot is not a qualified executable actual. Preserve
    // the existing pending phase when the real Prefix cannot yet qualify it;
    // missing/foreign source and direct execution demands still refuse.
    let mut pending_expression = false;
    for candidate in &identity.candidates {
        if !matches!(
            candidate.value,
            BorrowedCallActualValueV1::Expression
                | BorrowedCallActualValueV1::String(_)
                | BorrowedCallActualValueV1::Binding(_)
                | BorrowedCallActualValueV1::Unknown
        ) && !(matches!(candidate.value, BorrowedCallActualValueV1::FieldAccess)
            && ingress.original_map_field_actual_request_v1(&call.call, candidate.ordinal, &candidate.site)?) {
            continue;
        }
        match identity.argument_sources.get(&candidate.ordinal) {
            Some(model::ArgumentExpressionV1::Source(_)) => {
                let local_object = original_local_object_binding_actual_v1(
                    ingress,
                    contracts,
                    call,
                    identity,
                    candidate,
                )
                .is_some();
                // A map-held payload field read carries its own original get
                // and declared field source; it is not an unqualified Prefix slot.
                let map_field = matches!(candidate.value, BorrowedCallActualValueV1::FieldAccess);
                if !local_object
                    && !map_field
                    && (!matches!(candidate.value, BorrowedCallActualValueV1::Unknown)
                        || original_unknown_scalar_actual_v1(ingress, call, identity, candidate)
                            .is_none())
                {
                    pending_expression = true;
                }
            }
            Some(model::ArgumentExpressionV1::Qualified(_)) => {}
            None => return Err(freeze("borrowed-instance/source-identity")),
        }
    }
    if pending_expression {
        if let Some(ledger) = array_ledger {
            let retained = ledger
                .borrowed_formal_source
                .as_ref()
                .ok_or_else(|| freeze("borrowed-object/source-ledger-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            if !std::ptr::eq(retained, ingress) {
                return Err(freeze("borrowed-object/source-ledger-drift"));
            }
        }
        return Ok(None);
    }
    let reconstructed = super::object_source::reconstruct_original_argument_protocol_with_sources_v1(
            ingress,
            contracts,
            call,
            &identity.candidates,
            &mut |ordinal, site, formal| {
                let (_, expected_site, expected_formal) = identity
                    .incoming_arguments
                    .iter()
                    .find(|(index, _, _)| *index == ordinal)
                    .ok_or_else(|| freeze("borrowed-instance/array-original-ordinal"))?;
                if expected_site != site || *expected_formal != formal {
                    return Err(freeze("borrowed-instance/read-original-identity"));
                }
                let candidate = identity
                    .candidates
                    .get(ordinal as usize)
                    .ok_or_else(|| freeze("borrowed-instance/actual-identity"))?;
                if matches!(candidate.value, BorrowedCallActualValueV1::Unknown) {
                    if let Some(source) =
                        original_unknown_scalar_actual_v1(ingress, call, identity, candidate)
                    {
                        return Ok(Some(source));
                    }
                }
                if let Some(source) = original_local_object_binding_actual_v1(
                    ingress,
                    contracts,
                    call,
                    identity,
                    candidate,
                ) {
                    return Ok(Some(source));
                }
                if matches!(
                    candidate.value,
                    BorrowedCallActualValueV1::Expression
                        | BorrowedCallActualValueV1::String(_)
                        | BorrowedCallActualValueV1::Binding(_)
                        | BorrowedCallActualValueV1::Unknown
                ) {
                    return array_ledger
                        .map(|ledger| {
                            ledger
                                .borrowed_expression_actual_source_v1(&call.call, ordinal, site)
                                .map(|actual| Some(actual.source))
                        })
                        .unwrap_or(Ok(None));
                }
                if let BorrowedCallActualValueV1::MethodCall(method) = &candidate.value {
                    let parameter = contracts
                        .iter()
                        .find(|row| row.owner == call.callee)
                        .and_then(|row| row.parameters.get(ordinal as usize))
                        .filter(|row| row.ordinal == ordinal && row.binding == formal)
                        .ok_or_else(|| freeze("borrowed-instance/actual-identity"))?;
                    if static_source::ordinary_text_argument_source_v1(
                        ingress,
                        &OwnedExprSiteV1::new(call.call.owner(), site.clone()),
                        method,
                    )?
                    .is_some()
                        || static_source::core_argument_source_v1(
                            ingress,
                            &OwnedExprSiteV1::new(call.call.owner(), site.clone()),
                            method,
                            &parameter.kind,
                        )?
                        .is_some()
                    {
                        let actual = array_ledger
                            .ok_or_else(|| freeze("borrowed-instance/text-execution-unconnected"))?
                            .borrowed_expression_actual_source_v1(&call.call, ordinal, site)?;
                        if actual.formal != formal {
                            return Err(freeze("borrowed-instance/text-formal-identity"));
                        }
                        return Ok(Some(actual.source));
                    }
                }
                if matches!(candidate.value, BorrowedCallActualValueV1::FieldAccess) {
                    if let Some((read, _get, declared)) =
                        ingress.original_map_field_actual_source_v1(&call.call, ordinal, site)?
                    {
                        if read.site() != site {
                            return Err(freeze("borrowed-instance/map-field-original-identity"));
                        }
                        let class = ingress
                            .field_value_class_v1(declared)
                            .ok_or_else(|| freeze("borrowed-instance/map-field-class"))?;
                        return Ok(Some(BorrowedFormalActualSourceV1::Read(
                            crate::mir::normal_callable_semantic_package::ReadActualSourceLoanV1::from_map_field_v1(
                                read.clone(), class,
                            ),
                        )));
                    }
                }
                if let Some((_, loan)) = identity
                    .field_loans
                    .iter()
                    .find(|(index, _)| *index == ordinal)
                {
                    return array_ledger.ok_or_else(|| freeze("borrowed-instance/field-execution-unconnected"))?
                    .retained_returned_field_read_source_v1(&call.call, ordinal, loan.parent(), loan.acquisition())?
                    .map(crate::mir::normal_callable_semantic_package::ReadActualSourceLoanV1::from_field_v1).transpose()
                    .map(|row| row.map(BorrowedFormalActualSourceV1::Read));
                }
                let (_, loan) = identity
                    .array_loans
                    .iter()
                    .find(|(index, _)| *index == ordinal)
                    .ok_or_else(|| freeze("borrowed-instance/array-original-loan"))?;
                if expected_site != site
                    || *expected_formal != formal
                    || loan.read().site() != site
                    || loan.read().owner() != call.call.owner()
                {
                    return Err(freeze("borrowed-instance/array-original-identity"));
                }
                array_ledger.ok_or_else(|| freeze("borrowed-instance/array-execution-unconnected"))?
                .retained_owned_array_read_source_v1(loan)
                .map(|source| source.map(crate::mir::normal_callable_semantic_package::ReadActualSourceLoanV1::from_array_v1))
                .map(|row| row.map(BorrowedFormalActualSourceV1::Read))
            },
            array_ledger,
        )?;
    let Some(mut result) = reconstructed else {
        return Ok(None);
    };
    if result.ordered_arguments != original.ordered_arguments {
        return Err(freeze("borrowed-instance/ordered-input"));
    }
    result.phase = BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(identity.clone());
    Ok(Some(result))
}

/// A prefix gap may leave a direct local argument unclassified even though
/// retained source flow proves that exact read is signed-i64. Recover only
/// this source-backed scalar case; expression results still need their
/// qualified expression frame.
fn original_unknown_scalar_actual_v1(
    ingress: &PreparedBorrowedFormalIngressV1,
    call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    identity: &InstanceSourceActualIdentityV1,
    candidate: &BorrowedCallActualCandidateV1,
) -> Option<BorrowedFormalActualSourceV1> {
    use crate::mir::resolved_value_profile::expression_plan::{
        ExpressionNodeSourceV1, ExpressionPlanRootV1,
    };
    let model::ArgumentExpressionV1::Source(plan) =
        identity.argument_sources.get(&candidate.ordinal)?
    else {
        return None;
    };
    let [node] = plan.nodes() else {
        return None;
    };
    let binding = match node.source() {
        ExpressionNodeSourceV1::Reference(
            crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding),
        ) if node.site() == &candidate.site
            && binding.owner() == call.call.owner()
            && matches!(plan.root(), ExpressionPlanRootV1::CallArgument {
                call: original, ordinal, ..
            } if original.site() == call.call.site() && *ordinal == candidate.ordinal) => *binding,
        _ => return None,
    };
    ingress
        .source_actual_value_sum_v1(
            &call.call,
            candidate.ordinal,
            &candidate.site,
            Some(binding),
        )?
        .signed_only()
        .then_some(BorrowedFormalActualSourceV1::Scalar {
            binding,
            kind: SourceScalarKind::Integer,
        })
}

/// Retain an exact local object binding read as a borrowed actual. This is
/// source attribution only: both the caller actual and callee formal must have
/// the same singleton object class, and the original argument plan must name
/// this exact local read at this exact call ordinal.
fn original_local_object_binding_actual_v1(
    ingress: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    identity: &InstanceSourceActualIdentityV1,
    candidate: &BorrowedCallActualCandidateV1,
) -> Option<BorrowedFormalActualSourceV1> {
    use crate::mir::normal_callable_semantic_package::SourceValueAlternativeV1 as Value;
    use crate::mir::resolved_value_profile::expression_plan::{
        ExpressionNodeSourceV1, ExpressionPlanRootV1,
    };

    let model::ArgumentExpressionV1::Source(plan) =
        identity.argument_sources.get(&candidate.ordinal)? else { return None; };
    let [node] = plan.nodes() else { return None; };
    let ExpressionNodeSourceV1::Reference(
        crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding),
    ) = node.source() else { return None; };
    let binding = *binding;
    if !matches!(&candidate.value, BorrowedCallActualValueV1::Binding(actual) if *actual == binding)
        && !matches!(candidate.value, BorrowedCallActualValueV1::Unknown) {
        return None;
    }
    if binding.owner() != call.call.owner() {
        return None;
    }
    let (_, expected_site, formal) = identity
        .incoming_arguments
        .iter()
        .find(|(ordinal, _, _)| *ordinal == candidate.ordinal)?;
    if expected_site != &candidate.site || formal.owner() != call.callee {
        return None;
    }
    let parameter = contracts
        .iter()
        .find(|row| row.owner == call.callee)?
        .parameters
        .get(candidate.ordinal as usize)?;
    if parameter.ordinal != candidate.ordinal
        || parameter.binding != *formal
        || !parameter.kind.is_ordinary_borrowed_handle()
    {
        return None;
    }
    if node.site() != &candidate.site
        || !matches!(
            plan.root(),
            ExpressionPlanRootV1::CallArgument {
                call: original,
                ordinal,
                ..
            } if original.site() == call.call.site() && *ordinal == candidate.ordinal
        )
    {
        return None;
    }
    let actual_sum = ingress.source_actual_value_sum_v1(
        &call.call,
        candidate.ordinal,
        &candidate.site,
        Some(binding),
    )?;
    let formal_sum = ingress.source_value_sum_v1(*formal)?;
    let [Value::Object(class)] = actual_sum.alternatives().iter().collect::<Vec<_>>().as_slice()
    else {
        return None;
    };
    if actual_sum.alternatives() != formal_sum.alternatives()
        || formal_sum.alternatives().len() != 1
    {
        return None;
    }
    Some(BorrowedFormalActualSourceV1::LocalObjectBinding {
        binding,
        class: class.clone(),
    })
}

pub(super) fn recheck_instance_source_actuals_v1(
    ingress: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    rows: &PreparedBorrowedCallActualsV1,
    identity: &InstanceSourceActualIdentityV1,
    array_ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<(), String> {
    let rebuilt =
        reconstruct_instance_source_actuals_v1(ingress, contracts, call, identity, array_ledger)?
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
    revalidate_passive_instance_incoming_with_array_context_v1(
        ingress, contracts, call, rows, classes, None,
    )
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
            recheck_instance_source_actuals_v1(
                ingress,
                contracts,
                call,
                rows,
                identity,
                array_ledger,
            )?;
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
    let mut rebuilt =
        prepare_instance_source_actuals_v1(ingress, contracts, &call.call, &identity.candidates)?
            .ok_or_else(|| freeze("borrowed-instance/source-unavailable"))?;
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
            let current = ingress
                .object_source_target_at_v1(&producer.call)?
                .ok_or_else(|| freeze("array-source/producer-missing"))?;
            targets.insert(producer.call.clone(), current);
        }
        loan.recheck(ingress, classes, &targets)?;
    }
    // Passive validation skips executable actuals, so it must independently
    // retain the original whole-incoming formal class agreement.
    for (ordinal, _, formal) in call.arguments.iter() {
        let view = ingress.source_object_view_v1(*formal)?;
        let candidate = identity
            .candidates
            .get(*ordinal as usize)
            .ok_or_else(|| freeze("borrowed-instance/actual-identity"))?;
        let local_object = original_local_object_binding_actual_v1(ingress, contracts, call, identity, candidate);
        let home_class = if let BorrowedCallActualValueV1::Home { binding, root, acquisition } = &candidate.value {
            match array_ledger {
                Some(ledger) => super::object_source::ObjectActualHomeSourceV1::Ledger(ledger)
                    .home_class(&call.call, *binding, *root, acquisition)?,
                None => None,
            }
        } else { None };
        let class = match &candidate.value {
            BorrowedCallActualValueV1::Home { .. } => home_class.as_deref(),
            BorrowedCallActualValueV1::Binding(_) | BorrowedCallActualValueV1::Unknown =>
                match &local_object {
                    Some(BorrowedFormalActualSourceV1::LocalObjectBinding { class, .. }) => Some(class.as_ref()),
                    _ => None,
                },
            BorrowedCallActualValueV1::ReceivedField {
                parent,
                acquisition,
            } => {
                let (_, loan) = identity
                    .field_loans
                    .iter()
                    .find(|(index, _)| index == ordinal)
                    .ok_or_else(|| freeze("borrowed-instance/field-loan-missing"))?;
                let current = ingress
                    .original_returned_field_actual_source_v1(&call.call, *ordinal)?
                    .ok_or_else(|| freeze("borrowed-instance/field-original-source-missing"))?;
                if current != loan || loan.parent() != *parent || loan.acquisition() != acquisition
                {
                    return Err(freeze("borrowed-instance/field-original-source-drift"));
                }
                if let Some(ledger) = array_ledger {
                    classes
                        .recheck_returned_field_graph_v1(
                            loan,
                            &ledger.field_write_claims,
                            &mut |site| {
                                ingress
                                    .object_source_target_at_v1(site)
                                    .map_err(|_| "returned-field/current-target-identity")
                            },
                        )
                        .map_err(freeze)?;
                }
                loan.class()
            }
            BorrowedCallActualValueV1::MethodCall(ref method) => {
                let parameter = contracts
                    .iter()
                    .find(|row| row.owner == call.callee)
                    .and_then(|row| row.parameters.get(*ordinal as usize))
                    .filter(|row| row.ordinal == *ordinal && row.binding == *formal)
                    .ok_or_else(|| freeze("borrowed-instance/actual-identity"))?;
                if static_source::ordinary_text_argument_source_v1(
                    ingress,
                    &OwnedExprSiteV1::new(call.call.owner(), candidate.site.clone()),
                    method,
                )?
                .is_some()
                    || static_source::core_argument_source_v1(
                        ingress,
                        &OwnedExprSiteV1::new(call.call.owner(), candidate.site.clone()),
                        method,
                        &parameter.kind,
                    )?
                    .is_some()
                {
                    if view.is_some() {
                        return Err(freeze("borrowed-entry/object-view-drift"));
                    }
                    continue;
                }
                let loan = identity
                    .array_loans
                    .iter()
                    .find(|(index, _)| index == ordinal)
                    .ok_or_else(|| freeze("borrowed-instance/array-loan-missing"))?;
                if candidate.ordinal != *ordinal || !loan.1.corroborates_method_source_v1(method) {
                    return Err(freeze("borrowed-instance/method-original-source-drift"));
                }
                let requirement = ingress
                    .array_element_source_v1(loan.1.read())?
                    .ok_or_else(|| freeze("array-source/read-missing"))?;
                Some(requirement.class())
            }
            BorrowedCallActualValueV1::SelfRooted { root, .. } => ingress
                .source_object_view_v1(*root)?
                .map(|view| view.class()),
            BorrowedCallActualValueV1::ReceivedNullable(binding)
            | BorrowedCallActualValueV1::ReceivedMixed(binding) => {
                if let Some(loan) = ingress.original_mixed_input_source_v1(call.source.require_instance()?)? {
                    classes.recheck_mixed_local_roots_v1(loan).map_err(freeze)?;
                    Some(loan.fresh_class())
                } else if matches!(candidate.value, BorrowedCallActualValueV1::ReceivedNullable(_)) {
                    match ingress.original_acquired_object_input_source_v1(*binding)? {
                        Some(loan) => {
                            if !loan.nullable() { return Err(freeze("nullable-input/original-source-drift")); }
                            classes.recheck_owning_object_local_input_roots_v1(loan).map_err(freeze)?;
                            classes.get(loan.key()).and_then(|class| class.class())
                        }
                        None => None,
                    }
                } else {
                    match ingress.mixed_received_actual_source_v1(*binding)? {
                        Some(loan) => {
                            classes.recheck_mixed_local_roots_v1(loan).map_err(freeze)?;
                            Some(loan.fresh_class())
                        }
                        None => None,
                    }
                }
            }
            BorrowedCallActualValueV1::Null => continue,
            _ => None,
        };
        if view.is_some_and(|view| class != Some(view.class()))
            || (matches!(
                candidate.value,
                BorrowedCallActualValueV1::MethodCall(_)
                    | BorrowedCallActualValueV1::ReceivedField { .. }
            ) && view.is_none())
        {
            return Err(format!("{}: call={:?}, formal={formal:?}, candidate={candidate:?}, expected={:?}, actual={class:?}",
                freeze("borrowed-entry/object-view-drift"), call.call, view.map(|view| view.class())));
        }
    }
    Ok(true)
}

/// A distinct nonexecuting response prevents the strict argument fallback.
/// Demand rechecks original source and the full ordered candidate snapshot.
pub(in crate::mir::normal_callable_semantic_package) fn project_pending_instance_source_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1,
) -> Result<Option<Box<[LocalCallArgumentV1]>>, String> {
    project_pending_instance_source_arguments_with_array_context_v1(
        source, pending, contracts, site, None,
    )
}

pub(in crate::mir::normal_callable_semantic_package) fn project_pending_instance_source_arguments_with_array_context_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1,
    array_ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<Option<Box<[LocalCallArgumentV1]>>, String> {
    if let Ok(ingress) = source {
        let mut originals = ingress
            .source_incoming
            .exact_rows()
            .filter(|row| &row.call == site);
        if let Some(original) = originals.next() {
            if let Some(target) = original.source.instance() {
                // Both original input phases lend observation only. The object
                // projection independently rechecks its unchanged full row;
                // qualified result permission stays with its dedicated consumer.
                if super::object_source::selected_original_object_phase_v1(
                    ingress,
                    target.call_site(),
                )? || (ingress.original_owning_object_input_callee_v1(target)?
                    && ingress.source_incoming.has_object_input_callee_v1(target))
                {
                    if originals.next().is_some() {
                        return Err(freeze("borrowed-instance/duplicate-source"));
                    }
                    return super::object_arguments::project_pending_object_target_arguments_v1(
                        source, contracts, pending, target,
                        array_ledger,
                    ).and_then(|support| match support {
                        crate::mir::resolved_semantics::home_new_prefix::ObjectCallSourceSupportV1::SourceOnly(arguments)
                        | crate::mir::resolved_semantics::home_new_prefix::ObjectCallSourceSupportV1::Observed(arguments) => Ok(Some(arguments)),
                        _ => Err(freeze("borrowed-instance/source-phase-drift")),
                    });
                }
            }
        }
    }
    let rows = match pending.get(site) {
        Some(Ok(rows)) => rows,
        Some(Err(issue)) => {
            if let Ok(ingress) = source {
                if selected_original_instance_forward_v1(ingress, site)? {
                    return Err(issue.clone());
                }
            }
            return Ok(None);
        }
        None => {
            if let Ok(ingress) = source {
                if selected_original_instance_forward_v1(ingress, site)? {
                    return Err(freeze("borrowed-instance/source-actuals-missing"));
                }
            }
            return Ok(None);
        }
    };
    if let BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(identity) = &rows.phase {
        let ingress = source.as_ref().map_err(Clone::clone)?;
        let original = crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1::original_instance_input_call_v1(ingress, contracts, site)?
            .ok_or_else(|| freeze("borrowed-instance/original-source-missing"))?;
        recheck_instance_source_actuals_v1(
            ingress,
            contracts,
            &original,
            rows,
            identity,
            array_ledger,
        )?;
        return Ok(Some(rows.ordered_arguments.clone()));
    }
    let BorrowedCallActualEvidencePhaseV1::SourceInstance(identity) = &rows.phase else {
        if let Ok(ingress) = source {
            if selected_original_instance_forward_v1(ingress, site)? {
                return Err(freeze("borrowed-instance/source-phase-drift"));
            }
        }
        return Ok(None);
    };
    let ingress = source.as_ref().map_err(Clone::clone)?;
    let mut rechecked =
        prepare_instance_source_actuals_v1(ingress, contracts, site, &identity.candidates)?
            .ok_or_else(|| freeze("borrowed-instance/source-phase-drift"))?;
    model::retain_instance_argument_sources_v1(&mut rechecked, identity)?;
    if rows != &rechecked || !rows.opaque_actuals.is_empty() {
        return Err(freeze("borrowed-instance/source-snapshot-drift"));
    }
    Ok(Some(rows.ordered_arguments.clone()))
}

/// Lend scalar read coordinates from the same rechecked original input row.
/// This counts source uses only; no scalar operand or executable actual is lent.
pub(in crate::mir::normal_callable_semantic_package) fn source_instance_scalar_reads_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    target: &LexicalInstanceCallSourceTargetV1,
    ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
) -> Result<Vec<(SourceExprSiteV1, BindingRefV1, SourceScalarKind)>, String> {
    let rows = pending
        .get(target.call_site())
        .ok_or_else(|| freeze("borrowed-instance/source-actuals-missing"))?
        .as_ref()
        .map_err(Clone::clone)?;
    let identity = match &rows.phase {
        BorrowedCallActualEvidencePhaseV1::SourceInstance(identity)
        | BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(identity) => identity,
        _ => return Ok(Vec::new()),
    };
    if &identity.source != target {
        return Err(freeze("borrowed-instance/source-identity"));
    }
    let Some(arguments) = project_pending_instance_source_arguments_with_array_context_v1(
        source,
        pending,
        contracts,
        target.call_site(),
        Some(ledger),
    )?
    else {
        return Ok(Vec::new());
    };
    let mut reads = Vec::new();
    for candidate in &identity.candidates {
        let BorrowedCallActualValueV1::Scalar(binding, kind) = candidate.value else {
            continue;
        };
        if target.argument_sites().get(candidate.ordinal as usize) != Some(&candidate.site)
            || arguments.get(candidate.ordinal as usize)
                != Some(&LocalCallArgumentV1::BorrowedActual {
                    ordinal: candidate.ordinal,
                    site: candidate.site.clone(),
                })
        {
            return Err(freeze("borrowed-instance/actual-identity"));
        }
        reads.push((candidate.site.clone(), binding, kind));
    }
    Ok(reads)
}
