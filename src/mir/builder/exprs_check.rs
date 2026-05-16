use crate::ast::CheckItem;

use super::{MirInstruction, MirType, ValueId};

impl super::MirBuilder {
    pub(super) fn build_check_expression(
        &mut self,
        items: Vec<CheckItem>,
    ) -> Result<ValueId, String> {
        let one = crate::mir::builder::emission::constant::emit_integer(self, 1)?;
        let zero = crate::mir::builder::emission::constant::emit_integer(self, 0)?;
        let mut ok = one;

        for item in items {
            let condition = self.build_expression_impl(item.expression)?;
            let dst = self.next_value_id();
            self.emit_instruction(MirInstruction::Select {
                dst,
                cond: condition,
                then_val: ok,
                else_val: zero,
            })?;
            self.type_ctx.value_types.insert(dst, MirType::Integer);
            ok = dst;
        }

        Ok(ok)
    }
}
