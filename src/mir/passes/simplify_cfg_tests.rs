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
    assert!(simplified >= 1);

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
fn simplifies_constant_branch_to_jump_from_copied_bool() {
    let mut module = MirModule::new("simplify_cfg_const_branch".to_string());
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
        entry.instructions.push(MirInstruction::Copy {
            dst: ValueId(2),
            src: ValueId(1),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(2),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(2),
            then_edge_args: Some(EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(2)],
            }),
            else_edge_args: None,
        });
    }

    let mut then_block = BasicBlock::new(BasicBlockId(1));
    then_block.instructions.push(MirInstruction::Phi {
        dst: ValueId(3),
        inputs: vec![(BasicBlockId(0), ValueId(2)), (BasicBlockId(3), ValueId(4))],
        type_hint: Some(MirType::Integer),
    });
    then_block.instruction_spans.push(Span::unknown());
    then_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(3)),
    });
    function.add_block(then_block);

    let mut else_block = BasicBlock::new(BasicBlockId(2));
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });
    function.add_block(else_block);

    let mut helper = BasicBlock::new(BasicBlockId(3));
    helper.instructions.push(MirInstruction::Const {
        dst: ValueId(4),
        value: ConstValue::Integer(22),
    });
    helper.instruction_spans.push(Span::unknown());
    helper.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(1),
        edge_args: None,
    });
    function.add_block(helper);

    function
        .metadata
        .value_types
        .insert(ValueId(1), MirType::Bool);
    function
        .metadata
        .value_types
        .insert(ValueId(2), MirType::Bool);
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
    assert!(simplified >= 1);

    let function = module.functions.get("main").expect("main function");
    let entry = function.blocks.get(&BasicBlockId(0)).expect("entry block");
    assert!(matches!(
        entry.instructions.as_slice(),
        [
            MirInstruction::Const {
                dst: const_dst,
                value: ConstValue::Bool(true)
            },
            MirInstruction::Copy {
                dst: copy_dst,
                src: ValueId(1)
            }
        ] if *const_dst == ValueId(1) && *copy_dst == ValueId(2)
    ));
    assert!(matches!(
        &entry.terminator,
        Some(MirInstruction::Jump {
            target,
            edge_args: Some(EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values
            })
        }) if *target == BasicBlockId(1) && values.as_slice() == [ValueId(2)]
    ));
}

#[test]
fn simplifies_constant_branch_to_jump_from_constant_compare() {
    let mut module = MirModule::new("simplify_cfg_const_compare_branch".to_string());
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
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: ConstValue::Integer(7),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(MirInstruction::Compare {
            dst: ValueId(3),
            op: crate::mir::CompareOp::Eq,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(3),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(2),
            then_edge_args: Some(EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(1)],
            }),
            else_edge_args: None,
        });
    }

    let mut then_block = BasicBlock::new(BasicBlockId(1));
    then_block.instructions.push(MirInstruction::Phi {
        dst: ValueId(4),
        inputs: vec![(BasicBlockId(2), ValueId(4))],
        type_hint: Some(MirType::Integer),
    });
    then_block.instruction_spans.push(Span::unknown());
    then_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(1)),
    });
    function.add_block(then_block);

    let mut else_block = BasicBlock::new(BasicBlockId(2));
    else_block.set_terminator(MirInstruction::Return { value: None });
    function.add_block(else_block);

    function
        .metadata
        .value_types
        .insert(ValueId(1), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(2), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(3), MirType::Bool);
    function
        .metadata
        .value_types
        .insert(ValueId(4), MirType::Integer);
    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert!(simplified >= 1);

    let function = module.functions.get("main").expect("main function");
    let entry = function.blocks.get(&BasicBlockId(0)).expect("entry block");
    assert!(matches!(
        &entry.terminator,
        Some(MirInstruction::Jump {
            target,
            edge_args: Some(EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values
            })
        }) if *target == BasicBlockId(1) && values.as_slice() == [ValueId(1)]
    ));
}

