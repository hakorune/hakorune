use super::*;

#[test]
fn test_dce_keeps_edge_args_values() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v1 = ValueId(1);
    let v2 = ValueId(2);
    let v_dead = ValueId(3);
    let bb1 = BasicBlockId(1);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::Const {
            dst: v1,
            value: ConstValue::Integer(123),
        });
        bb0.instruction_spans.push(Span::unknown());

        copy_emitter::emit_copy_into_detached_block(
            bb0,
            v2,
            v1,
            CopyEmitReason::TestDceEdgeArgCopy,
        )
        .unwrap();

        bb0.instructions.push(MirInstruction::Const {
            dst: v_dead,
            value: ConstValue::Integer(999),
        });
        bb0.instruction_spans.push(Span::unknown());

        bb0.set_jump_with_edge_args(
            bb1,
            Some(crate::mir::EdgeArgs {
                layout:
                    crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout::CarriersOnly,
                values: vec![v2],
            }),
        );
    }

    let mut exit_block = crate::mir::BasicBlock::new(bb1);
    exit_block.set_terminator(MirInstruction::Return { value: None });
    func.add_block(exit_block);

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 1);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();

    assert_eq!(bb0.instructions.len(), bb0.instruction_spans.len());
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Copy { dst, .. } if *dst == v2)));
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v1)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_dead)));
}

#[test]
fn test_dce_prunes_unreachable_pure_block() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_entry = ValueId(1);
    let v_dead_copy = ValueId(2);
    let reachable_exit = BasicBlockId(1);
    let unreachable_bb = BasicBlockId(2);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::Const {
            dst: v_entry,
            value: ConstValue::Integer(123),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_jump_with_edge_args(reachable_exit, None);
    }

    let mut bb1 = BasicBlock::new(reachable_exit);
    bb1.set_terminator(MirInstruction::Return { value: None });
    func.add_block(bb1);

    let mut dead_block = BasicBlock::new(unreachable_bb);
    copy_emitter::emit_copy_into_detached_block(
        &mut dead_block,
        v_dead_copy,
        v_entry,
        CopyEmitReason::TestDceEdgeArgCopy,
    )
    .unwrap();
    dead_block.set_terminator(MirInstruction::Return { value: None });
    func.add_block(dead_block);

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 2);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();

    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_entry)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Copy { dst, .. } if *dst == v_dead_copy)));
    assert!(!func.blocks.contains_key(&unreachable_bb));
}

#[test]
fn test_dce_prunes_unreachable_effectful_block() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_entry = ValueId(1);
    let v_dead_ptr = ValueId(2);
    let reachable_exit = BasicBlockId(1);
    let unreachable_bb = BasicBlockId(2);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::Const {
            dst: v_entry,
            value: ConstValue::Integer(123),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_jump_with_edge_args(reachable_exit, None);
    }

    let mut bb1 = BasicBlock::new(reachable_exit);
    bb1.set_terminator(MirInstruction::Return { value: None });
    func.add_block(bb1);

    let mut dead_block = BasicBlock::new(unreachable_bb);
    dead_block.instructions.push(MirInstruction::Const {
        dst: v_dead_ptr,
        value: ConstValue::Integer(999),
    });
    dead_block.instruction_spans.push(Span::unknown());
    dead_block.instructions.push(MirInstruction::Store {
        value: v_entry,
        ptr: v_dead_ptr,
    });
    dead_block.instruction_spans.push(Span::unknown());
    dead_block.set_terminator(MirInstruction::Return { value: None });
    func.add_block(dead_block);

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 2);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();

    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_entry)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_dead_ptr)));
    assert!(!func.blocks.contains_key(&unreachable_bb));
}

