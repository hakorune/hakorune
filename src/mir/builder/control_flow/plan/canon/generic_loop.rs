//! Analysis-only canonical views for generic loop facts (no rewrite).

mod condition;
mod step;
mod types;

#[allow(unused_imports)]
pub(crate) use crate::mir::builder::control_flow::facts::canon::generic_loop::canon_update_for_loop_var;
#[allow(unused_imports)]
pub(crate) use crate::mir::builder::control_flow::facts::canon::generic_loop::types::{
    ConditionCanon, UpdateCanon,
};
#[allow(unused_imports)]
pub(crate) use condition::canon_condition_for_generic_loop_v0;
#[allow(unused_imports)]
pub(crate) use step::{
    canon_loop_increment_for_var, classify_step_placement, is_break_else_if_with_increment,
    is_continue_if_with_increment, matches_loop_increment,
};
#[allow(unused_imports)]
pub(crate) use types::{StepPlacement, StepPlacementDecision};
