use super::ValueId;
use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RecursiveChildLoweringPortV1,
};

impl super::MirBuilder {
    /// Lower a qmark operand through the existing raw child port.
    ///
    /// The Result-like control-flow and runtime-call semantics stay owned by
    /// this helper; only the operand cannot recreate a legacy-only port.
    pub(in crate::mir::builder) fn build_qmark_propagate_expression_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        expression: ASTNode,
    ) -> Result<ValueId, String>
    where
        Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
    {
        let res_val = drive_legacy_expression_v1(self, port, expression)?;
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
