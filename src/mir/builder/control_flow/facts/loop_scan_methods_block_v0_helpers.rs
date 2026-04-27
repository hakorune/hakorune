use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::facts::scan_common_predicates::{
    as_var_name, is_int_lit, is_loop_cond_var_lt_var as shared_is_loop_cond_var_lt_var,
    is_var_plus_one,
};
use crate::mir::builder::control_flow::facts::stmt_view::try_build_stmt_only_block_recipe;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::recipes::loop_scan_methods_block_v0::{
    LinearBlockRecipe, NestedLoopRecipe, ScanSegment,
};
use crate::mir::builder::control_flow::recipes::RecipeBody;

pub(in crate::mir::builder) fn release_enabled() -> bool {
    true
}

pub(in crate::mir::builder) fn is_loop_cond_i_lt_n(ast: &ASTNode) -> Option<(String, String)> {
    shared_is_loop_cond_var_lt_var(ast)
}

pub(super) fn declares_local_var(stmt: &ASTNode, name: &str) -> bool {
    let ASTNode::Local { variables, .. } = stmt else {
        return false;
    };
    variables.iter().any(|v| v == name)
}

pub(super) fn match_next_i_guard(stmt: &ASTNode, next_i: &str, loop_var: &str) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if else_body.is_some() {
        return false;
    }
    if then_body.len() != 1 {
        return false;
    }

    let cond_ok = matches!(
        condition.as_ref(),
        ASTNode::BinaryOp {
            operator: BinaryOperator::LessEqual,
            left,
            right,
            ..
        } if as_var_name(left.as_ref()) == Some(next_i) && as_var_name(right.as_ref()) == Some(loop_var)
    );
    if !cond_ok {
        return false;
    }

    matches!(
        &then_body[0],
        ASTNode::Assignment { target, value, .. }
            if as_var_name(target.as_ref()) == Some(next_i) && is_var_plus_one(value.as_ref(), loop_var)
    )
}

pub(super) fn extract_next_i_from_tail(stmt: &ASTNode, loop_var: &str) -> Option<String> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    if as_var_name(target.as_ref()) != Some(loop_var) {
        return None;
    }
    Some(as_var_name(value.as_ref())?.to_string())
}

fn block_stmt_body(stmt: &ASTNode) -> Option<&[ASTNode]> {
    match stmt {
        ASTNode::Program { statements, .. } => Some(statements),
        ASTNode::ScopeBox { body, .. } => Some(body),
        _ => None,
    }
}

pub(super) fn match_scan_window_block<'a>(
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
            operator: BinaryOperator::LessEqual,
            left,
            right,
            ..
        } if as_var_name(right.as_ref()) == Some(limit_var) => match left.as_ref() {
            ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
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
            operator: BinaryOperator::Equal,
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
                        ASTNode::BinaryOp { operator: BinaryOperator::Add, left: a, right: b, .. }
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
                ASTNode::BinaryOp { operator: BinaryOperator::Add, left, right, .. }
                    if as_var_name(left.as_ref()) == Some(j_var.as_str()) && is_int_lit(right.as_ref(), 1)
            )
    );
    if !step_ok {
        return None;
    }

    Some((stmts, inner_loop, j_var, m_var, recv_var))
}

pub(super) fn scan_window_substring_receiver(stmt: &ASTNode) -> Option<String> {
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

pub(super) fn flatten_stmt_list(stmts: &[ASTNode], out: &mut Vec<ASTNode>) {
    for stmt in stmts {
        match stmt {
            ASTNode::Program { statements, .. } => flatten_stmt_list(statements, out),
            ASTNode::ScopeBox { body, .. } => flatten_stmt_list(body, out),
            _ => out.push(stmt.clone()),
        }
    }
}

fn build_linear_block_recipe(stmts: &[ASTNode], allow_extended: bool) -> Option<LinearBlockRecipe> {
    if let Some(recipe) = try_build_no_exit_block_recipe(stmts, allow_extended) {
        return Some(LinearBlockRecipe::NoExit(recipe));
    }
    if let Some(recipe) = try_build_exit_allowed_block_recipe(stmts, allow_extended) {
        return Some(LinearBlockRecipe::ExitAllowed(recipe));
    }
    None
}

pub(super) fn try_segmentize_stmt_list(
    stmts: &[ASTNode],
    allow_extended: bool,
) -> Option<Vec<ScanSegment>> {
    if stmts.is_empty() {
        return None;
    }

    let mut segments = Vec::new();

    let mut cur_stmts: Vec<ASTNode> = Vec::new();
    let mut cur_recipe: Option<LinearBlockRecipe> = None;

    let flush_linear = |segments: &mut Vec<ScanSegment>,
                        cur_stmts: &mut Vec<ASTNode>,
                        cur_recipe: &mut Option<LinearBlockRecipe>| {
        if let Some(recipe) = cur_recipe.take() {
            segments.push(ScanSegment::Linear(recipe));
            cur_stmts.clear();
        }
    };

    for stmt in stmts {
        match stmt {
            ASTNode::Loop {
                condition, body, ..
            }
            | ASTNode::While {
                condition, body, ..
            } => {
                flush_linear(&mut segments, &mut cur_stmts, &mut cur_recipe);

                segments.push(ScanSegment::NestedLoop(NestedLoopRecipe {
                    cond_view: CondBlockView::from_expr(condition),
                    loop_stmt: stmt.clone(),
                    body: RecipeBody::new(body.to_vec()),
                    body_stmt_only: try_build_stmt_only_block_recipe(body),
                }));
            }
            _ => {
                let mut candidate = cur_stmts.clone();
                candidate.push(stmt.clone());

                if let Some(recipe) = build_linear_block_recipe(&candidate, allow_extended) {
                    cur_stmts = candidate;
                    cur_recipe = Some(recipe);
                    continue;
                }

                flush_linear(&mut segments, &mut cur_stmts, &mut cur_recipe);

                cur_stmts.push(stmt.clone());
                cur_recipe = build_linear_block_recipe(&cur_stmts, allow_extended);
                if cur_recipe.is_none() {
                    return None;
                }
            }
        }
    }

    flush_linear(&mut segments, &mut cur_stmts, &mut cur_recipe);
    if segments.is_empty() {
        return None;
    }
    Some(segments)
}
