//! Grouped owner for generic-loop canon helper surfaces.
//!
//! This module prevents many shallow generic-loop helper boxes from
//! accumulating directly under `control_flow/`.

pub(in crate::mir::builder) mod condition;
pub(in crate::mir::builder) mod step_extract;
pub(in crate::mir::builder) mod step_placement;
pub(in crate::mir::builder) mod update;
