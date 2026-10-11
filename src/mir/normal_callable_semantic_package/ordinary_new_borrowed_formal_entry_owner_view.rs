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
            if let Some(incoming) = self.checked_completed_static_bool_entry_v1(source, owner)? {
                return Ok(Some(BorrowedEntryOwnerViewV1 { definition, incoming }));
            }
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

impl BorrowedOrdinaryEntrySourceRefV1<'_> {
    /// Loan the original target, including its source owner and batch identity.
    pub(crate) fn incoming_targets(
        &self,
    ) -> impl Iterator<Item = Result<super::borrowed_formal_uses::BorrowedCallSourceLoanV1<'_>, String>>
    {
        self.incoming_rows
            .iter()
            .filter(move |row| row.callee == self.owner)
            .map(|row| {
                let loan = row.source.as_loan();
                if loan.call_site() != &row.call
                    || loan.callee_owner() != row.callee
                    || row.callee != self.owner
                {
                    return Err(freeze("borrowed-entry/incoming-source-identity"));
                }
                if let super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                    &row.source
                {
                    if !original.is_qualified()
                        && !((original.is_current_owner_i64_source_v1()
                            || (original.is_current_owner_bool_source_v1()
                                && self.source.source_only_definitions.contains_key(&self.owner)))
                            && self.incoming.iter().any(|(site, actuals)| {
                                *site == &row.call && actuals.len() == 1
                            }))
                    {
                        original.require_qualified()?;
                    }
                    let retained = self
                        .source
                        .source_incoming
                        .static_observations()
                        .get(&row.call)
                        .ok_or_else(|| freeze("borrowed-entry/incoming-source-missing"))?
                        .as_ref()
                        .map_err(Clone::clone)?;
                    if !Rc::ptr_eq(retained, original) {
                        return Err(freeze("borrowed-entry/incoming-source-drift"));
                    }
                }
                Ok(row.source.as_loan())
            })
    }
}
