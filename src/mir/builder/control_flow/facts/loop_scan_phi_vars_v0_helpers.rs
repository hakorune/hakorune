use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::scan_common_predicates::{
    as_var_name, is_int_lit, is_loop_cond_var_lt_var as shared_is_loop_cond_var_lt_var,
    is_var_plus_expr, is_var_plus_one,
};
use crate::mir::builder::control_flow::facts::stmt_view::try_build_stmt_only_block_recipe;
use crate::mir::builder::control_flow::recipes::scan_loop_segments::NestedLoopRecipe;
use crate::mir::builder::control_flow::recipes::RecipeBody;

pub(in crate::mir::builder) fn release_enabled() -> bool {
    true
}

pub(in crate::mir::builder) fn is_loop_cond_var_lt_var(ast: &ASTNode) -> Option<(String, String)> {
    shared_is_loop_cond_var_lt_var(ast)
}

pub(in crate::mir::builder) fn is_local_decl(stmt: &ASTNode) -> bool {
    matches!(stmt, ASTNode::Local { .. })
}

pub(in crate::mir::builder) fn is_local_init_zero(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Local { initial_values, .. } => {
            if initial_values.len() != 1 {
                return false;
            }
            match initial_values[0].as_ref() {
                Some(init) => is_int_lit(init, 0),
                None => false,
            }
        }
        _ => false,
    }
}

pub(in crate::mir::builder) fn is_loop_with_break(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Loop { body, .. } => body_contains_break(body),
        _ => false,
    }
}

fn body_contains_break(body: &[ASTNode]) -> bool {
    for stmt in body {
        match stmt {
            ASTNode::Break { .. } => return true,
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                if body_contains_break(then_body) {
                    return true;
                }
                if let Some(else_body) = else_body {
                    if body_contains_break(else_body) {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    false
}

pub(in crate::mir::builder) fn is_if_stmt(stmt: &ASTNode) -> bool {
    matches!(stmt, ASTNode::If { .. })
}

pub(in crate::mir::builder) fn is_inc_stmt(stmt: &ASTNode, loop_var: &str) -> bool {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            as_var_name(target.as_ref()) == Some(loop_var)
                && is_var_plus_one(value.as_ref(), loop_var)
        }
        _ => false,
    }
}

pub(in crate::mir::builder) fn is_var_step_stmt_nonconst(stmt: &ASTNode, loop_var: &str) -> bool {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            as_var_name(target.as_ref()) == Some(loop_var)
                && is_var_plus_expr(value.as_ref(), loop_var)
                && !is_var_plus_one(value.as_ref(), loop_var)
        }
        _ => false,
    }
}

fn contains_exit_anywhere(stmts: &[ASTNode]) -> bool {
    for stmt in stmts {
        match stmt {
            ASTNode::Break { .. }
            | ASTNode::Continue { .. }
            | ASTNode::Return { .. }
            | ASTNode::Throw { .. } => return true,
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                if contains_exit_anywhere(then_body) {
                    return true;
                }
                if else_body
                    .as_ref()
                    .is_some_and(|b| contains_exit_anywhere(b))
                {
                    return true;
                }
            }
            ASTNode::Loop { body, .. }
            | ASTNode::While { body, .. }
            | ASTNode::ForRange { body, .. }
            | ASTNode::Program {
                statements: body, ..
            }
            | ASTNode::ScopeBox { body, .. } => {
                if contains_exit_anywhere(body) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

pub(in crate::mir::builder) fn is_loop_without_exit(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Loop { body, .. } => !contains_exit_anywhere(body),
        _ => false,
    }
}

pub(in crate::mir::builder) fn build_nested_loop_recipe(
    stmt: &ASTNode,
) -> Option<NestedLoopRecipe> {
    match stmt {
        ASTNode::Loop {
            condition,
            body: inner_body,
            ..
        } => Some(NestedLoopRecipe {
            cond_view: CondBlockView::from_expr(condition),
            loop_stmt: stmt.clone(),
            body: RecipeBody::new(inner_body.to_vec()),
            body_stmt_only: try_build_stmt_only_block_recipe(inner_body),
        }),
        _ => None,
    }
}

pub(in crate::mir::builder) fn contains_exit_outside_nested_loops(stmts: &[ASTNode]) -> bool {
    fn walk(stmts: &[ASTNode]) -> bool {
        for stmt in stmts {
            match stmt {
                ASTNode::Break { .. }
                | ASTNode::Continue { .. }
                | ASTNode::Return { .. }
                | ASTNode::Throw { .. } => return true,
                ASTNode::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    if walk(then_body) {
                        return true;
                    }
                    if else_body.as_ref().is_some_and(|b| walk(b)) {
                        return true;
                    }
                }
                ASTNode::Program { statements, .. } => {
                    if walk(statements) {
                        return true;
                    }
                }
                ASTNode::ScopeBox { body, .. } => {
                    if walk(body) {
                        return true;
                    }
                }
                ASTNode::Loop { .. } | ASTNode::While { .. } | ASTNode::ForRange { .. } => {}
                _ => {}
            }
        }
        false
    }

    walk(stmts)
}
