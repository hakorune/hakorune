//! Compatibility surface for the facts-owned no-exit block recipe builder.
//!
//! Owner moved to `facts/no_exit_block.rs`.

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::facts::no_exit_block::{
    try_build_no_exit_block_recipe, NoExitBlockRecipe,
};
