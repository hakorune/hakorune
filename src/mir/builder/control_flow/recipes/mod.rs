//! Top-level owner surface for control-flow recipe infrastructure.
//!
//! During folderization, implementations still live under `plan/`.
//! Non-`plan/` consumers should depend on this module first.

pub(in crate::mir::builder) mod features;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::recipes::{
    refs, RecipeBody, StmtIdx, StmtRange,
};
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
