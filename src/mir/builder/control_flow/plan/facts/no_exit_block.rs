//! Facts SSOT: no-exit RecipeBlock builder (IfJoin included).
//!
//! Purpose:
//! - Build a `RecipeBlock` for a statement list that must contain no non-local exits.
//! - Allow join-bearing `if` via `RecipeItem::IfV2 { contract: Join, .. }`.
//! - Allow nested loops via `RecipeItem::LoopV0` (structure-only, lowered via Parts).
//! - Keep the shape checks and vocabulary checks in Facts (no re-check in features).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    count_control_flow, ControlFlowDetector,
};
use crate::mir::builder::control_flow::plan::facts::block_policies::is_allowed_effect_stmt;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_in_arena;
use crate::mir::builder::control_flow::plan::facts::expr_bool::is_supported_bool_expr_with_canon;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, IfContractKind, LoopKindV0, LoopV0Features, RecipeBlock, RecipeBodies,
    RecipeItem,
};
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct NoExitBlockRecipe {
    pub arena: RecipeBodies,
    pub block: RecipeBlock,
}

pub(in crate::mir::builder) fn try_build_no_exit_block_recipe(
    stmts: &[ASTNode],
    allow_extended: bool,
) -> Option<NoExitBlockRecipe> {
    if stmts.is_empty() {
        return None;
    }
    if body_has_non_local_exit_outside_loops(stmts) {
        return None;
    }

    let mut arena = RecipeBodies::new();
    let block = build_no_exit_block(&mut arena, stmts, allow_extended)?;
    Some(NoExitBlockRecipe { arena, block })
}

fn build_no_exit_block(
    arena: &mut RecipeBodies,
    stmts: &[ASTNode],
    allow_extended: bool,
) -> Option<RecipeBlock> {
    let body_id = arena.register(RecipeBody::new(stmts.to_vec()));
    let mut items = Vec::with_capacity(stmts.len());

    for (idx, stmt) in stmts.iter().enumerate() {
        items.push(build_no_exit_item(arena, stmt, idx, allow_extended)?);
    }

    Some(RecipeBlock::new(body_id, items))
}

fn build_no_exit_item(
    arena: &mut RecipeBodies,
    stmt: &ASTNode,
    idx: usize,
    allow_extended: bool,
) -> Option<RecipeItem> {
    if stmt.contains_non_local_exit_outside_loops() {
        return None;
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
            if body_has_non_local_exit_outside_loops(body) {
                return None;
            }
            let body_block = try_build_exit_allowed_block_in_arena(arena, body, allow_extended)?;
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
            if body_has_non_local_exit_outside_loops(then_body) {
                return None;
            }
            if else_body
                .as_ref()
                .is_some_and(|eb| body_has_non_local_exit_outside_loops(eb))
            {
                return None;
            }

            let then_block = build_no_exit_block(arena, then_body, allow_extended)?;

            let else_block = match else_body.as_ref() {
                Some(eb) => Some(Box::new(build_no_exit_block(arena, eb, allow_extended)?)),
                None => None,
            };

            Some(RecipeItem::IfV2 {
                if_stmt: StmtRef::new(idx),
                cond_view: CondBlockView::from_expr(condition),
                contract: IfContractKind::Join,
                then_block: Box::new(then_block),
                else_block,
            })
        }
        _ => {
            // Allow explicit sub-block statements in NoExit recipes as long as they contain no
            // non-local exits (Facts-level observation above).
            if matches!(stmt, ASTNode::Program { .. } | ASTNode::ScopeBox { .. }) {
                return Some(RecipeItem::Stmt(StmtRef::new(idx)));
            }
            if !is_allowed_effect_stmt(stmt, allow_extended) {
                return None;
            }
            Some(RecipeItem::Stmt(StmtRef::new(idx)))
        }
    }
}

fn body_has_non_local_exit_outside_loops(body: &[ASTNode]) -> bool {
    let mut detector = ControlFlowDetector::default();
    detector.count_returns = true;
    let counts = count_control_flow(body, detector);
    if counts.return_count > 0 {
        return true;
    }
    body.iter()
        .any(ASTNode::contains_non_local_exit_outside_loops)
}
