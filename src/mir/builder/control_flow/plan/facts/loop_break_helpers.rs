//! Phase 29ai P11: Shared helper functions for loop_break extraction
//! Shared helpers for loop_break facts extraction.
//!
//! These helpers are used by multiple subset detectors.
//!
//! Note: most helpers stay `pub(in crate::mir::builder::control_flow::plan::facts)`;
//! loop-owner subsets moved under `loop_break::facts` may require selected helpers to widen to
//! `pub(in crate::mir::builder::control_flow::plan)`.

pub(in crate::mir::builder::control_flow::plan::facts) use crate::mir::builder::control_flow::plan::loop_break::facts::helpers_common::{
    add, has_continue_statement, has_return_statement, index_of_call_expr, lit_int, lit_str,
    substring_call, var,
};

pub(in crate::mir::builder::control_flow::plan::facts) use crate::mir::builder::control_flow::plan::loop_break::facts::helpers_break_if::{
    extract_break_if_parts, find_break_if_parts,
};
pub(in crate::mir::builder::control_flow::plan::facts) use super::loop_break_helpers_loop::{
    extract_loop_increment_at_end, extract_loop_var_for_len_condition,
    extract_loop_var_for_plan_subset, has_assignment_after,
};
