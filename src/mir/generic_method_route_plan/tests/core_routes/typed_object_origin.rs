use super::*;

#[test]
fn records_runtime_data_set_from_declared_direct_array_i64_field_origin() {
    let mut module = MirModule::new("typed_object_direct_array_field_set_route_test".to_string());
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Page".to_string(),
        type_id: 2,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 1,
        fields: vec![TypedObjectFieldPlan {
            name: "free".to_string(),
            slot: 0,
            declared_type_name: Some("DirectArrayI64".to_string()),
            storage: TypedObjectFieldStorage::Handle,
            is_weak: false,
        }],
    });

    let mut reset = MirFunction::new(
        FunctionSignature {
            name: "Page.reset/2".to_string(),
            params: vec![
                MirType::Box("Page".to_string()),
                MirType::Integer,
                MirType::Integer,
            ],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    reset.params = vec![ValueId::new(0), ValueId::new(1), ValueId::new(2)];
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(3),
        base: ValueId::new(0),
        field: "free".to_string(),
        declared_type: None,
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(4),
        src: ValueId::new(3),
    });
    block.add_instruction(method_call(None, "RuntimeDataBox", "set", 4, vec![1, 2]));
    block.set_terminator(MirInstruction::Return { value: None });
    reset.add_block(block);
    module.add_function(reset);

    refresh_module_generic_method_routes(&mut module);

    let reset = module.get_function("Page.reset/2").expect("reset function");
    assert_eq!(reset.metadata.generic_method_routes.len(), 1);
    let route = &reset.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.set");
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "set");
    assert_eq!(route.receiver_origin_box(), Some("DirectArrayI64"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArrayStoreAny);
    let core_method = route
        .core_method()
        .expect("RuntimeDataBox DirectArrayI64 set core method op");
    assert_eq!(core_method.op, CoreMethodOp::ArraySet);
}

#[test]
fn records_runtime_data_get_from_typed_object_array_field_origin() {
    let mut module = MirModule::new("typed_object_array_field_get_route_test".to_string());
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Manifest".to_string(),
        type_id: 2,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 1,
        fields: vec![TypedObjectFieldPlan {
            name: "chunk_ids".to_string(),
            slot: 0,
            declared_type_name: None,
            storage: TypedObjectFieldStorage::Handle,
            is_weak: false,
        }],
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
    birth_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "ArrayBox".to_string(),
        args: vec![],
    });
    birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "chunk_ids".to_string(),
        value: ValueId::new(1),
        declared_type: None,
    });
    birth_block.set_terminator(MirInstruction::Return { value: None });
    birth.add_block(birth_block);

    let mut read = MirFunction::new(
        FunctionSignature {
            name: "Manifest.read/1".to_string(),
            params: vec![MirType::Box("Manifest".to_string()), MirType::Integer],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    read.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut read_block = BasicBlock::new(BasicBlockId::new(0));
    read_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(0),
        field: "chunk_ids".to_string(),
        declared_type: None,
    });
    read_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(2),
    });
    read_block.add_instruction(method_call(Some(4), "RuntimeDataBox", "get", 3, vec![1]));
    read_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    read.add_block(read_block);

    module.add_function(birth);
    module.add_function(read);

    refresh_module_generic_method_routes(&mut module);

    let read = module
        .get_function("Manifest.read/1")
        .expect("read function");
    assert_eq!(read.metadata.generic_method_routes.len(), 1);
    let route = &read.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.get");
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "get");
    assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLoadAny);
    let core_method = route
        .core_method()
        .expect("RuntimeDataBox array get core method op");
    assert_eq!(core_method.op, CoreMethodOp::ArrayGet);
}

