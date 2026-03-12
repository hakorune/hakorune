use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::generic_loop::matches_loop_increment;

/// Returns true when the loop body writes to variables other than the loop var.
pub(in crate::mir::builder) fn body_writes_non_loop_vars(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    for stmt in body {
        if matches_loop_increment(stmt, loop_var, loop_increment) {
            continue;
        }
        match stmt {
            ASTNode::Assignment { target, .. } => match target.as_ref() {
                ASTNode::Variable { name, .. } if name == loop_var => {}
                _ => return true,
            },
            ASTNode::Local { variables, .. } => {
                if variables.iter().any(|name| name != loop_var) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

/// Collect loop var candidates from body by finding variables used in increment expressions.
pub(in crate::mir::builder) fn collect_loop_var_candidates_from_body(
    body: &[ASTNode],
) -> Vec<String> {
    let mut out = Vec::new();
    fn walk(stmt: &ASTNode, out: &mut Vec<String>) {
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                if let ASTNode::Variable { name, .. } = target.as_ref() {
                    if let ASTNode::BinaryOp {
                        operator,
                        left,
                        right,
                        ..
                    } = value.as_ref()
                    {
                        if matches!(
                            operator,
                            crate::ast::BinaryOperator::Add | crate::ast::BinaryOperator::Subtract
                        ) && (matches!(left.as_ref(), ASTNode::Variable { name: ln, .. } if ln == name)
                            || matches!(right.as_ref(), ASTNode::Variable { name: rn, .. } if rn == name))
                        {
                            if !out.iter().any(|v| v == name) {
                                out.push(name.clone());
                            }
                        }
                    }
                }
            }
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                for s in then_body {
                    walk(s, out);
                }
                if let Some(eb) = else_body {
                    for s in eb {
                        walk(s, out);
                    }
                }
            }
            ASTNode::Loop { body, .. } | ASTNode::While { body, .. } => {
                for s in body {
                    walk(s, out);
                }
            }
            ASTNode::Program { statements, .. } => {
                for s in statements {
                    walk(s, out);
                }
            }
            _ => {}
        }
    }
    for stmt in body {
        walk(stmt, &mut out);
    }
    out
}

/// Check if a statement (recursively) contains a Continue statement.
pub(in crate::mir::builder) fn has_continue_recursive(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Continue { .. } => true,
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(has_continue_recursive)
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(has_continue_recursive))
        }
        ASTNode::Loop { body, .. } => body.iter().any(has_continue_recursive),
        ASTNode::While { body, .. } => body.iter().any(has_continue_recursive),
        ASTNode::ForRange { body, .. } => body.iter().any(has_continue_recursive),
        ASTNode::Program { statements, .. } => statements.iter().any(has_continue_recursive),
        ASTNode::ScopeBox { body, .. } => body.iter().any(has_continue_recursive),
        _ => false,
    }
}

pub(in crate::mir::builder) fn body_has_break_or_continue_stmt(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Break { .. } | ASTNode::Continue { .. } => true,
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(body_has_break_or_continue_stmt)
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(body_has_break_or_continue_stmt))
        }
        ASTNode::Loop { body, .. } => body.iter().any(body_has_break_or_continue_stmt),
        ASTNode::Program { statements, .. } => {
            statements.iter().any(body_has_break_or_continue_stmt)
        }
        _ => false,
    }
}
