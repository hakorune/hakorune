//! One original incoming inventory, then its checked selected projection.
use super::*;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct BorrowedIncomingInventoryV1
{
    pub(super) owners: std::collections::BTreeSet<FunctionOwnerIdV1>,
    pub(super) unsupported_static_spelling: std::collections::BTreeSet<FunctionOwnerIdV1>,
    pub(super) unsupported_static_context: std::collections::BTreeSet<FunctionOwnerIdV1>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) static_observations:
        BTreeMap<OwnedExprSiteV1, static_inventory::StaticIncomingObservationV1>,
    pub(super) observations: Vec<(
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
        target: &super::super::super::LexicalInstanceCallSourceTargetV1,
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
    pub(super) fn corroborate_scan_completeness(
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
            if row.is_err() || row.as_ref().is_ok_and(|row| row.source.is_pending()) {
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

    /// Bind original Stored source observations after the existing eligibility
    /// and receiver issuer. This visits retained rows only, never source again.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn finish_selected_calls_v1(
        &mut self,
        calls: &BTreeMap<OwnedExprSiteV1, &super::super::super::LexicalInstanceCallSourceTargetV1>,
    ) -> Result<(), BorrowedIncomingDraftErrorV1> {
        let mut seen = std::collections::BTreeSet::new();
        for (_, observation) in &mut self.observations {
            let Ok(row) = observation else { continue };
            if !seen.insert(row.call.clone()) {
                return Err(BorrowedIncomingDraftErrorV1::CallIdentity(row.call.clone()));
            }
            if matches!(row.source, BorrowedIncomingSourceV1::Static(_)) {
                continue;
            }
            let Some(target) = calls.get(&row.call) else {
                // Keep the original Pending row and all ordered source args.
                // Selected projection rejects it; absence is never NoIncoming.
                if !row.source.is_pending() {
                    return Err(BorrowedIncomingDraftErrorV1::CallIdentity(row.call.clone()));
                }
                continue;
            };
            row.source = row
                .source
                .bind_selected_v1(target)
                .map_err(|_| BorrowedIncomingDraftErrorV1::CallIdentity(row.call.clone()))?;
        }
        Ok(())
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
            .map(|(_, row)| {
                let row = row.as_ref().map_err(Clone::clone)?;
                if row.source.is_pending() {
                    return Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(
                        row.call.clone(),
                    ));
                }
                Ok(row.clone())
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Vec::into_boxed_slice)
    }
}
