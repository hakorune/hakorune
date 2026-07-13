//! Compile-time BindingRef transaction for one canonical conditional.
//!
//! This box snapshots only the ordered join domain selected by the verified
//! plan. Branch-local rebinding is authorized before mutation, journaled on
//! its first write, and restored without emitting MIR or touching source ledgers.

use std::collections::BTreeSet;

use crate::mir::resolved_region_flow::{ResolvedIfJoinContractV1, ResolvedIfPortValueSourceV1};
use crate::mir::resolved_semantics::{
    BindingRefV1, ScopeId, ScopeKindV1, VerifiedResolvedFunctionV1,
};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ResolvedEffectBindingClassV1 {
    Local,
    Visible,
}

#[derive(Debug)]
enum ActiveResolvedEffectFrameV1 {
    Condition {
        surrounding_scope: ScopeId,
        permits: BTreeSet<BindingRefV1>,
    },
    Branch {
        branch_scope: ScopeId,
        transaction: ResolvedBranchTransactionV1,
    },
}

/// Stack of commit-only condition effects and rollback branch effects.
///
/// Only the top frame observes a direct assignment. Nested-If whole effects
/// are validated against that same frame before the nested condition starts.
#[derive(Debug, Default)]
pub(super) struct ResolvedActiveEffectStackV1 {
    frames: Vec<ActiveResolvedEffectFrameV1>,
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

        let first_old_journal = entry
            .iter()
            .filter(|row| permits.contains(&row.binding))
            .copied()
            .collect();
        Ok(Self {
            entry,
            permits,
            first_old_journal,
            journaled: permitted_rebinds.iter().copied().collect(),
        })
    }

    pub(super) fn permits(&self, binding: BindingRefV1) -> bool {
        self.permits.contains(&binding)
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
        store.branch_rebind_authorized(authorization)
    }

    pub(super) fn capture_and_restore<S: BranchValueStoreV1>(
        &mut self,
        store: &mut S,
    ) -> Result<ResolvedBranchExitValuesV1, String> {
        let primary = self.capture_values(store);
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

    /// Selects every final input from the sealed per-binding source matrix.
    /// Lowering must not infer this choice again from branch effect sets.
    pub(super) fn join_rows_for_contract(
        &self,
        contract: &ResolvedIfJoinContractV1,
        then_values: &ResolvedBranchExitValuesV1,
        else_values: &ResolvedBranchExitValuesV1,
    ) -> Result<Vec<ResolvedJoinValueRowV1>, String> {
        if then_values.values.len() != self.entry.len()
            || else_values.values.len() != self.entry.len()
            || contract.rows().len() != self.entry.len()
        {
            return Err(
                "[freeze:contract][canonical_branch/join_domain_size_mismatch]".to_string(),
            );
        }

        self.entry
            .iter()
            .zip(&then_values.values)
            .zip(&else_values.values)
            .zip(contract.rows())
            .map(|(((entry, then_value), else_value), contract_row)| {
                if entry.binding != then_value.binding
                    || entry.binding != else_value.binding
                    || entry.binding != contract_row.binding()
                {
                    return Err(
                        "[freeze:contract][canonical_branch/join_domain_order_mismatch]"
                            .to_string(),
                    );
                }
                Ok(ResolvedJoinValueRowV1 {
                    binding: entry.binding,
                    entry: entry.value,
                    then_value: select_port_value(
                        contract_row.then_source(),
                        entry.value,
                        then_value.value,
                    ),
                    else_value: select_port_value(
                        contract_row.else_source(),
                        entry.value,
                        else_value.value,
                    ),
                })
            })
            .collect()
    }

    #[cfg(test)]
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

    fn capture_values<S: BranchValueStoreV1>(
        &self,
        store: &S,
    ) -> Result<ResolvedBranchExitValuesV1, String> {
        let mut values = Vec::with_capacity(self.entry.len());
        for entry in &self.entry {
            let value = store.branch_current_value(entry.binding)?;
            if !self.permits.contains(&entry.binding) && value != entry.value {
                return Err(format!(
                    "[freeze:contract][canonical_branch/nonpermit_domain_changed] binding={:?}",
                    entry.binding
                ));
            }
            values.push(JoinEntryValueV1 {
                binding: entry.binding,
                value,
            });
        }
        Ok(ResolvedBranchExitValuesV1 { values })
    }

    fn restore<S: BranchValueStoreV1>(&mut self, store: &mut S) -> Result<(), String> {
        let mut failures = Vec::new();
        while let Some(entry) = self.first_old_journal.pop() {
            let authorization = AuthorizedBranchRebindV1::new(entry.binding, entry.value);
            if let Err(error) = store.branch_rebind_authorized(authorization) {
                failures.push(format!("binding={:?} error={error}", entry.binding));
            }
            self.journaled.remove(&entry.binding);
        }
        if failures.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "[freeze:contract][canonical_branch/restore_failures] {}",
                failures.join(" | ")
            ))
        }
    }
}

