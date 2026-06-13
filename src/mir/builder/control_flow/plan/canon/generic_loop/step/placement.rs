mod decision;

pub(crate) use crate::mir::builder::control_flow::generic_loop_canon::{
    classify_step_placement, is_break_else_if_with_increment, is_continue_if_with_increment,
    matches_loop_increment,
};
#[allow(unused_imports)]
pub(crate) use decision::classify_step_placement as legacy_decision_facade;
