use crate::ast::Span;
use crate::mir::{
    BasicBlockId, CompareOp, MirFunction, MirInstruction, SpannedInstruction, ValueId,
};

/// Emit a MIR Compare instruction into the current block (function-level SSOT helper)
pub fn emit_compare_func(
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    dst: ValueId,
    op: CompareOp,
    lhs: ValueId,
    rhs: ValueId,
) {
    if let Some(bb) = f.get_block_mut(cur_bb) {
        bb.add_instruction(MirInstruction::Compare { dst, op, lhs, rhs });
    }
}

/// Set a conditional branch terminator on the current block and register predecessors.
pub fn set_branch(
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    condition: ValueId,
    then_bb: BasicBlockId,
    else_bb: BasicBlockId,
) {
    if let Some(bb) = f.get_block_mut(cur_bb) {
        bb.set_terminator(MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            then_edge_args: None,
            else_edge_args: None,
        });
    }
    if let Some(tb) = f.get_block_mut(then_bb) {
        tb.add_predecessor(cur_bb);
    }
    if let Some(eb) = f.get_block_mut(else_bb) {
        eb.add_predecessor(cur_bb);
    }
}

/// Set an unconditional jump terminator and register predecessor on target block.
pub fn set_jump(f: &mut MirFunction, cur_bb: BasicBlockId, target: BasicBlockId) {
    if let Some(bb) = f.get_block_mut(cur_bb) {
        bb.set_terminator(MirInstruction::Jump {
            target,
            edge_args: None,
        });
    }
    if let Some(tb) = f.get_block_mut(target) {
        tb.add_predecessor(cur_bb);
    }
}

/// Insert a PHI instruction at block head (after existing PHIs) with normalized inputs order.
pub fn insert_phi_at_head(
    f: &mut MirFunction,
    bb_id: BasicBlockId,
    dst: ValueId,
    inputs: Vec<(BasicBlockId, ValueId)>,
) -> Result<(), String> {
    insert_phi_at_head_spanned(f, bb_id, dst, inputs, Span::unknown())
}

/// Insert a PHI instruction at block head (after existing PHIs) with normalized inputs order.
/// Allows passing a span to retain source mapping when available.
pub fn insert_phi_at_head_spanned(
    f: &mut MirFunction,
    bb_id: BasicBlockId,
    dst: ValueId,
    mut inputs: Vec<(BasicBlockId, ValueId)>,
    span: Span,
) -> Result<(), String> {
    inputs.sort_by_key(|(bb, _)| bb.0);
    let fn_name = f.signature.name.clone();
    let bb = f.get_block_mut(bb_id).ok_or_else(|| {
        format!(
            "[freeze:contract][cf_common/phi_block_missing] fn={} bb_id={:?} dst={:?}",
            fn_name, bb_id, dst
        )
    })?;
    bb.insert_spanned_after_phis(SpannedInstruction {
        inst: MirInstruction::Phi {
            dst,
            inputs,
            type_hint: None,
        },
        span,
    });
    Ok(())
}
