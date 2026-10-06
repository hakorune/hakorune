use super::*;
use crate::mir::ArrayElementWriteKind;

impl MirInterpreter {
    /// A proven `ArrayBox.get/1` read — the same surface `call_method`
    /// executed, carried on the canonical read instruction.
    pub(in crate::backend::mir_interpreter) fn execute_array_element_read(
        &mut self,
        dst: Option<ValueId>,
        receiver: ValueId,
        index: ValueId,
    ) -> Result<(), VMError> {
        let result =
            self.execute_method_callee("ArrayBox", "get", &Some(receiver), &[index])?;
        if let Some(dst) = dst {
            self.write_reg(dst, result);
        }
        Ok(())
    }

    pub(in crate::backend::mir_interpreter) fn execute_array_element_write(
        &mut self,
        dst: Option<ValueId>,
        kind: ArrayElementWriteKind,
        receiver: ValueId,
        index: Option<ValueId>,
        value: ValueId,
    ) -> Result<(), VMError> {
        crate::mir::array_element_write::validate_shape(kind, index)
            .map_err(|reason| self.err_invalid(reason))?;
        let (method, args) = match kind {
            ArrayElementWriteKind::LiteralAppend | ArrayElementWriteKind::Push => {
                ("push", vec![value])
            }
            ArrayElementWriteKind::Set => ("set", vec![index.expect("shape checked"), value]),
            ArrayElementWriteKind::Insert => ("insert", vec![index.expect("shape checked"), value]),
        };
        self.execute_method_callee("ArrayBox", method, &Some(receiver), &args)?;
        if let Some(dst) = dst {
            self.write_reg(dst, VMValue::Void);
        }
        Ok(())
    }
}
