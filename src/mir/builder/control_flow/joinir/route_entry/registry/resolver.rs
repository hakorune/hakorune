//! B-lite loop route resolver vocabulary and shadow decision.
//!
//! This module is intentionally read-only over already-built loop facts and
//! the existing registry predicates. It does not select the runtime lowering
//! route; the router still uses the historical ordered registry. The purpose is
//! to make route ownership debt visible before retiring named routes.

use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;

use super::{collect_candidates, ENTRIES};

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
    pub(crate) selected_route: &'static str,
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
    pub(crate) raw_candidates: Vec<&'static str>,
    pub(crate) effective_candidates: Vec<&'static str>,
    pub(crate) suppressed_candidates: Vec<&'static str>,
}

impl LoopRouteShadowReport {
    pub(crate) fn route_disagreement(&self) -> bool {
        match self.decision {
            LoopRouteDecision::Allow(fact) => {
                self.effective_candidates.first().copied() != Some(fact.selected_route)
            }
            LoopRouteDecision::Deny(_) => !self.effective_candidates.is_empty(),
        }
    }

    pub(crate) fn trace_line(&self) -> String {
        format!(
            "[plan/trace:loop_resolver_b_lite] decision={} raw={} effective={} suppressed={} disagreement={}",
            self.decision.summary(),
            join_or_none(&self.raw_candidates),
            join_or_none(&self.effective_candidates),
            join_or_none(&self.suppressed_candidates),
            self.route_disagreement()
        )
    }
}

pub(crate) fn shadow_report(facts: Option<&CanonicalLoopFacts>) -> LoopRouteShadowReport {
    let raw_candidates = raw_candidates(facts);
    let effective_candidates = collect_candidates(facts);
    let suppressed_candidates = raw_candidates
        .iter()
        .copied()
        .filter(|candidate| !effective_candidates.contains(candidate))
        .collect::<Vec<_>>();
    let decision = resolve_from_effective(facts, &effective_candidates);
    LoopRouteShadowReport {
        decision,
        raw_candidates,
        effective_candidates,
        suppressed_candidates,
    }
}

fn resolve_from_effective(
    facts: Option<&CanonicalLoopFacts>,
    effective_candidates: &[&'static str],
) -> LoopRouteDecision {
    if facts.is_none() {
        return LoopRouteDecision::Deny(LoopRouteDenyReason::NoFacts);
    }
    match effective_candidates {
        [] => LoopRouteDecision::Deny(LoopRouteDenyReason::NoCandidate),
        [selected] => LoopRouteDecision::Allow(LoopRouteFact {
            selected_route: selected,
        }),
        _ => LoopRouteDecision::Deny(LoopRouteDenyReason::OverlappingNamedRoutes),
    }
}

fn raw_candidates(facts: Option<&CanonicalLoopFacts>) -> Vec<&'static str> {
    let Some(facts) = facts else {
        return Vec::new();
    };
    ENTRIES
        .iter()
        .filter(|entry| (entry.predicate)(facts))
        .map(|entry| entry.name)
        .collect()
}

fn join_or_none(items: &[&'static str]) -> String {
    if items.is_empty() {
        "none".to_string()
    } else {
        items.join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::{join_or_none, LoopRouteDecision, LoopRouteDenyReason};

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
        assert_eq!(join_or_none(&["a", "b"]), "a,b");
    }
}
