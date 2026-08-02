//! Test-only migration fixture adapter.
//!
//! This module intentionally imports neither the production registry nor its
//! selection function. M3-C has no production adapter caller.

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
