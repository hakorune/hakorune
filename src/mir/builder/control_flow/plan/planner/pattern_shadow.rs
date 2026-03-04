//! Shadow pattern selection for drift detection (behavior-invariant).
//!
//! NOTE: This module is **shadow-only / diagnostic** (NOT SSOT).
//! The priority table below intentionally follows current semantic rule keys.
//! Actual pattern selection is determined by push order in build.rs.
//! Unknown rules get priority 255 by design.

use super::candidates::PlanCandidate;
use crate::mir::builder::control_flow::plan::trace;

/// Priority table: semantic rule key -> priority (lower = higher priority).
///
/// DIAGNOSTIC ONLY - not authoritative. Actual selection uses push order.
/// Keep this table aligned with keys emitted from planner/build.rs.
fn rule_priority(rule: &str) -> u8 {
    match rule {
        "loop_cond_continue_with_return" => 10,
        "loop_cond_break_continue" => 11,
        "loop_cond_continue_only" => 12,
        "loop_cond_return_in_body" => 13,
        r if r.starts_with("cluster") => 20,
        r if r.contains("nested_depth") => 21,
        _ => 255,
    }
}

/// Shadow pick: select rule by priority table
fn shadow_pick_rule(candidates: &[PlanCandidate]) -> Option<&'static str> {
    candidates
        .iter()
        .min_by_key(|c| rule_priority(c.rule))
        .map(|c| c.rule)
}

/// Trace shadow pick for ambiguous cases (delegates to trace.rs)
pub fn trace_shadow_pick(candidates: &[PlanCandidate]) {
    if let Some(shadow_rule) = shadow_pick_rule(candidates) {
        trace::trace_pattern_shadow_pick(shadow_rule, candidates.len());
    }
}

#[cfg(test)]
mod tests {
    use super::rule_priority;

    #[test]
    fn semantic_rule_priority_is_stable() {
        assert_eq!(rule_priority("loop_cond_continue_with_return"), 10);
        assert_eq!(rule_priority("loop_cond_break_continue"), 11);
        assert_eq!(rule_priority("legacy/unknown_rule"), 255);
    }
}
