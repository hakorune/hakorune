use super::*;
use crate::mir::definitions::call_unified::TypeCertainty;
use crate::mir::function::{TypedObjectFieldPlan, TypedObjectFieldStorage, TypedObjectPlan};
use crate::mir::generic_method_route_plan::test_support::{
    array_push, runtime_data_map_get_mixed_i64_key_with_result_origin_box,
};
use crate::mir::{BasicBlock, BinaryOp, ConstValue, EffectMask, FunctionSignature, MirInstruction};

#[test]
fn refresh_module_user_box_method_routes_accepts_birth_same_module_target() {
    let mut module = MirModule::new("user_box_birth_route_test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Pair".to_string(), vec!["left".to_string()]);
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Pair".to_string(),
        type_id: 7,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut birth = MirFunction::new(
        FunctionSignature {
            name: "Pair.birth/0".to_string(),
            params: vec![MirType::Box("Pair".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    birth.params = vec![ValueId::new(0)];
    let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
    birth_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Void,
    });
    birth_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
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
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Pair".to_string(),
            method: "birth".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    });
    main.add_block(block);

    module.add_function(birth);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let main = module.get_function("main").expect("main function");
    let route = &main.metadata.user_box_method_routes[0];
    assert_eq!(route.route_id(), "user_box.method_call");
    assert_eq!(route.proof(), "typed_user_box_birth_same_module");
    assert_eq!(route.target_symbol(), "Pair.birth/0");
    assert_eq!(route.target_arity(), Some(1));
    assert_eq!(route.arity_matches(), Some(true));
    assert!(route.target_body_supported());
    assert_eq!(route.type_id(), Some(7));
    assert_eq!(route.definition_owner(), "typed_object_method");
}

#[test]
fn refresh_module_user_box_method_routes_rejects_unsupported_birth_body() {
    let mut module = MirModule::new("user_box_birth_route_reject_test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Pair".to_string(), vec!["left".to_string()]);
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Pair".to_string(),
        type_id: 7,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut birth = MirFunction::new(
        FunctionSignature {
            name: "Pair.birth/0".to_string(),
            params: vec![MirType::Box("Pair".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    birth.params = vec![ValueId::new(0)];
    let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
    birth_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("helper".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    birth_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Void,
    });
    birth_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
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
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Pair".to_string(),
            method: "birth".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    });
    main.add_block(block);

    module.add_function(birth);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let main = module.get_function("main").expect("main function");
    let route = &main.metadata.user_box_method_routes[0];
    assert_eq!(route.proof(), "typed_user_box_method_contract_missing");
    assert!(!route.target_body_supported());
    assert_eq!(route.reason(), Some("user_box_birth_body_unsupported"));
    assert_eq!(route.definition_owner(), "none");
}

#[test]
fn refresh_module_user_box_method_routes_accepts_birth_with_string_handle_const() {
    let mut module = MirModule::new("user_box_birth_string_const_route_test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Manifest".to_string(), vec!["root".to_string()]);
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Manifest".to_string(),
        type_id: 9,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

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
        value: ConstValue::String("".to_string()),
    });
    birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "root".to_string(),
        value: ValueId::new(1),
        declared_type: Some(MirType::String),
    });
    birth_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Void,
    });
    birth_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
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
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Manifest".to_string(),
            method: "birth".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    });
    main.add_block(block);

    module.add_function(birth);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let main = module.get_function("main").expect("main function");
    let route = &main.metadata.user_box_method_routes[0];
    assert_eq!(route.proof(), "typed_user_box_birth_same_module");
    assert_eq!(route.reason(), None);
    assert_eq!(route.return_shape(), Some("void_sentinel_i64_zero"));
}