#[test]
fn simplifies_compare_to_const_from_single_input_phi() {
    let mut module = MirModule::new("simplify_cfg_phi_compare".to_string());
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Bool],
            return_type: MirType::Bool,
            effects: EffectMask::PURE,
        },
        BasicBlockId(0),
    );

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
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(2),
            then_edge_args: None,
            else_edge_args: None,
        });
    }

    let mut then_block = BasicBlock::new(BasicBlockId(1));
    then_block.instructions.push(MirInstruction::Phi {
        dst: ValueId(2),
        inputs: vec![(BasicBlockId(0), ValueId(1))],
        type_hint: Some(MirType::Integer),
    });
    then_block.instruction_spans.push(Span::unknown());
    then_block.instructions.push(MirInstruction::Compare {
        dst: ValueId(3),
        op: crate::mir::CompareOp::Eq,
        lhs: ValueId(2),
        rhs: ValueId(1),
    });
    then_block.instruction_spans.push(Span::unknown());
    then_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(3)),
    });
    function.add_block(then_block);

    let mut else_block = BasicBlock::new(BasicBlockId(2));
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(1),
        edge_args: None,
    });
    function.add_block(else_block);

    function
        .metadata
        .value_types
        .insert(ValueId(0), MirType::Bool);
    function
        .metadata
        .value_types
        .insert(ValueId(1), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(2), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(3), MirType::Bool);
    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert!(simplified >= 1);

    let function = module.functions.get("main").expect("main function");
    let then_block = function.blocks.get(&BasicBlockId(1)).expect("then block");
    assert!(matches!(
        then_block.instructions.as_slice(),
        [
            MirInstruction::Phi { .. },
            MirInstruction::Const {
                dst,
                value: ConstValue::Bool(true)
            }
        ] if *dst == ValueId(3)
    ));
}

#[test]
fn simplifies_jump_block_and_rewrites_successor_phi_inputs() {
    let mut module = MirModule::new("simplify_cfg_phi_rewrite".to_string());
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Bool],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId(0),
    );

    {
        let entry = function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block");
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(3),
            then_edge_args: None,
            else_edge_args: None,
        });
    }

    let mut then_bridge = BasicBlock::new(BasicBlockId(1));
    then_bridge.instructions.push(MirInstruction::Debug {
        value: ValueId(0),
        message: "keep bridge merge-focused".to_string(),
    });
    then_bridge.instruction_spans.push(Span::unknown());
    then_bridge.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(2),
        edge_args: None,
    });
    function.add_block(then_bridge);

    let mut middle = BasicBlock::new(BasicBlockId(2));
    middle.instructions.push(MirInstruction::Phi {
        dst: ValueId(6),
        inputs: vec![(BasicBlockId(1), ValueId(2))],
        type_hint: Some(MirType::Integer),
    });
    middle.instruction_spans.push(Span::unknown());
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
        .insert(ValueId(0), MirType::Bool);
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
        .insert(ValueId(6), MirType::Integer);

    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert!(simplified >= 1);

    let function = module.functions.get("main").expect("main function");
    assert!(!function.blocks.contains_key(&BasicBlockId(2)));

    let then_block = function.blocks.get(&BasicBlockId(1)).expect("then block");
    assert!(matches!(
        then_block.instructions.as_slice(),
        [
            MirInstruction::Debug {
                value: ValueId(0),
                message
            },
            MirInstruction::Const {
                dst,
                value: ConstValue::Integer(11)
            }
        ] if message == "keep bridge merge-focused" && *dst == ValueId(2)
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
    assert!(simplified >= 1);

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
    assert!(simplified >= 1);

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
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Bool],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId(0),
    );

    {
        let entry = function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block");
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0),
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
        .insert(ValueId(0), MirType::Bool);
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
    assert!(simplified >= 1);

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

