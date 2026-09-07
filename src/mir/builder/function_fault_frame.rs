//! One physical Fault-frame definition per function, shared by source owners.
//! Role selection permission remains at the exact callable or Script entry.
//! This state neither discovers an entry nor issues source lifecycle meaning.

use crate::mir::instruction::FaultFrameMode;
use crate::mir::{MirBuilder, MirFunction, MirInstruction, ValueId};

#[derive(Debug)]
pub(in crate::mir::builder) struct FunctionFaultFrameV1 {
    mode: FaultFrameMode,
    value: Option<ValueId>,
}

impl FunctionFaultFrameV1 {
    pub(in crate::mir::builder) fn borrowed() -> Self {
        Self {
            mode: FaultFrameMode::Borrowed,
            value: None,
        }
    }

    pub(in crate::mir::builder) fn select_root(&mut self) -> Result<(), String> {
        if self.value.is_some() || self.mode != FaultFrameMode::Borrowed {
            return Err(freeze("root-role-after-use-or-duplicate"));
        }
        self.mode = FaultFrameMode::RootOwned;
        Ok(())
    }

    pub(in crate::mir::builder) fn materialize(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<ValueId, String> {
        if let Some(value) = self.value {
            return Ok(value);
        }
        let value = builder.next_value_id();
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| freeze("function-missing"))?;
        function
            .blocks
            .get_mut(&function.entry_block)
            .ok_or_else(|| freeze("entry-missing"))?
            .insert_instruction_after_phis(MirInstruction::FaultFrameEnter {
                dst: value,
                mode: self.mode,
            });
        self.value = Some(value);
        Ok(value)
    }
}

impl FunctionFaultFrameV1 {
    pub(in crate::mir::builder) fn validate(&self, function: &MirFunction) -> Result<(), String> {
        let definitions = function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|instruction| matches!(instruction, MirInstruction::FaultFrameEnter { .. }))
            .count();
        if definitions != usize::from(self.value.is_some()) {
            return Err(freeze("definition-count-drift"));
        }
        let Some(expected) = self.value else {
            return Ok(());
        };
        let entry = function
            .blocks
            .get(&function.entry_block)
            .ok_or_else(|| freeze("entry-missing"))?;
        if !entry.instructions.iter().any(|instruction| {
            matches!(instruction,
            MirInstruction::FaultFrameEnter { dst, mode }
                if *dst == expected && *mode == self.mode)
        }) {
            return Err(freeze("definition-or-role-drift"));
        }
        Ok(())
    }
}

fn freeze(reason: &str) -> String {
    // Preserve the established diagnostic contract while sharing its mechanics.
    format!("[freeze:contract][callable-fault-frame/{reason}]")
}

#[cfg(test)]
#[path = "function_fault_frame_tests.rs"]
mod tests;
