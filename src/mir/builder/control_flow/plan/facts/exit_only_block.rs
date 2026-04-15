//! Facts SSOT: exit-only RecipeBlock builder.
//!
//! Purpose:
//! - Build a `RecipeBlock` representing an "exit-only" statement list (may contain nested if trees).
//! - Keep the acceptance vocabulary and condition checks SSOT-backed and fail-fast (no rewrite).
//!
//! Notes:
//! - This builder is analysis-only: it clones AST nodes into a fresh `RecipeBodies` arena.
//! - `IfMode::ExitIf` (no else) is allowed as an intermediate item, but it does not count as
//!   "ends with exit on all paths" unless it is `ExitAll`.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::block_policies::is_allowed_effect_stmt;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::expr_bool::is_supported_bool_expr_with_canon;
use crate::mir::builder::control_flow::plan::parts::exit_kind_depth_view::ExitKindDepthView;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, ExitKind, IfContractKind, IfMode, LoopKindV0, LoopV0Features, RecipeBlock,
    RecipeBodies, RecipeItem,
};
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::recipes::RecipeBody;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct ExitOnlyBlockRecipe {
    pub arena: RecipeBodies,
    pub block: RecipeBlock,
}

impl ExitOnlyBlockRecipe {
    pub fn ends_with_exit_on_all_paths(&self) -> bool {
        exit_only_block_ends_with_exit_on_all_paths(&self.arena, &self.block)
    }
}

/// Try to build an exit-only block recipe from a statement list.
///
/// Contract (v1):
/// - Allowed non-exit statements are limited to `cond_prelude_vocab` (plus an allowlist tweak for
///   `Print` based on `allow_extended` for compatibility with existing boxes).
/// - Allowed exits: `break/continue/return`.
/// - Allowed nested control-flow: `if` only, when its condition is a supported bool expr and its
///   branches are themselves exit-only blocks.
pub(in crate::mir::builder) fn try_build_exit_only_block_recipe(
    stmts: &[ASTNode],
    allow_extended: bool,
) -> Option<ExitOnlyBlockRecipe> {
    if stmts.is_empty() {
        return None;
    }

    let mut arena = RecipeBodies::new();
    let block = build_exit_only_block(&mut arena, stmts, allow_extended)?;
    Some(ExitOnlyBlockRecipe { arena, block })
}

/// Exit-allowed RecipeBlock builder (Facts SSOT).
///
/// Purpose:
/// - Reuse the "exit-only vocabulary" builder, but without the "must end with exit on all paths"
///   contract. This enables recipe-first lowering for blocks that may fall through.
///
/// Contract (v1):
/// - Same item vocabulary as `ExitOnlyBlockRecipe` (Stmt / Exit / IfV2{ExitOnly}).
/// - No `ends_with_exit_on_all_paths` requirement.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct ExitAllowedBlockRecipe {
    pub arena: RecipeBodies,
    pub block: RecipeBlock,
}

pub(in crate::mir::builder) fn try_build_exit_allowed_block_recipe(
    stmts: &[ASTNode],
    allow_extended: bool,
) -> Option<ExitAllowedBlockRecipe> {
    if stmts.is_empty() {
        return None;
    }

    let mut arena = RecipeBodies::new();
    let block = build_exit_allowed_block(&mut arena, stmts, allow_extended)?;
    Some(ExitAllowedBlockRecipe { arena, block })
}

/// Build an exit-allowed block directly into an existing arena.
///
/// This is required when embedding the resulting `RecipeBlock` into another recipe,
/// because `RecipeBlock::body_id` is an index into the provided `arena`.
pub(in crate::mir::builder) fn try_build_exit_allowed_block_in_arena(
    arena: &mut RecipeBodies,
    stmts: &[ASTNode],
    allow_extended: bool,
) -> Option<RecipeBlock> {
    if stmts.is_empty() {
        return None;
    }
    build_exit_allowed_block(arena, stmts, allow_extended)
}

fn build_exit_item_from_stmt(stmt: &ASTNode, idx: usize) -> Option<RecipeItem> {
    let kind = match stmt {
        ASTNode::Break { .. } => ExitKind::Break { depth: 1 },
        ASTNode::Continue { .. } => ExitKind::Continue { depth: 1 },
        ASTNode::Return { .. } => ExitKind::Return,
        _ => return None,
    };
    let view = ExitKindDepthView::from_recipe_exit_kind(kind);
    Some(RecipeItem::Exit {
        kind: view.kind,
        stmt: StmtRef::new(idx),
    })
}

