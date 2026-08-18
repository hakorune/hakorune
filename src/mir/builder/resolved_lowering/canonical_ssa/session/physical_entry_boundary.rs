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

    /// Lifecycle-aware cursor lowering must populate the execution entry
    /// before the final frame and Residence Enter exist.  Defer only that
    /// entry's seal; all other CFG/SSA ownership remains canonical.
    pub(in crate::mir::builder::resolved_lowering) fn defer_physical_entry_seal(
        &mut self,
    ) -> Result<(), String> {
        if self.physical_entry_execution.is_none() {
            return Err("cannot defer a missing physical execution entry".to_owned());
        }
        if self.physical_entry_seal_deferred {
            return Err("physical execution-entry seal was already deferred".to_owned());
        }
        self.physical_entry_seal_deferred = true;
        Ok(())
    }

    pub(in crate::mir::builder::resolved_lowering) fn physical_entry_seal_deferred(&self) -> bool {
        self.physical_entry_seal_deferred
    }

    /// Close the deferred E1 only after Residence Enter has installed the
    /// E0 -> E1 edge.  The CFG witness and Binding/SSA witness are issued in
    /// this one canonical session; callers cannot append a predecessor later.
    pub(in crate::mir::builder::resolved_lowering) fn seal_deferred_physical_entry(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<(), String> {
        if !self.physical_entry_seal_deferred {
            return Err("physical execution-entry seal was not deferred".to_owned());
        }
        let boundary = self
            .physical_entry_execution
            .ok_or_else(|| "physical execution entry is missing".to_owned())?;
        let function_entry_witness = {
            let function = builder
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| "physical execution-entry seal requires a function".to_owned())?;
            self.cfg
                .seal_block(function, boundary.function_entry())
                .map_err(|error| error.to_string())?
        };
        self.identity
            .seal_block(
                builder,
                &mut self.phis,
                boundary.function_entry(),
                &function_entry_witness,
            )
            .map_err(|error| error.to_string())?;
        let execution_entry_witness = {
            let function = builder
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| "physical execution-entry seal requires a function".to_owned())?;
            self.cfg
                .seal_block(function, boundary.execution_entry())
                .map_err(|error| error.to_string())?
        };
        self.identity
            .seal_block(
                builder,
                &mut self.phis,
                boundary.execution_entry(),
                &execution_entry_witness,
            )
            .map_err(|error| error.to_string())?;
        self.physical_entry_seal_deferred = false;
        Ok(())
    }

    pub(in crate::mir::builder::resolved_lowering) fn defer_s6c_cursor_seals(
        &mut self,
        blocks: [BasicBlockId; 5],
    ) -> Result<(), String> {
        if !self.physical_entry_seal_deferred {
            return Err("S6C cursor seal deferral requires a deferred physical entry".to_owned());
        }
        if self.deferred_s6c_cursor_blocks.replace(blocks).is_some() {
            return Err("S6C cursor seals were already deferred".to_owned());
        }
        Ok(())
    }

    pub(in crate::mir::builder::resolved_lowering) fn seal_deferred_s6c_cursor_blocks(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<(), String> {
        let [body, continuation, condition, then_block, after] = self
            .deferred_s6c_cursor_blocks
            .take()
            .ok_or_else(|| "S6C cursor seals were not deferred".to_owned())?;
        for block in [body, continuation, condition, then_block, after] {
            let function = builder
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| "S6C cursor seal requires a function".to_owned())?;
            let witness = self
                .cfg
                .seal_block(function, block)
                .map_err(|error| error.to_string())?;
            self.identity
                .seal_block(builder, &mut self.phis, block, &witness)
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    }
}
