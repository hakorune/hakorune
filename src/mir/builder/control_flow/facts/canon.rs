//! Facts-side owner surface for analysis-only canon helpers.
//!
//! `generic_loop` already owns its facts-side update helper here. The remaining
//! condition-oriented canon modules still forward from `plan::canon` until they
//! get their own move.

pub(in crate::mir::builder) mod generic_loop;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::canon::cond;
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::canon::cond_block_view;
