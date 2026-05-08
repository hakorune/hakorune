use super::*;

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
