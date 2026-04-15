//! Compatibility surface for the facts-owned statement views.
//!
//! Owner moved to `facts/stmt_view.rs`.

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::facts::stmt_view::{
    flatten_scope_boxes, try_build_stmt_only_block_recipe, StmtOnlyBlockRecipe,
};
