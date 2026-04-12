use super::*;

#[test]
fn test_dce_prunes_overwritten_local_field_set_in_same_block() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

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
        bb0.instructions.push(MirInstruction::Const {
            dst: v_new,
            value: ConstValue::Integer(2),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::FieldSet {
            base: v_box,
            field: "child".to_string(),
            value: v_old,
            declared_type: Some(MirType::Integer),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::FieldSet {
            base: v_box,
            field: "child".to_string(),
            value: v_new,
            declared_type: Some(MirType::Integer),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::FieldGet {
            dst: v_read,
            base: v_box,
            field: "child".to_string(),
            declared_type: Some(MirType::Integer),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return {
            value: Some(v_read),
        });
    }

    module.add_function(func);

    eliminate_dead_code(&mut module);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    let field_sets = bb0
        .instructions
        .iter()
        .filter(|inst| matches!(inst, MirInstruction::FieldSet { .. }))
        .count();
    assert_eq!(field_sets, 1);
    assert!(bb0.instructions.iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::Const { dst, .. } if *dst == v_old
        )
    }));
    assert!(bb0.instructions.iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::FieldSet { value, .. } if *value == v_new
        )
    }));
    assert!(bb0.instructions.iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::FieldGet { dst, base, field, .. }
                if *dst == v_read && *base == v_box && field == "child"
        )
    }));
}

#[test]
fn test_dce_keeps_local_field_set_when_same_field_is_read_before_overwrite() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

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
        bb0.instructions.push(MirInstruction::Const {
            dst: v_new,
            value: ConstValue::Integer(2),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::FieldSet {
            base: v_box,
            field: "child".to_string(),
            value: v_old,
            declared_type: Some(MirType::Integer),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::FieldGet {
            dst: v_read,
            base: v_box,
            field: "child".to_string(),
            declared_type: Some(MirType::Integer),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::FieldSet {
            base: v_box,
            field: "child".to_string(),
            value: v_new,
            declared_type: Some(MirType::Integer),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return {
            value: Some(v_read),
        });
    }

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 0);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    let field_sets = bb0
        .instructions
        .iter()
        .filter(|inst| matches!(inst, MirInstruction::FieldSet { .. }))
        .count();
    assert_eq!(field_sets, 2);
}

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

#[test]
fn test_dce_prunes_overwritten_local_field_set_into_merge_successor() {
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
    func.blocks
        .insert(BasicBlockId(2), BasicBlock::new(BasicBlockId(2)));
    func.blocks
        .insert(BasicBlockId(3), BasicBlock::new(BasicBlockId(3)));

    let v_box = ValueId(1);
    let v_cond = ValueId(2);
    let v_left = ValueId(3);
    let v_right = ValueId(4);
    let v_new = ValueId(5);
    let v_read = ValueId(6);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Const {
            dst: v_cond,
            value: ConstValue::Bool(true),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Branch {
            condition: v_cond,
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        bb0.successors.insert(BasicBlockId(1));
        bb0.successors.insert(BasicBlockId(2));
    }

    {
        let bb1 = func.blocks.get_mut(&BasicBlockId(1)).unwrap();
        bb1.predecessors.insert(BasicBlockId(0));
        bb1.instructions.push(MirInstruction::Const {
            dst: v_left,
            value: ConstValue::Integer(1),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.instructions.push(MirInstruction::FieldSet {
            base: v_box,
            field: "child".to_string(),
            value: v_left,
            declared_type: Some(MirType::Integer),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(3),
            edge_args: None,
        });
        bb1.successors.insert(BasicBlockId(3));
    }

    {
        let bb2 = func.blocks.get_mut(&BasicBlockId(2)).unwrap();
        bb2.predecessors.insert(BasicBlockId(0));
        bb2.instructions.push(MirInstruction::Const {
            dst: v_right,
            value: ConstValue::Integer(2),
        });
        bb2.instruction_spans.push(Span::unknown());
        bb2.instructions.push(MirInstruction::FieldSet {
            base: v_box,
            field: "child".to_string(),
            value: v_right,
            declared_type: Some(MirType::Integer),
        });
        bb2.instruction_spans.push(Span::unknown());
        bb2.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(3),
            edge_args: None,
        });
        bb2.successors.insert(BasicBlockId(3));
    }

    {
        let bb3 = func.blocks.get_mut(&BasicBlockId(3)).unwrap();
        bb3.predecessors.insert(BasicBlockId(1));
        bb3.predecessors.insert(BasicBlockId(2));
        bb3.instructions.push(MirInstruction::Const {
            dst: v_new,
            value: ConstValue::Integer(3),
        });
        bb3.instruction_spans.push(Span::unknown());
        bb3.instructions.push(MirInstruction::FieldSet {
            base: v_box,
            field: "child".to_string(),
            value: v_new,
            declared_type: Some(MirType::Integer),
        });
        bb3.instruction_spans.push(Span::unknown());
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
    let bb1 = func.blocks.get(&BasicBlockId(1)).unwrap();
    let bb2 = func.blocks.get(&BasicBlockId(2)).unwrap();
    assert!(!bb1
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::FieldSet { .. })));
    assert!(!bb2
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::FieldSet { .. })));
    let bb3 = func.blocks.get(&BasicBlockId(3)).unwrap();
    assert!(bb3.instructions.iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::FieldSet { value, .. } if *value == v_new
        )
    }));
}