fn select_port_value(
    source: ResolvedIfPortValueSourceV1,
    entry: ValueId,
    branch_exit: ValueId,
) -> ValueId {
    match source {
        ResolvedIfPortValueSourceV1::PostConditionEntry => entry,
        ResolvedIfPortValueSourceV1::BranchExit => branch_exit,
    }
}

impl ResolvedActiveEffectStackV1 {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn push_condition(
        &mut self,
        product: &VerifiedResolvedFunctionV1,
        surrounding_scope: ScopeId,
        permits: &[BindingRefV1],
    ) -> Result<(), String> {
        require_scope(product, surrounding_scope)?;
        let permits = exact_permits(permits, "condition")?;
        for binding in &permits {
            if classify_binding(product, surrounding_scope, *binding, false)?
                != ResolvedEffectBindingClassV1::Visible
            {
                return Err(
                    "[freeze:contract][canonical_effect/condition_local_permit]".to_string()
                );
            }
        }
        self.frames.push(ActiveResolvedEffectFrameV1::Condition {
            surrounding_scope,
            permits,
        });
        Ok(())
    }

    pub(super) fn finish_condition(&mut self, expected_scope: ScopeId) -> Result<(), String> {
        match self.frames.last() {
            Some(ActiveResolvedEffectFrameV1::Condition {
                surrounding_scope, ..
            }) if *surrounding_scope == expected_scope => {
                self.frames.pop();
                Ok(())
            }
            _ => Err("[freeze:contract][canonical_effect/condition_pop_mismatch]".to_string()),
        }
    }

    pub(super) fn push_branch(
        &mut self,
        product: &VerifiedResolvedFunctionV1,
        branch_scope: ScopeId,
        transaction: ResolvedBranchTransactionV1,
    ) -> Result<(), String> {
        require_scope(product, branch_scope)?;
        self.frames.push(ActiveResolvedEffectFrameV1::Branch {
            branch_scope,
            transaction,
        });
        Ok(())
    }

    pub(super) fn authorize_current(
        &self,
        product: &VerifiedResolvedFunctionV1,
        binding: BindingRefV1,
    ) -> Result<ResolvedEffectBindingClassV1, String> {
        let frame = self.frames.last().ok_or_else(|| {
            "[freeze:contract][canonical_effect/authorize_without_frame]".to_string()
        })?;
        let (scope, inclusive_local, permitted) = match frame {
            ActiveResolvedEffectFrameV1::Condition {
                surrounding_scope,
                permits,
            } => (*surrounding_scope, false, permits.contains(&binding)),
            ActiveResolvedEffectFrameV1::Branch {
                branch_scope,
                transaction,
            } => (*branch_scope, true, transaction.permits(binding)),
        };
        let class = classify_binding(product, scope, binding, inclusive_local)?;
        if class == ResolvedEffectBindingClassV1::Visible && !permitted {
            return Err(format!(
                "[freeze:contract][canonical_effect/visible_binding_not_permitted] binding={binding:?}"
            ));
        }
        Ok(class)
    }

    pub(super) fn prime_current_effects(
        &self,
        product: &VerifiedResolvedFunctionV1,
        effects: &[BindingRefV1],
    ) -> Result<(), String> {
        if self.frames.is_empty() {
            return Ok(());
        }
        for binding in effects {
            self.authorize_current(product, *binding)?;
        }
        Ok(())
    }

    pub(super) fn rebind_current<S: BranchValueStoreV1>(
        &mut self,
        store: &mut S,
        product: &VerifiedResolvedFunctionV1,
        binding: BindingRefV1,
        value: ValueId,
    ) -> Result<ValueId, String> {
        let class = self.authorize_current(product, binding)?;
        match self
            .frames
            .last_mut()
            .expect("authorization required a frame")
        {
            ActiveResolvedEffectFrameV1::Branch { transaction, .. }
                if class == ResolvedEffectBindingClassV1::Visible =>
            {
                transaction.rebind(store, binding, value)
            }
            _ => store.branch_rebind_authorized(AuthorizedBranchRebindV1::new(binding, value)),
        }
    }

