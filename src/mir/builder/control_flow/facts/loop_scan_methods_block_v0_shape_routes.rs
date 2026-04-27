use crate::ast::ASTNode;

use crate::mir::builder::control_flow::facts::scan_common_predicates::{
    as_var_name, extract_step_var_from_tail, is_int_lit, match_next_i_guard,
};

pub(in crate::mir::builder) struct LoopScanMethodsBlockShapeMatch;

pub(in crate::mir::builder) fn try_match_loop_scan_methods_block_shape(
    body: &[ASTNode],
    loop_var: &str,
    limit_var: &str,
) -> Result<LoopScanMethodsBlockShapeMatch, String> {
    if body.len() < 6 {
        return Err("body_too_short".to_string());
    }

    if !declares_local_var(&body[0], "next_i")
        || !declares_local_var(&body[1], "k")
        || !declares_local_var(&body[2], "name_start")
    {
        return Err("missing_required_locals".to_string());
    }

    let Some(last) = body.last() else {
        return Err("body_last_missing".to_string());
    };
    let Some(next_i_var) = extract_step_var_from_tail(last, loop_var) else {
        return Err("tail_not_i_eq_next_i".to_string());
    };

    let Some(prev) = body.get(body.len().saturating_sub(2)) else {
        return Err("body_too_short_for_tail_guard".to_string());
    };
    if !match_next_i_guard(prev, &next_i_var, loop_var) {
        return Err("tail_guard_shape".to_string());
    }

    if match_scan_window_block(&body[3], limit_var).is_none() {
        if let Some(recv) = scan_window_substring_receiver(&body[3]) {
            return Err(format!("scan_window_block_shape receiver={recv}"));
        }
        return Err("scan_window_block_shape".to_string());
    }

    Ok(LoopScanMethodsBlockShapeMatch)
}

fn declares_local_var(stmt: &ASTNode, name: &str) -> bool {
    let ASTNode::Local { variables, .. } = stmt else {
        return false;
    };
    variables.iter().any(|v| v == name)
}

fn block_stmt_body(stmt: &ASTNode) -> Option<&[ASTNode]> {
    match stmt {
        ASTNode::Program { statements, .. } => Some(statements),
        ASTNode::ScopeBox { body, .. } => Some(body),
        _ => None,
    }
}

fn match_scan_window_block<'a>(
    stmt: &'a ASTNode,
    limit_var: &str,
) -> Option<(&'a [ASTNode], &'a ASTNode, String, String, String)> {
    let stmts = block_stmt_body(stmt)?;
    if stmts.len() != 4 {
        return None;
    }

    if !declares_local_var(&stmts[0], "pat") {
        return None;
    }
    if !declares_local_var(&stmts[1], "m") {
        return None;
    }
    if !declares_local_var(&stmts[2], "j") {
        return None;
    }

    let inner_loop = &stmts[3];
    let (condition, body) = match inner_loop {
        ASTNode::Loop {
            condition, body, ..
        }
        | ASTNode::While {
            condition, body, ..
        } => (condition.as_ref(), body.as_slice()),
        _ => return None,
    };

    let (j_var, m_var) = match condition {
        ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::LessEqual,
            left,
            right,
            ..
        } if as_var_name(right.as_ref()) == Some(limit_var) => match left.as_ref() {
            ASTNode::BinaryOp {
                operator: crate::ast::BinaryOperator::Add,
                left: add_left,
                right: add_right,
                ..
            } => (
                as_var_name(add_left.as_ref())?.to_string(),
                as_var_name(add_right.as_ref())?.to_string(),
            ),
            _ => return None,
        },
        _ => return None,
    };

    if body.len() != 2 {
        return None;
    }

    let ASTNode::If {
        condition: if_cond,
        then_body,
        else_body,
        ..
    } = &body[0]
    else {
        return None;
    };
    if else_body.is_some() {
        return None;
    }
    if then_body.len() != 2 {
        return None;
    }

    let (recv_var, substring_ok) = match if_cond.as_ref() {
        ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Equal,
            left,
            right,
            ..
        } => match left.as_ref() {
            ASTNode::MethodCall {
                object,
                method,
                arguments,
                ..
            } => {
                let recv_var = as_var_name(object.as_ref())?.to_string();
                let ok = method == "substring"
                    && arguments.len() == 2
                    && as_var_name(&arguments[0]) == Some(j_var.as_str())
                    && matches!(
                        &arguments[1],
                        ASTNode::BinaryOp { operator: crate::ast::BinaryOperator::Add, left: a, right: b, .. }
                            if as_var_name(a.as_ref()) == Some(j_var.as_str())
                                && as_var_name(b.as_ref()) == Some(m_var.as_str())
                    )
                    && as_var_name(right.as_ref()) == Some("pat");
                (Some(recv_var), ok)
            }
            _ => (None, false),
        },
        _ => (None, false),
    };
    let Some(recv_var) = recv_var else {
        return None;
    };
    if !substring_ok {
        return None;
    }

    let assigns_k = matches!(
        &then_body[0],
        ASTNode::Assignment { target, value, .. }
            if as_var_name(target.as_ref()) == Some("k") && as_var_name(value.as_ref()) == Some(j_var.as_str())
    );
    if !assigns_k {
        return None;
    }
    if !matches!(&then_body[1], ASTNode::Break { .. }) {
        return None;
    }

    let step_ok = matches!(
        &body[1],
        ASTNode::Assignment { target, value, .. }
            if as_var_name(target.as_ref()) == Some(j_var.as_str()) && matches!(
                value.as_ref(),
                ASTNode::BinaryOp { operator: crate::ast::BinaryOperator::Add, left, right, .. }
                    if as_var_name(left.as_ref()) == Some(j_var.as_str()) && is_int_lit(right.as_ref(), 1)
            )
    );
    if !step_ok {
        return None;
    }

    Some((stmts, inner_loop, j_var, m_var, recv_var))
}

fn scan_window_substring_receiver(stmt: &ASTNode) -> Option<String> {
    let stmts = block_stmt_body(stmt)?;
    if stmts.len() != 4 {
        return None;
    }

    let inner_loop = &stmts[3];
    let body = match inner_loop {
        ASTNode::Loop { body, .. } | ASTNode::While { body, .. } => body.as_slice(),
        _ => return None,
    };
    if body.is_empty() {
        return None;
    }

    let ASTNode::If { condition, .. } = &body[0] else {
        return None;
    };
    let ASTNode::BinaryOp { left, .. } = condition.as_ref() else {
        return None;
    };
    let ASTNode::MethodCall { object, method, .. } = left.as_ref() else {
        return None;
    };
    if method != "substring" {
        return None;
    }
    Some(as_var_name(object.as_ref())?.to_string())
}
