use super::*;

#[test]
fn refresh_module_global_call_routes_accepts_scalar_void_guard_in_string_or_void_body() {
    let mut module = MirModule::new("global_call_scalar_void_guard_string_body_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.lower_bool/1",
        Some(ValueId::new(40)),
        vec![ValueId::new(1)],
    );
    let mut parser = MirFunction::new(
        FunctionSignature {
            name: "Helper.read_bool/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(10),
    );
    parser.params = vec![ValueId::new(1)];
    let parser_entry = parser.blocks.get_mut(&BasicBlockId::new(10)).unwrap();
    parser_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Bool(true),
    });
    parser_entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(2),
        then_bb: BasicBlockId::new(11),
        else_bb: BasicBlockId::new(12),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut parser_scalar = BasicBlock::new(BasicBlockId::new(11));
    parser_scalar.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(1),
    });
    parser_scalar.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    let mut parser_null = BasicBlock::new(BasicBlockId::new(12));
    parser_null.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Void,
    });
    parser_null.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    parser.blocks.insert(BasicBlockId::new(11), parser_scalar);
    parser.blocks.insert(BasicBlockId::new(12), parser_null);

    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.lower_bool/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.read_bool/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
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
        value: ConstValue::Void,
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });

    let mut non_null = BasicBlock::new(BasicBlockId::new(2));
    non_null.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(6),
            inputs: vec![(BasicBlockId::new(0), ValueId::new(2))],
            type_hint: Some(MirType::Unknown),
        },
        MirInstruction::Const {
            dst: ValueId::new(7),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Compare {
            dst: ValueId::new(8),
            op: CompareOp::Eq,
            lhs: ValueId::new(6),
            rhs: ValueId::new(7),
        },
    ]);
    non_null.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(8),
        then_bb: BasicBlockId::new(3),
        else_bb: BasicBlockId::new(4),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut true_block = BasicBlock::new(BasicBlockId::new(3));
    true_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(9),
        value: ConstValue::String("true-json".to_string()),
    });
    true_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(9)),
    });

    let mut false_block = BasicBlock::new(BasicBlockId::new(4));
    false_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(10),
        value: ConstValue::String("false-json".to_string()),
    });
    false_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });

    callee.blocks.insert(BasicBlockId::new(1), null_block);
    callee.blocks.insert(BasicBlockId::new(2), non_null);
    callee.blocks.insert(BasicBlockId::new(3), true_block);
    callee.blocks.insert(BasicBlockId::new(4), false_block);

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.read_bool/1".to_string(), parser);
    module
        .functions
        .insert("Helper.lower_bool/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        None,
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}

#[test]
fn refresh_module_global_call_routes_accepts_mixed_param_substring_void_sentinel_body() {
    let mut module = MirModule::new("global_call_mixed_substring_void_sentinel_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.slice_or_null/2",
        Some(ValueId::new(7)),
        vec![ValueId::new(1), ValueId::new(2)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.slice_or_null/2".to_string(),
            params: vec![MirType::Unknown, MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String(String::new()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(4),
            op: BinaryOp::Add,
            lhs: ValueId::new(3),
            rhs: ValueId::new(1),
        },
        MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::Integer(0),
        },
        MirInstruction::Copy {
            dst: ValueId::new(12),
            src: ValueId::new(2),
        },
        MirInstruction::Compare {
            dst: ValueId::new(6),
            op: CompareOp::Lt,
            lhs: ValueId::new(12),
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
    let mut text_block = BasicBlock::new(BasicBlockId::new(1));
    text_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(7),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Copy {
            dst: ValueId::new(13),
            src: ValueId::new(2),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(8),
            op: BinaryOp::Add,
            lhs: ValueId::new(13),
            rhs: ValueId::new(7),
        },
        MirInstruction::Const {
            dst: ValueId::new(9),
            value: ConstValue::Integer(4),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId::new(4)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(13), ValueId::new(8)],
            effects: EffectMask::PURE,
        },
    ]);
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(2));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(11),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });
    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.slice_or_null/2".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        None,
        "reason={:?}",
        route.target_shape_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}