#[test]
fn refresh_module_user_box_method_routes_accepts_void_method_with_generic_route() {
    let mut module = MirModule::new("user_box_void_method_route_test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Manifest".to_string(), vec!["chunk_ids".to_string()]);
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Manifest".to_string(),
        type_id: 9,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut add_chunk = MirFunction::new(
        FunctionSignature {
            name: "Manifest.addChunk/1".to_string(),
            params: vec![
                MirType::Box("Manifest".to_string()),
                MirType::Box("StringBox".to_string()),
            ],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    add_chunk.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut add_block = BasicBlock::new(BasicBlockId::new(0));
    add_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "ArrayBox".to_string(),
            method: "push".to_string(),
            receiver: Some(ValueId::new(2)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(1)],
        effects: EffectMask::PURE,
    });
    add_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Void,
    });
    add_block.add_instruction(MirInstruction::KeepAlive {
        values: vec![ValueId::new(0), ValueId::new(1)],
    });
    add_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    add_chunk
        .metadata
        .generic_method_routes
        .push(array_push(0, 0, 2, 3));
    add_chunk.add_block(add_block);

    let mut main = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(5)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Manifest".to_string(),
            method: "addChunk".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![ValueId::new(2)],
        effects: EffectMask::PURE,
    });
    main.add_block(block);

    module.add_function(add_chunk);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let main = module.get_function("main").expect("main function");
    let route = &main.metadata.user_box_method_routes[0];
    assert_eq!(route.proof(), "typed_user_box_method_same_module");
    assert_eq!(route.reason(), None);
    assert_eq!(route.return_shape(), Some("void_sentinel_i64_zero"));
    assert_eq!(route.definition_owner(), "typed_object_method");
}

#[test]
fn refresh_module_user_box_method_routes_accepts_scalar_instance_method_target() {
    let mut module = MirModule::new("user_box_method_route_test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Pair".to_string(), vec!["left".to_string()]);
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Pair".to_string(),
        type_id: 7,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut sum = MirFunction::new(
        FunctionSignature {
            name: "Pair.sum/0".to_string(),
            params: vec![MirType::Box("Pair".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    sum.params = vec![ValueId::new(0)];
    let mut sum_block = BasicBlock::new(BasicBlockId::new(0));
    sum_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(30),
    });
    sum_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    sum.add_block(sum_block);

    let mut main = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Pair".to_string(),
            method: "sum".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    });
    main.add_block(block);

    module.add_function(sum);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let main = module.get_function("main").expect("main function");
    let route = &main.metadata.user_box_method_routes[0];
    assert_eq!(route.route_id(), "user_box.method_call");
    assert_eq!(route.route_kind(), "user_box.method");
    assert_eq!(route.proof(), "typed_user_box_method_same_module");
    assert_eq!(route.target_symbol(), "Pair.sum/0");
    assert_eq!(route.target_arity(), Some(1));
    assert_eq!(route.arity_matches(), Some(true));
    assert!(route.target_body_supported());
    assert_eq!(route.return_shape(), Some("scalar_i64"));
    assert_eq!(route.type_id(), Some(7));
    assert_eq!(route.definition_owner(), "typed_object_method");
}

