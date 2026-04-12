use super::*;

#[test]
fn rewrites_concat_slice_consumers_to_corridor_helpers() {
    let mut module = MirModule::new("substring_concat_corridor".to_string());
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
    block.instructions.push(method_call(
        ValueId(5),
        ValueId(0),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(4), ValueId(3)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(6),
        ValueId(0),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(3), ValueId(1)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(7),
        value: crate::mir::ConstValue::String("xx".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(8),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(5),
        rhs: ValueId(7),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(9),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(8),
        rhs: ValueId(6),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(10),
        ValueId(9),
        "RuntimeDataBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(11),
        value: crate::mir::ConstValue::Integer(1),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(12),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(1),
        rhs: ValueId(11),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(13),
        ValueId(9),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(11), ValueId(12)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(10)),
    });

    for (vid, ty) in [
        (ValueId(1), MirType::Integer),
        (ValueId(2), MirType::Integer),
        (ValueId(3), MirType::Integer),
        (ValueId(4), MirType::Integer),
        (ValueId(5), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(6), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(7), MirType::Box("StringBox".to_string())),
        (ValueId(10), MirType::Integer),
        (ValueId(11), MirType::Integer),
        (ValueId(12), MirType::Integer),
        (ValueId(13), MirType::Box("RuntimeDataBox".to_string())),
    ] {
        function.metadata.value_types.insert(vid, ty);
    }
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert!(
        rewritten >= 2,
        "concat corridor should rewrite both consumers, got {rewritten}"
    );

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
    let substring_len_calls: Vec<_> = block
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
        .collect();
    assert!(
        block.instructions.iter().any(|inst| {
            matches!(
                inst,
                MirInstruction::BinOp {
                    dst,
                    op: crate::mir::BinaryOp::Add,
                    lhs,
                    rhs,
                } if *dst == ValueId(10)
                    && ((*lhs == ValueId(1) && *rhs != ValueId(1))
                        || (*rhs == ValueId(1) && *lhs != ValueId(1)))
            )
        }),
        "concat length should rewrite to source_len + const_len: {:?}",
        block.instructions
    );
    assert!(
        substring_len_calls.is_empty(),
        "complementary substring_len_hii calls should fuse away: {:?}",
        block.instructions
    );
    assert!(
        block.instructions.iter().any(|inst| {
            matches!(
                inst,
                MirInstruction::Call {
                    dst: Some(dst),
                    callee: Some(Callee::Extern(name)),
                    args,
                    ..
                } if *dst == ValueId(13)
                    && name == SUBSTRING_CONCAT3_EXTERN
                    && args.as_slice()
                        == [ValueId(5), ValueId(7), ValueId(6), ValueId(11), ValueId(12)]
            )
        }),
        "concat substring should rewrite to substring_concat3 helper: {:?}",
        block.instructions
    );
}

