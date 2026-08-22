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
    "hako.pinned_text_backend_frame@2";
pub(crate) const PINNED_TEXT_BACKEND_FRAME_SCHEMA_REVISION_V1: u32 = 2;

/// Pre-entry physical provenance for one pinned-Text frame.
///
/// This deliberately has no plan table and therefore cannot advertise a
/// premature `plan_count`.  The canonical function owner consumes it only
/// after its own plan table has been populated, then obtains the final frame
/// contract by calling `finalize` exactly once.
#[derive(Debug)]
pub(crate) struct PinnedTextBackendFrameIngressV1<'target> {
    residence: ResidenceAbiLayoutV1,
    target: &'target PinnedTextCompileTargetCapabilityV1,
}

impl<'target> PinnedTextBackendFrameIngressV1<'target> {
    pub(crate) fn prepare(
        residence: ResidenceAbiLayoutV1,
        target: &'target PinnedTextCompileTargetCapabilityV1,
    ) -> Result<Self, PinnedTextBackendFrameContractIssueV1> {
        validate_residence_target(residence, target)?;
        Ok(Self { residence, target })
    }

    pub(crate) fn finalize(
        self,
        signature: &ResolvedCallablePhysicalSignatureLoanV1<'_>,
        plans: &PinnedTextAccessPlanTableV1,
    ) -> Result<PinnedTextBackendFrameContractV1, PinnedTextBackendFrameContractIssueV1> {
        issue_pinned_text_backend_frame_contract_v1(
            signature,
            plans,
            self.residence,
            self.target,
        )
    }
}

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
    llvm_c_api_abi_revision: &'static str,
    object_cpu: &'static str,
    object_features: &'static str,
    object_codegen_opt_level: u8,
    object_relocation_model: u8,
    object_code_model: u8,
}

impl PinnedTextBackendFrameContractV1 {
    /// Lend the already co-sealed facts for one synchronous backend callback.
    ///
    /// This is a compile-time view only: it carries no runtime residence,
    /// pointer, length, lease token, or generation value and cannot outlive
    /// the function-owned contract.
    #[must_use = "a backend-frame borrow must remain scoped to its contract"]
    pub(crate) fn borrow(&self) -> PinnedTextBackendFrameBorrowV1<'_> {
        PinnedTextBackendFrameBorrowV1 { contract: self }
    }

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
                "object_emitter": {
                    "llvm_c_api_abi_revision": self.llvm_c_api_abi_revision,
                    "cpu": self.object_cpu,
                    "features": self.object_features,
                    "codegen_opt_level": self.object_codegen_opt_level,
                    "relocation_model": self.object_relocation_model,
                    "code_model": self.object_code_model,
                },
            },
        })
    }

    #[cfg(test)]
    pub(crate) fn from_test(
        owner: FunctionOwnerIdV1,
        plan_stamp: u64,
        frame_revision: u32,
    ) -> Self {
        Self {
            owner,
            invocation_ordinal: 1,
            source_logical_arity: 1,
            receiver_lane_count: 0,
            physical_formal_lane_count: 2,
            physical_callable_lane_count: 2,
            exact_text_root_count: 1,
            plan_stamp,
            plan_count: 1,
            frame_revision,
            frame_size: 48,
            residence_abi_revision: "text-formal-residence-v1",
            header_size: 32,
            root_row_size: 16,
            header_alignment: 8,
            root_row_alignment: 8,
            max_root_count: 1024,
            max_frame_bytes: 65_536,
            target_profile_id: "test-pinned-text-target",
            target_triple: "x86_64-pc-linux-gnu",
            target_data_layout: "test-layout",
            target_little_endian: true,
            target_pointer_width: 64,
            target_pointer_alignment: 8,
            target_max_root_count: 1024,
            target_max_private_frame_bytes: 65_536,
            consumer_abi_revision: "test-consumer-v1",
            llvm_c_api_abi_revision: "test-llvm-c-api-v1",
            object_cpu: "",
            object_features: "",
            object_codegen_opt_level: 0,
            object_relocation_model: 0,
            object_code_model: 0,
        }
    }
}

/// Scoped, non-pointer projection of an already-issued backend-frame
/// contract.  This view is deliberately not `Clone` or `Copy`; the lifetime
/// ties every projection to the owning function contract and prevents a
/// detached backend table from becoming a second authority.
#[must_use = "a backend-frame borrow must be consumed within its callback"]
#[derive(Debug)]
pub(crate) struct PinnedTextBackendFrameBorrowV1<'contract> {
    contract: &'contract PinnedTextBackendFrameContractV1,
}