#[test]
fn refresh_module_user_box_method_routes_accepts_string_handle_method_target() {
    let mut module = MirModule::new("user_box_string_handle_method_route_test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Manifest".to_string(), vec!["name".to_string()]);
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Manifest".to_string(),
        type_id: 11,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut name = MirFunction::new(
        FunctionSignature {
            name: "Manifest.name/0".to_string(),
            params: vec![MirType::Box("Manifest".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    name.params = vec![ValueId::new(0)];
    let mut name_block = BasicBlock::new(BasicBlockId::new(0));
    name_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::String("payload-a".to_string()),
    });
    name_block.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(3),
        inputs: vec![(BasicBlockId::new(0), ValueId::new(1))],
        type_hint: Some(MirType::Integer),
    });
    name_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    name.add_block(name_block);

    let mut main = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Manifest".to_string(),
            method: "name".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    });
    main.add_block(block);

    module.add_function(name);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let main = module.get_function("main").expect("main function");
    let route = &main.metadata.user_box_method_routes[0];
    assert_eq!(route.proof(), "typed_user_box_method_same_module");
    assert_eq!(route.reason(), None);
    assert_eq!(route.return_shape(), Some("string_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.result_origin(), "string");
    assert_eq!(route.definition_owner(), "typed_object_method");
}

#[test]
fn refresh_module_user_box_method_routes_recovers_receiver_box_from_param_origin() {
    let mut module = MirModule::new("user_box_param_receiver_route_test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Store".to_string(), vec!["items".to_string()]);
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Store".to_string(),
        type_id: 13,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut put = MirFunction::new(
        FunctionSignature {
            name: "Store.put/1".to_string(),
            params: vec![
                MirType::Box("Store".to_string()),
                MirType::Box("StringBox".to_string()),
            ],
            return_type: MirType::Box("StringBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    put.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut put_block = BasicBlock::new(BasicBlockId::new(0));
    put_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    put_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    put.add_block(put_block);

    let mut caller = MirFunction::new(
        FunctionSignature {
            name: "Caller.run/1".to_string(),
            params: vec![MirType::Box("Store".to_string())],
            return_type: MirType::Box("StringBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    caller.params = vec![ValueId::new(0)];
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(1),
        src: ValueId::new(0),
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("chunk".to_string()),
    });
    block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "put".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(2)],
        effects: EffectMask::PURE,
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    caller.add_block(block);

    module.add_function(put);
    module.add_function(caller);

    refresh_module_user_box_method_routes(&mut module);

    let caller = module.get_function("Caller.run/1").expect("caller");
    let route = &caller.metadata.user_box_method_routes[0];
    assert_eq!(route.box_name(), "Store");
    assert_eq!(route.method(), "put");
    assert_eq!(route.reason(), None, "{route:?}");
    assert_eq!(route.proof(), "typed_user_box_method_same_module");
    assert_eq!(route.return_shape(), Some("string_handle"));
}

#[test]
fn refresh_module_user_box_method_routes_recovers_receiver_box_from_call_arg_origin() {
    let mut module = MirModule::new("user_box_call_arg_receiver_route_test".to_string());
    for name in ["Store", "Worker"] {
        module
            .metadata
            .user_box_decls
            .insert(name.to_string(), Vec::new());
    }
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Store".to_string(),
        type_id: 13,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Worker".to_string(),
        type_id: 14,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut put = MirFunction::new(
        FunctionSignature {
            name: "Store.put/1".to_string(),
            params: vec![
                MirType::Box("Store".to_string()),
                MirType::Box("StringBox".to_string()),
            ],
            return_type: MirType::Box("StringBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    put.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut put_block = BasicBlock::new(BasicBlockId::new(0));
    put_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    put_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    put.add_block(put_block);

    let mut run = MirFunction::new(
        FunctionSignature {
            name: "Worker.run/2".to_string(),
            params: vec![
                MirType::Box("Worker".to_string()),
                MirType::Unknown,
                MirType::Box("StringBox".to_string()),
            ],
            return_type: MirType::Box("StringBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    run.params = vec![ValueId::new(0), ValueId::new(1), ValueId::new(2)];
    let mut run_block = BasicBlock::new(BasicBlockId::new(0));
    run_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(1),
    });
    run_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(4),
        src: ValueId::new(2),
    });
    run_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(5)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "put".to_string(),
            receiver: Some(ValueId::new(3)),
            certainty: TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(4)],
        effects: EffectMask::PURE,
    });
    run_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    run.add_block(run_block);

    let mut main = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Box("StringBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut main_block = BasicBlock::new(BasicBlockId::new(0));
    main_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "Store".to_string(),
        args: Vec::new(),
    });
    main_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(2),
        box_type: "Worker".to_string(),
        args: Vec::new(),
    });
    main_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::String("chunk".to_string()),
    });
    main_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(4)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Worker".to_string(),
            method: "run".to_string(),
            receiver: Some(ValueId::new(2)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![ValueId::new(1), ValueId::new(3)],
        effects: EffectMask::PURE,
    });
    main_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    main.add_block(main_block);

    module.add_function(put);
    module.add_function(run);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let run = module.get_function("Worker.run/2").expect("run");
    let route = run
        .metadata
        .user_box_method_routes
        .iter()
        .find(|route| route.method() == "put")
        .expect("Store.put route");
    assert_eq!(route.box_name(), "Store");
    assert_eq!(route.reason(), None, "{route:?}");
    assert_eq!(route.return_shape(), Some("string_handle"));
    assert_eq!(
        run.metadata.value_types.get(&ValueId::new(1)),
        Some(&MirType::Box("Store".to_string()))
    );

    let main = module.get_function("main").expect("main");
    let route = main
        .metadata
        .user_box_method_routes
        .iter()
        .find(|route| route.method() == "run")
        .expect("Worker.run route");
    assert_eq!(route.reason(), None, "{route:?}");
    assert_eq!(route.return_shape(), Some("string_handle"));
}

