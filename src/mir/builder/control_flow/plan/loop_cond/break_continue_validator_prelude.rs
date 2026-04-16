//! Prelude validators for loop_cond_break_continue pattern detection.
//!
//! These validators check whether statements before an exit (break/continue/return)
//! are allowed in the exit-if pattern context.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::expr_bool::is_supported_bool_expr_with_canon;
use crate::mir::builder::control_flow::plan::facts::expr_generic_loop::is_pure_value_expr_for_generic_loop;
use std::collections::BTreeSet;

use super::break_continue_helpers::collect_vars_from_expr;
use super::break_continue_validator_cond::is_conditional_update_if;
use super::break_continue_validator_exit::is_exit_if_stmt;

/// Check if a prelude (statements before exit) is allowed for break/continue.
pub(in super::super) fn exit_prelude_is_allowed(prelude: &[ASTNode], allow_extended: bool) -> bool {
    for stmt in prelude {
        match stmt {
            ASTNode::Assignment { target, .. } => {
                if !matches!(target.as_ref(), ASTNode::Variable { .. }) {
                    return false;
                }
            }
            ASTNode::Local { .. } => {}
            ASTNode::MethodCall { .. } | ASTNode::FunctionCall { .. } | ASTNode::Call { .. } => {}
            ASTNode::Print { .. } => {
                if !allow_extended {
                    return false;
                }
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                if !allow_extended {
                    return false;
                }
                if !is_supported_bool_expr_with_canon(condition, allow_extended) {
                    return false;
                }
                // Facts->Lower contract: allow nested exit-if in the prelude.
                //
                // Lowering handles this via `exit_branch::lower_return_prelude_stmt` which can
                // recursively lower both general if and exit-if forms (no AST rewrite).
                if is_exit_if_stmt(condition, then_body, else_body.as_ref(), allow_extended) {
                    continue;
                }
                // Also allow nested conditional-update if (assignment/local + optional exit).
                // This is still "exit-ish" control flow and is lowered as an exit-if in the
                // prelude lowering path (contract: analysis-only observation, no AST rewrite).
                if is_conditional_update_if(
                    condition,
                    then_body,
                    else_body.as_ref(),
                    allow_extended,
                ) {
                    continue;
                }
                if !branch_effects_only(then_body, allow_extended) {
                    return false;
                }
                if let Some(else_body) = else_body {
                    if !branch_effects_only(else_body, allow_extended) {
                        return false;
                    }
                }
            }
            _ => return false,
        }
    }
    true
}

/// Check if a branch contains only effect statements (no control flow).
pub(in super::super) fn branch_effects_only(body: &[ASTNode], allow_extended: bool) -> bool {
    body.iter().all(|stmt| match stmt {
        ASTNode::Assignment { target, .. } => matches!(target.as_ref(), ASTNode::Variable { .. }),
        ASTNode::Local { .. } => true,
        ASTNode::MethodCall { .. } | ASTNode::FunctionCall { .. } | ASTNode::Call { .. } => true,
        ASTNode::Print { .. } => allow_extended,
        _ => false,
    })
}

/// Check if a prelude is allowed specifically for break exits.
pub(in super::super) fn exit_prelude_is_allowed_for_break(
    prelude: &[ASTNode],
    allow_extended: bool,
) -> bool {
    for stmt in prelude {
        match stmt {
            ASTNode::Assignment { target, .. } => {
                if !allow_extended {
                    return false;
                }
                if !matches!(target.as_ref(), ASTNode::Variable { .. }) {
                    return false;
                }
            }
            ASTNode::Local { .. } => {}
            ASTNode::MethodCall { .. } | ASTNode::FunctionCall { .. } | ASTNode::Call { .. } => {}
            ASTNode::Print { .. } => {
                if !allow_extended {
                    return false;
                }
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                if !allow_extended {
                    return false;
                }
                if !is_supported_bool_expr_with_canon(condition, allow_extended) {
                    return false;
                }
                if is_exit_if_stmt(condition, then_body, else_body.as_ref(), allow_extended) {
                    continue;
                }
                if is_conditional_update_if(
                    condition,
                    then_body,
                    else_body.as_ref(),
                    allow_extended,
                ) {
                    continue;
                }
                if !branch_effects_only_for_break(then_body, allow_extended) {
                    return false;
                }
                if let Some(else_body) = else_body {
                    if !branch_effects_only_for_break(else_body, allow_extended) {
                        return false;
                    }
                }
            }
            _ => return false,
        }
    }
    true
}

