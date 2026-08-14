use crate::mir::{BasicBlockId, EffectMask, MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet};

use super::site_plan::{
    CheckedCallOutLeaseSlotIdV1, CheckedCallOutNormalShapeV1, CheckedCallOutOutcomeSlotIdV1,
    CheckedCallOutPlanTableV1, CheckedCallOutSiteIdV1,
};

/// One canonical, transportable observation of a checked call site.  The
/// observation is issued by the final census and is never reconstructed by a
/// JSON or Boundary consumer.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CheckedCallOutSiteCensusV1 {
    site_id: CheckedCallOutSiteIdV1,
    source_block: BasicBlockId,
    receiver: ValueId,
    arguments: Box<[ValueId]>,
    normal_landing: BasicBlockId,
    fault_landing: BasicBlockId,
    fault_terminal_block: Option<BasicBlockId>,
    normal_result_block: BasicBlockId,
    normal_result_index: usize,
    normal_result_dst: ValueId,
    effects: EffectMask,
}

impl CheckedCallOutSiteCensusV1 {
    pub(crate) const fn site_id(&self) -> CheckedCallOutSiteIdV1 {
        self.site_id
    }
    pub(crate) const fn source_block(&self) -> BasicBlockId {
        self.source_block
    }
    pub(crate) const fn receiver(&self) -> ValueId {
        self.receiver
    }
    pub(crate) fn arguments(&self) -> &[ValueId] {
        &self.arguments
    }
    pub(crate) const fn normal_landing(&self) -> BasicBlockId {
        self.normal_landing
    }
    pub(crate) const fn fault_landing(&self) -> BasicBlockId {
        self.fault_landing
    }
    pub(crate) const fn fault_terminal_block(&self) -> Option<BasicBlockId> {
        self.fault_terminal_block
    }
    pub(crate) const fn normal_result_block(&self) -> BasicBlockId {
        self.normal_result_block
    }
    pub(crate) const fn normal_result_index(&self) -> usize {
        self.normal_result_index
    }
    pub(crate) const fn normal_result_dst(&self) -> ValueId {
        self.normal_result_dst
    }
    pub(crate) const fn effects(&self) -> EffectMask {
        self.effects
    }
}

/// A physical End position observed in the completed function.  The selected
/// cohort expects three of these facts; their placement remains lifecycle
/// evidence, not a new cleanup authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CheckedCallOutEndCensusV1 {
    site_id: CheckedCallOutSiteIdV1,
    lease_slot: CheckedCallOutLeaseSlotIdV1,
    block: BasicBlockId,
    instruction_index: usize,
}

impl CheckedCallOutEndCensusV1 {
    #[cfg(test)]
    pub(crate) const fn from_test(
        site_id: CheckedCallOutSiteIdV1,
        lease_slot: CheckedCallOutLeaseSlotIdV1,
        block: BasicBlockId,
        instruction_index: usize,
    ) -> Self {
        Self {
            site_id,
            lease_slot,
            block,
            instruction_index,
        }
    }

    pub(crate) const fn site_id(self) -> CheckedCallOutSiteIdV1 {
        self.site_id
    }
    pub(crate) const fn lease_slot(self) -> CheckedCallOutLeaseSlotIdV1 {
        self.lease_slot
    }
    pub(crate) const fn block(self) -> BasicBlockId {
        self.block
    }
    pub(crate) const fn instruction_index(self) -> usize {
        self.instruction_index
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CheckedCallOutFunctionCensusViewV1<'a> {
    sites: &'a [CheckedCallOutSiteCensusV1],
    ends: &'a [CheckedCallOutEndCensusV1],
}

impl<'a> CheckedCallOutFunctionCensusViewV1<'a> {
    pub(crate) fn sites(&self) -> &'a [CheckedCallOutSiteCensusV1] {
        self.sites
    }
    pub(crate) fn ends(&self) -> &'a [CheckedCallOutEndCensusV1] {
        self.ends
    }
    pub(crate) fn site(
        &self,
        id: CheckedCallOutSiteIdV1,
    ) -> Option<&'a CheckedCallOutSiteCensusV1> {
        self.sites.iter().find(|site| site.site_id() == id)
    }
}

/// Final, non-Clone proof that every admitted site was materialized exactly
/// once and that its physical facts are available to one downstream handoff.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedCheckedCallOutFunctionV1 {
    sites: Box<[CheckedCallOutSiteCensusV1]>,
    ends: Box<[CheckedCallOutEndCensusV1]>,
}

impl VerifiedCheckedCallOutFunctionV1 {
    pub(crate) fn site_count(&self) -> usize {
        self.sites.len()
    }