#[test]
fn threads_branch_through_empty_jump_trampoline() {
    let mut module = MirModule::new("simplify_cfg_jump_thread".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Bool],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));

    {
        let entry = function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block");
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(4),
            then_edge_args: None,
            else_edge_args: None,
        });
    }

    let mut trampoline = BasicBlock::new(BasicBlockId(1));
    trampoline.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });
    function.add_block(trampoline);

    let mut middle = BasicBlock::new(BasicBlockId(2));
    middle.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });
    function.add_block(middle);

    let mut dispatcher = BasicBlock::new(BasicBlockId(4));
    dispatcher.instructions.push(MirInstruction::Debug {
        value: ValueId(0),
        message: "keep dispatcher non-threadable".to_string(),
    });
    dispatcher.instruction_spans.push(Span::unknown());
    dispatcher.set_terminator(MirInstruction::Branch {
        condition: ValueId(0),
        then_bb: BasicBlockId(5),
        else_bb: BasicBlockId(6),
        then_edge_args: None,
        else_edge_args: None,
    });
    function.add_block(dispatcher);

    let mut anchor_then = BasicBlock::new(BasicBlockId(5));
    anchor_then.instructions.push(MirInstruction::Debug {
        value: ValueId(0),
        message: "anchor then".to_string(),
    });
    anchor_then.instruction_spans.push(Span::unknown());
    anchor_then.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(2),
        edge_args: None,
    });
    function.add_block(anchor_then);

    let mut anchor_else = BasicBlock::new(BasicBlockId(6));
    anchor_else.instructions.push(MirInstruction::Debug {
        value: ValueId(0),
        message: "anchor else".to_string(),
    });
    anchor_else.instruction_spans.push(Span::unknown());
    anchor_else.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(2),
        edge_args: None,
    });
    function.add_block(anchor_else);

    let mut final_block = BasicBlock::new(BasicBlockId(3));
    final_block.set_terminator(MirInstruction::Return { value: None });
    function.add_block(final_block);

    function
        .metadata
        .value_types
        .insert(ValueId(0), MirType::Bool);
    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert!(simplified >= 1);

    let function = module.functions.get("main").expect("main function");
    let entry = function.blocks.get(&BasicBlockId(0)).expect("entry block");
    assert!(matches!(
        entry.terminator,
        Some(MirInstruction::Branch {
            then_bb,
            else_bb,
            then_edge_args: None,
            else_edge_args: None,
            ..
        }) if then_bb == BasicBlockId(3) && else_bb == BasicBlockId(4)
    ));
    assert!(function.blocks.contains_key(&BasicBlockId(1)));
    assert!(function.blocks.contains_key(&BasicBlockId(2)));
    assert!(function.blocks.contains_key(&BasicBlockId(4)));
}

#[test]
fn threads_branch_through_empty_jump_trampoline_and_rewrites_final_phi_predecessor() {
    let mut module = MirModule::new("simplify_cfg_jump_thread_phi_target".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Bool],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));

    {
        let entry = function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block");
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(11),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(2),
            then_edge_args: None,
            else_edge_args: None,
        });
    }

    let mut trampoline = BasicBlock::new(BasicBlockId(1));
    trampoline.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });
    function.add_block(trampoline);

    let mut else_block = BasicBlock::new(BasicBlockId(2));
    else_block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: ConstValue::Integer(22),
    });
    else_block.instruction_spans.push(Span::unknown());
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });
    function.add_block(else_block);

    let mut final_block = BasicBlock::new(BasicBlockId(3));
    final_block.instructions.push(MirInstruction::Phi {
        dst: ValueId(3),
        inputs: vec![(BasicBlockId(1), ValueId(1)), (BasicBlockId(2), ValueId(2))],
        type_hint: Some(MirType::Integer),
    });
    final_block.instruction_spans.push(Span::unknown());
    final_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(3)),
    });
    function.add_block(final_block);

    function
        .metadata
        .value_types
        .insert(ValueId(0), MirType::Bool);
    function
        .metadata
        .value_types
        .insert(ValueId(1), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(2), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(3), MirType::Integer);
    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert!(simplified >= 1);

    let function = module.functions.get("main").expect("main function");
    let entry = function.blocks.get(&BasicBlockId(0)).expect("entry block");
    assert!(matches!(
        entry.terminator,
        Some(MirInstruction::Branch {
            then_bb,
            else_bb,
            then_edge_args: None,
            else_edge_args: None,
            ..
        }) if then_bb == BasicBlockId(3) && else_bb == BasicBlockId(2)
    ));

    let final_block = function.blocks.get(&BasicBlockId(3)).expect("final block");
    let MirInstruction::Phi { inputs, .. } = &final_block.instructions[0] else {
        panic!("expected phi");
    };
    assert_eq!(
        inputs,
        &vec![(BasicBlockId(0), ValueId(1)), (BasicBlockId(2), ValueId(2))]
    );
}

