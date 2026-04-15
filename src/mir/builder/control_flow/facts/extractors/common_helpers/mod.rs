//! Common extraction helpers for descriptive loop facts.
//!
//! Canon-dependent increment extraction is also actualized here.

mod condition;
mod control_flow;
mod increment;
mod loop_true_early_exit;

#[allow(unused_imports)]
pub(crate) use super::super::stmt_walk::{
    flatten_stmt_list, strip_trailing_continue_view, walk_stmt_list,
};
#[allow(unused_imports)]
pub(crate) use condition::{extract_loop_variable, is_true_literal};
#[allow(unused_imports)]
pub(crate) use control_flow::{
    count_control_flow, find_if_else_statement, has_break_statement, has_continue_statement,
    has_control_flow_statement, has_if_else_statement, has_if_statement, has_return_statement,
    ControlFlowCounts, ControlFlowDetector,
};
#[allow(unused_imports)]
pub(crate) use increment::extract_loop_increment_plan;
#[allow(unused_imports)]
pub(crate) use loop_true_early_exit::{validate_break_in_simple_if, validate_continue_at_end};
