use crate::backend::mir_interpreter::{MirInterpreter, VMError, VMValue};
use crate::mir::numeric_substrate::{
    exact_numeric_compare, exact_numeric_mir_type_from_declared_name,
    exact_numeric_value_from_dynamic_integer, ExactNumericCompareError, ExactNumericCompareOp,
    NumericTarget,
};
use crate::mir::{BasicBlockId, CompareOp, ValueId};

impl MirInterpreter {
    pub(in crate::backend::mir_interpreter::exec) fn try_handle_exact_numeric_compare_reference(
        &mut self,
        block: BasicBlockId,
        instruction_index: usize,
        dst: ValueId,
        op: CompareOp,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Result<bool, VMError> {
        let Some(declared_type_name) = self.exact_numeric_compare_route_declared_type(
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
                "[vm/exact_numeric_compare_route_invalid] declared_type={}",
                declared_type_name
            )));
        };

        let lhs_integer = self.exact_numeric_integer_operand("lhs", &declared_type_name, lhs)?;
        let rhs_integer = self.exact_numeric_integer_operand("rhs", &declared_type_name, rhs)?;
        let lhs_exact =
            exact_numeric_value_from_dynamic_integer(lhs_integer, &ty).map_err(|error| {
                self.exact_numeric_operand_range_error(
                    "lhs",
                    &declared_type_name,
                    lhs_integer,
                    error,
                )
            })?;
        let rhs_exact =
            exact_numeric_value_from_dynamic_integer(rhs_integer, &ty).map_err(|error| {
                self.exact_numeric_operand_range_error(
                    "rhs",
                    &declared_type_name,
                    rhs_integer,
                    error,
                )
            })?;

        let compare_op = exact_numeric_compare_op(op);
        let result = exact_numeric_compare(&lhs_exact, &rhs_exact, compare_op)
            .map_err(|error| self.exact_numeric_compare_error(&declared_type_name, error))?;

        self.vm_fast_cache_set_bool(dst, result);
        self.write_reg(dst, VMValue::Bool(result));
        Ok(true)
    }

    fn exact_numeric_compare_route_declared_type(
        &self,
        block: BasicBlockId,
        instruction_index: usize,
        dst: ValueId,
        op: CompareOp,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Option<String> {
        let function = self
            .cur_fn
            .as_ref()
            .and_then(|function_name| self.functions.get(function_name))?;

        function
            .metadata
            .exact_numeric_compare_route_facts
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

    fn exact_numeric_compare_error(
        &self,
        declared_type_name: &str,
        error: ExactNumericCompareError,
    ) -> VMError {
        match error {
            ExactNumericCompareError::TypeMismatch {
                left_source_name,
                right_source_name,
            } => self.err_invalid(format!(
                "[vm/exact_numeric_compare_type_mismatch] declared_type={} left={} right={}",
                declared_type_name, left_source_name, right_source_name
            )),
        }
    }
}

fn exact_numeric_compare_op(op: CompareOp) -> ExactNumericCompareOp {
    match op {
        CompareOp::Eq => ExactNumericCompareOp::Eq,
        CompareOp::Ne => ExactNumericCompareOp::Ne,
        CompareOp::Lt => ExactNumericCompareOp::Lt,
        CompareOp::Le => ExactNumericCompareOp::Le,
        CompareOp::Gt => ExactNumericCompareOp::Gt,
        CompareOp::Ge => ExactNumericCompareOp::Ge,
    }
}
