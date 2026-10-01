//! Source/domain/liveness join of the existing borrowed ingress preparation.
//! Pending actuals do not arm LocalCallObservation or a physical signature.
use super::borrowed_formal_source::PreparedBorrowedFormalIngressV1;
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    BorrowedCallActualCandidateV1, BorrowedCallActualValueV1, LocalCallArgumentV1, SourceScalarKind,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BorrowedFormalActualSourceV1 {
    Integer(i64),
    Bool(bool),
    Scalar {
        binding: BindingRefV1,
        kind: SourceScalarKind,
    },
    TypedHome {
        binding: BindingRefV1,
        root: BindingRefV1,
        acquisition: OwnedExprSiteV1,
        class: Box<str>,
    },
    EntryReceiver {
        binding: BindingRefV1,
        root: BindingRefV1,
        class: Box<str>,
    },
    Forwarded {
        binding: BindingRefV1,
        formal: BindingRefV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreparedBorrowedFormalActualV1 {
    pub(crate) ordinal: u32,
    pub(crate) site: SourceExprSiteV1,
    pub(crate) formal: BindingRefV1,
    pub(crate) source: BorrowedFormalActualSourceV1,
}

/// The same pending call owns both its opaque proofs and the full ordered
/// argument projection. No second site inventory or domain authority is minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::normal_callable_semantic_package) struct PreparedBorrowedCallActualsV1 {
    pub(super) opaque_actuals: Box<[PreparedBorrowedFormalActualV1]>,
    pub(super) ordered_arguments: Box<[LocalCallArgumentV1]>,
}

pub(in crate::mir::normal_callable_semantic_package) type PendingBorrowedFormalActualsV1 =
    BTreeMap<OwnedExprSiteV1, Result<PreparedBorrowedCallActualsV1, String>>;

pub(in crate::mir::normal_callable_semantic_package) fn prepare_borrowed_call_actuals_v1(
    prepared: &Result<PreparedBorrowedFormalIngressV1, String>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &OwnedExprSiteV1,
    actuals: &[BorrowedCallActualCandidateV1],
    candidates: &[super::super::candidate::OrdinaryNewCandidate],
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
) -> Result<Option<PreparedBorrowedCallActualsV1>, String> {
    let prepared = prepared.as_ref().map_err(Clone::clone)?;
    let mut incoming = prepared.incoming.iter().filter(|row| &row.call == call);
    let Some(incoming_row) = incoming.next() else {
        return Ok(None);
    };
    if incoming.next().is_some() {
        return Err(freeze("borrowed-actual/duplicate-incoming"));
    }
    let mut contracts = contracts
        .iter()
        .filter(|row| row.owner == incoming_row.callee);
    let contract = contracts
        .next()
        .ok_or_else(|| freeze("borrowed-actual/formal-missing"))?;
    if contracts.next().is_some() || actuals.len() != contract.parameters.len() {
        return Err(freeze("borrowed-actual/arity"));
    }
    if incoming_row.source.call_site() != call
        || incoming_row.source.callee_owner() != contract.owner
        || incoming_row.source.target_batch_slot() != contract.batch_slot
        || incoming_row.source.argument_sites().len() != contract.parameters.len()
    {
        return Err(freeze("borrowed-actual/source-identity"));
    }
    // The opaque subset cannot prove the unchanged scalar arguments beside
    // it. Cover every source ordinal before lending any successful actuals
    // to entry adoption or a continuation of this selected definition.
    for (index, (formal, actual)) in contract.parameters.iter().zip(actuals).enumerate() {
        if formal.ordinal as usize != index
            || formal.binding.owner() != contract.owner
            || actual.ordinal != formal.ordinal
            || actual.site != incoming_row.source.argument_sites()[index]
        {
            return Err(freeze("borrowed-actual/source-identity"));
        }
        match formal.kind {
            CallableParameterContractKindV1::OpaqueHandle => {}
            CallableParameterContractKindV1::ExactTrivial(_) => match &actual.value {
                BorrowedCallActualValueV1::Integer(_) => {}
                BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer)
                    if binding.owner() == call.owner() => {}
                _ => return Err(freeze("borrowed-actual/nonopaque-scalar-unproved")),
            },
            // Existing Map/Text/declared-handle contracts need their own
            // source consumer proof; an opaque neighbour cannot provide it.
            _ => return Err(freeze("borrowed-actual/nonopaque-contract-unproved")),
        }
    }
    let mut rows = Vec::new();
    for (ordinal, site, formal) in &incoming_row.arguments {
        let actual = actuals
            .get(*ordinal as usize)
            .ok_or_else(|| freeze("borrowed-actual/ordinal"))?;
        if actual.ordinal != *ordinal || actual.site != *site || formal.owner() != incoming_row.callee
            || contract.parameters.get(*ordinal as usize).is_none_or(|row| {
                row.ordinal != *ordinal || row.binding != *formal
                    || row.kind != crate::mir::callable_parameter_contract::CallableParameterContractKindV1::OpaqueHandle
            })
        { return Err(freeze("borrowed-actual/source-identity")); }
        let source = match &actual.value {
            BorrowedCallActualValueV1::Integer(value) => {
                BorrowedFormalActualSourceV1::Integer(*value)
            }
            BorrowedCallActualValueV1::Bool(value) => BorrowedFormalActualSourceV1::Bool(*value),
            BorrowedCallActualValueV1::Scalar(binding, kind) if binding.owner() == call.owner() => {
                BorrowedFormalActualSourceV1::Scalar {
                    binding: *binding,
                    kind: *kind,
                }
            }
            BorrowedCallActualValueV1::Home {
                binding,
                root,
                acquisition,
            } => {
                let mut matching = candidates
                    .iter()
                    .filter(|row| &row.site == acquisition && row.destination == *root);
                let candidate = matching
                    .next()
                    .ok_or_else(|| freeze("borrowed-actual/home-source"))?;
                if matching.next().is_some()
                    || binding.owner() != call.owner()
                    || root.owner() != call.owner()
                    || acquisition.owner() != call.owner()
                    || !candidate.construction.is_ok()
                {
                    return Err(freeze("borrowed-actual/home-domain"));
                }
                BorrowedFormalActualSourceV1::TypedHome {
                    binding: *binding,
                    root: *root,
                    acquisition: acquisition.clone(),
                    class: candidate.class.clone(),
                }
            }
            BorrowedCallActualValueV1::SelfRooted { binding, root } => {
                if let Some(formal_origin) = prepared
                    .definitions
                    .get(&call.owner())
                    .and_then(|draft| draft.origins.get(binding))
                {
                    if root != formal_origin
                        || !prepared.forwards.iter().any(|row| {
                            &row.call == call
                                && row.ordinal == *ordinal
                                && row.site.site() == site
                                && row.binding == *binding
                                && row.source_formal == *formal_origin
                                && row.callee_formal == *formal
                        })
                    {
                        return Err(freeze("borrowed-actual/forward-identity"));
                    }
                    BorrowedFormalActualSourceV1::Forwarded {
                        binding: *binding,
                        formal: *formal_origin,
                    }
                } else {
                    let (entry, class) =
                        receiver.ok_or_else(|| freeze("borrowed-actual/entry-domain"))?;
                    if *root != entry
                        || binding.owner() != call.owner()
                        || entry.owner() != call.owner()
                    {
                        return Err(freeze("borrowed-actual/entry-source"));
                    }
                    BorrowedFormalActualSourceV1::EntryReceiver {
                        binding: *binding,
                        root: entry,
                        class: class.name().into(),
                    }
                }
            }
            _ => return Err(freeze("borrowed-actual/unsupported-or-unavailable")),
        };
        rows.push(PreparedBorrowedFormalActualV1 {
            ordinal: *ordinal,
            site: site.clone(),
            formal: *formal,
            source,
        });
    }
    let ordered_arguments = actuals
        .iter()
        .map(|actual| {
            if rows.iter().any(|row| row.ordinal == actual.ordinal) {
                Ok(LocalCallArgumentV1::BorrowedActual {
                    ordinal: actual.ordinal,
                    site: actual.site.clone(),
                })
            } else {
                match actual.value {
                    BorrowedCallActualValueV1::Integer(value) => {
                        Ok(LocalCallArgumentV1::Integer(value))
                    }
                    BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer) => {
                        Ok(LocalCallArgumentV1::Scalar(binding))
                    }
                    _ => Err(freeze("borrowed-actual/nonopaque-scalar-unproved")),
                }
            }
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Some(PreparedBorrowedCallActualsV1 {
        opaque_actuals: rows.into_boxed_slice(),
        ordered_arguments: ordered_arguments.into_boxed_slice(),
    }))
}

