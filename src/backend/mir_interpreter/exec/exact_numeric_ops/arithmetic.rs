use crate::backend::mir_interpreter::{MirInterpreter, VMError};
use crate::mir::numeric_substrate::{
    exact_numeric_checked_arithmetic, exact_numeric_mir_type_from_declared_name,
    ExactNumericArithmeticError, ExactNumericArithmeticOp, NumericTarget,
};
use crate::mir::{BasicBlockId, BinaryOp, ValueId};

impl MirInterpreter {
    pub(in crate::backend::mir_interpreter::exec) fn try_handle_exact_numeric_binop_reference(
        &mut self,
        block: BasicBlockId,
        instruction_index: usize,
        dst: ValueId,
        op: BinaryOp,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Result<bool, VMError> {
        let Some(declared_type_name) = self.exact_numeric_binop_route_declared_type(
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
                "[vm/exact_numeric_op_route_invalid] declared_type={}",
                declared_type_name
            )));
        };

        let lhs_exact = self.exact_numeric_operand("lhs", &declared_type_name, &ty, lhs)?;
        let rhs_exact = self.exact_numeric_operand("rhs", &declared_type_name, &ty, rhs)?;

        let arithmetic_op = exact_numeric_arithmetic_op(op).ok_or_else(|| {
            self.err_invalid(format!(
                "[vm/exact_numeric_op_unsupported] op={:?} declared_type={}",
                op, declared_type_name
            ))
        })?;
        let result = exact_numeric_checked_arithmetic(&lhs_exact, &rhs_exact, arithmetic_op)
            .map_err(|error| self.exact_numeric_arithmetic_error(&declared_type_name, error))?;
        self.write_exact_numeric_result(dst, result);
        Ok(true)
    }

    fn exact_numeric_binop_route_declared_type(
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
            .exact_numeric_binary_op_route_facts
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

    fn exact_numeric_arithmetic_error(
        &self,
        declared_type_name: &str,
        error: ExactNumericArithmeticError,
    ) -> VMError {
        match error {
            ExactNumericArithmeticError::TypeMismatch {
                left_source_name,
                right_source_name,
            } => self.err_invalid(format!(
                "[vm/exact_numeric_op_type_mismatch] declared_type={} left={} right={}",
                declared_type_name, left_source_name, right_source_name
            )),
            ExactNumericArithmeticError::ResultOutOfRange {
                op,
                lhs,
                rhs,
                result,
                min,
                max,
                ..
            } => self.err_invalid(format!(
                "[vm/exact_numeric_op_overflow] declared_type={} op={:?} lhs={} rhs={} result={:?} range={}..={}",
                declared_type_name, op, lhs, rhs, result, min, max
            )),
        }
    }
}

fn exact_numeric_arithmetic_op(op: BinaryOp) -> Option<ExactNumericArithmeticOp> {
    match op {
        BinaryOp::Add => Some(ExactNumericArithmeticOp::Add),
        BinaryOp::Sub => Some(ExactNumericArithmeticOp::Sub),
        BinaryOp::Mul => Some(ExactNumericArithmeticOp::Mul),
        _ => None,
    }
}