#[test]
fn refresh_module_user_box_method_routes_recovers_receiver_box_from_generic_result_origin() {
    let mut module = MirModule::new("user_box_generic_result_receiver_route_test".to_string());
    for name in ["ContentChunk", "Store"] {
        module
            .metadata
            .user_box_decls
            .insert(name.to_string(), Vec::new());
    }
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "ContentChunk".to_string(),
        type_id: 21,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Store".to_string(),
        type_id: 22,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut retain = MirFunction::new(
        FunctionSignature {
            name: "ContentChunk.retain/0".to_string(),
            params: vec![MirType::Box("ContentChunk".to_string())],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    retain.params = vec![ValueId::new(0)];
    let mut retain_block = BasicBlock::new(BasicBlockId::new(0));
    retain_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(1),
        src: ValueId::new(0),
    });
    retain_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(1),
        field: "ref_count".to_string(),
        declared_type: None,
    });
    retain_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(2),
    });
    retain_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Integer(1),
    });
    retain_block.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(5),
        op: BinaryOp::Add,
        lhs: ValueId::new(3),
        rhs: ValueId::new(4),
    });
    retain_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(1),
        field: "ref_count".to_string(),
        value: ValueId::new(5),
        declared_type: None,
    });
    retain_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(6),
        base: ValueId::new(1),
        field: "ref_count".to_string(),
        declared_type: None,
    });
    retain_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(7),
        src: ValueId::new(6),
    });
    retain_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(7)),
    });
    retain.add_block(retain_block);
    retain
        .metadata
        .value_types
        .insert(ValueId::new(5), MirType::Integer);

    let mut put = MirFunction::new(
        FunctionSignature {
            name: "Store.put/0".to_string(),
            params: vec![MirType::Box("Store".to_string())],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    put.params = vec![ValueId::new(0)];
    let mut put_block = BasicBlock::new(BasicBlockId::new(0));
    put_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(7),
    });
    put_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "get".to_string(),
            receiver: Some(ValueId::new(10)),
            certainty: TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(1)],
        effects: EffectMask::PURE,
    });
    put_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(2),
    });
    put_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(4)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "retain".to_string(),
            receiver: Some(ValueId::new(3)),
            certainty: TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: Vec::new(),
        effects: EffectMask::PURE,
    });
    put_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    put.add_block(put_block);
    put.metadata.generic_method_routes.push(
        runtime_data_map_get_mixed_i64_key_with_result_origin_box(0, 1, 10, 1, 2, "ContentChunk"),
    );

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
        dst: ValueId::new(20),
        box_type: "Store".to_string(),
        args: Vec::new(),
    });
    main_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(21)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "put".to_string(),
            receiver: Some(ValueId::new(20)),
            certainty: TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: Vec::new(),
        effects: EffectMask::PURE,
    });
    main_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(21)),
    });
    main.add_block(main_block);

    module.add_function(retain);
    module.add_function(put);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let put = module.get_function("Store.put/0").expect("Store.put");
    let retain_route = put
        .metadata
        .user_box_method_routes
        .iter()
        .find(|route| route.method() == "retain")
        .expect("ContentChunk.retain route");
    assert_eq!(retain_route.box_name(), "ContentChunk");
    assert_eq!(retain_route.reason(), None, "{retain_route:?}");
    assert_eq!(retain_route.return_shape(), Some("scalar_i64"));

    let main = module.get_function("main").expect("main");
    let put_route = main
        .metadata
        .user_box_method_routes
        .iter()
        .find(|route| route.method() == "put")
        .expect("Store.put route");
    assert_eq!(put_route.box_name(), "Store");
    assert_eq!(put_route.reason(), None, "{put_route:?}");
    assert_eq!(put_route.return_shape(), Some("scalar_i64"));
}