#[test]
fn records_array_get_result_origin_from_typed_object_collection_push_param_flow() {
    let mut module = MirModule::new("typed_object_array_element_origin_test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Manifest".to_string(), vec!["chunk_ids".to_string()]);
    module.metadata.user_box_field_decls.insert(
        "Manifest".to_string(),
        vec![UserBoxFieldDecl {
            name: "chunk_ids".to_string(),
            declared_type_name: None,
            is_weak: false,
        }],
    );
    module
        .metadata
        .user_box_decls
        .insert("Store".to_string(), Vec::new());
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Manifest".to_string(),
        type_id: 2,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 1,
        fields: vec![TypedObjectFieldPlan {
            name: "chunk_ids".to_string(),
            slot: 0,
            declared_type_name: None,
            storage: TypedObjectFieldStorage::Handle,
            is_weak: false,
        }],
    });
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Store".to_string(),
        type_id: 3,
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
    birth_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "ArrayBox".to_string(),
        args: Vec::new(),
    });
    birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "chunk_ids".to_string(),
        value: ValueId::new(1),
        declared_type: None,
    });
    birth_block.set_terminator(MirInstruction::Return { value: None });
    birth.add_block(birth_block);

    let mut put = MirFunction::new(
        FunctionSignature {
            name: "Store.put/0".to_string(),
            params: vec![MirType::Box("Store".to_string())],
            return_type: MirType::Box("StringBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    put.params = vec![ValueId::new(0)];
    let mut put_block = BasicBlock::new(BasicBlockId::new(0));
    put_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::String("chunk-a".to_string()),
    });
    put_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global(
            "unsupported_side_effect_probe/0".to_string(),
        )),
        args: Vec::new(),
        effects: EffectMask::PURE,
    });
    put_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    put.add_block(put_block);

    let mut add = MirFunction::new(
        FunctionSignature {
            name: "Manifest.addChunk/1".to_string(),
            params: vec![MirType::Box("Manifest".to_string()), MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    add.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut add_block = BasicBlock::new(BasicBlockId::new(0));
    add_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(0),
        field: "chunk_ids".to_string(),
        declared_type: None,
    });
    add_block.add_instruction(method_call(Some(3), "RuntimeDataBox", "push", 2, vec![1]));
    add_block.set_terminator(MirInstruction::Return { value: None });
    add.add_block(add_block);

    let mut first = MirFunction::new(
        FunctionSignature {
            name: "Manifest.first/0".to_string(),
            params: vec![MirType::Box("Manifest".to_string())],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    first.params = vec![ValueId::new(0)];
    let mut first_block = BasicBlock::new(BasicBlockId::new(0));
    first_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(0),
        field: "chunk_ids".to_string(),
        declared_type: None,
    });
    first_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(0),
    });
    first_block.add_instruction(method_call(Some(4), "RuntimeDataBox", "get", 2, vec![3]));
    first_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    first.add_block(first_block);

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
        box_type: "Manifest".to_string(),
        args: Vec::new(),
    });
    main_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(2),
        box_type: "Store".to_string(),
        args: Vec::new(),
    });
    main_block.add_instruction(method_call(Some(3), "Store", "put", 2, vec![]));
    main_block.add_instruction(method_call(None, "Manifest", "addChunk", 1, vec![3]));
    main_block.add_instruction(method_call(Some(4), "Manifest", "first", 1, vec![]));
    main.add_block(main_block);

    module.add_function(birth);
    module.add_function(put);
    module.add_function(add);
    module.add_function(first);
    module.add_function(main);

    crate::mir::refresh_module_user_box_method_routes(&mut module);
    refresh_module_generic_method_routes(&mut module);

    let main = module.get_function("main").expect("main");
    let put_route = main
        .metadata
        .user_box_method_routes
        .iter()
        .find(|route| route.method() == "put")
        .expect("put route");
    assert_eq!(put_route.reason(), Some("user_box_method_body_unsupported"));
    assert_eq!(put_route.target_result_box_name(), Some("StringBox"));

    let first = module.get_function("Manifest.first/0").expect("first");
    let route = &first.metadata.generic_method_routes[0];
    assert_eq!(route.result_origin_box(), Some("StringBox"));
}
