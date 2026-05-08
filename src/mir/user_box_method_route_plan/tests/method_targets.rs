use super::*;

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
