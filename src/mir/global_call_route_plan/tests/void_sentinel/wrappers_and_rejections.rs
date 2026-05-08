use super::*;

#[test]
fn refresh_module_global_call_routes_accepts_unknown_return_string_or_void_wrapper() {
    let mut module = MirModule::new("global_call_unknown_return_string_or_void_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.methodize/1",
        Some(ValueId::new(20)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.methodize/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(2),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut pass_block = BasicBlock::new(BasicBlockId::new(1));
    pass_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    let mut child_or_null = BasicBlock::new(BasicBlockId::new(2));
    child_or_null.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Bool(true),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.child/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
    ]);
    child_or_null.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(3),
        then_bb: BasicBlockId::new(3),
        else_bb: BasicBlockId::new(4),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut child_return = BasicBlock::new(BasicBlockId::new(3));
    child_return.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    let mut void_return = BasicBlock::new(BasicBlockId::new(4));
    void_return.instructions.push(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::Void,
    });
    void_return.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    callee.blocks.insert(BasicBlockId::new(1), pass_block);
    callee.blocks.insert(BasicBlockId::new(2), child_or_null);
    callee.blocks.insert(BasicBlockId::new(3), child_return);
    callee.blocks.insert(BasicBlockId::new(4), void_return);

    let mut child = MirFunction::new(
        FunctionSignature {
            name: "Helper.child/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    child.params = vec![ValueId::new(1)];
    let child_block = child.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    child_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("ok".to_string()),
    });
    child_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.methodize/1".to_string(), callee);
    module.functions.insert("Helper.child/1".to_string(), child);

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
fn refresh_module_global_call_routes_accepts_string_typed_string_or_void_passthrough_wrapper() {
    let mut module =
        MirModule::new("global_call_string_typed_string_or_void_wrapper_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.methodize/1",
        Some(ValueId::new(20)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.methodize/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(2),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut pass_block = BasicBlock::new(BasicBlockId::new(1));
    pass_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    let mut child_or_null = BasicBlock::new(BasicBlockId::new(2));
    child_or_null.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Bool(true),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.child/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
    ]);
    child_or_null.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(3),
        then_bb: BasicBlockId::new(3),
        else_bb: BasicBlockId::new(4),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut child_return = BasicBlock::new(BasicBlockId::new(3));
    child_return.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    let mut void_return = BasicBlock::new(BasicBlockId::new(4));
    void_return.instructions.push(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::Void,
    });
    void_return.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    callee.blocks.insert(BasicBlockId::new(1), pass_block);
    callee.blocks.insert(BasicBlockId::new(2), child_or_null);
    callee.blocks.insert(BasicBlockId::new(3), child_return);
    callee.blocks.insert(BasicBlockId::new(4), void_return);

    let mut child = MirFunction::new(
        FunctionSignature {
            name: "Helper.child/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    child.params = vec![ValueId::new(1)];
    let child_block = child.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    child_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("ok".to_string()),
    });
    child_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.methodize/1".to_string(), callee);
    module.functions.insert("Helper.child/1".to_string(), child);

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
fn refresh_module_global_call_routes_rejects_string_typed_unknown_param_or_void_without_string_evidence(
) {
    let mut module =
        MirModule::new("global_call_string_typed_param_or_void_reject_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.pass_or_null/1",
        Some(ValueId::new(20)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.pass_or_null/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(2),
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
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    callee.blocks.insert(BasicBlockId::new(1), pass_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.pass_or_null/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_unsupported_void_sentinel_const")
    );
}

#[test]
fn refresh_module_global_call_routes_accepts_bool_not_in_string_or_void_body() {
    let mut module = MirModule::new("global_call_bool_not_string_or_void_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.not_or_text/1",
        Some(ValueId::new(20)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.not_or_text/1".to_string(),
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
            value: ConstValue::Bool(true),
        },
        MirInstruction::UnaryOp {
            dst: ValueId::new(3),
            op: UnaryOp::Not,
            operand: ValueId::new(2),
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
    text_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::String("ok".to_string()),
    });
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    callee.blocks.insert(BasicBlockId::new(1), void_block);
    callee.blocks.insert(BasicBlockId::new(2), text_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.not_or_text/1".to_string(), callee);

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
}

#[test]
fn refresh_module_global_call_routes_rejects_unknown_return_param_or_void_without_string_evidence()
{
    let mut module =
        MirModule::new("global_call_unknown_return_param_or_void_reject_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.pass_or_null/1",
        Some(ValueId::new(20)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.pass_or_null/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(2),
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
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    callee.blocks.insert(BasicBlockId::new(1), pass_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.pass_or_null/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_unsupported_void_sentinel_const")
    );
}

#[test]
fn refresh_module_global_call_routes_accepts_string_concat_loop_with_env_set() {
    let mut module = MirModule::new("global_call_string_concat_loop_profile_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.concat_loop_or_null/0",
        Some(ValueId::new(20)),
        vec![],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.concat_loop_or_null/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::String("HAKO_STAGEB_USING_DEPTH".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("0".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern("env.set/2".to_string())),
            args: vec![ValueId::new(1), ValueId::new(2)],
            effects: EffectMask::IO,
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::String(String::new()),
        },
        MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::String("body".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::String("\n".to_string()),
        },
    ]);
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });

    let mut header_block = BasicBlock::new(BasicBlockId::new(1));
    header_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(7),
            inputs: vec![
                (BasicBlockId::new(0), ValueId::new(4)),
                (BasicBlockId::new(2), ValueId::new(10)),
            ],
            type_hint: None,
        },
        MirInstruction::Const {
            dst: ValueId::new(11),
            value: ConstValue::Bool(true),
        },
    ]);
    header_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(11),
        then_bb: BasicBlockId::new(2),
        else_bb: BasicBlockId::new(3),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut append_block = BasicBlock::new(BasicBlockId::new(2));
    append_block.instructions.extend([
        MirInstruction::BinOp {
            dst: ValueId::new(8),
            op: BinaryOp::Add,
            lhs: ValueId::new(7),
            rhs: ValueId::new(6),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(9),
            op: BinaryOp::Add,
            lhs: ValueId::new(8),
            rhs: ValueId::new(5),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(10),
            op: BinaryOp::Add,
            lhs: ValueId::new(9),
            rhs: ValueId::new(6),
        },
        MirInstruction::Const {
            dst: ValueId::new(13),
            value: ConstValue::Bool(false),
        },
    ]);
    append_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(13),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(4),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut void_block = BasicBlock::new(BasicBlockId::new(3));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(12),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(12)),
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(4));
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });
    callee
        .metadata
        .value_types
        .insert(ValueId::new(7), MirType::Integer);
    callee
        .metadata
        .value_types
        .insert(ValueId::new(8), MirType::Integer);
    callee
        .metadata
        .value_types
        .insert(ValueId::new(9), MirType::Integer);
    callee
        .metadata
        .value_types
        .insert(ValueId::new(10), MirType::Integer);
    callee.blocks.insert(BasicBlockId::new(1), header_block);
    callee.blocks.insert(BasicBlockId::new(2), append_block);
    callee.blocks.insert(BasicBlockId::new(3), void_block);
    callee.blocks.insert(BasicBlockId::new(4), text_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.concat_loop_or_null/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}
