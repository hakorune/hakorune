use super::adapter::freeze_canonical_loop_route_schedule_fixture_v1;
use super::*;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;

fn retained_observation() -> FrozenLoopRouteObservationV1 {
    FrozenLoopRouteObservationV1::new(
        LoopRouteSuppressionDispositionV1::Retained,
        LoopModeReleaseSnapshotV1::Release {
            admission: LoopReleaseAdmissionObservationV1::Allowed,
        },
        LoopGlobalEntryDispositionV1::Allowed,
        LoopRouteSourceDispositionV1::Available,
        LoopRoutePolicyEvidenceV1::Candidate(LoopRouteCandidateFactsV1::SourceAvailable),
    )
}

fn retained_observations(count: usize) -> Box<[FrozenLoopRouteObservationV1]> {
    (0..count)
        .map(|_| retained_observation())
        .collect::<Box<[_]>>()
}

fn strict_observation(
    suppression: LoopRouteSuppressionDispositionV1,
    source: LoopRouteSourceDispositionV1,
) -> FrozenLoopRouteObservationV1 {
    FrozenLoopRouteObservationV1::new(
        suppression,
        LoopModeReleaseSnapshotV1::StrictOrDev {
            planner_contract: LoopPlannerContractObservationV1::Required,
        },
        LoopGlobalEntryDispositionV1::Allowed,
        source,
        LoopRoutePolicyEvidenceV1::Candidate(LoopRouteCandidateFactsV1::SourceAvailable),
    )
}

#[test]
fn fixture_seals_all_nineteen_rows_in_canonical_cursor_order() {
    let schedule =
        freeze_canonical_loop_route_schedule_fixture_v1().expect("canonical fixture seals");

    assert_eq!(schedule.rows().len(), CANONICAL_LOOP_ROUTE_COUNT_V1);
    assert_eq!(schedule.first().raw_cursor(), 0);
    for (raw_cursor, (row, expected_route)) in schedule
        .rows()
        .iter()
        .zip(CANONICAL_LOOP_ROUTE_ORDER_V1)
        .enumerate()
    {
        assert_eq!(row.raw_cursor(), raw_cursor);
        assert_eq!(row.route_id(), expected_route);
    }
}

#[test]
fn empty_schedule_is_a_typed_rejection() {
    assert_eq!(
        freeze_loop_route_schedule_v1(Box::default(), Box::default()),
        Err(FrozenLoopRouteScheduleRejectV1::EmptySchedule)
    );
}

#[test]
fn duplicate_route_is_a_typed_rejection() {
    let mut routes = CANONICAL_LOOP_ROUTE_ORDER_V1;
    routes[18] = routes[17];

    assert_eq!(
        freeze_loop_route_schedule_v1(routes.into(), retained_observations(19)),
        Err(FrozenLoopRouteScheduleRejectV1::DuplicateRoute {
            route: LoopRouteId::GenericLoopV0,
            first_cursor: 17,
            duplicate_cursor: 18,
        })
    );
}

#[test]
fn out_of_order_route_is_a_typed_rejection() {
    let mut routes = CANONICAL_LOOP_ROUTE_ORDER_V1;
    routes.swap(7, 8);

    assert_eq!(
        freeze_loop_route_schedule_v1(routes.into(), retained_observations(19)),
        Err(FrozenLoopRouteScheduleRejectV1::OutOfCanonicalOrder {
            raw_cursor: 7,
            expected: LoopRouteId::ScanWithInit,
            found: LoopRouteId::SplitScan,
        })
    );
}

#[test]
fn suffix_cannot_be_reissued_as_a_fresh_schedule() {
    let suffix = CANONICAL_LOOP_ROUTE_ORDER_V1[1..].into();

    assert_eq!(
        freeze_loop_route_schedule_v1(suffix, retained_observations(18)),
        Err(FrozenLoopRouteScheduleRejectV1::MustStartAtRawCursorZero {
            expected: LoopRouteId::LoopBreakRecipe,
            found: LoopRouteId::IfPhiJoin,
        })
    );
}

