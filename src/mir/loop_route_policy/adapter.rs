//! Test-only migration fixture adapter.
//!
//! This module is test-only. Its fixture path stays registry-free; the M3-F
//! parity submodule is the sole test adapter allowed to invoke the legacy
//! execution witness and compare opaque route provenance.

use super::{
    freeze_loop_route_schedule_v1, FrozenLoopRouteObservationV1, FrozenLoopRouteScheduleRejectV1,
    FrozenLoopRouteScheduleV1, LoopGlobalEntryDispositionV1, LoopModeReleaseSnapshotV1,
    LoopReleaseAdmissionObservationV1, LoopRouteCandidateFactsV1, LoopRoutePolicyEvidenceV1,
    LoopRouteSourceDispositionV1, LoopRouteSuppressionDispositionV1, CANONICAL_LOOP_ROUTE_ORDER_V1,
};

pub(super) fn freeze_canonical_loop_route_schedule_fixture_v1(
) -> Result<FrozenLoopRouteScheduleV1, FrozenLoopRouteScheduleRejectV1> {
    let observations = CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .map(|_| {
            FrozenLoopRouteObservationV1::new(
                LoopRouteSuppressionDispositionV1::Retained,
                LoopModeReleaseSnapshotV1::Release {
                    admission: LoopReleaseAdmissionObservationV1::Allowed,
                },
                LoopGlobalEntryDispositionV1::Allowed,
                LoopRouteSourceDispositionV1::Available,
                LoopRoutePolicyEvidenceV1::Candidate(LoopRouteCandidateFactsV1::SourceAvailable),
            )
        })
        .collect::<Box<[_]>>();
    freeze_loop_route_schedule_v1(CANONICAL_LOOP_ROUTE_ORDER_V1.into(), observations)
}

#[cfg(test)]
mod parity_tests {
    use super::super::{
        evaluate_frozen_loop_route_schedule_v1, freeze_loop_route_schedule_v1,
        FrozenLoopRouteObservationV1, LoopGlobalEntryDispositionV1, LoopModeReleaseSnapshotV1,
        LoopReleaseAdmissionObservationV1, LoopRouteCandidateFactsV1, LoopRoutePolicyBlockReasonV1,
        LoopRoutePolicyEvaluationV1, LoopRoutePolicyEvidenceV1,
        LoopRoutePolicySourceDeclineReasonV1, LoopRouteSourceDispositionV1,
        LoopRouteSuppressionDispositionV1, CANONICAL_LOOP_ROUTE_ORDER_V1,
    };
    use crate::mir::builder::{
        execute_legacy_policy_parity_v1, LegacyPolicyAttemptDispositionV1,
        LegacyPolicyParityReceiptV1,
    };
    use crate::mir::loop_recipe_contract::route_id::LoopRouteId;

