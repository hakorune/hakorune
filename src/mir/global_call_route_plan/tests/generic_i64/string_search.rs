use super::*;

#[test]
fn refresh_module_global_call_routes_marks_string_scan_generic_i64_body() {
    let mut module = MirModule::new("global_call_string_scan_i64_body_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.find/3",
        Some(ValueId::new(20)),
        vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.find/3".to_string(),
            params: vec![MirType::Unknown, MirType::Unknown, MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)];
    callee
        .metadata
        .value_types
        .insert(ValueId::new(3), MirType::Integer);
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "length".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(5)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "length".to_string(),
                receiver: Some(ValueId::new(2)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::BinOp {
            dst: ValueId::new(6),
            op: BinaryOp::Add,
            lhs: ValueId::new(3),
            rhs: ValueId::new(5),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(3), ValueId::new(6)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Compare {
            dst: ValueId::new(8),
            op: CompareOp::Eq,
            lhs: ValueId::new(7),
            rhs: ValueId::new(2),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(8),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut found_block = BasicBlock::new(BasicBlockId::new(1));
    found_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    let mut missing_block = BasicBlock::new(BasicBlockId::new(2));
    missing_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(9),
        value: ConstValue::Integer(-1),
    });
    missing_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(9)),
    });
    callee.blocks.insert(BasicBlockId::new(1), found_block);
    callee.blocks.insert(BasicBlockId::new(2), missing_block);
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.find/3".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}

#[test]
fn refresh_module_global_call_routes_accepts_generic_i64_select_flow() {
    let mut module = MirModule::new("global_call_generic_i64_select_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.pick_next/2",
        Some(ValueId::new(20)),
        vec![ValueId::new(1), ValueId::new(2)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.pick_next/2".to_string(),
            params: vec![MirType::String, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(1),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(4),
            op: BinaryOp::Add,
            lhs: ValueId::new(2),
            rhs: ValueId::new(3),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(5)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2), ValueId::new(4)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::String("[".to_string()),
        },
        MirInstruction::Compare {
            dst: ValueId::new(7),
            op: CompareOp::Eq,
            lhs: ValueId::new(5),
            rhs: ValueId::new(6),
        },
        MirInstruction::Select {
            dst: ValueId::new(8),
            cond: ValueId::new(7),
            then_val: ValueId::new(4),
            else_val: ValueId::new(2),
        },
    ]);
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(8)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.pick_next/2".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}

#[test]
fn refresh_module_global_call_routes_accepts_string_contains_not_scalar_body() {
    let mut module = MirModule::new("global_call_generic_i64_string_contains_not_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.header_present/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.header_present/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("\"version\"".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "contains".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2)],
            effects: EffectMask::PURE,
        },
        MirInstruction::UnaryOp {
            dst: ValueId::new(4),
            op: UnaryOp::Not,
            operand: ValueId::new(3),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(4),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut missing_block = BasicBlock::new(BasicBlockId::new(1));
    missing_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::Integer(0),
    });
    missing_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    let mut present_block = BasicBlock::new(BasicBlockId::new(2));
    present_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(6),
        value: ConstValue::Integer(1),
    });
    present_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });
    callee.blocks.insert(BasicBlockId::new(1), missing_block);
    callee.blocks.insert(BasicBlockId::new(2), present_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.header_present/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}

#[test]
fn refresh_module_global_call_routes_accepts_string_indexof_generic_i64_body() {
    let mut module = MirModule::new("global_call_string_indexof_i64_body_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.index_of_version/1",
        Some(ValueId::new(10)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.index_of_version/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("\"version\":0".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "indexOf".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2)],
            effects: EffectMask::PURE,
        },
    ]);
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.index_of_version/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}

#[test]
fn refresh_module_global_call_routes_accepts_string_lastindexof_generic_i64_body() {
    let mut module = MirModule::new("global_call_string_lastindexof_i64_body_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.last_index_of_version/1",
        Some(ValueId::new(10)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.last_index_of_version/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("\"version\"".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "lastIndexOf".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2)],
            effects: EffectMask::PURE,
        },
    ]);
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.last_index_of_version/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}

#[test]
fn refresh_module_global_call_routes_infers_unknown_string_search_needle_generic_i64_body() {
    let mut module = MirModule::new("global_call_string_search_unknown_needle_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.last_index_of_unknown_needle/2",
        Some(ValueId::new(10)),
        vec![ValueId::new(1), ValueId::new(2)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.last_index_of_unknown_needle/2".to_string(),
            params: vec![MirType::Unknown, MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "lastIndexOf".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Union,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(2)],
        effects: EffectMask::PURE,
    });
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.last_index_of_unknown_needle/2".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}

#[test]
fn refresh_module_global_call_routes_accepts_string_ordered_compare_generic_i64_body() {
    let mut module = MirModule::new("global_call_string_ordered_compare_i64_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.digit_floor/1",
        Some(ValueId::new(10)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.digit_floor/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(0),
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2), ValueId::new(3)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::String("0".to_string()),
        },
        MirInstruction::Compare {
            dst: ValueId::new(6),
            op: CompareOp::Lt,
            lhs: ValueId::new(4),
            rhs: ValueId::new(5),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(6),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut below = BasicBlock::new(BasicBlockId::new(1));
    below.instructions.push(MirInstruction::Const {
        dst: ValueId::new(7),
        value: ConstValue::Integer(0),
    });
    below.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(7)),
    });
    let mut ok = BasicBlock::new(BasicBlockId::new(2));
    ok.instructions.push(MirInstruction::Const {
        dst: ValueId::new(8),
        value: ConstValue::Integer(1),
    });
    ok.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(8)),
    });
    callee.blocks.insert(BasicBlockId::new(1), below);
    callee.blocks.insert(BasicBlockId::new(2), ok);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.digit_floor/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}
