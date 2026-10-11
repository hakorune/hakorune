//! Physical write boundary for the root exit graph.
//! The raw implementation preserves the current block and binding order.
use super::*;

pub(in crate::mir::builder) trait RootGraphWriterPortV1 {
    fn reserve_block(&mut self, builder: &mut MirBuilder) -> Result<BasicBlockId, String>;
    fn append_terminated(
        &mut self,
        builder: &mut MirBuilder,
        id: BasicBlockId,
        terminator: MirInstruction,
        bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String>;
    fn emit_current(
        &mut self,
        builder: &mut MirBuilder,
        instruction: MirInstruction,
    ) -> Result<(BasicBlockId, MirInstruction), String>;
    fn insert_projection(
        &mut self,
        builder: &mut MirBuilder,
        landing: BasicBlockId,
        projection: MirInstruction,
    ) -> Result<(), String>;
}

pub(super) struct RawRootGraphWriterV1;

impl RootGraphWriterPortV1 for RawRootGraphWriterV1 {
    fn reserve_block(&mut self, builder: &mut MirBuilder) -> Result<BasicBlockId, String> {
        Ok(builder.next_block_id())
    }

    fn append_terminated(
        &mut self,
        builder: &mut MirBuilder,
        id: BasicBlockId,
        terminator: MirInstruction,
        bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        if !matches!(
            terminator,
            MirInstruction::Return { .. }
                | MirInstruction::ReturnFault { .. }
                | MirInstruction::Jump {
                    edge_args: None,
                    ..
                }
                | MirInstruction::Invoke { .. }
        ) {
            return Err(freeze("root-graph-terminator-unsupported"));
        }
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| freeze("no-function"))?;
        if function.blocks.contains_key(&id) {
            return Err(freeze("duplicate-block"));
        }
        let mut block = BasicBlock::new(id);
        block.set_terminator(terminator.clone());
        function.add_block(block);
        bindings.push((id, terminator));
        Ok(())
    }

    fn emit_current(
        &mut self,
        builder: &mut MirBuilder,
        instruction: MirInstruction,
    ) -> Result<(BasicBlockId, MirInstruction), String> {
        if !matches!(
            instruction,
            MirInstruction::Jump {
                edge_args: None,
                ..
            } | MirInstruction::Invoke { .. }
        ) {
            return Err(freeze("root-graph-current-unsupported"));
        }
        let block = builder
            .function_state
            .current_block
            .ok_or_else(|| freeze("no-block"))?;
        builder.emit_instruction(instruction.clone())?;
        Ok((block, instruction))
    }

    fn insert_projection(
        &mut self,
        builder: &mut MirBuilder,
        landing: BasicBlockId,
        projection: MirInstruction,
    ) -> Result<(), String> {
        if !matches!(projection, MirInstruction::InvokeNormalResult { .. }) {
            return Err(freeze("root-graph-projection-unsupported"));
        }
        builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| freeze("no-function"))?
            .blocks
            .get_mut(&landing)
            .ok_or_else(|| freeze("no-normal-landing"))?
            .add_instruction(projection);
        Ok(())
    }
}
