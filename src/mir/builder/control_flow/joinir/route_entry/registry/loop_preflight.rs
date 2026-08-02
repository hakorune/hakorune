//! Builder-free all-route preflight vocabulary.
//!
//! This is a typed rejection boundary only. A later row may add the sole
//! producer after every route has a truthful source and policy proof.

use super::route_id::LoopRouteId;

/// Non-Clone preflight outcome; no qualified product exists in this foundation.
#[derive(Debug)]
pub(crate) enum LoopPreflightDispositionV1 {
    NoCandidate,
    Rejected(LoopPreflightRejectV1),
}

/// Every rejection is pre-effect and names the blocking route or order edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopPreflightRejectV1 {
    SourceTopologyUnavailable {
        route: LoopRouteId,
    },
    ScopeBoxLineageNotBorrowable {
        route: LoopRouteId,
    },
    PolicyAndTerminalityUnavailable {
        route: LoopRouteId,
    },
    SchedulerOrderBlocked {
        earlier: LoopRouteId,
        candidate: LoopRouteId,
    },
    PostEffectRetryDebt {
        route: LoopRouteId,
    },
}

#[cfg(test)]
mod tests {
    use super::{LoopPreflightDispositionV1, LoopPreflightRejectV1};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;

    #[test]
    fn vocabulary_keeps_source_policy_order_and_retry_debt_distinct() {
        let source = LoopPreflightDispositionV1::Rejected(
            LoopPreflightRejectV1::SourceTopologyUnavailable {
                route: LoopRouteId::LoopCharMap,
            },
        );
        let policy = LoopPreflightDispositionV1::Rejected(
            LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
                route: LoopRouteId::AccumConstLoop,
            },
        );
        let order =
            LoopPreflightDispositionV1::Rejected(LoopPreflightRejectV1::SchedulerOrderBlocked {
                earlier: LoopRouteId::LoopSimpleWhile,
                candidate: LoopRouteId::GenericLoopV0,
            });
        let debt =
            LoopPreflightDispositionV1::Rejected(LoopPreflightRejectV1::PostEffectRetryDebt {
                route: LoopRouteId::GenericLoopV1,
            });

        assert!(matches!(source, LoopPreflightDispositionV1::Rejected(_)));
        assert!(matches!(policy, LoopPreflightDispositionV1::Rejected(_)));
        assert!(matches!(order, LoopPreflightDispositionV1::Rejected(_)));
        assert!(matches!(debt, LoopPreflightDispositionV1::Rejected(_)));
    }
}
