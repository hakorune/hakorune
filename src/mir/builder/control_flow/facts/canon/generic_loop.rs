//! Facts-side owner for generic-loop canon helpers.
//!
//! Condition/update canon types and related analysis-only helpers are owned
//! here. Plan-owned placement decisions stay under `plan::canon::generic_loop`.

pub(in crate::mir::builder) mod condition;
pub(in crate::mir::builder) mod step;
pub(in crate::mir::builder) mod types;
mod update;

pub(crate) use condition::canon_condition_for_generic_loop_v0;
pub(crate) use step::{
    canon_loop_increment_for_var, is_break_else_if_with_increment, is_continue_if_with_increment,
    matches_loop_increment,
};
#[allow(unused_imports)]
pub(crate) use types::{ConditionCanon, UpdateCanon};
pub(crate) use update::canon_update_for_loop_var;
