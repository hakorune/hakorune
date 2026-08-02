//! Closed owned vocabulary for one frozen Loop route schedule.

use crate::mir::loop_recipe_contract::route_id::LoopRouteId;

pub(crate) const CANONICAL_LOOP_ROUTE_COUNT_V1: usize = 19;

/// The frozen legacy order used only as migration parity/provenance.
///
/// This constant is not a semantic recipe order and must not drive lowering.
pub(crate) const CANONICAL_LOOP_ROUTE_ORDER_V1: [LoopRouteId; CANONICAL_LOOP_ROUTE_COUNT_V1] = [
    LoopRouteId::LoopBreakRecipe,
    LoopRouteId::IfPhiJoin,
    LoopRouteId::LoopContinueOnly,
    LoopRouteId::LoopTrueEarlyExit,
    LoopRouteId::LoopSimpleWhile,
    LoopRouteId::LoopCharMap,
    LoopRouteId::LoopArrayJoin,
    LoopRouteId::ScanWithInit,
    LoopRouteId::SplitScan,
    LoopRouteId::BoolPredicateScan,
    LoopRouteId::AccumConstLoop,
    LoopRouteId::NestedLoopMinimal,
    LoopRouteId::LoopTrueBreakContinue,
    LoopRouteId::LoopCondBreakContinue,
    LoopRouteId::LoopCondContinueOnly,
    LoopRouteId::LoopCondContinueWithReturn,
    LoopRouteId::LoopCondReturnInBody,
    LoopRouteId::GenericLoopV0,
    LoopRouteId::GenericLoopV1,
];

/// Typed reasons why a matched candidate was suppressed before raw execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopRouteSuppressionCauseV1 {
    EarlierIfPhiJoinCandidate,
    EarlierLoopContinueOnlyCandidate,
    EarlierLoopCondContinueOnlyCandidate,
    EarlierLoopArrayJoinCandidate,
    EarlierLoopTrueEarlyExitCandidate,
    EarlierLoopTrueBreakContinueCandidate,
}

/// The captured suppression observation; it performs no suppression policy.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LoopRouteSuppressionDispositionV1 {
    Retained,
    SuppressedBy(Box<[LoopRouteSuppressionCauseV1]>),
}

/// Whether strict/dev mode required the legacy recipe contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopPlannerContractObservationV1 {
    Optional,
    Required,
}

/// The already-computed release admission observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopReleaseAdmissionObservationV1 {
    Allowed,
    BlockedByNestedLoopGate,
}

/// One closed snapshot of mode facts; invalid cross-mode combinations cannot
/// be represented.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopModeReleaseSnapshotV1 {
    Release {
        admission: LoopReleaseAdmissionObservationV1,
    },
    StrictOrDev {
        planner_contract: LoopPlannerContractObservationV1,
    },
}

/// The global recipe-first entry decision captured before individual rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopGlobalEntryDispositionV1 {
    Allowed,
    BlockedByReleaseGate,
}

/// Closed source-observation gaps. There is intentionally no `Unknown` or
/// free-form reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopRouteSourceUnavailableV1 {
    FactsAbsent,
    SourceTopologyUnavailable,
    ScopeBoxLineageUnsupported,
    UnsupportedAncestry,
}

/// Whether this row's source observation is available to a later policy row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopRouteSourceDispositionV1 {
    Available,
    Unavailable(LoopRouteSourceUnavailableV1),
}

/// Owned observations paired with one route during structural sealing.
///
/// This input is also non-`Clone`; observations are moved exactly once into
/// the frozen row.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct FrozenLoopRouteObservationV1 {
    pub(super) suppression: LoopRouteSuppressionDispositionV1,
    pub(super) mode_release: LoopModeReleaseSnapshotV1,
    pub(super) global_entry: LoopGlobalEntryDispositionV1,
    pub(super) source: LoopRouteSourceDispositionV1,
}

impl FrozenLoopRouteObservationV1 {
    pub(crate) fn new(
        suppression: LoopRouteSuppressionDispositionV1,
        mode_release: LoopModeReleaseSnapshotV1,
        global_entry: LoopGlobalEntryDispositionV1,
        source: LoopRouteSourceDispositionV1,
    ) -> Self {
        Self {
            suppression,
            mode_release,
            global_entry,
            source,
        }
    }
}

/// One frozen raw row. `route_id` is opaque parity/provenance only.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct FrozenLoopRouteRowV1 {
    pub(super) raw_cursor: usize,
    pub(super) route_id: LoopRouteId,
    pub(super) suppression: LoopRouteSuppressionDispositionV1,
    pub(super) mode_release: LoopModeReleaseSnapshotV1,
    pub(super) global_entry: LoopGlobalEntryDispositionV1,
    pub(super) source: LoopRouteSourceDispositionV1,
}

impl FrozenLoopRouteRowV1 {
    pub(crate) fn raw_cursor(&self) -> usize {
        self.raw_cursor
    }

    /// Opaque migration parity/provenance. This is not a semantic dispatch key.
    pub(crate) fn route_id(&self) -> LoopRouteId {
        self.route_id
    }

    pub(crate) fn suppression(&self) -> &LoopRouteSuppressionDispositionV1 {
        &self.suppression
    }

    pub(crate) fn mode_release(&self) -> LoopModeReleaseSnapshotV1 {
        self.mode_release
    }

    pub(crate) fn global_entry(&self) -> LoopGlobalEntryDispositionV1 {
        self.global_entry
    }

    pub(crate) fn source(&self) -> LoopRouteSourceDispositionV1 {
        self.source
    }
}

/// Non-Clone owned canonical schedule. It exposes views, never a suffix owner.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct FrozenLoopRouteScheduleV1 {
    pub(super) rows: Box<[FrozenLoopRouteRowV1]>,
}

impl FrozenLoopRouteScheduleV1 {
    pub(crate) fn rows(&self) -> &[FrozenLoopRouteRowV1] {
        &self.rows
    }

    pub(crate) fn first(&self) -> &FrozenLoopRouteRowV1 {
        // Structural sealing guarantees exactly 19 rows.
        &self.rows[0]
    }
}

/// Typed structural failures while issuing a fresh frozen schedule.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum FrozenLoopRouteScheduleRejectV1 {
    EmptySchedule,
    MustStartAtRawCursorZero {
        expected: LoopRouteId,
        found: LoopRouteId,
    },
    RouteCountMismatch {
        expected: usize,
        found: usize,
    },
    ObservationCountMismatch {
        routes: usize,
        observations: usize,
    },
    InconsistentModeReleaseSnapshot {
        raw_cursor: usize,
        expected: LoopModeReleaseSnapshotV1,
        found: LoopModeReleaseSnapshotV1,
    },
    InconsistentGlobalEntryDisposition {
        raw_cursor: usize,
        expected: LoopGlobalEntryDispositionV1,
        found: LoopGlobalEntryDispositionV1,
    },
    DuplicateRoute {
        route: LoopRouteId,
        first_cursor: usize,
        duplicate_cursor: usize,
    },
    OutOfCanonicalOrder {
        raw_cursor: usize,
        expected: LoopRouteId,
        found: LoopRouteId,
    },
    SuppressedWithoutCause {
        raw_cursor: usize,
        route: LoopRouteId,
    },
}
