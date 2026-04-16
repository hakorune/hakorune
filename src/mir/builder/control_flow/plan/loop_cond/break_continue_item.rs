//! Item building for loop_cond_break_continue recipe construction.
//!
//! This module contains the main item builder that converts AST statements
//! into LoopCondBreakContinueItem recipe elements.

use super::break_continue_recipe::{
    LoopCondBreakContinueItem, LoopCondBreakContinueRecipe, NestedLoopDepth1Recipe,
};
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::expr_bool::is_supported_bool_expr_with_canon;
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::facts::stmt_view::try_build_stmt_only_block_recipe;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::flatten_stmt_list;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::recipes::loop_cond_shared::LoopCondRecipe;
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::recipes::RecipeBody;

use super::break_continue_classify::{
    build_continue_if_with_else_recipes, continue_if_with_else_info, is_general_if_stmt,
};
use super::break_continue_helpers::is_nested_loop_allowed;
use super::break_continue_tree::build_exit_if_tree_recipe;
use super::break_continue_validator_cond::{
    build_conditional_update_branch_recipe, is_conditional_update_if,
};
use super::break_continue_validator_else::{
    build_else_guard_break_recipes, is_else_guard_break_if_shape, is_else_only_break_if_shape,
    is_else_only_return_if_shape, is_then_only_break_if_shape, is_then_only_return_if_shape,
};
use super::break_continue_validator_exit::{
    is_exit_if_stmt, try_build_else_nested_exit_if_return_exit_allowed_recipe,
};

#[derive(Debug, Clone, Copy)]
enum IfStmtKind {
    ExitIf,
    ContinueIf { continue_in_then: bool },
    ConditionalUpdate,
    GeneralIf,
}

/// Build a recipe for the loop body.
pub(super) fn build_loop_cond_break_continue_recipe(
    body: &[ASTNode],
    allow_nested: bool,
    allow_extended: bool,
    max_nested_loops: usize,
    debug: bool,
    exit_if_seen: &mut usize,
    continue_if_seen: &mut usize,
    conditional_update_seen: &mut usize,
    nested_seen: &mut usize,
    allow_break_tail: bool,
) -> Option<LoopCondBreakContinueRecipe> {
    build_loop_cond_break_continue_recipe_inner(
        body,
        allow_nested,
        allow_extended,
        max_nested_loops,
        debug,
        exit_if_seen,
        continue_if_seen,
        conditional_update_seen,
        nested_seen,
        allow_break_tail,
        false,
    )
}

pub(in crate::mir::builder) fn build_loop_cond_break_continue_recipe_inner(
    body: &[ASTNode],
    allow_nested: bool,
    allow_extended: bool,
    max_nested_loops: usize,
    debug: bool,
    exit_if_seen: &mut usize,
    continue_if_seen: &mut usize,
    conditional_update_seen: &mut usize,
    nested_seen: &mut usize,
    allow_break_tail: bool,
    log_program_rejects: bool,
) -> Option<LoopCondBreakContinueRecipe> {
    let recipe_body = RecipeBody::new(body.to_vec());
    let mut items = Vec::with_capacity(recipe_body.len());
    for (idx, stmt) in recipe_body.body.iter().enumerate() {
        let is_last = allow_break_tail && idx + 1 == recipe_body.len();
        let stmt_ref = StmtRef::new(idx);
        let Some(item) = build_loop_cond_break_continue_item(
            stmt,
            stmt_ref,
            is_last,
            exit_if_seen,
            continue_if_seen,
            conditional_update_seen,
            nested_seen,
            allow_extended,
            allow_nested,
            max_nested_loops,
            debug,
        ) else {
            if debug {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[plan/reject_detail] box=loop_cond_break_continue reason=unsupported_stmt idx={} kind={}",
                    idx,
                    stmt.node_type()
                ));
            } else if log_program_rejects {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[loop_cond_break_continue] reject: program_stmt idx={} kind={}",
                    idx,
                    stmt.node_type()
                ));
            }
            return None;
        };
        items.push(item);
    }
    Some(LoopCondRecipe::new(recipe_body.body, items))
}

