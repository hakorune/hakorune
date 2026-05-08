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
        None,
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
    assert_eq!(route.definition_owner(), "uniform_mir");
    assert_eq!(
        route.emit_trace_consumer(),
        "mir_call_global_uniform_mir_emit"
    );
    assert_eq!(route.reason(), None);
}

#[test]
fn refresh_module_global_call_routes_accepts_direct_child_string_or_void_sentinel_body() {
    let mut module =
        MirModule::new("global_call_direct_child_string_or_void_sentinel_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.wrapper/0", Some(ValueId::new(20)), vec![]);
    let mut child = MirFunction::new(
        FunctionSignature {
            name: "Helper.child_text/0".to_string(),
            params: vec![],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let child_entry = child.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    child_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::String("ok".to_string()),
    });
    child_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
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
    let wrapper_entry = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Bool(true),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.child_text/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        },
    ]);
    wrapper_entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(1));
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
    wrapper.blocks.insert(BasicBlockId::new(1), text_block);
    wrapper.blocks.insert(BasicBlockId::new(2), void_block);

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.child_text/0".to_string(), child);
    module
        .functions
        .insert("Helper.wrapper/0".to_string(), wrapper);

    refresh_module_global_call_routes(&mut module);

    let child_route = &module.functions["Helper.wrapper/0"]
        .metadata
        .global_call_routes[0];
    assert_eq!(child_route.target_shape(), Some("generic_pure_string_body"));

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
fn refresh_module_global_call_routes_accepts_direct_child_string_with_void_compare_guard() {
    let mut module = MirModule::new("global_call_direct_child_string_void_guard_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.wrapper/0", Some(ValueId::new(20)), vec![]);
    let mut child = MirFunction::new(
        FunctionSignature {
            name: "Helper.child_text/0".to_string(),
            params: vec![],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let child_entry = child.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    child_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::String("ok".to_string()),
    });
    child_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
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
    let wrapper_entry = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(1)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.child_text/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        },
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
    wrapper_entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(3),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut empty_block = BasicBlock::new(BasicBlockId::new(1));
    empty_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::String(String::new()),
    });
    empty_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(2));
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    wrapper.blocks.insert(BasicBlockId::new(1), empty_block);
    wrapper.blocks.insert(BasicBlockId::new(2), text_block);

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.child_text/0".to_string(), child);
    module
        .functions
        .insert("Helper.wrapper/0".to_string(), wrapper);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_pure_string_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.return_shape(), Some("string_handle"));
}

#[test]
fn refresh_module_global_call_routes_accepts_debug_print_direct_child_string_guard() {
    let mut module = MirModule::new("global_call_debug_print_child_string_guard_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.wrapper/0", Some(ValueId::new(20)), vec![]);
    let mut child = MirFunction::new(
        FunctionSignature {
            name: "Helper.child_text/0".to_string(),
            params: vec![],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let child_entry = child.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    child_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::String("ok".to_string()),
    });
    child_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });

    let mut flag = MirFunction::new(
        FunctionSignature {
            name: "Helper.flag/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let flag_entry = flag.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    flag_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(1),
    });
    flag_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });

    let mut debug_text = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug_text/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    debug_text.params = vec![ValueId::new(1)];
    let debug_entry = debug_text.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    debug_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("dbg".to_string()),
    });
    debug_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
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
    let wrapper_entry = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(1)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.child_text/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.flag/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Compare {
            dst: ValueId::new(4),
            op: CompareOp::Eq,
            lhs: ValueId::new(2),
            rhs: ValueId::new(3),
        },
    ]);
    wrapper_entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(4),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut debug_block = BasicBlock::new(BasicBlockId::new(1));
    debug_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::String("prefix=".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(6)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.debug_text/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
        MirInstruction::BinOp {
            dst: ValueId::new(7),
            op: BinaryOp::Add,
            lhs: ValueId::new(5),
            rhs: ValueId::new(6),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(8)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("print".to_string())),
            args: vec![ValueId::new(7)],
            effects: EffectMask::IO,
        },
    ]);
    debug_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });
    let mut skip_debug = BasicBlock::new(BasicBlockId::new(2));
    skip_debug.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });
    let mut guard_block = BasicBlock::new(BasicBlockId::new(3));
    guard_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(9),
            value: ConstValue::String(String::new()),
        },
        MirInstruction::Compare {
            dst: ValueId::new(10),
            op: CompareOp::Eq,
            lhs: ValueId::new(1),
            rhs: ValueId::new(9),
        },
    ]);
    guard_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(10),
        then_bb: BasicBlockId::new(4),
        else_bb: BasicBlockId::new(5),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut empty_block = BasicBlock::new(BasicBlockId::new(4));
    empty_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(11),
        value: ConstValue::String(String::new()),
    });
    empty_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(5));
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    wrapper.blocks.insert(BasicBlockId::new(1), debug_block);
    wrapper.blocks.insert(BasicBlockId::new(2), skip_debug);
    wrapper.blocks.insert(BasicBlockId::new(3), guard_block);
    wrapper.blocks.insert(BasicBlockId::new(4), empty_block);
    wrapper.blocks.insert(BasicBlockId::new(5), text_block);

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.child_text/0".to_string(), child);
    module.functions.insert("Helper.flag/0".to_string(), flag);
    module
        .functions
        .insert("Helper.debug_text/1".to_string(), debug_text);
    module
        .functions
        .insert("Helper.wrapper/0".to_string(), wrapper);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_pure_string_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.return_shape(), Some("string_handle"));
}

#[test]
fn refresh_module_global_call_routes_accepts_string_typed_null_sentinel_body() {
    let mut module = MirModule::new("global_call_string_typed_null_sentinel_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.typed_maybe_text/0",
        Some(ValueId::new(7)),
        vec![],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.typed_maybe_text/0".to_string(),
            params: vec![],
            return_type: MirType::String,
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
    let mut null_block = BasicBlock::new(BasicBlockId::new(2));
    null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), null_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.typed_maybe_text/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_return_type(), Some("str".to_string()));
    assert_eq!(
        route.target_shape(),
        None,
        "reason={:?}",
        route.target_shape_reason()
    );
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}

#[test]
fn refresh_module_global_call_routes_accepts_integer_typed_string_or_void_sentinel_body() {
    let mut module = MirModule::new("global_call_integer_typed_string_or_void_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.integer_typed_digits/0",
        Some(ValueId::new(7)),
        vec![],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.integer_typed_digits/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
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
        value: ConstValue::String("123".to_string()),
    });
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut null_block = BasicBlock::new(BasicBlockId::new(2));
    null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), null_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.integer_typed_digits/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_return_type(), Some("i64".to_string()));
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
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
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
fn refresh_module_global_call_routes_accepts_corridor_fact_substring_void_sentinel_body() {
    let mut module =
        MirModule::new("global_call_corridor_fact_substring_void_sentinel_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.corridor_slice_or_null/3",
        Some(ValueId::new(9)),
        vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.corridor_slice_or_null/3".to_string(),
            params: vec![MirType::Unknown, MirType::Unknown, MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(4),
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
        args: vec![ValueId::new(2), ValueId::new(3)],
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

    callee.metadata.string_corridor_facts.insert(
        ValueId::new(5),
        StringCorridorFact::str_slice(StringCorridorCarrier::MethodCall),
    );
    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.corridor_slice_or_null/3".to_string(), callee);

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
