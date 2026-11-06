//! Common extraction helpers for descriptive loop facts.
//!
//! Canon-dependent increment extraction is also actualized here.

mod condition;
mod control_flow;
mod increment;

pub(crate) use condition::is_true_literal;
pub(crate) use control_flow::{
    branch_tail_is_continue, branch_tail_is_continue_flattened, count_control_flow, has_break_statement, has_continue_statement, has_if_else_statement,
    has_return_statement, ControlFlowCounts, ControlFlowDetector,
};
pub(crate) use increment::extract_loop_increment_plan;
