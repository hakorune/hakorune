//! Join selected source formals to values allocated by the existing entry.
use super::*;

type PreparedEntryValues = Option<Result<Box<[(u32, BindingRefV1, ValueId)]>, String>>;

impl CallableSemanticLoweringState {
    pub(in crate::mir::builder) fn stage_borrowed_entry_formals(
        &mut self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        formals: Result<Option<Box<[(u32, BindingRefV1)]>>, String>,
    ) -> Result<(), String> {
        if owner != self.owner {
            return Err(freeze("borrowed-entry/source-owner-drift"));
        }
        if self.entry_installed || self.borrowed_entry_formals.is_some() {
            return Err(freeze("borrowed-entry/duplicate-source-preparation"));
        }
        self.borrowed_entry_formals = Some(formals);
        Ok(())
    }

    pub(super) fn prepare_borrowed_entry_values(
        &self,
        entry: &PreparedCallableEntryValuesV1,
    ) -> Result<PreparedEntryValues, String> {
        let formals = match &self.borrowed_entry_formals {
            None | Some(Ok(None)) => return Ok(None),
            Some(Err(issue)) => return Ok(Some(Err(issue.clone()))),
            Some(Ok(Some(formals))) => formals,
        };
        if self.ordinary_new_claim_ledger.is_none() {
            return Err(freeze("borrowed-entry/source-ledger-missing"));
        }
        if self.receiver.is_none() || entry.receiver().is_none() || formals.is_empty() {
            return Err(freeze("borrowed-entry/instance-shape"));
        }
        let mut seen = BTreeSet::new();
        let mut values = BTreeSet::new();
        let mut rows = Vec::new();
        for (ordinal, binding) in formals.iter() {
            let index = *ordinal as usize;
            if binding.owner() != self.owner
                || self.parameters.get(index) != Some(binding)
                || !seen.insert(*ordinal)
            {
                return Err(freeze("borrowed-entry/formal-value-identity"));
            }
            let value = entry
                .parameters()
                .get(index)
                .copied()
                .ok_or_else(|| freeze("borrowed-entry/formal-value-missing"))?;
            if Some(value) == entry.receiver()
                || entry
                    .parameters()
                    .iter()
                    .filter(|candidate| **candidate == value)
                    .count()
                    != 1
                || !values.insert(value)
            {
                return Err(freeze("borrowed-entry/formal-value-collision"));
            }
            rows.push((*ordinal, *binding, value));
        }
        let result = Ok(rows.into_boxed_slice());
        Ok(Some(result))
    }
}

#[cfg(test)]
#[path = "borrowed_entry_tests.rs"]
mod tests;
