use crate::mir::{BasicBlockId, BinaryOp, MirFunction, MirInstruction, ValueId};

/// Parse a binary operator string to BinaryOp
pub fn parse_binop_str(op: &str) -> Option<BinaryOp> {
    match op {
        "+" => Some(BinaryOp::Add),
        "-" => Some(BinaryOp::Sub),
        "*" => Some(BinaryOp::Mul),
        "/" => Some(BinaryOp::Div),
        "%" => Some(BinaryOp::Mod),
        "&" => Some(BinaryOp::BitAnd),
        "|" => Some(BinaryOp::BitOr),
        "^" => Some(BinaryOp::BitXor),
        "<<" => Some(BinaryOp::Shl),
        ">>" => Some(BinaryOp::Shr),
        _ => None,
    }
}

/// Emit a MIR BinOp into the current block and return the destination ValueId
pub fn emit_binop_func(
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    op: BinaryOp,
    lhs: ValueId,
    rhs: ValueId,
) -> ValueId {
    let dst = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur_bb) {
        bb.add_instruction(MirInstruction::BinOp { dst, op, lhs, rhs });
    }
    dst
}

/// Emit a MIR BinOp into the current block using the provided destination id.
/// This variant allows front-ends that pre-allocate `dst` (e.g., builders that
/// maintain their own value id generator) to route through the SSOT without
/// changing id allocation policy.
pub fn emit_binop_to_dst(
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    dst: ValueId,
    op: BinaryOp,
    lhs: ValueId,
    rhs: ValueId,
) {
    if let Some(bb) = f.get_block_mut(cur_bb) {
        bb.add_instruction(MirInstruction::BinOp { dst, op, lhs, rhs });
    }
}
