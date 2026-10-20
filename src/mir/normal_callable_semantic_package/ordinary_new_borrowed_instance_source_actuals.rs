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

/// Original computed Core actuals select preparation, the callee Home walk
/// and whole-input reconstruction together. This grants no executable frame.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn original_core_argument_input_request_v1(
    ingress: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1,
) -> Result<bool, String> {
    let mut originals = ingress.source_incoming.exact_rows().filter(|row| &row.call == site);
    let Some(original) = originals.next() else { return Ok(false) };
    if originals.next().is_some() { return Err(freeze("borrowed-instance/duplicate-source")); }
    let Some(target) = original.source.instance() else { return Ok(false) };
    let mut matching = contracts.iter().filter(|row| row.owner == original.callee);
    let contract = matching.next().ok_or_else(|| freeze("borrowed-instance/source-contract"))?;
    if matching.next().is_some() || target.call_site() != site
        || target.callee_owner() != original.callee || target.target_batch_slot() != contract.batch_slot
        || contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
        || target.argument_sites().len() != contract.parameters.len()
        || target.target().arity() as usize != contract.parameters.len()
    { return Err(freeze("borrowed-instance/source-identity")); }
    let mut requested = false;
    for (ordinal, (argument, formal)) in target.argument_sites().iter().zip(&contract.parameters).enumerate() {
        if formal.ordinal as usize != ordinal || formal.binding.owner() != original.callee {
            return Err(freeze("borrowed-instance/actual-identity"));
        }
        let read = OwnedExprSiteV1::new(site.owner(), argument.clone());
        if let Some(core) = ingress.ordinary_core_source_v1(&read) {
            requested |= static_source::core_argument_source_v1(
                ingress, &read, core.source().source(), &formal.kind)?.is_some();
        }
    }
    Ok(requested)
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
    let core_argument = original_core_argument_input_request_v1(ingress, contracts, site)?;
    let original_call = crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1::original_instance_input_call_v1(ingress, contracts, site)?;
    let finite_input = match original_call.as_ref() {
        Some(call) => crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1::original_finite_text_instance_input_request_v1(
            ingress, contracts, call.callee)?,
        None => false,
    };
    // The SAME LOCAL result's original key purpose selects source input
    // independently of callee domain/Completion success. It grants no Text.
    let map_key = original_call.as_ref().and_then(|call| call.source.instance())
        .and_then(|target| target.borrowed_nullable_local_source())
        .and_then(|loan| loan.map_key_actual_v1());
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
            if let Some(core) = ingress.ordinary_core_source_v1(&read) {
                if !std::sync::Arc::ptr_eq(method, core.source().source()) {
                    return Err(freeze("borrowed-instance/method-original-source-drift"));
                }
                continue;
            }
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
        .filter_map(|row| row.source.instance()).any(|target| target.uses_existing_borrowed_result_v1()
            && target.target().arity() == 0 && target.argument_sites().is_empty());
    let unit_request = ingress.source_incoming.exact_rows().filter(|row| &row.call == site)
        .filter_map(|row| row.source.instance()).any(|target| target.is_unit_discard_source_v1());
    let scalar_request = ingress.source_incoming.exact_rows().filter(|row| &row.call == site)
        .filter_map(|row| row.source.instance()).any(|target| matches!(&target.result_requirement,
            super::super::LexicalCallSourceResultRequirementV1::ScalarReturnSource(request) if request.is_literal_scalar_v1()));
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
    if !finite_input && !core_argument && array_loans.is_empty() && field_loans.is_empty() && !zero_request && !unit_request && !scalar_request && !object_producer_request && mixed_request.is_none() && !nullable_request && !expression_request && map_key.is_none() {
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
    let Some(original) = original_call.as_ref() else {
        return Ok(None);
    };
    let Some(target) = original.source.instance() else {
        return Ok(None);
    };
    if scalar_request {
        let super::super::LexicalCallSourceResultRequirementV1::ScalarReturnSource(request) = &target.result_requirement
            else { return Err(freeze("borrowed-instance/source-identity")) };
        if !request.is_literal_scalar_v1() || !request.matches(&super::super::source::CallTargetReferenceV1::from_target(target)) {
            return Err(freeze("borrowed-instance/source-identity"));
        }
    }
    if unit_request {
        let super::super::LexicalCallSourceResultRequirementV1::UnitDiscardSource(request) = &target.result_requirement
            else { return Err(freeze("borrowed-instance/unit-source-identity")) };
        if !request.is_unit_candidate_v1() || !request.matches(&super::super::source::CallTargetReferenceV1::from_target(target)) {
            return Err(freeze("borrowed-instance/unit-source-identity"));
        }
    }
    if zero_request {
        if let super::super::LexicalCallSourceResultRequirementV1::ScalarReturnSource(request) = &target.result_requirement {
            if !request.matches(&super::super::source::CallTargetReferenceV1::from_target(target)) {
                return Err(freeze("borrowed-instance/zero-source-identity"));
            }
        }
        if !actuals.is_empty() || !original.arguments.is_empty() {
            return Err(freeze("borrowed-instance/zero-source-identity"));
        }
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
                BorrowedCallActualValueV1::MethodCall(ref method)
                    if static_source::core_argument_source_v1(ingress,
                        &OwnedExprSiteV1::new(site.owner(), actual.site.clone()), method, &formal.kind)?.is_some() => {}
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
                BorrowedCallActualValueV1::Binding(binding)
                    if matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
                        && map_key.is_some_and(|(ordinal, read, original)| ordinal == actual.ordinal
                            && read.owner() == site.owner() && read.site() == &actual.site
                            && original == binding) => {},
                BorrowedCallActualValueV1::Expression
                    if matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
                        && ingress.source_value_sum_v1(formal.binding).is_some_and(|sum| sum.signed_only()) => {}
                BorrowedCallActualValueV1::String(_) if finite_input && matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle) => {},
                BorrowedCallActualValueV1::Integer(_)
                | BorrowedCallActualValueV1::Bool(_)
                | BorrowedCallActualValueV1::Null => {}
                _ => return Err(format!("{}: site={site:?} ordinal={} actual={:?}",
                    freeze("borrowed-instance/opaque-source-unsupported"), actual.ordinal, actual.value)),
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
        let original = crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1::original_instance_input_call_v1(ingress, contracts, site)?
            .ok_or_else(|| freeze("borrowed-instance/original-source-missing"))?;
        recheck_instance_source_actuals_v1(ingress, contracts, &original, rows, identity, array_ledger)?;
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
            if target.borrowed_nullable_local_source().is_some_and(|loan| loan.map_key_actual_v1().is_some()) { return Ok(true); }
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
            if target.uses_existing_borrowed_result_v1() && target.target().arity() == 0
                && target.argument_sites().is_empty() { return Ok(true); }
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

pub(super) use super::instance_validation::{reconstruct_instance_source_actuals_v1, recheck_instance_source_actuals_v1};
pub(in crate::mir::normal_callable_semantic_package) use super::instance_validation::{
    recheck_pending_array_source_actuals_v1, revalidate_passive_instance_incoming_v1,
    revalidate_passive_instance_incoming_with_array_context_v1,
};

#[cfg(test)]
#[path = "ordinary_new_borrowed_instance_source_actual_tests.rs"]
mod tests;