#[test]
fn refresh_module_user_box_method_routes_propagates_callee_param_box_to_caller_param() {
    let mut module = MirModule::new("user_box_callee_param_origin_backprop_test".to_string());
    for name in ["Handle", "Page", "Heap"] {
        module
            .metadata
            .user_box_decls
            .insert(name.to_string(), Vec::new());
    }
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Handle".to_string(),
        type_id: 51,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 2,
        fields: vec![
            TypedObjectFieldPlan {
                name: "page_id".to_string(),
                slot: 0,
                declared_type_name: None,
                storage: TypedObjectFieldStorage::I64,
                is_weak: false,
            },
            TypedObjectFieldPlan {
                name: "block_id".to_string(),
                slot: 1,
                declared_type_name: None,
                storage: TypedObjectFieldStorage::I64,
                is_weak: false,
            },
        ],
    });
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Page".to_string(),
        type_id: 52,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 1,
        fields: vec![TypedObjectFieldPlan {
            name: "page_id".to_string(),
            slot: 0,
            declared_type_name: None,
            storage: TypedObjectFieldStorage::I64,
            is_weak: false,
        }],
    });
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Heap".to_string(),
        type_id: 53,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut page_release = MirFunction::new(
        FunctionSignature {
            name: "Page.release/1".to_string(),
            params: vec![MirType::Box("Page".to_string()), MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    page_release.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut page_block = BasicBlock::new(BasicBlockId::new(0));
    page_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    page_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(3),
        base: ValueId::new(2),
        field: "block_id".to_string(),
        declared_type: None,
    });
    page_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    page_release.add_block(page_block);

    let mut heap_release = MirFunction::new(
        FunctionSignature {
            name: "Heap.release/1".to_string(),
            params: vec![MirType::Box("Heap".to_string()), MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    heap_release.params = vec![ValueId::new(10), ValueId::new(11)];
    let mut heap_block = BasicBlock::new(BasicBlockId::new(0));
    heap_block.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(12),
        inputs: vec![(BasicBlockId::new(0), ValueId::new(11))],
        type_hint: None,
    });
    heap_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(13),
        src: ValueId::new(12),
    });
    heap_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(14),
        base: ValueId::new(13),
        field: "page_id".to_string(),
        declared_type: None,
    });
    heap_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(15),
        box_type: "Page".to_string(),
        args: Vec::new(),
    });
    heap_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(16)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Page".to_string(),
            method: "release".to_string(),
            receiver: Some(ValueId::new(15)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![ValueId::new(13)],
        effects: EffectMask::PURE,
    });
    heap_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(16)),
    });
    heap_release.add_block(heap_block);

    module.add_function(page_release);
    module.add_function(heap_release);

    refresh_module_user_box_method_routes(&mut module);

    let page_release = module.get_function("Page.release/1").expect("Page.release");
    assert_eq!(
        page_release.metadata.value_types.get(&ValueId::new(1)),
        Some(&MirType::Box("Handle".to_string()))
    );

    let heap_release = module.get_function("Heap.release/1").expect("Heap.release");
    assert_eq!(
        heap_release.metadata.value_types.get(&ValueId::new(11)),
        Some(&MirType::Box("Handle".to_string()))
    );
    assert_eq!(
        heap_release.metadata.value_types.get(&ValueId::new(13)),
        Some(&MirType::Box("Handle".to_string()))
    );
}

