use std::collections::BTreeSet;

use crate::mir::function::{StaticDataPlan, TypedObjectPlan};
use crate::mir::typed_object_plan::TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0;
use crate::mir::verification_types::VerificationError;
use crate::mir::{MirInstruction, MirModule};

pub(super) fn check_module_metadata_invariants(
    module: &MirModule,
) -> Result<(), Vec<VerificationError>> {
    let mut errors = Vec::new();
    check_typed_object_plans(module, &mut errors);
    check_static_data_plans(module, &mut errors);
    check_static_data_loads(module, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn check_typed_object_plans(module: &MirModule, errors: &mut Vec<VerificationError>) {
    let mut box_names = BTreeSet::new();
    let mut type_ids = BTreeSet::new();
    for plan in &module.metadata.typed_object_plans {
        if plan.box_name.is_empty() {
            push_error(errors, "typed_object_plans", "<empty>", "box_name is empty");
        }
        if !box_names.insert(plan.box_name.clone()) {
            push_error(
                errors,
                "typed_object_plans",
                &plan.box_name,
                "duplicate typed object plan for box",
            );
        }
        if plan.type_id == 0 {
            push_error(
                errors,
                "typed_object_plans",
                &plan.box_name,
                "type_id must be nonzero",
            );
        } else if !type_ids.insert(plan.type_id) {
            push_error(
                errors,
                "typed_object_plans",
                &plan.box_name,
                &format!("duplicate type_id {}", plan.type_id),
            );
        }
        check_typed_object_plan_shape(plan, errors);
    }
}

fn check_typed_object_plan_shape(plan: &TypedObjectPlan, errors: &mut Vec<VerificationError>) {
    if plan.layout_kind != TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0 {
        push_error(
            errors,
            "typed_object_plans",
            &plan.box_name,
            &format!("unsupported layout_kind `{}`", plan.layout_kind),
        );
    }
    if plan.field_count as usize != plan.fields.len() {
        push_error(
            errors,
            "typed_object_plans",
            &plan.box_name,
            &format!(
                "field_count {} does not match fields.len {}",
                plan.field_count,
                plan.fields.len()
            ),
        );
    }

    let mut field_names = BTreeSet::new();
    let mut slots = BTreeSet::new();
    for field in &plan.fields {
        if field.name.is_empty() {
            push_error(
                errors,
                "typed_object_plans",
                &plan.box_name,
                "field name is empty",
            );
        }
        if !field_names.insert(field.name.clone()) {
            push_error(
                errors,
                "typed_object_plans",
                &plan.box_name,
                &format!("duplicate field `{}`", field.name),
            );
        }
        if field.slot >= plan.field_count {
            push_error(
                errors,
                "typed_object_plans",
                &plan.box_name,
                &format!("field `{}` slot {} is out of range", field.name, field.slot),
            );
        }
        if !slots.insert(field.slot) {
            push_error(
                errors,
                "typed_object_plans",
                &plan.box_name,
                &format!("duplicate slot {}", field.slot),
            );
        }
        if field.is_weak {
            push_error(
                errors,
                "typed_object_plans",
                &plan.box_name,
                &format!(
                    "field `{}` is weak; backend-active layout forbids weak fields",
                    field.name
                ),
            );
        }
    }

    for expected in 0..plan.field_count {
        if !slots.contains(&expected) {
            push_error(
                errors,
                "typed_object_plans",
                &plan.box_name,
                &format!("missing contiguous slot {}", expected),
            );
        }
    }
}

fn check_static_data_plans(module: &MirModule, errors: &mut Vec<VerificationError>) {
    let mut source_names = BTreeSet::new();
    let mut symbols = BTreeSet::new();
    for plan in &module.metadata.static_data_plans {
        if plan.source_name.is_empty() {
            push_error(
                errors,
                "static_data_plans",
                "<empty>",
                "source_name is empty",
            );
        }
        if plan.symbol.is_empty() {
            push_error(
                errors,
                "static_data_plans",
                &plan.source_name,
                "symbol is empty",
            );
        }
        if !source_names.insert(plan.source_name.clone()) {
            push_error(
                errors,
                "static_data_plans",
                &plan.source_name,
                "duplicate static data source_name",
            );
        }
        if !symbols.insert(plan.symbol.clone()) {
            push_error(
                errors,
                "static_data_plans",
                &plan.source_name,
                &format!("duplicate static data symbol `{}`", plan.symbol),
            );
        }
        check_static_data_plan_shape(plan, errors);
    }
}

fn check_static_data_plan_shape(plan: &StaticDataPlan, errors: &mut Vec<VerificationError>) {
    let Some(expected_align) = static_data_alignment(&plan.element) else {
        push_error(
            errors,
            "static_data_plans",
            &plan.source_name,
            &format!("unsupported element `{}`", plan.element),
        );
        return;
    };
    if plan.align != expected_align {
        push_error(
            errors,
            "static_data_plans",
            &plan.source_name,
            &format!(
                "align {} does not match element `{}` expected {}",
                plan.align, plan.element, expected_align
            ),
        );
    }
    let Some(max_value) = static_data_max_value(&plan.element) else {
        return;
    };
    for (index, value) in plan.values.iter().enumerate() {
        if *value > max_value {
            push_error(
                errors,
                "static_data_plans",
                &plan.source_name,
                &format!(
                    "value at index {} is {} but `{}` max is {}",
                    index, value, plan.element, max_value
                ),
            );
        }
    }
}

fn check_static_data_loads(module: &MirModule, errors: &mut Vec<VerificationError>) {
    for function in module.functions.values() {
        for block in function.blocks.values() {
            for instruction in &block.instructions {
                let MirInstruction::StaticDataLoad {
                    source_name,
                    symbol,
                    element,
                    len,
                    align,
                    ..
                } = instruction
                else {
                    continue;
                };
                if element != "u16" {
                    push_error(
                        errors,
                        "static_data_load",
                        source_name,
                        &format!("unsupported load element `{}`", element),
                    );
                }
                let Some(plan) = module
                    .metadata
                    .static_data_plans
                    .iter()
                    .find(|plan| plan.source_name == *source_name && plan.symbol == *symbol)
                else {
                    push_error(
                        errors,
                        "static_data_load",
                        source_name,
                        &format!("missing static_data_plans row for symbol `{}`", symbol),
                    );
                    continue;
                };
                if plan.element != *element {
                    push_error(
                        errors,
                        "static_data_load",
                        source_name,
                        &format!(
                            "element `{}` does not match plan element `{}`",
                            element, plan.element
                        ),
                    );
                }
                if plan.align != *align {
                    push_error(
                        errors,
                        "static_data_load",
                        source_name,
                        &format!("align {} does not match plan align {}", align, plan.align),
                    );
                }
                if plan.values.len() != *len as usize {
                    push_error(
                        errors,
                        "static_data_load",
                        source_name,
                        &format!("len {} does not match plan len {}", len, plan.values.len()),
                    );
                }
            }
        }
    }
}

fn static_data_alignment(element: &str) -> Option<u32> {
    match element {
        "u8" => Some(1),
        "u16" => Some(2),
        "u32" => Some(4),
        "u64" => Some(8),
        _ => None,
    }
}

fn static_data_max_value(element: &str) -> Option<u64> {
    match element {
        "u8" => Some(u8::MAX as u64),
        "u16" => Some(u16::MAX as u64),
        "u32" => Some(u32::MAX as u64),
        "u64" => Some(u64::MAX),
        _ => None,
    }
}

fn push_error(errors: &mut Vec<VerificationError>, key: &'static str, owner: &str, reason: &str) {
    errors.push(VerificationError::ModuleMetadataInvariantViolation {
        key,
        owner: owner.to_string(),
        reason: reason.to_string(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::{
        FunctionSignature, StaticDataPlan, TypedObjectFieldPlan, TypedObjectFieldStorage,
        TypedObjectPlan,
    };
    use crate::mir::{
        BasicBlock, BasicBlockId, EffectMask, MirFunction, MirInstruction, MirModule, MirType,
        ValueId,
    };

    fn typed_plan() -> TypedObjectPlan {
        TypedObjectPlan {
            box_name: "Pair".to_string(),
            type_id: 41,
            layout_kind: TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0.to_string(),
            field_count: 2,
            fields: vec![
                TypedObjectFieldPlan {
                    name: "left".to_string(),
                    slot: 0,
                    declared_type_name: Some("i64".to_string()),
                    storage: TypedObjectFieldStorage::I64,
                    is_weak: false,
                },
                TypedObjectFieldPlan {
                    name: "right".to_string(),
                    slot: 1,
                    declared_type_name: Some("i64".to_string()),
                    storage: TypedObjectFieldStorage::I64,
                    is_weak: false,
                },
            ],
        }
    }

    fn static_plan() -> StaticDataPlan {
        StaticDataPlan {
            source_name: "SIZE_CLASS".to_string(),
            symbol: ".hako.static.SIZE_CLASS".to_string(),
            element: "u16".to_string(),
            align: 2,
            linkage: "private".to_string(),
            unnamed_addr: true,
            values: vec![8, 16, 24],
        }
    }

    fn module_with_valid_metadata() -> MirModule {
        let mut module = MirModule::new("module-metadata-test".to_string());
        module.metadata.typed_object_plans.push(typed_plan());
        module.metadata.static_data_plans.push(static_plan());

        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: Vec::new(),
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: crate::mir::ConstValue::Integer(0),
        });
        block.add_instruction(MirInstruction::StaticDataLoad {
            dst: ValueId::new(2),
            source_name: "SIZE_CLASS".to_string(),
            symbol: ".hako.static.SIZE_CLASS".to_string(),
            element: "u16".to_string(),
            len: 3,
            align: 2,
            index: ValueId::new(1),
        });
        function.add_block(block);
        module.add_function(function);
        module
    }

    fn first_reason(module: &MirModule) -> String {
        let errors = check_module_metadata_invariants(module).unwrap_err();
        match &errors[0] {
            VerificationError::ModuleMetadataInvariantViolation { reason, .. } => reason.clone(),
            other => panic!("unexpected verifier error: {:?}", other),
        }
    }

    #[test]
    fn verifier_accepts_valid_module_metadata() {
        let module = module_with_valid_metadata();
        assert!(check_module_metadata_invariants(&module).is_ok());
    }

    #[test]
    fn verifier_rejects_typed_object_slot_drift() {
        let mut module = module_with_valid_metadata();
        module.metadata.typed_object_plans[0].fields[1].slot = 0;

        let reason = first_reason(&module);
        assert!(reason.contains("duplicate slot"));
    }

    #[test]
    fn verifier_rejects_typed_object_field_count_drift() {
        let mut module = module_with_valid_metadata();
        module.metadata.typed_object_plans[0].field_count = 3;

        let reason = first_reason(&module);
        assert!(reason.contains("field_count"));
    }

    #[test]
    fn verifier_rejects_static_data_value_overflow() {
        let mut module = module_with_valid_metadata();
        module.metadata.static_data_plans[0].values[1] = u16::MAX as u64 + 1;

        let reason = first_reason(&module);
        assert!(reason.contains("max"));
    }

    #[test]
    fn verifier_rejects_static_data_load_plan_mismatch() {
        let mut module = module_with_valid_metadata();
        module.metadata.static_data_plans[0].values.push(32);

        let reason = first_reason(&module);
        assert!(reason.contains("plan len"));
    }
}
