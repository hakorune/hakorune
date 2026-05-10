use super::*;
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, FunctionSignature,
    MirInstruction, MirType, UserBoxFieldDecl, ValueId,
};
use std::collections::BTreeMap;

#[path = "runtime_receiver_and_collection.rs"]
mod runtime_receiver_and_collection;

fn module_with_metadata(metadata: ModuleMetadata) -> MirModule {
    let mut module = MirModule::new("typed_object_plan_test".to_string());
    module.metadata = metadata;
    module
}

#[test]
fn build_typed_object_plans_accepts_nonweak_i64_fields() {
    let mut metadata = ModuleMetadata::default();
    metadata.user_box_field_decls.insert(
        "Pair".to_string(),
        vec![
            UserBoxFieldDecl {
                name: "left".to_string(),
                declared_type_name: Some("IntegerBox".to_string()),
                is_weak: false,
            },
            UserBoxFieldDecl {
                name: "right".to_string(),
                declared_type_name: Some("Integer".to_string()),
                is_weak: false,
            },
        ],
    );
    let module = module_with_metadata(metadata);

    let plans = build_typed_object_plans(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].box_name, "Pair");
    assert_eq!(plans[0].type_id, 1);
    assert_eq!(
        plans[0].layout_kind,
        TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0
    );
    assert_eq!(plans[0].field_count, 2);
    assert_eq!(plans[0].fields[0].slot, 0);
    assert_eq!(plans[0].fields[0].storage, TypedObjectFieldStorage::I64);
    assert_eq!(plans[0].fields[1].slot, 1);
}

#[test]
fn build_typed_object_plans_rejects_weak_or_unknown_storage() {
    let mut metadata = ModuleMetadata::default();
    metadata.user_box_field_decls.insert(
        "WeakBox".to_string(),
        vec![UserBoxFieldDecl {
            name: "next".to_string(),
            declared_type_name: Some("IntegerBox".to_string()),
            is_weak: true,
        }],
    );
    metadata.user_box_field_decls.insert(
        "AnyBox".to_string(),
        vec![UserBoxFieldDecl {
            name: "value".to_string(),
            declared_type_name: None,
            is_weak: false,
        }],
    );
    let module = module_with_metadata(metadata);

    let plans = build_typed_object_plans(&module);

    assert!(plans.is_empty());
}

#[test]
fn build_typed_object_plans_infers_untyped_i64_and_handle_fields() {
    let mut module = MirModule::new("typed_object_infer".to_string());
    module.metadata.user_box_field_decls.insert(
        "Holder".to_string(),
        vec![
            UserBoxFieldDecl {
                name: "count".to_string(),
                declared_type_name: None,
                is_weak: false,
            },
            UserBoxFieldDecl {
                name: "items".to_string(),
                declared_type_name: None,
                is_weak: false,
            },
        ],
    );

    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "Holder".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(7),
    });
    block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(1),
        field: "count".to_string(),
        value: ValueId::new(2),
        declared_type: None,
    });
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(3),
        box_type: "ArrayBox".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(1),
        field: "items".to_string(),
        value: ValueId::new(3),
        declared_type: None,
    });
    function.add_block(block);
    module.add_function(function);

    let plans = build_typed_object_plans(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].box_name, "Holder");
    assert_eq!(plans[0].fields[0].storage, TypedObjectFieldStorage::I64);
    assert_eq!(plans[0].fields[1].storage, TypedObjectFieldStorage::Handle);
}

#[test]
fn build_typed_object_plans_uses_phi_value_type_before_input_walk() {
    let mut module = MirModule::new("typed_object_phi_metadata".to_string());
    module.metadata.user_box_field_decls.insert(
        "Holder".to_string(),
        vec![UserBoxFieldDecl {
            name: "count".to_string(),
            declared_type_name: None,
            is_weak: false,
        }],
    );

    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    let block_id = BasicBlockId::new(0);
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "Holder".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(1),
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(2),
    });

    let mut left = ValueId::new(2);
    let mut right = ValueId::new(3);
    for offset in 0..40 {
        let dst = ValueId::new(10 + offset);
        block.add_instruction(MirInstruction::Phi {
            dst,
            inputs: vec![(block_id, left), (block_id, right)],
            type_hint: None,
        });
        function.metadata.value_types.insert(dst, MirType::Integer);
        left = right;
        right = dst;
    }
    block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(1),
        field: "count".to_string(),
        value: right,
        declared_type: None,
    });
    function.add_block(block);
    module.add_function(function);

    let plans = build_typed_object_plans(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].box_name, "Holder");
    assert_eq!(plans[0].fields[0].storage, TypedObjectFieldStorage::I64);
}

