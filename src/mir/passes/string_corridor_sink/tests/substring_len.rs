use super::*;

#[test]
fn rewrites_single_use_substring_length_chain_to_direct_extern() {
    let mut module = MirModule::new("substring_len".to_string());
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
    block.instructions.push(method_call(
        ValueId(4),
        ValueId(3),
        "StringBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(4)),
    });

    function
        .metadata
        .value_types
        .insert(ValueId(3), MirType::Box("StringBox".to_string()));
    function
        .metadata
        .value_types
        .insert(ValueId(4), MirType::Integer);
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert_eq!(rewritten, 1);

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
    assert_eq!(block.instructions.len(), block.instruction_spans.len());
    assert_eq!(block.instructions.len(), 1);
    match &block.instructions[0] {
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } => {
            assert_eq!(*dst, Some(ValueId(4)));
            assert_eq!(name, SUBSTRING_LEN_EXTERN);
            assert_eq!(args, &vec![ValueId(0), ValueId(1), ValueId(2)]);
        }
        other => panic!("expected direct extern rewrite, got {other:?}"),
    }
}

#[test]
fn rewrites_runtime_data_substring_length_chain_through_copy_chain() {
    let mut module = MirModule::new("substring_len_runtime_data".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![
            MirType::Box("RuntimeDataBox".to_string()),
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
        "RuntimeDataBox",
        "substring",
        vec![ValueId(1), ValueId(2)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(4),
        src: ValueId(3),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(5),
        src: ValueId(4),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(6),
        ValueId(5),
        "RuntimeDataBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(6)),
    });

    function
        .metadata
        .value_types
        .insert(ValueId(3), MirType::Box("RuntimeDataBox".to_string()));
    function
        .metadata
        .value_types
        .insert(ValueId(4), MirType::Box("RuntimeDataBox".to_string()));
    function
        .metadata
        .value_types
        .insert(ValueId(5), MirType::Box("RuntimeDataBox".to_string()));
    function
        .metadata
        .value_types
        .insert(ValueId(6), MirType::Integer);
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert_eq!(rewritten, 1);

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
    assert_eq!(block.instructions.len(), 3);
    match &block.instructions[2] {
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } => {
            assert_eq!(*dst, Some(ValueId(6)));
            assert_eq!(name, SUBSTRING_LEN_EXTERN);
            assert_eq!(args, &vec![ValueId(0), ValueId(1), ValueId(2)]);
        }
        other => panic!("expected direct extern rewrite, got {other:?}"),
    }
}

#[test]
fn keeps_substring_when_result_has_multiple_uses() {
    let mut module = MirModule::new("substring_len_multiuse".to_string());
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
    block.instructions.push(method_call(
        ValueId(4),
        ValueId(3),
        "StringBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(5),
        ValueId(3),
        "StringBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(4)),
    });

    function
        .metadata
        .value_types
        .insert(ValueId(3), MirType::Box("StringBox".to_string()));
    function
        .metadata
        .value_types
        .insert(ValueId(4), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(5), MirType::Integer);
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert_eq!(rewritten, 0);
}

