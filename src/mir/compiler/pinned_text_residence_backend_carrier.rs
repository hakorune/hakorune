//! Strict physical transport for one pinned-Text Residence candidate.
//!
//! This product only bundles already-issued physical facts.  It carries no
//! source meaning, runtime pointer, token, `ValueId`, or backend instruction.
//! The JSON projection is consumed by the selected pure-first backend; it must
//! never reconstruct lane/root meaning from an ordinal or frame row.

use crate::mir::basic_block::BasicBlockId;
use crate::mir::compiler::pinned_text_backend_frame::{
    PinnedTextBackendFrameBorrowV1, PINNED_TEXT_BACKEND_FRAME_CONTRACT_ID_V1,
    PINNED_TEXT_BACKEND_FRAME_SCHEMA_REVISION_V1,
};
use crate::mir::normal_callable_semantic_package::{
    PhysicalCallableLaneRoleV1, PhysicalCallableSignatureRowRefV1,
};
use crate::mir::pinned_text_residence_lifecycle::{
    PinnedTextResidencePlanIdV1, PreparedPinnedTextResidenceLifecycleV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};

pub(crate) const PINNED_TEXT_RESIDENCE_BACKEND_CARRIER_CONTRACT_ID_V1: &str =
    "hako.pinned_text_residence_carrier@1";
