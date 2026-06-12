use super::*;
use crate::ast::Span;
use crate::mir::{
    BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
    MirModule, MirType, ValueId,
};

#[test]
fn memory_effect_prunes_dead_load_from_private_carrier_root() {
    let mut module = MirModule::new("memory_effect_test".to_string());

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

    let stats = apply(&mut module);
    assert_eq!(stats.memory_effect_optimizations, 1);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::RefNew { dst, .. } if *dst == v_ptr)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Load { dst, .. } if *dst == v_loaded)));
}

#[test]
fn memory_effect_prunes_overwritten_store_on_private_carrier_root() {
    let mut module = MirModule::new("memory_effect_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_box = ValueId(1);
    let v_ptr = ValueId(2);
    let v_value1 = ValueId(3);
    let v_value2 = ValueId(4);

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
            dst: v_value1,
            value: ConstValue::Integer(7),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Const {
            dst: v_value2,
            value: ConstValue::Integer(9),
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Store {
            value: v_value1,
            ptr: v_ptr,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Store {
            value: v_value2,
            ptr: v_ptr,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return { value: None });
    }

    module.add_function(func);

    let stats = apply(&mut module);
    assert_eq!(stats.memory_effect_optimizations, 1);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Store { value, .. } if *value == v_value2)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Store { value, .. } if *value == v_value1)));
}

#[test]
fn memory_effect_forwards_same_block_store_to_load_from_private_carrier_root() {
    let mut module = MirModule::new("memory_effect_test".to_string());

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
            value: ConstValue::Integer(13),
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
        bb0.set_terminator(MirInstruction::Return {
            value: Some(v_loaded),
        });
    }

    module.add_function(func);

    let stats = apply(&mut module);
    assert_eq!(stats.memory_effect_optimizations, 1);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Copy { dst, src } if *dst == v_loaded && *src == v_value)));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Load { dst, .. } if *dst == v_loaded)));
}

#[test]
fn memory_effect_eliminates_same_block_redundant_load_on_private_carrier_root() {
    let mut module = MirModule::new("memory_effect_test".to_string());

    let sig = FunctionSignature {
        name: "test/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(sig, BasicBlockId(0));

    let v_box = ValueId(1);
    let v_ptr = ValueId(2);
    let v_loaded1 = ValueId(3);
    let v_loaded2 = ValueId(4);

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
            dst: v_loaded1,
            ptr: v_ptr,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.instructions.push(MirInstruction::Load {
            dst: v_loaded2,
            ptr: v_ptr,
        });
        bb0.instruction_spans.push(Span::unknown());
        bb0.set_terminator(MirInstruction::Return {
            value: Some(v_loaded2),
        });
    }

    module.add_function(func);

    let stats = apply(&mut module);
    assert_eq!(stats.memory_effect_optimizations, 1);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Load { dst, .. } if *dst == v_loaded1)));
    assert!(bb0.instructions.iter().any(|inst| matches!(
        inst,
        MirInstruction::Copy { dst, src } if *dst == v_loaded2 && *src == v_loaded1
    )));
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Load { dst, .. } if *dst == v_loaded2)));
}

#[test]
fn memory_effect_keeps_store_when_load_intervenes_on_private_carrier_root() {
    let mut module = MirModule::new("memory_effect_test".to_string());

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
        bb0.set_terminator(MirInstruction::Return {
            value: Some(v_seen),
        });
    }

    module.add_function(func);

    let stats = apply(&mut module);
    assert_eq!(stats.memory_effect_optimizations, 1);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Store { value, .. } if *value == v_old)));
    assert!(bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Store { value, .. } if *value == v_new)));
    assert!(bb0.instructions.iter().any(|inst| matches!(
        inst,
        MirInstruction::Copy { dst, src } if *dst == v_seen && *src == v_old
    )));
}

#[test]
fn memory_effect_prunes_private_carrier_store_overwritten_by_successor_store() {
    let mut module = MirModule::new("memory_effect_test".to_string());

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
    let bb1_id = BasicBlockId(1);
    func.add_block(crate::mir::BasicBlock::new(bb1_id));

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
        bb0.set_terminator(MirInstruction::Jump {
            target: bb1_id,
            edge_args: None,
        });
    }
    {
        let bb1 = func.blocks.get_mut(&bb1_id).unwrap();
        bb1.instructions.push(MirInstruction::Store {
            value: v_new,
            ptr: v_ptr,
        });
        bb1.instruction_spans.push(Span::unknown());
        bb1.set_terminator(MirInstruction::Return { value: None });
    }

    module.add_function(func);

    let stats = apply(&mut module);
    assert_eq!(stats.memory_effect_optimizations, 1);

    let func = module.get_function("test/0").unwrap();
    let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
    let bb1 = func.blocks.get(&bb1_id).unwrap();
    assert!(!bb0
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Store { value, .. } if *value == v_old)));
    assert!(bb1
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Store { value, .. } if *value == v_new)));
}
