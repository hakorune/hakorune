//! Grouped owner for generic-loop canon helper surfaces.
//!
//! This module prevents many shallow generic-loop helper boxes from
//! accumulating directly under `control_flow/`.

pub(in crate::mir::builder) mod condition;
pub(in crate::mir::builder) mod step_extract;
pub(in crate::mir::builder) mod step_placement;
pub(crate) mod types;
pub(in crate::mir::builder) mod update;

pub(crate) use condition::canon_condition_for_generic_loop_v0;
pub(crate) use step_extract::canon_loop_increment_for_var;
pub(crate) use step_placement::facts::{
    is_break_else_if_with_increment, is_continue_if_with_increment, matches_loop_increment,
};
pub(crate) use step_placement::plan::classify_step_placement;
pub(crate) use types::{ConditionCanon, StepPlacement, StepPlacementDecision, UpdateCanon};
pub(crate) use update::canon_update_for_loop_var;
