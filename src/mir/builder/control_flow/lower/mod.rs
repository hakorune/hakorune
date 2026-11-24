//! Top-level owner surface for control-flow lowering and orchestration.
//!
//! During folderization, implementations still live under `plan/`.
//! Non-`plan/` consumers should depend on this module first.


pub(in crate::mir::builder) mod normalize;



pub(in crate::mir::builder) use super::plan::planner::Freeze;
