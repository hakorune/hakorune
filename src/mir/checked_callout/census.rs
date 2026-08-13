use crate::mir::{BasicBlockId, EffectMask, MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet};

use super::site_plan::{
    CheckedCallOutLeaseSlotIdV1, CheckedCallOutNormalShapeV1, CheckedCallOutOutcomeSlotIdV1,
    CheckedCallOutPlanTableV1, CheckedCallOutSiteIdV1,
};

/// Borrow-free proof that every admitted site was materialized exactly once.
/// It carries census only; plan/CFG/SSA meaning stays with the existing owners.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VerifiedCheckedCallOutFunctionV1 {
    site_count: usize,
}

impl VerifiedCheckedCallOutFunctionV1 {
    pub(crate) const fn site_count(self) -> usize {
        self.site_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CheckedCallOutFunctionRejectV1 {
    OrphanPlan(CheckedCallOutSiteIdV1),
    OrphanTerminator(CheckedCallOutSiteIdV1),
    OrphanProjection(CheckedCallOutSiteIdV1),
    DuplicateTerminator(CheckedCallOutSiteIdV1),
    DuplicateProjection(CheckedCallOutSiteIdV1),
    LandingMismatch(CheckedCallOutSiteIdV1),
    LandingPredecessorMismatch(CheckedCallOutSiteIdV1),
    ProjectionOrder(CheckedCallOutSiteIdV1),
    EffectCacheMismatch(CheckedCallOutSiteIdV1),
    DuplicateOutcomeSlot(CheckedCallOutOutcomeSlotIdV1),
    DuplicateLeaseSlot(CheckedCallOutLeaseSlotIdV1),
    PlanStampMismatch(CheckedCallOutSiteIdV1),
}

#[derive(Debug, Clone, Copy)]
struct CheckedCallOutTerminatorObservationV1 {
    source: BasicBlockId,
    normal_landing: BasicBlockId,
    fault_landing: BasicBlockId,
    effects: EffectMask,
}

/// Final function census for the neutral CheckedCallOut vocabulary.
///
/// Local emitters remain the only CFG/SSA writers. This verifier observes the
/// completed unpublished function and rejects every orphan, duplicate, or
/// late edge that would violate the plan:terminator:projection 1:1:1 law.
pub(super) fn verify_checked_callout_function_v1(
    function: &MirFunction,
    plans: &CheckedCallOutPlanTableV1,
) -> Result<VerifiedCheckedCallOutFunctionV1, CheckedCallOutFunctionRejectV1> {
    let mut terminators = BTreeMap::new();
    let mut projections = BTreeMap::new();

    for (block_id, block) in &function.blocks {
        if let Some(MirInstruction::CheckedCallOut {
            site_id,
            normal_landing,
            fault_landing,
            effects,
            ..
        }) = block.terminator.as_ref()
        {
            let observed = CheckedCallOutTerminatorObservationV1 {
                source: *block_id,
                normal_landing: *normal_landing,
                fault_landing: *fault_landing,
                effects: *effects,
            };
            if terminators.insert(*site_id, observed).is_some() {
                return Err(CheckedCallOutFunctionRejectV1::DuplicateTerminator(
                    *site_id,
                ));
            }
        }
        for (index, instruction) in block.instructions.iter().enumerate() {
            if let MirInstruction::CheckedCallOutNormalResult { site_id, .. } = instruction {
                if projections.insert(*site_id, (*block_id, index)).is_some() {
                    return Err(CheckedCallOutFunctionRejectV1::DuplicateProjection(
                        *site_id,
                    ));
                }
            }
        }
    }

    for site in terminators.keys() {
        if !plans.contains_site(*site) {
            return Err(CheckedCallOutFunctionRejectV1::OrphanTerminator(*site));
        }
    }
    for site in projections.keys() {
        if !plans.contains_site(*site) {
            return Err(CheckedCallOutFunctionRejectV1::OrphanProjection(*site));
        }
    }

    let mut outcome_slots = BTreeSet::new();
    let mut lease_slots = BTreeSet::new();
    let expected_stamp = plans.iter().next().map(|(_, plan)| plan.plan_stamp());
    for (site, plan) in plans.iter() {
        let Some(terminator) = terminators.get(site) else {
            return Err(CheckedCallOutFunctionRejectV1::OrphanPlan(*site));
        };
        let Some((projection_block, projection_index)) = projections.get(site).copied() else {
            return Err(CheckedCallOutFunctionRejectV1::OrphanProjection(*site));
        };
        if terminator.normal_landing == terminator.fault_landing
            || projection_block != terminator.normal_landing
        {
            return Err(CheckedCallOutFunctionRejectV1::LandingMismatch(*site));
        }
        if terminator.effects != plan.effects() {
            return Err(CheckedCallOutFunctionRejectV1::EffectCacheMismatch(*site));
        }
        if expected_stamp.is_some_and(|stamp| !plan.plan_stamp().same(stamp)) {
            return Err(CheckedCallOutFunctionRejectV1::PlanStampMismatch(*site));
        }
        if !outcome_slots.insert(plan.outcome_slot()) {
            return Err(CheckedCallOutFunctionRejectV1::DuplicateOutcomeSlot(
                plan.outcome_slot(),
            ));
        }
        if let CheckedCallOutNormalShapeV1::EndAuthorizedHandle { lease_slot } = plan.normal_shape()
        {
            if !lease_slots.insert(lease_slot) {
                return Err(CheckedCallOutFunctionRejectV1::DuplicateLeaseSlot(
                    lease_slot,
                ));
            }
        }

        for landing in [terminator.normal_landing, terminator.fault_landing] {
            let Some(block) = function.get_block(landing) else {
                return Err(CheckedCallOutFunctionRejectV1::LandingMismatch(*site));
            };
            if block.predecessors.len() != 1 || !block.predecessors.contains(&terminator.source) {
                return Err(CheckedCallOutFunctionRejectV1::LandingPredecessorMismatch(
                    *site,
                ));
            }
        }
        let normal = function
            .get_block(terminator.normal_landing)
            .expect("Normal landing was checked above");
        let first_non_phi = normal
            .instructions
            .iter()
            .position(|instruction| !matches!(instruction, MirInstruction::Phi { .. }))
            .unwrap_or(normal.instructions.len());
        if projection_index != first_non_phi {
            return Err(CheckedCallOutFunctionRejectV1::ProjectionOrder(*site));
        }
    }

    Ok(VerifiedCheckedCallOutFunctionV1 {
        site_count: plans.len(),
    })
}

/// A Normal-landing projection is an ordinary SSA definition.  It is not a
/// terminator destination and therefore never dominates the Fault landing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CheckedCallOutNormalResultProjectionV1 {
    site_id: CheckedCallOutSiteIdV1,
    normal_landing: BasicBlockId,
    dst: ValueId,
}

impl CheckedCallOutNormalResultProjectionV1 {
    pub(crate) const fn new(
        site_id: CheckedCallOutSiteIdV1,
        normal_landing: BasicBlockId,
        dst: ValueId,
    ) -> Self {
        Self {
            site_id,
            normal_landing,
            dst,
        }
    }

    pub(crate) const fn site_id(self) -> CheckedCallOutSiteIdV1 {
        self.site_id
    }

    pub(crate) const fn normal_landing(self) -> BasicBlockId {
        self.normal_landing
    }

    pub(crate) const fn dst(self) -> ValueId {
        self.dst
    }
}
