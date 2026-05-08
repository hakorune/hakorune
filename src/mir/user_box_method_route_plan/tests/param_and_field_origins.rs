use super::*;

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
