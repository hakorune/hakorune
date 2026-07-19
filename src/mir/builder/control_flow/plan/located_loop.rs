//! Disconnected non-Clone seals for one final located Core Loop plan and its
//! single-use execution session.

use std::collections::BTreeMap;

use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::{
    ClaimedCallableResultLoopBatchV1, LegacyStmtInputV1, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultCallerLedgerV1, VerifiedCallableResultLoopClaimScheduleV1,
};

use super::{
    visit_core_call_sources_v1, CoreCallSourceV1, CorePlan, LocatedCoreLoopPlanErrorV1,
    PlanVerifier,
};

#[derive(Debug)]
struct LocatedCoreLoopPlanSealV1;

#[derive(Debug)]
struct ClaimedLocatedCoreLoopExecutionSealV1;

#[derive(Debug)]
struct LocatedCoreLoopExecutionSessionSealV1;

/// Typed failure vocabulary for the claimed CorePlan execution session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum LocatedCoreLoopExecutionSessionErrorV1 {
    ClaimBatch(String),
    Lowering(String),
    Unexecuted,
    Poisoned,
    AlreadyCompleted,
}

/// Owns one completed, verified, already-remapped Core Loop plan.
#[derive(Debug)]
pub(in crate::mir::builder) struct VerifiedLocatedCoreLoopPlanV1<'plan> {
    plan: CorePlan,
    schedule: VerifiedCallableResultLoopClaimScheduleV1<'plan>,
    _seal: LocatedCoreLoopPlanSealV1,
}

/// One claimed, single-use execution bundle for a sealed located Loop plan.
///
/// It owns both the already-remapped CorePlan and the source-order claims, so
/// neither can be paired with another plan or emitted twice.
#[derive(Debug)]
struct ClaimedLocatedCoreLoopExecutionV1<'plan> {
    plan: CorePlan,
    claims: ClaimedCallableResultLoopBatchV1<'plan>,
    _seal: ClaimedLocatedCoreLoopExecutionSealV1,
}

#[derive(Debug)]
enum LocatedCoreLoopExecutionStateV1<'plan> {
    Active(ClaimedLocatedCoreLoopExecutionV1<'plan>),
    Poisoned,
    Completed,
}

/// Stack-scoped state owner for one claimed CorePlan loop execution.
///
/// The session owns the already-claimed bundle.  It has no source-selection,
/// claim-creation, or route-selection capability: lowering consumes the bundle
/// once, then commits only the session state to `Completed` or `Poisoned`.
#[derive(Debug)]
pub(in crate::mir::builder) struct LocatedCoreLoopExecutionSessionV1<'plan> {
    state: LocatedCoreLoopExecutionStateV1<'plan>,
    _seal: LocatedCoreLoopExecutionSessionSealV1,
}

impl<'plan> VerifiedLocatedCoreLoopPlanV1<'plan> {
    pub(in crate::mir::builder) fn verify(
        plan: CorePlan,
        activation_plan: &'plan VerifiedCallableResultActivationPlanV1,
        caller: &CanonicalSameModuleCallableKeyV1,
        loop_statement: LegacyStmtInputV1<'plan>,
    ) -> Result<Self, LocatedCoreLoopPlanErrorV1> {
        if !matches!(plan, CorePlan::Loop(_)) {
            return Err(LocatedCoreLoopPlanErrorV1::ExpectedLoopPlan);
        }
        PlanVerifier::verify(&plan).map_err(LocatedCoreLoopPlanErrorV1::PlanVerification)?;
        let schedule = VerifiedCallableResultLoopClaimScheduleV1::verify(
            activation_plan,
            caller,
            loop_statement,
        )
        .map_err(LocatedCoreLoopPlanErrorV1::ClaimSchedule)?;

        let mut occurrences = BTreeMap::new();
        visit_core_call_sources_v1(&plan, &mut |source| {
            if let CoreCallSourceV1::LocatedMethodCall(site) = source {
                *occurrences.entry(site.clone()).or_insert(0usize) += 1;
            }
        });

        for site in schedule.sites_in_source_order() {
            match occurrences.remove(site) {
                None => {
                    return Err(LocatedCoreLoopPlanErrorV1::MissingLocatedOccurrence(
                        site.clone(),
                    ));
                }
                Some(count) if count > 1 => {
                    return Err(LocatedCoreLoopPlanErrorV1::DuplicateLocatedOccurrence(
                        site.clone(),
                    ));
                }
                Some(_) => {}
            }
        }
        if let Some((site, _)) = occurrences.into_iter().next() {
            return Err(LocatedCoreLoopPlanErrorV1::UnexpectedLocatedOccurrence(
                site,
            ));
        }

        Ok(Self {
            plan,
            schedule,
            _seal: LocatedCoreLoopPlanSealV1,
        })
    }