fn recipe_stmt(idx: usize) -> RecipeItem {
    RecipeItem::Stmt(StmtRef::new(idx))
}

fn build_exit_allowed_block(
    arena: &mut RecipeBodies,
    stmts: &[ASTNode],
    allow_extended: bool,
) -> Option<RecipeBlock> {
    let body_id = arena.register(RecipeBody::new(stmts.to_vec()));
    let mut items = Vec::with_capacity(stmts.len());

    for (idx, stmt) in stmts.iter().enumerate() {
        items.push(build_exit_allowed_item(arena, stmt, idx, allow_extended)?);
    }

    Some(RecipeBlock::new(body_id, items))
}

fn build_exit_allowed_item(
    arena: &mut RecipeBodies,
    stmt: &ASTNode,
    idx: usize,
    allow_extended: bool,
) -> Option<RecipeItem> {
    if let Some(exit_item) = build_exit_item_from_stmt(stmt, idx) {
        return Some(exit_item);
    }

    match stmt {
        ASTNode::Loop {
            condition, body, ..
        }
        | ASTNode::While {
            condition, body, ..
        } => {
            if !is_supported_bool_expr_with_canon(condition, allow_extended) {
                return None;
            }
            let body_block = build_exit_allowed_block(arena, body, allow_extended)?;
            Some(RecipeItem::LoopV0 {
                loop_stmt: StmtRef::new(idx),
                cond_view: CondBlockView::from_expr(condition),
                body_block: Box::new(body_block),
                body_contract: BlockContractKind::ExitAllowed,
                kind: LoopKindV0::WhileLike,
                features: LoopV0Features::default(),
            })
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            if !is_supported_bool_expr_with_canon(condition, allow_extended) {
                return None;
            }

            // ExitAllowed blocks must still model *exit-bearing* if trees explicitly, otherwise
            // loop-body lowering cannot wire break/continue/return correctly.
            //
            // For join-bearing / effect-only `if`, we intentionally keep it as a plain `Stmt`
            // and let Parts lower it via `lower_return_prelude_stmt` (planner-required join-if).

            // Try to build then as exit-only first
            let then_exit_only = build_exit_only_block(arena, then_body, allow_extended);
            let then_exits_all = then_exit_only
                .as_ref()
                .is_some_and(|b| exit_only_block_ends_with_exit_on_all_paths(arena, b));

            if then_exits_all {
                // Case 1: then exits on all paths → existing ExitOnly logic
                let then_exit_only = then_exit_only.unwrap();
                let (mode, else_block) = if let Some(else_body) = else_body {
                    let else_exit_only = build_exit_only_block(arena, else_body, allow_extended)?;
                    if !exit_only_block_ends_with_exit_on_all_paths(arena, &else_exit_only) {
                        return Some(recipe_stmt(idx));
                    }
                    (IfMode::ExitAll, Some(Box::new(else_exit_only)))
                } else {
                    (IfMode::ExitIf, None)
                };

                Some(RecipeItem::IfV2 {
                    if_stmt: StmtRef::new(idx),
                    cond_view: CondBlockView::from_expr(condition),
                    contract: IfContractKind::ExitOnly { mode },
                    then_block: Box::new(then_exit_only),
                    else_block,
                })
            } else if let Some(else_body) = else_body {
                // Case 2: then doesn't exit, but else might → ElseOnlyExit shape
                // Shape: if cond { stmts } else { break/continue/return }
                //
                // If else is not exit-only, treat the whole if as a stmt (join-bearing).
                let else_exit_only = build_exit_only_block(arena, else_body, allow_extended);
                let Some(else_exit_only) = else_exit_only else {
                    return Some(recipe_stmt(idx));
                };
                if !exit_only_block_ends_with_exit_on_all_paths(arena, &else_exit_only) {
                    return Some(recipe_stmt(idx));
                }

                // then is no-exit (fallthrough), build as exit-allowed block
                let then_no_exit = build_exit_allowed_block(arena, then_body, allow_extended)?;

                Some(RecipeItem::IfV2 {
                    if_stmt: StmtRef::new(idx),
                    cond_view: CondBlockView::from_expr(condition),
                    contract: IfContractKind::ExitAllowed {
                        mode: IfMode::ElseOnlyExit,
                    },
                    then_block: Box::new(then_no_exit),
                    else_block: Some(Box::new(else_exit_only)),
                })
            } else {
                // Case 3: then doesn't exit, no else → fallback to Stmt
                Some(recipe_stmt(idx))
            }
        }
        // Container statements are lowered via return-prelude container logic (Facts SSOT).
        ASTNode::Program { .. } | ASTNode::ScopeBox { .. } => Some(recipe_stmt(idx)),
        _ => {
            if !is_allowed_effect_stmt(stmt, allow_extended) {
                return None;
            }
            Some(recipe_stmt(idx))
        }
    }
}

