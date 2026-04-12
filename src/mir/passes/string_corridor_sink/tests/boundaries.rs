use super::*;

#[test]
fn sinks_publication_helper_to_same_block_store_boundary() {
    let mut module = MirModule::new("substring_concat_publication_store".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Integer, MirType::Box("RuntimeDataBox".to_string())],
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
        value: crate::mir::ConstValue::Integer(7),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(14),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(2),
        rhs: ValueId(13),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(15),
        src: ValueId(12),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Store {
        value: ValueId(15),
        ptr: ValueId(0),
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(14)),
    });

    for (vid, ty) in [
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
        (ValueId(15), MirType::Box("RuntimeDataBox".to_string())),
    ] {
        function.metadata.value_types.insert(vid, ty);
    }
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert!(
        rewritten >= 1,
        "publication helper store sink should rewrite, got {rewritten}"
    );

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
    assert!(
        block.instructions.iter().all(|inst| {
            !matches!(
                inst,
                MirInstruction::Copy { dst, .. } if *dst == ValueId(12) || *dst == ValueId(15)
            )
        }),
        "copy-only store chain should disappear: {:?}",
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
                } if *dst == ValueId(14) && *lhs == ValueId(2) && *rhs == ValueId(13)
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
                } if *dst == ValueId(11)
                    && name == SUBSTRING_CONCAT3_EXTERN
                    && args.as_slice()
                        == [ValueId(6), ValueId(8), ValueId(7), ValueId(9), ValueId(10)]
            )
        })
        .expect("sunk helper call");
    let store_idx = block
        .instructions
        .iter()
        .position(|inst| {
            matches!(
                inst,
                MirInstruction::Store { value, ptr } if *value == ValueId(11) && *ptr == ValueId(0)
            )
        })
        .expect("rewritten store");
    assert!(
        helper_idx > add_idx,
        "helper should sink below unrelated pure work: {:?}",
        block.instructions
    );
    assert_eq!(
        helper_idx + 1,
        store_idx,
        "helper should end immediately before store: {:?}",
        block.instructions
    );
}

#[test]
fn sinks_publication_helper_to_same_block_fieldset_boundary() {
    let mut module = MirModule::new("substring_concat_publication_fieldset".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![
            MirType::Box("RuntimeDataBox".to_string()),
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
        value: crate::mir::ConstValue::Integer(7),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(14),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(2),
        rhs: ValueId(13),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(15),
        src: ValueId(12),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::FieldSet {
        base: ValueId(0),
        field: "text".to_string(),
        value: ValueId(15),
        declared_type: Some(MirType::Box("RuntimeDataBox".to_string())),
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(14)),
    });

    for (vid, ty) in [
        (ValueId(0), MirType::Box("RuntimeDataBox".to_string())),
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
        (ValueId(15), MirType::Box("RuntimeDataBox".to_string())),
    ] {
        function.metadata.value_types.insert(vid, ty);
    }
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert!(
        rewritten >= 1,
        "publication helper fieldset sink should rewrite, got {rewritten}"
    );

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
    assert!(
        block.instructions.iter().all(|inst| {
            !matches!(
                inst,
                MirInstruction::Copy { dst, .. } if *dst == ValueId(12) || *dst == ValueId(15)
            )
        }),
        "copy-only fieldset chain should disappear: {:?}",
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
                } if *dst == ValueId(14) && *lhs == ValueId(2) && *rhs == ValueId(13)
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
                } if *dst == ValueId(11)
                    && name == SUBSTRING_CONCAT3_EXTERN
                    && args.as_slice()
                        == [ValueId(6), ValueId(8), ValueId(7), ValueId(9), ValueId(10)]
            )
        })
        .expect("sunk helper call");
    let fieldset_idx = block
        .instructions
        .iter()
        .position(|inst| {
            matches!(
                inst,
                MirInstruction::FieldSet { base, field, value, .. }
                    if *base == ValueId(0) && field == "text" && *value == ValueId(11)
            )
        })
        .expect("rewritten fieldset");
    assert!(
        helper_idx > add_idx,
        "helper should sink below unrelated pure work: {:?}",
        block.instructions
    );
    assert_eq!(
        helper_idx + 1,
        fieldset_idx,
        "helper should end immediately before fieldset: {:?}",
        block.instructions
    );
}

#[test]
fn sinks_publication_helper_to_same_block_runtime_data_set_boundary() {
    let mut module = MirModule::new("substring_concat_publication_runtime_data_set".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![
            MirType::Box("RuntimeDataBox".to_string()),
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
        value: crate::mir::ConstValue::Integer(7),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(14),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId(2),
        rhs: ValueId(13),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(15),
        src: ValueId(12),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(16),
        ValueId(0),
        "RuntimeDataBox",
        "set",
        vec![ValueId(5), ValueId(15)],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(16)),
    });

    for (vid, ty) in [
        (ValueId(0), MirType::Box("RuntimeDataBox".to_string())),
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
        (ValueId(15), MirType::Box("RuntimeDataBox".to_string())),
        (ValueId(16), MirType::Integer),
    ] {
        function.metadata.value_types.insert(vid, ty);
    }
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert!(
        rewritten >= 1,
        "publication helper host-boundary sink should rewrite, got {rewritten}"
    );

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
    assert!(
        block.instructions.iter().all(|inst| {
            !matches!(
                inst,
                MirInstruction::Copy { dst, .. } if *dst == ValueId(12) || *dst == ValueId(15)
            )
        }),
        "copy-only host-boundary chain should disappear: {:?}",
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
                } if *dst == ValueId(14) && *lhs == ValueId(2) && *rhs == ValueId(13)
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
                } if *dst == ValueId(11)
                    && name == SUBSTRING_CONCAT3_EXTERN
                    && args.as_slice()
                        == [ValueId(6), ValueId(8), ValueId(7), ValueId(9), ValueId(10)]
            )
        })
        .expect("sunk helper call");
    let set_idx = block
        .instructions
        .iter()
        .position(|inst| {
            matches!(
                inst,
                MirInstruction::Call {
                    dst: Some(dst),
                    callee: Some(Callee::Method { box_name, method, receiver: Some(receiver), .. }),
                    args,
                    ..
                } if *dst == ValueId(16)
                    && box_name == "RuntimeDataBox"
                    && method == "set"
                    && *receiver == ValueId(0)
                    && args.as_slice() == [ValueId(5), ValueId(11)]
            )
        })
        .expect("rewritten runtime-data set");
    assert!(
        helper_idx > add_idx,
        "helper should sink below unrelated pure work: {:?}",
        block.instructions
    );
    assert_eq!(
        helper_idx + 1,
        set_idx,
        "helper should end immediately before runtime-data set: {:?}",
        block.instructions
    );
}
