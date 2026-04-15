mod matcher;

pub(in crate::mir::builder) use matcher::{
    collect_conditional_step_indices, collect_direct_step_indices,
};
pub(crate) use matcher::{
    is_break_else_if_with_increment, is_continue_if_with_increment, matches_loop_increment,
};
