//! Top-level descriptive owner for generic-loop canon helpers.
//!
//! Only update observation is actualized here for now. The rest still routes
//! through plan-side compat until later folderization passes.

mod update;

#[allow(unused_imports)]
pub(crate) use crate::mir::builder::control_flow::plan::canon::generic_loop::{
    canon_condition_for_generic_loop_v0, canon_loop_increment_for_var,
    classify_step_placement, is_break_else_if_with_increment,
    is_continue_if_with_increment, matches_loop_increment, ConditionCanon,
    StepPlacement, StepPlacementDecision, UpdateCanon,
};
pub(crate) use update::canon_update_for_loop_var;
