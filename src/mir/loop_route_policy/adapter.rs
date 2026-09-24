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
            evaluate_frozen_loop_route_schedule_v1(
                &schedule,
                &crate::mir::resolved_semantics::loop_execution_frame_key_for_test(),
            ),
            LoopRoutePolicyEvaluationV1::Blocked(super::super::LoopPolicyBlockedReasonV1::Policy(
                LoopRoutePolicyBlockReasonV1::ReleaseNestedLoopGate,
            ))
        );
    }
}
