//! RecipeTree - Minimal vocabulary for recursive lowering
//!
//! SSOT: docs/development/current/main/design/recipe-tree-and-parts-ssot.md
//! RecipeTree owns structure-only recipe vocabulary, route composers, and
//! recipe contract verification used by the current control-flow planner.

use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::stmt_view::StmtOnlyBlockRecipe;
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::recipes::RecipeBody;

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
    check_block_contract, ObligationState, PortType, VerifiedRecipeBlock,
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

/// Build a root block with single IfV2{Join} item.
pub(in crate::mir::builder) fn build_if_v2_join_root(
    arena: &mut RecipeBodies,
    if_body_id: BodyId,
    cond_view: CondBlockView,
    then_body: &[crate::ast::ASTNode],
    else_body: Option<&[crate::ast::ASTNode]>,
) -> Result<RecipeBlock, String> {
    // Register then body
    let then_body_id = arena.register(RecipeBody::new(then_body.to_vec()));

    // Build then block
    let then_block = build_stmt_only_block(then_body_id, then_body.len());

    // Build else block (if present)
    let else_block = else_body.map(|eb| {
        let else_body_id = arena.register(RecipeBody::new(eb.to_vec()));
        Box::new(build_stmt_only_block(else_body_id, eb.len()))
    });

    // Build root block with IfV2{Join}
    Ok(RecipeBlock::new(
        if_body_id,
        vec![RecipeItem::IfV2 {
            if_stmt: StmtRef::new(0),
            cond_view,
            contract: IfContractKind::Join,
            then_block: Box::new(then_block),
            else_block,
        }],
    ))
}

/// Build arena and root block for single If statement (IfV2{Join}).
pub(in crate::mir::builder) fn build_arena_and_if_v2_join_root_from_single_if_stmt(
    stmt: &crate::ast::ASTNode,
    cond_view: CondBlockView,
    then_body: &[crate::ast::ASTNode],
    else_body: Option<&[crate::ast::ASTNode]>,
) -> Result<(RecipeBodies, RecipeBlock), String> {
    let mut arena = RecipeBodies::new();
    let if_body_id = arena.register(RecipeBody::new(vec![stmt.clone()]));
    let root = build_if_v2_join_root(&mut arena, if_body_id, cond_view, then_body, else_body)?;
    Ok((arena, root))
}

pub(in crate::mir::builder) fn build_arena_and_loop_v0_root_from_single_loop_stmt(
    loop_stmt: &crate::ast::ASTNode,
    cond_view: CondBlockView,
    body: &RecipeBody,
    body_contract: BlockContractKind,
) -> Result<(RecipeBodies, RecipeBlock), String> {
    let mut arena = RecipeBodies::new();

    let loop_body_id = arena.register(RecipeBody::new(vec![loop_stmt.clone()]));
    let nested_body_id = arena.register(body.clone());

    let nested_block = match body_contract {
        BlockContractKind::StmtOnly => build_stmt_only_block(nested_body_id, body.len()),
        _ => {
            return Err(format!(
                "[freeze:contract][recipe] LoopV0 builder only supports StmtOnly: contract={:?}",
                body_contract
            ));
        }
    };

    let root = RecipeBlock::new(
        loop_body_id,
        vec![RecipeItem::LoopV0 {
            loop_stmt: StmtRef::new(0),
            kind: LoopKindV0::WhileLike,
            cond_view,
            body_block: Box::new(nested_block),
            body_contract,
            features: LoopV0Features::default(),
        }],
    );

    Ok((arena, root))
}

pub(in crate::mir::builder) fn build_arena_and_loop_v0_root_from_nested_stmt_only(
    loop_stmt: &crate::ast::ASTNode,
    cond_view: CondBlockView,
    body_stmt_only: StmtOnlyBlockRecipe,
    kind: LoopKindV0,
    features: LoopV0Features,
) -> Result<(RecipeBodies, RecipeBlock), String> {
    let mut arena = body_stmt_only.arena;

    let loop_body_id = arena.register(RecipeBody::new(vec![loop_stmt.clone()]));
    let nested_block = body_stmt_only.block;

    let root = RecipeBlock::new(
        loop_body_id,
        vec![RecipeItem::LoopV0 {
            loop_stmt: StmtRef::new(0),
            kind,
            cond_view,
            body_block: Box::new(nested_block),
            body_contract: BlockContractKind::StmtOnly,
            features,
        }],
    );

    Ok((arena, root))
}

// Common exports
pub(in crate::mir::builder) use common::{ExitKind, IfMode};
