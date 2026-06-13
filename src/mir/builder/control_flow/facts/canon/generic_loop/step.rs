pub(in crate::mir::builder) mod extract;
pub(in crate::mir::builder) mod placement;

#[allow(unused_imports)]
pub(crate) use extract::canon_loop_increment_for_var;
#[allow(unused_imports)]
pub(crate) use placement::{
    is_break_else_if_with_increment, is_continue_if_with_increment, matches_loop_increment,
};
