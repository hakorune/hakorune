//! One retained source incoming inventory and its checked selected projection.
//! Pending source rows count as real incoming, never as executable receivers.
use super::*;

/// Participation is retained on the original observation, never in a second scan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum IncomingObservationScopeV1 {
    Both,
    SourceOnly,
    LegacyOnly,
}
impl IncomingObservationScopeV1 {
    fn legacy(self) -> bool {
        self != Self::SourceOnly
    }
    fn source(self) -> bool {
        self != Self::LegacyOnly
    }
}
type IncomingObservationResultV1 = (
    Option<FunctionOwnerIdV1>,
    Result<BorrowedIncomingCallDraftV1, BorrowedIncomingDraftErrorV1>,
);
#[derive(Debug, Clone)]
pub(super) struct IncomingObservationV1 {
    pub(super) scope: IncomingObservationScopeV1,
    pub(super) original: IncomingObservationResultV1,
    pub(super) method_source: Option<std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>>,
}
impl IncomingObservationV1 {
    pub(super) fn new(
        scope: IncomingObservationScopeV1,
        owner: Option<FunctionOwnerIdV1>,
        result: Result<BorrowedIncomingCallDraftV1, BorrowedIncomingDraftErrorV1>,
    ) -> Self {
        Self {
            scope,
            original: (owner, result),
            method_source: None,
        }
    }
}
impl From<IncomingObservationResultV1> for IncomingObservationV1 {
    fn from(original: IncomingObservationResultV1) -> Self {
        Self {
            scope: IncomingObservationScopeV1::Both,
            original,
            method_source: None,
        }
    }
}

