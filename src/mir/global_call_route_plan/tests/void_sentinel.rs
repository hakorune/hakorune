use super::*;

#[test]
fn refresh_module_global_call_routes_marks_string_or_void_sentinel_body_direct_target() {
    let mut module = MirModule::new("global_call_void_sentinel_test".to_string());
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
    text_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("ok".to_string()),
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
    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.maybe_text/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert!(route.target_exists());
    assert_eq!(route.target_symbol(), Some("Helper.maybe_text/0"));
    assert_eq!(route.target_return_type(), Some("void".to_string()));
    assert_eq!(
        route.target_shape(),
        Some("generic_string_or_void_sentinel_body"),
        "reason={:?}",
        route.target_shape_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.reason(), None);
}

#[test]
fn refresh_module_global_call_routes_accepts_substring_void_sentinel_body() {
    let mut module = MirModule::new("global_call_substring_void_sentinel_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.slice_or_null/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.slice_or_null/1".to_string(),
            params: vec![MirType::String],
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
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(0),
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Integer(4),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(2),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(1));
    text_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(5)),
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
        value: Some(ValueId::new(5)),
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(2));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(6),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });
    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.slice_or_null/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_string_or_void_sentinel_body"),
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
        Some("generic_string_or_void_sentinel_body"),
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
        Some("generic_string_or_void_sentinel_body"),
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
    assert_eq!(
        route.target_shape(),
        Some("generic_string_or_void_sentinel_body")
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}

#[test]
fn refresh_module_global_call_routes_marks_void_sentinel_child_blocker() {
    let mut module = MirModule::new("global_call_void_sentinel_child_test".to_string());
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
    entry.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.flag/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(1));
    text_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("ok".to_string()),
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
    let flag = MirFunction::new(
        FunctionSignature {
            name: "Helper.flag/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.flag/0".to_string(), flag);
    module
        .functions
        .insert("Helper.maybe_text/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert!(route.target_exists());
    assert_eq!(route.target_symbol(), Some("Helper.maybe_text/0"));
    assert_eq!(route.target_return_type(), Some("void".to_string()));
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_global_target_shape_unknown")
    );
    assert_eq!(route.target_shape_blocker_symbol(), Some("Helper.flag/0"));
    assert_eq!(
        route.target_shape_blocker_reason(),
        Some("generic_string_no_string_surface")
    );
    assert_eq!(route.tier(), "Unsupported");
    assert_eq!(route.reason(), Some("missing_multi_function_emitter"));
}

