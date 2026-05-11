use super::super::{MirInterpreter, VMError};
use crate::mir::numeric_substrate::{
    exact_numeric_mir_type_from_declared_name, exact_numeric_value_from_dynamic_integer,
    ExactNumericConversionError, NumericTarget,
};
use crate::mir::{BasicBlockId, ExactNumericRuntimeCheckContractKind, ValueId};

impl MirInterpreter {
    pub(super) fn check_exact_numeric_runtime_check_contract(
        &mut self,
        block: BasicBlockId,
        instruction_index: usize,
        field: &str,
        value: ValueId,
    ) -> Result<(), VMError> {
        let Some(declared_type_name) = self.exact_numeric_runtime_check_contract_declared_type(
            block,
            instruction_index,
            field,
            value,
        ) else {
            return Ok(());
        };

        let Some(ty) = exact_numeric_mir_type_from_declared_name(
            Some(declared_type_name.as_str()),
            NumericTarget::host(),
        ) else {
            return Err(self.err_invalid(format!(
                "[vm/numeric_dynamic_range_contract_invalid] declared_type={}",
                declared_type_name
            )));
        };

        let vm_value = self.reg_load(value)?;
        let integer = vm_value.as_integer().map_err(|_| {
            self.err_invalid(format!(
                "[vm/numeric_dynamic_range_type] field={} declared_type={} value={} actual={:?}",
                field, declared_type_name, value, vm_value
            ))
        })?;

        exact_numeric_value_from_dynamic_integer(integer, &ty).map_err(|error| {
            let range = ty.kind.value_range();
            let reason = match error {
                ExactNumericConversionError::NegativeToUnsigned { .. } => {
                    "negative-to-unsigned"
                }
                ExactNumericConversionError::OutOfRange { .. } => "out-of-range",
            };
            self.err_invalid(format!(
                "[vm/numeric_dynamic_range] field={} declared_type={} value={} range={}..={} reason={}",
                field, declared_type_name, integer, range.min, range.max, reason
            ))
        })?;

        Ok(())
    }

    fn exact_numeric_runtime_check_contract_declared_type(
        &self,
        block: BasicBlockId,
        instruction_index: usize,
        field: &str,
        value: ValueId,
    ) -> Option<String> {
        let function = self
            .cur_fn
            .as_ref()
            .and_then(|function_name| self.functions.get(function_name))?;

        function
            .metadata
            .exact_numeric_runtime_check_contracts
            .iter()
            .find(|contract| {
                contract.kind == ExactNumericRuntimeCheckContractKind::DynamicIntegerRange
                    && contract.block == block
                    && contract.instruction_index == instruction_index
                    && contract.field == field
                    && contract.value == value
            })
            .map(|contract| contract.declared_type_name.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::mir_interpreter::MirInterpreter;
    use crate::backend::vm_types::VMValue;
    use crate::mir::{
        BasicBlockId, EffectMask, ExactNumericRuntimeCheckContract, FunctionSignature, MirFunction,
        MirInstruction, MirModule, MirType, UserBoxFieldDecl,
    };

    fn module_with_runtime_check_contract(declared_type_name: &str) -> MirModule {
        let entry = BasicBlockId::new(0);
        let signature = FunctionSignature {
            name: "Main.main/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, entry);
        let value_param = function.params[0];
        let object = function.next_value_id();

        function
            .metadata
            .exact_numeric_runtime_check_contracts
            .push(ExactNumericRuntimeCheckContract {
                block: entry,
                instruction_index: 1,
                field: "capacity".to_string(),
                value: value_param,
                declared_type_name: declared_type_name.to_string(),
                kind: ExactNumericRuntimeCheckContractKind::DynamicIntegerRange,
            });

        let block = function.get_block_mut(entry).unwrap();
        block.add_instruction(MirInstruction::NewBox {
            dst: object,
            box_type: "Page".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::FieldSet {
            base: object,
            field: "capacity".to_string(),
            value: value_param,
            declared_type: Some(MirType::Integer),
        });
        block.add_instruction(MirInstruction::Return { value: None });

        let mut module = MirModule::new("numeric_runtime_contract_test".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Page".to_string(), vec!["capacity".to_string()]);
        module.metadata.user_box_field_decls.insert(
            "Page".to_string(),
            vec![UserBoxFieldDecl {
                name: "capacity".to_string(),
                declared_type_name: Some(declared_type_name.to_string()),
                is_weak: false,
            }],
        );
        module.add_function(function);
        module
    }

    #[test]
    fn vm_executes_dynamic_integer_range_contract_for_usize_success() {
        let module = module_with_runtime_check_contract("usize");
        let mut vm = MirInterpreter::new();

        let result = vm.execute_function_with_args(&module, "Main.main/1", &[VMValue::Integer(42)]);

        assert!(result.is_ok());
    }

    #[test]
    fn vm_rejects_negative_dynamic_integer_for_usize_contract() {
        let module = module_with_runtime_check_contract("usize");
        let mut vm = MirInterpreter::new();

        let error = vm
            .execute_function_with_args(&module, "Main.main/1", &[VMValue::Integer(-1)])
            .expect_err("negative usize contract input must fail");

        assert!(error.to_string().contains("[vm/numeric_dynamic_range]"));
        assert!(error.to_string().contains("negative-to-unsigned"));
    }

    #[test]
    fn vm_rejects_non_integer_dynamic_value_for_numeric_contract() {
        let module = module_with_runtime_check_contract("usize");
        let mut vm = MirInterpreter::new();

        let error = vm
            .execute_function_with_args(
                &module,
                "Main.main/1",
                &[VMValue::String("oops".to_string())],
            )
            .expect_err("non-integer numeric contract input must fail");

        assert!(error
            .to_string()
            .contains("[vm/numeric_dynamic_range_type]"));
    }

    #[test]
    fn vm_rejects_out_of_range_dynamic_integer_for_u8_contract() {
        let module = module_with_runtime_check_contract("u8");
        let mut vm = MirInterpreter::new();

        let error = vm
            .execute_function_with_args(&module, "Main.main/1", &[VMValue::Integer(256)])
            .expect_err("u8 contract input above range must fail");

        assert!(error.to_string().contains("[vm/numeric_dynamic_range]"));
        assert!(error.to_string().contains("out-of-range"));
    }
}
