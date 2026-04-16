use super::super::*;
use super::shared;

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

    block.instructions.push(shared::method_call(
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
    block.instructions.push(shared::method_call(
        ValueId(5),
        ValueId(0),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(4), ValueId(3)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(shared::method_call(
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
    block.instructions.push(shared::method_call(
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
    block.instructions.push(shared::method_call(
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
                    callee: Some(Callee::Extern(name)),
                    args,
                    ..
                } if name == INSERT_HSI_EXTERN
                    && args.as_slice() == [ValueId(0), ValueId(7), ValueId(3)]
            )
        }),
        "concat substring should delete producer substrings via insert_hsi: {:?}",
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
                    && name == "nyash.string.substring_hii"
                    && args.len() == 3
                    && args[1] == ValueId(11)
                    && args[2] == ValueId(12)
            )
        }),
        "outer substring should remain as one direct substring_hii on insert result: {:?}",
        block.instructions
    );
    let substring_calls: Vec<_> = block
        .instructions
        .iter()
        .filter(|inst| match inst {
            MirInstruction::Call {
                callee: Some(Callee::Method { method, .. }),
                args,
                ..
            } => args.len() == 2 && matches!(method.as_str(), "substring" | "slice"),
            MirInstruction::Call {
                callee: Some(Callee::Extern(name)),
                args,
                ..
            } => args.len() == 3 && name == "nyash.string.substring_hii",
            _ => false,
        })
        .collect();
    assert_eq!(
        substring_calls.len(),
        1,
        "delete-oriented rewrite should retire producer substring calls: {:?}",
        block.instructions
    );
}
