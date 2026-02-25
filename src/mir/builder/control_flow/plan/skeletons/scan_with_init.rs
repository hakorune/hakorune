//! ScanWithInit skeleton allocation (blocks/slots only, no AST analysis).

use crate::mir::builder::control_flow::plan::{ScanDirection, ScanWithInitPlan};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirType, ValueId};

pub(in crate::mir::builder) struct ScanWithInitSkeleton {
    pub preheader_bb: BasicBlockId,
    pub header_bb: BasicBlockId,
    pub body_bb: BasicBlockId,
    pub step_bb: BasicBlockId,
    pub after_bb: BasicBlockId,
    pub found_bb: BasicBlockId,
    pub i_current: ValueId,
    pub i_next: ValueId,
    pub one_val: ValueId,
    pub needle_len_val: ValueId,
    pub len_val: ValueId,
    pub bound_val: ValueId,
    pub cond_loop: ValueId,
    pub i_plus_needle_len: ValueId,
    pub window_val: ValueId,
    pub cond_match: ValueId,
    pub zero_val: Option<ValueId>,
}

pub(in crate::mir::builder) fn alloc_scan_with_init_skeleton(
    builder: &mut MirBuilder,
    parts: &ScanWithInitPlan,
) -> Result<ScanWithInitSkeleton, String> {
    let preheader_bb = builder
        .current_block
        .ok_or_else(|| "[normalizer] No current block for loop entry".to_string())?;

    let header_bb = builder.next_block_id();
    let body_bb = builder.next_block_id();
    let step_bb = builder.next_block_id();
    let after_bb = builder.next_block_id();
    let found_bb = builder.next_block_id();

    let i_current = builder.next_value_id();
    builder.type_ctx.set_type(i_current, MirType::Integer);

    let one_val = builder.next_value_id();
    builder.type_ctx.set_type(one_val, MirType::Integer);

    let needle_len_val = if parts.dynamic_needle {
        let v = builder.next_value_id();
        builder.type_ctx.set_type(v, MirType::Integer);
        v
    } else {
        one_val
    };

    let len_val = builder.next_value_id();
    builder.type_ctx.set_type(len_val, MirType::Integer);

    let bound_val = builder.next_value_id();
    builder.type_ctx.set_type(bound_val, MirType::Integer);

    let cond_loop = builder.next_value_id();
    builder.type_ctx.set_type(cond_loop, MirType::Bool);

    let i_plus_needle_len = builder.next_value_id();
    builder
        .type_ctx
        .set_type(i_plus_needle_len, MirType::Integer);

    let window_val = builder.next_value_id();
    builder.type_ctx.set_type(window_val, MirType::String);

    let cond_match = builder.next_value_id();
    builder.type_ctx.set_type(cond_match, MirType::Bool);

    let i_next = builder.next_value_id();
    builder.type_ctx.set_type(i_next, MirType::Integer);

    let zero_val = if matches!(parts.scan_direction, ScanDirection::Reverse) {
        let v = builder.next_value_id();
        builder.type_ctx.set_type(v, MirType::Integer);
        Some(v)
    } else {
        None
    };

    Ok(ScanWithInitSkeleton {
        preheader_bb,
        header_bb,
        body_bb,
        step_bb,
        after_bb,
        found_bb,
        i_current,
        i_next,
        one_val,
        needle_len_val,
        len_val,
        bound_val,
        cond_loop,
        i_plus_needle_len,
        window_val,
        cond_match,
        zero_val,
    })
}
