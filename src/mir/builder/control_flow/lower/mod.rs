//! Top-level owner surface for control-flow lowering and orchestration.
//!
//! During folderization, implementations still live under `plan/`.
//! Non-`plan/` consumers should depend on this module first.

pub(in crate::mir::builder) mod composer;
pub(in crate::mir::builder) mod normalize;
pub(in crate::mir::builder) mod planner;
pub(in crate::mir::builder) mod single_planner;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::lowerer::*;
