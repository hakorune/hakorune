use crate::mir::numeric_substrate::{
    exact_numeric_mir_type_from_declared_name,
    exact_numeric_type_requires_dynamic_integer_range_check,
    exact_numeric_value_from_dynamic_integer, ExactNumericConversionError, ExactNumericMirType,
    NumericTarget,
};
use crate::mir::verification_types::VerificationError;
use crate::mir::ExactNumericRuntimeCheckContractKind;
use crate::mir::{ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueId};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
enum ObjectDef {
    Box(String),
    Copy(ValueId),
}

#[derive(Debug, Clone, Copy)]
enum IntegerDef {
    Const(i64),
    Copy(ValueId),
}

#[derive(Debug, Clone)]
enum ValueProducer {
    Param,
    ConstInteger,
    ConstNonInteger,
    Copy(ValueId),
    Dynamic(&'static str),
}

pub(super) fn check_exact_numeric_field_assignments(
    module: &MirModule,
) -> Result<(), Vec<VerificationError>> {
    let fields = exact_numeric_field_decls(module);
    if fields.is_empty() {
        return Ok(());
    }

    let mut errors = Vec::new();
    for function in module.functions.values() {
        check_function(function, &fields, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn exact_numeric_field_decls(
    module: &MirModule,
) -> BTreeMap<(String, String), ExactNumericMirType> {
    let mut fields = BTreeMap::new();
    let target = NumericTarget::host();

    for (box_name, decls) in &module.metadata.user_box_field_decls {
        for decl in decls {
            if let Some(ty) = exact_numeric_mir_type_from_declared_name(
                decl.declared_type_name.as_deref(),
                target,
            ) {
                fields.insert((box_name.clone(), decl.name.clone()), ty);
            }
        }
    }

    fields
}

fn check_function(
    function: &MirFunction,
    fields: &BTreeMap<(String, String), ExactNumericMirType>,
    errors: &mut Vec<VerificationError>,
) {
    let object_defs = collect_object_defs(function);
    let integer_defs = collect_integer_defs(function);
    let value_producers = collect_value_producers(function);

    for (block, basic_block) in &function.blocks {
        for (instruction_index, spanned) in basic_block.all_spanned_instructions_enumerated() {
            let MirInstruction::FieldSet {
                base, field, value, ..
            } = spanned.inst
            else {
                continue;
            };

            let Some(box_name) = resolve_object_box(*base, &object_defs) else {
                continue;
            };
            let Some(ty) = fields.get(&(box_name.clone(), field.clone())) else {
                continue;
            };
            match resolve_integer_const(*value, &integer_defs) {
                Some(integer_value) => {
                    if let Err(error) = exact_numeric_value_from_dynamic_integer(integer_value, ty)
                    {
                        errors.push(exact_numeric_violation(
                            function,
                            *block,
                            instruction_index,
                            box_name,
                            field.clone(),
                            integer_value,
                            ty,
                            error,
                        ));
                    }
                }
                None => {
                    if exact_numeric_type_requires_dynamic_integer_range_check(ty)
                        && !has_runtime_check_contract(
                            function,
                            *block,
                            instruction_index,
                            field,
                            *value,
                            ty,
                        )
                    {
                        errors.push(exact_numeric_dynamic_check_required(
                            function,
                            *block,
                            instruction_index,
                            box_name,
                            field.clone(),
                            *value,
                            ty,
                            resolve_value_producer_label(*value, &value_producers),
                        ));
                    }
                }
            }
        }
    }
}

fn has_runtime_check_contract(
    function: &MirFunction,
    block: crate::mir::BasicBlockId,
    instruction_index: usize,
    field: &str,
    value: ValueId,
    ty: &ExactNumericMirType,
) -> bool {
    function
        .metadata
        .exact_numeric_runtime_check_contracts
        .iter()
        .any(|contract| {
            contract.kind == ExactNumericRuntimeCheckContractKind::DynamicIntegerRange
                && contract.block == block
                && contract.instruction_index == instruction_index
                && contract.field == field
                && contract.value == value
                && contract.declared_type_name == ty.source_name
        })
}

fn collect_object_defs(function: &MirFunction) -> HashMap<ValueId, ObjectDef> {
    let mut defs = HashMap::new();

    for (idx, param) in function.params.iter().enumerate() {
        if let Some(MirType::Box(box_name)) = function.signature.params.get(idx) {
            defs.insert(*param, ObjectDef::Box(box_name.clone()));
        }
    }

    for block in function.blocks.values() {
        for spanned in block.all_spanned_instructions() {
            match spanned.inst {
                MirInstruction::NewBox { dst, box_type, .. } => {
                    defs.insert(*dst, ObjectDef::Box(box_type.clone()));
                }
                MirInstruction::Copy { dst, src } => {
                    defs.insert(*dst, ObjectDef::Copy(*src));
                }
                _ => {}
            }
        }
    }

    defs
}

fn collect_integer_defs(function: &MirFunction) -> HashMap<ValueId, IntegerDef> {
    let mut defs = HashMap::new();

    for block in function.blocks.values() {
        for spanned in block.all_spanned_instructions() {
            match spanned.inst {
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(value),
                } => {
                    defs.insert(*dst, IntegerDef::Const(*value));
                }
                MirInstruction::Copy { dst, src } => {
                    defs.insert(*dst, IntegerDef::Copy(*src));
                }
                _ => {}
            }
        }
    }

    defs
}

fn collect_value_producers(function: &MirFunction) -> HashMap<ValueId, ValueProducer> {
    let mut producers = HashMap::new();

    for param in &function.params {
        producers.insert(*param, ValueProducer::Param);
    }

    for block in function.blocks.values() {
        for spanned in block.all_spanned_instructions() {
            match spanned.inst {
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(_),
                } => {
                    producers.insert(*dst, ValueProducer::ConstInteger);
                }
                MirInstruction::Const { dst, .. } => {
                    producers.insert(*dst, ValueProducer::ConstNonInteger);
                }
                MirInstruction::Copy { dst, src } => {
                    producers.insert(*dst, ValueProducer::Copy(*src));
                }
                MirInstruction::BinOp { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("binop"));
                }
                MirInstruction::UnaryOp { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("unaryop"));
                }
                MirInstruction::Compare { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("compare"));
                }
                MirInstruction::Load { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("load"));
                }
                MirInstruction::StaticDataLoad { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("static_data_load"));
                }
                MirInstruction::FieldGet { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("field_get"));
                }
                MirInstruction::VariantMake { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("variant_make"));
                }
                MirInstruction::VariantTag { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("variant_tag"));
                }
                MirInstruction::VariantProject { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("variant_project"));
                }
                MirInstruction::Call { dst: Some(dst), .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("call"));
                }
                MirInstruction::NewClosure { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("new_closure"));
                }
                MirInstruction::Phi { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("phi"));
                }
                MirInstruction::NewBox { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("new_box"));
                }
                MirInstruction::TypeOp { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("typeop"));
                }
                MirInstruction::Catch {
                    exception_value, ..
                } => {
                    producers.insert(*exception_value, ValueProducer::Dynamic("catch"));
                }
                MirInstruction::RefNew { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("ref_new"));
                }
                MirInstruction::WeakRef { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("weakref"));
                }
                MirInstruction::FutureNew { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("future_new"));
                }
                MirInstruction::Await { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("await"));
                }
                MirInstruction::Select { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("select"));
                }
                _ => {}
            }
        }
    }

    producers
}

