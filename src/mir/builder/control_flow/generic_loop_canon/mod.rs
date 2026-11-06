//! Grouped owner for generic-loop canon helper surfaces.
//!
//! This module prevents many shallow generic-loop helper boxes from
//! accumulating directly under `control_flow/`.

pub(in crate::mir::builder) mod condition;
pub(in crate::mir::builder) mod step_extract;
pub(in crate::mir::builder) mod step_placement;
pub(crate) mod types;
pub(in crate::mir::builder) mod update;

pub(crate) use types::{ConditionCanon, UpdateCanon};
pub(crate) use update::canon_update_for_loop_var;
