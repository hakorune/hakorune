use super::*;

fn build_i64_coerce_helper(name: &str) -> MirFunction {
    let mut helper = MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    helper.params = vec![ValueId::new(1)];
    let block = helper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(0),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    helper
}

fn push_pattern_util_scan_surface(block: &mut BasicBlock, next: &mut u32) {
    for text in [
        "\"type\":\"Local\"",
        "\"name\":\"",
        "\"value\":",
        "\"expr\":{\"type\":\"Int\"",
        "\"expr\":{\"type\":\"Binary\"",
        "\"expr\":{\"type\":\"Bool\"",
        "\"expr\":{\"type\":\"Compare\"",
    ] {
        let dst = ValueId::new(*next);
        *next += 1;
        block.instructions.push(MirInstruction::Const {
            dst,
            value: ConstValue::String(text.to_string()),
        });
    }
}

fn push_global_call(block: &mut BasicBlock, dst: u32, name: &str, args: Vec<ValueId>) -> ValueId {
    let dst = ValueId::new(dst);
    block.instructions.push(MirInstruction::Call {
        dst: Some(dst),
        func: ValueId::INVALID,
        callee: Some(Callee::Global(name.to_string())),
        args,
        effects: EffectMask::PURE,
    });
    dst
}

fn build_local_int_probe(name: &str, coerce_name: &str) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: vec![MirType::Unknown, MirType::Unknown, MirType::Unknown],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.params = vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)];
    let block = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    let mut next = 10;
    block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Null,
    });
    push_pattern_util_scan_surface(block, &mut next);
    for dst in 20..23 {
        push_global_call(
            block,
            dst,
            "JsonFragBox.index_of_from/3",
            vec![ValueId::new(1), ValueId::new(10), ValueId::new(3)],
        );
    }
    push_global_call(
        block,
        23,
        "JsonFragBox.read_string_after/2",
        vec![ValueId::new(1), ValueId::new(20)],
    );
    let digits = push_global_call(
        block,
        24,
        "JsonFragBox.read_int_after/2",
        vec![ValueId::new(1), ValueId::new(21)],
    );
    let recursive = push_global_call(
        block,
        25,
        name,
        vec![ValueId::new(1), ValueId::new(23), ValueId::new(3)],
    );
    let coerced = push_global_call(block, 26, coerce_name, vec![recursive]);
    block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(27),
        value: ConstValue::Integer(1),
    });
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId::new(28),
        op: BinaryOp::Add,
        lhs: coerced,
        rhs: ValueId::new(27),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(digits),
    });
    function
}

fn build_local_bool_probe(name: &str, int_probe_name: &str, coerce_name: &str) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: vec![MirType::Unknown, MirType::Unknown, MirType::Unknown],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.params = vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)];
    let block = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    let mut next = 10;
    block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Void,
    });
    push_pattern_util_scan_surface(block, &mut next);
    for dst in 20..23 {
        push_global_call(
            block,
            dst,
            "JsonFragBox.index_of_from/3",
            vec![ValueId::new(1), ValueId::new(10), ValueId::new(3)],
        );
    }
    push_global_call(
        block,
        23,
        "JsonFragBox.read_string_after/2",
        vec![ValueId::new(1), ValueId::new(20)],
    );
    push_global_call(
        block,
        24,
        "JsonFragBox.read_int_after/2",
        vec![ValueId::new(1), ValueId::new(21)],
    );
    let bool_value = push_global_call(
        block,
        25,
        "JsonFragBox.read_bool_after/2",
        vec![ValueId::new(1), ValueId::new(22)],
    );
    let lhs = push_global_call(
        block,
        26,
        int_probe_name,
        vec![ValueId::new(1), ValueId::new(23), ValueId::new(3)],
    );
    let rhs = push_global_call(
        block,
        27,
        int_probe_name,
        vec![ValueId::new(1), ValueId::new(23), ValueId::new(3)],
    );
    let lhs_i64 = push_global_call(block, 28, coerce_name, vec![lhs]);
    let rhs_i64 = push_global_call(block, 29, coerce_name, vec![rhs]);
    block.instructions.push(MirInstruction::Compare {
        dst: ValueId::new(30),
        op: CompareOp::Eq,
        lhs: lhs_i64,
        rhs: rhs_i64,
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(bool_value),
    });
    function
}

#[test]
fn refresh_module_global_call_routes_accepts_pattern_util_local_value_probe_contract() {
    let mut module = MirModule::new("global_call_pattern_util_local_value_probe_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.find_local_int_before/3",
        Some(ValueId::new(40)),
        vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
    );
    let probe = build_local_int_probe(
        "Helper.find_local_int_before/3",
        "Helper._coerce_i64_compat/1",
    );
    let coerce = build_i64_coerce_helper("Helper._coerce_i64_compat/1");

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.find_local_int_before/3".to_string(), probe);
    module
        .functions
        .insert("Helper._coerce_i64_compat/1".to_string(), coerce);

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
        "typed_global_call_pattern_util_local_value_probe"
    );
    assert_eq!(route.return_shape(), Some("mixed_runtime_i64_or_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.definition_owner(), "uniform_mir");
    assert_eq!(
        route.emit_trace_consumer(),
        "mir_call_global_uniform_mir_emit"
    );
}

#[test]
fn refresh_module_global_call_routes_accepts_pattern_util_bool_probe_via_local_value_child() {
    let mut module = MirModule::new("global_call_pattern_util_bool_probe_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.find_local_bool_before/3",
        Some(ValueId::new(40)),
        vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
    );
    let int_probe = build_local_int_probe(
        "Helper.find_local_int_before/3",
        "Helper._coerce_i64_compat/1",
    );
    let bool_probe = build_local_bool_probe(
        "Helper.find_local_bool_before/3",
        "Helper.find_local_int_before/3",
        "Helper._coerce_i64_compat/1",
    );
    let coerce = build_i64_coerce_helper("Helper._coerce_i64_compat/1");

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.find_local_int_before/3".to_string(), int_probe);
    module
        .functions
        .insert("Helper.find_local_bool_before/3".to_string(), bool_probe);
    module
        .functions
        .insert("Helper._coerce_i64_compat/1".to_string(), coerce);

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
        "typed_global_call_pattern_util_local_value_probe"
    );
    assert_eq!(route.return_shape(), Some("mixed_runtime_i64_or_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");

    let int_child_routes = &module.functions["Helper.find_local_bool_before/3"]
        .metadata
        .global_call_routes;
    assert!(int_child_routes.iter().any(|route| {
        route.callee_name() == "Helper.find_local_int_before/3"
            && route.proof() == "typed_global_call_pattern_util_local_value_probe"
            && route.return_shape() == Some("mixed_runtime_i64_or_handle")
    }));
}
