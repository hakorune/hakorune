//! Rust-side co-seal for the pinned-Text backend-frame contract.
//!
//! This owner is intentionally pre-JSON and pre-backend.  It validates the
//! relation between the package lane cohort, the function-local access-plan
//! census, the Residence ABI view, and one explicit compile target.  It does
//! not publish pointers, runtime tokens, or MIR execution instructions.

use crate::mir::normal_callable_semantic_package::ResolvedCallablePhysicalSignatureLoanV1;
use crate::mir::pinned_text_access_plan::PinnedTextAccessPlanTableV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::runtime::text_formal_residence::ResidenceAbiLayoutV1;

use super::target_capability::PinnedTextCompileTargetCapabilityV1;
use crate::mir::normal_callable_semantic_package::{
    PhysicalCallableLaneRoleV1, PhysicalCallableLaneV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PinnedTextBackendFrameContractIssueV1 {
    ResidenceRevisionMismatch,
    ResidenceFrameMismatch,
    TargetLayoutMismatch,
    LaneIndexMismatch,
    ReceiverLaneMismatch,
    FormalOrdinalMismatch,
    TextLanePairMismatch,
    RootCountMismatch,
    FrameSizeOverflow,
    FrameLimitExceeded,
}

/// Owned private facts returned by the co-seal.  The lane rows themselves are
/// validated through the scoped loan and only their derived census survives.
/// No semantic or runtime authority is copied into this product.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PinnedTextBackendFrameContractV1 {
    owner: FunctionOwnerIdV1,
    invocation_ordinal: u64,
    source_logical_arity: u32,
    receiver_lane_count: u32,
    physical_formal_lane_count: u32,
    physical_callable_lane_count: u32,
    exact_text_root_count: u32,
    plan_stamp: u64,
    plan_count: u32,
    frame_revision: u32,
    frame_size: u32,
}

impl PinnedTextBackendFrameContractV1 {
    pub(crate) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn exact_text_root_count(self) -> u32 {
        self.exact_text_root_count
    }

    pub(crate) const fn plan_stamp(self) -> u64 {
        self.plan_stamp
    }

    pub(crate) const fn plan_count(self) -> u32 {
        self.plan_count
    }
}

/// Sole bridge for the four-input backend-frame relation.
pub(crate) fn issue_pinned_text_backend_frame_contract_v1(
    signature: &ResolvedCallablePhysicalSignatureLoanV1<'_>,
    plans: &PinnedTextAccessPlanTableV1,
    residence: ResidenceAbiLayoutV1,
    target: &PinnedTextCompileTargetCapabilityV1,
) -> Result<PinnedTextBackendFrameContractV1, PinnedTextBackendFrameContractIssueV1> {
    let profile = target.profile();
    if residence.revision() != profile.residence_abi_revision() {
        return Err(PinnedTextBackendFrameContractIssueV1::ResidenceRevisionMismatch);
    }
    if residence.frame_revision() != 1
        || residence.header_size() != 32
        || residence.root_row_size() != 16
        || residence.header_alignment() != 8
        || residence.root_row_alignment() != 8
    {
        return Err(PinnedTextBackendFrameContractIssueV1::ResidenceFrameMismatch);
    }
    if profile.address_space_zero_pointer_width() != 64
        || profile.address_space_zero_abi_alignment() != 8
        || !profile.little_endian()
    {
        return Err(PinnedTextBackendFrameContractIssueV1::TargetLayoutMismatch);
    }

    let lanes = signature.lanes();
    let (receiver_lane_count, exact_text_root_count) = validate_lanes(lanes)?;
    let lane_count = u32::try_from(lanes.len())
        .map_err(|_| PinnedTextBackendFrameContractIssueV1::RootCountMismatch)?;
    if receiver_lane_count != signature.receiver_lane_count()
        || lane_count != signature.physical_callable_lane_count()
        || lane_count
            .checked_sub(receiver_lane_count)
            .ok_or(PinnedTextBackendFrameContractIssueV1::RootCountMismatch)?
            != signature.physical_formal_lane_count()
    {
        return Err(PinnedTextBackendFrameContractIssueV1::RootCountMismatch);
    }
    if exact_text_root_count > residence.max_root_count()
        || exact_text_root_count > profile.max_root_count()
    {
        return Err(PinnedTextBackendFrameContractIssueV1::FrameLimitExceeded);
    }
    let frame_size = residence
        .frame_size_for_roots(exact_text_root_count)
        .ok_or(PinnedTextBackendFrameContractIssueV1::FrameSizeOverflow)?;
    if frame_size > residence.max_frame_bytes() || frame_size > profile.max_private_frame_bytes() {
        return Err(PinnedTextBackendFrameContractIssueV1::FrameLimitExceeded);
    }

    let plan_count = u32::try_from(plans.row_count())
        .map_err(|_| PinnedTextBackendFrameContractIssueV1::FrameSizeOverflow)?;
    Ok(PinnedTextBackendFrameContractV1 {
        owner: signature.owner(),
        invocation_ordinal: target.invocation_ordinal().get(),
        source_logical_arity: signature.source_logical_arity(),
        receiver_lane_count,
        physical_formal_lane_count: signature.physical_formal_lane_count(),
        physical_callable_lane_count: signature.physical_callable_lane_count(),
        exact_text_root_count,
        plan_stamp: plans.stamp(),
        plan_count,
        frame_revision: residence.frame_revision(),
        frame_size,
    })
}

