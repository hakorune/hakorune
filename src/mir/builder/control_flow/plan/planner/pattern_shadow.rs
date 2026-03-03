//! Shadow pattern selection for drift detection (behavior-invariant)
//!
//! NOTE: This module is **shadow-only / diagnostic** (NOT SSOT).
//! The priority table below is for observability and debugging only.
//! Actual pattern selection is determined by push order in build.rs.
//! Unknown rules get priority 255 and that's intentional.

use super::candidates::PlanCandidate;
use crate::mir::builder::control_flow::plan::trace;

/// Canonicalize legacy pattern-style rule ids into semantic ids.
///
/// D1 policy: semantic ids are preferred; legacy aliases remain accepted.
fn canonical_shadow_rule(rule: &str) -> &str {
    match rule {
        "loop/pattern1_simplewhile" => "loop/loop_simple_while",
        "loop/pattern1_char_map" => "loop/char_map",
        "loop/pattern1_array_join" => "loop/array_join",
        "loop/pattern2_break" => "loop/loop_break_recipe",
        "loop/pattern3_ifphi" => "loop/if_phi_join",
        "loop/pattern4_continue" => "loop/loop_continue_only",
        "loop/pattern5_infinite_early_exit" => "loop/loop_true_early_exit",
        "loop/pattern8_bool_predicate_scan" => "loop/bool_predicate_scan",
        "loop/pattern9_accum_const_loop" => "loop/accum_const_loop",
        _ => rule,
    }
}

/// Priority table: rule → priority (lower = higher priority)
///
/// DIAGNOSTIC ONLY - not authoritative. Actual selection uses push order.
/// Unknown rules get priority 255 (lowest) by design.
fn rule_priority(rule: &str) -> u8 {
    let canonical = canonical_shadow_rule(rule);
    match canonical {
        // TIER 1: High-Priority Scans
        "loop/scan_with_init" => 10,
        "loop/split_scan" => 11,

        // TIER 2: Classic Patterns
        "loop/loop_break_recipe" => 20,
        "loop/if_phi_join" => 21,
        "loop/loop_continue_only" => 22,
        "loop/loop_true_early_exit" => 23,

        // TIER 3: Specialized
        "loop/bool_predicate_scan" => 30,
        "loop/accum_const_loop" => 31,

        // TIER 4: Pattern 1 Variants
        "loop/char_map" => 40,
        "loop/array_join" => 41,
        "loop/loop_simple_while" => 42,

        // TIER 5: V0 Fallbacks
        "loop/flag_exit_v0" => 50,
        "loop/scan_phi_vars_v0" => 51,
        "loop/scan_methods_block_v0" => 52,
        "loop/bundle_resolver_v0" => 53,
        "loop/collect_using_entries_v0" => 54,

        // TIER 6: Recipe Block
        "loop/loop_cond_break_continue" => 60,
        "loop/scan_methods_v0" => 61,
        "loop/scan_v0" => 62,

        // TIER 7: Loop Control Variants
        "loop/loop_true_break_continue" => 70,
        "loop/loop_cond_continue_only" => 71,
        "loop/loop_cond_continue_with_return" => 72,
        "loop/loop_cond_return_in_body" => 73,

        // TIER 8: Generic Fallbacks
        "loop/generic_v1" => 80,
        "loop/generic_v0" => 81,

        // Cluster variants (nested_loop_profile)
        r if r.starts_with("loop/cluster") => 65,
        r if r.contains("nested_depth") => 55,

        // Unknown = lowest priority
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
    fn legacy_rule_aliases_map_to_semantic_priority() {
        assert_eq!(
            rule_priority("loop/pattern2_break"),
            rule_priority("loop/loop_break_recipe")
        );
        assert_eq!(
            rule_priority("loop/pattern1_simplewhile"),
            rule_priority("loop/loop_simple_while")
        );
        assert_eq!(
            rule_priority("loop/pattern8_bool_predicate_scan"),
            rule_priority("loop/bool_predicate_scan")
        );
    }
}