#[test]
fn test_dce_prunes_phi_inputs_from_removed_unreachable_predecessors() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let entry_value = ValueId(1);
    let dead_value = ValueId(2);
    let phi_dst = ValueId(3);
    let header_bb = BasicBlockId(1);
    let dead_backedge_bb = BasicBlockId(2);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::Const {
            dst: entry_value,
            value: ConstValue::Integer(1),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_jump_with_edge_args(header_bb, None);
    }

    let mut header = BasicBlock::new(header_bb);
    header.instructions.push(MirInstruction::Phi {
        dst: phi_dst,
        inputs: vec![(BasicBlockId(0), entry_value), (dead_backedge_bb, dead_value)],
        type_hint: Some(MirType::Integer),
    });
    header.instruction_spans.push(Span::unknown());
    header.set_terminator(MirInstruction::Return {
        value: Some(phi_dst),
    });
    func.add_block(header);

    let mut dead_backedge = BasicBlock::new(dead_backedge_bb);
    dead_backedge.instructions.push(MirInstruction::Const {
        dst: dead_value,
        value: ConstValue::Integer(9),
    });
    dead_backedge.instruction_spans.push(Span::unknown());
    dead_backedge.set_jump_with_edge_args(header_bb, None);
    func.add_block(dead_backedge);

    module.add_function(func);

    let _ = eliminate_dead_code(&mut module);

    let func = module.get_function("test/0").unwrap();
    assert!(!func.blocks.contains_key(&dead_backedge_bb));

    let header = func.blocks.get(&header_bb).unwrap();
    let MirInstruction::Phi { inputs, .. } = &header.instructions[0] else {
        panic!("expected phi");
    };
    assert_eq!(inputs, &vec![(BasicBlockId(0), entry_value)]);
}

#[test]
fn test_dce_prunes_redundant_keepalive_when_return_already_keeps_value_live() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v1 = ValueId(1);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::Const {
            dst: v1,
            value: ConstValue::Integer(123),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions
            .push(MirInstruction::KeepAlive { values: vec![v1] });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return { value: Some(v1) });
    }

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 1);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v1)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::KeepAlive { values } if values == &vec![v1])));
}

#[test]
fn test_dce_keeps_return_operand_value_live() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_ret = ValueId(1);
    let v_dead = ValueId(2);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::Const {
            dst: v_ret,
            value: ConstValue::Integer(123),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Const {
            dst: v_dead,
            value: ConstValue::Integer(999),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return { value: Some(v_ret) });
    }

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 1);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_ret)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_dead)));
}

#[test]
fn test_dce_keeps_branch_condition_operand_live() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_cond = ValueId(1);
    let v_dead = ValueId(2);
    let then_bb = BasicBlockId(1);
    let else_bb = BasicBlockId(2);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::Const {
            dst: v_cond,
            value: ConstValue::Bool(true),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Const {
            dst: v_dead,
            value: ConstValue::Integer(999),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Branch {
            condition: v_cond,
            then_bb,
            else_bb,
            then_edge_args: None,
            else_edge_args: None,
        });
    }

    let mut then_block = BasicBlock::new(then_bb);
    then_block.set_terminator(MirInstruction::Return { value: None });
    func.add_block(then_block);

    let mut else_block = BasicBlock::new(else_bb);
    else_block.set_terminator(MirInstruction::Return { value: None });
    func.add_block(else_block);

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 1);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_cond)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_dead)));
}

#[test]
fn test_dce_keeps_nonredundant_keepalive_when_it_is_the_only_live_reason() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v1 = ValueId(1);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::Const {
            dst: v1,
            value: ConstValue::Integer(123),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions
            .push(MirInstruction::KeepAlive { values: vec![v1] });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return { value: None });
    }

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 0);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v1)));
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::KeepAlive { values } if values == &vec![v1])));
}

#[test]
fn test_dce_prunes_safepoint_noop_when_it_has_no_other_effects() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v1 = ValueId(1);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::Const {
            dst: v1,
            value: ConstValue::Integer(123),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Safepoint);
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return { value: Some(v1) });
    }

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 1);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v1)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Safepoint)));
}

#[test]
fn test_dce_prunes_pure_no_dst_call_and_its_dead_operand_chain() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v1 = ValueId(1);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::Const {
            dst: v1,
            value: ConstValue::Integer(123),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Call {
            dst: None,
            func: ValueId(999),
            callee: Some(Callee::Global("noop".to_string())),
            args: vec![v1],
            effects: EffectMask::PURE,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return { value: None });
    }

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 2);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v1)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Call { dst: None, .. })));
}
