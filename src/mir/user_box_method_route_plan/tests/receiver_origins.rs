use super::*;

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
