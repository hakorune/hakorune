use super::ValueId;
use crate::ast::ASTNode;

impl super::MirBuilder {
    // QMarkPropagate: result?.value (Result-like)
    pub(super) fn build_qmark_propagate_expression(
        &mut self,
        expression: ASTNode,
    ) -> Result<ValueId, String> {
        let res_val = self.build_expression_impl(expression)?;
        let res_local = self.local_ssa_ensure(res_val, 0);
        let ok_id = self.next_value_id();
        self.emit_instruction(crate::mir::ssot::method_call::runtime_method_call(
            Some(ok_id),
            res_local,
            "RuntimeDataBox",
            "isOk",
            vec![],
            super::EffectMask::PURE,
            crate::mir::definitions::call_unified::TypeCertainty::Union,
        ))?;
        let then_block = self.next_block_id();
        let else_block = self.next_block_id();
        let ok_local = self.local_ssa_ensure(ok_id, 4);
        crate::mir::builder::emission::branch::emit_conditional(
            self, ok_local, then_block, else_block,
        )?;
        self.start_new_block(then_block)?;
        self.emit_instruction(super::MirInstruction::Return {
            value: Some(res_local),
        })?;
        self.start_new_block(else_block)?;
        let val_id = self.next_value_id();
        self.emit_instruction(crate::mir::ssot::method_call::runtime_method_call(
            Some(val_id),
            res_local,
            "RuntimeDataBox",
            "getValue",
            vec![],
            super::EffectMask::PURE,
            crate::mir::definitions::call_unified::TypeCertainty::Union,
        ))?;
        Ok(val_id)
    }
}