#[test]
fn fuses_complementary_substring_len_pair_back_to_source_length() {
    let mut module = MirModule::new("substring_len_fusion".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Box("StringBox".to_string())],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

    block.instructions.push(method_call(
        ValueId(1),
        ValueId(0),
        "StringBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: crate::mir::ConstValue::Integer(2),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(3),
        op: crate::mir::BinaryOp::Div,
        lhs: ValueId(1),
        rhs: ValueId(2),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(4),
        value: crate::mir::ConstValue::Integer(0),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(9),
        value: crate::mir::ConstValue::Integer(7),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(extern_call(
        ValueId(5),
        SUBSTRING_LEN_EXTERN,
        vec![ValueId(0), ValueId(4), ValueId(3)],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(extern_call(
        ValueId(6),
        SUBSTRING_LEN_EXTERN,
        vec![ValueId(0), ValueId(3), ValueId(1)],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(7),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(9),
        rhs: ValueId(5),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(8),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(7),
        rhs: ValueId(6),
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(8)),
    });

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
    function
        .metadata
        .value_types
        .insert(ValueId(4), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(5), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(6), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(7), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(8), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(9), MirType::Integer);
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert_eq!(rewritten, 1);

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
    assert!(
        block.instructions.iter().all(|inst| {
            !matches!(
                inst,
                MirInstruction::Call {
                    callee: Some(Callee::Extern(name)),
                    ..
                } if name == SUBSTRING_LEN_EXTERN
            )
        }),
        "complementary substring_len_hii calls should be gone: {:?}",
        block.instructions
    );
    assert!(
        block.instructions.iter().any(|inst| {
            matches!(
                inst,
                MirInstruction::BinOp {
                    dst,
                    op: crate::mir::BinaryOp::Add,
                    lhs,
                    rhs,
                } if *dst == ValueId(8) && *lhs == ValueId(9) && *rhs == ValueId(1)
            )
        }),
        "outer add should fuse to acc + source_len: {:?}",
        block.instructions
    );
}

#[test]
fn keeps_non_complementary_substring_len_pair() {
    let mut module = MirModule::new("substring_len_fusion_negative".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Box("StringBox".to_string())],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

    block.instructions.push(method_call(
        ValueId(1),
        ValueId(0),
        "StringBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: crate::mir::ConstValue::Integer(6),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(3),
        value: crate::mir::ConstValue::Integer(9),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(extern_call(
        ValueId(4),
        SUBSTRING_LEN_EXTERN,
        vec![ValueId(0), ValueId(2), ValueId(3)],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(extern_call(
        ValueId(5),
        SUBSTRING_LEN_EXTERN,
        vec![ValueId(0), ValueId(3), ValueId(2)],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(6),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(4),
        rhs: ValueId(5),
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(6)),
    });

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
    function
        .metadata
        .value_types
        .insert(ValueId(4), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(5), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(6), MirType::Integer);
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert_eq!(rewritten, 0);

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
    let substring_len_calls = block
        .instructions
        .iter()
        .filter(|inst| {
            matches!(
                inst,
                MirInstruction::Call {
                    callee: Some(Callee::Extern(name)),
                    ..
                } if name == SUBSTRING_LEN_EXTERN
            )
        })
        .count();
    assert_eq!(substring_len_calls, 2);
}

#[test]
fn fuses_complementary_substring_len_pair_with_entry_len_and_duplicated_const_source() {
    let mut module = MirModule::new("substring_len_fusion_cross_block".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let entry = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId(9),
        value: crate::mir::ConstValue::String("line-seed-abcdef".to_string()),
    });
    entry.instruction_spans.push(Span::unknown());
    entry.instructions.push(MirInstruction::Copy {
        dst: ValueId(8),
        src: ValueId(9),
    });
    entry.instruction_spans.push(Span::unknown());
    entry.instructions.push(method_call(
        ValueId(5),
        ValueId(8),
        "StringBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    entry.instruction_spans.push(Span::unknown());
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId(25),
        value: crate::mir::ConstValue::Integer(7),
    });
    entry.instruction_spans.push(Span::unknown());
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(19),
        edge_args: None,
    });

    let mut loop_block = BasicBlock::new(BasicBlockId(19));
    loop_block.instructions.push(MirInstruction::Const {
        dst: ValueId(41),
        value: crate::mir::ConstValue::Integer(0),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::Copy {
        dst: ValueId(46),
        src: ValueId(5),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::Copy {
        dst: ValueId(45),
        src: ValueId(46),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::Copy {
        dst: ValueId(44),
        src: ValueId(45),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::Const {
        dst: ValueId(47),
        value: crate::mir::ConstValue::Integer(2),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(43),
        op: crate::mir::BinaryOp::Div,
        lhs: ValueId(44),
        rhs: ValueId(47),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::Copy {
        dst: ValueId(42),
        src: ValueId(43),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::Const {
        dst: ValueId(49),
        value: crate::mir::ConstValue::String("line-seed-abcdef".to_string()),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::Copy {
        dst: ValueId(48),
        src: ValueId(49),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::Copy {
        dst: ValueId(50),
        src: ValueId(46),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(extern_call(
        ValueId(30),
        SUBSTRING_LEN_EXTERN,
        vec![ValueId(48), ValueId(41), ValueId(42)],
    ));
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::Copy {
        dst: ValueId(53),
        src: ValueId(25),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::Copy {
        dst: ValueId(54),
        src: ValueId(30),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(extern_call(
        ValueId(32),
        SUBSTRING_LEN_EXTERN,
        vec![ValueId(48), ValueId(42), ValueId(50)],
    ));
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::Copy {
        dst: ValueId(58),
        src: ValueId(53),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::Copy {
        dst: ValueId(59),
        src: ValueId(54),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(57),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(58),
        rhs: ValueId(59),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::Copy {
        dst: ValueId(60),
        src: ValueId(32),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(33),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(57),
        rhs: ValueId(60),
    });
    loop_block.instruction_spans.push(Span::unknown());
    loop_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(33)),
    });
    function.add_block(loop_block);

    for vid in [
        5u32, 25, 30, 32, 33, 41, 42, 43, 44, 45, 46, 47, 50, 53, 54, 57, 58, 59, 60,
    ] {
        function
            .metadata
            .value_types
            .insert(ValueId(vid), MirType::Integer);
    }
    module.add_function(function);

    let function = module.get_function("main").expect("main");
    let def_map = build_value_def_map(function);
    let use_counts = build_use_counts(function);
    let first_call =
        match_substring_len_call_in_block(function, BasicBlockId(19), &def_map, ValueId(30))
            .expect("first substring_len");
    let second_call =
        match_substring_len_call_in_block(function, BasicBlockId(19), &def_map, ValueId(32))
            .expect("second substring_len");
    assert_eq!(
        complementary_pair_source_len(function, &def_map, &first_call, &second_call),
        Some(ValueId(5))
    );
    let plans = collect_complementary_len_fusion_plans(function, &def_map, &use_counts);
    assert_eq!(plans.get(&BasicBlockId(19)).map(Vec::len), Some(1));

    let function = module.get_function_mut("main").expect("main");
    let rewritten = sink_borrowed_string_corridors_in_function(function);
    assert_eq!(rewritten, 1);

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(19)).expect("loop");
    assert!(
        block.instructions.iter().all(|inst| {
            !matches!(
                inst,
                MirInstruction::Call {
                    callee: Some(Callee::Extern(name)),
                    ..
                } if name == SUBSTRING_LEN_EXTERN
            )
        }),
        "cross-block complementary substring_len_hii calls should be gone: {:?}",
        block.instructions
    );
    assert!(
        block.instructions.iter().any(|inst| {
            matches!(
                inst,
                MirInstruction::BinOp {
                    dst,
                    op: crate::mir::BinaryOp::Add,
                    lhs,
                    rhs,
                } if *dst == ValueId(33) && *lhs == ValueId(25) && *rhs == ValueId(5)
            )
        }),
        "outer add should fuse to acc + source_len across blocks: {:?}",
        block.instructions
    );
}

#[test]
fn rewrites_retained_slice_length_consumer_across_blocks() {
    let mut module = MirModule::new("substring_len_retained_cross_block".to_string());
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
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert_eq!(rewritten, 1);

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(1)).expect("loop");
    assert!(
        block.instructions.iter().any(|inst| {
            matches!(
                inst,
                MirInstruction::Call {
                    dst: Some(dst),
                    callee: Some(Callee::Extern(name)),
                    args,
                    ..
                } if *dst == ValueId(8)
                    && name == SUBSTRING_LEN_EXTERN
                    && args.as_slice() == [ValueId(0), ValueId(4), ValueId(3)]
            )
        }),
        "retained slice length should rewrite to substring_len_hii: {:?}",
        block.instructions
    );
}

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
