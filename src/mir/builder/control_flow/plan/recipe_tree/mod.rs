//! RecipeTree - Minimal vocabulary for recursive lowering
//!
//! SSOT: docs/development/current/main/design/recipe-tree-and-parts-ssot.md
//! RecipeTree owns structure-only recipe vocabulary, route composers, and
//! recipe contract verification used by the current control-flow planner.

use crate::mir::builder::control_flow::recipes::refs::StmtRef;

mod block;
mod common;
mod join_scope;
// Re-export block types explicitly (use block::* doesn't work with visibility)
pub(in crate::mir::builder) use block::{
    BlockContractKind, BodyId, IfContractKind, LoopKindV0, LoopV0Features, RecipeBlock,
    RecipeBodies, RecipeItem,
};
pub(in crate::mir::builder) use join_scope::collect_branch_local_vars_from_block_recursive;

// Builder modules stay private; callers enter through composer/matcher owners.
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
mod split_scan_builder;

// Composer modules attach route-specific methods to RecipeComposer.
mod accum_const_loop_composer;
mod bool_predicate_scan_composer;
mod generic_loop_composer;
mod if_phi_join_composer;
mod loop_break_composer;
mod loop_cond_composer;
mod loop_continue_only_composer;
mod loop_simple_while_composer;
mod loop_true_composer;
mod loop_true_early_exit_composer;
mod scan_with_init_composer;
mod split_scan_composer;

mod contracts;
mod matcher;
pub(in crate::mir::builder) use matcher::RecipeMatcher;
mod verified;
pub(in crate::mir::builder) use contracts::{RecipeContract, RecipeContractKind, StmtConstraint};
pub(in crate::mir::builder::control_flow::plan) use verified::verify_block_contract_with_pre;
pub(in crate::mir::builder) use verified::{
    check_block_contract, verify_port_sig_obligations_if_enabled, VerifiedRecipeBlock,
};

// ===== RecipeComposer route entry facade =====
pub(in crate::mir::builder) struct RecipeComposer;

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
