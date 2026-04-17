use super::*;

#[test]
fn sinks_materialization_helper_to_array_store_boundary() {
    let mut module = MirModule::new("substring_concat_materialization_store".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![
            MirType::Box("ArrayBox".to_string()),
            MirType::Box("RuntimeDataBox".to_string()),
        ],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

    block.instructions.push(method_call(
        ValueId(2),
        ValueId(1),
        "RuntimeDataBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(3),
        value: crate::mir::ConstValue::Integer(2),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(4),
        op: crate::mir::BinaryOp::Div,
        lhs: ValueId(2),
        rhs: ValueId(3),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(5),
        value: crate::mir::ConstValue::Integer(0),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(6),
        ValueId(1),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(5), ValueId(4)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(7),
        ValueId(1),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(4), ValueId(2)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(8),
        value: crate::mir::ConstValue::String("xx".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(9),
        value: crate::mir::ConstValue::Integer(1),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(10),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(2),
        rhs: ValueId(9),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(extern_call(
        ValueId(11),
        SUBSTRING_CONCAT3_EXTERN,
        vec![ValueId(6), ValueId(8), ValueId(7), ValueId(9), ValueId(10)],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(12),
        src: ValueId(11),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(13),
        value: crate::mir::ConstValue::Integer(99),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(14),
        ValueId(0),
        "ArrayBox",
        "set",
        vec![ValueId(5), ValueId(12)],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(14)),
    });

    for (vid, ty) in [
        (ValueId(0), MirType::Box("ArrayBox".to_string())),
        (ValueId(2), MirType::Integer),
        (ValueId(3), MirType::Integer),
        (ValueId(4), MirType::Integer),
        (ValueId(5), MirType::Integer),
        (ValueId(6), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(7), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(8), MirType::Box("StringBox".to_string())),
        (ValueId(9), MirType::Integer),
        (ValueId(10), MirType::Integer),
        (ValueId(11), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(12), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(13), MirType::Integer),
        (ValueId(14), MirType::Integer),
    ] {
        function.metadata.value_types.insert(vid, ty);
    }
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert!(
        rewritten >= 1,
        "materialization store sink should rewrite, got {rewritten}"
    );

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
    assert!(
        block.instructions.iter().all(|inst| {
            !matches!(inst, MirInstruction::Copy { dst, .. } if *dst == ValueId(12))
        }),
        "copy-only store consumer should disappear: {:?}",
        block.instructions
    );

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
                } if *dst == ValueId(11)
                    && name == SUBSTRING_CONCAT3_EXTERN
                    && args.as_slice()
                        == [ValueId(6), ValueId(8), ValueId(7), ValueId(9), ValueId(10)]
            )
        })
        .expect("materialization helper call");
    let const_idx = block
        .instructions
        .iter()
        .position(|inst| {
            matches!(
                inst,
                MirInstruction::Const {
                    dst,
                    value: crate::mir::ConstValue::Integer(99),
                } if *dst == ValueId(13)
            )
        })
        .expect("unrelated const");
    let store_idx = block
        .instructions
        .iter()
        .position(|inst| {
            matches!(
                inst,
                MirInstruction::Call {
                    dst: Some(dst),
                    callee: Some(Callee::Method { method, receiver: Some(receiver), .. }),
                    args,
                    ..
                } if *dst == ValueId(14)
                    && method == "set"
                    && *receiver == ValueId(0)
                    && args.as_slice() == [ValueId(5), ValueId(11)]
            )
        })
        .expect("array store");
    assert!(
        helper_idx > const_idx,
        "helper should sink past unrelated pure instructions: {:?}",
        block.instructions
    );
    assert_eq!(
        helper_idx + 1,
        store_idx,
        "materialization helper should sit right before the store boundary: {:?}",
        block.instructions
    );
}

#[test]
fn reuses_store_side_const_suffix_for_trailing_substring() {
    let mut module = MirModule::new("store_shared_receiver_substring".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![
            MirType::Box("ArrayBox".to_string()),
            MirType::Box("RuntimeDataBox".to_string()),
        ],
        return_type: MirType::Box("RuntimeDataBox".to_string()),
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

    block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: crate::mir::ConstValue::Integer(0),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(3),
        value: crate::mir::ConstValue::String("xy".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(4),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(1),
        rhs: ValueId(3),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(5),
        ValueId(0),
        "ArrayBox",
        "set",
        vec![ValueId(2), ValueId(4)],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(6),
        src: ValueId(1),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(7),
        value: crate::mir::ConstValue::String("xy".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(8),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(6),
        rhs: ValueId(7),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(9),
        value: crate::mir::ConstValue::Integer(1),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(10),
        value: crate::mir::ConstValue::Integer(3),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(11),
        ValueId(8),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(9), ValueId(10)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(11)),
    });

    for (vid, ty) in [
        (ValueId(0), MirType::Box("ArrayBox".to_string())),
        (ValueId(1), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(2), MirType::Integer),
        (ValueId(3), MirType::Box("StringBox".to_string())),
        (ValueId(4), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(5), MirType::Integer),
        (ValueId(6), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(7), MirType::Box("StringBox".to_string())),
        (ValueId(8), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(9), MirType::Integer),
        (ValueId(10), MirType::Integer),
        (ValueId(11), MirType::Box("RuntimeDataBox".to_string())),
    ] {
        function.metadata.value_types.insert(vid, ty);
    }
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert!(
        rewritten >= 1,
        "store/shared receiver rewrite should trigger, got {rewritten}"
    );

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
    assert!(
        block.instructions.iter().all(|inst| {
            !matches!(
                inst,
                MirInstruction::Copy { dst, .. } if *dst == ValueId(6)
            ) && !matches!(
                inst,
                MirInstruction::Const { dst, value: crate::mir::ConstValue::String(text) }
                    if *dst == ValueId(7) && text == "xy"
            ) && !matches!(
                inst,
                MirInstruction::BinOp { dst, op: crate::mir::BinaryOp::Add, .. } if *dst == ValueId(8)
            )
        }),
        "duplicate const-suffix producer should disappear: {:?}",
        block.instructions
    );
    assert!(
        block.instructions.iter().any(|inst| {
            matches!(
                inst,
                MirInstruction::Call {
                    dst: Some(dst),
                    callee:
                        Some(Callee::Method {
                            method,
                            receiver: Some(receiver),
                            ..
                        }),
                    args,
                    ..
                } if *dst == ValueId(11)
                    && method == "substring"
                    && *receiver == ValueId(4)
                    && args.as_slice() == [ValueId(9), ValueId(10)]
            )
        }),
        "substring should reuse the store-side producer: {:?}",
        block.instructions
    );
}

