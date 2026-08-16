//! Caller-zero allocation of the one synthetic common-V2 After block.
//!
//! The source-backed After relation and the source-segment block receipt are
//! the only inputs.  This module never chooses a successor or emits an edge;
//! it only issues one unpublished block through the canonical CFG owner.

use std::marker::PhantomData;

use crate::mir::loop_recipe_contract::{
    LoopV2AfterBoundaryRelationV1, PreparedLoopV2PreSessionEnvelopeV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, LoopExecutionFrameKeyV1};
use crate::mir::{BasicBlockId, MirBuilder};

use super::canonical_ssa::CanonicalSsaFunctionSessionV2;
use super::common_v2_segment_block_allocation::PreparedSegmentBlockReceiptV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) enum AfterBlockAllocationRejectV1 {
    AlreadyPrepared,
    AlreadyAllocated,
    OwnerMismatch,
    MissingRootAfter,
    RelationMismatch,
    SegmentCoverageMismatch,
    MissingFunction,
    CursorRange,
    ExistingBlock(BasicBlockId),
    Allocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) enum AfterBlockAllocationStateV1 {
    Available,
    Prepared,
    Allocated,
}

/// A one-shot, physical-ID-free allocation demand.  The constructor is
/// private to the same-session issuer below; callers cannot manufacture a
/// plan from a raw After key or block id.
#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) struct PreparedLoopV2AfterAllocationPlanV1 {
    owner: FunctionOwnerIdV1,
    loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    source_site: crate::mir::resolved_semantics::SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
    segment_count: u32,
}

impl PreparedLoopV2AfterAllocationPlanV1 {
    fn new(
        owner: FunctionOwnerIdV1,
        loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
        source_site: crate::mir::resolved_semantics::SourceStmtSiteV1,
        frame: LoopExecutionFrameKeyV1,
        segment_count: u32,
    ) -> Self {
        Self {
            owner,
            loop_key,
            source_site,
            frame,
            segment_count,
        }
    }
}

/// The only physical result of this I0.  The lifetime prevents the view from
/// outliving the callback-scoped canonical session; it carries no edge or
/// publication authority.
#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) struct PreparedAfterBlockViewV1<'session> {
    owner: FunctionOwnerIdV1,
    loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    source_site: crate::mir::resolved_semantics::SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
    physical_block: BasicBlockId,
    _session: PhantomData<&'session mut ()>,
}

impl PreparedAfterBlockViewV1<'_> {
    pub(in crate::mir::builder::resolved_lowering) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder::resolved_lowering) const fn loop_key(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopNodeKeyV1 {
        self.loop_key
    }

    pub(in crate::mir::builder::resolved_lowering) const fn source_site(
        &self,
    ) -> &crate::mir::resolved_semantics::SourceStmtSiteV1 {
        &self.source_site
    }

    pub(in crate::mir::builder::resolved_lowering) fn frame(&self) -> LoopExecutionFrameKeyV1 {
        self.frame.clone()
    }

    pub(in crate::mir::builder::resolved_lowering) const fn physical_block(&self) -> BasicBlockId {
        self.physical_block
    }
}

