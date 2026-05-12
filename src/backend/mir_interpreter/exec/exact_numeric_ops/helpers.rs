use crate::backend::mir_interpreter::{MirInterpreter, VMError, VMValue};
use crate::backend::vm_types::ExactNumericRuntimeValue;
use crate::mir::numeric_substrate::{
    exact_numeric_const_from_i128, exact_numeric_value_from_dynamic_integer,
    ExactNumericConstValue, ExactNumericConversionError, ExactNumericMirType,
};
use crate::mir::ValueId;

impl MirInterpreter {
    pub(in crate::backend::mir_interpreter::exec::exact_numeric_ops) fn exact_numeric_operand(
        &self,
        role: &'static str,
        declared_type_name: &str,
        ty: &ExactNumericMirType,
        value: ValueId,
    ) -> Result<ExactNumericConstValue, VMError> {
        let vm_value = self.reg_load(value)?;
        match vm_value {
            VMValue::Integer(integer) => exact_numeric_value_from_dynamic_integer(integer, ty)
                .map_err(|error| {
                    self.exact_numeric_operand_range_error(
                        role,
                        declared_type_name,
                        i128::from(integer),
                        error,
                    )
                }),
            VMValue::ExactNumeric(exact) if exact.source_name == ty.source_name => {
                exact_numeric_const_from_i128(exact.value, ty).map_err(|error| {
                    self.exact_numeric_operand_range_error(
                        role,
                        declared_type_name,
                        exact.value,
                        error,
                    )
                })
            }
            other => Err(self.err_invalid(format!(
                "[vm/exact_numeric_op_type] role={} declared_type={} value={} actual={:?}",
                role, declared_type_name, value, other
            ))),
        }
    }

    pub(in crate::backend::mir_interpreter::exec::exact_numeric_ops) fn write_exact_numeric_result(
        &mut self,
        dst: ValueId,
        value: ExactNumericConstValue,
    ) {
        self.vm_fast_cache_clear(dst);
        self.write_reg(
            dst,
            VMValue::ExactNumeric(ExactNumericRuntimeValue::new(
                value.ty.source_name,
                value.value,
            )),
        );
    }

    pub(in crate::backend::mir_interpreter::exec::exact_numeric_ops) fn exact_numeric_operand_range_error(
        &self,
        role: &'static str,
        declared_type_name: &str,
        value: i128,
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
