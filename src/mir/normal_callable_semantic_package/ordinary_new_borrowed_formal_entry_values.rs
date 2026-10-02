//! Physical formal correspondence retained by the original ordinary ledger.
use super::*;
use crate::mir::ValueId;

type EntryValues = Box<[(u32, BindingRefV1, ValueId)]>;

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn record_borrowed_ordinary_entry_values_v1(
        &self,
        owner: FunctionOwnerIdV1,
        values: Result<EntryValues, String>,
    ) -> Result<(), String> {
        self.validate_borrowed_ordinary_entry_values_v1(owner, &values)?;
        let mut recorded = self.borrowed_entry_values.borrow_mut();
        if recorded.contains_key(&owner) {
            return Err(freeze("borrowed-entry/duplicate-entry-values"));
        }
        recorded.insert(owner, values);
        Ok(())
    }

    pub(crate) fn validate_borrowed_ordinary_entry_values_v1(
        &self,
        owner: FunctionOwnerIdV1,
        values: &Result<EntryValues, String>,
    ) -> Result<(), String> {
        if self.borrowed_entry_values.borrow().contains_key(&owner) {
            return Err(freeze("borrowed-entry/duplicate-entry-values"));
        }
        self.check_borrowed_ordinary_entry_values_v1(owner, values)
    }

    // The final source loan checks the same correspondence without repeating
    // the record-only duplicate-state guard.
    pub(super) fn check_borrowed_ordinary_entry_values_v1(
        &self,
        owner: FunctionOwnerIdV1,
        values: &Result<EntryValues, String>,
    ) -> Result<(), String> {
        if let Ok(rows) = values {
            let source = self
                .borrowed_formal_source
                .as_ref()
                .ok_or_else(|| freeze("borrowed-entry/source-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            let definition = source
                .definitions
                .get(&owner)
                .ok_or_else(|| freeze("borrowed-entry/entry-owner"))?;
            let roots: BTreeSet<_> = definition.origins.values().copied().collect();
            if roots != rows.iter().map(|(_, binding, _)| *binding).collect()
                || rows.len() != roots.len()
                || rows.iter().any(|(_, binding, _)| binding.owner() != owner)
                || rows.windows(2).any(|pair| pair[0].0 >= pair[1].0)
                || rows
                    .iter()
                    .map(|(_, _, value)| *value)
                    .collect::<BTreeSet<_>>()
                    .len()
                    != rows.len()
            {
                return Err(freeze("borrowed-entry/entry-values-identity"));
            }
            self.checked_borrowed_entry_incoming(source, owner)?;
            for call in source.incoming.iter().filter(|call| call.callee == owner) {
                if call.arguments.len() != rows.len()
                    || call.arguments.iter().zip(rows.iter()).any(
                        |((ordinal, _, binding), (observed, formal, _))| {
                            ordinal != observed || binding != formal
                        },
                    )
                {
                    return Err(freeze("borrowed-entry/entry-values-source-drift"));
                }
            }
        }
        Ok(())
    }

    /// An actual borrowed consumer must demand this Result. Absence and Err
    /// never mean an untagged ABI, and metadata does not prove correspondence.
    pub(crate) fn borrowed_ordinary_entry_values_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<EntryValues, String> {
        self.borrowed_entry_values
            .borrow()
            .get(&owner)
            .ok_or_else(|| freeze("borrowed-entry/entry-values-missing"))?
            .clone()
    }
}
