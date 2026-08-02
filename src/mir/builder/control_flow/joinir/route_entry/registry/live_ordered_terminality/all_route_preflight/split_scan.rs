//! SplitScan's source-topology-only preflight rejection.

use crate::mir::builder::control_flow::joinir::route_entry::registry::loop_preflight::LoopPreflightRejectV1;
use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
use crate::mir::builder::control_flow::plan::facts::LoopFacts;

pub(super) fn classify_split_scan(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(split_scan) = facts.split_scan() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::SplitScan,
        };
    };
    let Some(topology) = split_scan.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::SplitScan,
        };
    };
    if topology.has_scope_box_lineage() {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::SplitScan,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::SplitScan,
    }
}
