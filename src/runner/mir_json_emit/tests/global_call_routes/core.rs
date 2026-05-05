use super::*;

#[test]
fn build_mir_json_root_emits_global_call_routes_and_unsupported_plan() {
    let mut function = make_function("main", true);
    function
        .metadata
        .global_call_routes
        .push(GlobalCallRoute::new(
            GlobalCallRouteSite::new(BasicBlockId::new(0), 0),
            "Stage1ModeContractBox.resolve_mode/0",
            0,
            Some(ValueId::new(45)),
            GlobalCallTargetFacts::missing(),
        ));
    let mut module = crate::mir::MirModule::new("json_global_call_routes_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["route_id"], "global.user_call");
    assert_eq!(route["block"], 0);
    assert_eq!(route["instruction_index"], 0);
    assert_eq!(route["callee_name"], "Stage1ModeContractBox.resolve_mode/0");
    assert_eq!(route["target_symbol"], serde_json::Value::Null);
    assert_eq!(route["core_op"], "UserGlobalCall");
    assert_eq!(route["tier"], "Unsupported");
    assert_eq!(route["emit_kind"], "unsupported");
    assert_eq!(route["proof"], "typed_global_call_contract_missing");
    assert_eq!(route["route_kind"], "global.user_call");
    assert_eq!(route["arity"], 0);
    assert_eq!(route["target_exists"], false);
    assert_eq!(route["target_arity"], serde_json::Value::Null);
    assert_eq!(route["target_return_type"], serde_json::Value::Null);
    assert_eq!(route["target_shape"], serde_json::Value::Null);
    assert_eq!(route["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(
        route["target_shape_blocker_symbol"],
        serde_json::Value::Null
    );
    assert_eq!(
        route["target_shape_blocker_reason"],
        serde_json::Value::Null
    );
    assert_eq!(route["arity_matches"], serde_json::Value::Null);
    assert_eq!(route["result_value"], 45);
    assert_eq!(route["return_shape"], serde_json::Value::Null);
    assert_eq!(route["value_demand"], "typed_global_call_contract_missing");
    assert_eq!(route["result_origin"], "none");
    assert_eq!(route["definition_owner"], "none");
    assert_eq!(route["emit_trace_consumer"], "mir_call_global_unknown_emit");
    assert_eq!(route["reason"], "unknown_global_callee");
    assert_eq!(route["effects"], serde_json::json!(["call.global"]));

    let lowering_plan = root["functions"][0]["metadata"]["lowering_plan"]
        .as_array()
        .expect("lowering_plan");
    assert_eq!(lowering_plan.len(), 1);
    let plan = &lowering_plan[0];
    assert_eq!(plan["site"], "b0.i0");
    assert_eq!(plan["source"], "global_call_routes");
    assert_eq!(plan["source_route_id"], "global.user_call");
    assert_eq!(plan["callee_name"], "Stage1ModeContractBox.resolve_mode/0");
    assert_eq!(plan["target_symbol"], serde_json::Value::Null);
    assert_eq!(plan["core_op"], "UserGlobalCall");
    assert_eq!(plan["tier"], "Unsupported");
    assert_eq!(plan["emit_kind"], "unsupported");
    assert_eq!(plan["symbol"], serde_json::Value::Null);
    assert_eq!(plan["proof"], "typed_global_call_contract_missing");
    assert_eq!(plan["route_proof"], "typed_global_call_contract_missing");
    assert_eq!(plan["route_kind"], "global.user_call");
    assert_eq!(plan["perf_proof"], false);
    assert_eq!(plan["arity"], 0);
    assert_eq!(plan["target_exists"], false);
    assert_eq!(plan["target_arity"], serde_json::Value::Null);
    assert_eq!(plan["target_return_type"], serde_json::Value::Null);
    assert_eq!(plan["target_shape"], serde_json::Value::Null);
    assert_eq!(plan["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(plan["target_shape_blocker_symbol"], serde_json::Value::Null);
    assert_eq!(plan["target_shape_blocker_reason"], serde_json::Value::Null);
    assert_eq!(plan["arity_matches"], serde_json::Value::Null);
    assert_eq!(plan["result_value"], 45);
    assert_eq!(plan["return_shape"], serde_json::Value::Null);
    assert_eq!(plan["value_demand"], "typed_global_call_contract_missing");
    assert_eq!(plan["result_origin"], "none");
    assert_eq!(plan["definition_owner"], "none");
    assert_eq!(plan["emit_trace_consumer"], "mir_call_global_unknown_emit");
    assert_eq!(plan["publication_policy"], serde_json::Value::Null);
    assert_eq!(plan["reason"], "unknown_global_callee");
    assert_eq!(plan["effects"], serde_json::json!(["call.global"]));
}

#[test]
fn refresh_function_global_call_routes_is_available_to_json_emit_tests() {
    let mut function = make_function("main", true);
    refresh_function_global_call_routes(&mut function);
    assert!(function.metadata.global_call_routes.is_empty());
}

#[test]
fn build_mir_json_root_emits_target_shape_reason_for_existing_unsupported_target() {
    let mut module = crate::mir::MirModule::new("json_global_call_target_reason_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.pending/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
    let callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.pending/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    module.add_function(caller);
    module.add_function(callee);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_exists"], true);
    assert_eq!(route["target_return_type"], "i64");
    assert_eq!(route["target_shape"], serde_json::Value::Null);
    assert_eq!(
        route["target_shape_reason"],
        "generic_string_no_string_surface"
    );
    assert_eq!(
        route["target_shape_blocker_symbol"],
        serde_json::Value::Null
    );
    assert_eq!(
        route["target_shape_blocker_reason"],
        serde_json::Value::Null
    );
    assert_eq!(route["reason"], "missing_multi_function_emitter");
    assert_eq!(route["definition_owner"], "uniform_mir");
    assert_eq!(
        route["emit_trace_consumer"],
        "mir_call_global_uniform_mir_emit"
    );

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["target_exists"], true);
    assert_eq!(plan["target_return_type"], "i64");
    assert_eq!(plan["target_shape"], serde_json::Value::Null);
    assert_eq!(
        plan["target_shape_reason"],
        "generic_string_no_string_surface"
    );
    assert_eq!(plan["target_shape_blocker_symbol"], serde_json::Value::Null);
    assert_eq!(plan["target_shape_blocker_reason"], serde_json::Value::Null);
    assert_eq!(plan["reason"], "missing_multi_function_emitter");
    assert_eq!(plan["definition_owner"], "uniform_mir");
    assert_eq!(
        plan["emit_trace_consumer"],
        "mir_call_global_uniform_mir_emit"
    );
}

#[test]
fn build_mir_json_root_emits_target_return_type_for_return_abi_blocker() {
    let mut module =
        crate::mir::MirModule::new("json_global_call_target_return_type_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.maybe/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
    let callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.maybe/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    module.add_function(caller);
    module.add_function(callee);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_exists"], true);
    assert_eq!(route["target_return_type"], "void");
    assert_eq!(route["target_shape"], serde_json::Value::Null);
    assert_eq!(
        route["target_shape_reason"],
        "generic_string_return_abi_not_handle_compatible"
    );

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["target_exists"], true);
    assert_eq!(plan["target_return_type"], "void");
    assert_eq!(plan["target_shape"], serde_json::Value::Null);
    assert_eq!(
        plan["target_shape_reason"],
        "generic_string_return_abi_not_handle_compatible"
    );
}
