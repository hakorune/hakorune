//! Physical-only lifecycle carriers for one pinned-Text Residence.
//!
//! The ids in this module are compile-time provenance.  They contain no
//! handle, generation, frame address, pointer, or status value.  The affine
//! carrier is consumed by the canonical unpublished session; it is not a
//! source receipt and it is not a backend/runtime transport object.

use crate::mir::basic_block::BasicBlockId;
use crate::mir::compiler::pinned_text_backend_frame::PinnedTextBackendFrameBorrowV1;
use crate::mir::pinned_text_access_plan::PinnedTextAccessPlanTableV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct PinnedTextResidencePlanIdV1 {
    owner: FunctionOwnerIdV1,
    invocation_ordinal: u64,
    plan_stamp: u64,
    frame_revision: u32,
    residence_abi_revision: &'static str,
}

impl PinnedTextResidencePlanIdV1 {
    pub(crate) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn plan_stamp(self) -> u64 {
        self.plan_stamp
    }
}

/// Function-local identity for the one Residence obligation created by Enter.
/// It is deliberately not a runtime lease/token identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TextFormalResidenceIdV1 {
    owner: FunctionOwnerIdV1,
    invocation_ordinal: u64,
    plan_stamp: u64,
}

impl TextFormalResidenceIdV1 {
    pub(crate) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn plan_stamp(self) -> u64 {
        self.plan_stamp
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PinnedTextResidenceLifecycleRejectV1 {
    OwnerMismatch,
    MissingPlanStamp,
    PlanStampMismatch,
    FrameRevisionMismatch,
    ResidenceAbiMismatch,
    LandingBlocksMustDiffer,
}

/// Private, affine physical admission for the Enter/Finish pair.
///
/// This aggregate only co-seals existing frame/plan provenance and landing
/// placement.  It does not issue source meaning, runtime state, or a new
/// semantic `Verified*`/`Prepared*` receipt.
#[must_use = "the lifecycle carrier must be consumed by the canonical session"]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PreparedPinnedTextResidenceLifecycleV1 {
    plan: PinnedTextResidencePlanIdV1,
    residence: TextFormalResidenceIdV1,
    normal_landing: BasicBlockId,
    trap_landing: BasicBlockId,
}

impl PreparedPinnedTextResidenceLifecycleV1 {
    pub(crate) fn issue_from_frame(
        owner: FunctionOwnerIdV1,
        plans: &PinnedTextAccessPlanTableV1,
        frame: PinnedTextBackendFrameBorrowV1<'_>,
        normal_landing: BasicBlockId,
        trap_landing: BasicBlockId,
    ) -> Result<Self, PinnedTextResidenceLifecycleRejectV1> {
        if owner != frame.owner() {
            return Err(PinnedTextResidenceLifecycleRejectV1::OwnerMismatch);
        }
        if normal_landing == trap_landing {
            return Err(PinnedTextResidenceLifecycleRejectV1::LandingBlocksMustDiffer);
        }
        if frame.plan_stamp() == 0 || plans.stamp() == 0 {
            return Err(PinnedTextResidenceLifecycleRejectV1::MissingPlanStamp);
        }
        if plans.stamp() != frame.plan_stamp() {
            return Err(PinnedTextResidenceLifecycleRejectV1::PlanStampMismatch);
        }
        if frame.frame_revision() != 1 {
            return Err(PinnedTextResidenceLifecycleRejectV1::FrameRevisionMismatch);
        }
        if frame.residence_abi_revision().is_empty() {
            return Err(PinnedTextResidenceLifecycleRejectV1::ResidenceAbiMismatch);
        }
        let plan = PinnedTextResidencePlanIdV1 {
            owner,
            invocation_ordinal: frame.invocation_ordinal(),
            plan_stamp: frame.plan_stamp(),
            frame_revision: frame.frame_revision(),
            residence_abi_revision: frame.residence_abi_revision(),
        };
        let residence = TextFormalResidenceIdV1 {
            owner,
            invocation_ordinal: plan.invocation_ordinal,
            plan_stamp: plan.plan_stamp,
        };
        Ok(Self {
            plan,
            residence,
            normal_landing,
            trap_landing,
        })
    }

    pub(crate) const fn plan(&self) -> PinnedTextResidencePlanIdV1 {
        self.plan
    }

    pub(crate) const fn residence(&self) -> TextFormalResidenceIdV1 {
        self.residence
    }

    pub(crate) const fn normal_landing(&self) -> BasicBlockId {
        self.normal_landing
    }