#[test]
fn test_dce_keeps_merge_predecessor_field_set_when_merge_reads_before_overwrite() {
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
    func.blocks
        .insert(BasicBlockId(2), BasicBlock::new(BasicBlockId(2)));
    func.blocks
        .insert(BasicBlockId(3), BasicBlock::new(BasicBlockId(3)));

    let v_box = ValueId(1);
    let v_cond = ValueId(2);
    let v_left = ValueId(3);
    let v_right = ValueId(4);
    let v_read = ValueId(5);
    let v_new = ValueId(6);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Const {
            dst: v_cond,
            value: ConstValue::Bool(true),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Branch {
            condition: v_cond,
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        bb0.successors.insert(BasicBlockId(1));
        bb0.successors.insert(BasicBlockId(2));
    }

    {
        let bb1 = func.blocks.get_mut(&BasicBlockId(1)).unwrap();
        bb1.predecessors.insert(BasicBlockId(0));
        bb1.instructions.push(MirInstruction::Const {
            dst: v_left,
            value: ConstValue::Integer(1),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.instructions.push(MirInstruction::FieldSet {
            base: v_box,
            field: "child".to_string(),
            value: v_left,
            declared_type: Some(MirType::Integer),
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(3),
            edge_args: None,
        });
        bb1.successors.insert(BasicBlockId(3));
    }

    {
        let bb2 = func.blocks.get_mut(&BasicBlockId(2)).unwrap();
        bb2.predecessors.insert(BasicBlockId(0));
        bb2.instructions.push(MirInstruction::Const {
            dst: v_right,
            value: ConstValue::Integer(2),
        });
        bb2.instruction_spans.push(Span::unknown());
        bb2.instructions.push(MirInstruction::FieldSet {
            base: v_box,
            field: "child".to_string(),
            value: v_right,
            declared_type: Some(MirType::Integer),
        });
        bb2.instruction_spans.push(Span::unknown());
        bb2.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(3),
            edge_args: None,
        });
        bb2.successors.insert(BasicBlockId(3));
    }

    {
        let bb3 = func.blocks.get_mut(&BasicBlockId(3)).unwrap();
        bb3.predecessors.insert(BasicBlockId(1));
        bb3.predecessors.insert(BasicBlockId(2));
        bb3.instructions.push(MirInstruction::FieldGet {
            dst: v_read,
            base: v_box,
            field: "child".to_string(),
            declared_type: Some(MirType::Integer),
        });
        bb3.instruction_spans.push(Span::unknown());
        bb3.instructions.push(MirInstruction::Const {
            dst: v_new,
            value: ConstValue::Integer(3),
        });
        bb3.instruction_spans.push(Span::unknown());
        bb3.instructions.push(MirInstruction::FieldSet {
            base: v_box,
            field: "child".to_string(),
            value: v_new,
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
    let bb1 = func.blocks.get(&BasicBlockId(1)).unwrap();
    let bb2 = func.blocks.get(&BasicBlockId(2)).unwrap();
    assert!(bb1
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::FieldSet { .. })));
    assert!(bb2
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::FieldSet { .. })));
}
