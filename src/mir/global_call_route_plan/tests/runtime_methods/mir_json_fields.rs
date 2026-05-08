use super::*;

#[test]
fn refresh_module_global_call_routes_accepts_mir_json_emit_box_value_field_reads() {
    let mut module = MirModule::new("global_call_mir_json_emit_box_value_test".to_string());
    let caller = make_function_with_global_call_args(
        "MirJsonEmitBox._emit_box_value/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut emit_box_value = MirFunction::new(
        FunctionSignature {
            name: "MirJsonEmitBox._emit_box_value/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    emit_box_value.params = vec![ValueId::new(1)];
    let block = emit_box_value
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("type".to_string()),
        },
        method_call(
            Some(ValueId::new(3)),
            "RuntimeDataBox",
            "get",
            ValueId::new(1),
            vec![ValueId::new(2)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::String(String::new()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(5),
            op: BinaryOp::Add,
            lhs: ValueId::new(4),
            rhs: ValueId::new(3),
        },
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::String("value".to_string()),
        },
        method_call(
            Some(ValueId::new(7)),
            "RuntimeDataBox",
            "get",
            ValueId::new(1),
            vec![ValueId::new(6)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(8),
            value: ConstValue::String("{\"type\":".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(9),
            op: BinaryOp::Add,
            lhs: ValueId::new(8),
            rhs: ValueId::new(5),
        },
        MirInstruction::Const {
            dst: ValueId::new(10),
            value: ConstValue::String(",\"value\":".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(11),
            op: BinaryOp::Add,
            lhs: ValueId::new(9),
            rhs: ValueId::new(10),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(12),
            op: BinaryOp::Add,
            lhs: ValueId::new(11),
            rhs: ValueId::new(7),
        },
        MirInstruction::Const {
            dst: ValueId::new(13),
            value: ConstValue::String("}".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(14),
            op: BinaryOp::Add,
            lhs: ValueId::new(12),
            rhs: ValueId::new(13),
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(14)),
    });
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "MirJsonEmitBox._emit_box_value/1".to_string(),
        emit_box_value,
    );

    refresh_module_semantic_metadata(&mut module);

    let helper = &module.functions["MirJsonEmitBox._emit_box_value/1"];
    assert_eq!(helper.metadata.generic_method_routes.len(), 2);
    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_accepts_mir_json_function_field_reads() {
    let mut module = MirModule::new("global_call_mir_json_function_field_test".to_string());
    let caller = make_function_with_global_call_args(
        "MirJsonEmitBox._emit_function/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut emit_function = MirFunction::new(
        FunctionSignature {
            name: "MirJsonEmitBox._emit_function/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    emit_function.params = vec![ValueId::new(1)];
    let block = emit_function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("name".to_string()),
        },
        method_call(
            Some(ValueId::new(3)),
            "RuntimeDataBox",
            "get",
            ValueId::new(1),
            vec![ValueId::new(2)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::String(String::new()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(5),
            op: BinaryOp::Add,
            lhs: ValueId::new(4),
            rhs: ValueId::new(3),
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("MirJsonEmitBox._emit_function/1".to_string(), emit_function);

    refresh_module_semantic_metadata(&mut module);

    let helper = &module.functions["MirJsonEmitBox._emit_function/1"];
    assert!(helper.metadata.generic_method_routes.iter().any(|route| {
        route.route_id() == "generic_method.get"
            && route.proof_tag() == "mir_json_function_field"
            && route.key_const_text() == Some("name")
    }));
    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_accepts_collection_or_void_phi_for_mir_json_function_blocks() {
    let mut module = MirModule::new("global_call_collection_or_void_phi_test".to_string());
    let caller = make_function_with_global_call_args(
        "MirJsonEmitBox._emit_function/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut emit_function = MirFunction::new(
        FunctionSignature {
            name: "MirJsonEmitBox._emit_function/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    emit_function.params = vec![ValueId::new(1)];

    let entry = emit_function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("blocks".to_string()),
        },
        method_call(
            Some(ValueId::new(3)),
            "RuntimeDataBox",
            "get",
            ValueId::new(1),
            vec![ValueId::new(2)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Bool(false),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(4),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut null_assign = BasicBlock::new(BasicBlockId::new(1));
    null_assign.instructions.push(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::Void,
    });
    null_assign.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut present_assign = BasicBlock::new(BasicBlockId::new(2));
    present_assign.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut merge = BasicBlock::new(BasicBlockId::new(3));
    merge.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(6),
            inputs: vec![
                (BasicBlockId::new(1), ValueId::new(5)),
                (BasicBlockId::new(2), ValueId::new(3)),
            ],
            type_hint: None,
        },
        MirInstruction::Const {
            dst: ValueId::new(7),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(8),
            op: CompareOp::Eq,
            lhs: ValueId::new(6),
            rhs: ValueId::new(7),
        },
    ]);
    merge.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(8),
        then_bb: BasicBlockId::new(4),
        else_bb: BasicBlockId::new(5),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut null_return = BasicBlock::new(BasicBlockId::new(4));
    null_return.instructions.push(MirInstruction::Const {
        dst: ValueId::new(9),
        value: ConstValue::String("[]".to_string()),
    });
    null_return.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(9)),
    });

    let mut present_return = BasicBlock::new(BasicBlockId::new(5));
    present_return.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(10),
            inputs: vec![(BasicBlockId::new(3), ValueId::new(6))],
            type_hint: None,
        },
        method_call(
            Some(ValueId::new(11)),
            "RuntimeDataBox",
            "length",
            ValueId::new(10),
            vec![],
        ),
        MirInstruction::Const {
            dst: ValueId::new(12),
            value: ConstValue::String("[]".to_string()),
        },
    ]);
    present_return.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(12)),
    });

    emit_function
        .blocks
        .insert(BasicBlockId::new(1), null_assign);
    emit_function
        .blocks
        .insert(BasicBlockId::new(2), present_assign);
    emit_function.blocks.insert(BasicBlockId::new(3), merge);
    emit_function
        .blocks
        .insert(BasicBlockId::new(4), null_return);
    emit_function
        .blocks
        .insert(BasicBlockId::new(5), present_return);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("MirJsonEmitBox._emit_function/1".to_string(), emit_function);

    refresh_module_semantic_metadata(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_accepts_mir_json_flags_keys_route() {
    let mut module = MirModule::new("global_call_mir_json_flags_keys_test".to_string());
    let caller = make_function_with_global_call_args(
        "MirJsonEmitBox._emit_flags/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut emit_flags = MirFunction::new(
        FunctionSignature {
            name: "MirJsonEmitBox._emit_flags/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    emit_flags.params = vec![ValueId::new(1)];
    let block = emit_flags
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.instructions.extend([
        method_call(
            Some(ValueId::new(2)),
            "RuntimeDataBox",
            "keys",
            ValueId::new(1),
            vec![],
        ),
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("{}".to_string()),
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("MirJsonEmitBox._emit_flags/1".to_string(), emit_flags);

    refresh_module_semantic_metadata(&mut module);

    let helper = &module.functions["MirJsonEmitBox._emit_flags/1"];
    assert!(helper.metadata.generic_method_routes.iter().any(|route| {
        route.route_id() == "generic_method.keys"
            && route.proof_tag() == "mir_json_flags_keys"
            && route.route_kind_tag() == "map_keys_array"
    }));
    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_accepts_mir_json_flags_keys_null_guard() {
    let mut module = MirModule::new("global_call_mir_json_flags_keys_guard_test".to_string());
    let caller = make_function_with_global_call_args(
        "MirJsonEmitBox._emit_flags/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut emit_flags = MirFunction::new(
        FunctionSignature {
            name: "MirJsonEmitBox._emit_flags/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    emit_flags.params = vec![ValueId::new(1)];

    let entry = emit_flags
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    entry.instructions.extend([
        method_call(
            Some(ValueId::new(2)),
            "RuntimeDataBox",
            "keys",
            ValueId::new(1),
            vec![],
        ),
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(4),
            op: CompareOp::Eq,
            lhs: ValueId::new(2),
            rhs: ValueId::new(3),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(4),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut null_block = BasicBlock::new(BasicBlockId::new(1));
    null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::String("{}".to_string()),
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });

    let mut present_block = BasicBlock::new(BasicBlockId::new(2));
    present_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(6),
            inputs: vec![(BasicBlockId::new(0), ValueId::new(2))],
            type_hint: Some(MirType::Box("ArrayBox".to_string())),
        },
        method_call(
            Some(ValueId::new(7)),
            "RuntimeDataBox",
            "length",
            ValueId::new(6),
            vec![],
        ),
        MirInstruction::Const {
            dst: ValueId::new(8),
            value: ConstValue::String("{}".to_string()),
        },
    ]);
    present_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(8)),
    });

    emit_flags.blocks.insert(BasicBlockId::new(1), null_block);
    emit_flags
        .blocks
        .insert(BasicBlockId::new(2), present_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("MirJsonEmitBox._emit_flags/1".to_string(), emit_flags);

    refresh_module_semantic_metadata(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}
