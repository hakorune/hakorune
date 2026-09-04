use super::ValueId;
use crate::ast::ASTNode;
use crate::mir::builder::calls::drive_call_arguments_v1;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1,
};

impl super::MirBuilder {
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
        if !super::calls::call_unified::is_unified_call_enabled() {
            return Err(
                "[freeze:contract][raw-indirect/unified-disabled-before-descent]".to_owned(),
            );
        }

        let callee_id = drive_legacy_expression_v1(self, port, callee)?;
        let arg_ids = drive_call_arguments_v1(self, port, arguments.as_slice())?;

        let dst = self.next_value_id();
        self.emit_unified_call(Some(dst), super::CallTarget::Value(callee_id), arg_ids)?;
        Ok(dst)
    }
}
