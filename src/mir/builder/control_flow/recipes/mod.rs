//! Top-level owner surface for control-flow recipe infrastructure.
//!
//! During folderization, implementations still live under `plan/`.
//! Non-`plan/` consumers should depend on this module first.

pub(in crate::mir::builder) mod features;
pub(in crate::mir::builder) mod parts;
pub(in crate::mir::builder) mod recipe_tree;
pub(in crate::mir::builder) mod skeletons;
pub(in crate::mir::builder) mod steps;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::recipes::*;
#[allow(unused_imports)]
pub(in crate::mir::builder) use recipe_tree::*;
