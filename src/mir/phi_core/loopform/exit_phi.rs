//! Phase 191: LoopForm Exit PHI Builder
//!
//! Responsibility: Exit PHI generation (break/continue merge handling)
//! - Exit block value merging
//! - Pinned/Carrier variable classification
//! - Collaboration with loop_snapshot_merge.rs
//!
//! This module extracts the exit PHI building logic from loopform_builder.rs
//! to improve modularity and testability.

use crate::mir::control_form::ControlForm;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

/// Exit PHI builder (currently a delegation wrapper)
///
/// Future: Will contain the extracted build_exit_phis logic
pub struct ExitPhiBuilder;

impl ExitPhiBuilder {
    /// Build exit PHIs using LoopFormBuilder delegation
    ///
    /// This is a temporary implementation that delegates to the existing
    /// build_exit_phis method in LoopFormBuilder. Future refactoring will
    /// move the actual implementation here.
    ///
    /// # Arguments
    /// * `loopform` - The LoopFormBuilder containing carrier/pinned metadata
    /// * `ops` - Operations trait for MIR emission
    /// * `exit_id` - Exit block ID
    /// * `branch_source_block` - Branch source block
    /// * `exit_snapshots` - Snapshots from each exit predecessor
    pub fn build_exit_phis<O: super::builder_core::LoopFormOps>(
        loopform: &super::builder_core::LoopFormBuilder,
        ops: &mut O,
        exit_id: BasicBlockId,
        branch_source_block: BasicBlockId,
        exit_snapshots: &[(BasicBlockId, BTreeMap<String, ValueId>)],
    ) -> Result<(), String> {
        // Delegate to existing implementation
        loopform.build_exit_phis(ops, exit_id, branch_source_block, exit_snapshots)
    }

    /// Build exit PHIs for ControlForm (adapter)
    ///
    /// This adapter extracts the LoopShape from ControlForm and calls build_exit_phis.
    /// This is the interface used by the loop lowering code.
    ///
    /// # Arguments
    /// * `loopform` - The LoopFormBuilder
    /// * `ops` - Operations trait
    /// * `form` - ControlForm containing loop structure
    /// * `exit_snapshots` - Snapshots from exit predecessors
    /// * `branch_source_block` - Branch source block
    pub fn build_exit_phis_for_control<O: super::builder_core::LoopFormOps>(
        loopform: &super::builder_core::LoopFormBuilder,
        ops: &mut O,
        form: &ControlForm,
        exit_snapshots: &[(BasicBlockId, BTreeMap<String, ValueId>)],
        branch_source_block: BasicBlockId,
    ) -> Result<(), String> {
        use crate::mir::control_form::ControlKind;

        // Only process Loop structures; silently succeed for other control kinds
        let shape = match &form.kind {
            ControlKind::Loop(shape) => shape,
            _ => return Ok(()),
        };

        // Extract exit_id from LoopShape
        let exit_id = shape.exit;

        // Log ControlForm usage when trace is enabled
        let trace = std::env::var("NYASH_LOOPFORM_DEBUG").ok().is_some();
        if trace {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[loopform/exit-phi/control-form] Using ControlForm wrapper: exit={:?} branch_source={:?}",
                exit_id, branch_source_block
            ));
        }

        // Delegate to the main build_exit_phis
        Self::build_exit_phis(loopform, ops, exit_id, branch_source_block, exit_snapshots)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_phi_builder_exists() {
        // ExitPhiBuilder is stateless, so just check it exists
        let _ = ExitPhiBuilder;
    }
}
