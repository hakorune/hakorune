use super::*;

fn run_dead_code_and_memory_effect(module: &mut MirModule) -> usize {
    let mut eliminated = eliminate_dead_code(module);
    eliminated += crate::mir::passes::memory_effect::apply(module).memory_effect_optimizations;
    eliminated += eliminate_dead_code(module);
    eliminated
}

#[test]
fn test_dce_prunes_dead_load_from_private_carrier_root() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_box = ValueId(1);
    let v_ptr = ValueId(2);
    let v_loaded = ValueId(3);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::RefNew {
            dst: v_ptr,
            box_val: v_box,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Load {
            dst: v_loaded,
            ptr: v_ptr,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return { value: None });
    }

    module.add_function(func);

    let eliminated = run_dead_code_and_memory_effect(&mut module);
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
        .any(|inst| matches!(inst, MirInstruction::RefNew { dst, .. } if *dst == v_ptr)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Load { dst, .. } if *dst == v_loaded)));
}

#[test]
fn test_dce_keeps_live_load_from_private_carrier_root() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_box = ValueId(1);
    let v_ptr = ValueId(2);
    let v_loaded = ValueId(3);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::RefNew {
            dst: v_ptr,
            box_val: v_box,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Load {
            dst: v_loaded,
            ptr: v_ptr,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return {
            value: Some(v_loaded),
        });
    }

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 0);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::RefNew { dst, .. } if *dst == v_ptr)));
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Load { dst, .. } if *dst == v_loaded)));
}

#[test]
fn test_dce_keeps_load_when_private_carrier_escapes_via_call() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_box = ValueId(1);
    let v_ptr = ValueId(2);
    let v_loaded = ValueId(3);
    let v_func = ValueId(4);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::RefNew {
            dst: v_ptr,
            box_val: v_box,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Const {
            dst: v_func,
            value: ConstValue::Integer(0),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Call {
            dst: None,
            func: v_func,
            callee: Some(Callee::Extern("observe_ptr".to_string())),
            args: vec![v_ptr],
            effects: EffectMask::READ,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Load {
            dst: v_loaded,
            ptr: v_ptr,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return { value: None });
    }

    module.add_function(func);

    let eliminated = run_dead_code_and_memory_effect(&mut module);
    assert_eq!(eliminated, 1);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::RefNew { dst, .. } if *dst == v_ptr)));
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Call { args, .. } if args == &vec![v_ptr])));
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Load { dst, .. } if *dst == v_loaded)));
}

#[test]
fn test_dce_keeps_dead_load_when_same_private_carrier_has_store() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_box = ValueId(1);
    let v_ptr = ValueId(2);
    let v_value = ValueId(3);
    let v_loaded = ValueId(4);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::RefNew {
            dst: v_ptr,
            box_val: v_box,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Const {
            dst: v_value,
            value: ConstValue::Integer(7),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Store {
            value: v_value,
            ptr: v_ptr,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Load {
            dst: v_loaded,
            ptr: v_ptr,
        });
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
        .any(|inst| matches!(inst, MirInstruction::Store { ptr, .. } if *ptr == v_ptr)));
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Load { dst, .. } if *dst == v_loaded)));
}

#[test]
fn test_dce_prunes_dead_load_through_copy_alias_private_carrier() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_box = ValueId(1);
    let v_ptr = ValueId(2);
    let v_ptr_copy = ValueId(3);
    let v_loaded = ValueId(4);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::RefNew {
            dst: v_ptr,
            box_val: v_box,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Copy {
            dst: v_ptr_copy,
            src: v_ptr,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Load {
            dst: v_loaded,
            ptr: v_ptr_copy,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return { value: None });
    }

    module.add_function(func);

    let eliminated = run_dead_code_and_memory_effect(&mut module);
    assert_eq!(eliminated, 3);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::NewBox { dst, .. } if *dst == v_box)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::RefNew { dst, .. } if *dst == v_ptr)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Copy { dst, .. } if *dst == v_ptr_copy)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Load { dst, .. } if *dst == v_loaded)));
}

#[test]
fn test_dce_prunes_overwritten_store_on_private_carrier_root() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_box = ValueId(1);
    let v_ptr = ValueId(2);
    let v_old = ValueId(3);
    let v_new = ValueId(4);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::RefNew {
            dst: v_ptr,
            box_val: v_box,
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
        bb0.instructions.push(MirInstruction::Store {
            value: v_old,
            ptr: v_ptr,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Store {
            value: v_new,
            ptr: v_ptr,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return { value: None });
    }

    module.add_function(func);

    let eliminated = run_dead_code_and_memory_effect(&mut module);
    assert_eq!(eliminated, 2);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    let stores = bb0
        .instructions
        .iter()
        .filter(|inst| matches!(inst, MirInstruction::Store { .. }))
        .count();
    assert_eq!(stores, 1);
    assert!(bb0.instructions.iter().any(|inst| {
        matches!(inst, MirInstruction::Store { value, ptr } if *value == v_new && *ptr == v_ptr)
    }));
}

#[test]
fn test_dce_keeps_overwritten_store_when_load_intervenes_on_private_carrier() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_box = ValueId(1);
    let v_ptr = ValueId(2);
    let v_old = ValueId(3);
    let v_new = ValueId(4);
    let v_seen = ValueId(5);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::RefNew {
            dst: v_ptr,
            box_val: v_box,
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
        bb0.instructions.push(MirInstruction::Store {
            value: v_old,
            ptr: v_ptr,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Load {
            dst: v_seen,
            ptr: v_ptr,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Store {
            value: v_new,
            ptr: v_ptr,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return { value: None });
    }

    module.add_function(func);

    let eliminated = eliminate_dead_code(&mut module);
    assert_eq!(eliminated, 0);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    let stores = bb0
        .instructions
        .iter()
        .filter(|inst| matches!(inst, MirInstruction::Store { .. }))
        .count();
    assert_eq!(stores, 2);
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Load { dst, .. } if *dst == v_seen)));
}

#[test]
fn test_dce_prunes_overwritten_store_through_copy_alias_private_carrier() {
    let mut module = MirModule::new("dce_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_box = ValueId(1);
    let v_ptr = ValueId(2);
    let v_ptr_copy = ValueId(3);
    let v_old = ValueId(4);
    let v_new = ValueId(5);

    {
        let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
        bb0.instructions.push(MirInstruction::NewBox {
            dst: v_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::RefNew {
            dst: v_ptr,
            box_val: v_box,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Copy {
            dst: v_ptr_copy,
            src: v_ptr,
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
        bb0.instructions.push(MirInstruction::Store {
            value: v_old,
            ptr: v_ptr,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Store {
            value: v_new,
            ptr: v_ptr_copy,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return { value: None });
    }

    module.add_function(func);

    let eliminated = run_dead_code_and_memory_effect(&mut module);
    assert_eq!(eliminated, 2);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    let stores = bb0
        .instructions
        .iter()
        .filter(|inst| matches!(inst, MirInstruction::Store { .. }))
        .count();
    assert_eq!(stores, 1);
    assert!(bb0.instructions.iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::Store { value, ptr } if *value == v_new && *ptr == v_ptr_copy
        )
    }));
}
