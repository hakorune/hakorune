use super::ValueId;
use crate::ast::ASTNode;
use crate::mir::builder::calls::drive_call_arguments_v1;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1, RawLegacyChildLoweringPortV1,
};

impl super::MirBuilder {
    // Indirect call: (callee)(args...)
    pub(super) fn build_indirect_call_expression(
        &mut self,
        callee: ASTNode,
        arguments: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        let mut port = RawLegacyChildLoweringPortV1;
        self.build_indirect_call_expression_with_port_v1(&mut port, callee, arguments)
    }

    /// Lower an indirect call while retaining the caller's raw child port.
    pub(in crate::mir::builder) fn build_indirect_call_expression_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        callee: ASTNode,
        arguments: Vec<ASTNode>,
    ) -> Result<ValueId, String>
    where
        Port: RawAstChildLoweringPortV1,
    {
        let callee_id = drive_legacy_expression_v1(self, port, callee)?;
        let arg_ids = drive_call_arguments_v1(self, port, arguments.as_slice())?;

        // Phase 3.1: Use unified call with CallTarget::Value for indirect calls
        let use_unified = super::calls::call_unified::is_unified_call_enabled();

        if use_unified {
            // New unified path - use emit_unified_call with Value target
            let dst = self.next_value_id();
            self.emit_unified_call(Some(dst), super::CallTarget::Value(callee_id), arg_ids)?;
            Ok(dst)
        } else {
            // Unified-off path: still encode callee as Value to avoid by-name resolution
            let dst = self.next_value_id();
            self.emit_instruction(super::MirInstruction::Call {
                dst: Some(dst),
                func: callee_id,
                callee: Some(crate::mir::definitions::call_unified::Callee::Value(
                    callee_id,
                )),
                args: arg_ids,
                effects: super::EffectMask::PURE,
            })?;
            Ok(dst)
        }
    }
}