    pub(super) fn capture_branch<S: BranchValueStoreV1>(
        &mut self,
        store: &mut S,
        expected_scope: ScopeId,
    ) -> Result<(ResolvedBranchTransactionV1, ResolvedBranchExitValuesV1), String> {
        let Some(ActiveResolvedEffectFrameV1::Branch { branch_scope, .. }) = self.frames.last()
        else {
            return Err("[freeze:contract][canonical_effect/branch_capture_without_frame]".into());
        };
        if *branch_scope != expected_scope {
            return Err("[freeze:contract][canonical_effect/branch_capture_scope_mismatch]".into());
        }
        let ActiveResolvedEffectFrameV1::Branch {
            mut transaction, ..
        } = self.frames.pop().expect("checked branch frame")
        else {
            unreachable!()
        };
        let values = transaction.capture_and_restore(store)?;
        Ok((transaction, values))
    }

    pub(super) fn restore_branch<S: BranchValueStoreV1>(
        &mut self,
        store: &mut S,
        expected_scope: ScopeId,
    ) -> Result<(), String> {
        let Some(ActiveResolvedEffectFrameV1::Branch { branch_scope, .. }) = self.frames.last()
        else {
            return Err("[freeze:contract][canonical_effect/branch_restore_without_frame]".into());
        };
        if *branch_scope != expected_scope {
            return Err("[freeze:contract][canonical_effect/branch_restore_scope_mismatch]".into());
        }
        let ActiveResolvedEffectFrameV1::Branch {
            mut transaction, ..
        } = self.frames.pop().expect("checked branch frame")
        else {
            unreachable!()
        };
        transaction.restore_error(store)
    }

    pub(super) fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
}

fn exact_permits(permits: &[BindingRefV1], owner: &str) -> Result<BTreeSet<BindingRefV1>, String> {
    let set = permits.iter().copied().collect::<BTreeSet<_>>();
    if set.len() != permits.len() {
        return Err(format!(
            "[freeze:contract][canonical_effect/{owner}_duplicate_permit]"
        ));
    }
    Ok(set)
}

fn require_scope(product: &VerifiedResolvedFunctionV1, scope: ScopeId) -> Result<(), String> {
    product
        .scope(scope)
        .map(|_| ())
        .ok_or_else(|| "[freeze:contract][canonical_effect/missing_or_foreign_scope]".to_string())
}

fn classify_binding(
    product: &VerifiedResolvedFunctionV1,
    frame_scope: ScopeId,
    binding: BindingRefV1,
    inclusive_local: bool,
) -> Result<ResolvedEffectBindingClassV1, String> {
    let owner_scope = product
        .binding(binding)
        .ok_or_else(|| {
            "[freeze:contract][canonical_effect/missing_or_foreign_binding]".to_string()
        })?
        .owner_scope();
    if owner_scope != frame_scope
        && inclusive_local
        && scope_is_ancestor(product, frame_scope, owner_scope)?
    {
        return Ok(ResolvedEffectBindingClassV1::Local);
    }
    if inclusive_local && owner_scope == frame_scope {
        return Ok(ResolvedEffectBindingClassV1::Local);
    }
    if !inclusive_local && proper_block_expr_descendant(product, frame_scope, owner_scope)? {
        return Ok(ResolvedEffectBindingClassV1::Local);
    }
    if scope_is_ancestor(product, owner_scope, frame_scope)? {
        return Ok(ResolvedEffectBindingClassV1::Visible);
    }
    Err("[freeze:contract][canonical_effect/sibling_binding_scope]".to_string())
}

fn proper_block_expr_descendant(
    product: &VerifiedResolvedFunctionV1,
    ancestor: ScopeId,
    mut descendant: ScopeId,
) -> Result<bool, String> {
    if descendant == ancestor {
        return Ok(false);
    }
    loop {
        let record = product.scope(descendant).ok_or_else(|| {
            "[freeze:contract][canonical_effect/missing_scope_in_condition_ancestry]".to_string()
        })?;
        if record.kind() != ScopeKindV1::BlockExpr {
            return Ok(false);
        }
        let Some(parent) = record.parent() else {
            return Ok(false);
        };
        if parent == ancestor {
            return Ok(true);
        }
        descendant = parent;
    }
}

fn scope_is_ancestor(
    product: &VerifiedResolvedFunctionV1,
    ancestor: ScopeId,
    mut descendant: ScopeId,
) -> Result<bool, String> {
    loop {
        if descendant == ancestor {
            return Ok(true);
        }
        let Some(parent) = product
            .scope(descendant)
            .ok_or_else(|| {
                "[freeze:contract][canonical_effect/missing_scope_in_ancestry]".to_string()
            })?
            .parent()
        else {
            return Ok(false);
        };
        descendant = parent;
    }
}
