//! Ordinary borrowed transport selection over the prepared lexical source rows.
//! These rows precede live-actual proof and never install a physical carrier.
use super::borrowed_formal_uses::*;
use super::*;
use crate::mir::callable_parameter_contract::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
};
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct PreparedBorrowedFormalIngressV1 {
    pub(super) definitions: BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    pub(super) forwards: Box<[BorrowedForwardUseDraftRowV1]>,
    pub(super) incoming: Box<[BorrowedIncomingCallDraftV1]>,
}

/// Only complete transport-use profiles are candidates. An ordinary source
/// use outside this profile is not an attempted borrowed ABI followed by a
/// fallback. Source identity corruption is never classified as profile-outside.
pub(in crate::mir::normal_callable_semantic_package) fn prepare_borrowed_formal_ingress_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    prepared: &PreparedLexicalInstanceCallSourceTargetsV1,
    app_main_slot: Option<u32>,
    dynamic_slot: Option<u32>,
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    instance_constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
) -> Result<PreparedBorrowedFormalIngressV1, String> {
    let mut calls = BTreeMap::new();
    for row in prepared.as_ref().map_err(Clone::clone)? {
        if let Some(row) = row.as_ref().map_err(Clone::clone)? {
            if calls.insert(row.call_site().clone(), row).is_some() {
                return Err(freeze("borrowed-formal/duplicate-source-call"));
            }
        }
    }
    let ordinary_callers: BTreeSet<_> = batch
        .declarations()
        .filter(|row| {
            Some(row.batch_slot()) != dynamic_slot
                && (selected.role_for_batch_slot(row.batch_slot()).is_some()
                    || Some(row.batch_slot()) == app_main_slot)
        })
        .map(|row| row.owner())
        .collect();
    let mut definitions = BTreeMap::new();
    let mut seen = BTreeSet::new();
    for contract in contracts {
        if !seen.insert(contract.owner) {
            return Err(freeze("borrowed-formal/duplicate-source-contract"));
        }
        if !ordinary_callers.contains(&contract.owner)
            || contract.mode != CallableParameterDeclarationModeV1::InstanceBoxMethod
            || !contract
                .parameters
                .iter()
                .any(|formal| formal.kind == CallableParameterContractKindV1::OpaqueHandle)
        {
            continue;
        }
        let receiver = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
            selected,
            batch,
            entry_home_loans.for_batch_slot(contract.batch_slot),
            contract.batch_slot,
        );
        let draft = batch
            .with_lowering_input(contract.batch_slot, |input| {
                draft_borrowed_formal_uses_v1(input, contract, instance_constructors, receiver)
            })
            .map_err(|_| freeze("borrowed-formal/batch-loan"))?;
        match draft {
            Ok(draft) => {
                definitions.insert(contract.owner, draft);
            }
            Err(
                BorrowedFormalUseDraftErrorV1::UnsupportedUse(_)
                | BorrowedFormalUseDraftErrorV1::Rebound(_)
                | BorrowedFormalUseDraftErrorV1::Captured(_)
                | BorrowedFormalUseDraftErrorV1::AnnotatedAlias(_),
            ) => {}
            Err(error) => {
                return Err(format!(
                    "{}: {error:?}",
                    freeze("borrowed-formal/source-identity")
                ));
            }
        }
    }
    // Close the finite graph before selection. Removing one outside-profile
    // destination invalidates every source that forwards an opaque value to it.
    // Repetition terminates because every nonfinal pass removes an owner.
    loop {
        let mut outside = BTreeSet::new();
        for (owner, draft) in &definitions {
            if !calls.values().any(|call| call.callee_owner() == *owner) {
                outside.insert(*owner);
            }
            for row in &draft.uses {
                let BorrowedFormalUseDraftKindV1::UnresolvedArgument { call, ordinal } = &row.kind
                else {
                    continue;
                };
                let Some(target) = calls.get(call) else {
                    outside.insert(*owner);
                    continue;
                };
                let contract = contracts
                    .iter()
                    .find(|contract| contract.owner == target.callee_owner());
                let Some(contract) = contract else {
                    return Err(freeze("borrowed-formal/missing-source-contract"));
                };
                if contract.batch_slot != target.target_batch_slot()
                    || row.site.site()
                        != target
                            .argument_sites()
                            .get(*ordinal as usize)
                            .ok_or_else(|| freeze("borrowed-formal/argument-ordinal"))?
                {
                    return Err(freeze("borrowed-formal/forward-source-identity"));
                }
                if !definitions.contains_key(&contract.owner)
                    || contract
                        .parameters
                        .get(*ordinal as usize)
                        .is_none_or(|formal| {
                            formal.kind != CallableParameterContractKindV1::OpaqueHandle
                        })
                {
                    outside.insert(*owner);
                }
            }
        }
        if outside.is_empty() {
            break;
        }
        for owner in outside {
            definitions.remove(&owner);
        }
    }
    let forwards = join_borrowed_forward_uses_v1(&definitions, contracts, &calls)
        .map_err(|error| format!("{}: {error:?}", freeze("borrowed-formal/forward-coverage")))?;
    // After profile selection, unresolved or outside-scope incoming calls are
    // named terminals. Never remove the selected definition to regain old ABI.
    let incoming = draft_borrowed_incoming_calls_v1(
        batch,
        selected,
        &definitions,
        contracts,
        &calls,
        &ordinary_callers,
    )
    .map_err(|error| format!("{}: {error:?}", freeze("borrowed-formal/incoming-coverage")))?;
    Ok(PreparedBorrowedFormalIngressV1 {
        definitions,
        forwards,
        incoming,
    })
}

