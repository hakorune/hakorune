//! Facts SSOT: return-prelude container recipe builder.
//!
//! Purpose:
//! - Keep "container lowering" (Program/ScopeBox wrappers, loop statements) on the Facts side.
//! - Enable Parts to lower return preludes via RecipeBlocks (no AST-direct recursion in Parts).
//!
//! Notes:
//! - This is an analysis-only view: it clones existing AST nodes but does not rewrite or mutate the
//!   original AST.
//! - The recipe builder here is intentionally scoped to return-prelude usage and may allow a
//!   slightly different vocabulary than general-purpose block recipes (to avoid accidental drift).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe;
use crate::mir::builder::control_flow::plan::facts::expr_bool::is_supported_bool_expr_with_canon;
use crate::mir::builder::control_flow::plan::facts::no_exit_block::{
    try_build_no_exit_block_recipe, NoExitBlockRecipe,
};
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, ExitKind, LoopKindV0, LoopV0Features, RecipeBlock, RecipeBodies, RecipeItem,
};
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
use crate::mir::policies::return_prelude_policy::return_prelude_stmt_is_allowed;

/// Flatten `Program` and `ScopeBox` wrappers into a single sequential statement list.
///
/// This is an analysis-only view: it clones existing AST nodes but does not rewrite or mutate the
/// original AST.
///
/// Notes:
/// - This is intended for *container* lowering sites (e.g. return-prelude) where `Program/ScopeBox`
///   are treated as statement containers in JoinIR plan lowering.
fn flatten_block_containers(body: &[ASTNode]) -> Vec<ASTNode> {
    let mut out = Vec::new();

    fn strip_stmt(node: &ASTNode) -> ASTNode {
        match node {
            ASTNode::If {
                condition,
                then_body,
                else_body,
                span,
            } => ASTNode::If {
                condition: condition.clone(),
                then_body: flatten_block_containers(then_body),
                else_body: else_body
                    .as_ref()
                    .map(|body| flatten_block_containers(body)),
                span: span.clone(),
            },
            ASTNode::Loop {
                condition,
                body,
                span,
            } => ASTNode::Loop {
                condition: condition.clone(),
                body: flatten_block_containers(body),
                span: span.clone(),
            },
            ASTNode::While {
                condition,
                body,
                span,
            } => ASTNode::While {
                condition: condition.clone(),
                body: flatten_block_containers(body),
                span: span.clone(),
            },
            ASTNode::ForRange {
                var_name,
                start,
                end,
                body,
                span,
            } => ASTNode::ForRange {
                var_name: var_name.clone(),
                start: start.clone(),
                end: end.clone(),
                body: flatten_block_containers(body),
                span: span.clone(),
            },
            ASTNode::Lambda { params, body, span } => ASTNode::Lambda {
                params: params.clone(),
                body: flatten_block_containers(body),
                span: span.clone(),
            },
            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                span,
            } => ASTNode::TryCatch {
                try_body: flatten_block_containers(try_body),
                catch_clauses: catch_clauses
                    .iter()
                    .map(|clause| crate::ast::CatchClause {
                        exception_type: clause.exception_type.clone(),
                        variable_name: clause.variable_name.clone(),
                        body: flatten_block_containers(&clause.body),
                        span: clause.span,
                    })
                    .collect(),
                finally_body: finally_body
                    .as_ref()
                    .map(|body| flatten_block_containers(body)),
                span: span.clone(),
            },
            _ => node.clone(),
        }
    }

    fn push_node(node: &ASTNode, out: &mut Vec<ASTNode>) {
        match node {
            ASTNode::Program { statements, .. } => {
                for inner in statements {
                    push_node(inner, out);
                }
            }
            ASTNode::ScopeBox { body, .. } => {
                for inner in body {
                    push_node(inner, out);
                }
            }
            _ => out.push(strip_stmt(node)),
        }
    }

    for stmt in body {
        push_node(stmt, &mut out);
    }

    out
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum ReturnPreludeContainerRecipe {
    NoExit(NoExitBlockRecipe),
    ExitAllowed(ExitAllowedBlockRecipe),
}

