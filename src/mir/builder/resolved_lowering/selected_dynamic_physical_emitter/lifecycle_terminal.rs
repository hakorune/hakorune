//! Physical lifecycle terminal handoff for the selected Dynamic V2 cohort.
//!
//! This is a narrow physical projection of already verified cleanup rows and
//! the admitted I6/I7 site plans.  It does not release a handle, issue a lease,
//! or choose a runtime path.  The canonical SSA/CFG sessions consume the
//! resulting plan later; runtime remains the execution consumer.

use crate::mir::builder::resolved_lowering::selected_dynamic_physical_capability::DynamicV2TemporaryDischargeRowV1;
use crate::mir::checked_callout::{
    CheckedCallOutLeaseSlotIdV1, CheckedCallOutNormalShapeV1, CheckedCallOutSiteIdV1,
    CheckedCallOutSitePlanPairV1,
};
use crate::mir::compiler::dynamic_full_body_recipe::DynamicInvocationCleanupRowKindV1;
use crate::mir::MirInstruction;

const I6: u32 = 6;
const I7: u32 = 7;
const V10: u32 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DynamicV2PhysicalEndCutPointV1 {
    I7Fault,
    InnerReturn,
    Backedge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DynamicV2PhysicalLifecycleTerminalRejectV1 {
    SiteShape,
    PlanStamp,
    CleanupCoverage,
    CleanupAction,
    CleanupConsumption,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DynamicV2PhysicalCleanupCutPointV1 {
    I6Fault,
    I7Fault,
    InnerReturn,
    Backedge,
}

impl DynamicV2PhysicalCleanupCutPointV1 {
    const fn index(self) -> usize {
        match self {
            Self::I6Fault => 0,
            Self::I7Fault => 1,
            Self::InnerReturn => 2,
            Self::Backedge => 3,
        }
    }
}

/// Session-private exact-once cursor for the four existing cleanup rows.
/// It transports no semantic meaning; the row issuer and cleanup locations
/// remain the existing A-prime/Recipe products and profile census.
#[derive(Debug)]
pub(super) struct DynamicV2PhysicalCleanupCursorV1 {
    rows: [Option<DynamicV2TemporaryDischargeRowV1>; 4],
}

impl DynamicV2PhysicalCleanupCursorV1 {
    pub(super) fn issue(rows: [DynamicV2TemporaryDischargeRowV1; 4]) -> Self {
        Self {
            rows: rows.map(Some),
        }
    }

    pub(super) fn with_row<R>(
        &self,
        cut: DynamicV2PhysicalCleanupCutPointV1,
        callback: impl FnOnce(&DynamicV2TemporaryDischargeRowV1) -> R,
    ) -> Result<R, DynamicV2PhysicalLifecycleTerminalRejectV1> {
        self.rows[cut.index()]
            .as_ref()
            .map(callback)
            .ok_or(DynamicV2PhysicalLifecycleTerminalRejectV1::CleanupConsumption)
    }

    pub(super) fn claim(
        &mut self,
        cut: DynamicV2PhysicalCleanupCutPointV1,
    ) -> Result<DynamicV2TemporaryDischargeRowV1, DynamicV2PhysicalLifecycleTerminalRejectV1> {
        self.rows[cut.index()]
            .take()
            .ok_or(DynamicV2PhysicalLifecycleTerminalRejectV1::CleanupConsumption)
    }

    pub(super) fn close(self) -> Result<(), DynamicV2PhysicalLifecycleTerminalRejectV1> {
        if self.rows.iter().any(Option::is_some) {
            Err(DynamicV2PhysicalLifecycleTerminalRejectV1::CleanupConsumption)
        } else {
            Ok(())
        }
    }
}

/// Move-only physical lifecycle plan.  It records only the exact I6 lease
/// slot and the three cleanup cut points which must consume it.
#[derive(Debug)]
pub(super) struct DynamicV2PhysicalLifecycleTerminalPlanV1 {
    i6_site: CheckedCallOutSiteIdV1,
    i7_site: CheckedCallOutSiteIdV1,
    lease_slot: CheckedCallOutLeaseSlotIdV1,
    end_cutpoints: [DynamicV2PhysicalEndCutPointV1; 3],
}

impl DynamicV2PhysicalLifecycleTerminalPlanV1 {
    pub(super) fn issue(
        site_plans: &CheckedCallOutSitePlanPairV1,
        cleanup: &[DynamicV2TemporaryDischargeRowV1; 4],
    ) -> Result<Self, DynamicV2PhysicalLifecycleTerminalRejectV1> {
        site_plans.with_sites(|i6, i7| {
            if i6.site_id() != CheckedCallOutSiteIdV1(0)
                || i7.site_id() != CheckedCallOutSiteIdV1(1)
                || !matches!(
                    i6.normal_shape(),
                    CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                        lease_slot: CheckedCallOutLeaseSlotIdV1(0)
                    }
                )
                || !matches!(i7.normal_shape(), CheckedCallOutNormalShapeV1::ImmediateI64)
                || !i6.plan_stamp().same(i7.plan_stamp())
            {
                return Err(DynamicV2PhysicalLifecycleTerminalRejectV1::SiteShape);
            }
            if !valid_cleanup(cleanup) {
                return Err(DynamicV2PhysicalLifecycleTerminalRejectV1::CleanupCoverage);
            }
            Ok(Self {
                i6_site: i6.site_id(),
                i7_site: i7.site_id(),
                lease_slot: CheckedCallOutLeaseSlotIdV1(0),
                end_cutpoints: [
                    DynamicV2PhysicalEndCutPointV1::I7Fault,
                    DynamicV2PhysicalEndCutPointV1::InnerReturn,
                    DynamicV2PhysicalEndCutPointV1::Backedge,
                ],
            })
        })
    }

    pub(super) const fn i6_site(&self) -> CheckedCallOutSiteIdV1 {
        self.i6_site
    }

    pub(super) const fn i7_site(&self) -> CheckedCallOutSiteIdV1 {
        self.i7_site
    }

    pub(super) const fn lease_slot(&self) -> CheckedCallOutLeaseSlotIdV1 {
        self.lease_slot
    }

    pub(super) const fn end_cutpoints(&self) -> [DynamicV2PhysicalEndCutPointV1; 3] {
        self.end_cutpoints
    }

    pub(super) fn end_instruction(&self) -> MirInstruction {
        MirInstruction::CheckedCallOutEnd {
            site_id: self.i6_site,
            lease_slot: self.lease_slot,
        }
    }

    pub(super) fn fault_instruction(
        &self,
        site_id: CheckedCallOutSiteIdV1,
    ) -> Result<MirInstruction, DynamicV2PhysicalLifecycleTerminalRejectV1> {
        if site_id != self.i6_site && site_id != self.i7_site {
            return Err(DynamicV2PhysicalLifecycleTerminalRejectV1::SiteShape);
        }
        Ok(MirInstruction::CheckedCallOutFault { site_id })
    }
}

fn valid_cleanup(rows: &[DynamicV2TemporaryDischargeRowV1; 4]) -> bool {
    let expected = [
        (DynamicInvocationCleanupRowKindV1::Fault, Some(I6), None),
        (
            DynamicInvocationCleanupRowKindV1::Fault,
            Some(I7),
            Some((I6, V10)),
        ),
        (
            DynamicInvocationCleanupRowKindV1::InnerReturn,
            None,
            Some((I6, V10)),
        ),
        (
            DynamicInvocationCleanupRowKindV1::Backedge,
            None,
            Some((I6, V10)),
        ),
    ];
    rows.iter()
        .zip(expected)
        .enumerate()
        .all(|(index, (row, expected))| {
            let (kind, item, action) = expected;
            let action_pair = row
                .first()
                .map(|action| (action.producer().raw(), action.result().raw()));
            row.kind() == kind
                && row.item().map(|item| item.raw()) == item
                && action_pair == action
                && row.second().is_none()
                && match index {
                    0 | 1 => row.inner_return_site().is_none() && row.backedge_loop().is_none(),
                    2 => row.inner_return_site().is_some() && row.backedge_loop().is_none(),
                    3 => row.inner_return_site().is_none() && row.backedge_loop().is_some(),
                    _ => false,
                }
        })
}
