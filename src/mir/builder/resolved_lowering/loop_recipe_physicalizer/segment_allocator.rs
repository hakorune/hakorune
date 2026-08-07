//! R3 exact segment-block allocator.
//!
//! Allocation is deliberately separate from the segment receipt validator:
//! this module performs the only Builder-affecting block allocation, while
//! `segment_topology` owns the immutable receipt and its exact-coverage rules.

use super::segment_topology::{
    LoopPhysicalSegmentBlockReceiptRejectV1, LoopPhysicalSegmentBlockReceiptV1,
    LoopPhysicalSegmentBlockRowV1,
};
use super::topology::{
    LoopPhysicalBlockRoleV1, LoopPhysicalServicesV1, LoopPhysicalizerRejectV1, ReadyLoopEntryV1,
};
use crate::mir::loop_recipe_contract::{LoopConditionV1, PreparedLoopPhysicalLayoutV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LoopPhysicalSegmentAllocatorRejectV1 {
    EntryOwnerMismatch,
    Block(LoopPhysicalizerRejectV1),
    Receipt(LoopPhysicalSegmentBlockReceiptRejectV1),
}

pub(super) fn allocate_for_layout(
    layout: &PreparedLoopPhysicalLayoutV1,
    entry: &ReadyLoopEntryV1,
    services: &mut LoopPhysicalServicesV1<'_>,
) -> Result<LoopPhysicalSegmentBlockReceiptV1, LoopPhysicalSegmentAllocatorRejectV1> {
    let owner = layout.program().demand().context().owner();
    if entry.owner() != owner {
        return Err(LoopPhysicalSegmentAllocatorRejectV1::EntryOwnerMismatch);
    }

    let mut rows = Vec::with_capacity(layout.segments().len());
    for segment in layout.segments() {
        let physical_block = services
            .allocate_block()
            .map_err(LoopPhysicalSegmentAllocatorRejectV1::Block)?;
        rows.push(LoopPhysicalSegmentBlockRowV1::new(
            segment.key(),
            segment_role(layout, segment.key()),
            physical_block,
        ));
    }

    let root_after = services
        .allocate_block()
        .map_err(LoopPhysicalSegmentAllocatorRejectV1::Block)?;
    let expected = layout
        .segments()
        .iter()
        .map(|segment| segment.key())
        .collect::<Vec<_>>();
    LoopPhysicalSegmentBlockReceiptV1::issue_with_boundary(
        owner,
        entry.preheader(),
        layout.entry_segment(),
        root_after,
        &expected,
        rows,
    )
    .map_err(LoopPhysicalSegmentAllocatorRejectV1::Receipt)
}

fn segment_role(
    layout: &PreparedLoopPhysicalLayoutV1,
    segment: crate::mir::loop_recipe_contract::LoopPhysicalSegmentKeyV1,
) -> LoopPhysicalBlockRoleV1 {
    let is_condition = layout
        .program()
        .demand()
        .operation_effect()
        .core()
        .recipe()
        .as_recipe()
        .loops
        .iter()
        .any(|loop_row| {
            loop_row.key == segment.loop_key()
                && matches!(
                    loop_row.condition,
                    LoopConditionV1::Predicate { block, .. } if block == segment.block()
                )
        });
    if is_condition {
        LoopPhysicalBlockRoleV1::Header
    } else {
        LoopPhysicalBlockRoleV1::Body
    }
}
