//! Shared if-shape predicates for loop_cond_break_continue facts extraction.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::expr_bool::is_supported_bool_expr_with_canon;
use crate::mir::builder::control_flow::facts::no_exit_block::{
    try_build_no_exit_block_recipe, NoExitBlockRecipe,
};
use crate::mir::builder::control_flow::plan::extractors::common_helpers::flatten_stmt_list;
use crate::mir::builder::control_flow::plan::loop_cond_shared::branch_tail_is_continue_flattened;

use super::break_continue_helpers::{branch_has_exit_or_loop, is_nested_loop_allowed};
use super::break_continue_validator_prelude::exit_prelude_is_allowed;

/// Check for continue-if-with-else pattern and return which branch has the continue.
///
/// Returns Some(true) if continue is in then branch, Some(false) if in else branch,
/// None if not a continue-if-with-else pattern.
pub(super) fn continue_if_with_else_info(
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    allow_extended: bool,
) -> Option<bool> {
    if !allow_extended || !is_supported_bool_expr_with_canon(condition, allow_extended) {
        return None;
    }
    let Some(else_body) = else_body else {
        return None;
    };

    let then_has_continue = branch_tail_is_continue_flattened(then_body);
    let else_has_continue = branch_tail_is_continue_flattened(else_body);
    if then_has_continue == else_has_continue {
        return None;
    }

    let continue_in_then = then_has_continue;
    let (continue_body, fallthrough_body) = if continue_in_then {
        (then_body, else_body.as_slice())
    } else {
        (else_body.as_slice(), then_body)
    };

    let prelude = if continue_body.len() > 1 {
        &continue_body[..continue_body.len() - 1]
    } else {
        &[]
    };
    if !exit_prelude_is_allowed(prelude, allow_extended) {
        return None;
    }
    if branch_has_exit_or_loop(fallthrough_body) {
        return None;
    }
    Some(continue_in_then)
}

/// Build recipes for continue-if-with-else pattern.
///
/// Returns (continue_prelude, fallthrough_body) recipes.
pub(super) fn build_continue_if_with_else_recipes(
    then_body: &[ASTNode],
    else_body: &[ASTNode],
    continue_in_then: bool,
    allow_nested: bool,
    allow_extended: bool,
    max_nested_loops: usize,
    debug: bool,
) -> Option<(Option<NoExitBlockRecipe>, Option<NoExitBlockRecipe>)> {
    let (continue_body, fallthrough_body) = if continue_in_then {
        (then_body, else_body)
    } else {
        (else_body, then_body)
    };

    let mut flat_continue: Vec<ASTNode> = flatten_stmt_list(continue_body)
        .into_iter()
        .map(|stmt| stmt.clone())
        .collect();
    if flat_continue.is_empty() {
        return None;
    }
    // Drop the tail continue (recipe is prelude-only).
    flat_continue.pop();

    let flat_fallthrough: Vec<ASTNode> = flatten_stmt_list(fallthrough_body)
        .into_iter()
        .map(|stmt| stmt.clone())
        .collect();

    // Recipe-first: represent both branches as NoExit blocks.
    //
    // Notes:
    // - The continue branch prelude can be empty (e.g., `if cond { continue } else { ... }`).
    // - The fallthrough branch can also be empty (e.g., `if cond { continue } else { }`).
    // - We intentionally reject shapes that would build an ExitOnly/ExitIf tree here; those were
    //   never valid in the continue prelude lowering path (no exits allowed before the tail continue).
    let continue_prelude = if flat_continue.is_empty() {
        None
    } else {
        Some(try_build_no_exit_block_recipe(
            &flat_continue,
            allow_extended,
        )?)
    };
    let fallthrough_body = if flat_fallthrough.is_empty() {
        None
    } else {
        Some(try_build_no_exit_block_recipe(
            &flat_fallthrough,
            allow_extended,
        )?)
    };

    let _ = (allow_nested, max_nested_loops, debug);
    Some((continue_prelude, fallthrough_body))
}

pub(super) fn is_general_if_stmt(
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    allow_extended: bool,
    allow_nested: bool,
    debug: bool,
) -> bool {
    if !allow_extended || !is_supported_bool_expr_with_canon(condition, allow_extended) {
        return false;
    }
    if branch_has_exit_or_forbidden_loop(then_body, allow_extended, allow_nested, debug) {
        return false;
    }
    if let Some(else_body) = else_body {
        if branch_has_exit_or_forbidden_loop(else_body, allow_extended, allow_nested, debug) {
            return false;
        }
    }
    true
}

fn branch_has_exit_or_forbidden_loop(
    body: &[ASTNode],
    allow_extended: bool,
    allow_nested: bool,
    debug: bool,
) -> bool {
    for stmt in body {
        match stmt {
            ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. } => {
                return true
            }
            ASTNode::Loop {
                condition, body, ..
            } => {
                if !allow_nested || !is_nested_loop_allowed(condition, body, allow_extended, debug)
                {
                    return true;
                }
            }
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                if branch_has_exit_or_forbidden_loop(then_body, allow_extended, allow_nested, debug)
                {
                    return true;
                }
                if let Some(else_body) = else_body {
                    if branch_has_exit_or_forbidden_loop(
                        else_body,
                        allow_extended,
                        allow_nested,
                        debug,
                    ) {
                        return true;
                    }
                }
            }
            ASTNode::Program { statements, .. } => {
                if branch_has_exit_or_forbidden_loop(
                    statements,
                    allow_extended,
                    allow_nested,
                    debug,
                ) {
                    return true;
                }
            }
            ASTNode::ScopeBox { body, .. } => {
                if branch_has_exit_or_forbidden_loop(body, allow_extended, allow_nested, debug) {
                    return true;
                }
            }
            ASTNode::While { .. } | ASTNode::ForRange { .. } => return true,
            _ => {}
        }
    }
    false
}