#[test]
fn observation_count_mismatch_is_a_typed_rejection() {
    assert_eq!(
        freeze_loop_route_schedule_v1(
            CANONICAL_LOOP_ROUTE_ORDER_V1.into(),
            retained_observations(18),
        ),
        Err(FrozenLoopRouteScheduleRejectV1::ObservationCountMismatch {
            routes: 19,
            observations: 18,
        })
    );
}

#[test]
fn suppressed_disposition_requires_at_least_one_typed_cause() {
    let observations = CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .enumerate()
        .map(|(cursor, _)| {
            if cursor == 13 {
                FrozenLoopRouteObservationV1::new(
                    LoopRouteSuppressionDispositionV1::SuppressedBy(Box::default()),
                    LoopModeReleaseSnapshotV1::Release {
                        admission: LoopReleaseAdmissionObservationV1::Allowed,
                    },
                    LoopGlobalEntryDispositionV1::Allowed,
                    LoopRouteSourceDispositionV1::Available,
                    LoopRoutePolicyEvidenceV1::Candidate(
                        LoopRouteCandidateFactsV1::SourceAvailable,
                    ),
                )
            } else {
                retained_observation()
            }
        })
        .collect::<Box<[_]>>();

    assert_eq!(
        freeze_loop_route_schedule_v1(CANONICAL_LOOP_ROUTE_ORDER_V1.into(), observations),
        Err(FrozenLoopRouteScheduleRejectV1::SuppressedWithoutCause {
            raw_cursor: 13,
            route: LoopRouteId::LoopCondBreakContinue,
        })
    );
}

#[test]
fn row_preserves_closed_typed_dispositions_without_evaluating_them() {
    let observations = CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .enumerate()
        .map(|(cursor, _)| {
            if cursor == 13 {
                strict_observation(
                    LoopRouteSuppressionDispositionV1::SuppressedBy(
                        [
                            LoopRouteSuppressionCauseV1::EarlierIfPhiJoinCandidate,
                            LoopRouteSuppressionCauseV1::EarlierLoopArrayJoinCandidate,
                        ]
                        .into(),
                    ),
                    LoopRouteSourceDispositionV1::Unavailable(
                        LoopRouteSourceUnavailableV1::ScopeBoxLineageUnsupported,
                    ),
                )
            } else {
                strict_observation(
                    LoopRouteSuppressionDispositionV1::Retained,
                    LoopRouteSourceDispositionV1::Available,
                )
            }
        })
        .collect::<Box<[_]>>();
    let schedule =
        freeze_loop_route_schedule_v1(CANONICAL_LOOP_ROUTE_ORDER_V1.into(), observations)
            .expect("typed observations seal structurally");
    let row = &schedule.rows()[13];

    assert!(matches!(
        row.suppression(),
        LoopRouteSuppressionDispositionV1::SuppressedBy(causes)
            if causes.as_ref()
                == [
                    LoopRouteSuppressionCauseV1::EarlierIfPhiJoinCandidate,
                    LoopRouteSuppressionCauseV1::EarlierLoopArrayJoinCandidate,
                ]
    ));
    assert_eq!(
        row.mode_release(),
        LoopModeReleaseSnapshotV1::StrictOrDev {
            planner_contract: LoopPlannerContractObservationV1::Required,
        }
    );
    assert_eq!(row.global_entry(), LoopGlobalEntryDispositionV1::Allowed);
    assert_eq!(
        row.source(),
        LoopRouteSourceDispositionV1::Unavailable(
            LoopRouteSourceUnavailableV1::ScopeBoxLineageUnsupported,
        )
    );
    assert_eq!(
        row.policy_evidence(),
        LoopRoutePolicyEvidenceV1::Candidate(LoopRouteCandidateFactsV1::SourceAvailable)
    );
}