/// Check if a branch contains only effect statements suitable for break prelude.
pub(in super::super) fn branch_effects_only_for_break(
    body: &[ASTNode],
    allow_extended: bool,
) -> bool {
    body.iter().all(|stmt| match stmt {
        ASTNode::Local { .. } => true,
        ASTNode::MethodCall { .. } | ASTNode::FunctionCall { .. } | ASTNode::Call { .. } => true,
        ASTNode::Print { .. } => allow_extended,
        _ => false,
    })
}

/// Check if a prelude is allowed before a return statement.
pub(in super::super) fn return_prelude_is_allowed(
    prelude: &[ASTNode],
    return_stmt: &ASTNode,
    allow_extended: bool,
) -> bool {
    let ASTNode::Return { value, .. } = return_stmt else {
        return false;
    };
    let Some(value) = value.as_ref() else {
        return false;
    };

    // Minimal BoxCount extension (Phase 29bq):
    // Allow `local t = <pure>; return t` as an exit-if prelude shape.
    //
    // Rationale: this is not an AST rewrite. Lowering can evaluate the local-init effect(s)
    // and then return the bound variable, and the temp is scoped to the exiting branch.
    if return_prelude_is_allowed_local_return_var(prelude, value, allow_extended) {
        return true;
    }

    let mut assigned = BTreeSet::new();
    for stmt in prelude {
        match stmt {
            ASTNode::Assignment { target, .. } => {
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return false;
                };
                assigned.insert(name.clone());
            }
            ASTNode::Local { variables, .. } => {
                for name in variables {
                    assigned.insert(name.clone());
                }
            }
            ASTNode::Print { .. } => {
                if !allow_extended {
                    return false;
                }
            }
            ASTNode::MethodCall { .. } | ASTNode::FunctionCall { .. } | ASTNode::Call { .. } => {}
            _ => return false,
        }
    }
    let mut used = BTreeSet::new();
    if !collect_vars_from_expr(value, &mut used) {
        return false;
    }
    assigned.is_disjoint(&used)
}

fn return_prelude_is_allowed_local_return_var(
    prelude: &[ASTNode],
    value: &ASTNode,
    allow_extended: bool,
) -> bool {
    let ASTNode::Variable {
        name: return_var, ..
    } = value
    else {
        return false;
    };

    if prelude.len() != 1 {
        return false;
    }

    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = &prelude[0]
    else {
        return false;
    };

    if variables.len() != 1 || initial_values.len() != 1 {
        return false;
    }

    if variables[0] != *return_var {
        return false;
    }

    let Some(init) = initial_values[0].as_ref() else {
        return false;
    };

    if !is_pure_local_init_value_expr(init, allow_extended) {
        return false;
    }

    // Ensure we don't reference the new local on the RHS (conservative, analysis-only).
    let mut vars = BTreeSet::new();
    if !collect_vars_from_expr_allow_blockexpr(init, &mut vars) {
        return false;
    }
    !vars.contains(return_var)
}

fn collect_vars_from_expr_allow_blockexpr(ast: &ASTNode, vars: &mut BTreeSet<String>) -> bool {
    match ast {
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => {
            for stmt in prelude_stmts {
                let ASTNode::Local { initial_values, .. } = stmt else {
                    return false;
                };
                for init in initial_values.iter().flatten() {
                    if !collect_vars_from_expr_allow_blockexpr(init, vars) {
                        return false;
                    }
                }
            }
            collect_vars_from_expr_allow_blockexpr(tail_expr.as_ref(), vars)
        }
        _ => collect_vars_from_expr(ast, vars),
    }
}

