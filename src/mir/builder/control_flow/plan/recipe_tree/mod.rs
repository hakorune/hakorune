//! RecipeTree - Minimal vocabulary for recursive lowering
//!
//! SSOT: docs/development/current/main/design/recipe-tree-and-parts-ssot.md
//! RecipeTree owns structure-only recipe vocabulary, route builders, and
//! recipe contract verification used by the current control-flow planner.

use crate::mir::builder::control_flow::recipes::refs::StmtRef;

mod block;
mod common;
// Re-export block types explicitly (use block::* doesn't work with visibility)
pub(in crate::mir::builder) use block::{
    BlockContractKind, BodyId, IfContractKind, LoopKindV0, LoopV0Features, RecipeBlock,
    RecipeBodies, RecipeItem,
};

/// Route-neutral owned bundle produced by builder-local Recipe constructors.
///
/// This is intentionally distinct from the portable verified Recipe artifact:
/// route builders still own their Facts/AST reconstruction and admission
/// policy.  The bundle only removes the repeated `{ arena, root }` shell.
#[derive(Debug)]
pub(in crate::mir::builder) struct BuiltRecipeTree {
    pub arena: RecipeBodies,
    pub root: RecipeBlock,
}

// Builder modules stay private; callers enter through the matcher owner.
mod accum_const_loop_builder;
mod array_join_builder;
mod bool_predicate_scan_builder;
mod char_map_builder;
mod if_phi_join_builder;
mod loop_break_builder;
mod loop_continue_only_builder;
mod loop_simple_while_builder;
mod loop_true_early_exit_builder;
mod scan_with_init_builder;

mod contracts;
mod matcher;
pub(in crate::mir::builder) use matcher::RecipeMatcher;
mod verified;
pub(in crate::mir::builder) use contracts::{RecipeContract, RecipeContractKind};


// ===== Shared RecipeBlock construction helpers =====

/// Build a block containing only Stmt items.
pub(in crate::mir::builder) fn build_stmt_only_block(
    body_id: BodyId,
    stmt_count: usize,
) -> RecipeBlock {
    let items = (0..stmt_count)
        .map(|idx| RecipeItem::Stmt(StmtRef::new(idx)))
        .collect();
    RecipeBlock::new(body_id, items)
}

// Common exports
pub(in crate::mir::builder) use common::{ExitKind, IfMode};
