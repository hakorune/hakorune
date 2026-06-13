//! Analysis-only canonical views for generic loop facts (no rewrite).

mod condition;
mod step;
mod types;

#[allow(unused_imports)]
pub(crate) use crate::mir::builder::control_flow::generic_loop_canon::{
    canon_condition_for_generic_loop_v0, canon_loop_increment_for_var, canon_update_for_loop_var,
    classify_step_placement, is_break_else_if_with_increment, is_continue_if_with_increment,
    matches_loop_increment, ConditionCanon, StepPlacement, StepPlacementDecision, UpdateCanon,
};
#[allow(unused_imports)]
pub(crate) use condition::canon_condition_for_generic_loop_v0 as legacy_condition_facade;
#[allow(unused_imports)]
pub(crate) use step::{
    canon_loop_increment_for_var as legacy_step_increment_facade,
    classify_step_placement as legacy_step_placement_facade,
    is_break_else_if_with_increment as legacy_break_else_step_facade,
    is_continue_if_with_increment as legacy_continue_step_facade,
    matches_loop_increment as legacy_matches_loop_increment_facade,
};
#[allow(unused_imports)]
pub(crate) use types::{
    StepPlacement as LegacyStepPlacement, StepPlacementDecision as LegacyStepPlacementDecision,
};
