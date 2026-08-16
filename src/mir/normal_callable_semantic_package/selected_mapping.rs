//! Exact selected-key to private semantic-batch-slot co-seal.

use std::collections::BTreeSet;

use crate::mir::builder::{
    SelectedCallableConsumptionRoleV1, SelectedNormalCallableKeyV1,
    VerifiedSourceBackedSameModuleCallableCatalogV1,
};
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::parser::CallableDeclarationIdentityV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SelectedCallableBatchMapIssueV1 {
    MissingBatchRow,
    DuplicateBatchIdentity,
    DuplicateBatchSlot,
}

#[derive(Debug)]
struct SelectedCallableBatchMapRowV1 {
    key: SelectedNormalCallableKeyV1,
    batch_slot: u32,
    identity: CallableDeclarationIdentityV1,
    role: SelectedCallableConsumptionRoleV1,
}

#[derive(Debug)]
pub(super) struct VerifiedSelectedCallableBatchMapV1 {
    rows: Box<[SelectedCallableBatchMapRowV1]>,
}

impl VerifiedSelectedCallableBatchMapV1 {
    pub(super) fn main_static_child_rows(
        &self,
    ) -> impl Iterator<Item = SelectedCallableBatchMapRowRefV1<'_>> {
        self.rows.iter().filter_map(|row| {
            row.role
                .is_main_static_child()
                .then_some(SelectedCallableBatchMapRowRefV1 { row })
        })
    }

    pub(super) fn batch_slot(&self, key: &SelectedNormalCallableKeyV1) -> Option<u32> {
        self.rows
            .binary_search_by(|row| row.key.cmp(key))
            .ok()
            .map(|index| self.rows[index].batch_slot)
    }

    pub(super) fn is_main_child_key(&self, key: &SelectedNormalCallableKeyV1) -> bool {
        self.rows
            .binary_search_by(|row| row.key.cmp(key))
            .ok()
            .is_some_and(|index| self.rows[index].role.is_main_static_child())
    }

    pub(super) fn contains_batch_slot(&self, batch_slot: u32) -> bool {
        self.rows.iter().any(|row| row.batch_slot == batch_slot)
    }

    pub(super) fn dynamic_eligible_batch_slot(&self, batch_slot: u32) -> bool {
        self.rows
            .iter()
            .find(|row| row.batch_slot == batch_slot)
            .is_some_and(|row| row.role.admits_dynamic())
    }

    pub(super) fn role_for_batch_slot(
        &self,
        batch_slot: u32,
    ) -> Option<SelectedCallableConsumptionRoleV1> {
        self.rows
            .iter()
            .find(|row| row.batch_slot == batch_slot)
            .map(|row| row.role)
    }

    pub(super) fn identity_for_batch_slot(
        &self,
        batch_slot: u32,
    ) -> Option<&CallableDeclarationIdentityV1> {
        self.rows
            .iter()
            .find(|row| row.batch_slot == batch_slot)
            .map(|row| &row.identity)
    }

    pub(super) fn main_child_selection(
        &self,
        statement: u32,
        method: crate::ast::BoxMethodInventoryOrdinalV1,
    ) -> Option<(
        &SelectedNormalCallableKeyV1,
        &CallableDeclarationIdentityV1,
        SelectedCallableConsumptionRoleV1,
    )> {
        self.rows.iter().find_map(|row| {
            (row.role.main_static_child_slot() == Some((statement, method))).then_some((
                &row.key,
                &row.identity,
                row.role,
            ))
        })
    }

    pub(super) fn key_for_batch_slot(
        &self,
        batch_slot: u32,
    ) -> Option<&SelectedNormalCallableKeyV1> {
        self.rows
            .iter()
            .find(|row| row.batch_slot == batch_slot)
            .map(|row| &row.key)
    }

    pub(super) fn keys(&self) -> impl ExactSizeIterator<Item = &SelectedNormalCallableKeyV1> {
        self.rows.iter().map(|row| &row.key)
    }
}

#[derive(Clone, Copy)]
pub(super) struct SelectedCallableBatchMapRowRefV1<'a> {
    row: &'a SelectedCallableBatchMapRowV1,
}

impl SelectedCallableBatchMapRowRefV1<'_> {
    pub(super) const fn batch_slot(self) -> u32 {
        self.row.batch_slot
    }

    pub(super) fn identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.row.identity
    }

    pub(super) const fn role(self) -> SelectedCallableConsumptionRoleV1 {
        self.row.role
    }
}

pub(super) fn issue_selected_callable_batch_map_v1(
    catalog: &VerifiedSourceBackedSameModuleCallableCatalogV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
) -> Result<VerifiedSelectedCallableBatchMapV1, SelectedCallableBatchMapIssueV1> {
    let declarations = batch.declarations().collect::<Vec<_>>();
    let mut used_slots = BTreeSet::new();
    let mut rows = Vec::new();
    for (key, identity, role) in catalog.selected_identities() {
        let mut matches = declarations
            .iter()
            .copied()
            .filter(|row| row.same_declaration_identity(identity));
        let Some(matched) = matches.next() else {
            return Err(SelectedCallableBatchMapIssueV1::MissingBatchRow);
        };
        if matches.next().is_some() {
            return Err(SelectedCallableBatchMapIssueV1::DuplicateBatchIdentity);
        }
        let batch_slot = matched.batch_slot();
        if !used_slots.insert(batch_slot) {
            return Err(SelectedCallableBatchMapIssueV1::DuplicateBatchSlot);
        }
        rows.push(SelectedCallableBatchMapRowV1 {
            key: key.clone(),
            batch_slot,
            identity: identity.clone(),
            role,
        });
    }
    rows.sort_by(|left, right| left.key.cmp(&right.key));
    Ok(VerifiedSelectedCallableBatchMapV1 {
        rows: rows.into_boxed_slice(),
    })
}
