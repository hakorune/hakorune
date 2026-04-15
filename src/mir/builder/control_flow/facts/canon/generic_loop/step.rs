pub(in crate::mir::builder) mod extract;
pub(in crate::mir::builder) mod placement;

pub(crate) use extract::canon_loop_increment_for_var;
pub(crate) use placement::{
    is_break_else_if_with_increment, is_continue_if_with_increment, matches_loop_increment,
};
