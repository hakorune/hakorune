use super::*;

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
fn build_mir_json_root_emits_void_sentinel_return_child_blocker() {
    let mut module =
        crate::mir::MirModule::new("json_global_call_void_return_child_blocker_test".to_string());
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
    module.add_function(caller);
    module.add_function(callee);
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
fn build_mir_json_root_emits_transitive_void_sentinel_return_child_blocker() {
    let mut module =
        crate::mir::MirModule::new("json_global_call_void_transitive_blocker_test".to_string());
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
    text_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.wrapper/0".to_string())),
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

    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.wrapper/0".to_string(),
            params: vec![],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let wrapper_block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.map/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    wrapper_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    let map = MirFunction::new(
        FunctionSignature {
            name: "Helper.map/0".to_string(),
            params: vec![],
            return_type: MirType::Box("MapBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );

    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.add_function(caller);
    module.add_function(callee);
    module.add_function(wrapper);
    module.add_function(map);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(
        route["target_shape_reason"],
        "generic_string_global_target_shape_unknown"
    );
    assert_eq!(route["target_shape_blocker_symbol"], "Helper.map/0");
    assert_eq!(
        route["target_shape_blocker_reason"],
        "generic_string_return_object_abi_not_handle_compatible"
    );

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(
        plan["target_shape_reason"],
        "generic_string_global_target_shape_unknown"
    );
    assert_eq!(plan["target_shape_blocker_symbol"], "Helper.map/0");
    assert_eq!(
        plan["target_shape_blocker_reason"],
        "generic_string_return_object_abi_not_handle_compatible"
    );
}