#[test]
fn rewrites_publication_helper_length_via_plan_metadata() {
    let mut module = MirModule::new("substring_concat_publication_len".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Box("RuntimeDataBox".to_string())],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

    block.instructions.push(method_call(
        ValueId(1),
        ValueId(0),
        "RuntimeDataBox",
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
    block.instructions.push(method_call(
        ValueId(5),
        ValueId(0),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(4), ValueId(3)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(6),
        ValueId(0),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(3), ValueId(1)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(7),
        value: crate::mir::ConstValue::String("xx".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(8),
        value: crate::mir::ConstValue::Integer(1),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(9),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(1),
        rhs: ValueId(8),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(extern_call(
        ValueId(10),
        SUBSTRING_CONCAT3_EXTERN,
        vec![ValueId(5), ValueId(7), ValueId(6), ValueId(8), ValueId(9)],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(11),
        src: ValueId(10),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(12),
        ValueId(11),
        "RuntimeDataBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(12)),
    });

    for (vid, ty) in [
        (ValueId(1), MirType::Integer),
        (ValueId(2), MirType::Integer),
        (ValueId(3), MirType::Integer),
        (ValueId(4), MirType::Integer),
        (ValueId(5), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(6), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(7), MirType::Box("StringBox".to_string())),
        (ValueId(8), MirType::Integer),
        (ValueId(9), MirType::Integer),
        (ValueId(10), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(11), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(12), MirType::Integer),
    ] {
        function.metadata.value_types.insert(vid, ty);
    }
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert!(
        rewritten >= 1,
        "publication helper length should rewrite, got {rewritten}"
    );

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
    assert!(
        block.instructions.iter().any(|inst| {
            matches!(
                inst,
                MirInstruction::BinOp {
                    dst,
                    op: crate::mir::BinaryOp::Sub,
                    lhs,
                    rhs,
                } if *dst == ValueId(12) && *lhs == ValueId(9) && *rhs == ValueId(8)
            )
        }),
        "publication helper length should rewrite to end-start: {:?}",
        block.instructions
    );
}

#[test]
fn rewrites_publication_helper_substring_via_plan_metadata() {
    let mut module = MirModule::new("substring_concat_publication_substring".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Box("RuntimeDataBox".to_string())],
        return_type: MirType::Box("RuntimeDataBox".to_string()),
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

    block.instructions.push(method_call(
        ValueId(1),
        ValueId(0),
        "RuntimeDataBox",
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
    block.instructions.push(method_call(
        ValueId(5),
        ValueId(0),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(4), ValueId(3)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(6),
        ValueId(0),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(3), ValueId(1)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(7),
        value: crate::mir::ConstValue::String("xx".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(8),
        value: crate::mir::ConstValue::Integer(1),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(9),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(1),
        rhs: ValueId(8),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(extern_call(
        ValueId(10),
        SUBSTRING_CONCAT3_EXTERN,
        vec![ValueId(5), ValueId(7), ValueId(6), ValueId(8), ValueId(9)],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(11),
        src: ValueId(10),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(12),
        value: crate::mir::ConstValue::Integer(2),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(13),
        value: crate::mir::ConstValue::Integer(4),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(14),
        ValueId(11),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(12), ValueId(13)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(14)),
    });

    for (vid, ty) in [
        (ValueId(1), MirType::Integer),
        (ValueId(2), MirType::Integer),
        (ValueId(3), MirType::Integer),
        (ValueId(4), MirType::Integer),
        (ValueId(5), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(6), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(7), MirType::Box("StringBox".to_string())),
        (ValueId(8), MirType::Integer),
        (ValueId(9), MirType::Integer),
        (ValueId(10), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(11), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(12), MirType::Integer),
        (ValueId(13), MirType::Integer),
        (ValueId(14), MirType::Box("RuntimeDataBox".to_string())),
    ] {
        function.metadata.value_types.insert(vid, ty);
    }
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert!(
        rewritten >= 1,
        "publication helper substring should rewrite, got {rewritten}"
    );

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");

    let mut add_roots = std::collections::BTreeMap::new();
    for inst in &block.instructions {
        if let MirInstruction::BinOp {
            dst,
            op: crate::mir::BinaryOp::Add,
            lhs,
            rhs,
        } = inst
        {
            add_roots.insert(*dst, (*lhs, *rhs));
        }
    }

    let helper_call = block.instructions.iter().find_map(|inst| match inst {
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if *dst == ValueId(14) && name == SUBSTRING_CONCAT3_EXTERN => Some(args.clone()),
        _ => None,
    });
    let helper_args = helper_call.expect("publication helper substring call");
    assert_eq!(&helper_args[..3], &[ValueId(5), ValueId(7), ValueId(6)]);
    let composed_start = helper_args[3];
    let composed_end = helper_args[4];
    assert_eq!(
        add_roots.get(&composed_start),
        Some(&(ValueId(8), ValueId(12)))
    );
    assert_eq!(
        add_roots.get(&composed_end),
        Some(&(ValueId(8), ValueId(13)))
    );
}

#[test]
fn sinks_publication_helper_to_same_block_return_boundary() {
    let mut module = MirModule::new("substring_concat_publication_return".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Box("RuntimeDataBox".to_string())],
        return_type: MirType::Box("RuntimeDataBox".to_string()),
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

    block.instructions.push(method_call(
        ValueId(1),
        ValueId(0),
        "RuntimeDataBox",
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
    block.instructions.push(method_call(
        ValueId(5),
        ValueId(0),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(4), ValueId(3)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(6),
        ValueId(0),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(3), ValueId(1)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(7),
        value: crate::mir::ConstValue::String("xx".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(8),
        value: crate::mir::ConstValue::Integer(1),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(9),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(1),
        rhs: ValueId(8),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(extern_call(
        ValueId(10),
        SUBSTRING_CONCAT3_EXTERN,
        vec![ValueId(5), ValueId(7), ValueId(6), ValueId(8), ValueId(9)],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(11),
        src: ValueId(10),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(12),
        value: crate::mir::ConstValue::Integer(7),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(13),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(1),
        rhs: ValueId(12),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(14),
        src: ValueId(11),
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(14)),
    });

    for (vid, ty) in [
        (ValueId(1), MirType::Integer),
        (ValueId(2), MirType::Integer),
        (ValueId(3), MirType::Integer),
        (ValueId(4), MirType::Integer),
        (ValueId(5), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(6), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(7), MirType::Box("StringBox".to_string())),
        (ValueId(8), MirType::Integer),
        (ValueId(9), MirType::Integer),
        (ValueId(10), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(11), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(12), MirType::Integer),
        (ValueId(13), MirType::Integer),
        (ValueId(14), MirType::Box("RuntimeDataBox".to_string())),
    ] {
        function.metadata.value_types.insert(vid, ty);
    }
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert!(
        rewritten >= 1,
        "publication helper return sink should rewrite, got {rewritten}"
    );

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
    assert!(
        block.instructions.iter().all(|inst| {
            !matches!(
                inst,
                MirInstruction::Copy { dst, .. } if *dst == ValueId(11) || *dst == ValueId(14)
            )
        }),
        "copy-only return chain should disappear: {:?}",
        block.instructions
    );

    let add_idx = block
        .instructions
        .iter()
        .position(|inst| {
            matches!(
                inst,
                MirInstruction::BinOp {
                    dst,
                    op: crate::mir::BinaryOp::Add,
                    lhs,
                    rhs,
                } if *dst == ValueId(13) && *lhs == ValueId(1) && *rhs == ValueId(12)
            )
        })
        .expect("unrelated pure add");
    let helper_idx = block
        .instructions
        .iter()
        .position(|inst| {
            matches!(
                inst,
                MirInstruction::Call {
                    dst: Some(dst),
                    callee: Some(Callee::Extern(name)),
                    args,
                    ..
                } if *dst == ValueId(10)
                    && name == SUBSTRING_CONCAT3_EXTERN
                    && args.as_slice()
                        == [ValueId(5), ValueId(7), ValueId(6), ValueId(8), ValueId(9)]
            )
        })
        .expect("sunk helper call");
    assert!(
        helper_idx > add_idx,
        "helper should sink below unrelated pure work: {:?}",
        block.instructions
    );
    assert_eq!(
        helper_idx + 1,
        block.instructions.len(),
        "helper should end immediately before return: {:?}",
        block.instructions
    );
    assert!(matches!(
        block.terminator,
        Some(MirInstruction::Return {
            value: Some(ValueId(10))
        })
    ));
}
