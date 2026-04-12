use super::*;
use crate::ast::Span;
use crate::mir::{
    join_ir::lowering::inline_boundary::JumpArgsLayout, ConstValue, EdgeArgs, EffectMask,
    FunctionSignature, MirModule, MirType, ValueId,
};

fn test_signature(name: &str, return_type: MirType) -> FunctionSignature {
    FunctionSignature {
        name: name.to_string(),
        params: vec![],
        return_type,
        effects: EffectMask::PURE,
    }
}

#[test]
fn simplifies_linear_single_predecessor_jump_block() {
    let mut module = MirModule::new("simplify_cfg_linear".to_string());
    let mut function = MirFunction::new(test_signature("main", MirType::Integer), BasicBlockId(0));

    {
        let entry = function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block");
        entry.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(1),
            edge_args: None,
        });
    }

    let mut middle = BasicBlock::new(BasicBlockId(1));
    middle.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: ConstValue::Integer(7),
    });
    middle.instruction_spans.push(Span::unknown());
    middle.set_terminator(MirInstruction::Return {
        value: Some(ValueId(1)),
    });
    function.add_block(middle);
    function
        .metadata
        .value_types
        .insert(ValueId(1), MirType::Integer);

    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert_eq!(simplified, 1);

    let function = module.functions.get("main").expect("main function");
    assert_eq!(function.blocks.len(), 1);
    let entry = function.blocks.get(&BasicBlockId(0)).expect("entry block");
    assert!(matches!(
        entry.instructions.as_slice(),
        [MirInstruction::Const {
            dst,
            value: ConstValue::Integer(7)
        }] if *dst == ValueId(1)
    ));
    assert!(matches!(
        entry.terminator,
        Some(MirInstruction::Return {
            value: Some(value)
        }) if value == ValueId(1)
    ));
}

#[test]
fn simplifies_jump_block_and_rewrites_successor_phi_inputs() {
    let mut module = MirModule::new("simplify_cfg_phi_rewrite".to_string());
    let mut function = MirFunction::new(test_signature("main", MirType::Integer), BasicBlockId(0));

    {
        let entry = function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block");
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Bool(true),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(1),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(3),
            then_edge_args: None,
            else_edge_args: None,
        });
    }

    let mut then_bridge = BasicBlock::new(BasicBlockId(1));
    then_bridge.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(2),
        edge_args: None,
    });
    function.add_block(then_bridge);

    let mut middle = BasicBlock::new(BasicBlockId(2));
    middle.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: ConstValue::Integer(11),
    });
    middle.instruction_spans.push(Span::unknown());
    middle.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(4),
        edge_args: None,
    });
    function.add_block(middle);

    let mut else_block = BasicBlock::new(BasicBlockId(3));
    else_block.instructions.push(MirInstruction::Const {
        dst: ValueId(3),
        value: ConstValue::Integer(22),
    });
    else_block.instruction_spans.push(Span::unknown());
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(4),
        edge_args: None,
    });
    function.add_block(else_block);

    let mut merge = BasicBlock::new(BasicBlockId(4));
    merge.instructions.push(MirInstruction::Phi {
        dst: ValueId(4),
        inputs: vec![(BasicBlockId(2), ValueId(2)), (BasicBlockId(3), ValueId(3))],
        type_hint: Some(MirType::Integer),
    });
    merge.instruction_spans.push(Span::unknown());
    merge.set_terminator(MirInstruction::Return {
        value: Some(ValueId(4)),
    });
    function.add_block(merge);

    function
        .metadata
        .value_types
        .insert(ValueId(1), MirType::Bool);
    function
        .metadata
        .value_types
        .insert(ValueId(2), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(3), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(4), MirType::Integer);

    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert_eq!(simplified, 1);

    let function = module.functions.get("main").expect("main function");
    assert!(!function.blocks.contains_key(&BasicBlockId(2)));

    let then_block = function.blocks.get(&BasicBlockId(1)).expect("then block");
    assert!(matches!(
        then_block.instructions.as_slice(),
        [MirInstruction::Const {
            dst,
            value: ConstValue::Integer(11)
        }] if *dst == ValueId(2)
    ));

    let merge = function.blocks.get(&BasicBlockId(4)).expect("merge block");
    let MirInstruction::Phi { inputs, .. } = &merge.instructions[0] else {
        panic!("expected phi");
    };
    assert_eq!(
        inputs,
        &vec![(BasicBlockId(1), ValueId(2)), (BasicBlockId(3), ValueId(3))]
    );
}

