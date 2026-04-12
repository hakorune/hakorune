use super::*;

#[test]
fn test_dce_prunes_dead_field_get_from_non_escaping_local_box() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_box = ValueId(1);
    let v_copy = ValueId(2);
    let v_field = ValueId(3);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Copy {
            dst: v_copy,
            src: v_box,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::FieldGet {
            dst: v_field,
            base: v_copy,
            field: "child".to_string(),
            declared_type: Some(MirType::Integer),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return { value: None });
    }

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 2);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::NewBox { dst, .. } if *dst == v_box)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Copy { dst, .. } if *dst == v_copy)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::FieldGet { dst, .. } if *dst == v_field)));
}

#[test]
fn test_dce_prunes_dead_field_set_on_non_escaping_local_box() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_box = ValueId(1);
    let v_copy = ValueId(2);
    let v_value = ValueId(3);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Copy {
            dst: v_copy,
            src: v_box,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Const {
            dst: v_value,
            value: ConstValue::Integer(77),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::FieldSet {
            base: v_copy,
            field: "child".to_string(),
            value: v_value,
            declared_type: Some(MirType::Integer),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return {
            value: Some(v_value),
        });
    }

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 2);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::NewBox { dst, .. } if *dst == v_box)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Copy { dst, .. } if *dst == v_copy)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::FieldSet { .. })));
}

#[test]
fn test_dce_prunes_dead_field_get_through_same_root_phi() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));
    func.add_block(BasicBlock::new(BasicBlockId(1)));
    func.add_block(BasicBlock::new(BasicBlockId(2)));
    func.add_block(BasicBlock::new(BasicBlockId(3)));

    let v_box = ValueId(1);
    let v_cond = ValueId(2);
    let v_left = ValueId(3);
    let v_right = ValueId(4);
    let v_phi = ValueId(5);
    let v_field = ValueId(6);

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
    }
    {
        let bb1 = func.blocks.get_mut(&BasicBlockId(1)).unwrap();
        bb1.instructions.push(MirInstruction::Copy {
            dst: v_left,
            src: v_box,
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(3),
            edge_args: None,
        });
    }
    {
        let bb2 = func.blocks.get_mut(&BasicBlockId(2)).unwrap();
        bb2.instructions.push(MirInstruction::Copy {
            dst: v_right,
            src: v_box,
        });
        bb2.instruction_spans.push(Span::unknown());
        bb2.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(3),
            edge_args: None,
        });
    }
    {
        let bb3 = func.blocks.get_mut(&BasicBlockId(3)).unwrap();
        bb3.instructions.push(MirInstruction::Phi {
            dst: v_phi,
            inputs: vec![(BasicBlockId(1), v_left), (BasicBlockId(2), v_right)],
            type_hint: Some(MirType::Box("Point".to_string())),
        });
        bb3.instruction_spans.push(Span::unknown());
        bb3.instructions.push(MirInstruction::FieldGet {
            dst: v_field,
            base: v_phi,
            field: "child".to_string(),
            declared_type: Some(MirType::Integer),
        });
        bb3.instruction_spans.push(Span::unknown());
        bb3.set_terminator(MirInstruction::Return { value: None });
    }

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 4);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    let bb1 = func.blocks.get(&BasicBlockId(1)).unwrap();
    let bb2 = func.blocks.get(&BasicBlockId(2)).unwrap();
    let bb3 = func.blocks.get(&BasicBlockId(3)).unwrap();

    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::NewBox { dst, .. } if *dst == v_box)));
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_cond)));
    assert!(bb1.instructions.is_empty());
    assert!(bb2.instructions.is_empty());
    assert!(!bb3
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Phi { dst, .. } if *dst == v_phi)));
    assert!(!bb3
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::FieldGet { dst, .. } if *dst == v_field)));
}

