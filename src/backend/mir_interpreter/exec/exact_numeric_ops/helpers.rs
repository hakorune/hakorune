use crate::backend::mir_interpreter::{MirInterpreter, VMError};
use crate::mir::numeric_substrate::ExactNumericConversionError;
use crate::mir::ValueId;

impl MirInterpreter {
    pub(in crate::backend::mir_interpreter::exec::exact_numeric_ops) fn exact_numeric_integer_operand(
        &self,
        role: &'static str,
        declared_type_name: &str,
        value: ValueId,
    ) -> Result<i64, VMError> {
        let vm_value = self.reg_load(value)?;
        vm_value.as_integer().map_err(|_| {
            self.err_invalid(format!(
                "[vm/exact_numeric_op_type] role={} declared_type={} value={} actual={:?}",
                role, declared_type_name, value, vm_value
            ))
        })
    }

    pub(in crate::backend::mir_interpreter::exec::exact_numeric_ops) fn exact_numeric_operand_range_error(
        &self,
        role: &'static str,
        declared_type_name: &str,
        value: i64,
        error: ExactNumericConversionError,
    ) -> VMError {
        let reason = match error {
            ExactNumericConversionError::NegativeToUnsigned { .. } => "negative-to-unsigned",
            ExactNumericConversionError::OutOfRange { .. } => "out-of-range",
        };
        self.err_invalid(format!(
            "[vm/exact_numeric_op_range] role={} declared_type={} value={} reason={}",
            role, declared_type_name, value, reason
        ))
    }
}
