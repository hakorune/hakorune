//! Sole incoming scan retains qualified Static evidence without transport admission.
use super::*;
#[path = "ordinary_new_borrowed_static_inventory.rs"]
mod static_inventory;
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) use static_inventory::StaticIncomingContextV1;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct BorrowedIncomingInventoryV1
{
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) incoming:
        Box<[BorrowedIncomingCallDraftV1]>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) static_observations:
        BTreeMap<OwnedExprSiteV1, static_inventory::StaticIncomingObservationV1>,
}

/// Enumerate the complete source batch, including unselected callers. An
/// unresolved selector/arity match can veto selection but never proves a
/// target. `ordinary_callers` must later be corroborated against the issued
/// Ordinary source scopes; these draft rows install no ABI or live actual.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn inventory_borrowed_incoming_calls_v1(
    batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::VerifiedSelectedCallableBatchMapV1,
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    calls: &BTreeMap<OwnedExprSiteV1, &super::super::LexicalInstanceCallSourceTargetV1>,
    ordinary_callers: &std::collections::BTreeSet<FunctionOwnerIdV1>,
    static_context: Option<&StaticIncomingContextV1<'_>>,
) -> Result<BorrowedIncomingInventoryV1, BorrowedIncomingDraftErrorV1> {
    let mut definitions = BTreeMap::new();
    for owner in drafts.keys() {
        let mut matching = contracts.iter().filter(|row| row.owner == *owner);
        let contract = matching
            .next()
            .ok_or(BorrowedIncomingDraftErrorV1::SourceIdentity)?;
        let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) =
            selected.key_for_batch_slot(contract.batch_slot)
        else {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        };
        if matching.next().is_some()
            || key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
            || key.arity() as usize != contract.parameters.len()
            || contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
            || !contract.parameters.iter().enumerate().all(|(ordinal, formal)| {
                formal.ordinal as usize == ordinal && formal.binding.owner() == *owner
                    && (!formal.kind.is_ordinary_borrowed_handle()
                        || drafts[owner].origins.get(&formal.binding) == Some(&formal.binding))
            })
            || !contract
                .parameters
                .iter()
                .any(|formal| formal.kind.is_ordinary_borrowed_handle())
        {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        }
        definitions.insert(*owner, (contract, key));
    }
    let mut static_observations = BTreeMap::new();
    let mut static_seen = std::collections::BTreeSet::new();
    let mut rows = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for declaration in batch.declarations() {
        batch
            .with_lowering_input(declaration.batch_slot(), |input| {
                for (site, call) in input.function().method_calls() {
                    let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
                    static_seen.insert(owned.clone());
                    if let Some(context) = static_context {
                        if let Some(row) = context.observe(
                            declaration.batch_slot(),
                            &owned,
                            call,
                            selected,
                            contracts,
                        )? {
                            if static_observations.insert(owned.clone(), row).is_some() {
                                return Err(BorrowedIncomingDraftErrorV1::CallIdentity(owned));
                            }
                        }
                    }
                    let exact = calls.get(&owned);
                    for (callee, (contract, key)) in &definitions {
                        if call.selector() != key.name() || call.arity() != key.arity() {
                            continue;
                        }
                        let Some(exact) = exact else {
                            return Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(owned));
                        };
                        // A proven different receiver class is not an incoming
                        // edge of this definition, despite the same method name.
                        if exact.target() != *key {
                            continue;
                        }
                        if !ordinary_callers.contains(&input.owner()) {
                            return Err(BorrowedIncomingDraftErrorV1::OutsideOrdinaryScope(owned));
                        }
                        if exact.call_site() != &owned
                            || exact.callee_owner() != *callee
                            || exact.target_batch_slot() != contract.batch_slot
                            || call.owner() != input.owner()
                            || call.site() != site
                            || call.arguments().len() != contract.parameters.len()
                            || exact.argument_sites().len() != call.arguments().len()
                            || !call
                                .arguments()
                                .iter()
                                .enumerate()
                                .all(|(ordinal, argument)| {
                                    argument.ordinal() as usize == ordinal
                                        && exact.argument_sites()[ordinal] == *argument.site()
                                        && contract.parameters[ordinal].ordinal as usize == ordinal
                                })
                        {
                            return Err(BorrowedIncomingDraftErrorV1::CallIdentity(owned));
                        }
                        let arguments = call
                            .arguments()
                            .iter()
                            .zip(&contract.parameters)
                            .filter(|(_, formal)| formal.kind.is_ordinary_borrowed_handle())
                            .map(|(argument, formal)| {
                                (argument.ordinal(), argument.site().clone(), formal.binding)
                            })
                            .collect();
                        rows.push(BorrowedIncomingCallDraftV1 {
                            source: (*exact).clone(),
                            call: owned.clone(),
                            callee: *callee,
                            arguments,
                        });
                        seen.insert(*callee);
                    }
                }
                Ok(())
            })
            .map_err(|_| BorrowedIncomingDraftErrorV1::BatchLoan)??;
    }
    for owner in definitions.keys() {
        if !seen.contains(owner) {
            return Err(BorrowedIncomingDraftErrorV1::NoIncoming(*owner));
        }
    }
    if static_context.is_some_and(|context| {
        context
            .arguments
            .keys()
            .any(|(site, _)| !static_seen.contains(site))
    }) {
        return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
    }
    Ok(BorrowedIncomingInventoryV1 {
        incoming: rows.into_boxed_slice(),
        static_observations,
    })
}
