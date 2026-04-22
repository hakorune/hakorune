use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, CompareOp, ConstValue, EffectMask, FunctionSignature,
    MirFunction, MirInstruction, MirType, ValueId,
};

mod array_routes;
mod debug;
mod decl_values;
mod function_attrs;
mod generic_method_routes;
mod ordering;
mod placement;
mod string_corridor;
mod string_direct_set_routes;
mod thin_entry;

fn make_function(name: &str, is_entry_point: bool) -> MirFunction {
    let signature = FunctionSignature {
        name: name.to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    function.metadata.is_entry_point = is_entry_point;
    function
}

fn make_string_loop_function() -> MirFunction {
    let mut function = make_function("main", true);
    let entry = BasicBlockId::new(0);
    let header = BasicBlockId::new(18);
    let body = BasicBlockId::new(19);
    let exit = BasicBlockId::new(21);

    function
        .blocks
        .get_mut(&entry)
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Const {
                dst: ValueId::new(3),
                value: ConstValue::String("line-seed-abcdef".to_string()),
            },
            MirInstruction::Copy {
                dst: ValueId::new(4),
                src: ValueId::new(3),
            },
            MirInstruction::Const {
                dst: ValueId::new(5),
                value: ConstValue::Integer(16),
            },
        ]);

    let mut header_block = BasicBlock::new(header);
    header_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(15),
            inputs: vec![(entry, ValueId::new(12)), (body, ValueId::new(16))],
            type_hint: Some(MirType::Integer),
        },
        MirInstruction::Phi {
            dst: ValueId::new(21),
            inputs: vec![(entry, ValueId::new(4)), (body, ValueId::new(36))],
            type_hint: Some(MirType::String),
        },
        MirInstruction::Const {
            dst: ValueId::new(41),
            value: ConstValue::Integer(300000),
        },
        MirInstruction::Compare {
            dst: ValueId::new(37),
            op: CompareOp::Lt,
            lhs: ValueId::new(15),
            rhs: ValueId::new(41),
        },
        MirInstruction::Branch {
            condition: ValueId::new(37),
            then_bb: body,
            else_bb: exit,
            then_edge_args: None,
            else_edge_args: None,
        },
    ]);
    function.blocks.insert(header, header_block);

    let mut body_block = BasicBlock::new(body);
    body_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(50),
            value: ConstValue::Integer(2),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(47),
            op: BinaryOp::Div,
            lhs: ValueId::new(5),
            rhs: ValueId::new(50),
        },
        MirInstruction::Const {
            dst: ValueId::new(66),
            value: ConstValue::String("xx".to_string()),
        },
        MirInstruction::Copy {
            dst: ValueId::new(36),
            src: ValueId::new(21),
        },
    ]);
    function.blocks.insert(body, body_block);
    function.blocks.insert(exit, BasicBlock::new(exit));
    function
}
