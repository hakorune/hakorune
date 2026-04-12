use super::super::*;
use super::shared;

#[test]
fn publication_helper_shape_reads_folded_route_proof_without_legacy_candidates() {
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
    ] {
        function.metadata.value_types.insert(vid, ty);
    }

    crate::mir::refresh_function_string_corridor_metadata(&mut function);
    crate::mir::refresh_function_placement_effect_routes(&mut function);
    function.metadata.string_corridor_candidates.clear();

    let def_map = build_value_def_map(&function);
    let helper = publication_helper_shape(&function, &def_map, ValueId(11))
        .expect("publication helper shape from folded placement_effect route");
    assert_eq!(helper.dst, ValueId(10));
    assert_eq!(helper.start, ValueId(8));
    assert_eq!(helper.end, ValueId(9));
}
