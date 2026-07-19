//! Legacy loop route observer vocabulary and shadow decision.
//!
//! This module is intentionally read-only over already-built loop facts and
//! the existing registry predicates. It does not select the runtime lowering
//! route and it is not an independent semantic resolver yet; the router still
//! uses the historical ordered registry. The purpose is to make route ownership
//! debt visible before retiring named routes.

use super::{selection::RecipeFirstRouteSelectionV1, types::LoopRouteId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopRouteDenyReason {
    NoFacts,
    NoCandidate,
    OverlappingNamedRoutes,
}

impl LoopRouteDenyReason {
    pub(crate) fn owner(self) -> &'static str {
        match self {
            Self::NoFacts => "recipe_fact_producer",
            Self::NoCandidate => "fixture_inventory",
            Self::OverlappingNamedRoutes => "loop_route_retire_selection",
        }
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::NoFacts => "NoFacts",
            Self::NoCandidate => "NoCandidate",
            Self::OverlappingNamedRoutes => "OverlappingNamedRoutes",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopRouteFact {
    pub(crate) selected_route: LoopRouteId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopRouteDecision {
    Allow(LoopRouteFact),
    Deny(LoopRouteDenyReason),
}

impl LoopRouteDecision {
    pub(crate) fn summary(self) -> String {
        match self {
            Self::Allow(fact) => format!("allow:{}", fact.selected_route),
            Self::Deny(reason) => {
                format!("deny:{} owner={}", reason.as_str(), reason.owner())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopRouteShadowReport {
    pub(crate) decision: LoopRouteDecision,
    pub(crate) legacy_matched_candidates: Vec<LoopRouteId>,
    pub(crate) legacy_effective_candidates: Vec<LoopRouteId>,
    pub(crate) legacy_suppressed_candidates: Vec<LoopRouteId>,
}

impl LoopRouteShadowReport {
    pub(crate) fn trace_line(&self) -> String {
        format!(
            "[plan/trace:loop_legacy_observer] decision={} legacy_matched={} legacy_effective={} legacy_suppressed={}",
            self.decision.summary(),
            join_or_none(&self.legacy_matched_candidates),
            join_or_none(&self.legacy_effective_candidates),
            join_or_none(&self.legacy_suppressed_candidates)
        )
    }
}

pub(crate) fn shadow_report(selection: &RecipeFirstRouteSelectionV1) -> LoopRouteShadowReport {
    let legacy_matched_candidates = selection.matched_routes().to_vec();
    let legacy_effective_candidates = selection.diagnostic_effective_routes().to_vec();
    let legacy_suppressed_candidates = legacy_matched_candidates
        .iter()
        .copied()
        .filter(|candidate| !legacy_effective_candidates.contains(candidate))
        .collect::<Vec<_>>();
    let decision = resolve_from_effective(selection.facts_present(), &legacy_effective_candidates);
    LoopRouteShadowReport {
        decision,
        legacy_matched_candidates,
        legacy_effective_candidates,
        legacy_suppressed_candidates,
    }
}

fn resolve_from_effective(
    facts_present: bool,
    effective_candidates: &[LoopRouteId],
) -> LoopRouteDecision {
    if !facts_present {
        return LoopRouteDecision::Deny(LoopRouteDenyReason::NoFacts);
    }
    match effective_candidates {
        [] => LoopRouteDecision::Deny(LoopRouteDenyReason::NoCandidate),
        [selected] => LoopRouteDecision::Allow(LoopRouteFact {
            selected_route: *selected,
        }),
        _ => LoopRouteDecision::Deny(LoopRouteDenyReason::OverlappingNamedRoutes),
    }
}

fn join_or_none(items: &[LoopRouteId]) -> String {
    if items.is_empty() {
        "none".to_string()
    } else {
        items
            .iter()
            .map(|item| item.as_str())
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::{join_or_none, LoopRouteDecision, LoopRouteDenyReason};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::types::LoopRouteId;

    #[test]
    fn b_lite_deny_reason_maps_to_owner() {
        assert_eq!(
            LoopRouteDenyReason::OverlappingNamedRoutes.owner(),
            "loop_route_retire_selection"
        );
    }

    #[test]
    fn b_lite_decision_summary_is_stable() {
        let decision = LoopRouteDecision::Deny(LoopRouteDenyReason::NoCandidate);
        assert_eq!(
            decision.summary(),
            "deny:NoCandidate owner=fixture_inventory"
        );
    }

    #[test]
    fn join_or_none_uses_none_for_empty_lists() {
        assert_eq!(join_or_none(&[]), "none");
        assert_eq!(
            join_or_none(&[LoopRouteId::IfPhiJoin, LoopRouteId::LoopContinueOnly]),
            "if_phi_join,loop_continue_only"
        );
    }
}
