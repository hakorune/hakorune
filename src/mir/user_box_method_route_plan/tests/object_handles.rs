use super::*;

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

#[test]
fn refresh_module_user_box_method_routes_accepts_loop_carried_nullable_object_return() {
    let mut module =
        MirModule::new("user_box_loop_carried_nullable_object_return_test".to_string());
    for name in ["Queue", "Item"] {
        module
            .metadata
            .user_box_decls
            .insert(name.to_string(), Vec::new());
    }
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Queue".to_string(),
        type_id: 51,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Item".to_string(),
        type_id: 52,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut select = MirFunction::new(
        FunctionSignature {
            name: "Queue.select/0".to_string(),
            params: vec![MirType::Box("Queue".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    select.params = vec![ValueId::new(0)];

    let mut entry = BasicBlock::new(BasicBlockId::new(0));
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Null,
    });
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });

    let mut header = BasicBlock::new(BasicBlockId::new(1));
    header.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(2),
        inputs: vec![
            (BasicBlockId::new(0), ValueId::new(1)),
            (BasicBlockId::new(3), ValueId::new(3)),
        ],
        type_hint: Some(MirType::Void),
    });
    header.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Bool(true),
    });
    header.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(4),
        then_bb: BasicBlockId::new(2),
        else_bb: BasicBlockId::new(4),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut body = BasicBlock::new(BasicBlockId::new(2));
    body.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(5),
        box_type: "Item".to_string(),
        args: Vec::new(),
    });
    body.add_instruction(MirInstruction::Select {
        dst: ValueId::new(6),
        cond: ValueId::new(4),
        then_val: ValueId::new(5),
        else_val: ValueId::new(2),
    });
    body.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut latch = BasicBlock::new(BasicBlockId::new(3));
    latch.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(6),
    });
    latch.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });

    let mut exit = BasicBlock::new(BasicBlockId::new(4));
    exit.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });

    select.add_block(entry);
    select.add_block(header);
    select.add_block(body);
    select.add_block(latch);
    select.add_block(exit);

    let mut main = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Box("Item".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(10),
        box_type: "Queue".to_string(),
        args: Vec::new(),
    });
    block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(11)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "select".to_string(),
            receiver: Some(ValueId::new(10)),
            certainty: TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: Vec::new(),
        effects: EffectMask::PURE,
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });
    main.add_block(block);

    module.add_function(select);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let main = module.get_function("main").expect("main");
    let route = &main.metadata.user_box_method_routes[0];
    assert_eq!(route.reason(), None, "{route:?}");
    assert_eq!(route.return_shape(), Some("object_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.target_result_box_name(), Some("Item"));
    assert_eq!(
        main.metadata.value_types.get(&ValueId::new(11)),
        Some(&MirType::Box("Item".to_string()))
    );
}

#[test]
fn refresh_module_user_box_method_routes_refines_void_placeholder_object_route_result() {
    let mut module =
        MirModule::new("user_box_void_placeholder_object_route_result_test".to_string());
    for name in ["Factory", "Item"] {
        module
            .metadata
            .user_box_decls
            .insert(name.to_string(), Vec::new());
    }
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Factory".to_string(),
        type_id: 61,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Item".to_string(),
        type_id: 62,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut value = MirFunction::new(
        FunctionSignature {
            name: "Item.value/0".to_string(),
            params: vec![MirType::Box("Item".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    value.params = vec![ValueId::new(0)];
    let mut value_block = BasicBlock::new(BasicBlockId::new(0));
    value_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(7),
    });
    value_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    value.add_block(value_block);

    let mut make = MirFunction::new(
        FunctionSignature {
            name: "Factory.make/0".to_string(),
            params: vec![MirType::Box("Factory".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    make.params = vec![ValueId::new(0)];
    let mut make_block = BasicBlock::new(BasicBlockId::new(0));
    make_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(2),
        box_type: "Item".to_string(),
        args: Vec::new(),
    });
    make_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    make.add_block(make_block);

    let mut consume = MirFunction::new(
        FunctionSignature {
            name: "Factory.consume/0".to_string(),
            params: vec![MirType::Box("Factory".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    consume.params = vec![ValueId::new(0)];
    consume.metadata.value_types.insert(ValueId::new(2), MirType::Void);
    let mut consume_block = BasicBlock::new(BasicBlockId::new(0));
    consume_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Factory".to_string(),
            method: "make".to_string(),
            receiver: Some(ValueId::new(0)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: Vec::new(),
        effects: EffectMask::PURE,
    });
    consume_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "value".to_string(),
            receiver: Some(ValueId::new(2)),
            certainty: TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: Vec::new(),
        effects: EffectMask::PURE,
    });
    consume_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    consume.add_block(consume_block);

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
        box_type: "Factory".to_string(),
        args: Vec::new(),
    });
    main_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(11)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Factory".to_string(),
            method: "consume".to_string(),
            receiver: Some(ValueId::new(10)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: Vec::new(),
        effects: EffectMask::PURE,
    });
    main_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });
    main.add_block(main_block);

    module.add_function(value);
    module.add_function(make);
    module.add_function(consume);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let consume = module
        .get_function("Factory.consume/0")
        .expect("consume function");
    assert_eq!(
        consume.metadata.value_types.get(&ValueId::new(2)),
        Some(&MirType::Box("Item".to_string()))
    );
    assert!(consume.metadata.user_box_method_routes.iter().any(|route| {
        route.method() == "value"
            && route.reason().is_none()
            && route.return_shape() == Some("scalar_i64")
    }));

    let main = module.get_function("main").expect("main");
    let route = &main.metadata.user_box_method_routes[0];
    assert_eq!(route.reason(), None, "{route:?}");
    assert!(route.target_body_supported());
    assert_eq!(route.return_shape(), Some("scalar_i64"));
}
