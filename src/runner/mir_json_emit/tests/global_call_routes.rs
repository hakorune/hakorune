use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::global_call_route_plan::{
    refresh_function_global_call_routes, refresh_module_global_call_routes, GlobalCallRoute,
    GlobalCallRouteSite, GlobalCallTargetFacts,
};
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, Callee, CompareOp, ConstValue, EffectMask,
    FunctionSignature, MirFunction, MirInstruction, MirType, ValueId,
};

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

#[test]
fn build_mir_json_root_emits_target_reason_for_string_or_void_sentinel_candidate() {
    let mut module = crate::mir::MirModule::new("json_global_call_void_sentinel_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.maybe_text/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
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
        "generic_string_return_void_sentinel_candidate"
    );

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["target_exists"], true);
    assert_eq!(plan["target_return_type"], "void");
    assert_eq!(plan["target_shape"], serde_json::Value::Null);
    assert_eq!(
        plan["target_shape_reason"],
        "generic_string_return_void_sentinel_candidate"
    );
}

#[test]
fn build_mir_json_root_emits_child_blocker_for_string_or_void_sentinel_candidate() {
    let mut module =
        crate::mir::MirModule::new("json_global_call_void_sentinel_child_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.maybe_text/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
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
    module.add_function(caller);
    module.add_function(flag);
    module.add_function(callee);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_exists"], true);
    assert_eq!(route["target_return_type"], "void");
    assert_eq!(route["target_shape"], serde_json::Value::Null);
    assert_eq!(
        route["target_shape_reason"],
        "generic_string_global_target_shape_unknown"
    );
    assert_eq!(route["target_shape_blocker_symbol"], "Helper.flag/0");
    assert_eq!(
        route["target_shape_blocker_reason"],
        "generic_string_no_string_surface"
    );

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["target_exists"], true);
    assert_eq!(plan["target_return_type"], "void");
    assert_eq!(plan["target_shape"], serde_json::Value::Null);
    assert_eq!(
        plan["target_shape_reason"],
        "generic_string_global_target_shape_unknown"
    );
    assert_eq!(plan["target_shape_blocker_symbol"], "Helper.flag/0");
    assert_eq!(
        plan["target_shape_blocker_reason"],
        "generic_string_no_string_surface"
    );
}

#[test]
fn build_mir_json_root_emits_void_sentinel_const_shape_reason() {
    let mut module =
        crate::mir::MirModule::new("json_global_call_void_const_reason_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.flag/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        });
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.flag/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Void,
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    module.add_function(caller);
    module.add_function(callee);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_shape"], serde_json::Value::Null);
    assert_eq!(
        route["target_shape_reason"],
        "generic_string_unsupported_void_sentinel_const"
    );
    assert_eq!(
        route["target_shape_blocker_symbol"],
        serde_json::Value::Null
    );
    assert_eq!(
        route["target_shape_blocker_reason"],
        serde_json::Value::Null
    );

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["target_shape"], serde_json::Value::Null);
    assert_eq!(
        plan["target_shape_reason"],
        "generic_string_unsupported_void_sentinel_const"
    );
    assert_eq!(plan["target_shape_blocker_symbol"], serde_json::Value::Null);
    assert_eq!(plan["target_shape_blocker_reason"], serde_json::Value::Null);
}

#[test]
fn build_mir_json_root_emits_method_blocker_after_null_guard() {
    let mut module =
        crate::mir::MirModule::new("json_global_call_null_guard_method_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.preview/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        });
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.preview/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::String,
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
    let mut null_block = BasicBlock::new(BasicBlockId::new(1));
    null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::String("<null>".to_string()),
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    let mut method_block = BasicBlock::new(BasicBlockId::new(2));
    method_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(5)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "length".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    });
    method_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    callee.blocks.insert(BasicBlockId::new(1), null_block);
    callee.blocks.insert(BasicBlockId::new(2), method_block);
    module.add_function(caller);
    module.add_function(callee);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_shape"], serde_json::Value::Null);
    assert_eq!(
        route["target_shape_reason"],
        "generic_string_unsupported_method_call"
    );
    assert_eq!(
        route["target_shape_blocker_symbol"],
        serde_json::Value::Null
    );
    assert_eq!(
        route["target_shape_blocker_reason"],
        serde_json::Value::Null
    );

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["target_shape"], serde_json::Value::Null);
    assert_eq!(
        plan["target_shape_reason"],
        "generic_string_unsupported_method_call"
    );
    assert_eq!(plan["target_shape_blocker_symbol"], serde_json::Value::Null);
    assert_eq!(plan["target_shape_blocker_reason"], serde_json::Value::Null);
}

