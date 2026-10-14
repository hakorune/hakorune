//! Sole original actual classifier product; projections never revisit source.
//! Source Integer evidence and selected transport share the same immutable rows.
use super::*;

#[derive(Debug)]
struct OriginalCallSeedsV1 {
    incoming: BorrowedIncomingCallDraftV1,
    seeds: Box<[OriginalFormalActualSeedV1]>,
}

#[derive(Debug)]
pub(super) struct OriginalFormalSeedProductV1 {
    rows: Box<[OriginalCallSeedsV1]>,
    owners: BTreeSet<FunctionOwnerIdV1>,
    source_vetoes: BTreeSet<FunctionOwnerIdV1>,
    definition_owners: BTreeSet<FunctionOwnerIdV1>,
    source_integers: BTreeSet<BindingRefV1>,
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) struct OriginalBorrowedFormalSourceV1 {
    pub(super) inventory: BorrowedIncomingInventoryV1,
    pub(super) seeds: OriginalFormalSeedProductV1,
}

#[derive(Clone, Copy)]
enum FormalDomainProjectionV1<'a> {
    SourceInteger,
    SelectedTransport {
        owners: &'a BTreeSet<FunctionOwnerIdV1>,
    },
}

/// Actual sites are classified once, before result/receiver eligibility.
/// PendingStored retains its complete original ordered input without a loan.
#[allow(clippy::too_many_arguments)]
pub(super) fn prepare_original_formal_seeds_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    results: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    candidates: &BTreeMap<u32, Result<Vec<super::super::super::candidate::OrdinaryNewCandidate>, super::super::super::OrdinaryNewCoSealIssueV1>>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    inventory: &BorrowedIncomingInventoryV1,
    guarded: &BTreeMap<(OwnedExprSiteV1, u32), BorrowedGuardedActualV1>,
) -> Result<OriginalFormalSeedProductV1, String> {
    let slots: BTreeMap<_, _> = batch.declarations()
        .map(|row| (row.owner(), row.batch_slot())).collect();
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    for call in inventory.exact_rows() {
        if !seen.insert(call.call.clone()) {
            return Err(freeze("borrowed-view/duplicate-original-call"));
        }
        let caller = call.call.owner();
        let slot = *slots.get(&caller).ok_or_else(|| freeze("borrowed-view/caller-slot"))?;
        let candidates = candidates.get(&slot).and_then(|rows| rows.as_ref().ok())
            .map(Vec::as_slice).unwrap_or(&[]);
        let receiver = contracts.iter().find(|row| row.owner == caller)
            .filter(|row| row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod)
            .and_then(|_| crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
                selected, batch, entry_home_loans.for_batch_slot(slot), slot,
            ));
        let seeds = batch.with_lowering_input(slot, |input| {
            call.arguments.iter().map(|(ordinal, site, _)| {
                classify_original_actual_seed_in_scope(
                    input, batch, selected, results, caller, candidates, receiver,
                    definitions, contracts, site,
                    guarded.get(&(call.call.clone(), *ordinal))
                        .map(|fact| (fact, &call.call, *ordinal)),
                )
            }).collect::<Vec<_>>()
        }).map_err(|_| freeze("borrowed-view/caller-loan"))?;
        rows.push(OriginalCallSeedsV1 { incoming: call.clone(), seeds: seeds.into_boxed_slice() });
    }
    let mut product = OriginalFormalSeedProductV1 {
        rows: rows.into_boxed_slice(),
        owners: inventory.registered_owners_v1().clone(),
        source_vetoes: inventory.source_vetoed_owners_v1(),
        definition_owners: definitions.keys().copied().collect(),
        source_integers: BTreeSet::new(),
    };
    product.source_integers = product.project_v1(
        batch, constructors, contracts, inventory,
        FormalDomainProjectionV1::SourceInteger,
    )?.1;
    Ok(product)
}