pub(crate) const PINNED_TEXT_RESIDENCE_BACKEND_CARRIER_SCHEMA_REVISION_V1: u32 = 1;
pub(crate) const PINNED_TEXT_RESIDENCE_FINISH_OBLIGATION_V1: &str =
    "finish_every_explicit_normal_return";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PinnedTextResidenceBackendCarrierIssueV1 {
    OwnerMismatch,
    FrameContractMismatch,
    MissingPlanStamp,
    PlanStampMismatch,
    LandingBlocksMustDiffer,
    FinishSiteMissing,
    FinishSiteDuplicate,
    FinishOnTrap,
    FinishCountMismatch,
    LaneIndexMismatch,
    ExactTextPairMismatch,
    RootCountMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PinnedTextResidenceBackendRootRowV1 {
    frame_row: u32,
    logical_ordinal: u32,
    source_binding: BindingRefV1,
    slot_lane: u32,
    generation_lane: u32,
}

impl PinnedTextResidenceBackendRootRowV1 {
    pub(crate) const fn frame_row(self) -> u32 {
        self.frame_row
    }

    pub(crate) const fn logical_ordinal(self) -> u32 {
        self.logical_ordinal
    }

    pub(crate) const fn source_binding(self) -> BindingRefV1 {
        self.source_binding
    }

    pub(crate) const fn slot_lane(self) -> u32 {
        self.slot_lane
    }

    pub(crate) const fn generation_lane(self) -> u32 {
        self.generation_lane
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PinnedTextResidenceBackendCarrierV1 {
    frame_contract_id: &'static str,
    frame_schema_revision: u32,
    owner: FunctionOwnerIdV1,
    invocation_ordinal: u64,
    target_profile_id: &'static str,
    target_triple: &'static str,
    target_data_layout: &'static str,
    residence_abi_revision: &'static str,
    plan: PinnedTextResidencePlanIdV1,
    enter_source: BasicBlockId,
    normal_landing: BasicBlockId,
    trap_landing: BasicBlockId,
    finish_blocks: Box<[BasicBlockId]>,
    normal_exit_count: u32,
    roots: Box<[PinnedTextResidenceBackendRootRowV1]>,
}

pub(super) struct PinnedTextResidenceBackendCarrierProjectionViewV1<'carrier> {
    pub(super) owner: FunctionOwnerIdV1,
    pub(super) invocation_ordinal: u64,
    pub(super) target_profile_id: &'carrier str,
    pub(super) target_triple: &'carrier str,
    pub(super) target_data_layout: &'carrier str,
    pub(super) residence_abi_revision: &'carrier str,
    pub(super) plan: PinnedTextResidencePlanIdV1,
    pub(super) enter_source: BasicBlockId,
    pub(super) normal_landing: BasicBlockId,
    pub(super) trap_landing: BasicBlockId,
    pub(super) finish_blocks: &'carrier [BasicBlockId],
    pub(super) normal_exit_count: u32,
}

/// Move-only lineage retained before the canonical Enter consumes its
/// lifecycle carrier.  It keeps the package-owned signature beside the exact
/// physical placement, so DraftSeal can add the authoritative exit set
/// without reconstructing plan or block identity from MIR.
#[derive(Debug)]
pub(crate) struct PinnedTextResidenceBackendCarrierLineageV1<'loan> {
    signature: PhysicalCallableSignatureRowRefV1<'loan>,
    plan: PinnedTextResidencePlanIdV1,
    enter_source: BasicBlockId,
    normal_landing: BasicBlockId,
    trap_landing: BasicBlockId,
}

impl<'loan> PinnedTextResidenceBackendCarrierLineageV1<'loan> {
    pub(crate) fn from_lifecycle(
        signature: PhysicalCallableSignatureRowRefV1<'loan>,
        enter_source: BasicBlockId,
        lifecycle: &PreparedPinnedTextResidenceLifecycleV1,
    ) -> Result<Self, PinnedTextResidenceBackendCarrierIssueV1> {
        if signature.owner() != lifecycle.plan().owner() {
            return Err(PinnedTextResidenceBackendCarrierIssueV1::OwnerMismatch);
        }
        if enter_source == lifecycle.normal_landing()
            || enter_source == lifecycle.trap_landing()
        {
            return Err(PinnedTextResidenceBackendCarrierIssueV1::LandingBlocksMustDiffer);
        }
        Ok(Self {
            signature,
            plan: lifecycle.plan(),
            enter_source,
            normal_landing: lifecycle.normal_landing(),
            trap_landing: lifecycle.trap_landing(),
        })
    }

    pub(crate) fn issue(
        self,
        frame: PinnedTextBackendFrameBorrowV1<'_>,
        finish_blocks: Box<[BasicBlockId]>,
        normal_exit_count: u32,
    ) -> Result<PinnedTextResidenceBackendCarrierV1, PinnedTextResidenceBackendCarrierIssueV1>
    {
        PinnedTextResidenceBackendCarrierV1::issue(
            self.signature,
            frame,
            self.plan,
            self.enter_source,
            self.normal_landing,
            self.trap_landing,
            finish_blocks,
            normal_exit_count,
        )
    }
}

impl PinnedTextResidenceBackendCarrierV1 {
    /// Issue one carrier from the package-owned signature and the existing
    /// frame/lifecycle products.  No lane or root relation is inferred.
    fn issue(
        signature: PhysicalCallableSignatureRowRefV1<'_>,
        frame: PinnedTextBackendFrameBorrowV1<'_>,
        plan: PinnedTextResidencePlanIdV1,
        enter_source: BasicBlockId,
        normal_landing: BasicBlockId,
        trap_landing: BasicBlockId,
        finish_blocks: Box<[BasicBlockId]>,
        normal_exit_count: u32,
    ) -> Result<Self, PinnedTextResidenceBackendCarrierIssueV1> {
        if signature.owner() != frame.owner() || plan.owner() != frame.owner() {
            return Err(PinnedTextResidenceBackendCarrierIssueV1::OwnerMismatch);
        }
        if frame.contract_id() != PINNED_TEXT_BACKEND_FRAME_CONTRACT_ID_V1
            || frame.schema_revision() != PINNED_TEXT_BACKEND_FRAME_SCHEMA_REVISION_V1
        {
            return Err(PinnedTextResidenceBackendCarrierIssueV1::FrameContractMismatch);
        }
        if frame.plan_stamp() == 0 || plan.plan_stamp() == 0 {
            return Err(PinnedTextResidenceBackendCarrierIssueV1::MissingPlanStamp);
        }
        if frame.plan_stamp() != plan.plan_stamp()
            || frame.frame_revision() != 1
            || frame.residence_abi_revision().is_empty()
        {
            return Err(PinnedTextResidenceBackendCarrierIssueV1::PlanStampMismatch);
        }
        if enter_source == normal_landing
            || enter_source == trap_landing
            || normal_landing == trap_landing
        {
            return Err(PinnedTextResidenceBackendCarrierIssueV1::LandingBlocksMustDiffer);
        }
        if normal_exit_count == 0
            || usize::try_from(normal_exit_count).ok() != Some(finish_blocks.len())
        {
            return Err(PinnedTextResidenceBackendCarrierIssueV1::FinishCountMismatch);
        }

        let mut seen_finish_blocks = std::collections::BTreeSet::new();
        for block in &finish_blocks {
            if *block == trap_landing {
                return Err(PinnedTextResidenceBackendCarrierIssueV1::FinishOnTrap);
            }
            if !seen_finish_blocks.insert(*block) {
                return Err(PinnedTextResidenceBackendCarrierIssueV1::FinishSiteDuplicate);
            }
        }
        if finish_blocks.is_empty() {
            return Err(PinnedTextResidenceBackendCarrierIssueV1::FinishSiteMissing);
        }

        let mut roots = Vec::new();
        let lanes = signature.lanes();
        let mut index = 0usize;
        let mut expected_ordinal = 0u32;
        while index < lanes.len() {
            let slot = lanes[index];
            if slot.index() != u32::try_from(index).unwrap_or(u32::MAX) {
                return Err(PinnedTextResidenceBackendCarrierIssueV1::LaneIndexMismatch);
            }
            match slot.role() {
                PhysicalCallableLaneRoleV1::InstanceReceiver => {
                    if slot.logical_ordinal().is_some() {
                        return Err(PinnedTextResidenceBackendCarrierIssueV1::LaneIndexMismatch);
                    }
                    index += 1;
                }
                PhysicalCallableLaneRoleV1::OrdinaryScalar => {
                    if slot.logical_ordinal() != Some(expected_ordinal) {
                        return Err(PinnedTextResidenceBackendCarrierIssueV1::LaneIndexMismatch);
                    }
                    expected_ordinal = expected_ordinal.saturating_add(1);
                    index += 1;
                }
                PhysicalCallableLaneRoleV1::ExactTextSlot => {
                    let Some(generation) = lanes.get(index + 1).copied() else {
                        return Err(PinnedTextResidenceBackendCarrierIssueV1::ExactTextPairMismatch);
                    };
                    if generation.role() != PhysicalCallableLaneRoleV1::ExactTextGeneration
                        || generation.index() != slot.index().saturating_add(1)
                        || generation.logical_ordinal() != slot.logical_ordinal()
                        || generation.binding() != slot.binding()
                    {
                        return Err(PinnedTextResidenceBackendCarrierIssueV1::ExactTextPairMismatch);
                    }
                    let Some(logical_ordinal) = slot.logical_ordinal() else {
                        return Err(PinnedTextResidenceBackendCarrierIssueV1::ExactTextPairMismatch);
                    };
                    if logical_ordinal != expected_ordinal {
                        return Err(PinnedTextResidenceBackendCarrierIssueV1::LaneIndexMismatch);
                    }
                    roots.push(PinnedTextResidenceBackendRootRowV1 {
                        frame_row: u32::try_from(roots.len()).map_err(|_| {
                            PinnedTextResidenceBackendCarrierIssueV1::RootCountMismatch
                        })?,
                        logical_ordinal,
                        source_binding: slot.binding(),
                        slot_lane: slot.index(),
                        generation_lane: generation.index(),
                    });
                    expected_ordinal = expected_ordinal.saturating_add(1);
                    index += 2;
                }
                PhysicalCallableLaneRoleV1::ExactTextGeneration => {
                    return Err(PinnedTextResidenceBackendCarrierIssueV1::ExactTextPairMismatch);
                }
            }
        }
        if roots.len() != usize::try_from(frame.exact_text_root_count()).unwrap_or(usize::MAX) {
            return Err(PinnedTextResidenceBackendCarrierIssueV1::RootCountMismatch);
        }

        Ok(Self {
            frame_contract_id: PINNED_TEXT_BACKEND_FRAME_CONTRACT_ID_V1,
            frame_schema_revision: PINNED_TEXT_BACKEND_FRAME_SCHEMA_REVISION_V1,
            owner: frame.owner(),
            invocation_ordinal: frame.invocation_ordinal(),
            target_profile_id: frame.target_profile_id(),
            target_triple: frame.target_triple(),
            target_data_layout: frame.target_data_layout(),
            residence_abi_revision: frame.residence_abi_revision(),
            plan,
            enter_source,
            normal_landing,
            trap_landing,
            finish_blocks,
            normal_exit_count,
            roots: roots.into_boxed_slice(),
        })
    }

    #[cfg(test)]
    pub(crate) fn issue_for_test(
        signature: PhysicalCallableSignatureRowRefV1<'_>,
        frame: PinnedTextBackendFrameBorrowV1<'_>,
        plan: PinnedTextResidencePlanIdV1,
        enter_source: BasicBlockId,
        normal_landing: BasicBlockId,
        trap_landing: BasicBlockId,
        finish_blocks: Box<[BasicBlockId]>,
        normal_exit_count: u32,
    ) -> Result<Self, PinnedTextResidenceBackendCarrierIssueV1> {
        Self::issue(
            signature,
            frame,
            plan,
            enter_source,
            normal_landing,
            trap_landing,
            finish_blocks,
            normal_exit_count,
        )
    }

    pub(super) fn projection_view(&self) -> PinnedTextResidenceBackendCarrierProjectionViewV1<'_> {
        PinnedTextResidenceBackendCarrierProjectionViewV1 {
            owner: self.owner,
            invocation_ordinal: self.invocation_ordinal,
            target_profile_id: self.target_profile_id,
            target_triple: self.target_triple,
            target_data_layout: self.target_data_layout,
            residence_abi_revision: self.residence_abi_revision,
            plan: self.plan,
            enter_source: self.enter_source,
            normal_landing: self.normal_landing,
            trap_landing: self.trap_landing,
            finish_blocks: &self.finish_blocks,
            normal_exit_count: self.normal_exit_count,
        }
    }

    pub(crate) fn to_transport_json(&self) -> serde_json::Value {
        let owner = |owner: FunctionOwnerIdV1| {
            serde_json::json!({
                "compilation_brand": owner.compilation_brand(),
                "slot": owner.slot(),
            })
        };
        serde_json::json!({
            "contract_id": PINNED_TEXT_RESIDENCE_BACKEND_CARRIER_CONTRACT_ID_V1,
            "schema_revision": PINNED_TEXT_RESIDENCE_BACKEND_CARRIER_SCHEMA_REVISION_V1,
            "frame_contract": {
                "contract_id": self.frame_contract_id,
                "schema_revision": self.frame_schema_revision,
            },
            "owner": owner(self.owner),
            "invocation_ordinal": self.invocation_ordinal,
            "target": {
                "profile_id": self.target_profile_id,
                "triple": self.target_triple,
                "data_layout": self.target_data_layout,
            },
            "residence_abi_revision": self.residence_abi_revision,
            "plan_stamp": self.plan.plan_stamp(),
            "enter": {
                "source": self.enter_source.as_u32(),
                "normal": self.normal_landing.as_u32(),
                "trap": self.trap_landing.as_u32(),
            },
            "finish_obligation": PINNED_TEXT_RESIDENCE_FINISH_OBLIGATION_V1,
            "normal_exit_count": self.normal_exit_count,
            "finish_sites": self.finish_blocks.iter().map(|block| block.as_u32()).collect::<Vec<_>>(),
            "roots": self.roots.iter().map(|root| {
                serde_json::json!({
                    "frame_row": root.frame_row(),
                    "logical_ordinal": root.logical_ordinal(),
                    "source_binding": {
                        "owner": owner(root.source_binding().owner()),
                        "binding_id": root.source_binding().binding().raw(),
                    },
                    "slot_lane": root.slot_lane(),
                    "generation_lane": root.generation_lane(),
                })
            }).collect::<Vec<_>>(),
        })
    }
}