    pub(crate) const fn trap_landing(&self) -> BasicBlockId {
        self.trap_landing
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        PinnedTextResidencePlanIdV1,
        TextFormalResidenceIdV1,
        BasicBlockId,
        BasicBlockId,
    ) {
        (
            self.plan,
            self.residence,
            self.normal_landing,
            self.trap_landing,
        )
    }
}

/// One-shot function-local finish obligation returned only after the canonical
/// Enter edge is installed.  The obligation is consumed once by the
/// DraftSeal exit-set projector; the projector may then place the same
/// residence marker on each already-validated explicit exit.  Exit placement
/// is intentionally not encoded here because `PreparedFunctionExitSetV1` is
/// the sole exit authority.
#[must_use = "the Enter result must be consumed by the canonical Finish writer"]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PinnedTextResidenceFinishCapabilityV1 {
    residence: TextFormalResidenceIdV1,
}

impl PinnedTextResidenceFinishCapabilityV1 {
    pub(crate) const fn residence(&self) -> TextFormalResidenceIdV1 {
        self.residence
    }

    pub(crate) fn into_residence(self) -> TextFormalResidenceIdV1 {
        self.residence
    }

    pub(in crate::mir) const fn from_parts(residence: TextFormalResidenceIdV1) -> Self {
        Self { residence }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::pinned_text_backend_frame::PinnedTextBackendFrameContractV1;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

    fn owner() -> FunctionOwnerIdV1 {
        FunctionOwnerIssuerV1::new_for_compilation()
            .expect("compilation brand")
            .issue()
            .expect("function owner")
    }

    #[test]
    fn carrier_co_seals_frame_and_plan_provenance() {
        let owner_id = owner();
        let plans = PinnedTextAccessPlanTableV1::new(17);
        let frame_contract = PinnedTextBackendFrameContractV1::from_test(owner_id, 17, 1);
        let frame = frame_contract.borrow();
        let carrier = PreparedPinnedTextResidenceLifecycleV1::issue_from_frame(
            owner_id,
            &plans,
            frame,
            BasicBlockId::new(1),
            BasicBlockId::new(2),
        )
        .expect("same-cohort carrier");

        assert_eq!(carrier.plan().owner(), owner_id);
        assert_eq!(carrier.plan().plan_stamp(), 17);
        assert_eq!(carrier.residence().owner(), owner_id);
        assert_eq!(carrier.normal_landing(), BasicBlockId::new(1));
        assert_eq!(carrier.trap_landing(), BasicBlockId::new(2));
    }

    #[test]
    fn carrier_rejects_foreign_stale_and_duplicate_placement() {
        let owner_id = owner();
        let foreign = owner();
        let plans = PinnedTextAccessPlanTableV1::new(17);
        let frame_contract = PinnedTextBackendFrameContractV1::from_test(owner_id, 17, 1);
        let frame = frame_contract.borrow();
        assert_eq!(
            PreparedPinnedTextResidenceLifecycleV1::issue_from_frame(
                foreign,
                &plans,
                frame,
                BasicBlockId::new(1),
                BasicBlockId::new(2),
            ),
            Err(PinnedTextResidenceLifecycleRejectV1::OwnerMismatch)
        );

        let foreign_plans = PinnedTextAccessPlanTableV1::new(18);
        let frame_contract = PinnedTextBackendFrameContractV1::from_test(owner_id, 17, 1);
        let frame = frame_contract.borrow();
        assert_eq!(
            PreparedPinnedTextResidenceLifecycleV1::issue_from_frame(
                owner_id,
                &foreign_plans,
                frame,
                BasicBlockId::new(1),
                BasicBlockId::new(2),
            ),
            Err(PinnedTextResidenceLifecycleRejectV1::PlanStampMismatch)
        );

        let plans = PinnedTextAccessPlanTableV1::new(17);
        let frame_contract = PinnedTextBackendFrameContractV1::from_test(owner_id, 17, 1);
        let frame = frame_contract.borrow();
        assert_eq!(
            PreparedPinnedTextResidenceLifecycleV1::issue_from_frame(
                owner_id,
                &plans,
                frame,
                BasicBlockId::new(1),
                BasicBlockId::new(1),
            ),
            Err(PinnedTextResidenceLifecycleRejectV1::LandingBlocksMustDiffer)
        );
    }

    #[test]
    fn carrier_rejects_stale_frame_revision() {
        let owner = owner();
        let plans = PinnedTextAccessPlanTableV1::new(17);
        let frame_contract = PinnedTextBackendFrameContractV1::from_test(owner, 17, 2);
        let frame = frame_contract.borrow();
        assert_eq!(
            PreparedPinnedTextResidenceLifecycleV1::issue_from_frame(
                owner,
                &plans,
                frame,
                BasicBlockId::new(1),
                BasicBlockId::new(2),
            ),
            Err(PinnedTextResidenceLifecycleRejectV1::FrameRevisionMismatch)
        );
    }
}