impl OriginalFormalSeedProductV1 {
    /// Binding selected Stored targets changes no original row/ordered actual.
    /// Equal but newly issued source Rcs, loss and additional rows stay refused.
    fn corroborate_original_v1(&self, inventory: &BorrowedIncomingInventoryV1) -> bool {
        if inventory.registered_owners_v1() != &self.owners
            || inventory.source_vetoed_owners_v1() != self.source_vetoes
            || inventory.exact_rows().count() != self.rows.len()
        { return false; }
        let mut current = BTreeMap::new();
        for call in inventory.exact_rows() {
            if current.insert(call.call.clone(), call).is_some() { return false; }
        }
        self.rows.iter().all(|row| {
            let Some(call) = current.get(&row.incoming.call) else { return false; };
            call.callee == row.incoming.callee
                && call.arguments == row.incoming.arguments
                && call.source.same_original_source_v1(&row.incoming.source)
                && row.seeds.len() == call.arguments.len()
        })
    }

    pub(super) fn candidate_integer_agreement_v1(
        &self, inventory: &BorrowedIncomingInventoryV1, formal: BindingRefV1,
    ) -> bool {
        self.corroborate_original_v1(inventory) && self.source_integers.contains(&formal)
    }

    pub(super) fn selected_views_v1(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        owners: &BTreeSet<FunctionOwnerIdV1>,
        inventory: &BorrowedIncomingInventoryV1,
    ) -> Result<(BTreeMap<BindingRefV1, BorrowedFormalObjectViewV1>, BTreeSet<BindingRefV1>, BTreeSet<BindingRefV1>), String> {
        self.project_v1(batch, constructors, contracts, inventory,
            FormalDomainProjectionV1::SelectedTransport { owners })
    }

    fn project_v1(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        inventory: &BorrowedIncomingInventoryV1,
        projection: FormalDomainProjectionV1<'_>,
    ) -> Result<(BTreeMap<BindingRefV1, BorrowedFormalObjectViewV1>, BTreeSet<BindingRefV1>, BTreeSet<BindingRefV1>), String> {
        if !self.corroborate_original_v1(inventory) {
            return Err(freeze("borrowed-view/original-seed-identity"));
        }
        let vetoes = match projection {
            FormalDomainProjectionV1::SourceInteger => inventory.source_vetoed_owners_v1(),
            FormalDomainProjectionV1::SelectedTransport { .. } => inventory.vetoed_owners(),
        };
        let mut seeds: BTreeMap<BindingRefV1, Vec<FormalActualSeedV1>> = BTreeMap::new();
        let mut declared = BTreeSet::new();
        for contract in contracts.iter().filter(|row| self.definition_owners.contains(&row.owner)) {
            for formal in &contract.parameters {
                if formal.kind.is_ordinary_borrowed_handle() && vetoes.contains(&contract.owner)
                { seeds.entry(formal.binding).or_default().push(FormalActualSeedV1::Conflict); }
                if let CallableParameterContractKindV1::DeclaredObject(class) = &formal.kind {
                    declared.insert(formal.binding);
                    seeds.entry(formal.binding).or_default().push(FormalActualSeedV1::Class(class.clone()));
                }
            }
        }
        for row in &self.rows {
            let caller = row.incoming.call.owner();
            for ((_, _site, formal), raw) in row.incoming.arguments.iter().zip(row.seeds.iter()) {
                let seed = match projection {
                    FormalDomainProjectionV1::SourceInteger => raw.project_source_integer_v1(),
                    FormalDomainProjectionV1::SelectedTransport { owners } => raw.project_selected_v1(caller, Some(owners)),
                };
                seeds.entry(*formal).or_default().push(seed);
            }
        }
        let (mut objects, integers) = value_domain::resolve_formal_domains_v1(&seeds, batch, constructors);
        let mut mismatches = BTreeSet::new();
        for formal in &declared {
            if let Some(view) = objects.get_mut(formal) { view.declared = true; }
            else { mismatches.insert(*formal); }
        }
        // The early projection lends Integer only; no Object source permission.
        if matches!(projection, FormalDomainProjectionV1::SourceInteger) { objects.clear(); }
        Ok((objects, integers, mismatches))
    }
}
