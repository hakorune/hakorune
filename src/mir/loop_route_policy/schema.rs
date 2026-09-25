//! Closed owned vocabulary retained for all-route observation provenance.

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

/// Closed source-observation gaps. There is intentionally no `Unknown` or
/// free-form reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopRouteSourceUnavailableV1 {
    FactsAbsent,
    SourceTopologyUnavailable,
    ScopeBoxLineageUnsupported,
    UnsupportedAncestry,
}
