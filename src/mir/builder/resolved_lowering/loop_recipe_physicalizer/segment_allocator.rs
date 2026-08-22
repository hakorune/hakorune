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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
    use crate::mir::builder::MirBuilder;
    use crate::mir::compiler::callable_single_loop_operation_effect::callable_operation_demand_parts_for_test;
    use crate::mir::loop_recipe_contract::VerifiedLoopOperationPhysicalDemandV1;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
    use crate::mir::BasicBlockId;

    fn callable_layout() -> PreparedLoopPhysicalLayoutV1 {
        let (effect, context, continuation) = callable_operation_demand_parts_for_test();
        VerifiedLoopOperationPhysicalDemandV1::issue(context, effect, continuation)
            .expect("callable demand")
            .prepare_all()
            .expect("callable program")
            .prepare_physical_layout()
            .expect("callable layout")
    }

    #[test]
    fn allocator_rejects_foreign_entry_owner_before_block_allocation() {
        let layout = callable_layout();
        let mut owners = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        let foreign_owner = owners.issue().expect("foreign owner");
        let owner = layout.program().demand().context().owner();
        assert_ne!(owner, foreign_owner);

        let entry = ReadyLoopEntryV1::new_for_test(foreign_owner, BasicBlockId::new(0), Vec::new());
        let mut builder = MirBuilder::new();
        let mut cfg = CanonicalCfgSessionV1::new();
        let mut services = LoopPhysicalServicesV1::new(&mut builder, &mut cfg);

        assert_eq!(
            allocate_for_layout(&layout, &entry, &mut services),
            Err(LoopPhysicalSegmentAllocatorRejectV1::EntryOwnerMismatch)
        );
    }
}
