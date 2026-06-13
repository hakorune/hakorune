pub(in crate::mir::builder) use crate::mir::builder::control_flow::step_placement::facts::{
    collect_conditional_step_indices, collect_direct_step_indices,
};
pub(crate) use crate::mir::builder::control_flow::step_placement::facts::{
    is_break_else_if_with_increment, is_continue_if_with_increment, matches_loop_increment,
};