#[test]
fn box_origin_for_value_uses_phi_value_type_before_input_walk() {
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    let block_id = BasicBlockId::new(0);
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "Cell".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(2),
        box_type: "OtherCell".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(3),
        inputs: vec![(block_id, ValueId::new(1)), (block_id, ValueId::new(2))],
        type_hint: None,
    });
    function
        .metadata
        .value_types
        .insert(ValueId::new(3), MirType::Box("Cell".to_string()));
    function.add_block(block);

    let module = MirModule::new("typed_object_phi_box_origin".to_string());
    let def_map = crate::mir::value_origin::build_value_def_map(&function);

    assert_eq!(
        box_origin_for_value(
            &module,
            &function,
            &def_map,
            ValueId::new(3),
            &BTreeMap::new(),
            &BTreeMap::new(),
        ),
        Some("Cell".to_string())
    );
}

#[test]
fn build_typed_object_plans_infers_birth_param_field_storage_from_newbox_args() {
    let mut module = MirModule::new("typed_object_birth_param".to_string());
    module.metadata.user_box_decls.insert(
        "Page".to_string(),
        vec!["page_id".to_string(), "capacity".to_string()],
    );
    module.metadata.user_box_field_decls.insert(
        "Page".to_string(),
        vec![
            UserBoxFieldDecl {
                name: "page_id".to_string(),
                declared_type_name: None,
                is_weak: false,
            },
            UserBoxFieldDecl {
                name: "capacity".to_string(),
                declared_type_name: None,
                is_weak: false,
            },
        ],
    );

    let mut birth = MirFunction::new(
        FunctionSignature {
            name: "Page.birth/2".to_string(),
            params: vec![
                MirType::Box("Page".to_string()),
                MirType::Unknown,
                MirType::Unknown,
            ],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    birth.params = vec![ValueId::new(0), ValueId::new(1), ValueId::new(2)];
    let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
    birth_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(0),
    });
    birth_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(4),
        src: ValueId::new(1),
    });
    birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(3),
        field: "page_id".to_string(),
        value: ValueId::new(4),
        declared_type: None,
    });
    birth_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(5),
        src: ValueId::new(2),
    });
    birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(3),
        field: "capacity".to_string(),
        value: ValueId::new(5),
        declared_type: None,
    });
    birth.add_block(birth_block);

    let mut main = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut main_block = BasicBlock::new(BasicBlockId::new(0));
    main_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(7),
    });
    main_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(9),
    });
    main_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(3),
        box_type: "Page".to_string(),
        args: vec![ValueId::new(1), ValueId::new(2)],
    });
    main.add_block(main_block);
    module.add_function(birth);
    module.add_function(main);

    let plans = build_typed_object_plans(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].box_name, "Page");
    assert_eq!(plans[0].fields[0].storage, TypedObjectFieldStorage::I64);
    assert_eq!(plans[0].fields[1].storage, TypedObjectFieldStorage::I64);
}

#[test]
fn build_typed_object_plans_infers_birth_param_storage_through_same_module_method_call() {
    let mut module = MirModule::new("typed_object_method_param_flow".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Factory".to_string(), Vec::new());
    module
        .metadata
        .user_box_decls
        .insert("Item".to_string(), vec!["name".to_string()]);
    module.metadata.user_box_field_decls.insert(
        "Item".to_string(),
        vec![UserBoxFieldDecl {
            name: "name".to_string(),
            declared_type_name: None,
            is_weak: false,
        }],
    );

    let mut birth = MirFunction::new(
        FunctionSignature {
            name: "Item.birth/1".to_string(),
            params: vec![MirType::Box("Item".to_string()), MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    birth.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
    birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "name".to_string(),
        value: ValueId::new(1),
        declared_type: None,
    });
    birth.add_block(birth_block);

    let mut make = MirFunction::new(
        FunctionSignature {
            name: "Factory.make/1".to_string(),
            params: vec![MirType::Box("Factory".to_string()), MirType::Unknown],
            return_type: MirType::Box("Item".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    make.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut make_block = BasicBlock::new(BasicBlockId::new(0));
    make_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(2),
        box_type: "Item".to_string(),
        args: vec![ValueId::new(1)],
    });
    make.add_block(make_block);

    let mut main = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut main_block = BasicBlock::new(BasicBlockId::new(0));
    main_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "Factory".to_string(),
        args: vec![],
    });
    main_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("item-a".to_string()),
    });
    main_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Factory".to_string(),
            method: "make".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![ValueId::new(2)],
        effects: EffectMask::PURE,
    });
    main.add_block(main_block);

    module.add_function(birth);
    module.add_function(make);
    module.add_function(main);

    let plans = build_typed_object_plans(&module);

    let item = plans
        .iter()
        .find(|plan| plan.box_name == "Item")
        .expect("Item typed object plan");
    assert_eq!(item.fields[0].storage, TypedObjectFieldStorage::Handle);
}

