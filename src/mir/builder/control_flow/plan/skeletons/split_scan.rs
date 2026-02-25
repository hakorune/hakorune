//! SplitScan skeleton allocation (blocks/slots only, no AST analysis).

use crate::mir::builder::control_flow::plan::SplitScanPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirType, ValueId};

pub(in crate::mir::builder) struct SplitScanSkeleton {
    pub preheader_bb: BasicBlockId,
    pub header_bb: BasicBlockId,
    pub body_bb: BasicBlockId,
    pub then_bb: BasicBlockId,
    pub else_bb: BasicBlockId,
    pub step_bb: BasicBlockId,
    pub after_bb: BasicBlockId,
    pub i_current: ValueId,
    pub start_current: ValueId,
    pub i_next: ValueId,
    pub start_next: ValueId,
    pub sep_len: ValueId,
    pub s_len: ValueId,
    pub limit: ValueId,
    pub cond_loop: ValueId,
    pub i_plus_sep: ValueId,
    pub chunk: ValueId,
    pub cond_match: ValueId,
    pub segment: ValueId,
    pub start_next_then: ValueId,
    pub one: ValueId,
    pub i_next_else: ValueId,
}

pub(in crate::mir::builder) fn alloc_split_scan_skeleton(
    builder: &mut MirBuilder,
    _parts: &SplitScanPlan,
) -> Result<SplitScanSkeleton, String> {
    let preheader_bb = builder
        .current_block
        .ok_or_else(|| "[normalizer] No current block for loop entry".to_string())?;

    let header_bb = builder.next_block_id();
    let body_bb = builder.next_block_id();
    let then_bb = builder.next_block_id();
    let else_bb = builder.next_block_id();
    let step_bb = builder.next_block_id();
    let after_bb = builder.next_block_id();

    let i_current = builder.next_value_id();
    builder.type_ctx.set_type(i_current, MirType::Integer);

    let start_current = builder.next_value_id();
    builder.type_ctx.set_type(start_current, MirType::Integer);

    let i_next = builder.next_value_id();
    builder.type_ctx.set_type(i_next, MirType::Integer);

    let start_next = builder.next_value_id();
    builder.type_ctx.set_type(start_next, MirType::Integer);

    let sep_len = builder.next_value_id();
    builder.type_ctx.set_type(sep_len, MirType::Integer);

    let s_len = builder.next_value_id();
    builder.type_ctx.set_type(s_len, MirType::Integer);

    let limit = builder.next_value_id();
    builder.type_ctx.set_type(limit, MirType::Integer);

    let cond_loop = builder.next_value_id();
    builder.type_ctx.set_type(cond_loop, MirType::Bool);

    let i_plus_sep = builder.next_value_id();
    builder.type_ctx.set_type(i_plus_sep, MirType::Integer);

    let chunk = builder.next_value_id();
    builder.type_ctx.set_type(chunk, MirType::String);

    let cond_match = builder.next_value_id();
    builder.type_ctx.set_type(cond_match, MirType::Bool);

    let segment = builder.next_value_id();
    builder.type_ctx.set_type(segment, MirType::String);

    let start_next_then = builder.next_value_id();
    builder.type_ctx.set_type(start_next_then, MirType::Integer);

    let one = builder.next_value_id();
    builder.type_ctx.set_type(one, MirType::Integer);

    let i_next_else = builder.next_value_id();
    builder.type_ctx.set_type(i_next_else, MirType::Integer);

    Ok(SplitScanSkeleton {
        preheader_bb,
        header_bb,
        body_bb,
        then_bb,
        else_bb,
        step_bb,
        after_bb,
        i_current,
        start_current,
        i_next,
        start_next,
        sep_len,
        s_len,
        limit,
        cond_loop,
        i_plus_sep,
        chunk,
        cond_match,
        segment,
        start_next_then,
        one,
        i_next_else,
    })
}