#[test]
fn build_mir_json_root_emits_direct_plan_for_generic_i64_global_call() {
    let mut module = crate::mir::MirModule::new("json_global_call_generic_i64_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.debug/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });

    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let wrapper_block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::String("DEBUG".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.flag/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
    ]);
    wrapper_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });

    let mut flag = MirFunction::new(
        FunctionSignature {
            name: "Helper.flag/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    flag.params = vec![ValueId::new(1)];
    let entry = flag.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern("env.get/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(4),
            op: CompareOp::Ne,
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
    let mut yes_block = BasicBlock::new(BasicBlockId::new(1));
    yes_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::Integer(1),
    });
    yes_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    let mut no_block = BasicBlock::new(BasicBlockId::new(2));
    no_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(6),
        value: ConstValue::Integer(0),
    });
    no_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });
    flag.blocks.insert(BasicBlockId::new(1), yes_block);
    flag.blocks.insert(BasicBlockId::new(2), no_block);

    module.add_function(caller);
    module.add_function(wrapper);
    module.add_function(flag);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_shape"], "generic_i64_body");
    assert_eq!(route["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(route["tier"], "DirectAbi");
    assert_eq!(route["emit_kind"], "direct_function_call");
    assert_eq!(route["proof"], "typed_global_call_generic_i64");
    assert_eq!(route["return_shape"], "ScalarI64");
    assert_eq!(route["value_demand"], "scalar_i64");
    assert_eq!(route["reason"], serde_json::Value::Null);

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["source"], "global_call_routes");
    assert_eq!(plan["target_shape"], "generic_i64_body");
    assert_eq!(plan["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(plan["tier"], "DirectAbi");
    assert_eq!(plan["emit_kind"], "direct_function_call");
    assert_eq!(plan["proof"], "typed_global_call_generic_i64");
    assert_eq!(plan["route_proof"], "typed_global_call_generic_i64");
    assert_eq!(plan["return_shape"], "ScalarI64");
    assert_eq!(plan["value_demand"], "scalar_i64");
    assert_eq!(plan["reason"], serde_json::Value::Null);
}

#[test]
fn build_mir_json_root_emits_target_shape_child_blocker_for_unknown_child_target() {
    let mut module = crate::mir::MirModule::new("json_global_call_child_blocker_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.wrapper/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.wrapper/0".to_string(),
            params: vec![],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    wrapper
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(1)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.pending/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
    wrapper
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(1)),
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
    module.add_function(caller);
    module.add_function(wrapper);
    module.add_function(pending);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(
        route["target_shape_reason"],
        "generic_string_global_target_shape_unknown"
    );
    assert_eq!(route["target_shape_blocker_symbol"], "Helper.pending/0");
    assert_eq!(
        route["target_shape_blocker_reason"],
        "generic_string_no_string_surface"
    );

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(
        plan["target_shape_reason"],
        "generic_string_global_target_shape_unknown"
    );
    assert_eq!(plan["target_shape_blocker_symbol"], "Helper.pending/0");
    assert_eq!(
        plan["target_shape_blocker_reason"],
        "generic_string_no_string_surface"
    );
}

#[test]
fn build_mir_json_root_emits_direct_plan_for_numeric_i64_leaf_global_call() {
    let mut module = crate::mir::MirModule::new("json_global_call_leaf_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.add/2".to_string())),
            args: vec![ValueId::new(1), ValueId::new(2)],
            effects: EffectMask::PURE,
        });
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.add/2".to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId::new(3),
        op: BinaryOp::Add,
        lhs: ValueId::new(1),
        rhs: ValueId::new(2),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    module.add_function(caller);
    module.add_function(callee);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_symbol"], "Helper.add/2");
    assert_eq!(route["target_return_type"], "i64");
    assert_eq!(route["target_shape"], "numeric_i64_leaf");
    assert_eq!(route["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(
        route["target_shape_blocker_symbol"],
        serde_json::Value::Null
    );
    assert_eq!(
        route["target_shape_blocker_reason"],
        serde_json::Value::Null
    );
    assert_eq!(route["tier"], "DirectAbi");
    assert_eq!(route["emit_kind"], "direct_function_call");
    assert_eq!(route["proof"], "typed_global_call_leaf_numeric_i64");
    assert_eq!(route["return_shape"], "ScalarI64");
    assert_eq!(route["value_demand"], "scalar_i64");
    assert_eq!(route["reason"], serde_json::Value::Null);

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["source"], "global_call_routes");
    assert_eq!(plan["source_route_id"], "global.user_call");
    assert_eq!(plan["core_op"], "UserGlobalCall");
    assert_eq!(plan["target_symbol"], "Helper.add/2");
    assert_eq!(plan["target_return_type"], "i64");
    assert_eq!(plan["target_shape"], "numeric_i64_leaf");
    assert_eq!(plan["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(plan["target_shape_blocker_symbol"], serde_json::Value::Null);
    assert_eq!(plan["target_shape_blocker_reason"], serde_json::Value::Null);
    assert_eq!(plan["tier"], "DirectAbi");
    assert_eq!(plan["emit_kind"], "direct_function_call");
    assert_eq!(plan["symbol"], "Helper.add/2");
    assert_eq!(plan["proof"], "typed_global_call_leaf_numeric_i64");
    assert_eq!(plan["route_proof"], "typed_global_call_leaf_numeric_i64");
    assert_eq!(plan["return_shape"], "ScalarI64");
    assert_eq!(plan["value_demand"], "scalar_i64");
    assert_eq!(plan["reason"], serde_json::Value::Null);
}

#[test]
fn build_mir_json_root_keeps_callee_name_and_emits_canonical_target_symbol() {
    let mut module =
        crate::mir::MirModule::new("json_global_call_static_entry_alias_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("main._helper/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Main._helper/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(42),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    module.add_function(caller);
    module.add_function(callee);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["callee_name"], "main._helper/0");
    assert_eq!(route["target_symbol"], "Main._helper/0");
    assert_eq!(route["target_return_type"], "i64");
    assert_eq!(route["target_shape"], "numeric_i64_leaf");
    assert_eq!(route["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(route["reason"], serde_json::Value::Null);

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["callee_name"], "main._helper/0");
    assert_eq!(plan["target_symbol"], "Main._helper/0");
    assert_eq!(plan["symbol"], "Main._helper/0");
    assert_eq!(plan["target_return_type"], "i64");
    assert_eq!(plan["target_shape"], "numeric_i64_leaf");
    assert_eq!(plan["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(plan["reason"], serde_json::Value::Null);
}

#[test]
fn build_mir_json_root_emits_direct_plan_for_generic_pure_string_global_call() {
    let mut module = crate::mir::MirModule::new("json_global_call_generic_string_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.normalize/2".to_string())),
            args: vec![ValueId::new(1), ValueId::new(2)],
            effects: EffectMask::PURE,
        });
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.normalize/2".to_string(),
            params: vec![MirType::String, MirType::String],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(9)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("dev".to_string()),
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

    let mut then_block = BasicBlock::new(BasicBlockId::new(1));
    then_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::String("vm".to_string()),
    });
    then_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.instructions.push(MirInstruction::Copy {
        dst: ValueId::new(5),
        src: ValueId::new(1),
    });
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut merge_block = BasicBlock::new(BasicBlockId::new(3));
    merge_block.instructions.push(MirInstruction::Phi {
        dst: ValueId::new(6),
        inputs: vec![
            (BasicBlockId::new(1), ValueId::new(4)),
            (BasicBlockId::new(2), ValueId::new(5)),
        ],
        type_hint: Some(MirType::String),
    });
    merge_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });

    callee.blocks.insert(BasicBlockId::new(1), then_block);
    callee.blocks.insert(BasicBlockId::new(2), else_block);
    callee.blocks.insert(BasicBlockId::new(3), merge_block);
    module.add_function(caller);
    module.add_function(callee);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_symbol"], "Helper.normalize/2");
    assert_eq!(route["target_return_type"], "str");
    assert_eq!(route["target_shape"], "generic_pure_string_body");
    assert_eq!(route["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(route["tier"], "DirectAbi");
    assert_eq!(route["emit_kind"], "direct_function_call");
    assert_eq!(route["proof"], "typed_global_call_generic_pure_string");
    assert_eq!(route["return_shape"], "string_handle");
    assert_eq!(route["value_demand"], "runtime_i64_or_handle");
    assert_eq!(route["reason"], serde_json::Value::Null);

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["source"], "global_call_routes");
    assert_eq!(plan["source_route_id"], "global.user_call");
    assert_eq!(plan["core_op"], "UserGlobalCall");
    assert_eq!(plan["target_symbol"], "Helper.normalize/2");
    assert_eq!(plan["target_return_type"], "str");
    assert_eq!(plan["target_shape"], "generic_pure_string_body");
    assert_eq!(plan["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(plan["tier"], "DirectAbi");
    assert_eq!(plan["emit_kind"], "direct_function_call");
    assert_eq!(plan["symbol"], "Helper.normalize/2");
    assert_eq!(plan["proof"], "typed_global_call_generic_pure_string");
    assert_eq!(plan["route_proof"], "typed_global_call_generic_pure_string");
    assert_eq!(plan["return_shape"], "string_handle");
    assert_eq!(plan["value_demand"], "runtime_i64_or_handle");
    assert_eq!(plan["reason"], serde_json::Value::Null);
}
