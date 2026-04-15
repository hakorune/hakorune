//! Top-level owner surface for control-flow facts.
//!
//! Facts-owned modules should live here first. Remaining `plan::facts` residue
//! is forwarded explicitly so non-`plan/` callers can depend on this module
//! without inheriting a wildcard compat surface.

pub(crate) mod ast_feature_extractor;
pub(in crate::mir::builder) mod canon;
pub(in crate::mir::builder) mod escape_shape_recognizer;
pub(in crate::mir::builder) mod extractors;
pub(in crate::mir::builder) mod route_shape_recognizers;
pub(in crate::mir::builder) mod stmt_walk;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::facts::{
    accum_const_loop_facts, block_policies, bool_predicate_scan_facts, escape_map_facts,
    exit_only_block, expr_bool, expr_generic_loop, expr_value, feature_facts, if_phi_join_facts,
    int_to_str_facts, loop_array_join_facts, loop_builder, loop_char_map_facts,
    loop_condition_shape, loop_continue_only_facts, loop_scan_with_init,
    loop_simple_while_facts, loop_split_scan, loop_step_shape, loop_true_early_exit_facts,
    loop_types, match_return_facts, nested_loop_minimal_facts, nested_loop_profile,
    no_exit_block, reject_reason, return_prelude, scan_shapes, skeleton_facts,
    skip_whitespace_facts, split_lines_facts, starts_with_facts, stmt_view,
    string_is_integer_facts, AccumConstLoopFacts, BoolPredicateScanFacts, IfPhiJoinFacts,
    LoopArrayJoinFacts, LoopCharMapFacts, LoopContinueOnlyFacts, LoopFacts,
    LoopSimpleWhileFacts, LoopTrueEarlyExitFacts, MatchReturnFacts, MatchReturnScrutinee,
    NestedLoopMinimalFacts, try_build_loop_facts, try_build_loop_facts_with_ctx,
    try_extract_if_phi_join_facts, try_extract_loop_continue_only_facts, try_extract_match_return_facts,
};
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::loop_break::facts::LoopBreakFacts;

#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::facts::{
    loop_break_tests, loop_tests,
};