#[test]
fn test_dce_prunes_dead_field_set_through_same_root_phi() {
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
    let v_cond = ValueId(2);
    let v_left = ValueId(3);
    let v_right = ValueId(4);
    let v_phi = ValueId(5);
    let v_keep = ValueId(6);

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
        bb0.instructions.push(MirInstruction::Const {
            dst: v_keep,
            value: ConstValue::Integer(7),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Branch {
            condition: v_cond,
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(2),
            then_edge_args: None,
            else_edge_args: None,
        });
    }
    {
        let bb1 = func.blocks.get_mut(&BasicBlockId(1)).unwrap();
        bb1.instructions.push(MirInstruction::Copy {
            dst: v_left,
            src: v_box,
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(3),
            edge_args: None,
        });
    }
    {
        let bb2 = func.blocks.get_mut(&BasicBlockId(2)).unwrap();
        bb2.instructions.push(MirInstruction::Copy {
            dst: v_right,
            src: v_box,
        });
        bb2.instruction_spans.push(Span::unknown());
        bb2.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(3),
            edge_args: None,
        });
    }
    {
        let bb3 = func.blocks.get_mut(&BasicBlockId(3)).unwrap();
        bb3.instructions.push(MirInstruction::Phi {
            dst: v_phi,
            inputs: vec![(BasicBlockId(1), v_left), (BasicBlockId(2), v_right)],
            type_hint: Some(MirType::Box("Point".to_string())),
        });
        bb3.instruction_spans.push(Span::unknown());
        bb3.instructions.push(MirInstruction::FieldSet {
            base: v_phi,
            field: "child".to_string(),
            value: v_keep,
            declared_type: Some(MirType::Integer),
        });
        bb3.instruction_spans.push(Span::unknown());
        bb3.set_terminator(MirInstruction::Return {
            value: Some(v_keep),
        });
    }

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 4);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    let bb1 = func.blocks.get(&BasicBlockId(1)).unwrap();
    let bb2 = func.blocks.get(&BasicBlockId(2)).unwrap();
    let bb3 = func.blocks.get(&BasicBlockId(3)).unwrap();

    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_cond)));
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_keep)));
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::NewBox { dst, .. } if *dst == v_box)));
    assert!(bb1.instructions.is_empty());
    assert!(bb2.instructions.is_empty());
    assert!(!bb3
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Phi { dst, .. } if *dst == v_phi)));
    assert!(!bb3
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::FieldSet { .. })));
}

#[test]
fn test_dce_prunes_dead_loop_carried_same_root_field_get() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
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
    let v_phi = ValueId(3);
    let v_read = ValueId(4);
    let v_back = ValueId(5);

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
        bb2.instructions.push(MirInstruction::FieldGet {
            dst: v_read,
            base: v_phi,
            field: "child".to_string(),
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
        bb3.set_terminator(MirInstruction::Return { value: None });
    }

    module.add_function(func);

    eliminate_dead_code(&mut module);

    let func = module.get_function("test/0").unwrap();
    let bb2 = func.blocks.get(&BasicBlockId(2)).unwrap();
    assert!(!bb2.instructions.iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::FieldGet { dst, .. } if *dst == v_read
        )
    }));
}

#[test]
fn test_dce_prunes_dead_loop_carried_same_root_field_set() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
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
    let v_phi = ValueId(3);
    let v_value = ValueId(4);
    let v_back = ValueId(5);

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
            dst: v_value,
            value: ConstValue::Integer(1),
        });
        bb2.instruction_spans.push(Span::unknown());
        bb2.instructions.push(MirInstruction::FieldSet {
            base: v_phi,
            field: "child".to_string(),
            value: v_value,
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
        bb3.set_terminator(MirInstruction::Return { value: None });
    }

    module.add_function(func);

    eliminate_dead_code(&mut module);

    let func = module.get_function("test/0").unwrap();
    let bb2 = func.blocks.get(&BasicBlockId(2)).unwrap();
    assert!(!bb2
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::FieldSet { .. })));
}
