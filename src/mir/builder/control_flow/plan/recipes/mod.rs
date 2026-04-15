//! Compatibility surface for recipes-owned base recipe types.
//!
//! Owner moved to `control_flow/recipes/`.

pub mod refs;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::recipes::{
    RecipeBody, StmtIdx, StmtRange,
};
