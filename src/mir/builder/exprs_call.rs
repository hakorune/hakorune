use super::ValueId;
use crate::ast::ASTNode;

impl super::MirBuilder {
    // Indirect call: (callee)(args...)
    pub(super) fn build_indirect_call_expression(
        &mut self,
        callee: ASTNode,
        arguments: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        let callee_id = self.build_expression_impl(callee)?;
        let arg_ids = self.build_call_args(&arguments)?;

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
