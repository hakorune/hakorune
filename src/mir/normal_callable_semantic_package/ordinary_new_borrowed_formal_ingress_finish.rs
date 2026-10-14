//! Prepare the original incoming inventory once, then select transport from it.
//! No source rescan or provisional executable Stored receiver occurs at finish.
use super::*;

#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn prepare_original_incoming_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    omitted: &BTreeSet<FunctionOwnerIdV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    calls: &BTreeMap<OwnedExprSiteV1, &LexicalInstanceCallSourceTargetV1>,
    ordinary_callers: &BTreeSet<FunctionOwnerIdV1>,
    definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    static_claims: Option<&crate::mir::normal_callable_semantic_package::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1>,
    static_arguments: &BTreeMap<
        (OwnedExprSiteV1, u32),
        super::super::borrowed_static_argument::StaticArgumentSourceV1,
    >,
    app_main: Option<&BorrowedAppMainSourceLoanV1<'_>>,
    stored_dispatch: Option<&super::super::source::PreparedSourceNeedsV1>,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
) -> Result<BorrowedIncomingInventoryV1, String> {
    let context = static_claims.map(|claims| StaticIncomingContextV1 {
        claims,
        arguments: static_arguments,
        main: app_main,
    });
    inventory_borrowed_incoming_with_stored_dispatch_v1(
        batch,
        selected,
        omitted,
        definitions,
        contracts,
        calls,
        ordinary_callers,
        context.as_ref(),
        stored_dispatch,
        Some(constructors),
    )
    .map_err(|error| format!("{}: {error:?}", freeze("borrowed-formal/incoming-coverage")))
}

/// Consume the original drafts once; never reconstruct them after profile selection.
#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn finish_ingress_from_drafts_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    instance_constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    local_candidates: &BTreeMap<
        u32,
        Result<
            Vec<super::super::super::candidate::OrdinaryNewCandidate>,
            super::super::super::OrdinaryNewCoSealIssueV1,
        >,
    >,
    callable_result_classes: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    calls: BTreeMap<OwnedExprSiteV1, &LexicalInstanceCallSourceTargetV1>,
    static_claims: Option<&crate::mir::normal_callable_semantic_package::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1>,
    app_main: Option<&BorrowedAppMainSourceLoanV1<'_>>,
    drafts: (
        BTreeSet<FunctionOwnerIdV1>,
        BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
        BTreeSet<OwnedExprSiteV1>,
        BTreeMap<(OwnedExprSiteV1, u32), BorrowedGuardedActualV1>,
    ),
    static_arguments: BTreeMap<
        (OwnedExprSiteV1, u32),
        super::borrowed_static_argument::StaticArgumentSourceV1,
    >,
    original_inventory: Result<BorrowedIncomingInventoryV1, String>,
) -> Result<PreparedBorrowedFormalIngressV1, String> {
    let (_ordinary_callers, definitions, dominated_view_sites, guarded_actuals) = drafts;
    let mut transport_owners: BTreeSet<_> = contracts
        .iter()
        .filter(|row| {
            row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod
                && definitions.contains_key(&row.owner)
        })
        .map(|row| row.owner)
        .collect();
    let mut inventory = original_inventory?;
    inventory.finish_selected_calls_v1(&calls)
        .map_err(|error| format!("{}: {error:?}", freeze("borrowed-formal/incoming-coverage")))?;
    let checked_static_inputs =
        checked_input::issue_checked_static_inputs_v1(contracts, &definitions, &static_arguments)?;
    let mut target_static = target_static::issue_target_static_source_cohorts_v1(
        batch,
        selected,
        contracts,
        &definitions,
        &inventory,
        &static_arguments,
        &checked_static_inputs,
        static_claims,
    )?;
    source_drafts::seed_static_transport_owners_v1(
        selected,
        contracts,
        &definitions,
        &inventory,
        &static_arguments,
        &checked_static_inputs,
        &mut transport_owners,
    )?;
    let call_sources = borrow_call_sources_v1(
        &calls,
        inventory
            .static_observations()
            .values()
            .filter_map(|row| row.as_ref().ok().map(|source| source.as_ref())),
    )?;
    // Close the finite graph before selection. Removing one outside-profile
    // destination invalidates every source that forwards an opaque value to it.
    // Repetition terminates because every nonfinal pass removes an owner.
    loop {
        let mut outside = BTreeSet::new();
        for owner in &transport_owners {
            let draft = &definitions[owner];
            if !call_sources
                .values()
                .any(|call| call.callee_owner() == *owner)
            {
                outside.insert(*owner);
            }
            for row in &draft.uses {
                let BorrowedFormalUseDraftKindV1::UnresolvedArgument { call, ordinal } = &row.kind
                else {
                    continue;
                };
                let Some(target) = call_sources.get(call) else {
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
                if !transport_owners.contains(&contract.owner)
                    || contract
                        .parameters
                        .get(*ordinal as usize)
                        .is_none_or(|formal| !formal.kind.is_ordinary_borrowed_handle())
                {
                    outside.insert(*owner);
                }
            }
        }
        if outside.is_empty() {
            break;
        }
        for owner in outside {
            transport_owners.remove(&owner);
        }
    }
    let (mut object_views, integer_agreements, source_declared_mismatches) =
        prepare_borrowed_formal_views_v1(
            batch,
            selected,
            entry_home_loans,
            instance_constructors,
            callable_result_classes,
            local_candidates,
            contracts,
            &definitions,
            &transport_owners,
            &inventory,
            &guarded_actuals,
        )?;
    let (definitions, source_only_definitions): (BTreeMap<_, _>, BTreeMap<_, _>) = definitions
        .into_iter()
        .partition(|(owner, _)| transport_owners.contains(owner));
    target_static.retain(|owner, _| source_only_definitions.contains_key(owner));
    let forwards = join_borrowed_forward_uses_v1(&definitions, contracts, &call_sources)
        .map_err(|error| format!("{}: {error:?}", freeze("borrowed-formal/forward-coverage")))?;
    // Project the SAME immutable inventory; retain it for candidate activation.
    let incoming = inventory
        .project(&transport_owners)
        .map_err(|error| format!("{}: {error:?}", freeze("borrowed-formal/incoming-coverage")))?;
    if source_declared_mismatches
        .iter()
        .any(|formal| transport_owners.contains(&formal.owner()))
    {
        return Err(freeze("borrowed-view/declared-object-class"));
    }
    object_views.retain(|formal, _| transport_owners.contains(&formal.owner()));
    let app_main_terminal_sites = observation::app_main_terminal_sites_v1(batch, app_main)?;
    Ok(PreparedBorrowedFormalIngressV1 {
        app_main_terminal_sites,
        app_main_owner: contracts
            .iter()
            .find(|row| app_main.is_some_and(|main| main.matches_contract(row)))
            .map(|row| row.owner),
        definitions,
        source_only_definitions,
        target_static,
        dominated_view_sites,
        forwards,
        incoming,
        source_incoming: inventory,
        object_views,
        integer_agreements,
        checked_static_inputs,
        static_arguments,
        guarded_actuals,
    })
}