fn build_exit_only_block(
    arena: &mut RecipeBodies,
    stmts: &[ASTNode],
    allow_extended: bool,
) -> Option<RecipeBlock> {
    let body_id = arena.register(RecipeBody::new(stmts.to_vec()));
    let mut items = Vec::with_capacity(stmts.len());

    for (idx, stmt) in stmts.iter().enumerate() {
        items.push(build_exit_only_item(arena, stmt, idx, allow_extended)?);
    }

    Some(RecipeBlock::new(body_id, items))
}

fn build_exit_only_item(
    arena: &mut RecipeBodies,
    stmt: &ASTNode,
    idx: usize,
    allow_extended: bool,
) -> Option<RecipeItem> {
    if let Some(exit_item) = build_exit_item_from_stmt(stmt, idx) {
        return Some(exit_item);
    }

    match stmt {
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            // Condition must be a supported bool expr (Facts SSOT).
            if !is_supported_bool_expr_with_canon(condition, allow_extended) {
                return None;
            }

            let then_block = build_exit_only_block(arena, then_body, allow_extended)?;
            if !exit_only_block_ends_with_exit_on_all_paths(arena, &then_block) {
                return None;
            }

            let (mode, else_block) = if let Some(else_body) = else_body {
                let else_block = build_exit_only_block(arena, else_body, allow_extended)?;
                if !exit_only_block_ends_with_exit_on_all_paths(arena, &else_block) {
                    return None;
                }
                (IfMode::ExitAll, Some(Box::new(else_block)))
            } else {
                (IfMode::ExitIf, None)
            };

            Some(RecipeItem::IfV2 {
                if_stmt: StmtRef::new(idx),
                cond_view: CondBlockView::from_expr(condition),
                contract: IfContractKind::ExitOnly { mode },
                then_block: Box::new(then_block),
                else_block,
            })
        }
        _ => {
            // Compatibility: exit-only recipes use a narrow effect vocabulary.
            // Policy is centralized in `facts::block_policies`.
            if !is_allowed_effect_stmt(stmt, allow_extended) {
                return None;
            }
            Some(recipe_stmt(idx))
        }
    }
}

pub(in crate::mir::builder) fn exit_only_block_ends_with_exit_on_all_paths(
    arena: &RecipeBodies,
    block: &RecipeBlock,
) -> bool {
    let Some(body) = arena.get(block.body_id) else {
        return false;
    };
    if body.len() != block.items.len() {
        return false;
    }

    let Some(last) = block.items.last() else {
        return false;
    };
    exit_only_item_exits_on_all_paths(arena, last)
}

fn exit_only_item_exits_on_all_paths(arena: &RecipeBodies, item: &RecipeItem) -> bool {
    match item {
        RecipeItem::Exit { .. } => true,
        RecipeItem::IfV2 {
            contract,
            then_block,
            else_block,
            ..
        } => {
            match contract {
                IfContractKind::ExitOnly { mode } => match mode {
                    IfMode::ExitAll => {
                        let Some(else_block) = else_block else {
                            return false;
                        };
                        exit_only_block_ends_with_exit_on_all_paths(arena, then_block)
                            && exit_only_block_ends_with_exit_on_all_paths(arena, else_block)
                    }
                    IfMode::ExitIf | IfMode::ElseOnlyExit => false,
                },
                // ElseOnlyExit: then falls through, so not exit-all
                IfContractKind::ExitAllowed {
                    mode: IfMode::ElseOnlyExit,
                } => false,
                _ => false,
            }
        }
        _ => false,
    }
}
