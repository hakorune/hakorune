//! Owned, caller-zero M3-C snapshot of the canonical Loop route rows.
//!
//! `evaluate` is structural validation/sealing only. It is not the M3-E route
//! policy evaluator. See `README.md` for the authority boundary.

#[cfg(test)]
mod adapter;
mod evaluate;
mod schema;

#[allow(unused_imports)]
pub(crate) use evaluate::freeze_loop_route_schedule_v1;
#[allow(unused_imports)]
pub(crate) use schema::{
    FrozenLoopRouteObservationV1, FrozenLoopRouteRowV1, FrozenLoopRouteScheduleRejectV1,
    FrozenLoopRouteScheduleV1, LoopGlobalEntryDispositionV1, LoopModeReleaseSnapshotV1,
    LoopPlannerContractObservationV1, LoopReleaseAdmissionObservationV1,
    LoopRouteSourceDispositionV1, LoopRouteSourceUnavailableV1, LoopRouteSuppressionCauseV1,
    LoopRouteSuppressionDispositionV1, CANONICAL_LOOP_ROUTE_COUNT_V1,
    CANONICAL_LOOP_ROUTE_ORDER_V1,
};

#[cfg(test)]
mod tests;