/// Immutable receipt of scoped unresolved source obligations, not a second census.
#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct SourceIncomingRefusalsV1(Box<[IncomingObservationV1]>);

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct BorrowedIncomingInventoryV1
{
    pub(super) owners: std::collections::BTreeSet<FunctionOwnerIdV1>,
    pub(super) unsupported_static_spelling: std::collections::BTreeSet<FunctionOwnerIdV1>,
    pub(super) unsupported_static_context: std::collections::BTreeSet<FunctionOwnerIdV1>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) static_observations:
        BTreeMap<OwnedExprSiteV1, static_inventory::StaticIncomingObservationV1>,
    pub(super) observations: Vec<IncomingObservationV1>,
    pub(super) source_owners: std::collections::BTreeSet<FunctionOwnerIdV1>,
}
impl BorrowedIncomingInventoryV1 {
    pub(super) fn legacy_observations(&self) -> impl Iterator<Item = &IncomingObservationResultV1> {
        self.observations
            .iter()
            .filter(|row| row.scope.legacy())
            .map(|row| &row.original)
    }
    fn source_observations(&self) -> impl Iterator<Item = &IncomingObservationResultV1> {
        self.observations
            .iter()
            .filter(|row| row.scope.source())
            .map(|row| &row.original)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn source_refusals_v1(
        &self,
    ) -> SourceIncomingRefusalsV1 {
        SourceIncomingRefusalsV1(self.observations.iter().filter(|row| row.scope.source() && row.original.1.is_err()).cloned().collect())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn corroborates_refusals_v1(
        &self, retained: &SourceIncomingRefusalsV1,
    ) -> bool {
        let mut current = self.observations.iter().filter(|row| row.scope.source() && row.original.1.is_err());
        retained.0.iter().all(|old| current.next().is_some_and(|row|
            old.scope == row.scope && old.original.0 == row.original.0
                && old.original.1.as_ref().err() == row.original.1.as_ref().err()
                && match (&old.method_source, &row.method_source) {
                    (Some(old), Some(row)) => std::sync::Arc::ptr_eq(old, row),
                    (None, None) => true,
                    _ => false,
                })) && current.next().is_none()
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn source_registered_owners_v1(
        &self,
    ) -> &std::collections::BTreeSet<FunctionOwnerIdV1> {
        &self.source_owners
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn source_rows_v1(
        &self,
    ) -> impl Iterator<Item = &BorrowedIncomingCallDraftV1> {
        self.source_observations()
            .filter_map(|(_, row)| row.as_ref().ok())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn source_rows_with_legacy_v1(
        &self,
    ) -> impl Iterator<Item = (&BorrowedIncomingCallDraftV1, bool)> {
        self.observations
            .iter()
            .filter(|row| row.scope.source())
            .filter_map(|row| {
                row.original
                    .1
                    .as_ref()
                    .ok()
                    .map(|source| (source, row.scope.legacy()))
            })
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn sum_vetoed_owners_v1(
        &self,
    ) -> std::collections::BTreeSet<FunctionOwnerIdV1> {
        let mut vetoes = std::collections::BTreeSet::new();
        for (owner, row) in self.source_observations() {
            // Complete-empty source remains an unresolved SUM, not a failed scan.
            if row.is_err() && !matches!(row, Err(BorrowedIncomingDraftErrorV1::NoIncoming(_))) {
                match owner {
                    Some(owner) => {
                        vetoes.insert(*owner);
                    }
                    None => return self.source_owners.clone(),
                }
            }
        }
        vetoes
    }

    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn replace_stored_origin_for_test(
        &mut self,
        site: &OwnedExprSiteV1,
    ) {
        let (_, row) = &mut self
            .observations
            .iter_mut()
            .find(|row| row.original.1.as_ref().is_ok_and(|row| &row.call == site))
            .unwrap()
            .original;
        row.as_mut()
            .unwrap()
            .source
            .reissue_stored_origin_for_test();
    }
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn duplicate_object_row_for_test(
        &mut self,
        site: &OwnedExprSiteV1,
    ) {
        let observation = self
            .observations
            .iter()
            .find(|row| row.original.1.as_ref().is_ok_and(|row| &row.call == site))
            .unwrap()
            .clone();
        self.observations.push(observation);
    }

    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn remove_exact_row_for_test(
        &mut self,
        site: &OwnedExprSiteV1,
    ) {
        self.observations
            .retain(|row| !row.original.1.as_ref().is_ok_and(|row| &row.call == site));
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
        if self.legacy_observations().any(|(owner, row)| {
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
        self.legacy_observations()
            .filter_map(|(_, row)| row.as_ref().ok())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn registered_owners_v1(
        &self,
    ) -> &std::collections::BTreeSet<FunctionOwnerIdV1> {
        &self.owners
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn source_vetoed_owners_v1(
        &self,
    ) -> std::collections::BTreeSet<FunctionOwnerIdV1> {
        self.collect_vetoes_v1(false)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn vetoed_owners(
        &self,
    ) -> std::collections::BTreeSet<FunctionOwnerIdV1> {
        self.collect_vetoes_v1(true)
    }
    fn collect_vetoes_v1(
        &self,
        pending_execution_veto: bool,
    ) -> std::collections::BTreeSet<FunctionOwnerIdV1> {
        let mut vetoes = std::collections::BTreeSet::new();
        for (owner, row) in self.legacy_observations() {
            if row.is_err()
                || (pending_execution_veto && row.as_ref().is_ok_and(|row| row.source.is_pending()))
            {
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
        for observation in self.observations.iter_mut().filter(|row| row.scope.legacy()) {
            let Ok(row) = &mut observation.original.1 else {
                continue;
            };
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

    /// Declaration-only view of the SAME registered completed scan. A clean
    /// NoIncoming sentinel binds a declaration, never executable incoming.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn project_declaration_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Box<[BorrowedIncomingCallDraftV1]>, BorrowedIncomingDraftErrorV1> {
        if !self.owners.contains(&owner) {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        }
        // Preserve every global scan failure before considering an empty owner.
        if !self.project(&Default::default())?.is_empty() {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        }
        let rows: Vec<_> = self
            .legacy_observations()
            .filter(|(label, _)| *label == Some(owner))
            .collect();
        let empty = rows
            .iter()
            .any(|(_, row)| matches!(row, Err(BorrowedIncomingDraftErrorV1::NoIncoming(_))));
        if empty {
            for (_, row) in &rows {
                if let Err(error) = row {
                    if !matches!(error, BorrowedIncomingDraftErrorV1::NoIncoming(_)) {
                        return Err(error.clone());
                    }
                }
            }
            if rows.len() != 1
                || !matches!(&rows[0].1,
                Err(BorrowedIncomingDraftErrorV1::NoIncoming(observed)) if *observed == owner)
            {
                return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
            }
            return Ok(Box::new([]));
        }
        // Lost observations cannot be reconstructed into a clean empty scan.
        if rows.is_empty() {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        }
        self.project(&std::collections::BTreeSet::from([owner]))
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn project(
        &self,
        owners: &std::collections::BTreeSet<FunctionOwnerIdV1>,
    ) -> Result<Box<[BorrowedIncomingCallDraftV1]>, BorrowedIncomingDraftErrorV1> {
        if !owners.is_subset(&self.owners) {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        }
        self.legacy_observations()
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
