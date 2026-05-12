use super::*;
use crate::mir::{MirModule, UserBoxFieldDecl};

fn module_with_metadata(metadata: ModuleMetadata) -> MirModule {
    let mut module = MirModule::new("typed_object_numeric_substrate_test".to_string());
    module.metadata = metadata;
    module
}

#[test]
fn build_typed_object_plans_accepts_numeric_substrate_type_names_as_exact_storage() {
    let mut metadata = ModuleMetadata::default();
    metadata.user_box_field_decls.insert(
        "Counters".to_string(),
        vec![
            UserBoxFieldDecl {
                name: "len".to_string(),
                declared_type_name: Some("usize".to_string()),
                is_weak: false,
            },
            UserBoxFieldDecl {
                name: "mask".to_string(),
                declared_type_name: Some("u64".to_string()),
                is_weak: false,
            },
            UserBoxFieldDecl {
                name: "delta".to_string(),
                declared_type_name: Some("i32".to_string()),
                is_weak: false,
            },
        ],
    );
    let module = module_with_metadata(metadata);

    let plans = build_typed_object_plans(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].box_name, "Counters");
    assert_eq!(plans[0].fields.len(), 3);
    assert_eq!(plans[0].fields[0].storage, TypedObjectFieldStorage::USize);
    assert_eq!(plans[0].fields[1].storage, TypedObjectFieldStorage::U64);
    assert_eq!(plans[0].fields[2].storage, TypedObjectFieldStorage::I32);
    assert_eq!(
        plans[0].fields[0].declared_type_name.as_deref(),
        Some("usize")
    );
    assert_eq!(
        plans[0].fields[1].declared_type_name.as_deref(),
        Some("u64")
    );
    assert_eq!(
        plans[0].fields[2].declared_type_name.as_deref(),
        Some("i32")
    );
}

#[test]
fn build_typed_object_plans_keeps_exact_declared_storage_with_integer_lane_observation() {
    let mut module = MirModule::new("typed_object_numeric_observed".to_string());
    module.metadata.user_box_field_decls.insert(
        "Counters".to_string(),
        vec![UserBoxFieldDecl {
            name: "len".to_string(),
            declared_type_name: Some("usize".to_string()),
            is_weak: false,
        }],
    );
    let mut function = crate::mir::MirFunction::new(
        crate::mir::FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: crate::mir::MirType::Void,
            effects: crate::mir::EffectMask::PURE,
        },
        crate::mir::BasicBlockId::new(0),
    );
    let obj = function.next_value_id();
    let value = function.next_value_id();
    let block = function
        .get_block_mut(crate::mir::BasicBlockId::new(0))
        .unwrap();
    block.add_instruction(crate::mir::MirInstruction::NewBox {
        dst: obj,
        box_type: "Counters".to_string(),
        args: vec![],
    });
    block.add_instruction(crate::mir::MirInstruction::Const {
        dst: value,
        value: crate::mir::ConstValue::Integer(7),
    });
    block.add_instruction(crate::mir::MirInstruction::FieldSet {
        base: obj,
        field: "len".to_string(),
        value,
        declared_type: Some(crate::mir::MirType::Integer),
    });
    module.add_function(function);

    let plans = build_typed_object_plans(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].fields[0].storage, TypedObjectFieldStorage::USize);
}
