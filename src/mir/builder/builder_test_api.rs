use super::MirBuilder;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirInstruction, MirType, ValueId};

impl MirBuilder {
    pub fn enter_function_for_test(&mut self, name: String) {
        let entry_block = self.core_ctx.next_block();
        let signature = FunctionSignature {
            name,
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let function = self.new_function_with_metadata(signature, entry_block);
        self.function_state.current_function = Some(function);
        self.function_state.current_block = Some(entry_block);
        // Phase 29bq+: reset sealing session for new function
        self.function_state.frag_emit_session.reset();
    }

    pub fn exit_function_for_test(&mut self) {
        self.function_state.current_function = None;
        self.function_state.current_block = None;
    }

    pub fn push_block_for_test(&mut self) -> Result<BasicBlockId, String> {
        let block_id = self.core_ctx.next_block();
        self.start_new_block(block_id)?;
        Ok(block_id)
    }

    pub fn current_block_for_test(&self) -> Result<BasicBlockId, String> {
        self.function_state
            .current_block
            .ok_or_else(|| "No current block".to_string())
    }

    pub fn alloc_value_for_test(&mut self) -> ValueId {
        self.next_value_id()
    }

    pub fn emit_for_test(&mut self, inst: MirInstruction) -> Result<(), String> {
        self.emit_instruction(inst)
    }

    pub fn bind_function_parameter_for_test(&mut self, name: &str, value: ValueId) {
        self.function_state
            .scope
            .function_param_names
            .insert(name.to_string());
        self.function_state
            .variable_ctx
            .insert(name.to_string(), value);
    }

    pub fn bind_variable_for_test(&mut self, name: &str, value: ValueId) {
        self.function_state
            .variable_ctx
            .insert(name.to_string(), value);
    }
}
