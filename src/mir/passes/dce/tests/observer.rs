use super::*;

#[test]
fn test_dce_keeps_debug_observer_and_operand_live() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_observed = ValueId(1);
    let v_dead = ValueId(2);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::Const {
            dst: v_observed,
            value: ConstValue::Integer(123),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Const {
            dst: v_dead,
            value: ConstValue::Integer(999),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Debug {
            value: v_observed,
            message: "observe".to_string(),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return { value: None });
    }

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 1);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_observed)));
    assert!(bb0.instructions.iter().any(
        |inst| matches!(inst, MirInstruction::Debug { value, message } if *value == v_observed && message == "observe")
    ));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_dead)));
}