pub(in crate::mir::normal_callable_semantic_package) fn stage_borrowed_call_actuals_v1(
    staged: &mut PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    result: Result<Option<PreparedBorrowedCallActualsV1>, String>,
) {
    let result = match result {
        Ok(None) => return,
        Ok(Some(rows)) => Ok(rows),
        Err(error) => Err(error),
    };
    if let Some(previous) = staged.get(site) {
        if previous != &result {
            staged.insert(
                site.clone(),
                Err(freeze("borrowed-actual/repeated-walk-drift")),
            );
        }
    } else {
        staged.insert(site.clone(), result);
    }
}

/// A failed partial source walk cannot leave successful actual rows behind.
pub(in crate::mir::normal_callable_semantic_package) fn reject_borrowed_actuals_for_owner_v1(
    prepared: &Result<PreparedBorrowedFormalIngressV1, String>,
    owner: FunctionOwnerIdV1,
    staged: &mut PendingBorrowedFormalActualsV1,
    issue: String,
) {
    if let Ok(prepared) = prepared {
        for incoming in prepared
            .incoming
            .iter()
            .filter(|row| row.call.owner() == owner)
        {
            staged.insert(incoming.call.clone(), Err(issue.clone()));
        }
    }
}

pub(in crate::mir::normal_callable_semantic_package) fn finish_borrowed_call_actuals_v1(
    prepared: &Result<PreparedBorrowedFormalIngressV1, String>,
    staged: &mut PendingBorrowedFormalActualsV1,
) {
    if let Ok(prepared) = prepared {
        for incoming in &prepared.incoming {
            staged
                .entry(incoming.call.clone())
                .or_insert_with(|| Err(freeze("borrowed-actual/selected-incoming-unobserved")));
        }
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_actual_tests.rs"]
mod tests;
