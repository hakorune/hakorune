use super::super::{MirInterpreter, VMError, VMValue};
use crate::mir::numeric_substrate::{
    exact_numeric_checked_arithmetic, exact_numeric_compare,
    exact_numeric_mir_type_from_declared_name, exact_numeric_value_from_dynamic_integer,
    ExactNumericArithmeticError, ExactNumericArithmeticOp, ExactNumericCompareError,
    ExactNumericCompareOp, ExactNumericConversionError, NumericTarget,
};
use crate::mir::{BasicBlockId, BinaryOp, CompareOp, ValueId};
use std::convert::TryFrom;

impl MirInterpreter {
    pub(super) fn try_handle_exact_numeric_binop_reference(
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

        let arithmetic_op = exact_numeric_arithmetic_op(op).ok_or_else(|| {
            self.err_invalid(format!(
                "[vm/exact_numeric_op_unsupported] op={:?} declared_type={}",
                op, declared_type_name
            ))
        })?;
        let result = exact_numeric_checked_arithmetic(&lhs_exact, &rhs_exact, arithmetic_op)
            .map_err(|error| self.exact_numeric_arithmetic_error(&declared_type_name, error))?;
        let integer = i64::try_from(result.value).map_err(|_| {
            self.err_invalid(format!(
                "[vm/exact_numeric_op_result_unrepresentable] declared_type={} value={} vm_lane=Integer(i64)",
                declared_type_name, result.value
            ))
        })?;

        self.vm_fast_cache_set_i64(dst, integer);
        self.write_reg(dst, VMValue::Integer(integer));
        Ok(true)
    }

