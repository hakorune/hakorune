//! Prepare the original incoming inventory once, then select transport from it.
//! No source rescan or provisional executable Stored receiver occurs at finish.
use super::*;

#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn prepare_original_incoming_v1(
    brands: Option<&crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    omitted: &BTreeMap<FunctionOwnerIdV1, crate::mir::builder::CanonicalSameModuleCallableKeyV1>,
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
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    local_candidates: &BTreeMap<u32, Result<Vec<super::super::super::candidate::OrdinaryNewCandidate>, super::super::super::OrdinaryNewCoSealIssueV1>>,
    callable_result_classes: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    guarded_actuals: &BTreeMap<(OwnedExprSiteV1, u32), BorrowedGuardedActualV1>,
    array_i64_fields: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    array_field_push_candidates: &[super::super::super::array_i64_fields::ArrayFieldPushCandidateV1],
    array_field_get_candidates: &[super::super::super::array_i64_fields::ArrayFieldGetCandidateV1],
    fields: Option<&crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::OrdinaryNewFieldWriteClaimsV1>,
) -> Result<source_seeds::OriginalBorrowedFormalSourceV1, String> {
    let context = static_claims.map(|claims| StaticIncomingContextV1 {
        claims,
        arguments: static_arguments,
        main: app_main,
    });
    // Borrow original argument sites from both retained lexical and Stored
    // references. Only original borrowed formal ordinals select this family.
    let mut arguments = BTreeSet::new();
    let mut retain_arguments = |reference: super::super::source::CallTargetReferenceV1| -> Result<(), String> {
        let mut matches = contracts.iter().filter(|row| row.owner == reference.callee_owner);
        let contract = matches.next().ok_or_else(|| freeze("array-source/contract-missing"))?;
        if matches.next().is_some() || contract.batch_slot != reference.target_batch_slot
            || contract.parameters.len() != reference.argument_sites.len() {
            return Err(freeze("array-source/contract-identity"));
        }
        for (site, formal) in reference.argument_sites.iter().zip(&contract.parameters) {
            if formal.kind.is_ordinary_borrowed_handle() {
                arguments.insert(OwnedExprSiteV1::new(reference.call_site.owner(), site.clone()));
            }
        }
        Ok(())
    };
    for target in calls.values() {
        retain_arguments(super::super::source::CallTargetReferenceV1::from_target(target))?;
    }
    if let Some(prepared) = stored_dispatch {
        for need in prepared.as_ref().map_err(Clone::clone)? {
            if let Some(need) = need.as_ref().map_err(Clone::clone)? {
                if need.stored().is_some() { retain_arguments(need.reference())?; }
            }
        }
    }
    let array_sources = array_source::collect_array_structure_sources_v1(brands, batch, constructors, arguments)?;
    let inventory = inventory_borrowed_incoming_with_stored_dispatch_v1(
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
        Some(&array_sources),
    )
    .map_err(|error| format!("{}: {error:?}", freeze("borrowed-formal/incoming-coverage")))?;
    let seeds = source_seeds::prepare_original_formal_seeds_v1(
        batch, selected, entry_home_loans, constructors, callable_result_classes,
        local_candidates, contracts, definitions, &inventory, guarded_actuals, static_claims, array_i64_fields,
        array_field_push_candidates, array_field_get_candidates,
        app_main.map(|main| main.caller_source_ref_v1()), omitted, fields,
    )?;
    Ok(source_seeds::OriginalBorrowedFormalSourceV1 { inventory, seeds, array_sources })
}

/// Consume the original drafts once; never reconstruct them after profile selection.
#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn finish_ingress_from_drafts_v1(
    brands: Option<&crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1>,
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
        BTreeMap<FunctionOwnerIdV1, Rc<OriginalFormalUseSourceV1>>,
    ),
    static_arguments: BTreeMap<
        (OwnedExprSiteV1, u32),
        super::borrowed_static_argument::StaticArgumentSourceV1,
    >,
    original_inventory: Result<source_seeds::OriginalBorrowedFormalSourceV1, String>,
) -> Result<PreparedBorrowedFormalIngressV1, String> {
    let (_ordinary_callers, definitions, dominated_view_sites, guarded_actuals, originals) = drafts;
    let mut transport_owners: BTreeSet<_> = contracts
        .iter()
        .filter(|row| {
            row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod
                && definitions.get(&row.owner).is_some_and(|draft| !draft.uses.iter().any(|use_|
                    matches!(use_.kind, BorrowedFormalUseDraftKindV1::ConstructionChildArgument { .. }
                        | BorrowedFormalUseDraftKindV1::BorrowedFormalReturn { .. }
                        | BorrowedFormalUseDraftKindV1::FieldWriteReceiver { .. }
                        | BorrowedFormalUseDraftKindV1::FieldWriteValue { .. }
                        | BorrowedFormalUseDraftKindV1::ArrayElementFieldIndexValue { .. })))
        })
        .map(|row| row.owner)
        .collect();
    let source_seeds::OriginalBorrowedFormalSourceV1 { mut inventory, seeds: mut original_formal_seeds, mut array_sources } = original_inventory?;
    // Join the source products once before exposing the prepared ingress.
    original_formal_seeds.original_uses = Some(originals);
    inventory
        .finish_selected_calls_v1(&calls)
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
    let original_arguments = inventory.exact_rows().flat_map(|row|
        row.arguments.iter().map(move |(_, site, _)| OwnedExprSiteV1::new(row.call.owner(), site.clone()))
    ).collect();
    array_source::extend_array_structure_sources_v1(&mut array_sources, brands, batch, instance_constructors, original_arguments)?;
    let array_sources = array_source::qualify_array_sources_v1(
        array_sources, batch, instance_constructors, selected, callable_result_classes, &calls,
    )?;
    let (object_views, integer_agreements, source_declared_mismatches) =
        original_formal_seeds.selected_views_v1(
            &array_sources, batch, instance_constructors, contracts,
            &transport_owners, &inventory,
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
    let static_terminal_sites = observation::static_terminal_sites_v1(batch, selected, contracts, app_main)?;
    Ok(PreparedBorrowedFormalIngressV1 {
        static_terminal_sites,
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
        array_sources,
        source_declared_mismatches,
        integer_agreements,
        original_formal_seeds,
        checked_static_inputs,
        static_arguments,
        guarded_actuals,
    })
}
