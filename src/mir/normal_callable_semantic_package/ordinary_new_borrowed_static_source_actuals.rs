//! Static source and executable projections share the original incoming inventory.
//! SourceStatic itself owns no opaque actual and never activates an entry or ABI.
use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use crate::mir::resolved_semantics::home_new_prefix::QualifiedStaticCallClaimV1;
use crate::mir::resolved_semantics::home_new_prefix::StaticI64CallClaimV1;
use crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(super) struct StaticSourceActualIdentityV1 {
    pub(super) source: Rc<StaticIncomingSourceV1>,
    pub(super) candidates: Box<[BorrowedCallActualCandidateV1]>,
    pub(super) integer_evidence: Box<[bool]>,
}

impl PartialEq for StaticSourceActualIdentityV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source)
            && self.candidates == other.candidates
            && self.integer_evidence == other.integer_evidence
    }
}
impl Eq for StaticSourceActualIdentityV1 {}

pub(super) fn prepare_static_source_actuals_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &OwnedExprSiteV1,
    actuals: &[BorrowedCallActualCandidateV1],
) -> Result<Option<PreparedBorrowedCallActualsV1>, String> {
    // A zero-input CurrentOwner call retains the SAME original source without
    // inventing an opaque-argument fact. It is still SourceStatic, not Ready.
    let opaque_source = prepared
        .static_arguments
        .keys()
        .any(|(site, _)| site == call);
    let original_source = prepared
        .source_incoming
        .static_observations()
        .get(call)
        .and_then(|row| row.as_ref().ok())
        .is_some_and(|source| {
            source.is_zeroarg_i64_v1() || source.is_current_owner_i64_source_v1()
        });
    if !opaque_source && !original_source {
        return Ok(None);
    }
    let source = prepared
        .source_incoming
        .static_observations()
        .get(call)
        .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?
        .as_ref()
        .map_err(Clone::clone)?;
    let mut matching = contracts
        .iter()
        .filter(|row| row.owner == source.callee_owner());
    let contract = matching
        .next()
        .ok_or_else(|| freeze("borrowed-static/target-contract"))?;
    if matching.next().is_some()
        || source.call_site() != call
        || contract.batch_slot != source.target_batch_slot()
        || contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod
        || contract.parameters.len() != source.parameters().len()
        || actuals.len() != source.argument_sites().len()
        || actuals.len() != contract.parameters.len()
    {
        return Err(freeze("borrowed-static/source-actual-identity"));
    }
    let mut arguments = Vec::with_capacity(actuals.len());
    let mut integer_evidence = Vec::with_capacity(actuals.len());
    for (index, ((formal, original), actual)) in contract
        .parameters
        .iter()
        .zip(source.parameters())
        .zip(actuals)
        .enumerate()
    {
        if formal.ordinal as usize != index
            || formal.ordinal != original.ordinal
            || formal.binding != original.binding
            || formal.kind != original.kind
            || formal.binding.owner() != contract.owner
            || actual.ordinal != formal.ordinal
            || source.argument_sites().get(index) != Some(&actual.site)
        {
            return Err(freeze("borrowed-static/source-actual-identity"));
        }
        if let Some(fact) = prepared
            .static_arguments
            .get(&(call.clone(), actual.ordinal))
        {
            let same_binding = match &actual.value {
                BorrowedCallActualValueV1::SelfRooted { binding, root } => {
                    *binding == fact.binding() && *root == fact.formal()
                }
                BorrowedCallActualValueV1::Scalar(binding, _) => *binding == fact.binding(),
                _ => false,
            };
            if !Rc::ptr_eq(fact.retained_call_source(), source)
                || fact.call() != call
                || fact.ordinal() != actual.ordinal
                || fact.use_site().site() != &actual.site
                || fact.target_formal() != formal.binding
                || !same_binding
            {
                return Err(freeze("borrowed-static/forward-source-identity"));
            }
        }
        let integer = match &actual.value {
            BorrowedCallActualValueV1::Integer(_) => true,
            BorrowedCallActualValueV1::Bool(_) | BorrowedCallActualValueV1::Null => false,
            BorrowedCallActualValueV1::Scalar(binding, kind) if binding.owner() == call.owner() => {
                *kind == SourceScalarKind::Integer
            }
            BorrowedCallActualValueV1::SelfRooted { root, .. } => {
                if !prepared
                    .static_arguments
                    .contains_key(&(call.clone(), actual.ordinal))
                {
                    return Err(freeze("borrowed-static/forward-source-missing"));
                }
                prepared.candidate_integer_agreement(*root)
            }
            _ => return Err(freeze("borrowed-static/source-actual-unavailable")),
        };
        let argument = match &formal.kind {
            kind if kind.is_ordinary_borrowed_handle() => LocalCallArgumentV1::BorrowedActual {
                ordinal: actual.ordinal,
                site: actual.site.clone(),
            },
            CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64() => {
                match actual.value {
                    BorrowedCallActualValueV1::Integer(value) => {
                        LocalCallArgumentV1::Integer(value)
                    }
                    BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer)
                        if binding.owner() == call.owner() =>
                    {
                        LocalCallArgumentV1::Scalar(binding)
                    }
                    _ if source.is_current_owner_i64_source_v1()
                        && !matches!(
                            &actual.value,
                            BorrowedCallActualValueV1::Bool(_)
                                | BorrowedCallActualValueV1::Null
                                | BorrowedCallActualValueV1::Scalar(_, SourceScalarKind::Bool)
                        ) =>
                    {
                        // No I64 source proof yet. Keep the original incoming
                        // inventory passive without staging a selected call.
                        return Ok(None);
                    }
                    _ => return Err(freeze("borrowed-static/nonopaque-integer-unproved")),
                }
            }
            _ => return Err(freeze("borrowed-static/input-contract-unsupported")),
        };
        arguments.push(argument);
        integer_evidence.push(integer);
    }
    Ok(Some(PreparedBorrowedCallActualsV1 {
        opaque_actuals: Box::new([]),
        ordered_arguments: arguments.into_boxed_slice(),
        phase: BorrowedCallActualEvidencePhaseV1::SourceStatic(StaticSourceActualIdentityV1 {
            source: Rc::clone(source),
            candidates: actuals.to_vec().into_boxed_slice(),
            integer_evidence: integer_evidence.into_boxed_slice(),
        }),
    }))
}

