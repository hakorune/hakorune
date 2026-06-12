use super::*;

#[test]
fn refresh_module_global_call_routes_accepts_string_or_void_phi_guard_body() {
    let mut module = MirModule::new("global_call_string_or_void_phi_guard_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.message_or_null/0",
        Some(ValueId::new(20)),
        vec![],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.message_or_null/0".to_string(),
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
            value: ConstValue::Void,
        },
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Bool(true),
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
    text_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::String("message".to_string()),
    });
    text_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut null_carry_block = BasicBlock::new(BasicBlockId::new(2));
    null_carry_block.instructions.push(MirInstruction::Phi {
        dst: ValueId::new(4),
        inputs: vec![(BasicBlockId::new(0), ValueId::new(1))],
        type_hint: None,
    });
    null_carry_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut join_block = BasicBlock::new(BasicBlockId::new(3));
    join_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(5),
            inputs: vec![
                (BasicBlockId::new(1), ValueId::new(3)),
                (BasicBlockId::new(2), ValueId::new(4)),
            ],
            type_hint: None,
        },
        MirInstruction::Compare {
            dst: ValueId::new(6),
            op: CompareOp::Ne,
            lhs: ValueId::new(5),
            rhs: ValueId::new(1),
        },
    ]);
    join_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(6),
        then_bb: BasicBlockId::new(4),
        else_bb: BasicBlockId::new(5),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut return_text_block = BasicBlock::new(BasicBlockId::new(4));
    return_text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    let mut return_null_block = BasicBlock::new(BasicBlockId::new(5));
    return_null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });

    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), null_carry_block);
    callee.blocks.insert(BasicBlockId::new(3), join_block);
    callee
        .blocks
        .insert(BasicBlockId::new(4), return_text_block);
    callee
        .blocks
        .insert(BasicBlockId::new(5), return_null_block);
    callee
        .metadata
        .value_types
        .insert(ValueId::new(5), MirType::Integer);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.message_or_null/0".to_string(), callee);

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
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}

#[test]
fn refresh_module_global_call_routes_accepts_string_or_void_child_forward_phi_body() {
    let mut module = MirModule::new("global_call_string_or_void_child_forward_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.forward/0", Some(ValueId::new(20)), vec![]);

    let mut child = MirFunction::new(
        FunctionSignature {
            name: "Helper.maybe_text/0".to_string(),
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
    let mut child_null_block = BasicBlock::new(BasicBlockId::new(2));
    child_null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    child_null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    child.blocks.insert(BasicBlockId::new(1), child_text_block);
    child.blocks.insert(BasicBlockId::new(2), child_null_block);

    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.forward/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let wrapper_entry = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_entry.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.maybe_text/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    wrapper_entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });
    let mut guard_block = BasicBlock::new(BasicBlockId::new(1));
    guard_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(2),
            inputs: vec![(BasicBlockId::new(0), ValueId::new(1))],
            type_hint: None,
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
    guard_block.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(4),
        then_bb: BasicBlockId::new(2),
        else_bb: BasicBlockId::new(3),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut return_text_block = BasicBlock::new(BasicBlockId::new(2));
    return_text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut return_null_block = BasicBlock::new(BasicBlockId::new(3));
    return_null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    wrapper.blocks.insert(BasicBlockId::new(1), guard_block);
    wrapper
        .blocks
        .insert(BasicBlockId::new(2), return_text_block);
    wrapper
        .blocks
        .insert(BasicBlockId::new(3), return_null_block);
    wrapper
        .metadata
        .value_types
        .insert(ValueId::new(1), MirType::Void);
    wrapper
        .metadata
        .value_types
        .insert(ValueId::new(2), MirType::Void);

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.maybe_text/0".to_string(), child);
    module
        .functions
        .insert("Helper.forward/0".to_string(), wrapper);

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
        None,
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}
