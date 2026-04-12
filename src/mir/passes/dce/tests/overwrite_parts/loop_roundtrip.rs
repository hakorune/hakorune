use super::*;

#[test]
fn test_dce_prunes_overwritten_local_field_set_after_one_loop_header_roundtrip() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));
    func.add_block(BasicBlock::new(BasicBlockId(1)));
    func.add_block(BasicBlock::new(BasicBlockId(2)));
    func.add_block(BasicBlock::new(BasicBlockId(3)));

    let v_box = ValueId(1);
    let v_phi = ValueId(2);
    let v_cond = ValueId(3);
    let v_header = ValueId(4);
    let v_body = ValueId(5);
    let v_back = ValueId(6);
    let v_read = ValueId(7);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
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
        bb1.predecessors.insert(BasicBlockId(2));
        bb1.instructions.push(MirInstruction::Phi {
            dst: v_phi,
            inputs: vec![(BasicBlockId(0), v_box), (BasicBlockId(2), v_back)],
            type_hint: Some(MirType::Box("Point".to_string())),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.instructions.push(MirInstruction::Const {
            dst: v_cond,
            value: ConstValue::Bool(true),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.instructions.push(MirInstruction::Const {
            dst: v_header,
            value: ConstValue::Integer(9),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.instructions.push(MirInstruction::FieldSet {
            base: v_phi,
            field: "child".to_string(),
            value: v_header,
            declared_type: Some(MirType::Integer),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.set_terminator(MirInstruction::Branch {
            condition: v_cond,
            then_bb: BasicBlockId(2),
            else_bb: BasicBlockId(3),
            then_edge_args: None,
            else_edge_args: None,
        });
        bb1.successors.insert(BasicBlockId(2));
        bb1.successors.insert(BasicBlockId(3));
    }

    {
        let bb2 = func.blocks.get_mut(&BasicBlockId(2)).unwrap();
        bb2.predecessors.insert(BasicBlockId(1));
        bb2.instructions.push(MirInstruction::Const {
            dst: v_body,
            value: ConstValue::Integer(7),
        });
        bb2.instruction_spans.push(Span::unknown());
        bb2.instructions.push(MirInstruction::FieldSet {
            base: v_phi,
            field: "child".to_string(),
            value: v_body,
            declared_type: Some(MirType::Integer),
        });
        bb2.instruction_spans.push(Span::unknown());
        bb2.instructions.push(MirInstruction::Copy {
            dst: v_back,
            src: v_phi,
        });
        bb2.instruction_spans.push(Span::unknown());
        bb2.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(1),
            edge_args: None,
        });
        bb2.successors.insert(BasicBlockId(1));
    }

    {
        let bb3 = func.blocks.get_mut(&BasicBlockId(3)).unwrap();
        bb3.predecessors.insert(BasicBlockId(1));
        bb3.instructions.push(MirInstruction::FieldGet {
            dst: v_read,
            base: v_box,
            field: "child".to_string(),
            declared_type: Some(MirType::Integer),
        });
        bb3.instruction_spans.push(Span::unknown());
        bb3.set_terminator(MirInstruction::Return {
            value: Some(v_read),
        });
    }

    module.add_function(func);

    eliminate_dead_code(&mut module);

    let func = module.get_function("test/0").unwrap();
    let bb2 = func.blocks.get(&BasicBlockId(2)).unwrap();
    assert!(!bb2
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::FieldSet { .. })));

    let bb1 = func.blocks.get(&BasicBlockId(1)).unwrap();
    assert!(bb1.instructions.iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::FieldSet { value, .. } if *value == v_header
        )
    }));
    let bb3 = func.blocks.get(&BasicBlockId(3)).unwrap();
    assert!(bb3.instructions.iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::FieldGet { dst, .. } if *dst == v_read
        )
    }));
}

#[test]
fn test_dce_keeps_loop_roundtrip_field_set_when_header_reads_before_overwrite() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));
    func.add_block(BasicBlock::new(BasicBlockId(1)));
    func.add_block(BasicBlock::new(BasicBlockId(2)));
    func.add_block(BasicBlock::new(BasicBlockId(3)));

    let v_box = ValueId(1);
    let v_phi = ValueId(2);
    let v_cond = ValueId(3);
    let v_seen = ValueId(4);
    let v_header = ValueId(5);
    let v_body = ValueId(6);
    let v_back = ValueId(7);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
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
        bb1.predecessors.insert(BasicBlockId(2));
        bb1.instructions.push(MirInstruction::Phi {
            dst: v_phi,
            inputs: vec![(BasicBlockId(0), v_box), (BasicBlockId(2), v_back)],
            type_hint: Some(MirType::Box("Point".to_string())),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.instructions.push(MirInstruction::Const {
            dst: v_cond,
            value: ConstValue::Bool(true),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.instructions.push(MirInstruction::FieldGet {
            dst: v_seen,
            base: v_phi,
            field: "child".to_string(),
            declared_type: Some(MirType::Integer),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.instructions.push(MirInstruction::Const {
            dst: v_header,
            value: ConstValue::Integer(9),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.instructions.push(MirInstruction::FieldSet {
            base: v_phi,
            field: "child".to_string(),
            value: v_header,
            declared_type: Some(MirType::Integer),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.set_terminator(MirInstruction::Branch {
            condition: v_cond,
            then_bb: BasicBlockId(2),
            else_bb: BasicBlockId(3),
            then_edge_args: None,
            else_edge_args: None,
        });
        bb1.successors.insert(BasicBlockId(2));
        bb1.successors.insert(BasicBlockId(3));
    }

    {
        let bb2 = func.blocks.get_mut(&BasicBlockId(2)).unwrap();
        bb2.predecessors.insert(BasicBlockId(1));
        bb2.instructions.push(MirInstruction::Const {
            dst: v_body,
            value: ConstValue::Integer(7),
        });
        bb2.instruction_spans.push(Span::unknown());
        bb2.instructions.push(MirInstruction::FieldSet {
            base: v_phi,
            field: "child".to_string(),
            value: v_body,
            declared_type: Some(MirType::Integer),
        });
        bb2.instruction_spans.push(Span::unknown());
        bb2.instructions.push(MirInstruction::Copy {
            dst: v_back,
            src: v_phi,
        });
        bb2.instruction_spans.push(Span::unknown());
        bb2.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(1),
            edge_args: None,
        });
        bb2.successors.insert(BasicBlockId(1));
    }

    {
        let bb3 = func.blocks.get_mut(&BasicBlockId(3)).unwrap();
        bb3.predecessors.insert(BasicBlockId(1));
        bb3.set_terminator(MirInstruction::Return {
            value: Some(v_seen),
        });
    }

    module.add_function(func);

    eliminate_dead_code(&mut module);

    let func = module.get_function("test/0").unwrap();
    let bb2 = func.blocks.get(&BasicBlockId(2)).unwrap();
    assert!(bb2
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::FieldSet { .. })));
    let bb1 = func.blocks.get(&BasicBlockId(1)).unwrap();
    assert!(bb1
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::FieldGet { dst, .. } if *dst == v_seen)));
}
