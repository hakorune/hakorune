use super::*;
use crate::box_trait::NyashBox;
use crate::mir::{
    BasicBlock, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirModule, MirType,
    UserBoxFieldDecl, ValueId, WeakRefOp,
};
use std::collections::HashMap;
use std::sync::Arc;

fn module_with_function(function: MirFunction) -> MirModule {
    let mut module = MirModule::new("weak-field-runtime".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Node".to_string(), vec!["parent".to_string()]);
    module.metadata.user_box_field_decls.insert(
        "Node".to_string(),
        vec![UserBoxFieldDecl {
            name: "parent".to_string(),
            declared_type_name: None,
            is_weak: true,
        }],
    );
    module.add_function(function);
    module
}

fn function(instructions: Vec<MirInstruction>, return_type: MirType) -> MirFunction {
    let entry = BasicBlockId::new(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.test/0".to_string(),
            params: Vec::new(),
            return_type,
            effects: EffectMask::WRITE,
        },
        entry,
    );
    let mut block = BasicBlock::new(entry);
    for instruction in instructions {
        block.add_instruction(instruction);
    }
    function.add_block(block);
    function
}

#[test]
fn known_weak_field_write_reads_and_upgrades() {
    let base = ValueId::new(0);
    let target = ValueId::new(1);
    let weak = ValueId::new(2);
    let loaded_weak = ValueId::new(3);
    let loaded_strong = ValueId::new(4);
    let function = function(
        vec![
            MirInstruction::NewBox {
                dst: base,
                box_type: "Node".to_string(),
                args: Vec::new(),
            },
            MirInstruction::NewBox {
                dst: target,
                box_type: "Node".to_string(),
                args: Vec::new(),
            },
            MirInstruction::WeakRef {
                dst: weak,
                op: WeakRefOp::New,
                value: target,
            },
            MirInstruction::FieldSet {
                base,
                field: "parent".to_string(),
                value: weak,
                declared_type: None,
            },
            MirInstruction::FieldGet {
                dst: loaded_weak,
                base,
                field: "parent".to_string(),
                declared_type: None,
            },
            MirInstruction::WeakRef {
                dst: loaded_strong,
                op: WeakRefOp::Load,
                value: loaded_weak,
            },
            MirInstruction::Return {
                value: Some(loaded_strong),
            },
        ],
        MirType::Unknown,
    );

    let result = MirInterpreter::new()
        .execute_function_with_args(&module_with_function(function), "Main.test/0", &[])
        .expect("weak field write/read should succeed");

    assert!(matches!(result, VMValue::BoxRef(_)));
}

#[test]
fn weak_field_void_clear_reads_as_void() {
    let base = ValueId::new(0);
    let clear = ValueId::new(1);
    let loaded = ValueId::new(2);
    let function = function(
        vec![
            MirInstruction::NewBox {
                dst: base,
                box_type: "Node".to_string(),
                args: Vec::new(),
            },
            MirInstruction::Const {
                dst: clear,
                value: crate::mir::ConstValue::Void,
            },
            MirInstruction::FieldSet {
                base,
                field: "parent".to_string(),
                value: clear,
                declared_type: None,
            },
            MirInstruction::FieldGet {
                dst: loaded,
                base,
                field: "parent".to_string(),
                declared_type: None,
            },
            MirInstruction::Return {
                value: Some(loaded),
            },
        ],
        MirType::Void,
    );

    let result = MirInterpreter::new()
        .execute_function_with_args(&module_with_function(function), "Main.test/0", &[])
        .expect("Void clear should succeed");

    assert_eq!(result, VMValue::Void);
}

#[test]
fn known_strong_write_rejects_before_slot_mutation() {
    let base = ValueId::new(0);
    let target = ValueId::new(1);
    let function = function(
        vec![
            MirInstruction::NewBox {
                dst: base,
                box_type: "Node".to_string(),
                args: Vec::new(),
            },
            MirInstruction::NewBox {
                dst: target,
                box_type: "Node".to_string(),
                args: Vec::new(),
            },
            MirInstruction::FieldSet {
                base,
                field: "parent".to_string(),
                value: target,
                declared_type: None,
            },
            MirInstruction::Return { value: None },
        ],
        MirType::Void,
    );

    let error = MirInterpreter::new()
        .execute_function_with_args(&module_with_function(function), "Main.test/0", &[])
        .expect_err("strong write must reject");

    assert!(error
        .to_string()
        .contains(crate::runtime::weak_field::CONTRACT_VIOLATION_TAG));
}

#[test]
fn parameter_alias_write_uses_runtime_declaration_layout() {
    let base = ValueId::new(0);
    let value = ValueId::new(1);
    let entry = BasicBlockId::new(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.alias/2".to_string(),
            params: vec![MirType::Unknown, MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::WRITE,
        },
        entry,
    );
    function.params = vec![base, value];
    function.next_value_id = 2;
    let mut block = BasicBlock::new(entry);
    block.add_instruction(MirInstruction::FieldSet {
        base,
        field: "parent".to_string(),
        value,
        declared_type: None,
    });
    block.add_instruction(MirInstruction::Return { value: None });
    function.add_block(block);
    let module = module_with_function(function);
    let fields = vec![("parent".to_string(), true)];
    let typed = module.metadata.user_box_field_decls["Node"].clone();
    let fingerprint =
        crate::mir::type_contracts::weak_field::box_schema_fingerprint("Node", &typed);
    let instance: Arc<dyn NyashBox> =
        Arc::new(crate::instance_v2::InstanceBox::from_typed_declaration(
            "Node".to_string(),
            fields,
            fingerprint,
            HashMap::new(),
        ));

    let error = MirInterpreter::new()
        .execute_function_with_args(
            &module,
            "Main.alias/2",
            &[VMValue::BoxRef(instance), VMValue::Integer(1)],
        )
        .expect_err("dynamic alias strong write must reject");

    assert!(error
        .to_string()
        .contains(crate::runtime::weak_field::CONTRACT_VIOLATION_TAG));
}