pub(in crate::mir::normal_callable_semantic_package) fn project_pending_static_source_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    claim: &QualifiedStaticCallClaimV1,
) -> Result<Option<Box<[LocalCallArgumentV1]>>, String> {
    project_pending_static_source_arguments_for_route_v1(
        source,
        pending,
        site,
        claim.required_i64_arguments(),
        false,
    )
}

pub(in crate::mir::normal_callable_semantic_package) fn project_pending_current_owner_static_source_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    claim: &StaticI64CallClaimV1,
) -> Result<Option<Box<[LocalCallArgumentV1]>>, String> {
    let required = claim
        .current_owner_source_required_i64_arguments()
        .ok_or_else(|| freeze("borrowed-static/current-owner-claim-required"))?;
    let prepared = source.as_ref().map_err(Clone::clone)?;
    let original = prepared
        .source_incoming
        .static_observations()
        .get(site)
        .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?
        .as_ref()
        .map_err(Clone::clone)?;
    if !claim.corroborates_source(
        site,
        ResolvedMethodCallReceiverSourceV1::CurrentOwner,
        original.argument_sites().len() as u32,
    ) {
        return Err(freeze("borrowed-static/current-owner-claim-identity"));
    }
    if !pending.contains_key(site) {
        return Ok(None);
    }
    project_pending_static_source_arguments_for_route_v1(source, pending, site, required, true)
}