#[test]
fn mode_and_global_snapshots_must_be_frozen_once_for_all_rows() {
    let mut observations = retained_observations(19).into_vec();
    observations[4] = strict_observation(
        LoopRouteSuppressionDispositionV1::Retained,
        LoopRouteSourceDispositionV1::Available,
    );
    assert_eq!(
        freeze_loop_route_schedule_v1(
            CANONICAL_LOOP_ROUTE_ORDER_V1.into(),
            observations.into_boxed_slice(),
        ),
        Err(
            FrozenLoopRouteScheduleRejectV1::InconsistentModeReleaseSnapshot {
                raw_cursor: 4,
                expected: LoopModeReleaseSnapshotV1::Release {
                    admission: LoopReleaseAdmissionObservationV1::Allowed,
                },
                found: LoopModeReleaseSnapshotV1::StrictOrDev {
                    planner_contract: LoopPlannerContractObservationV1::Required,
                },
            }
        )
    );

    let mut observations = retained_observations(19).into_vec();
    observations[8] = FrozenLoopRouteObservationV1::new(
        LoopRouteSuppressionDispositionV1::Retained,
        LoopModeReleaseSnapshotV1::Release {
            admission: LoopReleaseAdmissionObservationV1::Allowed,
        },
        LoopGlobalEntryDispositionV1::BlockedByReleaseGate,
        LoopRouteSourceDispositionV1::Available,
        LoopRoutePolicyEvidenceV1::Candidate(LoopRouteCandidateFactsV1::SourceAvailable),
    );
    assert_eq!(
        freeze_loop_route_schedule_v1(
            CANONICAL_LOOP_ROUTE_ORDER_V1.into(),
            observations.into_boxed_slice(),
        ),
        Err(
            FrozenLoopRouteScheduleRejectV1::InconsistentGlobalEntryDisposition {
                raw_cursor: 8,
                expected: LoopGlobalEntryDispositionV1::Allowed,
                found: LoopGlobalEntryDispositionV1::BlockedByReleaseGate,
            }
        )
    );
}

#[test]
fn every_closed_enum_variant_is_constructible_without_unknown_or_text_payloads() {
    let suppression_causes = [
        LoopRouteSuppressionCauseV1::EarlierIfPhiJoinCandidate,
        LoopRouteSuppressionCauseV1::EarlierLoopContinueOnlyCandidate,
        LoopRouteSuppressionCauseV1::EarlierLoopCondContinueOnlyCandidate,
        LoopRouteSuppressionCauseV1::EarlierLoopArrayJoinCandidate,
        LoopRouteSuppressionCauseV1::EarlierLoopTrueEarlyExitCandidate,
        LoopRouteSuppressionCauseV1::EarlierLoopTrueBreakContinueCandidate,
    ];
    let planner = [
        LoopPlannerContractObservationV1::Optional,
        LoopPlannerContractObservationV1::Required,
    ];
    let release = [
        LoopReleaseAdmissionObservationV1::Allowed,
        LoopReleaseAdmissionObservationV1::BlockedByNestedLoopGate,
    ];
    let global = [
        LoopGlobalEntryDispositionV1::Allowed,
        LoopGlobalEntryDispositionV1::BlockedByReleaseGate,
    ];
    let source = [
        LoopRouteSourceUnavailableV1::FactsAbsent,
        LoopRouteSourceUnavailableV1::SourceTopologyUnavailable,
        LoopRouteSourceUnavailableV1::ScopeBoxLineageUnsupported,
        LoopRouteSourceUnavailableV1::UnsupportedAncestry,
    ];

    assert_eq!(suppression_causes.len(), 6);
    assert_eq!(planner.len(), 2);
    assert_eq!(release.len(), 2);
    assert_eq!(global.len(), 2);
    assert_eq!(source.len(), 4);
}
