//! One borrowed entry view for a global owner or a completed target-static cohort.
//! The target branch borrows the original source-only draft and never mutates
//! the global transport graph.
use super::super::borrowed_formal_uses::{
    BorrowedFormalUsesDraftV1, BorrowedIncomingCallDraftV1, BorrowedIncomingSourceV1,
};
use super::*;

pub(super) struct BorrowedEntryOwnerViewV1<'a> {
    pub(super) definition: &'a BorrowedFormalUsesDraftV1,
    pub(super) incoming: Box<[&'a BorrowedIncomingCallDraftV1]>,
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn checked_entry_owner_view_v1<'a>(
        &'a self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<Option<BorrowedEntryOwnerViewV1<'a>>, String> {
        if let Some(cohort) = source.target_static.get(&owner) {
            if source.definitions.contains_key(&owner) || cohort.owner != owner {
                return Err(freeze("borrowed-entry/target-owner-overlap"));
            }
            let definition = source
                .source_only_definitions
                .get(&owner)
                .ok_or_else(|| freeze("borrowed-entry/target-definition-missing"))?;
            if cohort.incoming.len() != 2 {
                return Err(freeze("borrowed-entry/target-cardinality"));
            }
            for call in &cohort.incoming {
                let BorrowedIncomingSourceV1::Static(original) = &call.source else {
                    return Err(freeze("borrowed-entry/target-source-kind"));
                };
                if call.callee != owner
                    || original.call_site() != &call.call
                    || self
                        .checked_completed_static_one_actuals_v1(original)?
                        .is_none()
                {
                    return Err(freeze("borrowed-entry/target-actuals-unfinished"));
                }
            }
            return Ok(Some(BorrowedEntryOwnerViewV1 {
                definition,
                incoming: cohort.incoming.iter().collect(),
            }));
        }
        if let Some(definition) = source.source_only_definitions.get(&owner) {
            if let Some(incoming) = self.checked_completed_static_scalar_cohort_v1(source, owner)? {
                return Ok(Some(BorrowedEntryOwnerViewV1 { definition, incoming }));
            }
            if let Ok(rows) = source.source_incoming.project(&std::collections::BTreeSet::from([owner])) {
                if let Some(first) = rows.first() {
                    if let BorrowedIncomingSourceV1::Static(original) = &first.source {
                        if let Some(incoming) = self.checked_completed_static_mixed_cohort_v1(source, original)? {
                            return Ok(Some(BorrowedEntryOwnerViewV1 { definition, incoming }));
                        }
                    }
                }
            }
        }
        let Some(definition) = source.definitions.get(&owner) else {
            return Ok(None);
        };
        Ok(Some(BorrowedEntryOwnerViewV1 {
            definition,
            incoming: source
                .incoming
                .iter()
                .filter(|call| call.callee == owner)
                .collect(),
        }))
    }
}
