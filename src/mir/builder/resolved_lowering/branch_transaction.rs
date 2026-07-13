//! Compile-time BindingRef transaction for one canonical conditional.
//!
//! This box snapshots only the ordered join domain selected by the verified
//! plan. Branch-local rebinding is authorized before mutation, journaled on
//! its first write, and restored without emitting MIR or touching source ledgers.

use std::collections::BTreeSet;

use crate::mir::resolved_semantics::BindingRefV1;
use crate::mir::ValueId;

pub(super) trait BranchValueStoreV1 {
    fn branch_current_value(&self, binding: BindingRefV1) -> Result<ValueId, String>;

    fn branch_rebind_authorized(
        &mut self,
        authorization: AuthorizedBranchRebindV1,
    ) -> Result<ValueId, String>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct AuthorizedBranchRebindV1 {
    binding: BindingRefV1,
    value: ValueId,
}

impl AuthorizedBranchRebindV1 {
    pub(super) const fn binding(self) -> BindingRefV1 {
        self.binding
    }

    pub(super) const fn value(self) -> ValueId {
        self.value
    }

    fn new(binding: BindingRefV1, value: ValueId) -> Self {
        Self { binding, value }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JoinEntryValueV1 {
    binding: BindingRefV1,
    value: ValueId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ResolvedBranchExitValuesV1 {
    values: Vec<JoinEntryValueV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ResolvedJoinValueRowV1 {
    binding: BindingRefV1,
    entry: ValueId,
    then_value: ValueId,
    else_value: ValueId,
}

impl ResolvedJoinValueRowV1 {
    pub(super) const fn binding(self) -> BindingRefV1 {
        self.binding
    }

    pub(super) const fn entry(self) -> ValueId {
        self.entry
    }

    pub(super) const fn then_value(self) -> ValueId {
        self.then_value
    }

    pub(super) const fn else_value(self) -> ValueId {
        self.else_value
    }
}

#[derive(Debug)]
pub(super) struct ResolvedBranchTransactionV1 {
    entry: Vec<JoinEntryValueV1>,
    permits: BTreeSet<BindingRefV1>,
    first_old_journal: Vec<JoinEntryValueV1>,
    journaled: BTreeSet<BindingRefV1>,
}

impl ResolvedBranchTransactionV1 {
    pub(super) fn snapshot<S: BranchValueStoreV1>(
        store: &S,
        ordered_join_domain: &[BindingRefV1],
        permitted_rebinds: &[BindingRefV1],
    ) -> Result<Self, String> {
        let mut domain = BTreeSet::new();
        let mut entry = Vec::with_capacity(ordered_join_domain.len());
        for binding in ordered_join_domain {
            if !domain.insert(*binding) {
                return Err(format!(
                    "[freeze:contract][canonical_branch/duplicate_join_binding] binding={binding:?}"
                ));
            }
            entry.push(JoinEntryValueV1 {
                binding: *binding,
                value: store.branch_current_value(*binding)?,
            });
        }

        let permits = permitted_rebinds.iter().copied().collect::<BTreeSet<_>>();
        if permits.len() != permitted_rebinds.len() {
            return Err("[freeze:contract][canonical_branch/duplicate_rebind_permit]".to_string());
        }
        if !permits.is_subset(&domain) {
            return Err(
                "[freeze:contract][canonical_branch/rebind_permit_outside_join_domain]".to_string(),
            );
        }

        Ok(Self {
            entry,
            permits,
            first_old_journal: Vec::new(),
            journaled: BTreeSet::new(),
        })
    }

    pub(super) fn rebind<S: BranchValueStoreV1>(
        &mut self,
        store: &mut S,
        binding: BindingRefV1,
        value: ValueId,
    ) -> Result<ValueId, String> {
        if !self.permits.contains(&binding) {
            return Err(format!(
                "[freeze:contract][canonical_branch/rebind_not_authorized] binding={binding:?}"
            ));
        }

        let authorization = AuthorizedBranchRebindV1::new(binding, value);
        let old = store.branch_rebind_authorized(authorization)?;
        if self.journaled.insert(binding) {
            self.first_old_journal.push(JoinEntryValueV1 {
                binding,
                value: old,
            });
        }
        Ok(old)
    }

    pub(super) fn capture_and_restore<S: BranchValueStoreV1>(
        &mut self,
        store: &mut S,
    ) -> Result<ResolvedBranchExitValuesV1, String> {
        let primary = self
            .entry
            .iter()
            .map(|entry| {
                Ok(JoinEntryValueV1 {
                    binding: entry.binding,
                    value: store.branch_current_value(entry.binding)?,
                })
            })
            .collect::<Result<Vec<_>, String>>()
            .map(|values| ResolvedBranchExitValuesV1 { values });
        let cleanup = self.restore(store);
        match (primary, cleanup) {
            (Ok(values), Ok(())) => Ok(values),
            (Err(primary), Ok(())) => Err(primary),
            (Ok(_), Err(cleanup)) => Err(cleanup),
            (Err(primary), Err(cleanup)) => Err(format!(
                "[freeze:contract][canonical_branch/during_cleanup] primary={primary} cleanup={cleanup}"
            )),
        }
    }

    pub(super) fn restore_error<S: BranchValueStoreV1>(
        &mut self,
        store: &mut S,
    ) -> Result<(), String> {
        self.restore(store)
    }

    pub(super) fn implicit_false_values(&self) -> ResolvedBranchExitValuesV1 {
        ResolvedBranchExitValuesV1 {
            values: self.entry.clone(),
        }
    }

    pub(super) fn join_rows(
        &self,
        then_values: &ResolvedBranchExitValuesV1,
        else_values: &ResolvedBranchExitValuesV1,
    ) -> Result<Vec<ResolvedJoinValueRowV1>, String> {
        if then_values.values.len() != self.entry.len()
            || else_values.values.len() != self.entry.len()
        {
            return Err(
                "[freeze:contract][canonical_branch/join_domain_size_mismatch]".to_string(),
            );
        }

        self.entry
            .iter()
            .zip(&then_values.values)
            .zip(&else_values.values)
            .map(|((entry, then_value), else_value)| {
                if entry.binding != then_value.binding || entry.binding != else_value.binding {
                    return Err(
                        "[freeze:contract][canonical_branch/join_domain_order_mismatch]"
                            .to_string(),
                    );
                }
                Ok(ResolvedJoinValueRowV1 {
                    binding: entry.binding,
                    entry: entry.value,
                    then_value: then_value.value,
                    else_value: else_value.value,
                })
            })
            .collect()
    }

    fn restore<S: BranchValueStoreV1>(&mut self, store: &mut S) -> Result<(), String> {
        while let Some(entry) = self.first_old_journal.last().copied() {
            let authorization = AuthorizedBranchRebindV1::new(entry.binding, entry.value);
            store.branch_rebind_authorized(authorization)?;
            self.first_old_journal.pop();
            self.journaled.remove(&entry.binding);
        }
        Ok(())
    }
}
