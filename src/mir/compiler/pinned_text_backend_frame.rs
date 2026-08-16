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

pub(crate) const PINNED_TEXT_BACKEND_FRAME_CONTRACT_ID_V1: &str =
    "hako.pinned_text_backend_frame@1";
pub(crate) const PINNED_TEXT_BACKEND_FRAME_SCHEMA_REVISION_V1: u32 = 1;

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
    residence_abi_revision: &'static str,
    header_size: u32,
    root_row_size: u32,
    header_alignment: u32,
    root_row_alignment: u32,
    max_root_count: u32,
    max_frame_bytes: u32,
    target_profile_id: &'static str,
    target_triple: &'static str,
    target_data_layout: &'static str,
    target_little_endian: bool,
    target_pointer_width: u16,
    target_pointer_alignment: u16,
    target_max_root_count: u32,
    target_max_private_frame_bytes: u32,
    consumer_abi_revision: &'static str,
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

    pub(crate) fn to_transport_json(self) -> serde_json::Value {
        let profile = self.target_profile_id;
        serde_json::json!({
            "contract_id": PINNED_TEXT_BACKEND_FRAME_CONTRACT_ID_V1,
            "schema_revision": PINNED_TEXT_BACKEND_FRAME_SCHEMA_REVISION_V1,
            "owner": {
                "compilation_brand": self.owner.compilation_brand(),
                "slot": self.owner.slot(),
            },
            "invocation_ordinal": self.invocation_ordinal,
            "source_logical_arity": self.source_logical_arity,
            "receiver_lane_count": self.receiver_lane_count,
            "physical_formal_lane_count": self.physical_formal_lane_count,
            "physical_callable_lane_count": self.physical_callable_lane_count,
            "exact_text_root_count": self.exact_text_root_count,
            "plan_stamp": self.plan_stamp,
            "plan_count": self.plan_count,
            "residence": {
                "abi_revision": self.residence_abi_revision,
                "frame_revision": self.frame_revision,
                "header_size": self.header_size,
                "root_row_size": self.root_row_size,
                "header_alignment": self.header_alignment,
                "root_row_alignment": self.root_row_alignment,
                "max_root_count": self.max_root_count,
                "max_frame_bytes": self.max_frame_bytes,
                "derived_frame_size": self.frame_size,
            },
            "target": {
                "profile_id": profile,
                "triple": self.target_triple,
                "data_layout": self.target_data_layout,
                "little_endian": self.target_little_endian,
                "address_space_zero_pointer_width": self.target_pointer_width,
                "address_space_zero_abi_alignment": self.target_pointer_alignment,
                "max_root_count": self.target_max_root_count,
                "max_private_frame_bytes": self.target_max_private_frame_bytes,
                "residence_abi_revision": self.residence_abi_revision,
                "consumer_abi_revision": self.consumer_abi_revision,
            },
        })
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
        residence_abi_revision: residence.revision(),
        header_size: residence.header_size(),
        root_row_size: residence.root_row_size(),
        header_alignment: residence.header_alignment(),
        root_row_alignment: residence.root_row_alignment(),
        max_root_count: residence.max_root_count(),
        max_frame_bytes: residence.max_frame_bytes(),
        target_profile_id: profile.profile_id(),
        target_triple: profile.target_triple(),
        target_data_layout: profile.data_layout(),
        target_little_endian: profile.little_endian(),
        target_pointer_width: profile.address_space_zero_pointer_width(),
        target_pointer_alignment: profile.address_space_zero_abi_alignment(),
        target_max_root_count: profile.max_root_count(),
        target_max_private_frame_bytes: profile.max_private_frame_bytes(),
        consumer_abi_revision: profile.consumer_abi_revision(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

    #[test]
    fn transport_projection_is_versioned_and_has_no_runtime_fields() {
        let mut owners = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        let owner = owners.issue().expect("function owner");
        let contract = PinnedTextBackendFrameContractV1 {
            owner,
            invocation_ordinal: 7,
            source_logical_arity: 2,
            receiver_lane_count: 1,
            physical_formal_lane_count: 4,
            physical_callable_lane_count: 5,
            exact_text_root_count: 2,
            plan_stamp: 19,
            plan_count: 3,
            frame_revision: 1,
            frame_size: 64,
            residence_abi_revision: "text-formal-residence-v1",
            header_size: 32,
            root_row_size: 16,
            header_alignment: 8,
            root_row_alignment: 8,
            max_root_count: 1024,
            max_frame_bytes: 65_536,
            target_profile_id: "nyrt-text-residence-ptr64-as0-v1",
            target_triple: "x86_64-pc-linux-gnu",
            target_data_layout:
                "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128",
            target_little_endian: true,
            target_pointer_width: 64,
            target_pointer_alignment: 8,
            target_max_root_count: 1024,
            target_max_private_frame_bytes: 65_536,
            consumer_abi_revision: "hako-llvmc-pure-first-v1",
        };

        let json = contract.to_transport_json();
        assert_eq!(json["contract_id"], PINNED_TEXT_BACKEND_FRAME_CONTRACT_ID_V1);
        assert_eq!(json["schema_revision"], PINNED_TEXT_BACKEND_FRAME_SCHEMA_REVISION_V1);
        assert_eq!(json["physical_callable_lane_count"], 5);
        assert_eq!(json["residence"]["derived_frame_size"], 64);
        assert_eq!(json["target"]["triple"], "x86_64-pc-linux-gnu");
        assert!(json.get("runtime").is_none());
        assert!(json.get("pointer").is_none());
    }
}
