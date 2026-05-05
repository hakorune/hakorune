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

#[test]
fn build_mir_json_root_emits_direct_plan_for_pattern_util_local_value_probe_contract() {
    let mut module =
        crate::mir::MirModule::new("json_global_call_pattern_util_probe_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(40)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.find_local_int_before/3".to_string())),
            args: vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
            effects: EffectMask::PURE,
        });

    module.add_function(caller);
    module.add_function(build_local_int_probe(
        "Helper.find_local_int_before/3",
        "Helper._coerce_i64_compat/1",
    ));
    module.add_function(build_i64_coerce_helper("Helper._coerce_i64_compat/1"));
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_shape"], serde_json::Value::Null);
    assert_eq!(route["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(route["tier"], "DirectAbi");
    assert_eq!(route["emit_kind"], "direct_function_call");
    assert_eq!(
        route["proof"],
        "typed_global_call_pattern_util_local_value_probe"
    );
    assert_eq!(route["return_shape"], "mixed_runtime_i64_or_handle");
    assert_eq!(route["value_demand"], "runtime_i64_or_handle");
    assert_eq!(route["result_origin"], "none");
    assert_eq!(route["definition_owner"], "uniform_mir");
    assert_eq!(
        route["emit_trace_consumer"],
        "mir_call_global_uniform_mir_emit"
    );

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["source"], "global_call_routes");
    assert_eq!(plan["target_shape"], serde_json::Value::Null);
    assert_eq!(plan["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(plan["tier"], "DirectAbi");
    assert_eq!(plan["emit_kind"], "direct_function_call");
    assert_eq!(
        plan["proof"],
        "typed_global_call_pattern_util_local_value_probe"
    );
    assert_eq!(
        plan["route_proof"],
        "typed_global_call_pattern_util_local_value_probe"
    );
    assert_eq!(plan["return_shape"], "mixed_runtime_i64_or_handle");
    assert_eq!(plan["value_demand"], "runtime_i64_or_handle");
    assert_eq!(plan["result_origin"], "none");
    assert_eq!(plan["definition_owner"], "uniform_mir");
    assert_eq!(
        plan["emit_trace_consumer"],
        "mir_call_global_uniform_mir_emit"
    );
}
