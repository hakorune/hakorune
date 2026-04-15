//! Facts-side owner for generic-loop canon helpers.
//!
//! Update observation is actualized here. Other generic-loop canon pieces still
//! forward from `plan::canon::generic_loop`, but the forwarded owner surface is
//! spelled out explicitly here.

mod update;

pub(crate) use update::canon_update_for_loop_var;

#[allow(unused_imports)]
pub(crate) use crate::mir::builder::control_flow::plan::canon::generic_loop::{
    ConditionCanon, StepPlacement, StepPlacementDecision, UpdateCanon,
    canon_condition_for_generic_loop_v0, canon_loop_increment_for_var, classify_step_placement,
    is_break_else_if_with_increment, is_continue_if_with_increment, matches_loop_increment,
};