#[test]
fn simplifies_jump_block_when_predecessor_edge_args_are_dead_for_middle() {
    let mut module = MirModule::new("simplify_cfg_edge_args_merge".to_string());
    let mut function = MirFunction::new(test_signature("main", MirType::Void), BasicBlockId(0));

    {
        let entry = function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block");
        entry.set_jump_with_edge_args(
            BasicBlockId(1),
            Some(EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(9)],
            }),
        );
    }

    let mut middle = BasicBlock::new(BasicBlockId(1));
    middle.set_terminator(MirInstruction::Return { value: None });
    function.add_block(middle);
    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert_eq!(simplified, 1);

    let function = module.functions.get("main").expect("main function");
    assert_eq!(function.blocks.len(), 1);
    let entry = function.blocks.get(&BasicBlockId(0)).expect("entry");
    assert!(matches!(
        entry.terminator,
        Some(MirInstruction::Return { value: None })
    ));
}

#[test]
fn keeps_jump_block_when_middle_has_phi_inputs_for_edge_args() {
    let mut module = MirModule::new("simplify_cfg_edge_args_phi_merge".to_string());
    let mut function = MirFunction::new(test_signature("main", MirType::Integer), BasicBlockId(0));

    {
        let entry = function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block");
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(7),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.set_jump_with_edge_args(
            BasicBlockId(1),
            Some(EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(1)],
            }),
        );
    }

    let mut middle = BasicBlock::new(BasicBlockId(1));
    middle.instructions.push(MirInstruction::Phi {
        dst: ValueId(2),
        inputs: vec![(BasicBlockId(0), ValueId(1))],
        type_hint: Some(MirType::Integer),
    });
    middle.instruction_spans.push(Span::unknown());
    middle.set_terminator(MirInstruction::Return {
        value: Some(ValueId(2)),
    });
    function.add_block(middle);
    function
        .metadata
        .value_types
        .insert(ValueId(1), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(2), MirType::Integer);
    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert_eq!(simplified, 1);

    let function = module.functions.get("main").expect("main function");
    assert_eq!(function.blocks.len(), 1);

    let entry = function.blocks.get(&BasicBlockId(0)).expect("entry");
    assert!(!entry
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Phi { .. })));
    assert!(matches!(
        entry.terminator,
        Some(MirInstruction::Return {
            value: Some(value)
        }) if value == ValueId(1)
    ));
}

#[test]
fn simplifies_jump_block_and_rewrites_successor_phi_values_for_trivial_middle_phi() {
    let mut module = MirModule::new("simplify_cfg_phi_value_rewrite".to_string());
    let mut function = MirFunction::new(test_signature("main", MirType::Integer), BasicBlockId(0));

    {
        let entry = function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block");
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Bool(true),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(1),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(3),
            then_edge_args: None,
            else_edge_args: None,
        });
    }

    let mut pred = BasicBlock::new(BasicBlockId(1));
    pred.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: ConstValue::Integer(11),
    });
    pred.instruction_spans.push(Span::unknown());
    pred.set_jump_with_edge_args(
        BasicBlockId(2),
        Some(EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![ValueId(2)],
        }),
    );
    function.add_block(pred);

    let mut middle = BasicBlock::new(BasicBlockId(2));
    middle.instructions.push(MirInstruction::Phi {
        dst: ValueId(3),
        inputs: vec![(BasicBlockId(1), ValueId(2))],
        type_hint: Some(MirType::Integer),
    });
    middle.instruction_spans.push(Span::unknown());
    middle.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(4),
        edge_args: None,
    });
    function.add_block(middle);

    let mut else_block = BasicBlock::new(BasicBlockId(3));
    else_block.instructions.push(MirInstruction::Const {
        dst: ValueId(4),
        value: ConstValue::Integer(22),
    });
    else_block.instruction_spans.push(Span::unknown());
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(4),
        edge_args: None,
    });
    function.add_block(else_block);

    let mut merge = BasicBlock::new(BasicBlockId(4));
    merge.instructions.push(MirInstruction::Phi {
        dst: ValueId(5),
        inputs: vec![(BasicBlockId(2), ValueId(3)), (BasicBlockId(3), ValueId(4))],
        type_hint: Some(MirType::Integer),
    });
    merge.instruction_spans.push(Span::unknown());
    merge.set_terminator(MirInstruction::Return {
        value: Some(ValueId(5)),
    });
    function.add_block(merge);

    function
        .metadata
        .value_types
        .insert(ValueId(1), MirType::Bool);
    function
        .metadata
        .value_types
        .insert(ValueId(2), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(3), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(4), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(5), MirType::Integer);
    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert_eq!(simplified, 1);

    let function = module.functions.get("main").expect("main function");
    assert!(!function.blocks.contains_key(&BasicBlockId(2)));

    let pred = function.blocks.get(&BasicBlockId(1)).expect("pred block");
    assert!(matches!(
        pred.terminator,
        Some(MirInstruction::Jump {
            target,
            edge_args: None
        }) if target == BasicBlockId(4)
    ));

    let merge = function.blocks.get(&BasicBlockId(4)).expect("merge block");
    let MirInstruction::Phi { inputs, .. } = &merge.instructions[0] else {
        panic!("expected phi");
    };
    assert_eq!(
        inputs,
        &vec![(BasicBlockId(1), ValueId(2)), (BasicBlockId(3), ValueId(4))]
    );
}