    pub(in crate::mir::builder) const fn schedule(
        &self,
    ) -> &VerifiedCallableResultLoopClaimScheduleV1<'plan> {
        &self.schedule
    }

    pub(in crate::mir::builder) fn plan_is_loop(&self) -> bool {
        matches!(self.plan, CorePlan::Loop(_))
    }

    /// Atomically acquires this plan's complete source-order claim batch.
    ///
    /// A failed claim leaves the ledger unchanged and drops the plan without
    /// invoking PlanLowerer.  The method consumes the seal, preventing a
    /// second schedule/plan pairing or retry from the same selected session.
    fn claim_execution(
        self,
        ledger: &mut VerifiedCallableResultCallerLedgerV1<'plan>,
    ) -> Result<ClaimedLocatedCoreLoopExecutionV1<'plan>, String> {
        let claims = ledger.claim_loop_batch(self.schedule).map_err(|error| {
            format!("[freeze:contract][callable_result/loop_claim_batch] {error:?}")
        })?;
        Ok(ClaimedLocatedCoreLoopExecutionV1 {
            plan: self.plan,
            claims,
            _seal: ClaimedLocatedCoreLoopExecutionSealV1,
        })
    }

    /// Consumes the verified plan and atomically claims its complete source
    /// schedule before creating the only CorePlan execution-state owner.
    pub(in crate::mir::builder) fn start_execution(
        self,
        ledger: &mut VerifiedCallableResultCallerLedgerV1<'plan>,
    ) -> Result<LocatedCoreLoopExecutionSessionV1<'plan>, LocatedCoreLoopExecutionSessionErrorV1>
    {
        self.claim_execution(ledger)
            .map(LocatedCoreLoopExecutionSessionV1::from_claimed)
            .map_err(LocatedCoreLoopExecutionSessionErrorV1::ClaimBatch)
    }

    #[cfg(test)]
    pub(in crate::mir::builder) const fn plan_for_tests(&self) -> &CorePlan {
        &self.plan
    }
}

impl<'plan> ClaimedLocatedCoreLoopExecutionV1<'plan> {
    /// Lowers once through the shared effect-emission port and then proves
    /// exact consumption of every prepared claim.  Lowering failure intentionally
    /// does not finish the batch: the caller owns poisoning of that session.
    pub(in crate::mir::builder) fn lower(
        self,
        builder: &mut crate::mir::builder::MirBuilder,
        ctx: &crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        let mut port =
            super::lowerer::emission_port::CorePlanEffectEmissionPortV1::claimed(self.claims);
        let lowered =
            super::PlanLowerer::lower_with_emission_port(builder, self.plan, ctx, &mut port)?;
        port.finish()?;
        Ok(lowered)
    }
}

impl<'plan> LocatedCoreLoopExecutionSessionV1<'plan> {
    fn from_claimed(execution: ClaimedLocatedCoreLoopExecutionV1<'plan>) -> Self {
        Self {
            state: LocatedCoreLoopExecutionStateV1::Active(execution),
            _seal: LocatedCoreLoopExecutionSessionSealV1,
        }
    }

    /// Lowers the claimed bundle exactly once.
    ///
    /// The bundle is removed before calling the lowerer, so a failure cannot
    /// expose a retry path.  The existing emission port remains the only owner
    /// of exact prepared-claim consumption.
    pub(in crate::mir::builder) fn lower_once(
        &mut self,
        builder: &mut crate::mir::builder::MirBuilder,
        ctx: &crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext,
    ) -> Result<Option<crate::mir::ValueId>, LocatedCoreLoopExecutionSessionErrorV1> {
        let state = std::mem::replace(&mut self.state, LocatedCoreLoopExecutionStateV1::Poisoned);
        match state {
            LocatedCoreLoopExecutionStateV1::Active(execution) => {
                match execution.lower(builder, ctx) {
                    Ok(value) => {
                        self.state = LocatedCoreLoopExecutionStateV1::Completed;
                        Ok(value)
                    }
                    Err(error) => Err(LocatedCoreLoopExecutionSessionErrorV1::Lowering(error)),
                }
            }
            LocatedCoreLoopExecutionStateV1::Poisoned => {
                self.state = LocatedCoreLoopExecutionStateV1::Poisoned;
                Err(LocatedCoreLoopExecutionSessionErrorV1::Poisoned)
            }
            LocatedCoreLoopExecutionStateV1::Completed => {
                self.state = LocatedCoreLoopExecutionStateV1::Completed;
                Err(LocatedCoreLoopExecutionSessionErrorV1::AlreadyCompleted)
            }
        }
    }

    /// Acknowledges successful single-use execution.
    ///
    /// `ClaimedLocatedCoreLoopExecutionV1::lower` already calls the emission
    /// port's `finish`; this method seals this session's state only and never
    /// creates a second claim or ledger completion path.
    pub(in crate::mir::builder) fn finish(
        self,
    ) -> Result<(), LocatedCoreLoopExecutionSessionErrorV1> {
        match self.state {
            LocatedCoreLoopExecutionStateV1::Completed => Ok(()),
            LocatedCoreLoopExecutionStateV1::Active(_) => {
                Err(LocatedCoreLoopExecutionSessionErrorV1::Unexecuted)
            }
            LocatedCoreLoopExecutionStateV1::Poisoned => {
                Err(LocatedCoreLoopExecutionSessionErrorV1::Poisoned)
            }
        }
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn is_active_for_tests(&self) -> bool {
        matches!(self.state, LocatedCoreLoopExecutionStateV1::Active(_))
    }
}