    /// Canonical row-zero fixture with an explicit typed pre-effect decline.
    /// The order is the production scheduler's frozen order; no AST/Builder
    /// is needed for this policy parity proof.
    fn ordered_decline_then_success_schedule() -> super::super::FrozenLoopRouteScheduleV1 {
        let observations = CANONICAL_LOOP_ROUTE_ORDER_V1
            .iter()
            .enumerate()
            .map(|(cursor, _)| {
                let evidence = if cursor == 5 {
                    LoopRoutePolicyEvidenceV1::Candidate(LoopRouteCandidateFactsV1::SourceAvailable)
                } else if cursor < 5 {
                    LoopRoutePolicyEvidenceV1::SourceDeclined(
                        LoopRoutePolicySourceDeclineReasonV1::PreEffectDeclined,
                    )
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
            .collect::<Box<[_]>>();
        freeze_loop_route_schedule_v1(CANONICAL_LOOP_ROUTE_ORDER_V1.into(), observations)
            .expect("ordered parity fixture seals")
    }

    #[test]
    fn legacy_decline_then_success_matches_pure_policy_winner() {
        let schedule = ordered_decline_then_success_schedule();
        let LoopRoutePolicyEvaluationV1::Qualified(_) =
            evaluate_frozen_loop_route_schedule_v1(&schedule)
        else {
            panic!("decline-then-success fixture must qualify");
        };
        let pure_cursor = 5;
        let dispositions = CANONICAL_LOOP_ROUTE_ORDER_V1
            .iter()
            .copied()
            .map(|route| {
                if route == LoopRouteId::LoopCharMap {
                    LegacyPolicyAttemptDispositionV1::Succeeded
                } else {
                    LegacyPolicyAttemptDispositionV1::PreEffectDeclined
                }
            })
            .collect::<Vec<_>>();
        let legacy = execute_legacy_policy_parity_v1(&CANONICAL_LOOP_ROUTE_ORDER_V1, &dispositions)
            .expect("legacy witness parity oracle succeeds");

        let expected_route = schedule.rows()[pure_cursor].route_id();
        assert_eq!(pure_cursor, 5);
        assert_eq!(expected_route, LoopRouteId::LoopCharMap);
        assert_eq!(
            legacy,
            LegacyPolicyParityReceiptV1::Succeeded {
                route: expected_route,
                attempted: CANONICAL_LOOP_ROUTE_ORDER_V1[..=pure_cursor].into(),
            }
        );
    }

    #[test]
    fn pure_policy_exhaustion_matches_legacy_exhaustion() {
        let observations = CANONICAL_LOOP_ROUTE_ORDER_V1
            .iter()
            .map(|_| {
                FrozenLoopRouteObservationV1::new(
                    LoopRouteSuppressionDispositionV1::Retained,
                    LoopModeReleaseSnapshotV1::Release {
                        admission: LoopReleaseAdmissionObservationV1::Allowed,
                    },
                    LoopGlobalEntryDispositionV1::Allowed,
                    LoopRouteSourceDispositionV1::Available,
                    LoopRoutePolicyEvidenceV1::SourceDeclined(
                        LoopRoutePolicySourceDeclineReasonV1::SuppressedByEarlierCandidate,
                    ),
                )
            })
            .collect::<Box<[_]>>();
        let schedule =
            freeze_loop_route_schedule_v1(CANONICAL_LOOP_ROUTE_ORDER_V1.into(), observations)
                .expect("all-declined parity fixture seals");
        let dispositions = CANONICAL_LOOP_ROUTE_ORDER_V1
            .iter()
            .copied()
            .map(|_| LegacyPolicyAttemptDispositionV1::PreEffectDeclined)
            .collect::<Vec<_>>();

        assert_eq!(
            evaluate_frozen_loop_route_schedule_v1(&schedule),
            LoopRoutePolicyEvaluationV1::Exhausted
        );
        assert_eq!(
            execute_legacy_policy_parity_v1(&CANONICAL_LOOP_ROUTE_ORDER_V1, &dispositions)
                .expect("legacy witness exhausts"),
            LegacyPolicyParityReceiptV1::Exhausted {
                attempted: CANONICAL_LOOP_ROUTE_ORDER_V1.into(),
            }
        );
    }

    #[test]
    fn blocked_policy_stops_on_a_fresh_row_zero_schedule() {
        let observations = CANONICAL_LOOP_ROUTE_ORDER_V1
            .iter()
            .enumerate()
            .map(|(cursor, _)| {
                let evidence = if cursor == 0 {
                    LoopRoutePolicyEvidenceV1::PolicyBlocked(
                        LoopRoutePolicyBlockReasonV1::ReleaseNestedLoopGate,
                    )
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
            .collect::<Box<[_]>>();
        let schedule =
            freeze_loop_route_schedule_v1(CANONICAL_LOOP_ROUTE_ORDER_V1.into(), observations)
                .expect("fresh blocked row-zero fixture seals");

        assert_eq!(schedule.first().raw_cursor(), 0);
        assert_eq!(
            evaluate_frozen_loop_route_schedule_v1(&schedule),
            LoopRoutePolicyEvaluationV1::Blocked(super::super::LoopPolicyBlockedReasonV1::Policy(
                LoopRoutePolicyBlockReasonV1::ReleaseNestedLoopGate,
            ))
        );
    }
}