#[test]
fn refresh_module_user_box_method_routes_refines_placeholder_param_for_string_field_return() {
    let mut module = MirModule::new("user_box_string_field_return_refinement_test".to_string());
    for name in ["ContentChunk", "Store"] {
        module
            .metadata
            .user_box_decls
            .insert(name.to_string(), Vec::new());
    }
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "ContentChunk".to_string(),
        type_id: 41,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 1,
        fields: vec![TypedObjectFieldPlan {
            name: "data".to_string(),
            slot: 0,
            declared_type_name: None,
            storage: TypedObjectFieldStorage::Handle,
            is_weak: false,
        }],
    });
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Store".to_string(),
        type_id: 42,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut chunk_birth = MirFunction::new(
        FunctionSignature {
            name: "ContentChunk.birth/1".to_string(),
            params: vec![MirType::Box("ContentChunk".to_string()), MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    chunk_birth.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut chunk_birth_block = BasicBlock::new(BasicBlockId::new(0));
    chunk_birth_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(0),
    });
    chunk_birth_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(1),
    });
    chunk_birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(2),
        field: "data".to_string(),
        value: ValueId::new(3),
        declared_type: None,
    });
    chunk_birth_block.set_terminator(MirInstruction::Return { value: None });
    chunk_birth.add_block(chunk_birth_block);

    let mut put = MirFunction::new(
        FunctionSignature {
            name: "Store.put/1".to_string(),
            params: vec![MirType::Box("Store".to_string()), MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    put.params = vec![ValueId::new(10), ValueId::new(11)];
    let mut put_block = BasicBlock::new(BasicBlockId::new(0));
    put_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(12),
        box_type: "ContentChunk".to_string(),
        args: vec![ValueId::new(11)],
    });
    put_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(13),
        src: ValueId::new(12),
    });
    put_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(14),
        src: ValueId::new(11),
    });
    put_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(15)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "ContentChunk".to_string(),
            method: "birth".to_string(),
            receiver: Some(ValueId::new(13)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![ValueId::new(14)],
        effects: EffectMask::PURE,
    });
    put_block.set_terminator(MirInstruction::Return { value: None });
    put.add_block(put_block);

    let mut read_data = MirFunction::new(
        FunctionSignature {
            name: "Store.readData/1".to_string(),
            params: vec![
                MirType::Box("Store".to_string()),
                MirType::Box("StringBox".to_string()),
            ],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    read_data.params = vec![ValueId::new(20), ValueId::new(21)];
    let mut read_block = BasicBlock::new(BasicBlockId::new(0));
    read_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(22)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "get".to_string(),
            receiver: Some(ValueId::new(30)),
            certainty: TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(21)],
        effects: EffectMask::PURE,
    });
    read_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(23),
        base: ValueId::new(22),
        field: "data".to_string(),
        declared_type: None,
    });
    read_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(24),
        src: ValueId::new(23),
    });
    read_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(24)),
    });
    read_data.add_block(read_block);
    read_data.metadata.generic_method_routes.push(
        runtime_data_map_get_mixed_i64_key_with_result_origin_box(0, 0, 30, 21, 22, "ContentChunk"),
    );

    let mut main = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: Vec::new(),
            return_type: MirType::Box("StringBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut main_block = BasicBlock::new(BasicBlockId::new(0));
    main_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(40),
        box_type: "Store".to_string(),
        args: Vec::new(),
    });
    main_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(41),
        value: ConstValue::String("chunk".to_string()),
    });
    main_block.add_instruction(MirInstruction::Call {
        dst: None,
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Store".to_string(),
            method: "put".to_string(),
            receiver: Some(ValueId::new(40)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![ValueId::new(41)],
        effects: EffectMask::PURE,
    });
    main_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(42)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Store".to_string(),
            method: "readData".to_string(),
            receiver: Some(ValueId::new(40)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![ValueId::new(41)],
        effects: EffectMask::PURE,
    });
    main_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(42)),
    });
    main.add_block(main_block);

    module.add_function(chunk_birth);
    module.add_function(put);
    module.add_function(read_data);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let put = module.get_function("Store.put/1").expect("Store.put");
    assert_eq!(
        put.metadata.value_types.get(&ValueId::new(11)),
        Some(&MirType::Box("StringBox".to_string()))
    );

    let read_data = module
        .get_function("Store.readData/1")
        .expect("Store.readData");
    assert_eq!(
        read_data.metadata.value_types.get(&ValueId::new(23)),
        Some(&MirType::Box("StringBox".to_string()))
    );

    let main = module.get_function("main").expect("main");
    let read_route = main
        .metadata
        .user_box_method_routes
        .iter()
        .find(|route| route.method() == "readData")
        .expect("Store.readData route");
    assert_eq!(read_route.reason(), None, "{read_route:?}");
    assert_eq!(read_route.return_shape(), Some("string_handle"));
    assert_eq!(read_route.target_result_box_name(), Some("StringBox"));
}

#[test]
fn refresh_module_user_box_method_routes_recovers_receiver_box_from_field_origin() {
    let mut module = MirModule::new("user_box_field_receiver_route_test".to_string());
    for name in ["Heap", "Store"] {
        module
            .metadata
            .user_box_decls
            .insert(name.to_string(), Vec::new());
    }
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Heap".to_string(),
        type_id: 31,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Store".to_string(),
        type_id: 32,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut allocate = MirFunction::new(
        FunctionSignature {
            name: "Heap.allocate/0".to_string(),
            params: vec![MirType::Box("Heap".to_string())],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    allocate.params = vec![ValueId::new(0)];
    let mut allocate_block = BasicBlock::new(BasicBlockId::new(0));
    allocate_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(99),
    });
    allocate_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    allocate.add_block(allocate_block);

    let mut birth = MirFunction::new(
        FunctionSignature {
            name: "Store.birth/0".to_string(),
            params: vec![MirType::Box("Store".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    birth.params = vec![ValueId::new(0)];
    let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
    birth_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "Heap".to_string(),
        args: Vec::new(),
    });
    birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "allocator".to_string(),
        value: ValueId::new(1),
        declared_type: None,
    });
    birth_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Void,
    });
    birth_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    birth.add_block(birth_block);

    let mut put = MirFunction::new(
        FunctionSignature {
            name: "Store.put/0".to_string(),
            params: vec![MirType::Box("Store".to_string())],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    put.params = vec![ValueId::new(0)];
    let mut put_block = BasicBlock::new(BasicBlockId::new(0));
    put_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(1),
        base: ValueId::new(0),
        field: "allocator".to_string(),
        declared_type: None,
    });
    put_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    put_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "allocate".to_string(),
            receiver: Some(ValueId::new(2)),
            certainty: TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: Vec::new(),
        effects: EffectMask::PURE,
    });
    put_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    put.add_block(put_block);

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
        dst: ValueId::new(10),
        box_type: "Store".to_string(),
        args: Vec::new(),
    });
    main_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(11)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "put".to_string(),
            receiver: Some(ValueId::new(10)),
            certainty: TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: Vec::new(),
        effects: EffectMask::PURE,
    });
    main_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });
    main.add_block(main_block);

    module.add_function(allocate);
    module.add_function(birth);
    module.add_function(put);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let put = module.get_function("Store.put/0").expect("Store.put");
    let allocate_route = put
        .metadata
        .user_box_method_routes
        .iter()
        .find(|route| route.method() == "allocate")
        .expect("Heap.allocate route");
    assert_eq!(allocate_route.box_name(), "Heap");
    assert_eq!(allocate_route.reason(), None, "{allocate_route:?}");
    assert_eq!(allocate_route.return_shape(), Some("scalar_i64"));

    let main = module.get_function("main").expect("main");
    let put_route = main
        .metadata
        .user_box_method_routes
        .iter()
        .find(|route| route.method() == "put")
        .expect("Store.put route");
    assert_eq!(put_route.box_name(), "Store");
    assert_eq!(put_route.reason(), None, "{put_route:?}");
    assert_eq!(put_route.return_shape(), Some("scalar_i64"));
}

