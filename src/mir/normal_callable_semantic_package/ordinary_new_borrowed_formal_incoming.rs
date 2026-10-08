//! Sole incoming scan retains qualified Static/Object evidence without transport admission.
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
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn duplicate_object_row_for_test(
        &mut self,
        site: &OwnedExprSiteV1,
    ) {
        let (owner, row) = self
            .observations
            .iter()
            .find(|(_, row)| row.as_ref().is_ok_and(|row| &row.call == site))
            .unwrap()
            .clone();
        self.observations.push((owner, row));
    }

    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn remove_exact_row_for_test(
        &mut self,
        site: &OwnedExprSiteV1,
    ) {
        self.observations
            .retain(|(_, row)| !row.as_ref().is_ok_and(|row| &row.call == site));
    }

    /// Original callers sharing a qualified callee retain input evidence only.
    /// The immutable inventory already corroborates its seed's sealed contract.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn has_object_input_callee_v1(
        &self,
        target: &super::super::LexicalInstanceCallSourceTargetV1,
    ) -> bool {
        self.exact_rows().any(|seed| {
            seed.source.instance().is_some_and(|original| {
                original.has_object_source_requirement()
                    && seed.callee == target.callee_owner()
                    && original.callee_owner() == seed.callee
                    && original.call_site() == &seed.call
                    && original.target() == target.target()
                    && original.target_batch_slot() == target.target_batch_slot()
            })
        })
    }
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
        stored_complete: bool,
    ) -> Result<(), BorrowedIncomingDraftErrorV1> {
        if self.observations.iter().any(|(owner, row)| {
            owner.is_none() && matches!(row, Err(BorrowedIncomingDraftErrorV1::BatchLoan))
        }) {
            return Ok(());
        }
        if !stored_complete
            || context.is_some_and(|context| {
                context
                    .arguments
                    .keys()
                    .any(|(site, _)| !seen.contains(site))
            })
        {
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
#[cfg(test)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn inventory_borrowed_incoming_calls_v1(
    batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::VerifiedSelectedCallableBatchMapV1,
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    calls: &BTreeMap<OwnedExprSiteV1, &super::super::LexicalInstanceCallSourceTargetV1>,
    ordinary_callers: &std::collections::BTreeSet<FunctionOwnerIdV1>,
    static_context: Option<&StaticIncomingContextV1<'_>>,
) -> Result<BorrowedIncomingInventoryV1, BorrowedIncomingDraftErrorV1> {
    inventory_borrowed_incoming_with_stored_dispatch_v1(
        batch,
        selected,
        drafts,
        contracts,
        calls,
        ordinary_callers,
        static_context,
        None,
    )
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn inventory_borrowed_incoming_with_stored_dispatch_v1(
    batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::VerifiedSelectedCallableBatchMapV1,
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    calls: &BTreeMap<OwnedExprSiteV1, &super::super::LexicalInstanceCallSourceTargetV1>,
    ordinary_callers: &std::collections::BTreeSet<FunctionOwnerIdV1>,
    static_context: Option<&StaticIncomingContextV1<'_>>,
    stored_dispatch: Option<&super::super::source::PreparedSourceNeedsV1>,
) -> Result<BorrowedIncomingInventoryV1, BorrowedIncomingDraftErrorV1> {
    let mut stored = BTreeMap::new();
    if let Some(prepared) = stored_dispatch {
        for need in prepared
            .as_ref()
            .map_err(|_| BorrowedIncomingDraftErrorV1::SourceIdentity)?
        {
            let need = need
                .as_ref()
                .map_err(|_| BorrowedIncomingDraftErrorV1::SourceIdentity)?;
            if let Some(super::super::source::PreparedSourceCallNeedV1::Stored {
                reference,
                receiver,
                ..
            }) = need
            {
                if stored
                    .insert(&reference.call_site, (reference, receiver))
                    .is_some()
                {
                    return Err(BorrowedIncomingDraftErrorV1::CallIdentity(
                        reference.call_site.clone(),
                    ));
                }
            }
        }
    }
    let mut stored_seen = std::collections::BTreeSet::new();
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
    // Qualified Object calls with typed inputs still need the same whole-batch
    // caller census. Their empty opaque subset grants no borrowed entry/domain.
    for (site, target) in calls {
        if !target.has_object_source_requirement() {
            continue;
        }
        let qualifications = target.object_return_sources();
        let mut matching = contracts
            .iter()
            .filter(|row| row.owner == target.callee_owner());
        let contract = matching
            .next()
            .ok_or(BorrowedIncomingDraftErrorV1::SourceIdentity)?;
        let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) =
            selected.key_for_batch_slot(contract.batch_slot)
        else {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        };
        if matching.next().is_some()
            || qualifications.is_some_and(|rows| {
                rows.is_empty()
                    || rows
                        .iter()
                        .any(|loan| loan.call() != site || loan.key() != target.target())
            })
            || target
                .object_producer_dependencies()
                .is_some_and(|rows| rows.is_empty())
            || target.call_site() != site
            || target.target() != key
            || target.target_batch_slot() != contract.batch_slot
            || key.namespace()
                != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
            || contract.mode != CallableParameterDeclarationModeV1::InstanceBoxMethod
            || key.arity() as usize != contract.parameters.len()
            || target.argument_sites().len() != contract.parameters.len()
            || contract
                .parameters
                .iter()
                .enumerate()
                .any(|(ordinal, formal)| {
                    formal.ordinal as usize != ordinal || formal.binding.owner() != contract.owner
                })
        {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        }
        if contract
            .parameters
            .iter()
            .any(|formal| formal.kind.is_ordinary_borrowed_handle())
        {
            continue;
        }
        if !contract.parameters.iter().all(|formal| matches!(formal.kind,
            crate::mir::callable_parameter_contract::CallableParameterContractKindV1::ExactTrivial(abi)
                if abi == crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64)) {
            continue;
        }
        definitions.insert(contract.owner, (contract, key));
    }
    // Zero-input Static owners have no borrowed draft. Register only an
    // original ExactI64 target in this SAME raw census, not transport owners
    // or Prepared definitions. Every matching caller/error stays below.
    if let Some(context) = static_context {
        for contract in contracts.iter().filter(|contract| {
            contract.mode == CallableParameterDeclarationModeV1::StaticBoxMethod
                && contract.parameters.is_empty()
        }) {
            let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) =
                selected.key_for_batch_slot(contract.batch_slot)
            else {
                continue;
            };
            if !context.claims.contains_zeroarg_i64_input_target(key) {
                continue;
            }
            if key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
                || key.arity() != 0
                || contracts
                    .iter()
                    .filter(|row| row.owner == contract.owner)
                    .count()
                    != 1
                || definitions
                    .insert(contract.owner, (contract, key))
                    .is_some()
            {
                return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
            }
        }
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
                let stored_target = if let Some((reference, receiver)) = stored.get(&owned) {
                    corroborate_stored_dispatch_v1(input, call, reference, receiver, selected, contracts)?;
                    if instance.is_some_and(|row| super::super::source::CallTargetReferenceV1::from_target(row) != **reference) {
                        return Err(BorrowedIncomingDraftErrorV1::CallIdentity(owned.clone()));
                    }
                    stored_seen.insert(owned.clone());
                    Some(&reference.target)
                } else { None };
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
                            // Passive dispatch excludes only a proved different callee.
                            // The same callee still requires its executable receiver.
                            if stored_target.is_some_and(|target| target != *key) {
                                continue;
                            }
                            instance.map(|row| BorrowedIncomingSourceV1::Instance((*row).clone()))
                        }
                        CallableParameterDeclarationModeV1::StaticBoxMethod => {
                            // Passive source identity is also exact about the namespace.
                            if stored_target.is_some() {
                                continue;
                            }
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
                                    BorrowedIncomingSourceV1::Static(std::rc::Rc::clone(
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
        owners: definitions.keys().copied().collect(),
        unsupported_static_spelling,
        unsupported_static_context,
        observations,
        static_observations,
    };
    inventory.corroborate_scan_completeness(
        &static_seen,
        static_context,
        stored_seen.len() == stored.len(),
    )?;
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

/// Check original passive field dispatch against the exact source call.
/// This issues no receiver inventory, incoming actual or physical permission.
fn corroborate_stored_dispatch_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    reference: &super::super::source::CallTargetReferenceV1,
    receiver: &super::super::source::StoredReceiverSourceV1,
    selected: &super::super::VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
) -> Result<(), BorrowedIncomingDraftErrorV1> {
    use crate::mir::resolved_semantics::{BodyExpressionShapeV1, BodyMeReceiverV1};
    let invalid = || BorrowedIncomingDraftErrorV1::CallIdentity(reference.call_site.clone());
    if reference.call_site.owner() != input.owner()
        || call.owner() != input.owner()
        || reference.call_site.site() != call.site()
        || reference.receiver_site != *call.receiver_site()
        || reference.target.namespace()
            != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
        || reference.target.owner() != receiver.child_class.as_ref()
        || reference.target.name() != call.selector()
        || reference.target.arity() != call.arity()
        || reference.argument_sites.len() != call.arguments().len()
        || !call
            .arguments()
            .iter()
            .enumerate()
            .all(|(ordinal, actual)| {
                actual.ordinal() as usize == ordinal
                    && actual.site() == &reference.argument_sites[ordinal]
            })
        || receiver.parent_binding.owner() != input.owner()
        || input
            .function()
            .binding(receiver.parent_binding)
            .is_none_or(|row| row.kind() != crate::mir::resolved_semantics::BindingKindV1::Receiver)
    {
        return Err(invalid());
    }
    let shape = input.body_shape().ok_or_else(invalid)?;
    if !matches!(shape.expression_shape(&reference.receiver_site),
        Some(BodyExpressionShapeV1::FieldAccess { object, field, .. })
            if object == &receiver.parent_site && field == &receiver.field_name)
        || !matches!(shape.expression_shape(&receiver.parent_site),
            Some(BodyExpressionShapeV1::Me { receiver: BodyMeReceiverV1::Lexical(binding), .. })
                if binding == &receiver.parent_binding)
    {
        return Err(invalid());
    }
    let mut targets = contracts
        .iter()
        .filter(|row| row.owner == reference.callee_owner);
    let target = targets
        .next()
        .filter(|row| {
            row.batch_slot == reference.target_batch_slot
                && row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod
                && row.parameters.len() == reference.argument_sites.len()
        })
        .ok_or_else(invalid)?;
    if targets.next().is_some()
        || !matches!(selected.key_for_batch_slot(target.batch_slot),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) if key == &reference.target)
        || !contracts.iter().any(|row| row.owner == input.owner()
            && matches!(selected.key_for_batch_slot(row.batch_slot),
                Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
                    if key.namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
                        && key.owner() == receiver.parent_class.as_ref()))
    { return Err(invalid()); }
    Ok(())
}
