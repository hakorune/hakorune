use crate::ast::ASTNode;
use crate::mir::builder::control_flow::generic_loop_canon::matches_loop_increment;

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
            ASTNode::MethodCall { object, .. } => {
                if let Some(name) = receiver_candidate_name(object.as_ref()) {
                    if !out.iter().any(|v| v == &name) {
                        out.push(name);
                    }
                }
            }
            ASTNode::Local { initial_values, .. } => {
                for init in initial_values.iter().flatten() {
                    walk(init.as_ref(), out);
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
            ASTNode::Loop { body, .. } => {
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

fn receiver_candidate_name(object: &ASTNode) -> Option<String> {
    match object {
        ASTNode::Variable { name, .. } => Some(name.clone()),
        ASTNode::Me { .. } => Some("me".to_string()),
        _ => None,
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
