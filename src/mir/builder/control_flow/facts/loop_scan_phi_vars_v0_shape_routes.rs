use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::scan_common_predicates::{
    as_var_name, is_int_lit, is_var_plus_expr, is_var_plus_one,
};
use crate::mir::builder::control_flow::recipes::loop_scan_phi_vars_v0::LoopScanPhiVarsV0Recipe;

pub(in crate::mir::builder) struct LoopScanPhiVarsShapeMatch {
    pub prefix_end: usize,
    pub nested_idx: usize,
    pub step_start: usize,
    pub recipe: LoopScanPhiVarsV0Recipe,
}

pub(in crate::mir::builder) fn try_match_loop_scan_phi_vars_len7_shape(
    body: &[ASTNode],
    loop_var: &str,
) -> Result<LoopScanPhiVarsShapeMatch, &'static str> {
    if !is_local_decl(&body[0]) {
        return Err("stmt0_not_local");
    }
    if !is_local_init_zero(&body[1]) {
        return Err("stmt1_not_local_init_zero");
    }
    if !is_local_decl(&body[2]) {
        return Err("stmt2_not_local_m");
    }
    if !is_local_init_zero(&body[3]) {
        return Err("stmt3_not_local_found_init_zero");
    }
    if !is_loop_with_break(&body[4]) {
        return Err("stmt4_not_loop_with_break");
    }
    if !is_if_stmt(&body[5]) {
        return Err("stmt5_not_if");
    }
    if !is_inc_stmt(&body[6], loop_var) {
        return Err("stmt6_not_inc");
    }

    Ok(LoopScanPhiVarsShapeMatch {
        prefix_end: 4,
        nested_idx: 4,
        step_start: 6,
        recipe: LoopScanPhiVarsV0Recipe {
            inner_loop_search: body[4].clone(),
            found_if_stmt: Some(body[5].clone()),
        },
    })
}

pub(in crate::mir::builder) fn try_match_loop_scan_phi_vars_ext_shape01(
    body: &[ASTNode],
    loop_var: &str,
) -> Result<LoopScanPhiVarsShapeMatch, &'static str> {
    if !is_local_init_zero(&body[0]) {
        return Err("stmt0_not_local_j_init_zero_ext_shape01");
    }
    if !is_local_decl(&body[1]) {
        return Err("stmt1_not_local_m_ext_shape01");
    }
    if !is_loop_without_exit(&body[2]) {
        return Err("stmt2_not_loop_no_exit_ext_shape01");
    }
    if !is_var_step_stmt_nonconst(&body[3], loop_var) {
        return Err("stmt3_not_nonconst_var_step_ext_shape01");
    }

    Ok(LoopScanPhiVarsShapeMatch {
        prefix_end: 2,
        nested_idx: 2,
        step_start: 3,
        recipe: LoopScanPhiVarsV0Recipe {
            inner_loop_search: body[2].clone(),
            found_if_stmt: None,
        },
    })
}

fn is_local_decl(stmt: &ASTNode) -> bool {
    matches!(stmt, ASTNode::Local { .. })
}

fn is_local_init_zero(stmt: &ASTNode) -> bool {
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

fn is_loop_with_break(stmt: &ASTNode) -> bool {
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

fn is_if_stmt(stmt: &ASTNode) -> bool {
    matches!(stmt, ASTNode::If { .. })
}

fn is_inc_stmt(stmt: &ASTNode, loop_var: &str) -> bool {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            as_var_name(target.as_ref()) == Some(loop_var)
                && is_var_plus_one(value.as_ref(), loop_var)
        }
        _ => false,
    }
}

fn is_var_step_stmt_nonconst(stmt: &ASTNode, loop_var: &str) -> bool {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            as_var_name(target.as_ref()) == Some(loop_var)
                && is_var_plus_expr(value.as_ref(), loop_var)
                && !is_var_plus_one(value.as_ref(), loop_var)
        }
        _ => false,
    }
}

fn is_loop_without_exit(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Loop { body, .. } => !contains_exit_anywhere(body),
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
