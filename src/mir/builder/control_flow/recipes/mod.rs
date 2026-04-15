//! Top-level owner surface for control-flow recipe infrastructure.
//!
//! During folderization, recipe types still live under `plan/`.
//! Non-`plan/` consumers should depend on this module first.

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::recipes::{
    refs, RecipeBody, StmtIdx, StmtRange,
};
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
