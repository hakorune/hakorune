//! Phase 29ai P11: Shared helper functions for loop_break extraction
//! Shared helpers for loop_break facts extraction.
//!
//! These helpers are used by multiple subset detectors.
//!
//! Note: All functions use `pub(in crate::mir::builder::control_flow::plan::facts)`
//! visibility to allow re-export via `pub(in crate::mir::builder::control_flow::plan::facts) use helpers::*;` in mod.rs.

pub(in crate::mir::builder::control_flow::plan::facts) use crate::mir::builder::control_flow::plan::loop_break::facts::helpers_common::{
    add, has_continue_statement, has_return_statement, index_of_call, index_of_call_expr,
    length_call, lit_int, lit_str, substring_call, var,
};

pub(in crate::mir::builder::control_flow::plan::facts) use crate::mir::builder::control_flow::plan::loop_break::facts::helpers_break_if::{
    extract_break_if_parts, find_break_if_parts,
};
pub(in crate::mir::builder::control_flow::plan::facts) use super::loop_break_helpers_loop::{
    extract_loop_increment_at_end, extract_loop_var_for_len_condition,
    extract_loop_var_for_plan_subset, has_assignment_after,
};

pub(in crate::mir::builder::control_flow::plan::facts) use super::loop_break_helpers_realworld::{
    match_break_if, match_loop_increment, match_seg_if_else,
};