#[test]
fn refresh_module_global_call_routes_marks_void_sentinel_return_child_blocker() {
    let mut module = MirModule::new("global_call_void_sentinel_return_child_test".to_string());
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
        callee: Some(Callee::Global("Helper.pending/0".to_string())),
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
    let pending = MirFunction::new(
        FunctionSignature {
            name: "Helper.pending/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.pending/0".to_string(), pending);
    module
        .functions
        .insert("Helper.maybe_text/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert!(route.target_exists());
    assert_eq!(route.target_symbol(), Some("Helper.maybe_text/0"));
    assert_eq!(route.target_return_type(), Some("void".to_string()));
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_global_target_shape_unknown")
    );
    assert_eq!(
        route.target_shape_blocker_symbol(),
        Some("Helper.pending/0")
    );
    assert_eq!(
        route.target_shape_blocker_reason(),
        Some("generic_string_no_string_surface")
    );
}

#[test]
fn refresh_module_global_call_routes_marks_void_typed_call_result_child_blocker() {
    let mut module = MirModule::new("global_call_void_typed_call_child_blocker_test".to_string());
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
    let mut child_block = BasicBlock::new(BasicBlockId::new(1));
    child_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.pending/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    child_block.set_terminator(MirInstruction::Return {
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
    callee
        .metadata
        .value_types
        .insert(ValueId::new(2), MirType::Void);
    let pending = MirFunction::new(
        FunctionSignature {
            name: "Helper.pending/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.blocks.insert(BasicBlockId::new(1), child_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.pending/0".to_string(), pending);
    module
        .functions
        .insert("Helper.maybe_text/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_global_target_shape_unknown")
    );
    assert_eq!(
        route.target_shape_blocker_symbol(),
        Some("Helper.pending/0")
    );
    assert_eq!(
        route.target_shape_blocker_reason(),
        Some("generic_string_no_string_surface")
    );
}

#[test]
fn string_return_blocker_ignores_direct_string_child_targets() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Helper.maybe_text/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
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
        callee: Some(Callee::Global("Helper.text/0".to_string())),
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
    function.blocks.insert(BasicBlockId::new(1), text_block);
    function.blocks.insert(BasicBlockId::new(2), void_block);
    let mut targets = BTreeMap::new();
    targets.insert(
        "Helper.text/0".to_string(),
        GlobalCallTargetFacts::present_with_shape(0, GlobalCallTargetShape::GenericPureStringBody),
    );

    assert_eq!(
        generic_string_void_sentinel_return_global_blocker(&function, &targets),
        None
    );
}

#[test]
fn refresh_module_global_call_routes_accepts_void_typed_direct_sentinel_child_return() {
    let mut module = MirModule::new("global_call_void_typed_sentinel_child_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.parent/0", Some(ValueId::new(7)), vec![]);
    let mut child = MirFunction::new(
        FunctionSignature {
            name: "Helper.child/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    child
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Bool(true),
        });
    child
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Branch {
            condition: ValueId::new(1),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
    let mut child_text_block = BasicBlock::new(BasicBlockId::new(1));
    child_text_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("ok".to_string()),
    });
    child_text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut child_void_block = BasicBlock::new(BasicBlockId::new(2));
    child_void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    child_void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    child.blocks.insert(BasicBlockId::new(1), child_text_block);
    child.blocks.insert(BasicBlockId::new(2), child_void_block);

    let mut parent = MirFunction::new(
        FunctionSignature {
            name: "Helper.parent/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    parent
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Bool(true),
        });
    parent
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Branch {
            condition: ValueId::new(1),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
    let mut parent_text_block = BasicBlock::new(BasicBlockId::new(1));
    parent_text_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.child/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    parent_text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut parent_void_block = BasicBlock::new(BasicBlockId::new(2));
    parent_void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    parent_void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    parent
        .blocks
        .insert(BasicBlockId::new(1), parent_text_block);
    parent
        .blocks
        .insert(BasicBlockId::new(2), parent_void_block);
    parent
        .metadata
        .value_types
        .insert(ValueId::new(2), MirType::Void);
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.child/0".to_string(), child);
    module
        .functions
        .insert("Helper.parent/0".to_string(), parent);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_string_or_void_sentinel_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}

#[test]
fn refresh_module_global_call_routes_uses_direct_child_route_over_void_metadata() {
    let mut module =
        MirModule::new("global_call_direct_child_route_over_void_metadata_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.resolve/1",
        Some(ValueId::new(30)),
        vec![ValueId::new(1)],
    );

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
    let mut child_text_block = BasicBlock::new(BasicBlockId::new(1));
    child_text_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("body".to_string()),
    });
    child_text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut child_void_block = BasicBlock::new(BasicBlockId::new(2));
    child_void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    child_void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    child.blocks.insert(BasicBlockId::new(1), child_text_block);
    child.blocks.insert(BasicBlockId::new(2), child_void_block);

    let mut parent = MirFunction::new(
        FunctionSignature {
            name: "Helper.resolve/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    parent.params = vec![ValueId::new(10)];
    let parent_entry = parent.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    parent_entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(11)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.child/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Copy {
            dst: ValueId::new(12),
            src: ValueId::new(11),
        },
        MirInstruction::Const {
            dst: ValueId::new(13),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(14),
            op: CompareOp::Ne,
            lhs: ValueId::new(12),
            rhs: ValueId::new(13),
        },
    ]);
    parent_entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(14),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut parent_child_block = BasicBlock::new(BasicBlockId::new(1));
    parent_child_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(12)),
    });
    let mut parent_fallback_block = BasicBlock::new(BasicBlockId::new(2));
    parent_fallback_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });
    parent
        .blocks
        .insert(BasicBlockId::new(1), parent_child_block);
    parent
        .blocks
        .insert(BasicBlockId::new(2), parent_fallback_block);
    parent
        .metadata
        .value_types
        .insert(ValueId::new(11), MirType::Void);
    parent
        .metadata
        .value_types
        .insert(ValueId::new(12), MirType::Void);

    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.child/0".to_string(), child);
    module
        .functions
        .insert("Helper.resolve/1".to_string(), parent);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_return_not_string"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
}
