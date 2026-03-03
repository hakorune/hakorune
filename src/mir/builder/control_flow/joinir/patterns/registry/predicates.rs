use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;

macro_rules! pred {
    ($name:ident, $field:ident) => {
        pub(crate) fn $name(facts: &CanonicalLoopFacts) -> bool {
            facts.facts.$field.is_some()
        }
    };
}

pred!(pred_loop_break_recipe, pattern2_break);
pred!(pred_if_phi_join, pattern3_ifphi);
pred!(pred_loop_continue_only_pattern, pattern4_continue);
pred!(pred_loop_true_early_exit, pattern5_infinite_early_exit);
pred!(pred_loop_simple_while, pattern1_simplewhile);
pred!(pred_loop_char_map, pattern1_char_map);
pred!(pred_loop_array_join, pattern1_array_join);
pred!(pred_scan_with_init, scan_with_init);
pred!(pred_split_scan, split_scan);
pred!(pred_bool_predicate_scan, pattern8_bool_predicate_scan);
pred!(pred_accum_const_loop, pattern9_accum_const_loop);

// Legacy predicate aliases (remove after all call sites stop using Pattern* names).
#[allow(dead_code)]
pub(crate) fn pred_pattern2_break(facts: &CanonicalLoopFacts) -> bool {
    pred_loop_break_recipe(facts)
}
#[allow(dead_code)]
pub(crate) fn pred_pattern3_ifphi(facts: &CanonicalLoopFacts) -> bool {
    pred_if_phi_join(facts)
}
#[allow(dead_code)]
pub(crate) fn pred_pattern4_continue(facts: &CanonicalLoopFacts) -> bool {
    pred_loop_continue_only_pattern(facts)
}
#[allow(dead_code)]
pub(crate) fn pred_pattern5_infinite_early_exit(facts: &CanonicalLoopFacts) -> bool {
    pred_loop_true_early_exit(facts)
}
#[allow(dead_code)]
pub(crate) fn pred_pattern1_simplewhile(facts: &CanonicalLoopFacts) -> bool {
    pred_loop_simple_while(facts)
}
#[allow(dead_code)]
pub(crate) fn pred_pattern1_char_map(facts: &CanonicalLoopFacts) -> bool {
    pred_loop_char_map(facts)
}
#[allow(dead_code)]
pub(crate) fn pred_pattern1_array_join(facts: &CanonicalLoopFacts) -> bool {
    pred_loop_array_join(facts)
}
#[allow(dead_code)]
pub(crate) fn pred_pattern8_bool_predicate_scan(facts: &CanonicalLoopFacts) -> bool {
    pred_bool_predicate_scan(facts)
}
#[allow(dead_code)]
pub(crate) fn pred_pattern9_accum_const_loop(facts: &CanonicalLoopFacts) -> bool {
    pred_accum_const_loop(facts)
}

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
    facts.facts.loop_true_break_continue.is_some() && !pred_loop_break_recipe(facts)
}
pub(crate) fn pred_loop_cond_break_continue(facts: &CanonicalLoopFacts) -> bool {
    let has_scan_v0 = pred_loop_scan_v0(facts)
        || pred_loop_scan_phi_vars_v0(facts)
        || pred_loop_collect_using_entries_v0(facts)
        || pred_loop_bundle_resolver_v0(facts);
    let has_scan_init = facts.facts.scan_with_init.is_some() || facts.facts.split_scan.is_some();
    let has_scan_predicate = pred_bool_predicate_scan(facts) || pred_accum_const_loop(facts);
    facts.facts.loop_cond_break_continue.is_some()
        && !pred_loop_break_recipe(facts)
        && !has_scan_v0
        && !has_scan_init
        && !has_scan_predicate
}
pub(crate) fn pred_loop_cond_continue_only(facts: &CanonicalLoopFacts) -> bool {
    facts.facts.loop_cond_continue_only.is_some() && !pred_loop_continue_only_pattern(facts)
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
    let has_scan_predicate = pred_bool_predicate_scan(facts) || pred_accum_const_loop(facts);
    !has_scan_methods && !has_scan_v0 && !has_scan_init && !has_scan_predicate
}
pred!(pred_generic_loop_v0, generic_loop_v0);
pub(crate) fn pred_generic_loop_v1(facts: &CanonicalLoopFacts) -> bool {
    if facts.facts.generic_loop_v1.is_none() {
        return false;
    }
    if pred_loop_break_recipe(facts) {
        return false;
    }
    if pred_loop_simple_while(facts) {
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
    let has_scan_predicate = pred_bool_predicate_scan(facts) || pred_accum_const_loop(facts);
    if has_scan_methods || has_scan_v0 || has_scan_init || has_scan_predicate {
        return false;
    }
    true
}
