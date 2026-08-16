//! Caller-zero allocation of source-backed common-V2 layout segments.
//!
//! This module allocates only the source segment blocks.  It deliberately has
//! no edge, terminator, operation, or synthetic After API.  The surrounding
//! function transaction remains the sole rollback owner.

use crate::mir::loop_recipe_contract::{
    PreparedLoopV2SegmentAllocationPlanV1, SegmentAllocationPlanRejectV1,
};
use crate::mir::{BasicBlockId, MirBuilder};

use super::canonical_ssa::CanonicalSsaFunctionSessionV2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) enum SegmentBlockAllocationRejectV1 {
    Plan(SegmentAllocationPlanRejectV1),
    OwnerMismatch,
    MissingFunction,
    CountOverflow,
    CursorRange,
    ExistingBlock(BasicBlockId),
    Allocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) struct SegmentBlockReceiptRowV1 {
    loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    logical_block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    split_ordinal: u32,
    physical_block: BasicBlockId,
}

impl SegmentBlockReceiptRowV1 {
    pub(in crate::mir::builder::resolved_lowering) const fn physical_block(self) -> BasicBlockId {
        self.physical_block
    }
}

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) struct PreparedSegmentBlockReceiptV1 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    rows: Box<[SegmentBlockReceiptRowV1]>,
}

impl PreparedSegmentBlockReceiptV1 {
    pub(in crate::mir::builder::resolved_lowering) const fn owner(
        &self,
    ) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder::resolved_lowering) fn rows(&self) -> &[SegmentBlockReceiptRowV1] {
        &self.rows
    }
}

pub(in crate::mir::builder::resolved_lowering) fn allocate_v2_segment_blocks(
    session: &mut CanonicalSsaFunctionSessionV2<'_>,
    builder: &mut MirBuilder,
    plan: PreparedLoopV2SegmentAllocationPlanV1<'_, '_>,
) -> Result<PreparedSegmentBlockReceiptV1, SegmentBlockAllocationRejectV1> {
    let owner = plan.owner();
    if owner != session.owner() {
        return Err(SegmentBlockAllocationRejectV1::OwnerMismatch);
    }
    let segments = plan.segments();
    let count =
        u32::try_from(segments.len()).map_err(|_| SegmentBlockAllocationRejectV1::CountOverflow)?;
    if builder.function_state.current_function.is_none() {
        return Err(SegmentBlockAllocationRejectV1::MissingFunction);
    }

    let next = builder.core_ctx.peek_next_block().as_u32();
    let end = next
        .checked_add(count)
        .filter(|end| *end < u32::MAX)
        .ok_or(SegmentBlockAllocationRejectV1::CursorRange)?;
    let _ = end;

    {
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .expect("current function checked above");
        for offset in 0..count {
            let candidate = BasicBlockId::new(next + offset);
            if function.get_block(candidate).is_some() {
                return Err(SegmentBlockAllocationRejectV1::ExistingBlock(candidate));
            }
        }
    }

    let mut rows = Vec::with_capacity(segments.len());
    for (offset, segment) in segments.iter().enumerate() {
        let physical_block = session
            .create_unpublished_block(builder)
            .map_err(|_| SegmentBlockAllocationRejectV1::Allocation)?;
        rows.push(SegmentBlockReceiptRowV1 {
            loop_key: segment.loop_key(),
            logical_block: segment.block(),
            split_ordinal: segment.split_ordinal(),
            physical_block,
        });
        debug_assert_eq!(physical_block, BasicBlockId::new(next + offset as u32));
    }

    Ok(PreparedSegmentBlockReceiptV1 {
        owner,
        rows: rows.into_boxed_slice(),
    })
}
