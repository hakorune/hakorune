use super::super::*;

#[test]
fn retained_slice_length_plan_reads_placement_effect_window_before_legacy_facts() {
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Box("StringBox".to_string())],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let entry = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");
    entry.instructions.push(method_call(
        ValueId(1),
        ValueId(0),
        "StringBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    entry.instruction_spans.push(Span::unknown());
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: crate::mir::ConstValue::Integer(2),
    });
    entry.instruction_spans.push(Span::unknown());
    entry.instructions.push(MirInstruction::BinOp {
        dst: ValueId(3),
        op: crate::mir::BinaryOp::Div,
        lhs: ValueId(1),
        rhs: ValueId(2),
    });
    entry.instruction_spans.push(Span::unknown());
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId(4),
        value: crate::mir::ConstValue::Integer(0),
    });
    entry.instruction_spans.push(Span::unknown());
    entry.instructions.push(method_call(
        ValueId(5),
        ValueId(0),
        "StringBox",
        "substring",
        vec![ValueId(4), ValueId(3)],
        MirType::Box("StringBox".to_string()),
    ));
    entry.instruction_spans.push(Span::unknown());
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(1),
        edge_args: None,
    });

    let mut loop_block = BasicBlock::new(BasicBlockId(1));
    loop_block.instructions.push(MirInstruction::Copy {
        dst: ValueId(6),
        src: ValueId(5),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::Copy {
        dst: ValueId(7),
        src: ValueId(6),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(method_call(
        ValueId(8),
        ValueId(7),
        "RuntimeDataBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(8)),
    });
    function.add_block(loop_block);

    for (vid, ty) in [
        (ValueId(1), MirType::Integer),
        (ValueId(2), MirType::Integer),
        (ValueId(3), MirType::Integer),
        (ValueId(4), MirType::Integer),
        (ValueId(5), MirType::Box("StringBox".to_string())),
        (ValueId(8), MirType::Integer),
    ] {
        function.metadata.value_types.insert(vid, ty);
    }

    refresh_function_string_corridor_folded_metadata(&mut function);
    function.metadata.string_corridor_facts.clear();

    let def_map = build_value_def_map(&function);
    let use_counts = build_use_counts(&function);
    let plans = collect_retained_len_plans(&function, &def_map, &use_counts);
    let block_plans = plans
        .get(&BasicBlockId(1))
        .expect("retained len plans for loop block");

    assert_eq!(block_plans.len(), 1);
    let plan = &block_plans[0];
    assert_eq!(plan.outer_dst, ValueId(8));
    assert_eq!(plan.source, ValueId(0));
    assert_eq!(plan.start, ValueId(4));
    assert_eq!(plan.end, ValueId(3));
}

#[test]
fn substring_len_plan_reads_placement_effect_window_before_legacy_facts() {
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![
            MirType::Box("StringBox".to_string()),
            MirType::Integer,
            MirType::Integer,
        ],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

    block.instructions.push(method_call(
        ValueId(3),
        ValueId(0),
        "StringBox",
        "substring",
        vec![ValueId(1), ValueId(2)],
        MirType::Box("StringBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(4),
        src: ValueId(3),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(5),
        ValueId(4),
        "RuntimeDataBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(5)),
    });

    function
        .metadata
        .value_types
        .insert(ValueId(3), MirType::Box("StringBox".to_string()));
    function
        .metadata
        .value_types
        .insert(ValueId(4), MirType::Box("RuntimeDataBox".to_string()));
    function
        .metadata
        .value_types
        .insert(ValueId(5), MirType::Integer);

    refresh_function_string_corridor_folded_metadata(&mut function);
    function.metadata.string_corridor_facts.clear();

    let def_map = build_value_def_map(&function);
    let use_counts = build_use_counts(&function);
    let plans = collect_plans(&function, &def_map, &use_counts);
    let block_plans = plans
        .get(&BasicBlockId(0))
        .expect("same-block substring len plans");

    assert_eq!(block_plans.len(), 1);
    let plan = block_plans[0];
    assert_eq!(plan.inner_dst, ValueId(4));
    assert_eq!(plan.outer_dst, ValueId(5));
    assert_eq!(plan.source, ValueId(0));
    assert_eq!(plan.start, ValueId(1));
    assert_eq!(plan.end, ValueId(2));
}
