use super::*;

fn same_module_constructed_handle(
    module_name: &str,
    symbol: &str,
    box_name: &str,
    return_type: MirType,
    typed_layout: Option<crate::mir::function::TypedObjectPlan>,
    copy_result: bool,
) -> MirModule {
    let mut module = MirModule::new(module_name.to_string());
    module.metadata.typed_object_plans.extend(typed_layout);
    let caller = make_function_with_global_call_args(symbol, Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: symbol.to_string(),
            params: vec![],
            return_type,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::NewBox {
        dst: ValueId::new(1),
        target: crate::mir::ConstructionTarget::Named(box_name.to_string()),
        args: vec![],
    });
    let returned = if copy_result {
        entry.instructions.push(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        ValueId::new(2)
    } else {
        ValueId::new(1)
    };
    entry.set_terminator(MirInstruction::Return {
        value: Some(returned),
    });
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(symbol.to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    module
}

#[test]
fn refresh_module_global_call_routes_accepts_typed_object_handle_return() {
    let module = same_module_constructed_handle(
        "global_call_typed_object_return_test",
        "TreeFactory.make/0",
        "TreeNode",
        MirType::Box("TreeNode".to_string()),
        Some(crate::mir::function::TypedObjectPlan {
            box_name: "TreeNode".to_string(),
            type_id: 7,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        }),
        false,
    );
    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.reason(), None);
    assert_eq!(route.return_shape(), Some("object_handle"));
    assert_eq!(route.target_result_box_name(), Some("TreeNode"));
    assert_eq!(route.definition_owner(), "uniform_mir");
    assert_eq!(route.proof(), "typed_global_call_same_module_object_handle");
}

#[test]
fn refresh_module_global_call_routes_accepts_builtin_map_handle_return() {
    let module = same_module_constructed_handle(
        "global_call_builtin_map_return_test",
        "Helper.make_map/0",
        "MapBox",
        MirType::Box("MapBox".to_string()),
        None,
        true,
    );
    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.reason(), None);
    assert_eq!(route.return_shape(), Some("map_handle"));
    assert_eq!(route.target_result_box_name(), Some("MapBox"));
    assert_eq!(route.definition_owner(), "uniform_mir");
    assert_eq!(route.proof(), "typed_global_call_same_module_object_handle");
}

#[test]
fn refresh_module_global_call_routes_rejects_legacy_scanner_void_map_return() {
    let mut module = MirModule::new("global_call_legacy_scanner_void_map_return_test".to_string());
    let caller = make_function_with_global_call_args(
        "ProgramJsonV0ScannerBox.read_int_field_in_obj/3",
        Some(ValueId::new(7)),
        vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "ProgramJsonV0ScannerBox.read_int_field_in_obj/3".to_string(),
            params: vec![MirType::String, MirType::String, MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::NewBox {
        dst: ValueId::new(1),
        target: crate::mir::ConstructionTarget::Named("MapBox".to_string()),
        args: vec![],
    });
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "ProgramJsonV0ScannerBox.read_int_field_in_obj/3".to_string(),
        callee,
    );

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_ne!(route.reason(), None);
    assert_eq!(route.return_shape(), None);
    assert_eq!(route.target_result_box_name(), None);
}

#[test]
fn refresh_module_global_call_routes_accepts_unknown_signature_builtin_array_handle_return() {
    let module = same_module_constructed_handle(
        "global_call_unknown_array_return_test",
        "Helper.array_or_empty/0",
        "ArrayBox",
        MirType::Unknown,
        None,
        false,
    );
    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.reason(), None, "{route:?}");
    assert_eq!(route.return_shape(), Some("object_handle"));
    assert_eq!(route.target_result_box_name(), None);
    assert_eq!(route.definition_owner(), "uniform_mir");
    assert_eq!(route.proof(), "typed_global_call_same_module_object_handle");
}

#[test]
fn refresh_module_global_call_routes_accepts_same_module_variant_handle_return() {
    let mut module =
        MirModule::new("global_call_same_module_variant_handle_return_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.kind/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.kind/0".to_string(),
            params: vec![],
            return_type: MirType::Box("MirValueKind".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::VariantMake {
        dst: ValueId::new(1),
        enum_name: "MirValueKind".to_string(),
        variant: "Temporary".to_string(),
        tag: 3,
        payload: None,
        payload_type: None,
    });
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.kind/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.reason(), None, "{route:?}");
    assert_eq!(route.return_shape(), Some("object_handle"));
    assert_eq!(route.target_result_box_name(), Some("MirValueKind"));
    assert_eq!(route.definition_owner(), "uniform_mir");
    assert_eq!(route.proof(), "typed_global_call_same_module_object_handle");
}

#[test]
fn refresh_module_global_call_routes_accepts_same_module_option_variant_handle_return() {
    let mut module =
        MirModule::new("global_call_same_module_option_variant_return_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.maybe_kind/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.maybe_kind/0".to_string(),
            params: vec![],
            return_type: MirType::Box("Option<MirValueKind>".to_string()),
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

    let mut some_block = BasicBlock::new(BasicBlockId::new(1));
    some_block.instructions.push(MirInstruction::VariantMake {
        dst: ValueId::new(2),
        enum_name: "MirValueKind".to_string(),
        variant: "Temporary".to_string(),
        tag: 3,
        payload: None,
        payload_type: None,
    });
    some_block.instructions.push(MirInstruction::VariantMake {
        dst: ValueId::new(3),
        enum_name: "Option".to_string(),
        variant: "Some".to_string(),
        tag: 1,
        payload: Some(ValueId::new(2)),
        payload_type: Some(MirType::Box("MirValueKind".to_string())),
    });
    some_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });

    let mut none_block = BasicBlock::new(BasicBlockId::new(2));
    none_block.instructions.push(MirInstruction::VariantMake {
        dst: ValueId::new(4),
        enum_name: "Option".to_string(),
        variant: "None".to_string(),
        tag: 0,
        payload: None,
        payload_type: None,
    });
    none_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });

    callee.blocks.insert(BasicBlockId::new(1), some_block);
    callee.blocks.insert(BasicBlockId::new(2), none_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.maybe_kind/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.reason(), None, "{route:?}");
    assert_eq!(route.return_shape(), Some("object_handle"));
    assert_eq!(route.target_result_box_name(), Some("Option<MirValueKind>"));
    assert_eq!(route.definition_owner(), "uniform_mir");
    assert_eq!(route.proof(), "typed_global_call_same_module_object_handle");
}
