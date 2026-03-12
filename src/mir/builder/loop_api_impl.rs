use super::{BasicBlockId, MirBuilder, MirInstruction, ValueId};

impl crate::mir::loop_api::LoopBuilderApi for MirBuilder {
    fn new_block(&mut self) -> BasicBlockId {
        self.core_ctx.next_block()
    }

    fn current_block(&self) -> Result<BasicBlockId, String> {
        self.current_block
            .ok_or_else(|| "No current block".to_string())
    }

    fn start_new_block(&mut self, block: BasicBlockId) -> Result<(), String> {
        MirBuilder::start_new_block(self, block)
    }

    fn emit(&mut self, inst: MirInstruction) -> Result<(), String> {
        self.emit_instruction(inst)
    }

    fn new_value(&mut self) -> ValueId {
        self.next_value_id()
    }

    fn add_predecessor(&mut self, block: BasicBlockId, pred: BasicBlockId) -> Result<(), String> {
        if let Some(ref mut f) = self.scope_ctx.current_function {
            if let Some(bb) = f.get_block_mut(block) {
                bb.add_predecessor(pred);
                Ok(())
            } else {
                Err(format!("Block {} not found", block.as_u32()))
            }
        } else {
            Err("No current function".into())
        }
    }

    fn seal_block(&mut self, block: BasicBlockId) -> Result<(), String> {
        if let Some(ref mut f) = self.scope_ctx.current_function {
            if let Some(bb) = f.get_block_mut(block) {
                bb.seal();
                Ok(())
            } else {
                Err(format!("Block {} not found", block.as_u32()))
            }
        } else {
            Err("No current function".into())
        }
    }

    fn insert_phi_at_block_start(
        &mut self,
        block: BasicBlockId,
        dst: ValueId,
        inputs: Vec<(BasicBlockId, ValueId)>,
    ) -> Result<(), String> {
        if let Some(ref mut f) = self.scope_ctx.current_function {
            crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
                f,
                block,
                dst,
                inputs,
                self.metadata_ctx.current_span(),
            )?;
            Ok(())
        } else {
            Err("No current function".into())
        }
    }
}
