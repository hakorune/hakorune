//! Stable original Instance forwarding on the SAME pending actual row.
//! This phase never lends an Integer class, opaque payload, or executable call.
use super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1;
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct InstanceSourceActualIdentityV1 {
    pub(super) source: LexicalInstanceCallSourceTargetV1,
    pub(super) incoming_arguments: Box<[(u32, SourceExprSiteV1, BindingRefV1)]>,
    pub(super) candidates: Box<[BorrowedCallActualCandidateV1]>,
    pub(super) argument_sources: BTreeMap<u32, ArgumentExpressionV1>,
    pub(super) array_loans: Box<
        [(
            u32,
            super::super::borrowed_formal_source::ArrayElementSourceLoanV1,
        )],
    >,
    pub(super) field_loans: Box<[(u32, super::super::super::result_class_claim::ReturnedFieldSourceLoanV1)]>,
}

pub(super) fn prepare_instance_source_actuals_v1(
    ingress: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1,
    actuals: &[BorrowedCallActualCandidateV1],
) -> Result<Option<PreparedBorrowedCallActualsV1>, String> {
    // Retained Static observations (including errors) belong to that owner.
    if ingress
        .source_incoming
        .static_observations()
        .contains_key(site)
    {
        return Ok(None);
    }
    let mut array_loans = Vec::new();
    let mut field_loans = Vec::new();
    for (position, actual) in actuals.iter().enumerate() {
        if let BorrowedCallActualValueV1::ReceivedField { parent, acquisition } = &actual.value {
            let Some(loan) = ingress.original_returned_field_actual_source_v1(site, actual.ordinal)?
                else { continue; };
            if loan.read().owner() != site.owner() || loan.read().site() != &actual.site
                || loan.parent() != *parent || loan.acquisition() != acquisition {
                return Err(freeze("borrowed-instance/field-original-source-drift"));
            }
            field_loans.push((actual.ordinal, loan.clone()));
        }
        if let BorrowedCallActualValueV1::MethodCall(method) = &actual.value {
            if actual.ordinal as usize != position || method.owner() != site.owner() || method.site() != &actual.site {
                return Err(freeze("borrowed-instance/method-original-source-drift"));
            }
            let read = OwnedExprSiteV1::new(site.owner(), actual.site.clone());
            if static_source::ordinary_text_argument_source_v1(ingress, &read, method)?.is_some() { continue; }
            if let Some(loan) =
                super::super::borrowed_formal_source::ArrayElementSourceLoanV1::issue(
                    ingress, &read,
                )?
            {
                if !loan.corroborates_method_source_v1(method) {
                    return Err(freeze("borrowed-instance/method-original-source-drift"));
                }
                array_loans.push((actual.ordinal, loan));
            }
        }
    }
    // An affirmative original zero-input result request needs the SAME
    // argument-row protocol even though its caller has no borrowed formal.
    let nullable_request = match ingress.object_source_target_at_v1(site)? {
        Some(target) => ingress.original_owning_object_input_callee_v1(target)?,
        None => false,
    };
    let zero_request = !nullable_request && ingress.source_incoming.exact_rows().filter(|row| &row.call == site)
        .filter_map(|row| row.source.instance()).any(|target| matches!(&target.result_requirement,
            super::super::LexicalCallSourceResultRequirementV1::ScalarReturnSource(_)));
    let unit_request = ingress.source_incoming.exact_rows().filter(|row| &row.call == site)
        .filter_map(|row| row.source.instance()).any(|target| target.is_unit_discard_source_v1());
    let object_producer_request = match ingress.object_source_target_at_v1(site)? {
        Some(target) => ingress.original_object_producer_callee_v1(target)?,
        None => false,
    };
    let mixed_request = match ingress.object_source_target_at_v1(site)? {
        Some(target) => ingress.original_mixed_input_source_v1(target)?,
        None => None,
    };
    let draft = ingress.source_definition_for(site.owner());
    let expression_request = ingress.original_signed_expression_argument_request_v1(site);
    if array_loans.is_empty() && field_loans.is_empty() && !zero_request && !unit_request && !object_producer_request && mixed_request.is_none() && !nullable_request && !expression_request {
        if ingress.definitions.contains_key(&site.owner()) {
            return Ok(None);
        }
        let Some(draft) = ingress.source_only_definitions.get(&site.owner()) else {
            return Ok(None);
        };
        // Existing opaque forward selection stays unchanged outside Array demand.
        if !actuals.iter().any(|actual| matches!(actual.value,
            BorrowedCallActualValueV1::SelfRooted { binding, .. } if draft.origins.contains_key(&binding))) {
            return Ok(None);
        }
    }
    let mut originals = ingress
        .source_incoming
        .exact_rows()
        .filter(|row| &row.call == site);
    // A caller draft can mention an Instance argument outside this borrowed
    // incoming profile. Only a retained original row selects this source law.
    let Some(original) = originals.next() else {
        return Ok(None);
    };
    if originals.next().is_some() {
        return Err(freeze("borrowed-instance/duplicate-source"));
    }
    let Some(target) = original.source.instance() else {
        return Ok(None);
    };
    if unit_request {
        let super::super::LexicalCallSourceResultRequirementV1::UnitDiscardSource(request) = &target.result_requirement
            else { return Err(freeze("borrowed-instance/unit-source-identity")) };
        if !request.is_unit_candidate_v1() || !request.matches(&super::super::source::CallTargetReferenceV1::from_target(target)) {
            return Err(freeze("borrowed-instance/unit-source-identity"));
        }
    }
    if zero_request {
        let super::super::LexicalCallSourceResultRequirementV1::ScalarReturnSource(request) = &target.result_requirement else {
            return Err(freeze("borrowed-instance/zero-source-identity"));
        };
        if !request.matches(&super::super::source::CallTargetReferenceV1::from_target(target))
            || !actuals.is_empty() || !original.arguments.is_empty()
        { return Err(freeze("borrowed-instance/zero-source-identity")); }
    }
    // The existing Object source owner keeps its original qualification.
    if ingress.source_incoming.has_object_input_callee_v1(target) {
        if expression_request {
            return object_source::prepare_object_source_actuals_v1(ingress, contracts, site, actuals);
        }
        return Ok(None);
    }
    let mut matching = contracts.iter().filter(|row| row.owner == original.callee);
    let contract = matching
        .next()
        .ok_or_else(|| freeze("borrowed-instance/source-contract"))?;
    if matching.next().is_some()
        || target.call_site() != site
        || target.callee_owner() != original.callee
        || target.target_batch_slot() != contract.batch_slot
        || contract.mode != original.source.as_loan()?.declaration_mode()
        || target.argument_sites().len() != target.target().arity() as usize
        || contract.parameters.len() != actuals.len()
        || actuals.len() != target.argument_sites().len()
    {
        return Err(freeze("borrowed-instance/source-identity"));
    }
    let opaque: Vec<_> = contract
        .parameters
        .iter()
        .filter(|formal| formal.kind.is_ordinary_borrowed_handle())
        .collect();
    if opaque.len() != original.arguments.len()
        || opaque
            .iter()
            .zip(&original.arguments)
            .any(|(formal, (ordinal, argument, binding))| {
                formal.ordinal != *ordinal
                    || formal.binding != *binding
                    || binding.owner() != original.callee
                    || target.argument_sites().get(*ordinal as usize) != Some(argument)
            })
    {
        return Err(freeze("borrowed-instance/opaque-identity"));
    }
    let mut ordered = Vec::new();
    for (index, (formal, actual)) in contract.parameters.iter().zip(actuals).enumerate() {
        if formal.ordinal as usize != index
            || actual.ordinal != formal.ordinal
            || formal.binding.owner() != original.callee
            || target.argument_sites().get(index) != Some(&actual.site)
        {
            return Err(freeze("borrowed-instance/actual-identity"));
        }
        let argument = if formal.kind.is_ordinary_borrowed_handle() {
            match actual.value {
                BorrowedCallActualValueV1::SelfRooted { binding, root } => {
                    let draft =
                        draft.ok_or_else(|| freeze("borrowed-instance/forward-source-missing"))?;
                    let mut uses = draft
                        .uses
                        .iter()
                        .filter(|row| row.site.site() == &actual.site);
                    let row = uses
                        .next()
                        .ok_or_else(|| freeze("borrowed-instance/forward-source-missing"))?;
                    if uses.next().is_some()
                        || binding.owner() != site.owner()
                        || root.owner() != site.owner()
                        || row.site.owner() != site.owner()
                        || row.binding != binding
                        || row.formal != root
                        || draft.origins.get(&binding) != Some(&root)
                        || !matches!(&row.kind, BorrowedFormalUseDraftKindV1::UnresolvedArgument { call, ordinal }
                            if call == site && *ordinal == actual.ordinal)
                    {
                        return Err(freeze("borrowed-instance/forward-source-identity"));
                    }
                }
                BorrowedCallActualValueV1::ReceivedNullable(binding)
                | BorrowedCallActualValueV1::ReceivedMixed(binding) => {
                    let loan = mixed_request.ok_or_else(|| freeze("borrowed-instance/mixed-request-missing"))?;
                    if loan.callee_ordinal() != actual.ordinal || loan.support() != binding
                        || binding.owner() != site.owner()
                    { return Err(freeze("borrowed-instance/mixed-support-identity")); }
                    let matches = match actual.value {
                        BorrowedCallActualValueV1::ReceivedNullable(_) =>
                            loan.corroborates_acquired_nullable_actual_v1(binding),
                        BorrowedCallActualValueV1::ReceivedMixed(_) => ingress.mixed_received_actual_source_v1(binding)?
                            .is_some_and(|parent| loan.corroborates_acquired_mixed_actual_v1(binding, parent)),
                        _ => unreachable!("received alternatives selected above"),
                    };
                    if !matches { return Err(freeze("borrowed-instance/mixed-acquisition-identity")); }
                }
                BorrowedCallActualValueV1::MethodCall(ref method)
                    if static_source::ordinary_text_argument_source_v1(ingress,
                        &OwnedExprSiteV1::new(site.owner(), actual.site.clone()), method)?.is_some() => {
                    if !matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle) {
                        return Err(freeze("borrowed-instance/text-formal-domain"));
                    }
                }
                BorrowedCallActualValueV1::MethodCall(ref method) => {
                    let mut loans = array_loans
                        .iter()
                        .filter(|(ordinal, _)| *ordinal == actual.ordinal);
                    let (_, loan) = loans
                        .next()
                        .ok_or_else(|| freeze("borrowed-instance/array-loan-missing"))?;
                    if loans.next().is_some()
                        || loan.read().owner() != site.owner()
                        || loan.read().site() != &actual.site
                        || loan.holder().owner() != site.owner()
                        || !loan.corroborates_method_source_v1(method)
                    {
                        return Err(freeze("borrowed-instance/array-loan-identity"));
                    }
                }
                BorrowedCallActualValueV1::ReceivedField { parent, ref acquisition } => {
                    let mut loans = field_loans.iter().filter(|(ordinal, _)| *ordinal == actual.ordinal);
                    let (_, loan) = loans.next().ok_or_else(|| freeze("borrowed-instance/field-loan-missing"))?;
                    if loans.next().is_some() || loan.read().owner() != site.owner()
                        || loan.read().site() != &actual.site || loan.parent() != parent
                        || loan.acquisition() != acquisition {
                        return Err(freeze("borrowed-instance/field-loan-identity"));
                    }
                }
                BorrowedCallActualValueV1::Expression
                    if matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
                        && ingress.source_value_sum_v1(formal.binding).is_some_and(|sum| sum.signed_only()) => {}
                BorrowedCallActualValueV1::Integer(_)
                | BorrowedCallActualValueV1::Bool(_)
                | BorrowedCallActualValueV1::Null => {}
                _ => return Err(freeze("borrowed-instance/opaque-source-unsupported")),
            }
            LocalCallArgumentV1::BorrowedActual {
                ordinal: actual.ordinal,
                site: actual.site.clone(),
            }
        } else {
            match (&formal.kind, &actual.value) {
                (
                    CallableParameterContractKindV1::ExactTrivial(abi),
                    BorrowedCallActualValueV1::Integer(value),
                ) if abi.is_i64() => LocalCallArgumentV1::Integer(*value),
                (
                    CallableParameterContractKindV1::ExactTrivial(abi),
                    BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer),
                ) if abi.is_i64() && binding.owner() == site.owner() => {
                    LocalCallArgumentV1::Scalar(*binding)
                }
                _ => return Err(freeze("borrowed-instance/nonopaque-source-unsupported")),
            }
        };
        ordered.push(argument);
    }
    Ok(Some(PreparedBorrowedCallActualsV1 {
        phase: BorrowedCallActualEvidencePhaseV1::SourceInstance(InstanceSourceActualIdentityV1 {
            source: target.clone(),
            incoming_arguments: original.arguments.clone(),
            candidates: actuals.to_vec().into_boxed_slice(),
            argument_sources: BTreeMap::new(),
            array_loans: array_loans.into_boxed_slice(),
            field_loans: field_loans.into_boxed_slice(),
        }),
        opaque_actuals: Box::new([]),
        ordered_arguments: ordered.into_boxed_slice(),
    }))
}

