//! Exact selected-key to private semantic-batch-slot co-seal.

use std::collections::BTreeSet;

use crate::mir::builder::{
    SelectedNormalCallableKeyV1, VerifiedSourceBackedSameModuleCallableCatalogV1,
};
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;

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
}

#[derive(Debug)]
pub(super) struct VerifiedSelectedCallableBatchMapV1 {
    rows: Box<[SelectedCallableBatchMapRowV1]>,
}

impl VerifiedSelectedCallableBatchMapV1 {
    pub(super) fn batch_slot(&self, key: &SelectedNormalCallableKeyV1) -> Option<u32> {
        self.rows
            .binary_search_by(|row| row.key.cmp(key))
            .ok()
            .map(|index| self.rows[index].batch_slot)
    }

    pub(super) fn contains_batch_slot(&self, batch_slot: u32) -> bool {
        self.rows.iter().any(|row| row.batch_slot == batch_slot)
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

pub(super) fn issue_selected_callable_batch_map_v1(
    catalog: &VerifiedSourceBackedSameModuleCallableCatalogV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
) -> Result<VerifiedSelectedCallableBatchMapV1, SelectedCallableBatchMapIssueV1> {
    let declarations = batch.declarations().collect::<Vec<_>>();
    let mut used_slots = BTreeSet::new();
    let mut rows = Vec::new();
    for (key, identity) in catalog.selected_identities() {
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
        });
    }
    rows.sort_by(|left, right| left.key.cmp(&right.key));
    Ok(VerifiedSelectedCallableBatchMapV1 {
        rows: rows.into_boxed_slice(),
    })
}
