//! Disconnected non-Clone seal for one final located Core Loop plan.

use std::collections::BTreeMap;

use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::{
    LegacyStmtInputV1, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultLoopClaimScheduleV1,
};

use super::{
    visit_core_call_sources_v1, CoreCallSourceV1, CorePlan, LocatedCoreLoopPlanErrorV1,
    PlanVerifier,
};

#[derive(Debug)]
struct LocatedCoreLoopPlanSealV1;

/// Owns one completed, verified, already-remapped Core Loop plan.
#[derive(Debug)]
pub(in crate::mir::builder) struct VerifiedLocatedCoreLoopPlanV1<'plan> {
    plan: CorePlan,
    schedule: VerifiedCallableResultLoopClaimScheduleV1<'plan>,
    _seal: LocatedCoreLoopPlanSealV1,
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

    #[cfg(test)]
    pub(in crate::mir::builder) const fn plan_for_tests(&self) -> &CorePlan {
        &self.plan
    }
}
