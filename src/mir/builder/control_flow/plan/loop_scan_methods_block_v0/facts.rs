//! Facts for loop_scan_methods_block_v0 (one-shape, planner-required only).

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::plan::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::facts::stmt_view::try_build_stmt_only_block_recipe;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
use crate::mir::policies::BodyLoweringPolicy;

use super::recipe::{
    LinearBlockRecipe, LoopScanMethodsBlockV0Recipe, NestedLoopRecipe, ScanSegment,
};

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopScanMethodsBlockV0Facts {
    pub loop_var: String,
    pub limit_var: String,
    pub condition: ASTNode,
    pub body_lowering_policy: BodyLoweringPolicy,
    pub recipe: LoopScanMethodsBlockV0Recipe,
}

fn release_enabled() -> bool {
    true
}

fn as_var_name(ast: &ASTNode) -> Option<&str> {
    match ast {
        ASTNode::Variable { name, .. } => Some(name),
        _ => None,
    }
}

fn is_int_lit(ast: &ASTNode, value: i64) -> bool {
    matches!(ast, ASTNode::Literal { value: LiteralValue::Integer(v), .. } if *v == value)
}

fn is_var_plus_one(ast: &ASTNode, var: &str) -> bool {
    matches!(
        ast,
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left,
            right,
            ..
        } if as_var_name(left.as_ref()) == Some(var) && is_int_lit(right.as_ref(), 1)
    )
}

fn is_loop_cond_i_lt_n(ast: &ASTNode) -> Option<(String, String)> {
    match ast {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left,
            right,
            ..
        } => Some((
            as_var_name(left.as_ref())?.to_string(),
            as_var_name(right.as_ref())?.to_string(),
        )),
        _ => None,
    }
}

fn declares_local_var(stmt: &ASTNode, name: &str) -> bool {
    let ASTNode::Local { variables, .. } = stmt else {
        return false;
    };
    variables.iter().any(|v| v == name)
}

fn match_next_i_guard(stmt: &ASTNode, next_i: &str, loop_var: &str) -> bool {
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

fn extract_next_i_from_tail(stmt: &ASTNode, loop_var: &str) -> Option<String> {
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

fn flatten_stmt_list(stmts: &[ASTNode], out: &mut Vec<ASTNode>) {
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

fn try_segmentize_stmt_list(stmts: &[ASTNode], allow_extended: bool) -> Option<Vec<ScanSegment>> {
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

pub(in crate::mir::builder) fn try_extract_loop_scan_methods_block_v0_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopScanMethodsBlockV0Facts>, Freeze> {
    let debug = crate::config::env::joinir_dev::debug_enabled();
    let debug_reject = |reason: &str| {
        if debug {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[plan/reject_detail] box=loop_scan_methods_block_v0 reason={}",
                reason
            ));
        }
    };

    if !release_enabled() {
        debug_reject("planner_required_off");
        return Ok(None);
    }

    let Some((loop_var, limit_var)) = is_loop_cond_i_lt_n(condition) else {
        debug_reject("cond_not_i_lt_n");
        return Ok(None);
    };

    if body.len() < 6 {
        debug_reject("body_too_short");
        return Ok(None);
    }

    if !declares_local_var(&body[0], "next_i")
        || !declares_local_var(&body[1], "k")
        || !declares_local_var(&body[2], "name_start")
    {
        debug_reject("missing_required_locals");
        return Ok(None);
    }

    let Some(last) = body.last() else {
        debug_reject("body_last_missing");
        return Ok(None);
    };
    let Some(next_i_var) = extract_next_i_from_tail(last, &loop_var) else {
        debug_reject("tail_not_i_eq_next_i");
        return Ok(None);
    };

    let Some(prev) = body.get(body.len().saturating_sub(2)) else {
        debug_reject("body_too_short_for_tail_guard");
        return Ok(None);
    };
    if !match_next_i_guard(prev, &next_i_var, &loop_var) {
        debug_reject("tail_guard_shape");
        return Ok(None);
    }

    if match_scan_window_block(&body[3], &limit_var).is_none() {
        if let Some(recv) = scan_window_substring_receiver(&body[3]) {
            debug_reject(&format!("scan_window_block_shape receiver={recv}"));
        } else {
            debug_reject("scan_window_block_shape");
        }
        return Ok(None);
    }

    const ALLOW_EXTENDED: bool = true;
    let mut flat = Vec::new();
    flatten_stmt_list(body, &mut flat);
    let Some(segments) = try_segmentize_stmt_list(&flat, ALLOW_EXTENDED) else {
        debug_reject("segmentize_failed");
        return Ok(None);
    };

    Ok(Some(LoopScanMethodsBlockV0Facts {
        loop_var,
        limit_var,
        condition: condition.clone(),
        body_lowering_policy: BodyLoweringPolicy::ExitAllowed {
            allow_join_if: false,
        },
        recipe: LoopScanMethodsBlockV0Recipe {
            next_i_var,
            body: RecipeBody::new(body.to_vec()),
            segments,
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::try_extract_loop_scan_methods_block_v0_facts;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::policies::BodyLoweringPolicy;

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn string(value: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(value.to_string()),
            span: Span::unknown(),
        }
    }

    fn binop(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn assign(target: ASTNode, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(target),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    fn local(name: &str, init: Option<ASTNode>) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![init.map(Box::new)],
            span: Span::unknown(),
        }
    }

    #[test]
    fn policy_exit_allowed_for_loop_scan_methods_block_v0() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(BinaryOperator::Less, var("i"), var("n"));
        let inner_loop_body = vec![
            ASTNode::If {
                condition: Box::new(binop(
                    BinaryOperator::Equal,
                    ASTNode::MethodCall {
                        object: Box::new(var("s")),
                        method: "substring".to_string(),
                        arguments: vec![var("j"), binop(BinaryOperator::Add, var("j"), var("m"))],
                        span: Span::unknown(),
                    },
                    var("pat"),
                )),
                then_body: vec![
                    assign(var("k"), var("j")),
                    ASTNode::Break {
                        span: Span::unknown(),
                    },
                ],
                else_body: None,
                span: Span::unknown(),
            },
            assign(var("j"), binop(BinaryOperator::Add, var("j"), int(1))),
        ];

        let scan_window_block = ASTNode::Program {
            statements: vec![
                local("pat", Some(string("p"))),
                local("m", Some(int(1))),
                local("j", Some(int(0))),
                ASTNode::Loop {
                    condition: Box::new(binop(
                        BinaryOperator::LessEqual,
                        binop(BinaryOperator::Add, var("j"), var("m")),
                        var("n"),
                    )),
                    body: inner_loop_body,
                    span: Span::unknown(),
                },
            ],
            span: Span::unknown(),
        };

        let body = vec![
            local("next_i", Some(int(0))),
            local("k", Some(int(0))),
            local("name_start", Some(int(0))),
            scan_window_block,
            ASTNode::If {
                condition: Box::new(binop(BinaryOperator::LessEqual, var("next_i"), var("i"))),
                then_body: vec![assign(
                    var("next_i"),
                    binop(BinaryOperator::Add, var("i"), int(1)),
                )],
                else_body: None,
                span: Span::unknown(),
            },
            assign(var("i"), var("next_i")),
        ];

        let facts = try_extract_loop_scan_methods_block_v0_facts(&condition, &body)
            .expect("extract ok")
            .expect("facts");

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::ExitAllowed { .. }
        ));
    }
}
