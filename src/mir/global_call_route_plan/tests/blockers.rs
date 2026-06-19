use super::*;

#[test]
fn refresh_module_global_call_routes_propagates_return_child_blocker_transitively() {
    let mut module = MirModule::new("global_call_void_sentinel_transitive_child_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.maybe_text/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.maybe_text/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(1));
    text_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.wrapper/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(2));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.wrapper/0".to_string(),
            params: vec![],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let wrapper_block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.map/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    wrapper_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    let map = MirFunction::new(
        FunctionSignature {
            name: "Helper.map/0".to_string(),
            params: vec![],
            return_type: MirType::Box("MapBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.wrapper/0".to_string(), wrapper);
    module.functions.insert("Helper.map/0".to_string(), map);
    module
        .functions
        .insert("Helper.maybe_text/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_global_target_shape_unknown")
    );
    assert_eq!(route.target_shape_blocker_symbol(), Some("Helper.map/0"));
    assert_eq!(
        route.target_shape_blocker_reason(),
        Some("generic_string_return_object_abi_not_handle_compatible")
    );
}

#[test]
fn refresh_module_global_call_routes_marks_void_sentinel_const_reason() {
    let mut module = MirModule::new("global_call_void_const_reason_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.flag/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.flag/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Void,
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.flag/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_unsupported_void_sentinel_const")
    );
    assert_eq!(route.target_shape_blocker_symbol(), None);
    assert_eq!(route.target_shape_blocker_reason(), None);
}

#[test]
fn refresh_module_global_call_routes_marks_object_return_abi_reason() {
    let mut module = MirModule::new("global_call_object_return_reason_test".to_string());
    let caller = make_function_with_global_call_args("Helper.map/0", Some(ValueId::new(7)), vec![]);
    let callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.map/0".to_string(),
            params: vec![],
            return_type: MirType::Box("MapBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.map/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_return_object_abi_not_handle_compatible")
    );
    assert_eq!(route.target_shape_blocker_symbol(), None);
    assert_eq!(route.target_shape_blocker_reason(), None);
}

#[test]
fn refresh_module_global_call_routes_accepts_typed_object_handle_return() {
    let mut module = MirModule::new("global_call_typed_object_return_test".to_string());
    module
        .metadata
        .typed_object_plans
        .push(crate::mir::function::TypedObjectPlan {
            box_name: "TreeNode".to_string(),
            type_id: 7,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });
    let caller =
        make_function_with_global_call_args("TreeFactory.make/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "TreeFactory.make/0".to_string(),
            params: vec![],
            return_type: MirType::Box("TreeNode".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "TreeNode".to_string(),
        args: vec![],
    });
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("TreeFactory.make/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.reason(), None);
    assert_eq!(route.return_shape(), Some("object_handle"));
    assert_eq!(route.target_result_box_name(), Some("TreeNode"));
    assert_eq!(route.definition_owner(), "uniform_mir");
    assert_eq!(route.proof(), "typed_global_call_same_module_object_handle");
}

#[test]
fn refresh_module_global_call_routes_accepts_builtin_map_handle_return() {
    let mut module = MirModule::new("global_call_builtin_map_return_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.make_map/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.make_map/0".to_string(),
            params: vec![],
            return_type: MirType::Box("MapBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    entry.instructions.push(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.make_map/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.reason(), None);
    assert_eq!(route.return_shape(), Some("object_handle"));
    assert_eq!(route.target_result_box_name(), Some("MapBox"));
    assert_eq!(route.definition_owner(), "uniform_mir");
    assert_eq!(route.proof(), "typed_global_call_same_module_object_handle");
}

#[test]
fn refresh_module_global_call_routes_accepts_same_module_mixed_runtime_return() {
    let mut module =
        MirModule::new("global_call_same_module_mixed_runtime_return_test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Node".to_string(), Vec::new());
    module
        .metadata
        .typed_object_plans
        .push(crate::mir::function::TypedObjectPlan {
            box_name: "Node".to_string(),
            type_id: 71,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

    let caller =
        make_function_with_global_call_args("Helper.item/0", Some(ValueId::new(7)), vec![]);

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
    let item_entry = item.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    item_entry.instructions.push(MirInstruction::NewBox {
        dst: ValueId::new(2),
        box_type: "MapBox".to_string(),
        args: Vec::new(),
    });
    item_entry.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "get".to_string(),
            receiver: Some(ValueId::new(2)),
            certainty: TypeCertainty::Union,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(1)],
        effects: EffectMask::PURE,
    });
    item_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    item.metadata.generic_method_routes.push(
        crate::mir::generic_method_route_plan::test_support::runtime_data_map_get_mixed_i64_key(
            0, 1, 2, 1, 3,
        ),
    );

    let mut helper = MirFunction::new(
        FunctionSignature {
            name: "Helper.item/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let helper_entry = helper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    helper_entry.instructions.push(MirInstruction::NewBox {
        dst: ValueId::new(10),
        box_type: "Node".to_string(),
        args: Vec::new(),
    });
    helper_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(11),
        value: ConstValue::Integer(0),
    });
    helper_entry.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(12)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Node".to_string(),
            method: "item".to_string(),
            receiver: Some(ValueId::new(10)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::UserDefined,
        }),
        args: vec![ValueId::new(11)],
        effects: EffectMask::PURE,
    });
    helper_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(12)),
    });

    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Node.item/1".to_string(), item);
    module.functions.insert("Helper.item/0".to_string(), helper);

    crate::mir::user_box_method_route_plan::refresh_module_user_box_method_routes(&mut module);
    refresh_module_global_call_routes(&mut module);

    let child_route = &module.functions["Helper.item/0"]
        .metadata
        .user_box_method_routes[0];
    assert_eq!(child_route.reason(), None, "{child_route:?}");
    assert_eq!(
        child_route.return_shape(),
        Some("mixed_runtime_i64_or_handle")
    );

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.reason(), None, "{route:?}");
    assert_eq!(route.return_shape(), Some("mixed_runtime_i64_or_handle"));
    assert_eq!(route.target_result_box_name(), None);
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.definition_owner(), "uniform_mir");
    assert_eq!(route.proof(), "typed_global_call_same_module_mixed_runtime");
}

