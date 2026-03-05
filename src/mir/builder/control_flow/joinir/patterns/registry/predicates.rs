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
pred!(pred_loop_continue_recipe, pattern4_continue);
pred!(pred_loop_true_early_exit, pattern5_infinite_early_exit);
pub(crate) fn pred_loop_simple_while(facts: &CanonicalLoopFacts) -> bool {
    if facts.facts.pattern1_simplewhile.is_none() {
        return false;
    }
    // Keep scan-methods families on their dedicated routes.
    // Otherwise loop_simple_while can over-capture nested scan loops and produce
    // unstable step wiring (seen in selfhost scan_methods nested fixtures).
    let scan = ScanFamilyPresence::from_facts(facts);
    !scan.blocks_simple_while()
}
pred!(pred_loop_char_map, pattern1_char_map);
pred!(pred_loop_array_join, pattern1_array_join);
pred!(pred_scan_with_init, scan_with_init);
pred!(pred_split_scan, split_scan);
pred!(pred_bool_predicate_scan, pattern8_bool_predicate_scan);
pred!(pred_accum_const_loop, pattern9_accum_const_loop);

#[derive(Debug, Clone, Copy)]
struct ScanFamilyPresence {
    methods: bool,
    v0: bool,
    init: bool,
    predicate: bool,
}

impl ScanFamilyPresence {
    fn from_facts(facts: &CanonicalLoopFacts) -> Self {
        let methods =
            pred_loop_scan_methods_v0(facts) || pred_loop_scan_methods_block_v0(facts);
        let v0 = pred_loop_scan_v0(facts)
            || pred_loop_scan_phi_vars_v0(facts)
            || pred_loop_collect_using_entries_v0(facts)
            || pred_loop_bundle_resolver_v0(facts);
        let init = facts.facts.scan_with_init.is_some() || facts.facts.split_scan.is_some();
        let predicate = pred_bool_predicate_scan(facts) || pred_accum_const_loop(facts);
        Self {
            methods,
            v0,
            init,
            predicate,
        }
    }

    fn blocks_simple_while(self) -> bool {
        self.methods || self.v0
    }

    fn blocks_loop_cond_break(self) -> bool {
        self.v0 || self.init || self.predicate
    }

    fn blocks_return_or_generic(self) -> bool {
        self.methods || self.v0 || self.init || self.predicate
    }
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
pred!(pred_nested_loop_minimal, pattern6_nested_minimal);
pub(crate) fn pred_loop_bundle_resolver_v0(facts: &CanonicalLoopFacts) -> bool {
    facts.facts.loop_bundle_resolver_v0.is_some()
}
pub(crate) fn pred_loop_true_break_continue(facts: &CanonicalLoopFacts) -> bool {
    facts.facts.loop_true_break_continue.is_some() && !pred_loop_break_recipe(facts)
}
pub(crate) fn pred_loop_cond_break_continue(facts: &CanonicalLoopFacts) -> bool {
    let scan = ScanFamilyPresence::from_facts(facts);
    facts.facts.loop_cond_break_continue.is_some()
        && !pred_loop_break_recipe(facts)
        && !scan.blocks_loop_cond_break()
}
pub(crate) fn pred_loop_cond_continue_only(facts: &CanonicalLoopFacts) -> bool {
    facts.facts.loop_cond_continue_only.is_some() && !pred_loop_continue_recipe(facts)
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
    let scan = ScanFamilyPresence::from_facts(facts);
    !scan.blocks_return_or_generic()
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
    let scan = ScanFamilyPresence::from_facts(facts);
    if scan.blocks_return_or_generic() {
        return false;
    }
    true
}
