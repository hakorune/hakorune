//! Neutral MIR vocabulary for a checked call with canonical Normal/Fault CFG.
//!
//! This module is deliberately physical-only.  It does not resolve a provider,
//! selector, runtime lease token, or backend function address.  A function-local
//! site plan is admitted once and the canonical CFG/SSA sessions consume it.

use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::{BasicBlockId, EffectMask, ValueId};
use std::collections::BTreeMap;

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
    use crate::mir::MirInstruction;

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
}
