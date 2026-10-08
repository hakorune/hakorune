//! Sole incoming seed producer for the original borrowed source inventory.
//! No transport owner or execution permission is selected here.
use super::*;

/// Co-seal every borrowed formal's source domain before the declaration
/// walk: classify each incoming argument's exact source expression — never
/// MIR types or runtime layout — then agree the classes per callee formal
/// across all calls, resolving Integer/Object forward chains in one fixed point. This
/// mints admission evidence only; the physical param row still must carry
/// the canonical object id it claims.
#[allow(clippy::too_many_arguments)]
pub(super) fn prepare_borrowed_formal_views_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    instance_constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    callable_result_classes: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    local_candidates: &BTreeMap<
        u32,
        Result<
            Vec<super::super::super::candidate::OrdinaryNewCandidate>,
            super::super::super::OrdinaryNewCoSealIssueV1,
        >,
    >,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    transport_owners: &BTreeSet<FunctionOwnerIdV1>,
    inventory: &BorrowedIncomingInventoryV1,
    guarded_actuals: &BTreeMap<(OwnedExprSiteV1, u32), BorrowedGuardedActualV1>,
) -> Result<
    (
        BTreeMap<BindingRefV1, BorrowedFormalObjectViewV1>,
        BTreeSet<BindingRefV1>,
        BTreeSet<BindingRefV1>,
    ),
    String,
> {
    let slots: BTreeMap<FunctionOwnerIdV1, u32> = batch
        .declarations()
        .map(|row| (row.owner(), row.batch_slot()))
        .collect();
    let mut seeds: BTreeMap<BindingRefV1, Vec<FormalActualSeedV1>> = BTreeMap::new();
    let mut declared = BTreeSet::new();
    let vetoed = inventory.vetoed_owners();
    for contract in contracts
        .iter()
        .filter(|row| definitions.contains_key(&row.owner))
    {
        for formal in &contract.parameters {
            if formal.kind.is_ordinary_borrowed_handle() && vetoed.contains(&contract.owner) {
                seeds
                    .entry(formal.binding)
                    .or_default()
                    .push(FormalActualSeedV1::Conflict);
            }
            if let CallableParameterContractKindV1::DeclaredObject(class) = &formal.kind {
                declared.insert(formal.binding);
                seeds
                    .entry(formal.binding)
                    .or_default()
                    .push(FormalActualSeedV1::Class(class.clone()));
            }
        }
    }
    for call in inventory.exact_rows() {
        let caller = call.call.owner();
        let caller_slot = slots
            .get(&caller)
            .ok_or_else(|| freeze("borrowed-view/caller-slot"))?;
        let candidates = local_candidates
            .get(caller_slot)
            .and_then(|rows| rows.as_ref().ok())
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let receiver = contracts.iter().find(|row| row.owner == caller)
            .filter(|row| row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod)
            .and_then(|_| crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
                selected, batch, entry_home_loans.for_batch_slot(*caller_slot), *caller_slot,
            ));
        let arguments: Vec<_> = call.arguments.to_vec();
        batch
            .with_lowering_input(*caller_slot, |input| {
                for (ordinal, site, formal) in &arguments {
                    seeds
                        .entry(*formal)
                        .or_default()
                        .push(classify_actual_seed_in_scope(
                            input,
                            batch,
                            selected,
                            callable_result_classes,
                            caller,
                            candidates,
                            receiver,
                            definitions,
                            contracts,
                            site,
                            Some(transport_owners),
                            guarded_actuals
                                .get(&(call.call.clone(), *ordinal))
                                .map(|fact| (fact, &call.call, *ordinal)),
                        ));
                }
            })
            .map_err(|_| freeze("borrowed-view/caller-loan"))?;
    }
    let (mut objects, integers) =
        value_domain::resolve_formal_domains_v1(&seeds, batch, instance_constructors);
    let mut mismatches = BTreeSet::new();
    for formal in &declared {
        if let Some(view) = objects.get_mut(formal) {
            view.declared = true;
        } else {
            mismatches.insert(*formal);
        }
    }
    Ok((objects, integers, mismatches))
}
