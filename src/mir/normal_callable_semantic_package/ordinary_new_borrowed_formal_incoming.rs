//! Sole incoming scan retains qualified Static evidence without transport admission.
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
#[path = "ordinary_new_borrowed_static_inventory.rs"]
mod static_inventory;
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) use static_inventory::StaticIncomingContextV1;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct BorrowedIncomingInventoryV1
{
    owners: std::collections::BTreeSet<FunctionOwnerIdV1>,
    unsupported_static_spelling: std::collections::BTreeSet<FunctionOwnerIdV1>,
    unsupported_static_context: std::collections::BTreeSet<FunctionOwnerIdV1>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) static_observations:
        BTreeMap<OwnedExprSiteV1, static_inventory::StaticIncomingObservationV1>,
    observations: Vec<(
        Option<FunctionOwnerIdV1>,
        Result<BorrowedIncomingCallDraftV1, BorrowedIncomingDraftErrorV1>,
    )>,
}
impl BorrowedIncomingInventoryV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn has_unsupported_static_spelling(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> bool {
        self.unsupported_static_spelling.contains(&owner)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn has_unsupported_static_context(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> bool {
        self.unsupported_static_context.contains(&owner)
    }

    /// Orphan proof requires a complete scan. A global loan failure remains
    /// retained and fails every final projection, including the empty set.
    fn corroborate_scan_completeness(
        &self,
        seen: &std::collections::BTreeSet<OwnedExprSiteV1>,
        context: Option<&StaticIncomingContextV1<'_>>,
    ) -> Result<(), BorrowedIncomingDraftErrorV1> {
        if self.observations.iter().any(|(owner, row)| {
            owner.is_none() && matches!(row, Err(BorrowedIncomingDraftErrorV1::BatchLoan))
        }) {
            return Ok(());
        }
        if context.is_some_and(|context| {
            context
                .arguments
                .keys()
                .any(|(site, _)| !seen.contains(site))
        }) {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        }
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn static_observations(
        &self,
    ) -> &BTreeMap<OwnedExprSiteV1, static_inventory::StaticIncomingObservationV1> {
        &self.static_observations
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn exact_rows(
        &self,
    ) -> impl Iterator<Item = &BorrowedIncomingCallDraftV1> {
        self.observations
            .iter()
            .filter_map(|(_, row)| row.as_ref().ok())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn vetoed_owners(
        &self,
    ) -> std::collections::BTreeSet<FunctionOwnerIdV1> {
        let mut vetoes = std::collections::BTreeSet::new();
        for (owner, row) in &self.observations {
            if row.is_err() {
                match owner {
                    Some(owner) => {
                        vetoes.insert(*owner);
                    }
                    None => return self.owners.clone(),
                }
            }
        }
        vetoes
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn project(
        &self,
        owners: &std::collections::BTreeSet<FunctionOwnerIdV1>,
    ) -> Result<Box<[BorrowedIncomingCallDraftV1]>, BorrowedIncomingDraftErrorV1> {
        if !owners.is_subset(&self.owners) {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        }
        self.observations
            .iter()
            .filter(|(owner, _)| owner.is_none_or(|owner| owners.contains(&owner)))
            .map(|(_, row)| row.clone())
            .collect::<Result<Vec<_>, _>>()
            .map(Vec::into_boxed_slice)
    }
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
            || !matches!(
                (key.namespace(), contract.mode),
                (
                    hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod,
                    CallableParameterDeclarationModeV1::InstanceBoxMethod
                ) | (
                    hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod,
                    CallableParameterDeclarationModeV1::StaticBoxMethod
                )
            )
            || key.arity() as usize != contract.parameters.len()
            || !contract
                .parameters
                .iter()
                .enumerate()
                .all(|(ordinal, formal)| {
                    formal.ordinal as usize == ordinal
                        && formal.binding.owner() == *owner
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
    let mut observations = Vec::new();
    let mut unsupported_static_spelling = std::collections::BTreeSet::new();
    let mut unsupported_static_context = std::collections::BTreeSet::new();
    let mut seen = std::collections::BTreeSet::new();
    for declaration in batch.declarations() {
        let loan = batch.with_lowering_input(declaration.batch_slot(), |input| {
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
                let instance = calls.get(&owned);
                for (callee, (contract, key)) in &definitions {
                    if call.selector() != key.name() || call.arity() != key.arity() {
                        continue;
                    }
                    let exact = match contract.mode {
                        CallableParameterDeclarationModeV1::InstanceBoxMethod => {
                            // A qualified original Static loan belongs to a different
                            // namespace. Missing/failed authority remains a veto.
                            if let Some(Ok(original)) = static_observations.get(&owned) {
                                let mut target_contracts = contracts.iter().filter(|row|
                                    row.owner == original.callee_owner());
                                let target_contract = target_contracts.next()
                                    .ok_or(BorrowedIncomingDraftErrorV1::SourceIdentity)?;
                                if instance.is_some()
                                    || original.call_site() != &owned
                                    || original.target().namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
                                    || original.target().name() != call.selector()
                                    || original.target().arity() != call.arity()
                                    || original.argument_sites().len() != call.arguments().len()
                                    || target_contracts.next().is_some()
                                    || target_contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
                                    || target_contract.batch_slot != original.target_batch_slot()
                                    || !matches!(selected.key_for_batch_slot(target_contract.batch_slot),
                                        Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(target)) if target == original.target())
                                    || original.parameters().len() != target_contract.parameters.len()
                                    || original.parameters().iter().zip(&target_contract.parameters).any(|(source, formal)|
                                        source.ordinal != formal.ordinal || source.binding != formal.binding || source.kind != formal.kind)
                                    || !call.arguments().iter().enumerate().all(|(ordinal, argument)|
                                        argument.ordinal() as usize == ordinal
                                            && original.argument_sites()[ordinal] == *argument.site())
                                {
                                    return Err(BorrowedIncomingDraftErrorV1::CallIdentity(owned.clone()));
                                }
                                continue;
                            }
                            instance.map(|row| BorrowedIncomingSourceV1::Instance((*row).clone()))
                        }
                        CallableParameterDeclarationModeV1::StaticBoxMethod => {
                            // The original proved Instance source is another namespace.
                            if let Some(original) = instance {
                                if original.call_site() != &owned
                                    || original.target().namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
                                    || original.target().name() != call.selector()
                                    || original.target().arity() != call.arity()
                                    || original.argument_sites().len() != call.arguments().len()
                                    || !call.arguments().iter().enumerate().all(|(ordinal, argument)|
                                        argument.ordinal() as usize == ordinal
                                        && original.argument_sites()[ordinal] == *argument.site())
                                {
                                    return Err(BorrowedIncomingDraftErrorV1::CallIdentity(owned.clone()));
                                }
                                continue;
                            }
                            static_observations
                                .get(&owned)
                                .and_then(|row| row.as_ref().ok())
                                .map(|row| {
                                    BorrowedIncomingSourceV1::QualifiedStatic(std::rc::Rc::clone(
                                        row,
                                    ))
                                })
                        }
                        _ => return Err(BorrowedIncomingDraftErrorV1::SourceIdentity),
                    };
                    let Some(exact) = exact else {
                        observations.push((
                            Some(*callee),
                            Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(
                                owned.clone(),
                            )),
                        ));
                        continue;
                    };
                    // A proven different receiver class is not an incoming
                    // edge of this definition, despite the same method name.
                    if exact.target() != *key {
                        continue;
                    }
                    if !ordinary_callers.contains(&input.owner()) {
                        observations.push((
                            Some(*callee),
                            Err(BorrowedIncomingDraftErrorV1::OutsideOrdinaryScope(
                                owned.clone(),
                            )),
                        ));
                        continue;
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
                        observations.push((
                            Some(*callee),
                            Err(BorrowedIncomingDraftErrorV1::CallIdentity(owned.clone())),
                        ));
                        continue;
                    }
                    // Capability exclusions apply to the whole callee. Retain
                    // the original row below, including every caller and veto.
                    if contract.mode == CallableParameterDeclarationModeV1::StaticBoxMethod {
                        if !input.function().expression_source().initializers().any(|initializer|
                            initializer.initializer_site() == Some(site)) {
                            unsupported_static_context.insert(*callee);
                        }
                        if call.arguments().iter().zip(&contract.parameters).any(|(argument, formal)|
                            formal.kind.is_ordinary_borrowed_handle()
                                && crate::mir::resolved_semantics::home_new_prefix::borrowed_actual_source_atom_v1(input, argument.site()).is_none()) {
                            unsupported_static_spelling.insert(*callee);
                        }
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
                    observations.push((
                        Some(*callee),
                        Ok(BorrowedIncomingCallDraftV1 {
                            source: exact,
                            call: owned.clone(),
                            callee: *callee,
                            arguments,
                        }),
                    ));
                    seen.insert(*callee);
                }
            }
            Ok(())
        });
        match loan {
            Ok(result) => result?,
            Err(_) => {
                observations.push((None, Err(BorrowedIncomingDraftErrorV1::BatchLoan)));
                break;
            }
        }
    }
    for owner in definitions.keys() {
        if !seen.contains(owner) {
            observations.push((
                Some(*owner),
                Err(BorrowedIncomingDraftErrorV1::NoIncoming(*owner)),
            ));
        }
    }
    let inventory = BorrowedIncomingInventoryV1 {
        owners: drafts.keys().copied().collect(),
        unsupported_static_spelling,
        unsupported_static_context,
        observations,
        static_observations,
    };
    inventory.corroborate_scan_completeness(&static_seen, static_context)?;
    Ok(inventory)
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_source_graph_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_source_domain_tests.rs"]
mod static_source_domain_tests;

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_capability_tests.rs"]
mod static_capability_tests;