pub(in crate::mir::builder::resolved_lowering) fn issue_after_allocation_plan(
    state: &mut AfterBlockAllocationStateV1,
    session: &CanonicalSsaFunctionSessionV2<'_>,
    envelope: &PreparedLoopV2PreSessionEnvelopeV1<'_, '_>,
    segment_receipt: &PreparedSegmentBlockReceiptV1,
) -> Result<PreparedLoopV2AfterAllocationPlanV1, AfterBlockAllocationRejectV1> {
    match state {
        AfterBlockAllocationStateV1::Available => {}
        AfterBlockAllocationStateV1::Prepared => {
            return Err(AfterBlockAllocationRejectV1::AlreadyPrepared)
        }
        AfterBlockAllocationStateV1::Allocated => {
            return Err(AfterBlockAllocationRejectV1::AlreadyAllocated)
        }
    }

    let relation = envelope.after_boundary();
    if relation.owner() != session.owner() || segment_receipt.owner() != session.owner() {
        return Err(AfterBlockAllocationRejectV1::OwnerMismatch);
    }
    if relation.relation() != LoopV2AfterBoundaryRelationV1::RootAfter {
        return Err(AfterBlockAllocationRejectV1::MissingRootAfter);
    }
    let (after_loop, _, _) = envelope.layout().after();
    if after_loop != relation.loop_key() {
        return Err(AfterBlockAllocationRejectV1::RelationMismatch);
    }

    let segments = envelope.layout().segments();
    let expected_count = u32::try_from(segments.len())
        .map_err(|_| AfterBlockAllocationRejectV1::SegmentCoverageMismatch)?;
    if segment_receipt.rows().len() != segments.len() {
        return Err(AfterBlockAllocationRejectV1::SegmentCoverageMismatch);
    }
    for (row, segment) in segment_receipt.rows().iter().zip(segments.iter()) {
        if row.loop_key() != segment.loop_key()
            || row.logical_block() != segment.block()
            || row.split_ordinal() != segment.split_ordinal()
        {
            return Err(AfterBlockAllocationRejectV1::SegmentCoverageMismatch);
        }
    }

    *state = AfterBlockAllocationStateV1::Prepared;
    Ok(PreparedLoopV2AfterAllocationPlanV1::new(
        session.owner(),
        relation.loop_key(),
        relation.source_site().clone(),
        relation.frame().clone(),
        expected_count,
    ))
}

pub(in crate::mir::builder::resolved_lowering) fn allocate_after_block<'session>(
    state: &mut AfterBlockAllocationStateV1,
    session: &'session mut CanonicalSsaFunctionSessionV2<'_>,
    builder: &mut MirBuilder,
    plan: PreparedLoopV2AfterAllocationPlanV1,
) -> Result<PreparedAfterBlockViewV1<'session>, AfterBlockAllocationRejectV1> {
    if *state != AfterBlockAllocationStateV1::Prepared {
        return Err(match *state {
            AfterBlockAllocationStateV1::Available => AfterBlockAllocationRejectV1::AlreadyPrepared,
            AfterBlockAllocationStateV1::Prepared => unreachable!(),
            AfterBlockAllocationStateV1::Allocated => {
                AfterBlockAllocationRejectV1::AlreadyAllocated
            }
        });
    }
    if plan.owner != session.owner() {
        return Err(AfterBlockAllocationRejectV1::OwnerMismatch);
    }
    if plan.segment_count == 0 {
        return Err(AfterBlockAllocationRejectV1::SegmentCoverageMismatch);
    }
    if builder.function_state.current_function.is_none() {
        return Err(AfterBlockAllocationRejectV1::MissingFunction);
    }

    let next = builder.core_ctx.peek_next_block().as_u32();
    let end = next
        .checked_add(1)
        .filter(|end| *end < u32::MAX)
        .ok_or(AfterBlockAllocationRejectV1::CursorRange)?;
    let _ = end;
    let candidate = BasicBlockId::new(next);
    if builder
        .function_state
        .current_function
        .as_ref()
        .expect("current function checked above")
        .get_block(candidate)
        .is_some()
    {
        return Err(AfterBlockAllocationRejectV1::ExistingBlock(candidate));
    }

    let physical_block = session
        .create_unpublished_block(builder)
        .map_err(|_| AfterBlockAllocationRejectV1::Allocation)?;
    *state = AfterBlockAllocationStateV1::Allocated;
    Ok(PreparedAfterBlockViewV1 {
        owner: plan.owner,
        loop_key: plan.loop_key,
        source_site: plan.source_site,
        frame: plan.frame,
        physical_block,
        _session: PhantomData,
    })
}
