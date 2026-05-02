use super::*;

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