#[test]
fn refresh_module_global_call_routes_accepts_map_handle_child_field_get_string_body() {
    let mut module = MirModule::new("global_call_map_child_field_get_string_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.get_value/0", Some(ValueId::new(7)), vec![]);

    let mut make_map = MirFunction::new(
        FunctionSignature {
            name: "Helper.make_map/0".to_string(),
            params: vec![],
            return_type: MirType::Box("MapBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let make_map_entry = make_map.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    make_map_entry.instructions.push(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    make_map_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });

    let mut get_value = MirFunction::new(
        FunctionSignature {
            name: "Helper.get_value/0".to_string(),
            params: vec![],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    get_value
        .metadata
        .value_types
        .insert(ValueId::new(2), MirType::String);
    let get_value_entry = get_value.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    get_value_entry.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.make_map/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    get_value_entry.instructions.push(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(1),
        field: "value".to_string(),
        declared_type: None,
    });
    get_value_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.make_map/0".to_string(), make_map);
    module
        .functions
        .insert("Helper.get_value/0".to_string(), get_value);

    refresh_module_global_call_routes(&mut module);

    let child_route = module.functions["Helper.get_value/0"]
        .metadata
        .global_call_routes
        .iter()
        .find(|route| route.callee_name() == "Helper.make_map/0")
        .expect("child map factory route");
    assert_eq!(child_route.reason(), None);
    assert_eq!(child_route.return_shape(), Some("object_handle"));
    assert_eq!(child_route.target_result_box_name(), Some("MapBox"));

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.reason(), None, "{route:?}");
    assert_eq!(route.return_shape(), Some("string_handle"));
    assert_eq!(route.definition_owner(), "module_generic");
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_accepts_same_module_bool_return() {
    let mut module = MirModule::new("global_call_same_module_bool_return_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.is_zero/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.is_zero/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Bool,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(0),
    });
    entry.instructions.push(MirInstruction::Compare {
        dst: ValueId::new(3),
        op: CompareOp::Eq,
        lhs: ValueId::new(1),
        rhs: ValueId::new(2),
    });
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.is_zero/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.reason(), None);
    assert_eq!(route.return_shape(), Some("ScalarI64"));
    assert_eq!(route.target_result_box_name(), None);
    assert_eq!(route.definition_owner(), "generic_i64_or_leaf");
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
}

#[test]
fn refresh_module_global_call_routes_marks_void_signature_object_or_void_return_reason() {
    let mut module =
        MirModule::new("global_call_void_signature_object_return_reason_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.entries/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.entries/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(1));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut object_block = BasicBlock::new(BasicBlockId::new(2));
    object_block.instructions.push(MirInstruction::NewBox {
        dst: ValueId::new(3),
        box_type: "ArrayBox".to_string(),
        args: vec![],
    });
    object_block.instructions.push(MirInstruction::Copy {
        dst: ValueId::new(4),
        src: ValueId::new(3),
    });
    object_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    callee.blocks.insert(BasicBlockId::new(1), void_block);
    callee.blocks.insert(BasicBlockId::new(2), object_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.entries/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_return_type(), Some("void".to_string()));
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_return_object_abi_not_handle_compatible")
    );
    assert_eq!(route.target_shape_blocker_symbol(), None);
    assert_eq!(route.target_shape_blocker_reason(), None);
}

#[test]
fn refresh_module_global_call_routes_allows_null_guard_before_method_blocker() {
    let mut module = MirModule::new("global_call_null_guard_method_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.preview/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.preview/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(3),
            op: CompareOp::Eq,
            lhs: ValueId::new(1),
            rhs: ValueId::new(2),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(3),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut null_block = BasicBlock::new(BasicBlockId::new(1));
    null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::String("<null>".to_string()),
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    let mut method_block = BasicBlock::new(BasicBlockId::new(2));
    method_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(5)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "debugPreview".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Union,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    });
    method_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    callee.blocks.insert(BasicBlockId::new(1), null_block);
    callee.blocks.insert(BasicBlockId::new(2), method_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.preview/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_unsupported_method_call")
    );
    assert_eq!(route.target_shape_blocker_symbol(), None);
    assert_eq!(route.target_shape_blocker_reason(), None);
}
