//! Analysis-only canonical views for generic loop facts (no rewrite).

mod condition;
mod step;
mod types;
mod update;

pub(crate) use condition::canon_condition_for_generic_loop_v0;
pub(crate) use step::{
    canon_loop_increment_for_var, classify_step_placement, is_break_else_if_with_increment,
    is_continue_if_with_increment, matches_loop_increment,
};
pub(crate) use types::{ConditionCanon, StepPlacement, StepPlacementDecision, UpdateCanon};
pub(crate) use update::canon_update_for_loop_var;