#[test]
fn build_typed_object_plans_infers_handle_from_same_module_string_like_global_return() {
    let mut module = MirModule::new("typed_object_global_return_infer".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Manifest".to_string(), vec!["root_id".to_string()]);
    module.metadata.user_box_field_decls.insert(
        "Manifest".to_string(),
        vec![UserBoxFieldDecl {
            name: "root_id".to_string(),
            declared_type_name: None,
            is_weak: false,
        }],
    );

    let mut digest = MirFunction::new(
        FunctionSignature {
            name: "Hasher.digest/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    digest.params = vec![ValueId::new(0)];
    let mut digest_block = BasicBlock::new(BasicBlockId::new(0));
    digest_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::String("bt-".to_string()),
    });
    digest_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(7),
    });
    digest_block.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(3),
        op: BinaryOp::Add,
        lhs: ValueId::new(1),
        rhs: ValueId::new(2),
    });
    digest_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    digest.add_block(digest_block);

    let mut birth = MirFunction::new(
        FunctionSignature {
            name: "Manifest.birth/0".to_string(),
            params: vec![MirType::Box("Manifest".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    birth.params = vec![ValueId::new(0)];
    let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
    birth_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::String(String::new()),
    });
    birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "root_id".to_string(),
        value: ValueId::new(1),
        declared_type: None,
    });
    birth.add_block(birth_block);

    let mut seal = MirFunction::new(
        FunctionSignature {
            name: "Manifest.seal/0".to_string(),
            params: vec![MirType::Box("Manifest".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    seal.params = vec![ValueId::new(0)];
    seal.metadata
        .value_types
        .insert(ValueId::new(2), MirType::Integer);
    let mut seal_block = BasicBlock::new(BasicBlockId::new(0));
    seal_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(13),
    });
    seal_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Hasher.digest/1".to_string())),
        args: vec![ValueId::new(1)],
        effects: EffectMask::PURE,
    });
    seal_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "root_id".to_string(),
        value: ValueId::new(2),
        declared_type: None,
    });
    seal.add_block(seal_block);

    module.add_function(digest);
    module.add_function(birth);
    module.add_function(seal);

    let plans = build_typed_object_plans(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].box_name, "Manifest");
    assert_eq!(plans[0].fields[0].name, "root_id");
    assert_eq!(plans[0].fields[0].storage, TypedObjectFieldStorage::Handle);
}

#[test]
fn build_typed_object_plans_accepts_observed_empty_user_box() {
    let mut module = MirModule::new("typed_object_empty".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Worker".to_string(), Vec::new());
    module
        .metadata
        .user_box_field_decls
        .insert("Worker".to_string(), Vec::new());

    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "Worker".to_string(),
        args: vec![],
    });
    function.add_block(block);
    module.add_function(function);

    let plans = build_typed_object_plans(&module);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].box_name, "Worker");
    assert_eq!(plans[0].field_count, 0);
    assert!(plans[0].fields.is_empty());
}

#[test]
fn build_typed_object_plans_rejects_conflicting_untyped_storage() {
    let mut module = MirModule::new("typed_object_conflict".to_string());
    module.metadata.user_box_field_decls.insert(
        "Bad".to_string(),
        vec![UserBoxFieldDecl {
            name: "value".to_string(),
            declared_type_name: None,
            is_weak: false,
        }],
    );

    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "Bad".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(7),
    });
    block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(1),
        field: "value".to_string(),
        value: ValueId::new(2),
        declared_type: None,
    });
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(3),
        box_type: "ArrayBox".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(1),
        field: "value".to_string(),
        value: ValueId::new(3),
        declared_type: None,
    });
    function.add_block(block);
    module.add_function(function);

    let plans = build_typed_object_plans(&module);

    assert!(plans.is_empty());
}