#[test]
fn refresh_module_user_box_method_routes_accepts_object_handle_method_target() {
    let mut module = MirModule::new("user_box_object_handle_method_route_test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Manifest".to_string(), vec!["name".to_string()]);
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Manifest".to_string(),
        type_id: 12,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut identity = MirFunction::new(
        FunctionSignature {
            name: "Manifest.identity/0".to_string(),
            params: vec![MirType::Box("Manifest".to_string())],
            return_type: MirType::Box("Manifest".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    identity.params = vec![ValueId::new(0)];
    let mut identity_block = BasicBlock::new(BasicBlockId::new(0));
    identity_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(1),
        src: ValueId::new(0),
    });
    identity_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    identity.add_block(identity_block);

    let mut main = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Manifest".to_string(),
            method: "identity".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    });
    main.add_block(block);

    module.add_function(identity);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let main = module.get_function("main").expect("main function");
    let route = &main.metadata.user_box_method_routes[0];
    assert_eq!(route.proof(), "typed_user_box_method_same_module");
    assert_eq!(route.reason(), None);
    assert_eq!(route.return_shape(), Some("object_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.result_origin(), "none");
    assert_eq!(route.definition_owner(), "typed_object_method");
}

#[test]
fn refresh_module_user_box_method_routes_accepts_nullable_object_handle_method_target() {
    let mut module =
        MirModule::new("user_box_nullable_object_handle_method_route_test".to_string());
    for name in ["Allocator", "Handle"] {
        module
            .metadata
            .user_box_decls
            .insert(name.to_string(), Vec::new());
    }
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Allocator".to_string(),
        type_id: 41,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Handle".to_string(),
        type_id: 42,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut allocate = MirFunction::new(
        FunctionSignature {
            name: "Allocator.allocate/0".to_string(),
            params: vec![MirType::Box("Allocator".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    allocate.params = vec![ValueId::new(0)];
    let mut null_block = BasicBlock::new(BasicBlockId::new(0));
    null_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Null,
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    let mut handle_block = BasicBlock::new(BasicBlockId::new(1));
    handle_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(2),
        box_type: "Handle".to_string(),
        args: Vec::new(),
    });
    handle_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    allocate.add_block(null_block);
    allocate.add_block(handle_block);

    let mut main = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Box("Handle".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(3),
        box_type: "Allocator".to_string(),
        args: Vec::new(),
    });
    block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(4)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "allocate".to_string(),
            receiver: Some(ValueId::new(3)),
            certainty: TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: Vec::new(),
        effects: EffectMask::PURE,
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    main.add_block(block);

    module.add_function(allocate);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let main = module.get_function("main").expect("main");
    let route = &main.metadata.user_box_method_routes[0];
    assert_eq!(route.reason(), None, "{route:?}");
    assert_eq!(route.return_shape(), Some("object_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
}
