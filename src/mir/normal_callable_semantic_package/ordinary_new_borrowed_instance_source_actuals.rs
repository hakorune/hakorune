//! Stable original Instance forwarding on the SAME pending actual row.
//! This phase never lends an Integer class, opaque payload, or executable call.
use super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1;
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct InstanceSourceActualIdentityV1 {
    pub(super) source: LexicalInstanceCallSourceTargetV1,
    pub(super) incoming_arguments: Box<[(u32, SourceExprSiteV1, BindingRefV1)]>,
    pub(super) candidates: Box<[BorrowedCallActualCandidateV1]>,
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
    if ingress.definitions.contains_key(&site.owner()) {
        return Ok(None);
    }
    let Some(draft) = ingress.source_only_definitions.get(&site.owner()) else {
        return Ok(None);
    };
    // Select the exact original opaque forward, never an entry receiver.
    if !actuals.iter().any(|actual| {
        matches!(actual.value, BorrowedCallActualValueV1::SelfRooted { binding, .. }
            if draft.origins.contains_key(&binding))
    }) {
        return Ok(None);
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
    // The existing Object source owner keeps its original qualification.
    if ingress.source_incoming.has_object_input_callee_v1(target) {
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
        || contract.mode != original.source.as_loan().declaration_mode()
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
    let BorrowedCallActualEvidencePhaseV1::SourceInstance(identity) = &rows.phase else {
        if let Ok(ingress) = source {
            if selected_original_instance_forward_v1(ingress, site)? {
                return Err(freeze("borrowed-instance/source-phase-drift"));
            }
        }
        return Ok(None);
    };
    let ingress = source.as_ref().map_err(Clone::clone)?;
    let rechecked =
        prepare_instance_source_actuals_v1(ingress, contracts, site, &identity.candidates)?
            .ok_or_else(|| freeze("borrowed-instance/source-phase-drift"))?;
    if rows != &rechecked || !rows.opaque_actuals.is_empty() {
        return Err(freeze("borrowed-instance/source-snapshot-drift"));
    }
    Ok(Some(rows.ordered_arguments.clone()))
}

/// Selection remains independent of final transport incoming membership.
/// This establishes only original Instance forward demand, never a payload.
fn selected_original_instance_forward_v1(
    ingress: &PreparedBorrowedFormalIngressV1,
    site: &OwnedExprSiteV1,
) -> Result<bool, String> {
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