impl PreparedBorrowedFormalIngressV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn incoming_calls_for_owner(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> bool {
        self.incoming.iter().any(|row| row.call.owner() == owner)
    }

    /// Corroborate the final issuer consumes precisely the source rows used by
    /// selection. This does not authorize actual liveness, result or tagged ABI.
    pub(super) fn corroborate_source_targets(
        &self,
        prepared: &[Result<Option<LexicalInstanceCallSourceTargetV1>, String>],
    ) -> Result<(), String> {
        let calls: BTreeMap<_, _> = prepared
            .iter()
            .filter_map(|row| {
                row.as_ref()
                    .ok()
                    .and_then(Option::as_ref)
                    .map(|row| (row.call_site(), row))
            })
            .collect();
        for incoming in &self.incoming {
            let source = calls
                .get(&incoming.call)
                .ok_or_else(|| freeze("borrowed-formal/final-source-missing"))?;
            let draft = self
                .definitions
                .get(&incoming.callee)
                .ok_or_else(|| freeze("borrowed-formal/final-definition-missing"))?;
            if *source != &incoming.source
                || source.callee_owner() != incoming.callee
                || incoming.arguments.iter().any(|(ordinal, site, formal)| {
                    source.argument_sites().get(*ordinal as usize) != Some(site)
                        || draft.origins.get(formal) != Some(formal)
                })
            {
                return Err(freeze("borrowed-formal/final-incoming-drift"));
            }
        }
        for forward in &self.forwards {
            let source = calls
                .get(&forward.call)
                .ok_or_else(|| freeze("borrowed-formal/final-source-missing"))?;
            if source.target() != &forward.target
                || source.callee_owner() != forward.callee_formal.owner()
                || source.argument_sites().get(forward.ordinal as usize)
                    != Some(forward.site.site())
                || self
                    .definitions
                    .get(&forward.site.owner())
                    .and_then(|draft| draft.origins.get(&forward.binding))
                    != Some(&forward.source_formal)
            {
                return Err(freeze("borrowed-formal/final-forward-drift"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_source_tests.rs"]
mod tests;
