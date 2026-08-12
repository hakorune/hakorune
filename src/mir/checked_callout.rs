//! Neutral MIR vocabulary for a checked call with canonical Normal/Fault CFG.
//!
//! This module is deliberately physical-only.  It does not resolve a provider,
//! selector, runtime lease token, or backend function address.  A function-local
//! site plan is admitted once and the canonical CFG/SSA sessions consume it.

use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::{BasicBlockId, EffectMask, MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CheckedCallOutSiteIdV1(pub(crate) u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CheckedCallOutEntryIdV1(pub(crate) u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CheckedCallOutOutcomeSlotIdV1(pub(crate) u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CheckedCallOutLeaseSlotIdV1(pub(crate) u32);

/// The only Normal result shapes admitted by the bounded TextScan cohort.
/// Fault never carries a result or lease slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CheckedCallOutNormalShapeV1 {
    EndAuthorizedHandle {
        lease_slot: CheckedCallOutLeaseSlotIdV1,
    },
    ImmediateI64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedCallOutSitePlanV1 {
    site_id: CheckedCallOutSiteIdV1,
    admitted_entry: CheckedCallOutEntryIdV1,
    call_abi_revision: u32,
    wire_revision: u32,
    normal_shape: CheckedCallOutNormalShapeV1,
    effects: EffectMask,
    outcome_slot: CheckedCallOutOutcomeSlotIdV1,
    plan_stamp: ModuleInvocationBrandV1,
}

/// Physical input used by the selected TextScan admission.  The caller may
/// provide only already-admitted ABI facts; site/slot identities are issued by
/// the pair constructor below.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct CheckedCallOutAdmittedSiteInputV1 {
    pub(in crate::mir) entry: CheckedCallOutEntryIdV1,
    pub(in crate::mir) call_abi_revision: u32,
    pub(in crate::mir) wire_revision: u32,
    pub(in crate::mir) normal_shape: CheckedCallOutNormalShapeV1,
    pub(in crate::mir) effects: EffectMask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum CheckedCallOutSitePlanPairRejectV1 {
    InvalidI6,
    InvalidI7,
    NonDistinctEntry,
    PlanStampMismatch,
}

/// Exactly the two admitted TextScan call plans.  It cannot be cloned or
/// split; the selected session consumes it into its function-local plan table.
#[derive(Debug)]
pub(in crate::mir) struct CheckedCallOutSitePlanPairV1 {
    i6: CheckedCallOutSitePlanV1,
    i7: CheckedCallOutSitePlanV1,
}

impl CheckedCallOutSitePlanV1 {
    #[cfg(test)]
    pub(crate) fn from_test(
        site_id: CheckedCallOutSiteIdV1,
        admitted_entry: CheckedCallOutEntryIdV1,
        normal_shape: CheckedCallOutNormalShapeV1,
        effects: EffectMask,
        plan_stamp: ModuleInvocationBrandV1,
    ) -> Self {
        Self {
            site_id,
            admitted_entry,
            call_abi_revision: 1,
            wire_revision: 2,
            normal_shape,
            effects,
            outcome_slot: CheckedCallOutOutcomeSlotIdV1(site_id.0),
            plan_stamp,
        }
    }

    pub(crate) const fn site_id(&self) -> CheckedCallOutSiteIdV1 {
        self.site_id
    }

    pub(crate) const fn effects(&self) -> EffectMask {
        self.effects
    }

    pub(crate) const fn normal_shape(&self) -> CheckedCallOutNormalShapeV1 {
        self.normal_shape
    }

    pub(crate) const fn outcome_slot(&self) -> CheckedCallOutOutcomeSlotIdV1 {
        self.outcome_slot
    }

    pub(crate) const fn plan_stamp(&self) -> ModuleInvocationBrandV1 {
        self.plan_stamp
    }

    pub(crate) fn validate_instruction(
        &self,
        site_id: CheckedCallOutSiteIdV1,
        normal_landing: BasicBlockId,
        fault_landing: BasicBlockId,
        effects: EffectMask,
    ) -> Result<(), CheckedCallOutPlanRejectV1> {
        if self.site_id != site_id {
            return Err(CheckedCallOutPlanRejectV1::ForeignSite);
        }
        if normal_landing == fault_landing {
            return Err(CheckedCallOutPlanRejectV1::NonDistinctLanding);
        }
        if self.effects != effects {
            return Err(CheckedCallOutPlanRejectV1::EffectCacheMismatch);
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn to_json_for_test(&self) -> serde_json::Value {
        let shape = match self.normal_shape {
            CheckedCallOutNormalShapeV1::EndAuthorizedHandle { lease_slot } => {
                serde_json::json!({"kind":"end_authorized_handle","lease_slot":lease_slot.0})
            }
            CheckedCallOutNormalShapeV1::ImmediateI64 => {
                serde_json::json!({"kind":"immediate_i64"})
            }
        };
        serde_json::json!({
            "site_id": self.site_id.0,
            "admitted_entry": self.admitted_entry.0,
            "call_abi_revision": self.call_abi_revision,
            "wire_revision": self.wire_revision,
            "normal_shape": shape,
            "effects": self.effects.bits(),
            "outcome_slot": self.outcome_slot.0,
            "plan_stamp": {
                "compiler_domain": self.plan_stamp.compiler_domain().get(),
                "invocation_ordinal": self.plan_stamp.invocation_ordinal().get(),
            },
        })
    }

    #[cfg(test)]
    pub(crate) fn from_json_for_test(value: &serde_json::Value) -> Result<Self, String> {
        let number = |name: &str| {
            value
                .get(name)
                .and_then(serde_json::Value::as_u64)
                .ok_or_else(|| format!("missing numeric field {name}"))
        };
        let site_id = CheckedCallOutSiteIdV1(
            u32::try_from(number("site_id")?).map_err(|_| "site id overflow".to_owned())?,
        );
        let admitted_entry = CheckedCallOutEntryIdV1(
            u32::try_from(number("admitted_entry")?).map_err(|_| "entry id overflow".to_owned())?,
        );
        let shape = value
            .get("normal_shape")
            .and_then(|shape| shape.get("kind"))
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "missing normal shape".to_owned())?;
        let normal_shape = match shape {
            "immediate_i64" => CheckedCallOutNormalShapeV1::ImmediateI64,
            "end_authorized_handle" => CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                lease_slot: CheckedCallOutLeaseSlotIdV1(
                    u32::try_from(
                        value["normal_shape"]["lease_slot"]
                            .as_u64()
                            .ok_or_else(|| "missing lease slot".to_owned())?,
                    )
                    .map_err(|_| "lease slot overflow".to_owned())?,
                ),
            },
            _ => return Err("unknown normal shape".to_owned()),
        };
        let effects = EffectMask::from_bits(
            u16::try_from(number("effects")?).map_err(|_| "effect overflow".to_owned())?,
        );
        let outcome_slot = CheckedCallOutOutcomeSlotIdV1(
            u32::try_from(number("outcome_slot")?)
                .map_err(|_| "outcome slot overflow".to_owned())?,
        );
        let ordinal = value["plan_stamp"]["invocation_ordinal"]
            .as_u64()
            .ok_or_else(|| "missing invocation ordinal".to_owned())?;
        Ok(Self {
            site_id,
            admitted_entry,
            call_abi_revision: u32::try_from(number("call_abi_revision")?)
                .map_err(|_| "call ABI revision overflow".to_owned())?,
            wire_revision: u32::try_from(number("wire_revision")?)
                .map_err(|_| "wire revision overflow".to_owned())?,
            normal_shape,
            effects,
            outcome_slot,
            plan_stamp: ModuleInvocationBrandV1::test_with_ordinal(ordinal),
        })
    }
}

impl CheckedCallOutSitePlanPairV1 {
    pub(in crate::mir) fn from_admitted(
        i6: CheckedCallOutAdmittedSiteInputV1,
        i7: CheckedCallOutAdmittedSiteInputV1,
        plan_stamp: ModuleInvocationBrandV1,
    ) -> Result<Self, CheckedCallOutSitePlanPairRejectV1> {
        if i6.entry == i7.entry {
            return Err(CheckedCallOutSitePlanPairRejectV1::NonDistinctEntry);
        }
        if i6.call_abi_revision != 1
            || i6.wire_revision != 2
            || !matches!(
                i6.normal_shape,
                CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                    lease_slot: CheckedCallOutLeaseSlotIdV1(0)
                }
            )
        {
            return Err(CheckedCallOutSitePlanPairRejectV1::InvalidI6);
        }
        if i7.call_abi_revision != 1
            || i7.wire_revision != 2
            || !matches!(i7.normal_shape, CheckedCallOutNormalShapeV1::ImmediateI64)
        {
            return Err(CheckedCallOutSitePlanPairRejectV1::InvalidI7);
        }
        Ok(Self {
            i6: CheckedCallOutSitePlanV1 {
                site_id: CheckedCallOutSiteIdV1(0),
                admitted_entry: i6.entry,
                call_abi_revision: i6.call_abi_revision,
                wire_revision: i6.wire_revision,
                normal_shape: i6.normal_shape,
                effects: i6.effects,
                outcome_slot: CheckedCallOutOutcomeSlotIdV1(0),
                plan_stamp,
            },
            i7: CheckedCallOutSitePlanV1 {
                site_id: CheckedCallOutSiteIdV1(1),
                admitted_entry: i7.entry,
                call_abi_revision: i7.call_abi_revision,
                wire_revision: i7.wire_revision,
                normal_shape: i7.normal_shape,
                effects: i7.effects,
                outcome_slot: CheckedCallOutOutcomeSlotIdV1(1),
                plan_stamp,
            },
        })
    }

    /// Consume the pair without exposing a re-pairing/parts API.
    pub(in crate::mir) fn consume<R>(
        self,
        callback: impl FnOnce(CheckedCallOutSitePlanV1, CheckedCallOutSitePlanV1) -> R,
    ) -> R {
        callback(self.i6, self.i7)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CheckedCallOutPlanRejectV1 {
    DuplicateSite(CheckedCallOutSiteIdV1),
    ForeignSite,
    NonDistinctLanding,
    EffectCacheMismatch,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct CheckedCallOutPlanTableV1 {
    plans: BTreeMap<CheckedCallOutSiteIdV1, CheckedCallOutSitePlanV1>,
}

impl CheckedCallOutPlanTableV1 {
    pub(crate) fn admit(
        &mut self,
        plan: CheckedCallOutSitePlanV1,
    ) -> Result<(), CheckedCallOutPlanRejectV1> {
        let site = plan.site_id();
        if self.plans.contains_key(&site) {
            return Err(CheckedCallOutPlanRejectV1::DuplicateSite(site));
        }
        self.plans.insert(site, plan);
        Ok(())
    }

    pub(crate) fn get(&self, site: CheckedCallOutSiteIdV1) -> Option<&CheckedCallOutSitePlanV1> {
        self.plans.get(&site)
    }

    pub(crate) fn verify_function(
        &self,
        function: &MirFunction,
    ) -> Result<VerifiedCheckedCallOutFunctionV1, CheckedCallOutFunctionRejectV1> {
        verify_checked_callout_function_v1(function, self)
    }
}

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
fn verify_checked_callout_function_v1(
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
        if !plans.plans.contains_key(site) {
            return Err(CheckedCallOutFunctionRejectV1::OrphanTerminator(*site));
        }
    }
    for site in projections.keys() {
        if !plans.plans.contains_key(site) {
            return Err(CheckedCallOutFunctionRejectV1::OrphanProjection(*site));
        }
    }

    let mut outcome_slots = BTreeSet::new();
    let mut lease_slots = BTreeSet::new();
    let expected_stamp = plans.plans.values().next().map(|plan| plan.plan_stamp());
    for (site, plan) in &plans.plans {
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
        site_count: plans.plans.len(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
    use crate::mir::{BasicBlock, FunctionSignature, MirInstruction, MirType};

    fn test_function_with_site(with_projection: bool) -> (MirFunction, CheckedCallOutPlanTableV1) {
        let source = BasicBlockId::new(0);
        let normal = BasicBlockId::new(1);
        let fault = BasicBlockId::new(2);
        let site = CheckedCallOutSiteIdV1(6);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "checked/0".to_owned(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::READ,
            },
            source,
        );
        function.add_block(BasicBlock::new(normal));
        function.add_block(BasicBlock::new(fault));
        function
            .get_block_mut(source)
            .unwrap()
            .set_terminator(MirInstruction::CheckedCallOut {
                site_id: site,
                receiver: ValueId::new(0),
                arguments: vec![],
                normal_landing: normal,
                fault_landing: fault,
                effects: EffectMask::READ,
            });
        for landing in [normal, fault] {
            function
                .get_block_mut(landing)
                .unwrap()
                .add_predecessor(source);
        }
        if with_projection {
            function.get_block_mut(normal).unwrap().add_instruction(
                MirInstruction::CheckedCallOutNormalResult {
                    site_id: site,
                    dst: ValueId::new(1),
                },
            );
        }
        let mut plans = CheckedCallOutPlanTableV1::default();
        plans
            .admit(CheckedCallOutSitePlanV1::from_test(
                site,
                CheckedCallOutEntryIdV1(17),
                CheckedCallOutNormalShapeV1::ImmediateI64,
                EffectMask::READ,
                ModuleInvocationBrandV1::legacy_test(),
            ))
            .unwrap();
        (function, plans)
    }

    #[test]
    fn plan_json_roundtrip_preserves_site_shape_and_stamp() {
        let plan = CheckedCallOutSitePlanV1::from_test(
            CheckedCallOutSiteIdV1(6),
            CheckedCallOutEntryIdV1(17),
            CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                lease_slot: CheckedCallOutLeaseSlotIdV1(1),
            },
            EffectMask::READ,
            ModuleInvocationBrandV1::legacy_test(),
        );
        let json = plan.to_json_for_test();
        let roundtrip =
            CheckedCallOutSitePlanV1::from_json_for_test(&json).expect("test-only JSON roundtrip");
        assert_eq!(roundtrip, plan);
        assert_eq!(json["site_id"], 6);
        assert_eq!(json["normal_shape"]["kind"], "end_authorized_handle");
        assert_eq!(json["plan_stamp"]["invocation_ordinal"], 1);
    }

    #[test]
    fn duplicate_site_and_wrong_effect_are_rejected() {
        let plan = CheckedCallOutSitePlanV1::from_test(
            CheckedCallOutSiteIdV1(7),
            CheckedCallOutEntryIdV1(18),
            CheckedCallOutNormalShapeV1::ImmediateI64,
            EffectMask::READ,
            ModuleInvocationBrandV1::legacy_test(),
        );
        let mut table = CheckedCallOutPlanTableV1::default();
        table.admit(plan.clone()).expect("first site");
        assert!(matches!(
            table.admit(plan),
            Err(CheckedCallOutPlanRejectV1::DuplicateSite(_))
        ));
        assert!(matches!(
            table
                .get(CheckedCallOutSiteIdV1(7))
                .unwrap()
                .validate_instruction(
                    CheckedCallOutSiteIdV1(7),
                    BasicBlockId::new(1),
                    BasicBlockId::new(2),
                    EffectMask::WRITE,
                ),
            Err(CheckedCallOutPlanRejectV1::EffectCacheMismatch)
        ));
    }

    #[test]
    fn non_aot_backends_reject_checked_callout_by_name() {
        let term = MirInstruction::CheckedCallOut {
            site_id: CheckedCallOutSiteIdV1(1),
            receiver: ValueId::new(0),
            arguments: vec![],
            normal_landing: BasicBlockId::new(1),
            fault_landing: BasicBlockId::new(2),
            effects: EffectMask::READ,
        };
        assert_eq!(
            crate::mir::contracts::backend_core_ops::instruction_tag(&term),
            "CheckedCallOut"
        );
        assert!(!crate::mir::contracts::backend_core_ops::is_supported_mir_json_terminator(&term));
        assert!(!crate::mir::contracts::backend_core_ops::is_supported_vm_terminator(&term));
        assert!(
            crate::mir::contracts::backend_core_ops::llvm_json_ops_for_instruction(&term)
                .is_empty()
        );
    }

    #[test]
    fn function_census_accepts_exact_plan_terminator_projection_triplet() {
        let (function, plans) = test_function_with_site(true);
        let verified = plans.verify_function(&function).expect("exact triplet");
        assert_eq!(verified.site_count(), 1);
    }

    #[test]
    fn function_census_rejects_orphan_projection_and_late_predecessor() {
        let (function, plans) = test_function_with_site(false);
        assert!(matches!(
            plans.verify_function(&function),
            Err(CheckedCallOutFunctionRejectV1::OrphanProjection(
                CheckedCallOutSiteIdV1(6)
            ))
        ));

        let (mut function, plans) = test_function_with_site(true);
        function
            .get_block_mut(BasicBlockId::new(1))
            .unwrap()
            .add_predecessor(BasicBlockId::new(9));
        assert!(matches!(
            plans.verify_function(&function),
            Err(CheckedCallOutFunctionRejectV1::LandingPredecessorMismatch(
                CheckedCallOutSiteIdV1(6)
            ))
        ));
    }

    #[test]
    fn admitted_text_scan_pair_is_typed_and_move_only() {
        let pair = CheckedCallOutSitePlanPairV1::from_admitted(
            CheckedCallOutAdmittedSiteInputV1 {
                entry: CheckedCallOutEntryIdV1(1),
                call_abi_revision: 1,
                wire_revision: 2,
                normal_shape: CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                    lease_slot: CheckedCallOutLeaseSlotIdV1(0),
                },
                effects: EffectMask::READ,
            },
            CheckedCallOutAdmittedSiteInputV1 {
                entry: CheckedCallOutEntryIdV1(2),
                call_abi_revision: 1,
                wire_revision: 2,
                normal_shape: CheckedCallOutNormalShapeV1::ImmediateI64,
                effects: EffectMask::READ,
            },
            ModuleInvocationBrandV1::legacy_test(),
        )
        .expect("exact TextScan pair");
        pair.consume(|i6, i7| {
            assert_eq!(i6.site_id(), CheckedCallOutSiteIdV1(0));
            assert_eq!(i7.site_id(), CheckedCallOutSiteIdV1(1));
            assert!(matches!(
                i6.normal_shape(),
                CheckedCallOutNormalShapeV1::EndAuthorizedHandle { .. }
            ));
            assert!(matches!(
                i7.normal_shape(),
                CheckedCallOutNormalShapeV1::ImmediateI64
            ));
        });
    }

    #[test]
    fn admitted_text_scan_pair_rejects_wrong_i7_shape() {
        let error = CheckedCallOutSitePlanPairV1::from_admitted(
            CheckedCallOutAdmittedSiteInputV1 {
                entry: CheckedCallOutEntryIdV1(1),
                call_abi_revision: 1,
                wire_revision: 2,
                normal_shape: CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                    lease_slot: CheckedCallOutLeaseSlotIdV1(0),
                },
                effects: EffectMask::READ,
            },
            CheckedCallOutAdmittedSiteInputV1 {
                entry: CheckedCallOutEntryIdV1(2),
                call_abi_revision: 1,
                wire_revision: 2,
                normal_shape: CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                    lease_slot: CheckedCallOutLeaseSlotIdV1(1),
                },
                effects: EffectMask::READ,
            },
            ModuleInvocationBrandV1::legacy_test(),
        );
        assert!(matches!(
            error,
            Err(CheckedCallOutSitePlanPairRejectV1::InvalidI7)
        ));
    }
}
