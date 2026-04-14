//! Loop parts (scaffold).
//!
//! Purpose (L0):
//! - Provide a Parts entry for lowering a loop body represented as `RecipeBlock`.
//! - Keep the contract explicit and fail-fast (no silent fallback).
//!
//! NOTE:
//! - This is an implementation-prep step. Producers are unchanged.

mod analysis;
mod body_block;
mod debug;
mod loop_v0;
mod nested_depth1;
mod vars;

pub(in crate::mir::builder) type LoopBodyContractKind =
    crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

#[allow(unused_imports)]
pub(in crate::mir::builder) use body_block::{
    lower_loop_with_body_block, lower_loop_with_body_block_with_break_phi_dsts,
    lower_loop_with_exit_only_body_block,
};
#[allow(unused_imports)]
pub(in crate::mir::builder) use loop_v0::lower_loop_v0;
#[allow(unused_imports)]
pub(in crate::mir::builder) use nested_depth1::{
    lower_nested_loop_depth1_stmt_only, lower_nested_loop_recipe_stmt_only,
    try_lower_nested_loop_depth1_stmt_only_fastpath,
};
