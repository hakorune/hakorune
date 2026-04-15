//! Top-level descriptive owner surface for analysis-only canon helpers.
//!
//! During folderization, generic-loop update observation lives here first.
//! The remaining condition-oriented canon modules still stay behind plan-side
//! compatibility re-exports until they get their own actual move.

pub(in crate::mir::builder) mod generic_loop;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::canon::{
    cond, cond_block_view,
};
