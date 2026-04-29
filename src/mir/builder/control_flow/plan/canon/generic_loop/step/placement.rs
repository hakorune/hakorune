mod decision;

pub(crate) use crate::mir::builder::control_flow::facts::canon::generic_loop::{
    is_break_else_if_with_increment, is_continue_if_with_increment, matches_loop_increment,
};
pub(crate) use decision::classify_step_placement;
