use crate::backend::mir_interpreter::{MirInterpreter, VMError};
use crate::mir::numeric_substrate::{
    exact_numeric_logical_shr, exact_numeric_mir_type_from_declared_name, ExactNumericShiftError,
    NumericTarget,
};
use crate::mir::{BasicBlockId, BinaryOp, ValueId};
use std::convert::TryFrom;

impl MirInterpreter {
    pub(in crate::backend::mir_interpreter::exec) fn try_handle_exact_numeric_shift_reference(
        &mut self,
        block: BasicBlockId,
        instruction_index: usize,
        dst: ValueId,
        op: BinaryOp,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Result<bool, VMError> {
        let Some(declared_type_name) = self.exact_numeric_shift_route_declared_type(
            block,
            instruction_index,
            dst,
            op,
            lhs,
            rhs,
        ) else {
            return Ok(false);
        };

        let Some(ty) = exact_numeric_mir_type_from_declared_name(
            Some(declared_type_name.as_str()),
            NumericTarget::host(),
        ) else {
            return Err(self.err_invalid(format!(
                "[vm/exact_numeric_shift_route_invalid] declared_type={}",
                declared_type_name
            )));
        };

        let lhs_exact = self.exact_numeric_operand("lhs", &declared_type_name, &ty, lhs)?;
        let rhs_value = self.reg_load(rhs)?;
        let rhs_integer = rhs_value.as_integer().map_err(|_| {
            self.err_invalid(format!(
                "[vm/exact_numeric_op_type] role=shift declared_type={} value={} actual={:?}",
                declared_type_name, rhs, rhs_value
            ))
        })?;
        let shift = u32::try_from(rhs_integer).map_err(|_| {
            self.err_invalid(format!(
                "[vm/exact_numeric_shift_count] declared_type={} shift={} reason=out-of-u32",
                declared_type_name, rhs_integer
            ))
        })?;

        let result = exact_numeric_logical_shr(&lhs_exact, shift)
            .map_err(|error| self.exact_numeric_shift_error(&declared_type_name, error))?;
        self.write_exact_numeric_result(dst, result);
        Ok(true)
    }

    fn exact_numeric_shift_route_declared_type(
        &self,
        block: BasicBlockId,
        instruction_index: usize,
        dst: ValueId,
        op: BinaryOp,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Option<String> {
        let function = self
            .cur_fn
            .as_ref()
            .and_then(|function_name| self.functions.get(function_name))?;

        function
            .metadata
            .exact_numeric_shift_route_facts
            .iter()
            .find(|fact| {
                fact.block == block
                    && fact.instruction_index == instruction_index
                    && fact.dst == dst
                    && fact.op == op
                    && fact.lhs == lhs
                    && fact.rhs == rhs
            })
            .map(|fact| fact.declared_type_name.clone())
    }

    fn exact_numeric_shift_error(
        &self,
        declared_type_name: &str,
        error: ExactNumericShiftError,
    ) -> VMError {
        match error {
            ExactNumericShiftError::SignedLogicalShift { source_name } => self.err_invalid(format!(
                "[vm/exact_numeric_shift_signed] declared_type={} source={}",
                declared_type_name, source_name
            )),
            ExactNumericShiftError::ShiftCountOutOfRange {
                shift, width_bits, ..
            } => self.err_invalid(format!(
                "[vm/exact_numeric_shift_count] declared_type={} shift={} width_bits={} reason=out-of-range",
                declared_type_name, shift, width_bits
            )),
        }
    }
}