    /// The callback prevents the borrowed census from escaping into a second
    /// plan/authority.  Callers must immediately project or cross-check it.
    pub(crate) fn with_view<R>(
        &self,
        f: impl for<'a> FnOnce(CheckedCallOutFunctionCensusViewV1<'a>) -> R,
    ) -> R {
        f(CheckedCallOutFunctionCensusViewV1 {
            sites: &self.sites,
            ends: &self.ends,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CheckedCallOutFunctionRejectV1 {
    OrphanPlan(CheckedCallOutSiteIdV1),
    OrphanTerminator(CheckedCallOutSiteIdV1),
    OrphanProjection(CheckedCallOutSiteIdV1),
    OrphanFault(CheckedCallOutSiteIdV1),
    DuplicateTerminator(CheckedCallOutSiteIdV1),
    DuplicateProjection(CheckedCallOutSiteIdV1),
    DuplicateFault(CheckedCallOutSiteIdV1),
    LandingMismatch(CheckedCallOutSiteIdV1),
    LandingPredecessorMismatch(CheckedCallOutSiteIdV1),
    ProjectionOrder(CheckedCallOutSiteIdV1),
    EffectCacheMismatch(CheckedCallOutSiteIdV1),
    DuplicateOutcomeSlot(CheckedCallOutOutcomeSlotIdV1),
    DuplicateLeaseSlot(CheckedCallOutLeaseSlotIdV1),
    PlanStampMismatch(CheckedCallOutSiteIdV1),
}

#[derive(Debug)]
struct CheckedCallOutTerminatorObservationV1 {
    source: BasicBlockId,
    receiver: ValueId,
    arguments: Box<[ValueId]>,
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
    let mut faults = BTreeMap::new();
    let mut ends = Vec::new();

    for (block_id, block) in &function.blocks {
        if let Some(MirInstruction::CheckedCallOut {
            site_id,
            receiver,
            arguments,
            normal_landing,
            fault_landing,
            effects,
            ..
        }) = block.terminator.as_ref()
        {
            let observed = CheckedCallOutTerminatorObservationV1 {
                source: *block_id,
                receiver: *receiver,
                arguments: arguments.clone().into_boxed_slice(),
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
            match instruction {
                MirInstruction::CheckedCallOutNormalResult { site_id, dst } => {
                    if projections
                        .insert(*site_id, (*block_id, index, *dst))
                        .is_some()
                    {
                        return Err(CheckedCallOutFunctionRejectV1::DuplicateProjection(
                            *site_id,
                        ));
                    }
                }
                MirInstruction::CheckedCallOutEnd {
                    site_id,
                    lease_slot,
                } => {
                    ends.push((
                        *site_id,
                        *lease_slot,
                        *block_id,
                        emitted_index(block, index),
                    ));
                }
                _ => {}
            }
        }
        if let Some(MirInstruction::CheckedCallOutFault { site_id }) = block.terminator.as_ref() {
            if faults.insert(*site_id, *block_id).is_some() {
                return Err(CheckedCallOutFunctionRejectV1::DuplicateFault(*site_id));
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
    for (site, _, _, _) in &ends {
        if !plans.contains_site(*site) {
            return Err(CheckedCallOutFunctionRejectV1::OrphanProjection(*site));
        }
    }
    for site in faults.keys() {
        if !plans.contains_site(*site) {
            return Err(CheckedCallOutFunctionRejectV1::OrphanFault(*site));
        }
    }

    let mut outcome_slots = BTreeSet::new();
    let mut lease_slots = BTreeSet::new();
    let mut site_facts = Vec::with_capacity(plans.len());
    let expected_stamp = plans.iter().next().map(|(_, plan)| plan.plan_stamp());
    for (site, plan) in plans.iter() {
        let Some(terminator) = terminators.get(site) else {
            return Err(CheckedCallOutFunctionRejectV1::OrphanPlan(*site));
        };
        let Some((projection_block, projection_index, projection_dst)) =
            projections.get(site).copied()
        else {
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
        site_facts.push(CheckedCallOutSiteCensusV1 {
            site_id: *site,
            source_block: terminator.source,
            receiver: terminator.receiver,
            arguments: terminator.arguments.clone(),
            normal_landing: terminator.normal_landing,
            fault_landing: terminator.fault_landing,
            fault_terminal_block: faults.get(site).copied(),
            normal_result_block: projection_block,
            normal_result_index: emitted_index(normal, projection_index),
            normal_result_dst: projection_dst,
            effects: terminator.effects,
        });
    }

    Ok(VerifiedCheckedCallOutFunctionV1 {
        sites: site_facts.into_boxed_slice(),
        ends: ends
            .into_iter()
            .map(
                |(site_id, lease_slot, block, instruction_index)| CheckedCallOutEndCensusV1 {
                    site_id,
                    lease_slot,
                    block,
                    instruction_index,
                },
            )
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    })
}

fn emitted_index(block: &crate::mir::BasicBlock, raw_index: usize) -> usize {
    let phi_count = block
        .instructions
        .iter()
        .filter(|instruction| matches!(instruction, MirInstruction::Phi { .. }))
        .count();
    let non_phi_before = block.instructions[..raw_index]
        .iter()
        .filter(|instruction| !matches!(instruction, MirInstruction::Phi { .. }))
        .count();
    if matches!(block.instructions[raw_index], MirInstruction::Phi { .. }) {
        block.instructions[..raw_index]
            .iter()
            .filter(|instruction| matches!(instruction, MirInstruction::Phi { .. }))
            .count()
    } else {
        phi_count + non_phi_before
    }
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