    pub(super) fn try_handle_exact_numeric_compare_reference(
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

    fn exact_numeric_integer_operand(
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

    fn exact_numeric_operand_range_error(
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

fn exact_numeric_arithmetic_op(op: BinaryOp) -> Option<ExactNumericArithmeticOp> {
    match op {
        BinaryOp::Add => Some(ExactNumericArithmeticOp::Add),
        BinaryOp::Sub => Some(ExactNumericArithmeticOp::Sub),
        BinaryOp::Mul => Some(ExactNumericArithmeticOp::Mul),
        _ => None,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::mir_interpreter::MirInterpreter;
    use crate::backend::vm_types::VMValue;
    use crate::mir::exact_numeric_value_facts::refresh_module_exact_numeric_value_facts;
    use crate::mir::function::MirParamDecl;
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirModule,
        MirType,
    };

    fn module_with_exact_numeric_arithmetic_route(
        declared_type_name: &str,
        op: BinaryOp,
    ) -> MirModule {
        let entry = BasicBlockId::new(0);
        let signature = FunctionSignature {
            name: "Main.arithmetic/2".to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, entry);
        let lhs = function.params[0];
        let rhs = function.params[1];
        let sum = function.next_value_id();
        function.metadata.declared_param_decls = vec![
            MirParamDecl {
                name: "lhs".to_string(),
                declared_type_name: Some(declared_type_name.to_string()),
            },
            MirParamDecl {
                name: "rhs".to_string(),
                declared_type_name: Some(declared_type_name.to_string()),
            },
        ];

        let block = function.get_block_mut(entry).unwrap();
        block.add_instruction(MirInstruction::BinOp {
            dst: sum,
            op,
            lhs,
            rhs,
        });
        block.add_instruction(MirInstruction::Return { value: Some(sum) });

        let mut module = MirModule::new("vm_exact_numeric_add_test".to_string());
        module.add_function(function);
        refresh_module_exact_numeric_value_facts(&mut module);
        let route_count = module
            .functions
            .get("Main.arithmetic/2")
            .expect("test function must exist")
            .metadata
            .exact_numeric_binary_op_route_facts
            .len();
        assert_eq!(route_count, 1);
        module
    }

    fn module_with_exact_numeric_compare_route(
        declared_type_name: &str,
        op: CompareOp,
    ) -> MirModule {
        let entry = BasicBlockId::new(0);
        let signature = FunctionSignature {
            name: "Main.compare/2".to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Bool,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, entry);
        let lhs = function.params[0];
        let rhs = function.params[1];
        let result = function.next_value_id();
        function.metadata.declared_param_decls = vec![
            MirParamDecl {
                name: "lhs".to_string(),
                declared_type_name: Some(declared_type_name.to_string()),
            },
            MirParamDecl {
                name: "rhs".to_string(),
                declared_type_name: Some(declared_type_name.to_string()),
            },
        ];

        let block = function.get_block_mut(entry).unwrap();
        block.add_instruction(MirInstruction::Compare {
            dst: result,
            op,
            lhs,
            rhs,
        });
        block.set_terminator(MirInstruction::Return {
            value: Some(result),
        });
        let mut module = MirModule::new("exact_numeric_vm_reference_test".to_string());
        module.add_function(function);

        refresh_module_exact_numeric_value_facts(&mut module);
        let route_count = module
            .functions
            .get("Main.compare/2")
            .expect("test function must exist")
            .metadata
            .exact_numeric_compare_route_facts
            .len();
        assert_eq!(
            route_count, 1,
            "test module must publish one exact compare route"
        );
        module
    }

    #[test]
    fn vm_reference_executes_exact_usize_add_route() {
        let module = module_with_exact_numeric_arithmetic_route("usize", BinaryOp::Add);
        let mut vm = MirInterpreter::new();

        let result = vm
            .execute_function_with_args(
                &module,
                "Main.arithmetic/2",
                &[VMValue::Integer(40), VMValue::Integer(2)],
            )
            .expect("exact usize add route should execute");

        assert_eq!(result, VMValue::Integer(42));
    }

    #[test]
    fn vm_reference_executes_exact_usize_sub_route() {
        let module = module_with_exact_numeric_arithmetic_route("usize", BinaryOp::Sub);
        let mut vm = MirInterpreter::new();

        let result = vm
            .execute_function_with_args(
                &module,
                "Main.arithmetic/2",
                &[VMValue::Integer(40), VMValue::Integer(2)],
            )
            .expect("exact usize sub route should execute");

        assert_eq!(result, VMValue::Integer(38));
    }

    #[test]
    fn vm_reference_rejects_negative_usize_add_operand() {
        let module = module_with_exact_numeric_arithmetic_route("usize", BinaryOp::Add);
        let mut vm = MirInterpreter::new();

        let error = vm
            .execute_function_with_args(
                &module,
                "Main.arithmetic/2",
                &[VMValue::Integer(-1), VMValue::Integer(2)],
            )
            .expect_err("negative usize operand must fail before generic i64 add");

        assert!(error.to_string().contains("[vm/exact_numeric_op_range]"));
        assert!(error.to_string().contains("negative-to-unsigned"));
    }

    #[test]
    fn vm_reference_rejects_exact_u8_add_overflow() {
        let module = module_with_exact_numeric_arithmetic_route("u8", BinaryOp::Add);
        let mut vm = MirInterpreter::new();

        let error = vm
            .execute_function_with_args(
                &module,
                "Main.arithmetic/2",
                &[VMValue::Integer(250), VMValue::Integer(10)],
            )
            .expect_err("u8 exact add overflow must fail before generic i64 add");

        assert!(error.to_string().contains("[vm/exact_numeric_op_overflow]"));
    }

    #[test]
    fn vm_reference_rejects_exact_u8_mul_overflow() {
        let module = module_with_exact_numeric_arithmetic_route("u8", BinaryOp::Mul);
        let mut vm = MirInterpreter::new();

        let error = vm
            .execute_function_with_args(
                &module,
                "Main.arithmetic/2",
                &[VMValue::Integer(16), VMValue::Integer(16)],
            )
            .expect_err("u8 exact mul overflow must fail before generic i64 mul");

        assert!(error.to_string().contains("[vm/exact_numeric_op_overflow]"));
    }

    #[test]
    fn vm_reference_rejects_exact_usize_result_outside_current_i64_lane() {
        let module = module_with_exact_numeric_arithmetic_route("usize", BinaryOp::Add);
        let mut vm = MirInterpreter::new();

        let error = vm
            .execute_function_with_args(
                &module,
                "Main.arithmetic/2",
                &[VMValue::Integer(i64::MAX), VMValue::Integer(1)],
            )
            .expect_err("usize result above i64 must fail until exact VMValue storage exists");

        assert!(error
            .to_string()
            .contains("[vm/exact_numeric_op_result_unrepresentable]"));
    }

    #[test]
    fn vm_reference_executes_exact_usize_compare_route() {
        let module = module_with_exact_numeric_compare_route("usize", CompareOp::Lt);
        let mut vm = MirInterpreter::new();

        let result = vm
            .execute_function_with_args(
                &module,
                "Main.compare/2",
                &[VMValue::Integer(2), VMValue::Integer(40)],
            )
            .expect("exact usize compare route should execute");

        assert_eq!(result, VMValue::Bool(true));
    }

    #[test]
    fn vm_reference_rejects_negative_usize_compare_operand() {
        let module = module_with_exact_numeric_compare_route("usize", CompareOp::Lt);
        let mut vm = MirInterpreter::new();

        let error = vm
            .execute_function_with_args(
                &module,
                "Main.compare/2",
                &[VMValue::Integer(-1), VMValue::Integer(2)],
            )
            .expect_err("negative usize compare operand must fail before generic i64 compare");

        assert!(error.to_string().contains("[vm/exact_numeric_op_range]"));
    }
}