/// A distinct nonexecuting response prevents the strict argument fallback.
/// Demand rechecks original source and the full ordered candidate snapshot.
pub(in crate::mir::normal_callable_semantic_package) fn project_pending_instance_source_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1,
) -> Result<Option<Box<[LocalCallArgumentV1]>>, String> {
    project_pending_instance_source_arguments_with_array_context_v1(source, pending, contracts, site, None)
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
                if super::object_source::selected_original_object_phase_v1(ingress, target.call_site())?
                    || (ingress.original_owning_object_input_callee_v1(target)?
                        && ingress.source_incoming.has_object_input_callee_v1(target)) {
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
        let original = ingress
            .source_incoming
            .exact_rows()
            .find(|row| &row.call == site)
            .ok_or_else(|| freeze("borrowed-instance/original-source-missing"))?;
        recheck_instance_source_actuals_v1(ingress, contracts, original, rows, identity, array_ledger)?;
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

/// Selection remains independent of final transport incoming membership.
/// This establishes only original Instance forward demand, never a payload.
pub(super) fn selected_original_instance_forward_v1(
    ingress: &PreparedBorrowedFormalIngressV1,
    site: &OwnedExprSiteV1,
) -> Result<bool, String> {
    let mut originals = ingress
        .source_incoming
        .exact_rows()
        .filter(|row| &row.call == site);
    if let Some(original) = originals.next() {
        if originals.next().is_some() {
            return Err(freeze("borrowed-instance/duplicate-source"));
        }
        if let Some(target) = original.source.instance() {
            if ingress.original_signed_expression_argument_request_v1(site) { return Ok(true); }
            if ingress.returned_field_argument_selected_v1(site) {
                for (ordinal, _, _) in original.arguments.iter() {
                    if ingress.original_returned_field_actual_source_v1(site, *ordinal)?.is_some() { return Ok(true); }
                }
                return Err(freeze("borrowed-instance/field-original-source-missing"));
            }
            if ingress.original_mixed_input_source_v1(target)?.is_some() { return Ok(true); }
            if ingress.original_owning_object_input_callee_v1(target)?
                && !ingress.source_incoming.has_object_input_callee_v1(target) { return Ok(true); }
            if !ingress.source_incoming.has_object_input_callee_v1(target)
                && ingress.original_object_producer_callee_v1(target)? { return Ok(true); }
            if target.is_unit_discard_source_v1() { return Ok(true); }
            if let super::super::LexicalCallSourceResultRequirementV1::ScalarReturnSource(request) = &target.result_requirement {
                if !request.matches(&super::super::source::CallTargetReferenceV1::from_target(target)) {
                    return Err(freeze("borrowed-instance/zero-source-identity"));
                }
                return Ok(true);
            }
        }
        if original
            .source
            .instance()
            .is_some_and(|target| !ingress.source_incoming.has_object_input_callee_v1(target))
        {
            for (_, argument, _) in original.arguments.iter() {
                if ingress
                    .array_element_source_v1(&OwnedExprSiteV1::new(site.owner(), argument.clone()))?
                    .is_some()
                {
                    return Ok(true);
                }
            }
        }
    }
    if ingress.definitions.contains_key(&site.owner())
        || ingress
            .source_incoming
            .static_observations()
            .contains_key(site)
    {
        return Ok(false);
    }
    let Some(draft) = ingress.source_only_definitions.get(&site.owner()) else {
        return Ok(false);
    };
    if !draft.uses.iter().any(|row| {
        row.site.owner() == site.owner()
            && draft.origins.get(&row.binding) == Some(&row.formal)
            && matches!(&row.kind, BorrowedFormalUseDraftKindV1::UnresolvedArgument { call, .. }
                if call == site)
    }) {
        return Ok(false);
    }
    let mut originals = ingress
        .source_incoming
        .exact_rows()
        .filter(|row| &row.call == site);
    let Some(original) = originals.next() else {
        return Ok(false);
    };
    if originals.next().is_some() {
        return Err(freeze("borrowed-instance/duplicate-source"));
    }
    Ok(original
        .source
        .instance()
        .is_some_and(|target| !ingress.source_incoming.has_object_input_callee_v1(target)))
}

/// Original declared root/alias constraint, never a caller entry permit.
pub(super) fn declared_source_actual_class_v1(
    ingress: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &OwnedExprSiteV1,
    binding: BindingRefV1,
    root: BindingRefV1,
) -> Result<Option<Box<str>>, String> {
    let mut matching = contracts.iter().filter(|row| row.owner == call.owner());
    let Some(contract) = matching.next() else {
        return Ok(None);
    };
    if matching.next().is_some() {
        return Err(freeze("borrowed-actual/declared-caller-duplicate"));
    }
    let mut matching = contract
        .parameters
        .iter()
        .filter(|formal| formal.binding == root);
    let Some(formal) = matching.next() else {
        return Ok(None);
    };
    if matching.next().is_some()
        || binding.owner() != call.owner()
        || root.owner() != call.owner()
        || contract
            .parameters
            .get(formal.ordinal as usize)
            .map(|row| row.binding)
            != Some(root)
    {
        return Err(freeze("borrowed-actual/declared-root-identity"));
    }
    let CallableParameterContractKindV1::DeclaredObject(class) = &formal.kind else {
        return Ok(None);
    };
    if let Some(draft) = ingress.source_definition_for(call.owner()) {
        if draft.origins.get(&binding) != Some(&root) {
            return Err(freeze("borrowed-actual/declared-alias-identity"));
        }
        let view = ingress
            .source_object_view_v1(root)?
            .ok_or_else(|| freeze("borrowed-actual/declared-source-domain"))?;
        if !view.is_declared() || view.class() != class.as_ref() {
            return Err(freeze("borrowed-actual/declared-source-class"));
        }
    } else if binding != root {
        return Err(freeze("borrowed-actual/declared-alias-unavailable"));
    }
    Ok(Some(class.clone()))
}

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
            if matches!(candidate.value, BorrowedCallActualValueV1::Expression) {
                return array_ledger.map(|ledger| ledger.borrowed_expression_actual_source_v1(&call.call, ordinal, site)
                    .map(|actual| Some(actual.source))).unwrap_or(Ok(None));
            }
            if let BorrowedCallActualValueV1::MethodCall(method) = &candidate.value {
                if static_source::ordinary_text_argument_source_v1(ingress,
                    &OwnedExprSiteV1::new(call.call.owner(), site.clone()), method)?.is_some() {
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
    let mut originals = ingress.source_incoming.exact_rows()
        .filter(|row| row.call == call.call);
    let original = originals.next()
        .ok_or_else(|| freeze("borrowed-instance/source-unavailable"))?;
    if originals.next().is_some() || original.callee != call.callee
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
                if static_source::ordinary_text_argument_source_v1(ingress,
                    &OwnedExprSiteV1::new(call.call.owner(), candidate.site.clone()), method)?.is_some() {
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

#[cfg(test)]
#[path = "ordinary_new_borrowed_instance_source_actual_tests.rs"]
mod tests;
