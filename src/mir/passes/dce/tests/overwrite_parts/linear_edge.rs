use super::*;

#[test]
fn test_dce_prunes_overwritten_local_field_set_across_linear_edge() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));
    func.blocks
        .insert(BasicBlockId(1), BasicBlock::new(BasicBlockId(1)));

    let v_box = ValueId(1);
    let v_old = ValueId(2);
    let v_new = ValueId(3);
    let v_read = ValueId(4);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Const {
            dst: v_old,
            value: ConstValue::Integer(1),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::FieldSet {
            base: v_box,
            field: "child".to_string(),
            value: v_old,
            declared_type: Some(MirType::Integer),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(1),
            edge_args: None,
        });
        bb0.successors.insert(BasicBlockId(1));
    }

    {
        let bb1 = func.blocks.get_mut(&BasicBlockId(1)).unwrap();
        bb1.predecessors.insert(BasicBlockId(0));
        bb1.instructions.push(MirInstruction::Const {
            dst: v_new,
            value: ConstValue::Integer(2),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.instructions.push(MirInstruction::FieldSet {
            base: v_box,
            field: "child".to_string(),
            value: v_new,
            declared_type: Some(MirType::Integer),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.instructions.push(MirInstruction::FieldGet {
            dst: v_read,
            base: v_box,
            field: "child".to_string(),
            declared_type: Some(MirType::Integer),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.set_terminator(MirInstruction::Return {
            value: Some(v_read),
        });
    }

    module.add_function(func);

    eliminate_dead_code(&mut module);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::FieldSet { .. })));
    let bb1 = func.blocks.get(&BasicBlockId(1)).unwrap();
    assert!(bb1.instructions.iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::FieldSet { value, .. } if *value == v_new
        )
    }));
    assert!(bb1.instructions.iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::FieldGet { dst, .. } if *dst == v_read
        )
    }));
}

#[test]
fn test_dce_keeps_cross_block_local_field_set_when_successor_reads_before_overwrite() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));
    func.blocks
        .insert(BasicBlockId(1), BasicBlock::new(BasicBlockId(1)));

    let v_box = ValueId(1);
    let v_old = ValueId(2);
    let v_new = ValueId(3);
    let v_read = ValueId(4);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Const {
            dst: v_old,
            value: ConstValue::Integer(1),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::FieldSet {
            base: v_box,
            field: "child".to_string(),
            value: v_old,
            declared_type: Some(MirType::Integer),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(1),
            edge_args: None,
        });
        bb0.successors.insert(BasicBlockId(1));
    }

    {
        let bb1 = func.blocks.get_mut(&BasicBlockId(1)).unwrap();
        bb1.predecessors.insert(BasicBlockId(0));
        bb1.instructions.push(MirInstruction::FieldGet {
            dst: v_read,
            base: v_box,
            field: "child".to_string(),
            declared_type: Some(MirType::Integer),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.instructions.push(MirInstruction::Const {
            dst: v_new,
            value: ConstValue::Integer(2),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.instructions.push(MirInstruction::FieldSet {
            base: v_box,
            field: "child".to_string(),
            value: v_new,
            declared_type: Some(MirType::Integer),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.set_terminator(MirInstruction::Return {
            value: Some(v_read),
        });
    }

    module.add_function(func);

    eliminate_dead_code(&mut module);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::FieldSet { .. })));
}