impl<'contract> PinnedTextBackendFrameBorrowV1<'contract> {
    pub(crate) const fn contract_id(&self) -> &'static str {
        PINNED_TEXT_BACKEND_FRAME_CONTRACT_ID_V1
    }

    pub(crate) const fn schema_revision(&self) -> u32 {
        PINNED_TEXT_BACKEND_FRAME_SCHEMA_REVISION_V1
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.contract.owner
    }

    pub(crate) const fn invocation_ordinal(&self) -> u64 {
        self.contract.invocation_ordinal
    }

    pub(crate) const fn source_logical_arity(&self) -> u32 {
        self.contract.source_logical_arity
    }

    pub(crate) const fn receiver_lane_count(&self) -> u32 {
        self.contract.receiver_lane_count
    }

    pub(crate) const fn physical_formal_lane_count(&self) -> u32 {
        self.contract.physical_formal_lane_count
    }

    pub(crate) const fn physical_callable_lane_count(&self) -> u32 {
        self.contract.physical_callable_lane_count
    }

    pub(crate) const fn exact_text_root_count(&self) -> u32 {
        self.contract.exact_text_root_count
    }

    pub(crate) const fn plan_stamp(&self) -> u64 {
        self.contract.plan_stamp
    }

    pub(crate) const fn plan_count(&self) -> u32 {
        self.contract.plan_count
    }

    pub(crate) const fn residence_abi_revision(&self) -> &'static str {
        self.contract.residence_abi_revision
    }

    pub(crate) const fn frame_revision(&self) -> u32 {
        self.contract.frame_revision
    }

    pub(crate) const fn frame_size(&self) -> u32 {
        self.contract.frame_size
    }

    pub(crate) const fn target_profile_id(&self) -> &'static str {
        self.contract.target_profile_id
    }

    pub(crate) const fn target_triple(&self) -> &'static str {
        self.contract.target_triple
    }

    pub(crate) const fn target_data_layout(&self) -> &'static str {
        self.contract.target_data_layout
    }

    pub(crate) const fn target_pointer_width(&self) -> u16 {
        self.contract.target_pointer_width
    }

    pub(crate) const fn target_pointer_alignment(&self) -> u16 {
        self.contract.target_pointer_alignment
    }

    pub(crate) const fn consumer_abi_revision(&self) -> &'static str {
        self.contract.consumer_abi_revision
    }

    pub(crate) const fn llvm_c_api_abi_revision(&self) -> &'static str {
        self.contract.llvm_c_api_abi_revision
    }

    /// Serialize the same owned contract projection without making the view
    /// itself an independent transport or authority.
    pub(crate) fn to_transport_json(&self) -> serde_json::Value {
        self.contract.to_transport_json()
    }
}

/// Sole bridge for the four-input backend-frame relation.
pub(crate) fn issue_pinned_text_backend_frame_contract_v1(
    signature: &ResolvedCallablePhysicalSignatureLoanV1<'_>,
    plans: &PinnedTextAccessPlanTableV1,
    residence: ResidenceAbiLayoutV1,
    target: &PinnedTextCompileTargetCapabilityV1,
) -> Result<PinnedTextBackendFrameContractV1, PinnedTextBackendFrameContractIssueV1> {
    validate_residence_target(residence, target)?;
    let profile = target.profile();

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
        llvm_c_api_abi_revision: profile.llvm_c_api_abi_revision(),
        object_cpu: profile.object_cpu(),
        object_features: profile.object_features(),
        object_codegen_opt_level: profile.object_codegen_opt_level(),
        object_relocation_model: profile.object_relocation_model(),
        object_code_model: profile.object_code_model(),
    })
}

fn validate_residence_target(
    residence: ResidenceAbiLayoutV1,
    target: &PinnedTextCompileTargetCapabilityV1,
) -> Result<(), PinnedTextBackendFrameContractIssueV1> {
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
    Ok(())
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
            consumer_abi_revision: "hako-llvmc-pure-first-v2",
            llvm_c_api_abi_revision: "llvm-c-api-18-v1",
            object_cpu: "",
            object_features: "",
            object_codegen_opt_level: 3,
            object_relocation_model: 0,
            object_code_model: 0,
        };

        let owned_json = contract.to_transport_json();
        let borrowed_json = {
            let borrow = contract.borrow();
            assert_eq!(borrow.contract_id(), PINNED_TEXT_BACKEND_FRAME_CONTRACT_ID_V1);
            assert_eq!(borrow.schema_revision(), PINNED_TEXT_BACKEND_FRAME_SCHEMA_REVISION_V1);
            assert_eq!(borrow.owner(), owner);
            assert_eq!(borrow.invocation_ordinal(), 7);
            assert_eq!(borrow.source_logical_arity(), 2);
            assert_eq!(borrow.receiver_lane_count(), 1);
            assert_eq!(borrow.physical_formal_lane_count(), 4);
            assert_eq!(borrow.physical_callable_lane_count(), 5);
            assert_eq!(borrow.exact_text_root_count(), 2);
            assert_eq!(borrow.plan_stamp(), 19);
            assert_eq!(borrow.plan_count(), 3);
            assert_eq!(borrow.residence_abi_revision(), "text-formal-residence-v1");
            assert_eq!(borrow.frame_revision(), 1);
            assert_eq!(borrow.frame_size(), 64);
            assert_eq!(borrow.target_profile_id(), "nyrt-text-residence-ptr64-as0-v1");
            assert_eq!(borrow.target_triple(), "x86_64-pc-linux-gnu");
            assert_eq!(borrow.target_data_layout(), "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128");
            assert_eq!(borrow.target_pointer_width(), 64);
            assert_eq!(borrow.target_pointer_alignment(), 8);
            assert_eq!(borrow.consumer_abi_revision(), "hako-llvmc-pure-first-v2");
            assert_eq!(borrow.llvm_c_api_abi_revision(), "llvm-c-api-18-v1");
            borrow.to_transport_json()
        };
        assert_eq!(owned_json, borrowed_json);
        let json = owned_json;
        assert_eq!(json["contract_id"], PINNED_TEXT_BACKEND_FRAME_CONTRACT_ID_V1);
        assert_eq!(json["schema_revision"], PINNED_TEXT_BACKEND_FRAME_SCHEMA_REVISION_V1);
        assert_eq!(json["physical_callable_lane_count"], 5);
        assert_eq!(json["residence"]["derived_frame_size"], 64);
        assert_eq!(json["target"]["triple"], "x86_64-pc-linux-gnu");
        assert!(json.get("runtime").is_none());
        assert!(json.get("pointer").is_none());
    }
}
