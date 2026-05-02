use super::*;

#[test]
fn threads_branch_through_empty_jump_trampoline() {
    let mut module = MirModule::new("simplify_cfg_jump_thread".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Bool],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));

    {
        let entry = function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block");
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(4),
            then_edge_args: None,
            else_edge_args: None,
        });
    }

    let mut trampoline = BasicBlock::new(BasicBlockId(1));
    trampoline.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });
    function.add_block(trampoline);

    let mut middle = BasicBlock::new(BasicBlockId(2));
    middle.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });
    function.add_block(middle);

    let mut dispatcher = BasicBlock::new(BasicBlockId(4));
    dispatcher.instructions.push(MirInstruction::Debug {
        value: ValueId(0),
        message: "keep dispatcher non-threadable".to_string(),
    });
    dispatcher.instruction_spans.push(Span::unknown());
    dispatcher.set_terminator(MirInstruction::Branch {
        condition: ValueId(0),
        then_bb: BasicBlockId(5),
        else_bb: BasicBlockId(6),
        then_edge_args: None,
        else_edge_args: None,
    });
    function.add_block(dispatcher);

    let mut anchor_then = BasicBlock::new(BasicBlockId(5));
    anchor_then.instructions.push(MirInstruction::Debug {
        value: ValueId(0),
        message: "anchor then".to_string(),
    });
    anchor_then.instruction_spans.push(Span::unknown());
    anchor_then.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(2),
        edge_args: None,
    });
    function.add_block(anchor_then);

    let mut anchor_else = BasicBlock::new(BasicBlockId(6));
    anchor_else.instructions.push(MirInstruction::Debug {
        value: ValueId(0),
        message: "anchor else".to_string(),
    });
    anchor_else.instruction_spans.push(Span::unknown());
    anchor_else.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(2),
        edge_args: None,
    });
    function.add_block(anchor_else);

    let mut final_block = BasicBlock::new(BasicBlockId(3));
    final_block.set_terminator(MirInstruction::Return { value: None });
    function.add_block(final_block);

    function
        .metadata
        .value_types
        .insert(ValueId(0), MirType::Bool);
    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert!(simplified >= 1);

    let function = module.functions.get("main").expect("main function");
    let entry = function.blocks.get(&BasicBlockId(0)).expect("entry block");
    assert!(matches!(
        entry.terminator,
        Some(MirInstruction::Branch {
            then_bb,
            else_bb,
            then_edge_args: None,
            else_edge_args: None,
            ..
        }) if then_bb == BasicBlockId(3) && else_bb == BasicBlockId(4)
    ));
    assert!(function.blocks.contains_key(&BasicBlockId(1)));
    assert!(function.blocks.contains_key(&BasicBlockId(2)));
    assert!(function.blocks.contains_key(&BasicBlockId(4)));
}

#[test]
fn threads_branch_through_empty_jump_trampoline_and_rewrites_final_phi_predecessor() {
    let mut module = MirModule::new("simplify_cfg_jump_thread_phi_target".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Bool],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));

    {
        let entry = function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block");
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(11),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(2),
            then_edge_args: None,
            else_edge_args: None,
        });
    }

    let mut trampoline = BasicBlock::new(BasicBlockId(1));
    trampoline.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });
    function.add_block(trampoline);

    let mut else_block = BasicBlock::new(BasicBlockId(2));
    else_block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: ConstValue::Integer(22),
    });
    else_block.instruction_spans.push(Span::unknown());
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });
    function.add_block(else_block);

    let mut final_block = BasicBlock::new(BasicBlockId(3));
    final_block.instructions.push(MirInstruction::Phi {
        dst: ValueId(3),
        inputs: vec![(BasicBlockId(1), ValueId(1)), (BasicBlockId(2), ValueId(2))],
        type_hint: Some(MirType::Integer),
    });
    final_block.instruction_spans.push(Span::unknown());
    final_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(3)),
    });
    function.add_block(final_block);

    function
        .metadata
        .value_types
        .insert(ValueId(0), MirType::Bool);
    function
        .metadata
        .value_types
        .insert(ValueId(1), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(2), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(3), MirType::Integer);
    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert!(simplified >= 1);

    let function = module.functions.get("main").expect("main function");
    let entry = function.blocks.get(&BasicBlockId(0)).expect("entry block");
    assert!(matches!(
        entry.terminator,
        Some(MirInstruction::Branch {
            then_bb,
            else_bb,
            then_edge_args: None,
            else_edge_args: None,
            ..
        }) if then_bb == BasicBlockId(3) && else_bb == BasicBlockId(2)
    ));

    let final_block = function.blocks.get(&BasicBlockId(3)).expect("final block");
    let MirInstruction::Phi { inputs, .. } = &final_block.instructions[0] else {
        panic!("expected phi");
    };
    assert_eq!(
        inputs,
        &vec![(BasicBlockId(0), ValueId(1)), (BasicBlockId(2), ValueId(2))]
    );
}