#[test]
fn sinks_materialization_helper_with_trailing_length_observer() {
    let mut module =
        MirModule::new("substring_concat_materialization_store_len_observer".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![
            MirType::Box("ArrayBox".to_string()),
            MirType::Box("RuntimeDataBox".to_string()),
        ],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

    block.instructions.push(method_call(
        ValueId(2),
        ValueId(1),
        "RuntimeDataBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(3),
        value: crate::mir::ConstValue::Integer(2),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(4),
        op: crate::mir::BinaryOp::Div,
        lhs: ValueId(2),
        rhs: ValueId(3),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(5),
        value: crate::mir::ConstValue::Integer(0),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(6),
        ValueId(1),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(5), ValueId(4)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(7),
        ValueId(1),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(4), ValueId(2)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(8),
        value: crate::mir::ConstValue::String("xx".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(9),
        value: crate::mir::ConstValue::Integer(1),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(10),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(2),
        rhs: ValueId(9),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(extern_call(
        ValueId(11),
        SUBSTRING_CONCAT3_EXTERN,
        vec![ValueId(6), ValueId(8), ValueId(7), ValueId(9), ValueId(10)],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(12),
        src: ValueId(11),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(13),
        ValueId(0),
        "ArrayBox",
        "set",
        vec![ValueId(5), ValueId(12)],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(14),
        src: ValueId(11),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(15),
        ValueId(14),
        "RuntimeDataBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(16),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(13),
        rhs: ValueId(15),
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(16)),
    });

    for (vid, ty) in [
        (ValueId(0), MirType::Box("ArrayBox".to_string())),
        (ValueId(2), MirType::Integer),
        (ValueId(3), MirType::Integer),
        (ValueId(4), MirType::Integer),
        (ValueId(5), MirType::Integer),
        (ValueId(6), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(7), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(8), MirType::Box("StringBox".to_string())),
        (ValueId(9), MirType::Integer),
        (ValueId(10), MirType::Integer),
        (ValueId(11), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(12), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(13), MirType::Integer),
        (ValueId(14), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(15), MirType::Integer),
        (ValueId(16), MirType::Integer),
    ] {
        function.metadata.value_types.insert(vid, ty);
    }
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert!(
        rewritten >= 2,
        "materialization store plus observer length should rewrite, got {rewritten}"
    );

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
    assert!(
        block.instructions.iter().all(|inst| {
            !matches!(
                inst,
                MirInstruction::Copy { dst, .. } if *dst == ValueId(12) || *dst == ValueId(14)
            )
        }),
        "copy-only store/observer chains should disappear: {:?}",
        block.instructions
    );
    assert!(
        block.instructions.iter().any(|inst| {
            matches!(
                inst,
                MirInstruction::BinOp {
                    dst,
                    op: crate::mir::BinaryOp::Sub,
                    lhs,
                    rhs,
                } if *dst == ValueId(15) && *lhs == ValueId(10) && *rhs == ValueId(9)
            )
        }),
        "trailing helper length should rewrite to end-start: {:?}",
        block.instructions
    );

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
                } if *dst == ValueId(11)
                    && name == SUBSTRING_CONCAT3_EXTERN
                    && args.as_slice()
                        == [ValueId(6), ValueId(8), ValueId(7), ValueId(9), ValueId(10)]
            )
        })
        .expect("materialization helper call");
    let store_idx = block
        .instructions
        .iter()
        .position(|inst| {
            matches!(
                inst,
                MirInstruction::Call {
                    dst: Some(dst),
                    callee: Some(Callee::Method { method, receiver: Some(receiver), .. }),
                    args,
                    ..
                } if *dst == ValueId(13)
                    && method == "set"
                    && *receiver == ValueId(0)
                    && args.as_slice() == [ValueId(5), ValueId(11)]
            )
        })
        .expect("array store");
    let len_idx = block
        .instructions
        .iter()
        .position(|inst| {
            matches!(
                inst,
                MirInstruction::BinOp {
                    dst,
                    op: crate::mir::BinaryOp::Sub,
                    ..
                } if *dst == ValueId(15)
            )
        })
        .expect("observer len rewrite");
    assert_eq!(
        helper_idx + 1,
        store_idx,
        "materialization helper should sit right before the store boundary: {:?}",
        block.instructions
    );
    assert!(
        len_idx > store_idx,
        "trailing len observer should stay after the store boundary: {:?}",
        block.instructions
    );
}
