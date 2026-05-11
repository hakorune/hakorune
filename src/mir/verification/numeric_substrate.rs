use crate::mir::numeric_substrate::{
    exact_numeric_mir_type_from_declared_name, exact_numeric_value_from_dynamic_integer,
    ExactNumericConversionError, ExactNumericMirType, NumericTarget,
};
use crate::mir::verification_types::VerificationError;
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
            let Some(integer_value) = resolve_integer_const(*value, &integer_defs) else {
                continue;
            };

            if let Err(error) = exact_numeric_value_from_dynamic_integer(integer_value, ty) {
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
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, UserBoxFieldDecl,
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
    fn accepts_in_range_const_assignment_to_numeric_field() {
        let module = module_with_numeric_field("usize", field_set_function(42));

        assert!(check_exact_numeric_field_assignments(&module).is_ok());
    }
}
