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
