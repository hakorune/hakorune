use super::*;

#[test]
fn build_mir_json_root_emits_string_or_void_sentinel_direct_route() {
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
    assert_eq!(route["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(
        route["proof"],
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route["tier"], "DirectAbi");
    assert_eq!(route["return_shape"], "string_handle_or_null");
    assert_eq!(route["result_origin"], "string");
    assert_eq!(route["definition_owner"], "module_generic");
    assert_eq!(
        route["emit_trace_consumer"],
        "mir_call_global_module_generic_emit"
    );

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["target_exists"], true);
    assert_eq!(plan["target_return_type"], "void");
    assert_eq!(plan["target_shape"], serde_json::Value::Null);
    assert_eq!(plan["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(
        plan["route_proof"],
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(plan["return_shape"], "string_handle_or_null");
    assert_eq!(plan["result_origin"], "string");
    assert_eq!(plan["definition_owner"], "module_generic");
    assert_eq!(
        plan["emit_trace_consumer"],
        "mir_call_global_module_generic_emit"
    );
}

#[test]
fn build_mir_json_root_emits_substring_string_or_void_sentinel_direct_route() {
    let mut module =
        crate::mir::MirModule::new("json_global_call_substring_void_sentinel_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.slice_or_null/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        });
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
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
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
    module.add_function(caller);
    module.add_function(callee);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_shape"], serde_json::Value::Null);
    assert_eq!(route["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(
        route["proof"],
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route["return_shape"], "string_handle_or_null");

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["target_shape"], serde_json::Value::Null);
    assert_eq!(
        plan["route_proof"],
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(plan["return_shape"], "string_handle_or_null");
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
fn build_mir_json_root_emits_object_return_abi_shape_reason() {
    let mut module =
        crate::mir::MirModule::new("json_global_call_object_return_reason_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.map/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
    let callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.map/0".to_string(),
            params: vec![],
            return_type: MirType::Box("MapBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    module.add_function(caller);
    module.add_function(callee);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(
        route["target_shape_reason"],
        "generic_string_return_object_abi_not_handle_compatible"
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
    assert_eq!(
        plan["target_shape_reason"],
        "generic_string_return_object_abi_not_handle_compatible"
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
            method: "debugPreview".to_string(),
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
