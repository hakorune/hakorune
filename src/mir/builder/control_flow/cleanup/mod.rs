//! Top-level owner surface for control-flow cleanup and policy helpers.
//!
//! During folderization, implementations still live under `plan/`.
//! Non-`plan/` consumers should depend on this module first.

pub(in crate::mir::builder) mod common;
pub(in crate::mir::builder) mod policies;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::common::*;
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::policies::*;
