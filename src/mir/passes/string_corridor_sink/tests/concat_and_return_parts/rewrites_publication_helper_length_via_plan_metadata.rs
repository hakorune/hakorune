use super::super::*;
use super::shared;

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

    block.instructions.push(shared::method_call(
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
    block.instructions.push(shared::extern_call(
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
    block.instructions.push(shared::method_call(
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
