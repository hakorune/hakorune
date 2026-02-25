use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;

macro_rules! pred {
    ($name:ident, $field:ident) => {
        pub(crate) fn $name(facts: &CanonicalLoopFacts) -> bool {
            facts.facts.$field.is_some()
        }
    };
}

pred!(pred_pattern2_break, pattern2_break);
pred!(pred_pattern3_ifphi, pattern3_ifphi);
pred!(pred_pattern4_continue, pattern4_continue);
pred!(pred_pattern5_infinite_early_exit, pattern5_infinite_early_exit);
pred!(pred_pattern1_simplewhile, pattern1_simplewhile);
pred!(pred_pattern1_char_map, pattern1_char_map);
pred!(pred_pattern1_array_join, pattern1_array_join);
pred!(pred_scan_with_init, scan_with_init);
pred!(pred_split_scan, split_scan);
pred!(pred_pattern8_bool_predicate_scan, pattern8_bool_predicate_scan);
pred!(pred_pattern9_accum_const_loop, pattern9_accum_const_loop);

pub(crate) fn pred_loop_scan_methods_v0(facts: &CanonicalLoopFacts) -> bool {
    facts.facts.loop_scan_methods_v0.is_some()
        && facts.facts.loop_scan_methods_block_v0.is_none()
}
pub(crate) fn pred_loop_scan_methods_block_v0(facts: &CanonicalLoopFacts) -> bool {
    facts.facts.loop_scan_methods_block_v0.is_some()
}

pred!(pred_loop_scan_phi_vars_v0, loop_scan_phi_vars_v0);
pred!(pred_loop_scan_v0, loop_scan_v0);
pred!(pred_loop_collect_using_entries_v0, loop_collect_using_entries_v0);
pub(crate) fn pred_loop_bundle_resolver_v0(facts: &CanonicalLoopFacts) -> bool {
    facts.facts.loop_bundle_resolver_v0.is_some()
}
pub(crate) fn pred_loop_true_break_continue(facts: &CanonicalLoopFacts) -> bool {
    facts.facts.loop_true_break_continue.is_some() && facts.facts.pattern2_break.is_none()
}
pub(crate) fn pred_loop_cond_break_continue(facts: &CanonicalLoopFacts) -> bool {
    let has_scan_v0 = pred_loop_scan_v0(facts)
        || pred_loop_scan_phi_vars_v0(facts)
        || pred_loop_collect_using_entries_v0(facts)
        || pred_loop_bundle_resolver_v0(facts);
    let has_scan_init = facts.facts.scan_with_init.is_some() || facts.facts.split_scan.is_some();
    let has_scan_predicate =
        facts.facts.pattern8_bool_predicate_scan.is_some() || facts.facts.pattern9_accum_const_loop.is_some();
    facts.facts.loop_cond_break_continue.is_some()
        && facts.facts.pattern2_break.is_none()
        && !has_scan_v0
        && !has_scan_init
        && !has_scan_predicate
}
pub(crate) fn pred_loop_cond_continue_only(facts: &CanonicalLoopFacts) -> bool {
    facts.facts.loop_cond_continue_only.is_some() && facts.facts.pattern4_continue.is_none()
}
pred!(pred_loop_cond_continue_with_return, loop_cond_continue_with_return);
pub(crate) fn pred_loop_cond_return_in_body(facts: &CanonicalLoopFacts) -> bool {
    if facts.facts.loop_cond_return_in_body.is_none() {
        return false;
    }
    // Keep planner-first contract stable: when break/continue shape is available,
    // route through LoopCondBreak and treat return_in_body as observational only.
    if facts.facts.loop_cond_break_continue.is_some() {
        return false;
    }
    let has_scan_methods =
        pred_loop_scan_methods_v0(facts) || pred_loop_scan_methods_block_v0(facts);
    let has_scan_v0 = pred_loop_scan_v0(facts)
        || pred_loop_scan_phi_vars_v0(facts)
        || pred_loop_collect_using_entries_v0(facts)
        || pred_loop_bundle_resolver_v0(facts);
    let has_scan_init = facts.facts.scan_with_init.is_some() || facts.facts.split_scan.is_some();
    let has_scan_predicate =
        facts.facts.pattern8_bool_predicate_scan.is_some() || facts.facts.pattern9_accum_const_loop.is_some();
    !has_scan_methods && !has_scan_v0 && !has_scan_init && !has_scan_predicate
}
pred!(pred_generic_loop_v0, generic_loop_v0);
pub(crate) fn pred_generic_loop_v1(facts: &CanonicalLoopFacts) -> bool {
    if facts.facts.generic_loop_v1.is_none() {
        return false;
    }
    if facts.facts.pattern2_break.is_some() {
        return false;
    }
    if facts.facts.pattern1_simplewhile.is_some() {
        return false;
    }
    if facts.facts.loop_cond_break_continue.is_some() {
        return false;
    }
    let has_scan_methods =
        pred_loop_scan_methods_v0(facts) || pred_loop_scan_methods_block_v0(facts);
    let has_scan_v0 = pred_loop_scan_v0(facts)
        || pred_loop_scan_phi_vars_v0(facts)
        || pred_loop_collect_using_entries_v0(facts)
        || pred_loop_bundle_resolver_v0(facts);
    let has_scan_init = facts.facts.scan_with_init.is_some() || facts.facts.split_scan.is_some();
    let has_scan_predicate =
        facts.facts.pattern8_bool_predicate_scan.is_some() || facts.facts.pattern9_accum_const_loop.is_some();
    if has_scan_methods || has_scan_v0 || has_scan_init || has_scan_predicate {
        return false;
    }
    true
}
