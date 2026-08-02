//! Pure left-to-right evaluation of frozen Loop policy evidence.
//!
//! The evaluator does not inspect route IDs or cursors. Declined evidence is
//! consumed internally; only Qualified, Blocked, or Exhausted escape.

use super::policy_evidence::{
    LoopGenericDebtKeyV1, LoopRouteCandidateFactsV1, LoopRoutePolicyBlockReasonV1,
    LoopRoutePolicyEvidenceV1, LoopRoutePolicySourceDeclineReasonV1,
};
use super::schema::{
    FrozenLoopRouteScheduleV1, LoopRouteSourceDispositionV1, LoopRouteSuppressionDispositionV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LoopQualifiedV1 {
    facts: LoopRouteCandidateFactsV1,
    seal: LoopQualifiedSealV1,
}

impl LoopQualifiedV1 {
    pub(crate) fn facts(&self) -> LoopRouteCandidateFactsV1 {
        self.facts
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LoopQualifiedSealV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopPolicyBlockedReasonV1 {
    Policy(LoopRoutePolicyBlockReasonV1),
    GenericDebt(LoopGenericDebtKeyV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LoopRoutePolicyEvaluationV1 {
    Qualified(LoopQualifiedV1),
    Blocked(LoopPolicyBlockedReasonV1),
    Exhausted,
}

pub(crate) fn evaluate_frozen_loop_route_schedule_v1(
    schedule: &FrozenLoopRouteScheduleV1,
) -> LoopRoutePolicyEvaluationV1 {
    for row in schedule.rows() {
        if matches!(
            row.suppression(),
            LoopRouteSuppressionDispositionV1::SuppressedBy(_)
        ) {
            continue;
        }
        if matches!(row.source(), LoopRouteSourceDispositionV1::Unavailable(_)) {
            continue;
        }
        match row.policy_evidence() {
            LoopRoutePolicyEvidenceV1::SourceDeclined(reason) => {
                if matches!(
                    reason,
                    LoopRoutePolicySourceDeclineReasonV1::SuppressedByEarlierCandidate
                        | LoopRoutePolicySourceDeclineReasonV1::Unavailable(_)
                ) {
                    continue;
                }
            }
            LoopRoutePolicyEvidenceV1::Candidate(facts) => {
                return LoopRoutePolicyEvaluationV1::Qualified(LoopQualifiedV1 {
                    facts,
                    seal: LoopQualifiedSealV1,
                });
            }
            LoopRoutePolicyEvidenceV1::PolicyBlocked(reason) => {
                return LoopRoutePolicyEvaluationV1::Blocked(LoopPolicyBlockedReasonV1::Policy(
                    reason,
                ));
            }
            LoopRoutePolicyEvidenceV1::GenericDebt(key) => {
                return LoopRoutePolicyEvaluationV1::Blocked(
                    LoopPolicyBlockedReasonV1::GenericDebt(key),
                );
            }
        }
    }
    LoopRoutePolicyEvaluationV1::Exhausted
}

#[cfg(test)]
mod tests {
    use super::{
        evaluate_frozen_loop_route_schedule_v1, LoopPolicyBlockedReasonV1,
        LoopRoutePolicyEvaluationV1,
    };
    use crate::mir::loop_route_policy::schema::{
        FrozenLoopRouteObservationV1, LoopGlobalEntryDispositionV1, LoopModeReleaseSnapshotV1,
        LoopReleaseAdmissionObservationV1, LoopRouteSourceDispositionV1,
        LoopRouteSuppressionDispositionV1, CANONICAL_LOOP_ROUTE_ORDER_V1,
    };
    use crate::mir::loop_route_policy::{
        LoopGenericDebtKeyV1, LoopRouteCandidateFactsV1, LoopRoutePolicyBlockReasonV1,
        LoopRoutePolicyEvidenceV1, LoopRoutePolicySourceDeclineReasonV1,
    };

    fn observations_with(
        evidence_at: usize,
        evidence: LoopRoutePolicyEvidenceV1,
    ) -> Box<[FrozenLoopRouteObservationV1]> {
        CANONICAL_LOOP_ROUTE_ORDER_V1
            .iter()
            .enumerate()
            .map(|(cursor, _)| {
                let evidence = if cursor == evidence_at {
                    evidence
                } else {
                    LoopRoutePolicyEvidenceV1::SourceDeclined(
                        LoopRoutePolicySourceDeclineReasonV1::SuppressedByEarlierCandidate,
                    )
                };
                FrozenLoopRouteObservationV1::new(
                    LoopRouteSuppressionDispositionV1::Retained,
                    LoopModeReleaseSnapshotV1::Release {
                        admission: LoopReleaseAdmissionObservationV1::Allowed,
                    },
                    LoopGlobalEntryDispositionV1::Allowed,
                    LoopRouteSourceDispositionV1::Available,
                    evidence,
                )
            })
            .collect()
    }

    fn schedule(
        evidence_at: usize,
        evidence: LoopRoutePolicyEvidenceV1,
    ) -> super::FrozenLoopRouteScheduleV1 {
        super::super::evaluate::freeze_loop_route_schedule_v1(
            CANONICAL_LOOP_ROUTE_ORDER_V1.into(),
            observations_with(evidence_at, evidence),
        )
        .expect("synthetic policy evidence seals")
    }

    #[test]
    fn all_declined_rows_exhaust_without_resume_state() {
        let schedule = schedule(
            0,
            LoopRoutePolicyEvidenceV1::SourceDeclined(
                LoopRoutePolicySourceDeclineReasonV1::SuppressedByEarlierCandidate,
            ),
        );
        assert_eq!(
            evaluate_frozen_loop_route_schedule_v1(&schedule),
            LoopRoutePolicyEvaluationV1::Exhausted
        );
    }

    #[test]
    fn declined_then_candidate_stops_at_first_candidate() {
        let schedule = schedule(
            3,
            LoopRoutePolicyEvidenceV1::Candidate(LoopRouteCandidateFactsV1::SourceAvailable),
        );
        assert!(matches!(
            evaluate_frozen_loop_route_schedule_v1(&schedule),
            LoopRoutePolicyEvaluationV1::Qualified(_)
        ));
    }

    #[test]
    fn declined_then_blocked_stops_without_suffix() {
        let schedule = schedule(
            4,
            LoopRoutePolicyEvidenceV1::PolicyBlocked(
                LoopRoutePolicyBlockReasonV1::PolicyAndTerminalityUnavailable,
            ),
        );
        assert!(matches!(
            evaluate_frozen_loop_route_schedule_v1(&schedule),
            LoopRoutePolicyEvaluationV1::Blocked(LoopPolicyBlockedReasonV1::Policy(
                LoopRoutePolicyBlockReasonV1::PolicyAndTerminalityUnavailable
            ))
        ));
    }

    #[test]
    fn generic_debt_is_an_opaque_blocked_m4_key() {
        let schedule = schedule(
            17,
            LoopRoutePolicyEvidenceV1::GenericDebt(LoopGenericDebtKeyV1::GenericPostEffectDebt),
        );
        assert!(matches!(
            evaluate_frozen_loop_route_schedule_v1(&schedule),
            LoopRoutePolicyEvaluationV1::Blocked(LoopPolicyBlockedReasonV1::GenericDebt(
                LoopGenericDebtKeyV1::GenericPostEffectDebt
            ))
        ));
    }
}