fn validate_lanes(
    lanes: &[PhysicalCallableLaneV1],
) -> Result<(u32, u32), PinnedTextBackendFrameContractIssueV1> {
    let mut receiver_count = 0u32;
    let mut roots = 0u32;
    let mut ordinal = 0u32;
    let mut index = 0usize;
    while index < lanes.len() {
        let lane = lanes[index];
        if lane.index() != index as u32 {
            return Err(PinnedTextBackendFrameContractIssueV1::LaneIndexMismatch);
        }
        match lane.role() {
            PhysicalCallableLaneRoleV1::InstanceReceiver => {
                if index != 0 || lane.logical_ordinal().is_some() {
                    return Err(PinnedTextBackendFrameContractIssueV1::ReceiverLaneMismatch);
                }
                receiver_count = receiver_count
                    .checked_add(1)
                    .ok_or(PinnedTextBackendFrameContractIssueV1::RootCountMismatch)?;
                index += 1;
            }
            PhysicalCallableLaneRoleV1::OrdinaryScalar => {
                if lane.logical_ordinal() != Some(ordinal) {
                    return Err(PinnedTextBackendFrameContractIssueV1::FormalOrdinalMismatch);
                }
                ordinal = ordinal
                    .checked_add(1)
                    .ok_or(PinnedTextBackendFrameContractIssueV1::RootCountMismatch)?;
                index += 1;
            }
            PhysicalCallableLaneRoleV1::ExactTextSlot => {
                let Some(generation) = lanes.get(index + 1).copied() else {
                    return Err(PinnedTextBackendFrameContractIssueV1::TextLanePairMismatch);
                };
                if lane.index().checked_add(1) != Some(generation.index())
                    || generation.role() != PhysicalCallableLaneRoleV1::ExactTextGeneration
                    || generation.logical_ordinal() != Some(ordinal)
                    || generation.binding() != lane.binding()
                {
                    return Err(PinnedTextBackendFrameContractIssueV1::TextLanePairMismatch);
                }
                ordinal = ordinal
                    .checked_add(1)
                    .ok_or(PinnedTextBackendFrameContractIssueV1::RootCountMismatch)?;
                roots = roots
                    .checked_add(1)
                    .ok_or(PinnedTextBackendFrameContractIssueV1::RootCountMismatch)?;
                index += 2;
            }
            PhysicalCallableLaneRoleV1::ExactTextGeneration => {
                return Err(PinnedTextBackendFrameContractIssueV1::TextLanePairMismatch);
            }
        }
    }
    Ok((receiver_count, roots))
}
