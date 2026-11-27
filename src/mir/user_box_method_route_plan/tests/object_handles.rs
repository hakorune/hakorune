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
    block.add_instruction(MirInstruction::LegacyCallV0 {
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
        target: crate::mir::ConstructionTarget::Named("Handle".to_string()),
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
        target: crate::mir::ConstructionTarget::Named("Allocator".to_string()),
        args: Vec::new(),
    });
    block.add_instruction(MirInstruction::LegacyCallV0 {
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
fn refresh_module_user_box_method_routes_declared_object_return_overrides_void_inference() {
    let mut module = MirModule::new("user_box_declared_object_return_override_test".to_string());
    for name in ["Parser", "Token"] {
        module
            .metadata
            .user_box_decls
            .insert(name.to_string(), Vec::new());
    }
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Parser".to_string(),
        type_id: 61,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Token".to_string(),
        type_id: 62,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut current = MirFunction::new(
        FunctionSignature {
            name: "Parser.current/0".to_string(),
            params: vec![MirType::Box("Parser".to_string())],
            return_type: MirType::Box("Token".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    current.params = vec![ValueId::new(0)];
    let mut current_block = BasicBlock::new(BasicBlockId::new(0));
    current_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Null,
    });
    current_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    current.add_block(current_block);

    let mut main = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Box("Token".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(2),
        target: crate::mir::ConstructionTarget::Named("Parser".to_string()),
        args: Vec::new(),
    });
    block.add_instruction(MirInstruction::LegacyCallV0 {
        dst: Some(ValueId::new(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "current".to_string(),
            receiver: Some(ValueId::new(2)),
            certainty: TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: Vec::new(),
        effects: EffectMask::PURE,
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    main.add_block(block);

    module.add_function(current);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let main = module.get_function("main").expect("main");
    let route = &main.metadata.user_box_method_routes[0];
    assert_eq!(route.reason(), None, "{route:?}");
    assert_eq!(route.return_shape(), Some("object_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.target_result_box_name(), Some("Token"));
}

#[test]
fn refresh_module_user_box_method_routes_accepts_mixed_runtime_get_return() {
    let mut module = MirModule::new("user_box_mixed_runtime_get_return_test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Node".to_string(), Vec::new());
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Node".to_string(),
        type_id: 71,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 0,
        fields: Vec::new(),
    });

    let mut item = MirFunction::new(
        FunctionSignature {
            name: "Node.item/1".to_string(),
            params: vec![MirType::Box("Node".to_string()), MirType::Integer],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    item.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut item_block = BasicBlock::new(BasicBlockId::new(0));
    item_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(2),
        target: crate::mir::ConstructionTarget::Named("MapBox".to_string()),
        args: Vec::new(),
    });
    item_block.add_instruction(MirInstruction::LegacyCallV0 {
        dst: Some(ValueId::new(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "get".to_string(),
            receiver: Some(ValueId::new(2)),
            certainty: TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(1)],
        effects: EffectMask::PURE,
    });
    item_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    item.add_block(item_block);
    item.metadata
        .generic_method_routes
        .push(runtime_data_map_get_mixed_i64_key(0, 1, 2, 1, 3));

    let mut main = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut main_block = BasicBlock::new(BasicBlockId::new(0));
    main_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(10),
        target: crate::mir::ConstructionTarget::Named("Node".to_string()),
        args: Vec::new(),
    });
    main_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(11),
        value: ConstValue::Integer(0),
    });
    main_block.add_instruction(MirInstruction::LegacyCallV0 {
        dst: Some(ValueId::new(12)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Node".to_string(),
            method: "item".to_string(),
            receiver: Some(ValueId::new(10)),
            certainty: TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![ValueId::new(11)],
        effects: EffectMask::PURE,
    });
    main_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(12)),
    });
    main.add_block(main_block);

    module.add_function(item);
    module.add_function(main);

    refresh_module_user_box_method_routes(&mut module);

    let main = module.get_function("main").expect("main");
    let route = &main.metadata.user_box_method_routes[0];
    assert_eq!(route.reason(), None, "{route:?}");
    assert_eq!(route.proof(), "typed_user_box_method_same_module");
    assert_eq!(route.target_return_type(), Some("unknown".to_string()));
    assert_eq!(route.return_shape(), Some("mixed_runtime_i64_or_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.target_result_box_name(), None);
}