fn is_pure_local_init_value_expr(init: &ASTNode, allow_extended: bool) -> bool {
    if is_pure_value_expr_for_generic_loop(init) {
        return true;
    }

    let ASTNode::BlockExpr {
        prelude_stmts,
        tail_expr,
        ..
    } = init
    else {
        return false;
    };

    // Contract (v1): BlockExpr used as a value must not contain exits in its prelude.
    if prelude_stmts.iter().any(ASTNode::contains_non_local_exit) {
        return false;
    }

    // Keep this conservative for "pure local-init" use-sites:
    // - prelude is Local-only (no calls/prints)
    // - each init value is recursively pure (or a nested BlockExpr that satisfies this contract)
    for stmt in prelude_stmts {
        let ASTNode::Local {
            variables,
            initial_values,
            ..
        } = stmt
        else {
            return false;
        };

        if variables.len() != initial_values.len() {
            return false;
        }

        for (name, init) in variables.iter().zip(initial_values.iter()) {
            let Some(init) = init.as_ref() else {
                return false;
            };
            if !is_pure_local_init_value_expr(init, allow_extended) {
                return false;
            }

            let mut used = BTreeSet::new();
            if !collect_vars_from_expr_allow_blockexpr(init, &mut used) {
                return false;
            }
            if used.contains(name) {
                return false;
            }
        }
    }

    // Tail must be value-lowerable (handled by PlanNormalizer::lower_value_ast).
    is_supported_value_ast_for_then_only_return(tail_expr.as_ref(), allow_extended)
}

/// Check if a prelude is allowed specifically for then-only-return patterns.
pub(in super::super) fn then_only_return_prelude_is_allowed_local_then_return_value(
    prelude: &[ASTNode],
    return_value: &ASTNode,
    allow_extended: bool,
) -> bool {
    if prelude.len() != 1 {
        return false;
    }

    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = &prelude[0]
    else {
        return false;
    };

    if variables.len() != 1 || initial_values.len() != 1 {
        return false;
    }

    let local_var = &variables[0];
    let Some(init) = initial_values[0].as_ref() else {
        return false;
    };

    if !is_pure_local_init_value_expr(init, allow_extended) {
        return false;
    }

    // Ensure we don't reference the new local on the RHS (conservative, analysis-only).
    let mut vars = BTreeSet::new();
    if !collect_vars_from_expr_allow_blockexpr(init, &mut vars) {
        return false;
    }
    if vars.contains(local_var) {
        return false;
    }

    // Accept only value ASTs expected to lower through PlanNormalizer::lower_value_ast,
    // plus BlockExpr (Phase B2-7) which is explicitly supported there.
    is_supported_value_ast_for_then_only_return(return_value, allow_extended)
}

fn is_supported_value_ast_for_then_only_return(ast: &ASTNode, allow_extended: bool) -> bool {
    use crate::ast::{BinaryOperator, UnaryOperator};

    match ast {
        ASTNode::Variable { .. } => true,
        ASTNode::Literal { .. } => true,
        ASTNode::UnaryOp {
            operator: UnaryOperator::Minus | UnaryOperator::BitNot,
            operand,
            ..
        } => is_supported_value_ast_for_then_only_return(operand, allow_extended),
        ASTNode::BinaryOp {
            operator:
                BinaryOperator::Add
                | BinaryOperator::Subtract
                | BinaryOperator::Multiply
                | BinaryOperator::Divide
                | BinaryOperator::Modulo,
            left,
            right,
            ..
        } => {
            is_supported_value_ast_for_then_only_return(left, allow_extended)
                && is_supported_value_ast_for_then_only_return(right, allow_extended)
        }
        ASTNode::MethodCall {
            object, arguments, ..
        } => {
            if !is_supported_value_ast_for_then_only_return(object, allow_extended) {
                return false;
            }
            arguments
                .iter()
                .all(|arg| is_supported_value_ast_for_then_only_return(arg, allow_extended))
        }
        ASTNode::FunctionCall { arguments, .. } => {
            if !allow_extended {
                return false;
            }
            arguments
                .iter()
                .all(|arg| is_supported_value_ast_for_then_only_return(arg, allow_extended))
        }
        ASTNode::Call {
            callee, arguments, ..
        } => {
            if !allow_extended {
                return false;
            }
            if !is_supported_value_ast_for_then_only_return(callee, allow_extended) {
                return false;
            }
            arguments
                .iter()
                .all(|arg| is_supported_value_ast_for_then_only_return(arg, allow_extended))
        }
        ASTNode::New { arguments, .. } => {
            if !allow_extended {
                return false;
            }
            arguments
                .iter()
                .all(|arg| is_supported_value_ast_for_then_only_return(arg, allow_extended))
        }
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => {
            if prelude_stmts.iter().any(ASTNode::contains_non_local_exit) {
                return false;
            }
            is_supported_value_ast_for_then_only_return(tail_expr.as_ref(), allow_extended)
        }
        _ => false,
    }
}
