use crate::mir::exact_numeric_field_contracts::{
    collect_exact_numeric_field_assignment_findings, ExactNumericFieldAssignmentFinding,
};
use crate::mir::verification_types::VerificationError;
use crate::mir::MirModule;

pub(super) fn check_exact_numeric_field_assignments(
    module: &MirModule,
) -> Result<(), Vec<VerificationError>> {
    let errors: Vec<VerificationError> = collect_exact_numeric_field_assignment_findings(module)
        .into_iter()
        .map(verification_error_from_finding)
        .collect();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn verification_error_from_finding(
    finding: ExactNumericFieldAssignmentFinding,
) -> VerificationError {
    match finding {
        ExactNumericFieldAssignmentFinding::RangeViolation(site) => {
            VerificationError::ExactNumericRangeViolation {
                function: site.function,
                block: site.block,
                instruction_index: site.instruction_index,
                box_name: site.box_name,
                field: site.field,
                declared_type_name: site.declared_type_name,
                value: site.value,
                min: site.min,
                max: site.max,
                reason: site.reason,
            }
        }
        ExactNumericFieldAssignmentFinding::DynamicCheckRequired(site) => {
            VerificationError::ExactNumericDynamicCheckRequired {
                function: site.function,
                block: site.block,
                instruction_index: site.instruction_index,
                box_name: site.box_name,
                field: site.field,
                declared_type_name: site.declared_type_name,
                value: site.value,
                producer: site.producer,
                reason: site.reason,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::{
        ExactNumericRuntimeCheckContract, ExactNumericRuntimeCheckContractKind,
    };
    use crate::mir::{
        BasicBlockId, BinaryOp, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirModule, MirType, UserBoxFieldDecl, ValueId,
    };

    fn module_with_numeric_field(declared_type_name: &str, function: MirFunction) -> MirModule {
        let mut module = MirModule::new("test".to_string());
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

    fn field_set_function(value: i64) -> MirFunction {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut function = MirFunction::new(signature, entry);
        let object = function.next_value_id();
        let constant = function.next_value_id();

        let block = function.get_block_mut(entry).unwrap();
        block.add_instruction(MirInstruction::NewBox {
            dst: object,
            box_type: "Page".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: constant,
            value: ConstValue::Integer(value),
        });
        block.add_instruction(MirInstruction::FieldSet {
            base: object,
            field: "capacity".to_string(),
            value: constant,
            declared_type: Some(MirType::Integer),
        });
        block.add_instruction(MirInstruction::Return { value: None });

        function
    }

    fn field_set_function_with_copies(value: i64) -> MirFunction {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut function = MirFunction::new(signature, entry);
        let object = function.next_value_id();
        let object_copy = function.next_value_id();
        let constant = function.next_value_id();
        let value_copy = function.next_value_id();

        let block = function.get_block_mut(entry).unwrap();
        block.add_instruction(MirInstruction::NewBox {
            dst: object,
            box_type: "Page".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Copy {
            dst: object_copy,
            src: object,
        });
        block.add_instruction(MirInstruction::Const {
            dst: constant,
            value: ConstValue::Integer(value),
        });
        block.add_instruction(MirInstruction::Copy {
            dst: value_copy,
            src: constant,
        });
        block.add_instruction(MirInstruction::FieldSet {
            base: object_copy,
            field: "capacity".to_string(),
            value: value_copy,
            declared_type: Some(MirType::Integer),
        });
        block.add_instruction(MirInstruction::Return { value: None });

        function
    }

    fn field_set_on_box_param_function(value: i64) -> MirFunction {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("Page".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut function = MirFunction::new(signature, entry);
        let object_param = function.params[0];
        let constant = function.next_value_id();

        let block = function.get_block_mut(entry).unwrap();
        block.add_instruction(MirInstruction::Const {
            dst: constant,
            value: ConstValue::Integer(value),
        });
        block.add_instruction(MirInstruction::FieldSet {
            base: object_param,
            field: "capacity".to_string(),
            value: constant,
            declared_type: Some(MirType::Integer),
        });
        block.add_instruction(MirInstruction::Return { value: None });

        function
    }

    fn field_set_param_value_function() -> MirFunction {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut function = MirFunction::new(signature, entry);
        let value_param = function.params[0];
        let object = function.next_value_id();

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

        function
    }

    fn field_set_param_value_function_with_runtime_check_contract() -> MirFunction {
        let mut function = field_set_param_value_function();
        function
            .metadata
            .exact_numeric_runtime_check_contracts
            .push(ExactNumericRuntimeCheckContract {
                block: BasicBlockId::new(0),
                instruction_index: 1,
                field: "capacity".to_string(),
                value: ValueId::new(0),
                declared_type_name: "usize".to_string(),
                kind: ExactNumericRuntimeCheckContractKind::DynamicIntegerRange,
            });
        function
    }

    fn field_set_param_value_function_with_mismatched_runtime_check_contract() -> MirFunction {
        let mut function = field_set_param_value_function();
        function
            .metadata
            .exact_numeric_runtime_check_contracts
            .push(ExactNumericRuntimeCheckContract {
                block: BasicBlockId::new(0),
                instruction_index: 1,
                field: "capacity".to_string(),
                value: ValueId::new(0),
                declared_type_name: "u8".to_string(),
                kind: ExactNumericRuntimeCheckContractKind::DynamicIntegerRange,
            });
        function
    }

    fn field_set_binop_value_function() -> MirFunction {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut function = MirFunction::new(signature, entry);
        let object = function.next_value_id();
        let lhs = function.next_value_id();
        let rhs = function.next_value_id();
        let sum = function.next_value_id();

        let block = function.get_block_mut(entry).unwrap();
        block.add_instruction(MirInstruction::NewBox {
            dst: object,
            box_type: "Page".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: lhs,
            value: ConstValue::Integer(1),
        });
        block.add_instruction(MirInstruction::Const {
            dst: rhs,
            value: ConstValue::Integer(2),
        });
        block.add_instruction(MirInstruction::BinOp {
            dst: sum,
            op: BinaryOp::Add,
            lhs,
            rhs,
        });
        block.add_instruction(MirInstruction::FieldSet {
            base: object,
            field: "capacity".to_string(),
            value: sum,
            declared_type: Some(MirType::Integer),
        });
        block.add_instruction(MirInstruction::Return { value: None });

        function
    }

    fn field_set_call_value_function() -> MirFunction {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut function = MirFunction::new(signature, entry);
        let object = function.next_value_id();
        let func = function.next_value_id();
        let call_value = function.next_value_id();

        let block = function.get_block_mut(entry).unwrap();
        block.add_instruction(MirInstruction::NewBox {
            dst: object,
            box_type: "Page".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: func,
            value: ConstValue::String("dynamic_source".to_string()),
        });
        block.add_instruction(MirInstruction::Call {
            dst: Some(call_value),
            func,
            callee: None,
            args: vec![],
            effects: EffectMask::PURE,
        });
        block.add_instruction(MirInstruction::FieldSet {
            base: object,
            field: "capacity".to_string(),
            value: call_value,
            declared_type: Some(MirType::Integer),
        });
        block.add_instruction(MirInstruction::Return { value: None });

        function
    }

    #[test]
    fn rejects_negative_const_assignment_to_usize_field() {
        let module = module_with_numeric_field("usize", field_set_function(-1));
        let errors = check_exact_numeric_field_assignments(&module).unwrap_err();

        assert_eq!(errors.len(), 1);
        assert!(matches!(
            &errors[0],
            VerificationError::ExactNumericRangeViolation {
                declared_type_name,
                value,
                reason,
                ..
            } if declared_type_name == "usize" && *value == -1 && reason == "negative-to-unsigned"
        ));
    }

    #[test]
    fn rejects_negative_assignment_through_copy_chains() {
        let module = module_with_numeric_field("usize", field_set_function_with_copies(-1));
        let errors = check_exact_numeric_field_assignments(&module).unwrap_err();

        assert!(matches!(
            &errors[0],
            VerificationError::ExactNumericRangeViolation {
                declared_type_name,
                value,
                reason,
                ..
            } if declared_type_name == "usize" && *value == -1 && reason == "negative-to-unsigned"
        ));
    }

    #[test]
    fn rejects_negative_assignment_to_box_typed_param_field() {
        let module = module_with_numeric_field("usize", field_set_on_box_param_function(-1));
        let errors = check_exact_numeric_field_assignments(&module).unwrap_err();

        assert!(matches!(
            &errors[0],
            VerificationError::ExactNumericRangeViolation {
                declared_type_name,
                value,
                reason,
                ..
            } if declared_type_name == "usize" && *value == -1 && reason == "negative-to-unsigned"
        ));
    }

    #[test]
    fn rejects_out_of_range_const_assignment_to_u8_field() {
        let module = module_with_numeric_field("u8", field_set_function(256));
        let errors = check_exact_numeric_field_assignments(&module).unwrap_err();

        assert!(matches!(
            &errors[0],
            VerificationError::ExactNumericRangeViolation {
                declared_type_name,
                value,
                min,
                max,
                reason,
                ..
            } if declared_type_name == "u8"
                && *value == 256
                && *min == 0
                && *max == 255
                && reason == "out-of-range"
        ));
    }

    #[test]
    fn rejects_dynamic_param_assignment_to_usize_field_until_runtime_check_exists() {
        let module = module_with_numeric_field("usize", field_set_param_value_function());
        let errors = check_exact_numeric_field_assignments(&module).unwrap_err();

        assert!(matches!(
            &errors[0],
            VerificationError::ExactNumericDynamicCheckRequired {
                declared_type_name,
                producer,
                reason,
                ..
            } if declared_type_name == "usize"
                && producer == "param"
                && reason == "dynamic-integer-range-check-required"
        ));
    }

    #[test]
    fn rejects_dynamic_binop_assignment_to_usize_field_until_runtime_check_exists() {
        let module = module_with_numeric_field("usize", field_set_binop_value_function());
        let errors = check_exact_numeric_field_assignments(&module).unwrap_err();

        assert!(matches!(
            &errors[0],
            VerificationError::ExactNumericDynamicCheckRequired {
                declared_type_name,
                producer,
                reason,
                ..
            } if declared_type_name == "usize"
                && producer == "binop"
                && reason == "dynamic-integer-range-check-required"
        ));
    }

    #[test]
    fn rejects_dynamic_call_assignment_to_usize_field_until_runtime_check_exists() {
        let module = module_with_numeric_field("usize", field_set_call_value_function());
        let errors = check_exact_numeric_field_assignments(&module).unwrap_err();

        assert!(matches!(
            &errors[0],
            VerificationError::ExactNumericDynamicCheckRequired {
                declared_type_name,
                producer,
                reason,
                ..
            } if declared_type_name == "usize"
                && producer == "call"
                && reason == "dynamic-integer-range-check-required"
        ));
    }

    #[test]
    fn accepts_dynamic_param_assignment_to_usize_field_with_runtime_check_contract() {
        let module = module_with_numeric_field(
            "usize",
            field_set_param_value_function_with_runtime_check_contract(),
        );

        assert!(check_exact_numeric_field_assignments(&module).is_ok());
    }

    #[test]
    fn rejects_dynamic_param_assignment_when_runtime_check_contract_decl_type_mismatches() {
        let module = module_with_numeric_field(
            "usize",
            field_set_param_value_function_with_mismatched_runtime_check_contract(),
        );
        let errors = check_exact_numeric_field_assignments(&module).unwrap_err();

        assert!(matches!(
            &errors[0],
            VerificationError::ExactNumericDynamicCheckRequired {
                declared_type_name,
                producer,
                ..
            } if declared_type_name == "usize" && producer == "param"
        ));
    }

    #[test]
    fn accepts_dynamic_param_assignment_to_i64_field_without_range_gap() {
        let module = module_with_numeric_field("i64", field_set_param_value_function());

        assert!(check_exact_numeric_field_assignments(&module).is_ok());
    }

    #[test]
    fn accepts_in_range_const_assignment_to_numeric_field() {
        let module = module_with_numeric_field("usize", field_set_function(42));

        assert!(check_exact_numeric_field_assignments(&module).is_ok());
    }
}