fn build_loop_cond_break_continue_item(
    stmt: &ASTNode,
    stmt_ref: StmtRef,
    is_last: bool,
    exit_if_seen: &mut usize,
    continue_if_seen: &mut usize,
    conditional_update_seen: &mut usize,
    nested_seen: &mut usize,
    allow_extended: bool,
    allow_nested: bool,
    max_nested_loops: usize,
    debug: bool,
) -> Option<LoopCondBreakContinueItem> {
    match stmt {
        ASTNode::Assignment { .. }
        | ASTNode::Local { .. }
        | ASTNode::MethodCall { .. }
        | ASTNode::FunctionCall { .. }
        | ASTNode::Call { .. } => Some(LoopCondBreakContinueItem::Stmt(stmt_ref)),
        ASTNode::Print { .. } => {
            if allow_extended {
                Some(LoopCondBreakContinueItem::Stmt(stmt_ref))
            } else {
                None
            }
        }
        ASTNode::Program { statements, .. } => {
            build_program_or_scope_item(statements, stmt_ref, allow_extended)
        }
        ASTNode::ScopeBox { body, .. } => {
            build_program_or_scope_item(body, stmt_ref, allow_extended)
        }
        ASTNode::Loop {
            condition, body, ..
        } => {
            if !allow_nested || *nested_seen >= max_nested_loops {
                return None;
            }
            if !is_nested_loop_allowed(condition, body, allow_extended, debug) {
                return None;
            }
            *nested_seen += 1;
            let body_recipe = if !allow_extended
                && body
                    .iter()
                    .any(|stmt| matches!(stmt, ASTNode::Print { .. }))
            {
                None
            } else {
                try_build_stmt_only_block_recipe(body)
            };
            Some(LoopCondBreakContinueItem::NestedLoopDepth1 {
                loop_stmt: stmt_ref,
                nested: NestedLoopDepth1Recipe {
                    cond_view: CondBlockView::from_expr(condition),
                    body: body_recipe,
                },
            })
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => build_if_item(
            stmt,
            condition,
            then_body,
            else_body.as_ref(),
            stmt_ref,
            exit_if_seen,
            continue_if_seen,
            conditional_update_seen,
            allow_extended,
            allow_nested,
            max_nested_loops,
            debug,
        ),
        ASTNode::Break { .. } => {
            if allow_extended && is_last {
                let exit_allowed_block =
                    try_build_exit_allowed_block_recipe(std::slice::from_ref(stmt), allow_extended);
                Some(LoopCondBreakContinueItem::tail_break_with_optional_block(
                    exit_allowed_block,
                ))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn build_program_or_scope_item(
    statements: &[ASTNode],
    stmt_ref: StmtRef,
    allow_extended: bool,
) -> Option<LoopCondBreakContinueItem> {
    let flat: Vec<ASTNode> = flatten_stmt_list(statements)
        .into_iter()
        .map(|stmt| stmt.clone())
        .collect();

    if flat.is_empty() {
        return Some(LoopCondBreakContinueItem::ProgramBlock {
            stmt: stmt_ref,
            stmt_only: None,
        });
    }
    if !allow_extended
        && flat
            .iter()
            .any(|stmt| matches!(stmt, ASTNode::Print { .. }))
    {
        return None;
    }
    let stmt_only = try_build_stmt_only_block_recipe(&flat);
    if stmt_only.is_none()
        && flat.iter().any(|stmt| {
            matches!(
                stmt,
                ASTNode::Return { .. }
                    | ASTNode::Break { .. }
                    | ASTNode::Continue { .. }
                    | ASTNode::Throw { .. }
            )
        })
    {
        return None;
    }
    Some(LoopCondBreakContinueItem::ProgramBlock {
        stmt: stmt_ref,
        stmt_only,
    })
}

#[allow(clippy::too_many_arguments)]
fn build_if_item(
    stmt: &ASTNode,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    stmt_ref: StmtRef,
    exit_if_seen: &mut usize,
    continue_if_seen: &mut usize,
    conditional_update_seen: &mut usize,
    allow_extended: bool,
    allow_nested: bool,
    max_nested_loops: usize,
    debug: bool,
) -> Option<LoopCondBreakContinueItem> {
    // Phase 29bq BoxCount: try ExitIfTree first (recipe-based)
    if let Some(tree_item) = build_exit_if_tree_recipe(
        condition,
        then_body,
        else_body,
        stmt_ref.index(),
        allow_extended,
    ) {
        *exit_if_seen += 1;
        return Some(tree_item);
    }
    // Phase 29bq BoxCount: `if { return } else { if { return } }` must be lowered via
    // exit-allowed recipe (Parts), not via the exit-if map.
    if let Some(exit_allowed_block) =
        try_build_else_nested_exit_if_return_exit_allowed_recipe(stmt, allow_extended)
    {
        *exit_if_seen += 1;
        return Some(LoopCondBreakContinueItem::exit_if_with_optional_block(
            stmt_ref,
            Some(exit_allowed_block),
        ));
    }
    // else-only return pattern check
    if is_else_only_return_if_shape(condition, then_body, else_body, allow_extended) {
        *exit_if_seen += 1;
        let else_return_stmt = if else_body.as_ref().is_some_and(|body| body.len() == 2) {
            StmtRef::new(1)
        } else {
            StmtRef::new(0)
        };
        let then_no_exit = try_build_no_exit_block_recipe(then_body, allow_extended);
        return Some(LoopCondBreakContinueItem::ElseOnlyReturnIf {
            if_stmt: stmt_ref,
            cond_view: CondBlockView::from_expr(condition),
            then_no_exit,
            else_return_stmt,
        });
    }
    // then-only return pattern check (if { return } else { non-exit })
    if is_then_only_return_if_shape(condition, then_body, else_body, allow_extended) {
        *exit_if_seen += 1;
        let then_return_stmt = if then_body.len() == 2 {
            StmtRef::new(1)
        } else {
            StmtRef::new(0)
        };
        let Some(else_body) = else_body else {
            return None;
        };
        let else_no_exit = try_build_no_exit_block_recipe(else_body, allow_extended);
        return Some(LoopCondBreakContinueItem::ThenOnlyReturnIf {
            if_stmt: stmt_ref,
            cond_view: CondBlockView::from_expr(condition),
            then_return_stmt,
            else_no_exit,
        });
    }
    // Phase 29bq: else-only break pattern (if { non-exit } else { break })
    if is_else_only_break_if_shape(condition, then_body, else_body, allow_extended) {
        *exit_if_seen += 1;
        let else_break_stmt = StmtRef::new(0);
        let then_no_exit = try_build_no_exit_block_recipe(then_body, allow_extended);
        return Some(LoopCondBreakContinueItem::ElseOnlyBreakIf {
            if_stmt: stmt_ref,
            cond_view: CondBlockView::from_expr(condition),
            then_no_exit,
            else_break_stmt,
        });
    }
    // Phase 29bq: then-only break pattern (if { break } else { non-exit })
    if is_then_only_break_if_shape(condition, then_body, else_body, allow_extended) {
        let Some(else_body) = else_body else {
            return None;
        };
        *exit_if_seen += 1;
        let then_break_stmt = StmtRef::new(0);
        let else_no_exit = try_build_no_exit_block_recipe(else_body, allow_extended);
        return Some(LoopCondBreakContinueItem::ThenOnlyBreakIf {
            if_stmt: stmt_ref,
            cond_view: CondBlockView::from_expr(condition),
            then_break_stmt,
            else_no_exit,
        });
    }
    // else-guard-break pattern
    if is_else_guard_break_if_shape(condition, then_body, else_body, allow_extended) {
        let else_body = else_body.unwrap();
        let else_exit_allowed = try_build_exit_allowed_block_recipe(else_body, allow_extended);
        if let Some((then_recipe, else_recipe)) = build_else_guard_break_recipes(
            then_body,
            else_body,
            allow_nested,
            allow_extended,
            max_nested_loops,
            debug,
        ) {
            let then_no_exit = try_build_no_exit_block_recipe(then_body, allow_extended);
            *exit_if_seen += 1; // else contains exit-ifs
            return Some(
                LoopCondBreakContinueItem::else_guard_break_if_with_optional_else_exit_allowed(
                    stmt_ref,
                    then_no_exit,
                    then_recipe,
                    else_recipe,
                    else_exit_allowed,
                ),
            );
        }
    }
    let if_kind = if is_exit_if_stmt(condition, then_body, else_body, allow_extended) {
        Some(IfStmtKind::ExitIf)
    } else if let Some(continue_in_then) =
        continue_if_with_else_info(condition, then_body, else_body, allow_extended)
    {
        Some(IfStmtKind::ContinueIf { continue_in_then })
    } else if is_conditional_update_if(condition, then_body, else_body, allow_extended) {
        Some(IfStmtKind::ConditionalUpdate)
    } else if is_general_if_stmt(
        condition,
        then_body,
        else_body,
        allow_extended,
        allow_nested,
        debug,
    ) {
        Some(IfStmtKind::GeneralIf)
    } else {
        None
    };

    match if_kind {
        Some(IfStmtKind::ExitIf) => {
            let tail_is_exit = |body: &[ASTNode]| {
                matches!(
                    body.last(),
                    Some(ASTNode::Return { .. } | ASTNode::Break { .. } | ASTNode::Continue { .. })
                )
            };
            let is_tail_exit_shape =
                tail_is_exit(then_body) && else_body.map_or(true, |b| tail_is_exit(b));
            *exit_if_seen += 1;
            let exit_allowed_block =
                try_build_exit_allowed_block_recipe(std::slice::from_ref(stmt), allow_extended);
            if !is_tail_exit_shape
                && exit_allowed_block
                    .as_ref()
                    .is_some_and(|block| !exit_allowed_block_has_non_stmt_items(block))
            {
                *conditional_update_seen += 1;
                return Some(LoopCondBreakContinueItem::ProgramBlock {
                    stmt: stmt_ref,
                    stmt_only: None,
                });
            }
            Some(LoopCondBreakContinueItem::exit_if_with_optional_block(
                stmt_ref,
                exit_allowed_block,
            ))
        }
        Some(IfStmtKind::ContinueIf { continue_in_then }) => {
            let Some(else_body) = else_body else {
                return None;
            };
            if let Some((continue_prelude, fallthrough_body)) = build_continue_if_with_else_recipes(
                then_body,
                else_body,
                continue_in_then,
                allow_nested,
                allow_extended,
                max_nested_loops,
                debug,
            ) {
                *continue_if_seen += 1;
                Some(LoopCondBreakContinueItem::ContinueIfWithElse {
                    if_stmt: stmt_ref,
                    continue_in_then,
                    continue_prelude,
                    fallthrough_body,
                })
            } else if allow_extended {
                *conditional_update_seen += 1;
                Some(LoopCondBreakContinueItem::ProgramBlock {
                    stmt: stmt_ref,
                    stmt_only: None,
                })
            } else {
                None
            }
        }
        Some(IfStmtKind::ConditionalUpdate) => {
            *conditional_update_seen += 1;
            let (then_body_recipe, then_exit) =
                build_conditional_update_branch_recipe(then_body, allow_extended)?;
            let (else_body_recipe, else_exit) = match else_body {
                Some(else_body) => {
                    build_conditional_update_branch_recipe(else_body, allow_extended)?
                }
                None => (None, None),
            };
            Some(LoopCondBreakContinueItem::ConditionalUpdateIf {
                if_stmt: stmt_ref,
                cond_view: CondBlockView::from_expr(condition),
                then_body: then_body_recipe,
                then_exit,
                else_body: else_body_recipe,
                else_exit,
            })
        }
        Some(IfStmtKind::GeneralIf) => {
            *conditional_update_seen += 1;
            if let Some(recipe) =
                try_build_no_exit_block_recipe(std::slice::from_ref(stmt), allow_extended)
            {
                return Some(LoopCondBreakContinueItem::GeneralIf(recipe));
            }
            if allow_extended
                && is_supported_bool_expr_with_canon(condition, allow_extended)
                && !stmt.contains_non_local_exit_outside_loops()
            {
                // Use ProgramBlock(None) so lowering goes through `lower_loop_cond_stmt`,
                // which supports generic if-join fallback for branch-local nested loops.
                return Some(LoopCondBreakContinueItem::ProgramBlock {
                    stmt: stmt_ref,
                    stmt_only: None,
                });
            }
            None
        }
        None => {
            if !allow_extended {
                if let Some(recipe) =
                    try_build_no_exit_block_recipe(std::slice::from_ref(stmt), allow_extended)
                {
                    *conditional_update_seen += 1;
                    return Some(LoopCondBreakContinueItem::GeneralIf(recipe));
                }
                if is_supported_bool_expr_with_canon(condition, allow_extended)
                    && !stmt.contains_non_local_exit_outside_loops()
                {
                    *conditional_update_seen += 1;
                    return Some(LoopCondBreakContinueItem::ProgramBlock {
                        stmt: stmt_ref,
                        stmt_only: None,
                    });
                }
            }
            if allow_extended && if_has_any_exit_signals(then_body, else_body) {
                if let Some(exit_allowed_block) =
                    try_build_exit_allowed_block_recipe(std::slice::from_ref(stmt), allow_extended)
                {
                    if !exit_allowed_block_has_non_stmt_items(&exit_allowed_block) {
                        *conditional_update_seen += 1;
                        return Some(LoopCondBreakContinueItem::ProgramBlock {
                            stmt: stmt_ref,
                            stmt_only: None,
                        });
                    }
                    *exit_if_seen += 1;
                    return Some(LoopCondBreakContinueItem::exit_if_with_optional_block(
                        stmt_ref,
                        Some(exit_allowed_block),
                    ));
                }
                *conditional_update_seen += 1;
                return Some(LoopCondBreakContinueItem::ProgramBlock {
                    stmt: stmt_ref,
                    stmt_only: None,
                });
            }
            None
        }
    }
}

fn exit_allowed_block_has_non_stmt_items(
    block: &crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe,
) -> bool {
    block.block.items.iter().any(|item| {
        !matches!(
            item,
            crate::mir::builder::control_flow::plan::recipe_tree::RecipeItem::Stmt(_)
        )
    })
}

fn if_has_any_exit_signals(then_body: &[ASTNode], else_body: Option<&Vec<ASTNode>>) -> bool {
    let then_counts = super::loop_cond_unified_helpers::count_control_flow_with_returns(then_body);
    if then_counts.break_count > 0 || then_counts.continue_count > 0 || then_counts.return_count > 0
    {
        return true;
    }
    let Some(else_body) = else_body else {
        return false;
    };
    let else_counts = super::loop_cond_unified_helpers::count_control_flow_with_returns(else_body);
    else_counts.break_count > 0 || else_counts.continue_count > 0 || else_counts.return_count > 0
}
