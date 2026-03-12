//! RecipeBlock dispatch (M5m-2).
//!
//! Lowers RecipeBlock via arena lookup.
//!
//! Modular structure:
//! - block.rs: Core block lowering (types, BlockKindInternal, lower_block_internal, entry points)
//! - if_join.rs: If-join lowering (join payload handling)
//! - if_exit_only.rs: Exit-only if lowering (exit-focused handling)

mod block;
mod if_exit_only;
mod if_join;

// Re-export public entry points
pub(in crate::mir::builder) use block::{
    lower_exit_allowed_block_verified, lower_exit_only_block_verified,
    lower_no_exit_block_verified, lower_no_exit_block_with_stmt_lowerer_verified,
    lower_stmt_only_block, plans_exit_on_all_paths,
};

pub(in crate::mir::builder) use if_join::{
    lower_if_join_with_branch_lowerers, lower_value_cond_if_with_filtered_joins,
};
