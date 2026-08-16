//! Source-backed, physical-ID-free segment allocation demand for common V2.
//!
//! This plan deliberately stops before synthetic After allocation.  The
//! existing layout segments are a complete source-owned set; an After block
//! needs a separate source-backed boundary and must not be inferred from the
//! JoinSig After port alone.

use super::common_v2_issuers::PreparedLoopV2PreSessionEnvelopeV1;
use super::common_v2_layout_input::{
    PreparedLoopV2LayoutSegmentRefV1, PreparedLoopV2PhysicalLayoutInputV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SegmentAllocationPlanRejectV1 {
    ForeignOwner,
    MissingSegments,
}

/// One callback-scoped allocation demand.  It borrows the exact envelope
/// layout instead of copying a second topology or issuing a new key set.
#[derive(Debug)]
pub(crate) struct PreparedLoopV2SegmentAllocationPlanV1<'layout, 'source> {
    owner: FunctionOwnerIdV1,
    layout: &'layout PreparedLoopV2PhysicalLayoutInputV1<'source>,
}

impl<'layout, 'source> PreparedLoopV2SegmentAllocationPlanV1<'layout, 'source> {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn segments(&self) -> &'layout [PreparedLoopV2LayoutSegmentRefV1<'source>] {
        self.layout.segments()
    }

    pub(crate) fn segment_count(&self) -> usize {
        self.layout.segment_count()
    }
}

pub(crate) fn issue_v2_segment_allocation_plan<'layout, 'source, 'join>(
    envelope: &'layout PreparedLoopV2PreSessionEnvelopeV1<'source, 'join>,
) -> Result<PreparedLoopV2SegmentAllocationPlanV1<'layout, 'source>, SegmentAllocationPlanRejectV1>
{
    let layout = envelope.layout();
    if layout.owner() != envelope.owner() {
        return Err(SegmentAllocationPlanRejectV1::ForeignOwner);
    }
    if layout.segments().is_empty() {
        return Err(SegmentAllocationPlanRejectV1::MissingSegments);
    }
    Ok(PreparedLoopV2SegmentAllocationPlanV1 {
        owner: envelope.owner(),
        layout,
    })
}
