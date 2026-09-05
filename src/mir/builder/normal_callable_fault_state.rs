//! One physical Fault-frame definition per exact callable source scope.
//! Source entry selection owns the role; consumers only borrow the operand.

use super::CallableSemanticLoweringState;
use crate::mir::instruction::FaultFrameMode;
use crate::mir::{MirBuilder, MirFunction, MirInstruction, ValueId};

#[derive(Debug)]
pub(super) struct CallableFaultFrame {
    mode: FaultFrameMode,
    value: Option<ValueId>,
}

impl CallableFaultFrame {
    pub(super) fn borrowed() -> Self {
        Self {
            mode: FaultFrameMode::Borrowed,
            value: None,
        }
    }

    fn select_root(&mut self) -> Result<(), String> {
        if self.value.is_some() || self.mode != FaultFrameMode::Borrowed {
            return Err(freeze("root-role-after-use-or-duplicate"));
        }
        self.mode = FaultFrameMode::RootOwned;
        Ok(())
    }

    fn materialize(&mut self, builder: &mut MirBuilder) -> Result<ValueId, String> {
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

impl CallableSemanticLoweringState {
    /// Only the source-identity-checked App Main root entry calls this.
    pub(in crate::mir::builder) fn select_root_fault_frame(&mut self) -> Result<(), String> {
        self.fault_frame.select_root()
    }

    pub(in crate::mir::builder) fn borrow_fault_frame(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<ValueId, String> {
        self.fault_frame.materialize(builder)
    }

    pub(in crate::mir::builder) fn validate_fault_frame(
        &self,
        function: &MirFunction,
    ) -> Result<(), String> {
        let Some(expected) = self.fault_frame.value else {
            return Ok(());
        };
        let entry = function
            .blocks
            .get(&function.entry_block)
            .ok_or_else(|| freeze("entry-missing"))?;
        if !entry.instructions.iter().any(|instruction| {
            matches!(instruction,
            MirInstruction::FaultFrameEnter { dst, mode }
                if *dst == expected && *mode == self.fault_frame.mode)
        }) {
            return Err(freeze("definition-or-role-drift"));
        }
        Ok(())
    }
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][callable-fault-frame/{reason}]")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType,
    };

    #[test]
    fn frame_is_issued_once_and_root_selection_precedes_materialization() {
        // Physical-state test; App Main source identity is checked by its
        // production root entry, not manufactured by this fixture.
        for mode in [FaultFrameMode::Borrowed, FaultFrameMode::RootOwned] {
            let mut state = CallableFaultFrame::borrowed();
            if mode == FaultFrameMode::RootOwned {
                state.select_root().unwrap();
                assert!(state.select_root().is_err());
            }
            let mut builder = MirBuilder::new();
            let entry = BasicBlockId::new(0);
            let mut function = MirFunction::new(
                FunctionSignature {
                    name: "frame_test".into(),
                    params: vec![],
                    return_type: MirType::Void,
                    effects: EffectMask::CONTROL,
                },
                entry,
            );
            function.add_block(BasicBlock::new(entry));
            builder.function_state.current_function = Some(function);
            let value = state.materialize(&mut builder).unwrap();
            assert_eq!(state.materialize(&mut builder).unwrap(), value);
            assert!(state.select_root().is_err());
            let function = builder.function_state.current_function.as_ref().unwrap();
            assert!(function.params.is_empty());
            let block = &function.blocks[&entry];
            assert_eq!(block.instructions.len(), 1);
            assert_eq!(block.instruction_spans.len(), 1);
            assert!(matches!(block.instructions[0],
                MirInstruction::FaultFrameEnter { dst, mode: actual }
                if dst == value && actual == mode));
        }
    }
}