#[test]
fn threads_branch_through_empty_jump_trampoline_and_drops_dead_edge_args() {
    let mut module = MirModule::new("simplify_cfg_jump_thread_edge_args".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Bool],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));

    {
        let entry = function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block");
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(11),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(3),
            then_edge_args: Some(EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(1)],
            }),
            else_edge_args: None,
        });
    }

    let mut trampoline = BasicBlock::new(BasicBlockId(1));
    trampoline.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });
    function.add_block(trampoline);

    let mut final_block = BasicBlock::new(BasicBlockId(3));
    final_block.set_terminator(MirInstruction::Return { value: None });
    function.add_block(final_block);

    function
        .metadata
        .value_types
        .insert(ValueId(0), MirType::Bool);
    function
        .metadata
        .value_types
        .insert(ValueId(1), MirType::Integer);
    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert!(simplified >= 1);

    let function = module.functions.get("main").expect("main function");
    let entry = function.blocks.get(&BasicBlockId(0)).expect("entry block");
    assert!(matches!(
        entry.terminator,
        Some(MirInstruction::Jump {
            target,
            edge_args: None
        }) if target == BasicBlockId(3)
    ));
}

#[test]
fn keeps_branch_trampoline_when_threaded_arm_edge_args_would_cross_final_phi() {
    let mut module = MirModule::new("simplify_cfg_jump_thread_edge_args_phi_guard".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Bool],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));

    {
        let entry = function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block");
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(11),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(2),
            then_edge_args: Some(EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(1)],
            }),
            else_edge_args: None,
        });
    }

    let mut trampoline = BasicBlock::new(BasicBlockId(1));
    trampoline.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });
    function.add_block(trampoline);

    let mut else_block = BasicBlock::new(BasicBlockId(2));
    else_block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: ConstValue::Integer(22),
    });
    else_block.instruction_spans.push(Span::unknown());
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });
    function.add_block(else_block);

    let mut final_block = BasicBlock::new(BasicBlockId(3));
    final_block.instructions.push(MirInstruction::Phi {
        dst: ValueId(3),
        inputs: vec![(BasicBlockId(1), ValueId(1)), (BasicBlockId(2), ValueId(2))],
        type_hint: Some(MirType::Integer),
    });
    final_block.instruction_spans.push(Span::unknown());
    final_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(3)),
    });
    function.add_block(final_block);

    function
        .metadata
        .value_types
        .insert(ValueId(0), MirType::Bool);
    function
        .metadata
        .value_types
        .insert(ValueId(1), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(2), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(3), MirType::Integer);
    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert_eq!(simplified, 0);

    let function = module.functions.get("main").expect("main function");
    let entry = function.blocks.get(&BasicBlockId(0)).expect("entry block");
    assert!(matches!(
        &entry.terminator,
        Some(MirInstruction::Branch {
            then_bb,
            else_bb,
            then_edge_args: Some(EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values
            }),
            else_edge_args: None,
            ..
        }) if *then_bb == BasicBlockId(1) && *else_bb == BasicBlockId(2) && values.as_slice() == [ValueId(1)]
    ));
}
