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
use crate::mir::loop_recipe_contract::{LoopPhysicalSegmentRoleV1, PreparedLoopPhysicalLayoutV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) enum LoopPhysicalSegmentAllocatorRejectV1 {
    EntryOwnerMismatch,
    Block(LoopPhysicalizerRejectV1),
    Receipt(LoopPhysicalSegmentBlockReceiptRejectV1),
}

pub(in crate::mir::builder::resolved_lowering) fn allocate_for_layout(
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
            match segment.role() {
                LoopPhysicalSegmentRoleV1::Header => LoopPhysicalBlockRoleV1::Header,
                LoopPhysicalSegmentRoleV1::Body => LoopPhysicalBlockRoleV1::Body,
            },
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
