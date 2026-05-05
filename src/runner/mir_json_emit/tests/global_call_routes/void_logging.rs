use super::*;

#[test]
fn build_mir_json_root_emits_direct_plan_for_void_logging_contract() {
    let mut module = crate::mir::MirModule::new("json_global_call_void_logging_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.log/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::IO,
        });

    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.log/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Void,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("log:".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(3),
            op: BinaryOp::Add,
            lhs: ValueId::new(2),
            rhs: ValueId::new(1),
        },
        MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Global("print".to_string())),
            args: vec![ValueId::new(3)],
            effects: EffectMask::IO,
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Void,
        },
    ]);
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });

    module.add_function(caller);
    module.add_function(callee);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_exists"], true);
    assert_eq!(route["target_return_type"], "void");
    assert_eq!(route["target_shape"], serde_json::Value::Null);
    assert_eq!(route["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(
        route["proof"],
        "typed_global_call_generic_string_void_logging"
    );
    assert_eq!(route["tier"], "DirectAbi");
    assert_eq!(route["emit_kind"], "direct_function_call");
    assert_eq!(route["return_shape"], "void_sentinel_i64_zero");
    assert_eq!(route["value_demand"], "scalar_i64");
    assert_eq!(route["result_origin"], "none");
    assert_eq!(route["definition_owner"], "uniform_mir");
    assert_eq!(
        route["emit_trace_consumer"],
        "mir_call_global_uniform_mir_emit"
    );

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["source"], "global_call_routes");
    assert_eq!(plan["target_exists"], true);
    assert_eq!(plan["target_return_type"], "void");
    assert_eq!(plan["target_shape"], serde_json::Value::Null);
    assert_eq!(plan["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(plan["tier"], "DirectAbi");
    assert_eq!(plan["emit_kind"], "direct_function_call");
    assert_eq!(
        plan["proof"],
        "typed_global_call_generic_string_void_logging"
    );
    assert_eq!(
        plan["route_proof"],
        "typed_global_call_generic_string_void_logging"
    );
    assert_eq!(plan["return_shape"], "void_sentinel_i64_zero");
    assert_eq!(plan["value_demand"], "scalar_i64");
    assert_eq!(plan["result_origin"], "none");
    assert_eq!(plan["definition_owner"], "uniform_mir");
    assert_eq!(
        plan["emit_trace_consumer"],
        "mir_call_global_uniform_mir_emit"
    );
}