fn resolve_object_box(value: ValueId, defs: &HashMap<ValueId, ObjectDef>) -> Option<String> {
    let mut current = value;
    for _ in 0..16 {
        match defs.get(&current)? {
            ObjectDef::Box(box_name) => return Some(box_name.clone()),
            ObjectDef::Copy(src) => current = *src,
        }
    }
    None
}

fn resolve_integer_const(value: ValueId, defs: &HashMap<ValueId, IntegerDef>) -> Option<i64> {
    let mut current = value;
    for _ in 0..16 {
        match defs.get(&current)? {
            IntegerDef::Const(value) => return Some(*value),
            IntegerDef::Copy(src) => current = *src,
        }
    }
    None
}

fn resolve_value_producer_label(
    value: ValueId,
    producers: &HashMap<ValueId, ValueProducer>,
) -> String {
    let mut current = value;
    for _ in 0..16 {
        match producers.get(&current) {
            Some(ValueProducer::Param) => return "param".to_string(),
            Some(ValueProducer::ConstInteger) => return "const_integer_unresolved".to_string(),
            Some(ValueProducer::ConstNonInteger) => return "const_non_integer".to_string(),
            Some(ValueProducer::Dynamic(label)) => return (*label).to_string(),
            Some(ValueProducer::Copy(src)) => current = *src,
            None => return "unknown".to_string(),
        }
    }
    "copy_chain_too_deep".to_string()
}

fn exact_numeric_violation(
    function: &MirFunction,
    block: crate::mir::BasicBlockId,
    instruction_index: usize,
    box_name: String,
    field: String,
    value: i64,
    ty: &ExactNumericMirType,
    error: ExactNumericConversionError,
) -> VerificationError {
    let range = ty.kind.value_range();
    let reason = match error {
        ExactNumericConversionError::NegativeToUnsigned { .. } => "negative-to-unsigned",
        ExactNumericConversionError::OutOfRange { .. } => "out-of-range",
    };

    VerificationError::ExactNumericRangeViolation {
        function: function.signature.name.clone(),
        block,
        instruction_index,
        box_name,
        field,
        declared_type_name: ty.source_name.clone(),
        value: i128::from(value),
        min: range.min,
        max: range.max,
        reason: reason.to_string(),
    }
}

fn exact_numeric_dynamic_check_required(
    function: &MirFunction,
    block: crate::mir::BasicBlockId,
    instruction_index: usize,
    box_name: String,
    field: String,
    value: ValueId,
    ty: &ExactNumericMirType,
    producer: String,
) -> VerificationError {
    VerificationError::ExactNumericDynamicCheckRequired {
        function: function.signature.name.clone(),
        block,
        instruction_index,
        box_name,
        field,
        declared_type_name: ty.source_name.clone(),
        value,
        producer,
        reason: "dynamic-integer-range-check-required".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlockId, BinaryOp, EffectMask, ExactNumericRuntimeCheckContract,
        ExactNumericRuntimeCheckContractKind, FunctionSignature, MirFunction, MirModule,
        UserBoxFieldDecl,
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