fn try_build_exit_allowed_return_prelude_recipe(
    stmts: &[ASTNode],
    allow_extended: bool,
) -> Option<ExitAllowedBlockRecipe> {
    if stmts.is_empty() {
        return None;
    }

    fn build_block(
        arena: &mut RecipeBodies,
        stmts: &[ASTNode],
        allow_extended: bool,
    ) -> Option<RecipeBlock> {
        let body_id = arena.register(RecipeBody::new(stmts.to_vec()));
        let mut items = Vec::with_capacity(stmts.len());
        for (idx, stmt) in stmts.iter().enumerate() {
            items.push(build_item(arena, stmt, idx, allow_extended)?);
        }
        Some(RecipeBlock::new(body_id, items))
    }

    fn build_item(
        arena: &mut RecipeBodies,
        stmt: &ASTNode,
        idx: usize,
        allow_extended: bool,
    ) -> Option<RecipeItem> {
        match stmt {
            ASTNode::Break { .. } => Some(RecipeItem::Exit {
                kind: ExitKind::Break { depth: 1 },
                stmt: StmtRef::new(idx),
            }),
            ASTNode::Continue { .. } => Some(RecipeItem::Exit {
                kind: ExitKind::Continue { depth: 1 },
                stmt: StmtRef::new(idx),
            }),
            ASTNode::Return { .. } => Some(RecipeItem::Exit {
                kind: ExitKind::Return,
                stmt: StmtRef::new(idx),
            }),
            ASTNode::Loop {
                condition, body, ..
            }
            | ASTNode::While {
                condition, body, ..
            } => {
                if !is_supported_bool_expr_with_canon(condition, allow_extended) {
                    return None;
                }
                let body_block = build_block(arena, body, allow_extended)?;
                Some(RecipeItem::LoopV0 {
                    loop_stmt: StmtRef::new(idx),
                    cond_view: CondBlockView::from_expr(condition),
                    body_block: Box::new(body_block),
                    body_contract: BlockContractKind::ExitAllowed,
                    kind: LoopKindV0::WhileLike,
                    features: LoopV0Features::default(),
                })
            }
            // Container wrappers must be flattened before recipe construction.
            ASTNode::Program { .. } | ASTNode::ScopeBox { .. } => None,
            _ => {
                if return_prelude_stmt_is_allowed(stmt, allow_extended) {
                    Some(RecipeItem::Stmt(StmtRef::new(idx)))
                } else {
                    None
                }
            }
        }
    }

    let mut arena = RecipeBodies::new();
    let block = build_block(&mut arena, stmts, allow_extended)?;
    Some(ExitAllowedBlockRecipe { arena, block })
}

fn build_from_flat(flat: &[ASTNode], allow_extended: bool) -> Option<ReturnPreludeContainerRecipe> {
    if let Some(no_exit) = try_build_no_exit_block_recipe(flat, allow_extended) {
        return Some(ReturnPreludeContainerRecipe::NoExit(no_exit));
    }
    try_build_exit_allowed_return_prelude_recipe(flat, allow_extended)
        .map(ReturnPreludeContainerRecipe::ExitAllowed)
}

/// Facts SSOT: Try to build a return-prelude recipe from a *container* statement.
///
/// Contract:
/// - Returns `None` for non-container statements (callers should use normal stmt lowering).
/// - When returns `Some`, the returned recipe is suitable for return-prelude lowering:
///   `NoExit` when possible; otherwise `ExitAllowed` as a compatibility fallback for containers that
///   contain conditional returns.
pub(in crate::mir::builder) fn try_build_return_prelude_container_recipe(
    stmt: &ASTNode,
    allow_extended: bool,
) -> Option<ReturnPreludeContainerRecipe> {
    match stmt {
        ASTNode::Program { statements, .. } => {
            let flat = flatten_block_containers(statements);
            build_from_flat(&flat, allow_extended)
        }
        ASTNode::ScopeBox { body, .. } => {
            let flat = flatten_block_containers(body);
            build_from_flat(&flat, allow_extended)
        }
        ASTNode::Loop {
            condition,
            body,
            span,
            ..
        } => {
            // Container wrappers (ScopeBox/Program) inside loop bodies must be flattened before
            // recipe construction, otherwise `ExitAllowed` loop-body recipes cannot represent them.
            let stmt = ASTNode::Loop {
                condition: condition.clone(),
                body: flatten_block_containers(body),
                span: span.clone(),
            };
            let stmts = std::slice::from_ref(&stmt);
            if let Some(recipe) = try_build_no_exit_block_recipe(stmts, allow_extended) {
                return Some(ReturnPreludeContainerRecipe::NoExit(recipe));
            }
            try_build_exit_allowed_return_prelude_recipe(stmts, allow_extended)
                .map(ReturnPreludeContainerRecipe::ExitAllowed)
        }
        ASTNode::While {
            condition,
            body,
            span,
            ..
        } => {
            let stmt = ASTNode::While {
                condition: condition.clone(),
                body: flatten_block_containers(body),
                span: span.clone(),
            };
            let stmts = std::slice::from_ref(&stmt);
            if let Some(recipe) = try_build_no_exit_block_recipe(stmts, allow_extended) {
                return Some(ReturnPreludeContainerRecipe::NoExit(recipe));
            }
            try_build_exit_allowed_return_prelude_recipe(stmts, allow_extended)
                .map(ReturnPreludeContainerRecipe::ExitAllowed)
        }
        _ => None,
    }
}