#[test]
fn threads_branch_through_empty_jump_trampoline_and_drops_dead_edge_args() {
    let mut module = MirModule::new("simplify_cfg_jump_thread_edge_args".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Bool],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));

    {
        let entry = function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block");
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(11),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(3),
            then_edge_args: Some(EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(1)],
            }),
            else_edge_args: None,
        });
    }

    let mut trampoline = BasicBlock::new(BasicBlockId(1));
    trampoline.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });
    function.add_block(trampoline);

    let mut final_block = BasicBlock::new(BasicBlockId(3));
    final_block.set_terminator(MirInstruction::Return { value: None });
    function.add_block(final_block);

    function
        .metadata
        .value_types
        .insert(ValueId(0), MirType::Bool);
    function
        .metadata
        .value_types
        .insert(ValueId(1), MirType::Integer);
    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert!(simplified >= 1);

    let function = module.functions.get("main").expect("main function");
    let entry = function.blocks.get(&BasicBlockId(0)).expect("entry block");
    assert!(matches!(
        entry.terminator,
        Some(MirInstruction::Jump {
            target,
            edge_args: None
        }) if target == BasicBlockId(3)
    ));
}

#[test]
fn keeps_branch_trampoline_when_threaded_arm_edge_args_would_cross_final_phi() {
    let mut module = MirModule::new("simplify_cfg_jump_thread_edge_args_phi_guard".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Bool],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));

    {
        let entry = function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block");
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(11),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(2),
            then_edge_args: Some(EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(1)],
            }),
            else_edge_args: None,
        });
    }

    let mut trampoline = BasicBlock::new(BasicBlockId(1));
    trampoline.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });
    function.add_block(trampoline);

    let mut else_block = BasicBlock::new(BasicBlockId(2));
    else_block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: ConstValue::Integer(22),
    });
    else_block.instruction_spans.push(Span::unknown());
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });
    function.add_block(else_block);

    let mut final_block = BasicBlock::new(BasicBlockId(3));
    final_block.instructions.push(MirInstruction::Phi {
        dst: ValueId(3),
        inputs: vec![(BasicBlockId(1), ValueId(1)), (BasicBlockId(2), ValueId(2))],
        type_hint: Some(MirType::Integer),
    });
    final_block.instruction_spans.push(Span::unknown());
    final_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(3)),
    });
    function.add_block(final_block);

    function
        .metadata
        .value_types
        .insert(ValueId(0), MirType::Bool);
    function
        .metadata
        .value_types
        .insert(ValueId(1), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(2), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(3), MirType::Integer);
    function.update_cfg();
    module.add_function(function);

    let simplified = simplify(&mut module);
    assert_eq!(simplified, 0);

    let function = module.functions.get("main").expect("main function");
    let entry = function.blocks.get(&BasicBlockId(0)).expect("entry block");
    assert!(matches!(
        &entry.terminator,
        Some(MirInstruction::Branch {
            then_bb,
            else_bb,
            then_edge_args: Some(EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values
            }),
            else_edge_args: None,
            ..
        }) if *then_bb == BasicBlockId(1) && *else_bb == BasicBlockId(2) && values.as_slice() == [ValueId(1)]
    ));
}
