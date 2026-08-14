//! Neutral MIR vocabulary for a checked call with canonical Normal/Fault CFG.
//!
//! This module is deliberately physical-only.  It does not resolve a provider,
//! selector, runtime lease token, or backend function address.  A function-local
//! site plan is admitted once and the canonical CFG/SSA sessions consume it.

use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::{BasicBlockId, EffectMask, MirFunction};
use std::collections::BTreeMap;

use super::census::{
    verify_checked_callout_function_v1, CheckedCallOutFunctionRejectV1,
    VerifiedCheckedCallOutFunctionV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CheckedCallOutSiteIdV1(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CheckedCallOutEntryIdV1(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CheckedCallOutOutcomeSlotIdV1(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CheckedCallOutLeaseSlotIdV1(u32);

macro_rules! checked_callout_id_api {
    ($id:ident) => {
        impl $id {
            /// Decode an ID carried by the neutral MIR transport.  This is
            /// parsing, not a new semantic/physical issuer.
            pub(crate) const fn from_wire(value: u32) -> Self {
                Self(value)
            }

            #[cfg(test)]
            pub(crate) const fn from_test(value: u32) -> Self {
                Self(value)
            }

            pub(crate) const fn as_u32(self) -> u32 {
                self.0
            }
        }
    };
}

checked_callout_id_api!(CheckedCallOutSiteIdV1);
checked_callout_id_api!(CheckedCallOutEntryIdV1);
checked_callout_id_api!(CheckedCallOutOutcomeSlotIdV1);
checked_callout_id_api!(CheckedCallOutLeaseSlotIdV1);

impl CheckedCallOutEntryIdV1 {
    /// Project an already-admitted provider entry into the neutral MIR type.
    /// The provider admission remains the identity authority.
    pub(crate) const fn from_admitted(value: u32) -> Self {
        Self(value)
    }
}

impl CheckedCallOutLeaseSlotIdV1 {
    /// Project the lease slot selected by the admitted TextScan ABI shape.
    /// The bounded cohort admits only slot zero; the pair issuer still owns
    /// the resulting site/outcome identities.
    pub(crate) const fn from_admitted(value: u32) -> Self {
        Self(value)
    }
}

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
pub(crate) struct CheckedCallOutAdmittedSiteInputV1 {
    pub(crate) entry: CheckedCallOutEntryIdV1,
    pub(crate) call_abi_revision: u32,
    pub(crate) wire_revision: u32,
    pub(crate) normal_shape: CheckedCallOutNormalShapeV1,
    pub(crate) effects: EffectMask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CheckedCallOutSitePlanPairRejectV1 {
    InvalidI6,
    InvalidI7,
    NonDistinctEntry,
    PlanStampMismatch,
}

/// Exactly the two admitted TextScan call plans.  It cannot be cloned or
/// split; the selected session consumes it into its function-local plan table.
#[derive(Debug)]
pub(crate) struct CheckedCallOutSitePlanPairV1 {
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
            outcome_slot: CheckedCallOutOutcomeSlotIdV1::from_test(site_id.as_u32()),
            plan_stamp,
        }
    }

    pub(crate) const fn site_id(&self) -> CheckedCallOutSiteIdV1 {
        self.site_id
    }

    pub(crate) const fn admitted_entry(&self) -> CheckedCallOutEntryIdV1 {
        self.admitted_entry
    }

    pub(crate) const fn call_abi_revision(&self) -> u32 {
        self.call_abi_revision
    }

    pub(crate) const fn wire_revision(&self) -> u32 {
        self.wire_revision
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
                serde_json::json!({"kind":"end_authorized_handle","lease_slot":lease_slot.as_u32()})
            }
            CheckedCallOutNormalShapeV1::ImmediateI64 => {
                serde_json::json!({"kind":"immediate_i64"})
            }
        };
        serde_json::json!({
            "site_id": self.site_id.as_u32(),
            "admitted_entry": self.admitted_entry.as_u32(),
            "call_abi_revision": self.call_abi_revision,
            "wire_revision": self.wire_revision,
            "normal_shape": shape,
            "effects": self.effects.bits(),
            "outcome_slot": self.outcome_slot.as_u32(),
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
        let site_id = CheckedCallOutSiteIdV1::from_test(
            u32::try_from(number("site_id")?).map_err(|_| "site id overflow".to_owned())?,
        );
        let admitted_entry = CheckedCallOutEntryIdV1::from_test(
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
                lease_slot: CheckedCallOutLeaseSlotIdV1::from_test(
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
        let outcome_slot = CheckedCallOutOutcomeSlotIdV1::from_test(
            u32::try_from(number("outcome_slot")?)
                .map_err(|_| "outcome slot overflow".to_owned())?,
        );
        let compiler_domain = value["plan_stamp"]["compiler_domain"]
            .as_u64()
            .ok_or_else(|| "missing compiler domain".to_owned())?;
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
            plan_stamp: ModuleInvocationBrandV1::try_test_with_parts(compiler_domain, ordinal)
                .map_err(str::to_owned)?,
        })
    }
}

impl CheckedCallOutSitePlanPairV1 {
    /// Borrow the canonical site identity for an already-admitted entry.
    ///
    /// The pair is the only owner of the entry -> site projection.  AOT
    /// transport consumers must borrow this mapping; they must not infer a
    /// site from a block, instruction index, selector, or role spelling.
    pub(crate) fn site_id_for_entry(
        &self,
        entry: CheckedCallOutEntryIdV1,
    ) -> Option<CheckedCallOutSiteIdV1> {
        if self.i6.admitted_entry == entry {
            Some(self.i6.site_id)
        } else if self.i7.admitted_entry == entry {
            Some(self.i7.site_id)
        } else {
            None
        }
    }

    pub(crate) fn from_admitted(
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
    pub(in crate::mir) fn with_sites<R>(
        &self,
        callback: impl FnOnce(&CheckedCallOutSitePlanV1, &CheckedCallOutSitePlanV1) -> R,
    ) -> R {
        callback(&self.i6, &self.i7)
    }

    /// Consume the pair without exposing a re-pairing/parts API.
    pub(in crate::mir) fn consume<R>(
        self,
        callback: impl FnOnce(CheckedCallOutSitePlanV1, CheckedCallOutSitePlanV1) -> R,
    ) -> R {
        callback(self.i6, self.i7)
    }

    #[cfg(test)]
    pub(crate) fn into_plan_table_for_test(self) -> CheckedCallOutPlanTableV1 {
        self.consume(|i6, i7| {
            let mut table = CheckedCallOutPlanTableV1::default();
            table.admit(i6).expect("test I6 plan");
            table.admit(i7).expect("test I7 plan");
            table
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

    /// Borrow the canonical entry -> site mapping after the session has
    /// installed the exact function-local plan table.  Consumers may not
    /// reconstruct a second site-plan pair from entry names or coordinates.
    pub(crate) fn site_id_for_entry(
        &self,
        entry: CheckedCallOutEntryIdV1,
    ) -> Option<CheckedCallOutSiteIdV1> {
        self.plans
            .values()
            .find_map(|plan| (plan.admitted_entry() == entry).then_some(plan.site_id()))
    }

    pub(crate) fn plan_for_entry(
        &self,
        entry: CheckedCallOutEntryIdV1,
    ) -> Option<&CheckedCallOutSitePlanV1> {
        self.plans
            .values()
            .find(|plan| plan.admitted_entry() == entry)
    }

    pub(super) fn contains_site(&self, site: CheckedCallOutSiteIdV1) -> bool {
        self.plans.contains_key(&site)
    }

    pub(super) fn iter(
        &self,
    ) -> impl Iterator<Item = (&CheckedCallOutSiteIdV1, &CheckedCallOutSitePlanV1)> {
        self.plans.iter()
    }

    pub(crate) fn len(&self) -> usize {
        self.plans.len()
    }

    pub(crate) fn verify_function(
        &self,
        function: &MirFunction,
    ) -> Result<VerifiedCheckedCallOutFunctionV1, CheckedCallOutFunctionRejectV1> {
        verify_checked_callout_function_v1(function, self)
    }
}
