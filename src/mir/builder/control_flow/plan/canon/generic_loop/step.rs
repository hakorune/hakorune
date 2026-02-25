mod extract;
mod placement;

pub(crate) use extract::canon_loop_increment_for_var;
pub(crate) use placement::{
    classify_step_placement, is_break_else_if_with_increment, is_continue_if_with_increment,
    matches_loop_increment,
};