fn project_pending_static_source_arguments_for_route_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    required_i64_arguments: &[u32],
    current_owner: bool,
) -> Result<Option<Box<[LocalCallArgumentV1]>>, String> {
    let prepared = source.as_ref().map_err(Clone::clone)?;
    let has_source_fact = prepared
        .static_arguments
        .keys()
        .any(|(call, _)| call == site);
    let has_final_static = prepared.incoming.iter().any(|row| {
        &row.call == site
            && matches!(
                row.source,
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(ref original) if original.is_qualified()
            )
    });
    let retained = prepared
        .source_incoming
        .static_observations()
        .get(site)
        .and_then(|row| row.as_ref().ok());
    let has_current_owner_source = retained.is_some_and(|row| row.is_current_owner_i64_source_v1());
    if !has_source_fact && !has_final_static && !(current_owner && has_current_owner_source) {
        return Ok(None);
    }
    if retained.is_none_or(|row| {
        if current_owner {
            !row.is_current_owner_i64_source_v1()
        } else {
            !row.is_qualified()
        }
    }) {
        return Err(freeze("borrowed-static/source-route-identity"));
    }
    let rows = pending
        .get(site)
        .ok_or_else(|| freeze("borrowed-static/source-unobserved"))?
        .as_ref()
        .map_err(Clone::clone)?;
    if matches!(rows.phase, BorrowedCallActualEvidencePhaseV1::Executable) {
        if current_owner {
            return Err(freeze(
                "borrowed-static/current-owner-source-phase-required",
            ));
        }
        let mut incoming = prepared.incoming.iter().filter(|row| &row.call == site);
        let call = incoming
            .next()
            .ok_or_else(|| freeze("borrowed-static/executable-incoming-missing"))?;
        let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
            &call.source
        else {
            return Err(freeze("borrowed-static/executable-source-kind"));
        };
        original.require_qualified()?;
        let retained = prepared
            .source_incoming
            .static_observations()
            .get(site)
            .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        if incoming.next().is_some()
            || !Rc::ptr_eq(original, retained)
            || original.call_site() != site
            || required_i64_arguments != original.required_i64_arguments()
        {
            return Err(freeze("borrowed-static/executable-source-identity"));
        }
        let arguments = rows.ordered_arguments_for_v1(call)?;
        for ordinal in required_i64_arguments {
            let integer = match arguments.get(*ordinal as usize) {
                Some(LocalCallArgumentV1::Integer(_) | LocalCallArgumentV1::Scalar(_)) => true,
                Some(LocalCallArgumentV1::BorrowedActual { ordinal, .. }) => rows
                    .opaque_actuals
                    .iter()
                    .find(|actual| actual.ordinal == *ordinal)
                    .is_some_and(|actual| match &actual.source {
                        BorrowedFormalActualSourceV1::Integer(_) => true,
                        BorrowedFormalActualSourceV1::Scalar {
                            kind: SourceScalarKind::Integer,
                            ..
                        } => true,
                        BorrowedFormalActualSourceV1::Forwarded { formal, .. } => {
                            prepared.candidate_integer_agreement(*formal)
                        }
                        _ => false,
                    }),
                _ => false,
            };
            if !integer {
                return Err(freeze("borrowed-static/required-integer-source-unproved"));
            }
        }
        return Ok(Some(arguments.into()));
    }
    if !has_source_fact && !current_owner {
        return Err(freeze("borrowed-static/source-fact-required"));
    }
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &rows.phase else {
        return Err(freeze("borrowed-static/source-phase-required"));
    };
    let retained = prepared
        .source_incoming
        .static_observations()
        .get(site)
        .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?
        .as_ref()
        .map_err(Clone::clone)?;
    if !Rc::ptr_eq(retained, &identity.source)
        || identity.source.call_site() != site
        || required_i64_arguments != identity.source.required_i64_arguments()
        || rows.ordered_arguments.len() != identity.source.argument_sites().len()
        || identity.candidates.len() != rows.ordered_arguments.len()
        || identity.integer_evidence.len() != rows.ordered_arguments.len()
        || !rows.opaque_actuals.is_empty()
    {
        return Err(freeze("borrowed-static/source-projection-identity"));
    }
    for (index, ((formal, candidate), argument)) in identity
        .source
        .parameters()
        .iter()
        .zip(identity.candidates.iter())
        .zip(rows.ordered_arguments.iter())
        .enumerate()
    {
        if formal.ordinal as usize != index
            || candidate.ordinal != formal.ordinal
            || formal.binding.owner() != identity.source.callee_owner()
            || identity.source.argument_sites().get(index) != Some(&candidate.site)
        {
            return Err(freeze("borrowed-static/source-projection-identity"));
        }
        let matches = match (&formal.kind, &candidate.value, argument) {
            (kind, _, LocalCallArgumentV1::BorrowedActual { ordinal, site })
                if kind.is_ordinary_borrowed_handle() =>
            {
                *ordinal == formal.ordinal && site == &candidate.site
            }
            (
                CallableParameterContractKindV1::ExactTrivial(abi),
                BorrowedCallActualValueV1::Integer(expected),
                LocalCallArgumentV1::Integer(actual),
            ) if abi.is_i64() => expected == actual,
            (
                CallableParameterContractKindV1::ExactTrivial(abi),
                BorrowedCallActualValueV1::Scalar(expected, SourceScalarKind::Integer),
                LocalCallArgumentV1::Scalar(actual),
            ) if abi.is_i64() => expected == actual && actual.owner() == site.owner(),
            _ => false,
        };
        if !matches {
            return Err(freeze("borrowed-static/source-projection-identity"));
        }
    }
    if required_i64_arguments
        .iter()
        .any(|ordinal| identity.integer_evidence.get(*ordinal as usize) != Some(&true))
    {
        if current_owner {
            return Ok(None);
        }
        return Err(freeze("borrowed-static/required-integer-source-unproved"));
    }
    Ok(Some(rows.ordered_arguments.clone()))
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_source_actual_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_local_issuer_tests.rs"]
mod local_issuer_tests;
