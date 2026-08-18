//! One physical execution-entry boundary for lifecycle-aware lowering.
//!
//! The function entry remains the sole parameter/sidecar origin.  A selected
//! lifecycle route may insert exactly one successor as its execution entry;
//! the canonical CFG/SSA session owns that relation and no caller remaps raw
//! block ids afterward.

use crate::mir::builder::MirBuilder;
use crate::mir::BasicBlockId;

use super::CanonicalSsaFunctionSessionV2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PhysicalEntryExecutionBoundaryV1 {
    function_entry: BasicBlockId,
    execution_entry: BasicBlockId,
}

impl PhysicalEntryExecutionBoundaryV1 {
    pub(super) const fn function_entry(self) -> BasicBlockId {
        self.function_entry
    }

    pub(super) const fn execution_entry(self) -> BasicBlockId {
        self.execution_entry
    }
}

impl<'source> CanonicalSsaFunctionSessionV2<'source> {
    pub(in crate::mir::builder::resolved_lowering) fn issue_physical_entry_execution_boundary(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<BasicBlockId, String> {
        if self.physical_entry_execution.is_some() {
            return Err("physical execution entry was already issued".to_owned());
        }
        let function_entry = self.entry_block(builder)?;
        if builder.function_state.current_block != Some(function_entry) {
            return Err("physical execution entry requires canonical function entry".to_owned());
        }
        let sidecar = self
            .physical_entry_sidecar
            .as_ref()
            .ok_or_else(|| "physical entry sidecar is missing".to_owned())?;
        if sidecar.entry() != function_entry || sidecar.owner() != self.owner {
            return Err("physical entry sidecar boundary drift".to_owned());
        }
        let execution_entry = self.create_unpublished_block(builder)?;
        let boundary = PhysicalEntryExecutionBoundaryV1 {
            function_entry,
            execution_entry,
        };
        self.physical_entry_execution = Some(boundary);
        Ok(boundary.execution_entry())
    }

    pub(in crate::mir::builder::resolved_lowering) fn physical_execution_entry(
        &self,
        builder: &MirBuilder,
    ) -> Result<BasicBlockId, String> {
        self.physical_entry_execution
            .map(|boundary| boundary.execution_entry())
            .or_else(|| self.entry_block(builder).ok())
            .ok_or_else(|| "physical execution entry is missing".to_owned())
    }
}
