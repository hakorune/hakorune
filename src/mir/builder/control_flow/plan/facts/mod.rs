//! Phase 29ai P0: Facts layer skeleton (SSOT)
//!
//! Responsibility: derive stable, structural "facts" from AST/CFG.
//! - No planning (no Plan/Frag decisions)
//! - No emission (no MIR/Frag generation)
//!
//! SSOT: docs/development/current/main/design/plan-dir-shallowing-ssot.md
//! shallowing: moved from legacy break-on-condition subdirs plus loop_facts/expr/.

// Flattened from the legacy break-on-condition facts cluster.
pub(in crate::mir::builder) mod loop_break_core;
pub(in crate::mir::builder) mod loop_break_helpers;
pub(in crate::mir::builder) mod loop_break_helpers_condition;
pub(in crate::mir::builder) mod loop_break_helpers_local;
pub(in crate::mir::builder) mod loop_break_helpers_realworld;
#[cfg(test)]
pub(in crate::mir::builder) mod loop_break_tests;

// Flattened from loop_facts/
pub(in crate::mir::builder) mod loop_builder;
pub(in crate::mir::builder) mod loop_condition_shape;
pub(in crate::mir::builder) mod loop_scan_with_init;
pub(in crate::mir::builder) mod loop_split_scan;
pub(in crate::mir::builder) mod loop_step_shape;
#[cfg(test)]
pub(in crate::mir::builder) mod loop_tests;
pub(in crate::mir::builder) mod loop_types;

// Flattened from expr/
pub(in crate::mir::builder) mod expr_bool;
pub(in crate::mir::builder) mod expr_generic_loop;
pub(in crate::mir::builder) mod expr_value;

// Existing modules
pub(in crate::mir::builder) mod accum_const_loop_facts;
pub(in crate::mir::builder) mod block_policies;
pub(in crate::mir::builder) mod bool_predicate_scan_facts;
pub(in crate::mir::builder) mod escape_map_facts;
pub(in crate::mir::builder) mod exit_only_block;
pub(in crate::mir::builder) mod feature_facts;
pub(in crate::mir::builder) mod if_phi_join_facts;
pub(in crate::mir::builder) mod int_to_str_facts;
pub(in crate::mir::builder) mod loop_array_join_facts;
pub(in crate::mir::builder) mod loop_break_body_local_facts;
pub(in crate::mir::builder) mod loop_char_map_facts;
pub(in crate::mir::builder) mod loop_continue_only_facts;
pub(in crate::mir::builder) mod loop_simple_while_facts;
pub(in crate::mir::builder) mod loop_true_early_exit_facts;
pub(in crate::mir::builder) mod match_return_facts;
pub(in crate::mir::builder) mod nested_loop_minimal_facts;
pub(in crate::mir::builder) mod nested_loop_profile;
pub(in crate::mir::builder) mod no_exit_block;
pub(in crate::mir::builder) mod reject_reason;
pub(in crate::mir::builder) mod return_prelude;
pub(in crate::mir::builder) mod scan_shapes;
pub(in crate::mir::builder) mod skeleton_facts;
pub(in crate::mir::builder) mod skip_whitespace_facts;
pub(in crate::mir::builder) mod split_lines_facts;
pub(in crate::mir::builder) mod starts_with_facts;
pub(in crate::mir::builder) mod stmt_view;
pub(in crate::mir::builder) mod string_is_integer_facts;

pub(in crate::mir::builder) use if_phi_join_facts::try_extract_if_phi_join_facts;
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::loop_break::facts::LoopBreakFacts;
pub(in crate::mir::builder) use loop_builder::{
    try_build_loop_facts, try_build_loop_facts_with_ctx,
};
pub(in crate::mir::builder) use loop_continue_only_facts::try_extract_loop_continue_only_facts;
pub(in crate::mir::builder) use loop_types::LoopFacts;
pub(in crate::mir::builder) type LoopSimpleWhileFacts =
    loop_simple_while_facts::LoopSimpleWhileFacts;
pub(in crate::mir::builder) type LoopCharMapFacts = loop_char_map_facts::LoopCharMapFacts;
pub(in crate::mir::builder) type LoopArrayJoinFacts = loop_array_join_facts::LoopArrayJoinFacts;
pub(in crate::mir::builder) type IfPhiJoinFacts = if_phi_join_facts::IfPhiJoinFacts;
pub(in crate::mir::builder) type LoopContinueOnlyFacts =
    loop_continue_only_facts::LoopContinueOnlyFacts;
pub(in crate::mir::builder) type LoopTrueEarlyExitFacts =
    loop_true_early_exit_facts::LoopTrueEarlyExitFacts;
pub(in crate::mir::builder) type NestedLoopMinimalFacts =
    nested_loop_minimal_facts::NestedLoopMinimalFacts;
pub(in crate::mir::builder) type BoolPredicateScanFacts =
    bool_predicate_scan_facts::BoolPredicateScanFacts;
pub(in crate::mir::builder) type AccumConstLoopFacts = accum_const_loop_facts::AccumConstLoopFacts;
pub(in crate::mir::builder) use match_return_facts::{
    try_extract_match_return_facts, MatchReturnFacts, MatchReturnScrutinee,
};