#[test]
fn refresh_module_global_call_routes_accepts_loop_scalar_phi_substring_void_sentinel_body() {
    let mut module =
        MirModule::new("global_call_loop_scalar_phi_substring_void_sentinel_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.scan_or_null/2",
        Some(ValueId::new(7)),
        vec![ValueId::new(1), ValueId::new(2)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.scan_or_null/2".to_string(),
            params: vec![MirType::String, MirType::Integer],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(0),
    });
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });

    let mut loop_block = BasicBlock::new(BasicBlockId::new(1));
    loop_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(4),
            inputs: vec![
                (BasicBlockId::new(0), ValueId::new(3)),
                (BasicBlockId::new(2), ValueId::new(8)),
            ],
            type_hint: Some(MirType::Integer),
        },
        MirInstruction::Compare {
            dst: ValueId::new(5),
            op: CompareOp::Lt,
            lhs: ValueId::new(4),
            rhs: ValueId::new(2),
        },
    ]);
    loop_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(5),
        then_bb: BasicBlockId::new(2),
        else_bb: BasicBlockId::new(3),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut back_block = BasicBlock::new(BasicBlockId::new(2));
    back_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::Integer(1),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(8),
            op: BinaryOp::Add,
            lhs: ValueId::new(4),
            rhs: ValueId::new(6),
        },
    ]);
    back_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });

    let mut exit_block = BasicBlock::new(BasicBlockId::new(3));
    exit_block.instructions.push(MirInstruction::Compare {
        dst: ValueId::new(9),
        op: CompareOp::Eq,
        lhs: ValueId::new(2),
        rhs: ValueId::new(3),
    });
    exit_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(9),
        then_bb: BasicBlockId::new(5),
        else_bb: BasicBlockId::new(4),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut text_block = BasicBlock::new(BasicBlockId::new(4));
    text_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(10)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "substring".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Union,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(3), ValueId::new(4)],
        effects: EffectMask::PURE,
    });
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });

    let mut void_block = BasicBlock::new(BasicBlockId::new(5));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(11),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });

    callee.blocks.insert(BasicBlockId::new(1), loop_block);
    callee.blocks.insert(BasicBlockId::new(2), back_block);
    callee.blocks.insert(BasicBlockId::new(3), exit_block);
    callee.blocks.insert(BasicBlockId::new(4), text_block);
    callee.blocks.insert(BasicBlockId::new(5), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.scan_or_null/2".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_return_type(), Some("?".to_string()));
    assert_eq!(
        route.target_shape(),
        None,
        "reason={:?}",
        route.target_shape_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}

#[test]
fn refresh_module_global_call_routes_accepts_unknown_return_void_sentinel_body() {
    let mut module = MirModule::new("global_call_unknown_return_void_sentinel_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.slice_or_null/1",
        Some(ValueId::new(20)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.slice_or_null/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Unknown,
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
    let mut void_block = BasicBlock::new(BasicBlockId::new(1));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(2));
    text_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::String(String::new()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(6),
            op: BinaryOp::Add,
            lhs: ValueId::new(5),
            rhs: ValueId::new(1),
        },
        MirInstruction::Const {
            dst: ValueId::new(7),
            value: ConstValue::Integer(0),
        },
        MirInstruction::Const {
            dst: ValueId::new(8),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(9)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId::new(6)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(7), ValueId::new(8)],
            effects: EffectMask::PURE,
        },
    ]);
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(9)),
    });
    callee.blocks.insert(BasicBlockId::new(1), void_block);
    callee.blocks.insert(BasicBlockId::new(2), text_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.slice_or_null/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        None,
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}

#[test]
fn refresh_module_global_call_routes_accepts_unknown_wrapper_returning_string_or_void_child() {
    let mut module =
        MirModule::new("global_call_unknown_wrapper_string_or_void_child_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.wrapper/0", Some(ValueId::new(20)), vec![]);

    let mut child = MirFunction::new(
        FunctionSignature {
            name: "Helper.child/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let child_entry = child.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    child_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Bool(true),
    });
    child_entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut child_text = BasicBlock::new(BasicBlockId::new(1));
    child_text.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("ok".to_string()),
    });
    child_text.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut child_void = BasicBlock::new(BasicBlockId::new(2));
    child_void.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    child_void.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    child.blocks.insert(BasicBlockId::new(1), child_text);
    child.blocks.insert(BasicBlockId::new(2), child_void);

    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.wrapper/0".to_string(),
            params: vec![],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let wrapper_entry = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_entry.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.child/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    wrapper_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });

    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.child/0".to_string(), child);
    module
        .functions
        .insert("Helper.wrapper/0".to_string(), wrapper);

    refresh_module_global_call_routes(&mut module);

    let child_route = &module.functions["Helper.wrapper/0"]
        .metadata
        .global_call_routes[0];
    assert_eq!(child_route.target_shape(), None);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        None,
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}

#[test]
fn refresh_module_global_call_routes_accepts_void_typed_unknown_param_or_void_sentinel_body() {
    let mut module = MirModule::new("global_call_unknown_param_or_void_sentinel_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.pass_or_null/1",
        Some(ValueId::new(20)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.pass_or_null/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Compare {
            dst: ValueId::new(3),
            op: CompareOp::Eq,
            lhs: ValueId::new(2),
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
    let mut pass_block = BasicBlock::new(BasicBlockId::new(1));
    pass_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(2));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    callee.blocks.insert(BasicBlockId::new(1), pass_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.pass_or_null/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        None,
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}
