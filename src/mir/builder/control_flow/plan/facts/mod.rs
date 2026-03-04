//! Phase 29ai P0: Facts layer skeleton (SSOT)
//!
//! Responsibility: derive stable, structural "facts" from AST/CFG.
//! - No planning (no Plan/Frag decisions)
//! - No emission (no MIR/Frag generation)
//!
//! SSOT: docs/development/current/main/design/plan-dir-shallowing-ssot.md
//! shallowing: moved from subdirs (pattern2_break_facts/, loop_facts/, expr/)

#![allow(dead_code)]

// Flattened from pattern2_break_facts/
pub(in crate::mir::builder) mod pattern2_break_core;
pub(in crate::mir::builder) mod pattern2_break_helpers;
pub(in crate::mir::builder) mod pattern2_break_loopbodylocal;
pub(in crate::mir::builder) mod pattern2_break_parse_integer;
pub(in crate::mir::builder) mod pattern2_break_read_digits;
pub(in crate::mir::builder) mod pattern2_break_realworld;
pub(in crate::mir::builder) mod pattern2_break_step_before_break;
#[cfg(test)]
pub(in crate::mir::builder) mod pattern2_break_tests;
pub(in crate::mir::builder) mod pattern2_break_trim_whitespace;
pub(in crate::mir::builder) mod pattern2_break_types;

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
pub(in crate::mir::builder) mod feature_facts;
pub(in crate::mir::builder) mod block_policies;
pub(in crate::mir::builder) mod pattern1_simplewhile_facts;
pub(in crate::mir::builder) mod pattern1_char_map_facts;
pub(in crate::mir::builder) mod pattern1_array_join_facts;
pub(in crate::mir::builder) mod pattern_is_integer_facts;
pub(in crate::mir::builder) mod pattern_starts_with_facts;
pub(in crate::mir::builder) mod pattern_int_to_str_facts;
pub(in crate::mir::builder) mod pattern_escape_map_facts;
pub(in crate::mir::builder) mod pattern_split_lines_facts;
pub(in crate::mir::builder) mod pattern_skip_ws_facts;
pub(in crate::mir::builder) mod pattern_match_return_facts;
pub(in crate::mir::builder) mod pattern3_ifphi_facts;
pub(in crate::mir::builder) mod pattern4_continue_facts;
pub(in crate::mir::builder) mod pattern5_infinite_early_exit_facts;
pub(in crate::mir::builder) mod pattern6_nested_minimal_facts;
pub(in crate::mir::builder) mod pattern8_bool_predicate_scan_facts;
pub(in crate::mir::builder) mod pattern9_accum_const_loop_facts;
pub(in crate::mir::builder) mod pattern2_loopbodylocal_facts;
pub(in crate::mir::builder) mod scan_shapes;
pub(in crate::mir::builder) mod skeleton_facts;
pub(in crate::mir::builder) mod stmt_view;
pub(in crate::mir::builder) mod return_prelude;
pub(in crate::mir::builder) mod exit_only_block;
pub(in crate::mir::builder) mod no_exit_block;
pub(in crate::mir::builder) mod nested_loop_profile;
pub(in crate::mir::builder) mod reject_reason;

pub(in crate::mir::builder) use loop_builder::{
    try_build_loop_facts, try_build_loop_facts_with_ctx,
};
pub(in crate::mir::builder) use loop_types::LoopFacts;
pub(in crate::mir::builder) use pattern_match_return_facts::{
    MatchReturnFacts, MatchReturnScrutinee, try_extract_match_return_facts,
};
