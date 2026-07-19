//! Disconnected CFG-ready PHI rows for the canonical resolved-If route.
//!
//! This bridge is deliberately route-specific. It can be formed only after
//! `VerifiedIfMergePredecessorsV1` has reverified the completed If CFG, and it
//! carries the exact two logical rows for each resolved join binding. It does
//! not create PHIs, publish type/origin facts, or expose a generic CFG-row
//! constructor.

use std::collections::BTreeSet;

use crate::mir::resolved_semantics::BindingRefV1;
use crate::mir::{BasicBlockId, ValueId};

use super::branch_transaction::ResolvedJoinValueRowV1;
use super::if_materialization::VerifiedIfMergePredecessorsV1;
use super::MirBuilder;

#[derive(Debug)]
pub(super) struct VerifiedResolvedIfCfgReadyJoinRowsV1 {
    predecessors: VerifiedIfMergePredecessorsV1,
    rows: Box<[VerifiedResolvedIfCfgReadyJoinRowV1]>,
}

#[derive(Debug)]
pub(super) struct VerifiedResolvedIfCfgReadyJoinRowV1 {
    binding: BindingRefV1,
    entry: ValueId,
    logical_inputs: [(BasicBlockId, ValueId); 2],
}

impl VerifiedResolvedIfCfgReadyJoinRowsV1 {
    pub(super) fn verify(
        builder: &MirBuilder,
        predecessors: VerifiedIfMergePredecessorsV1,
        rows: &[ResolvedJoinValueRowV1],
    ) -> Result<Self, String> {
        predecessors.reverify(builder)?;
        if builder.function_state.current_block != Some(predecessors.merge()) {
            return Err("[freeze:contract][canonical_if/cfg_ready_outside_merge]".to_string());
        }

        let mut bindings = BTreeSet::new();
        let mut sealed_rows = Vec::with_capacity(rows.len());
        for row in rows {
            if !bindings.insert(row.binding()) {
                return Err(format!(
                    "[freeze:contract][canonical_if/cfg_ready_duplicate_join_row] binding={:?}",
                    row.binding()
                ));
            }
            sealed_rows.push(VerifiedResolvedIfCfgReadyJoinRowV1 {
                binding: row.binding(),
                entry: row.entry(),
                logical_inputs: [
                    (predecessors.then_predecessor(), row.then_value()),
                    (predecessors.else_predecessor(), row.else_value()),
                ],
            });
        }

        Ok(Self {
            predecessors,
            rows: sealed_rows.into_boxed_slice(),
        })
    }

    /// Recheck the route-owned CFG witness immediately before a later
    /// completion consumer is allowed to materialize its final PHIs.
    pub(super) fn reverify(&self, builder: &MirBuilder) -> Result<(), String> {
        self.predecessors.reverify(builder)?;
        if builder.function_state.current_block != Some(self.predecessors.merge()) {
            return Err("[freeze:contract][canonical_if/cfg_ready_outside_merge]".to_string());
        }
        Ok(())
    }

    pub(super) fn rows(&self) -> &[VerifiedResolvedIfCfgReadyJoinRowV1] {
        &self.rows
    }
}

impl VerifiedResolvedIfCfgReadyJoinRowV1 {
    pub(super) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(super) const fn entry(&self) -> ValueId {
        self.entry
    }

    pub(super) const fn logical_inputs(&self) -> &[(BasicBlockId, ValueId); 2] {
        &self.logical_inputs
    }
}
