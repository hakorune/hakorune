//! Atomic source-order Loop claims with plan-order single-use consumption.

use std::collections::BTreeMap;

use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceStmtSiteV1};

use super::caller_ledger::ClaimedCallableResultActivationSiteV1;
use super::loop_claim_schedule::{
    CallableResultLoopClaimSchedulePartsV1, VerifiedCallableResultLoopClaimScheduleV1,
};
use super::{
    CallableResultCallerLedgerErrorV1, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultCallerLedgerV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableResultLoopClaimBatchErrorV1 {
    UnexpectedSite {
        site: SourceExprSiteV1,
    },
    AlreadyConsumed {
        site: SourceExprSiteV1,
    },
    Unconsumed {
        first: SourceExprSiteV1,
        remaining: usize,
    },
}

#[derive(Debug)]
enum LoopClaimSlotV1<'plan> {
    Available(ClaimedCallableResultActivationSiteV1<'plan>),
    Consumed,
}

/// Non-Clone post-claim batch. Source order is retained only for diagnostics.
#[derive(Debug)]
pub(crate) struct ClaimedCallableResultLoopBatchV1<'plan> {
    activation_plan: &'plan VerifiedCallableResultActivationPlanV1,
    caller: &'plan CanonicalSameModuleCallableKeyV1,
    loop_root: SourceStmtSiteV1,
    source_order: Box<[&'plan SourceExprSiteV1]>,
    claims_by_site: BTreeMap<SourceExprSiteV1, LoopClaimSlotV1<'plan>>,
}

impl<'plan> ClaimedCallableResultLoopBatchV1<'plan> {
    fn from_claimed_rows(
        parts: CallableResultLoopClaimSchedulePartsV1<'plan>,
        claims: Box<[ClaimedCallableResultActivationSiteV1<'plan>]>,
    ) -> Self {
        assert_eq!(
            parts.rows.len(),
            claims.len(),
            "sealed Loop schedule and claimed-token cardinality diverged"
        );
        let source_order = parts.rows.iter().map(|row| row.site()).collect();
        let mut claims_by_site = BTreeMap::new();
        for claim in claims.into_vec() {
            let previous =
                claims_by_site.insert(claim.site().clone(), LoopClaimSlotV1::Available(claim));
            assert!(
                previous.is_none(),
                "sealed Loop claim site became duplicate"
            );
        }
        Self {
            activation_plan: parts.activation_plan,
            caller: parts.caller,
            loop_root: parts.loop_root,
            source_order,
            claims_by_site,
        }
    }

    pub(crate) fn take_claim(
        &mut self,
        site: &SourceExprSiteV1,
    ) -> Result<ClaimedCallableResultActivationSiteV1<'plan>, CallableResultLoopClaimBatchErrorV1>
    {
        let Some(slot) = self.claims_by_site.get_mut(site) else {
            return Err(CallableResultLoopClaimBatchErrorV1::UnexpectedSite { site: site.clone() });
        };
        match std::mem::replace(slot, LoopClaimSlotV1::Consumed) {
            LoopClaimSlotV1::Available(claim) => Ok(claim),
            LoopClaimSlotV1::Consumed => {
                Err(CallableResultLoopClaimBatchErrorV1::AlreadyConsumed { site: site.clone() })
            }
        }
    }

    pub(crate) fn finish(self) -> Result<(), CallableResultLoopClaimBatchErrorV1> {
        let Some(first) = self.source_order.iter().find(|site| {
            matches!(
                self.claims_by_site.get(**site),
                Some(LoopClaimSlotV1::Available(_))
            )
        }) else {
            return Ok(());
        };
        let remaining = self
            .claims_by_site
            .values()
            .filter(|slot| matches!(slot, LoopClaimSlotV1::Available(_)))
            .count();
        Err(CallableResultLoopClaimBatchErrorV1::Unconsumed {
            first: (*first).clone(),
            remaining,
        })
    }

    pub(crate) const fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        self.caller
    }

    pub(crate) const fn loop_root(&self) -> &SourceStmtSiteV1 {
        &self.loop_root
    }

    pub(crate) fn is_branded_by(
        &self,
        activation_plan: &VerifiedCallableResultActivationPlanV1,
    ) -> bool {
        std::ptr::eq(self.activation_plan, activation_plan)
    }
}

impl<'plan> VerifiedCallableResultCallerLedgerV1<'plan> {
    pub(crate) fn claim_loop_batch(
        &mut self,
        schedule: VerifiedCallableResultLoopClaimScheduleV1<'plan>,
    ) -> Result<ClaimedCallableResultLoopBatchV1<'plan>, CallableResultCallerLedgerErrorV1> {
        let parts = schedule.into_claim_parts();
        let claims = self.prevalidate_and_commit_loop_schedule(&parts)?;
        Ok(ClaimedCallableResultLoopBatchV1::from_claimed_rows(
            parts, claims,
        ))
    }
}
