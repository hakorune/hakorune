mod decision;
mod matcher;

pub(crate) use decision::classify_step_placement;
pub(crate) use matcher::{
    is_break_else_if_with_increment, is_continue_if_with_increment, matches_loop_increment,
};
