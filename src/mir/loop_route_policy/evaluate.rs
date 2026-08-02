//! Structural validation and row sealing for M3-C.
//!
//! Despite the filename required by the task split, this module does not
//! evaluate route predicates, choose a winner, or implement the M3-E policy.

use crate::mir::loop_recipe_contract::route_id::LoopRouteId;

use super::schema::{
    FrozenLoopRouteObservationV1, FrozenLoopRouteRowV1, FrozenLoopRouteScheduleRejectV1,
    FrozenLoopRouteScheduleV1, LoopRouteSuppressionDispositionV1, CANONICAL_LOOP_ROUTE_COUNT_V1,
    CANONICAL_LOOP_ROUTE_ORDER_V1,
};

/// Sole caller-zero M3-C production facade for issuing a fresh row-zero
/// schedule from owned inputs.
pub(crate) fn freeze_loop_route_schedule_v1(
    route_ids: Box<[LoopRouteId]>,
    observations: Box<[FrozenLoopRouteObservationV1]>,
) -> Result<FrozenLoopRouteScheduleV1, FrozenLoopRouteScheduleRejectV1> {
    validate_route_ids(&route_ids)?;
    if observations.len() != route_ids.len() {
        return Err(FrozenLoopRouteScheduleRejectV1::ObservationCountMismatch {
            routes: route_ids.len(),
            observations: observations.len(),
        });
    }

    let expected_mode_release = observations[0].mode_release;
    let expected_global_entry = observations[0].global_entry;
    for (raw_cursor, observation) in observations.iter().enumerate().skip(1) {
        if observation.mode_release != expected_mode_release {
            return Err(
                FrozenLoopRouteScheduleRejectV1::InconsistentModeReleaseSnapshot {
                    raw_cursor,
                    expected: expected_mode_release,
                    found: observation.mode_release,
                },
            );
        }
        if observation.global_entry != expected_global_entry {
            return Err(
                FrozenLoopRouteScheduleRejectV1::InconsistentGlobalEntryDisposition {
                    raw_cursor,
                    expected: expected_global_entry,
                    found: observation.global_entry,
                },
            );
        }
    }

    let mut rows = Vec::with_capacity(CANONICAL_LOOP_ROUTE_COUNT_V1);
    for (raw_cursor, (route_id, observation)) in route_ids
        .into_vec()
        .into_iter()
        .zip(observations.into_vec())
        .enumerate()
    {
        if matches!(
            &observation.suppression,
            LoopRouteSuppressionDispositionV1::SuppressedBy(causes) if causes.is_empty()
        ) {
            return Err(FrozenLoopRouteScheduleRejectV1::SuppressedWithoutCause {
                raw_cursor,
                route: route_id,
            });
        }
        rows.push(FrozenLoopRouteRowV1 {
            raw_cursor,
            route_id,
            suppression: observation.suppression,
            mode_release: observation.mode_release,
            global_entry: observation.global_entry,
            source: observation.source,
        });
    }

    Ok(FrozenLoopRouteScheduleV1 {
        rows: rows.into_boxed_slice(),
    })
}

fn validate_route_ids(route_ids: &[LoopRouteId]) -> Result<(), FrozenLoopRouteScheduleRejectV1> {
    let Some(&first) = route_ids.first() else {
        return Err(FrozenLoopRouteScheduleRejectV1::EmptySchedule);
    };
    let expected_first = CANONICAL_LOOP_ROUTE_ORDER_V1[0];
    if first != expected_first {
        return Err(FrozenLoopRouteScheduleRejectV1::MustStartAtRawCursorZero {
            expected: expected_first,
            found: first,
        });
    }
    if route_ids.len() != CANONICAL_LOOP_ROUTE_COUNT_V1 {
        return Err(FrozenLoopRouteScheduleRejectV1::RouteCountMismatch {
            expected: CANONICAL_LOOP_ROUTE_COUNT_V1,
            found: route_ids.len(),
        });
    }

    for (duplicate_cursor, route) in route_ids.iter().copied().enumerate() {
        if let Some(first_cursor) = route_ids[..duplicate_cursor]
            .iter()
            .position(|earlier| *earlier == route)
        {
            return Err(FrozenLoopRouteScheduleRejectV1::DuplicateRoute {
                route,
                first_cursor,
                duplicate_cursor,
            });
        }
    }

    for (raw_cursor, (expected, found)) in CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .copied()
        .zip(route_ids.iter().copied())
        .enumerate()
    {
        if expected != found {
            return Err(FrozenLoopRouteScheduleRejectV1::OutOfCanonicalOrder {
                raw_cursor,
                expected,
                found,
            });
        }
    }
    Ok(())
}
