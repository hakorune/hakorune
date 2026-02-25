//! Facts for loop_scan_methods_v0 (one-shape, planner-required only).

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::facts::stmt_view::try_build_stmt_only_block_recipe;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
use crate::mir::policies::BodyLoweringPolicy;

use super::recipe::{LoopScanMethodsV0Recipe, LoopScanSegment, NestedLoopRecipe};

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopScanMethodsV0Facts {
    pub loop_var: String,
    pub limit_var: String,
    pub condition: ASTNode,
    pub body_lowering_policy: BodyLoweringPolicy,
    pub recipe: LoopScanMethodsV0Recipe,
}

fn planner_required_enabled() -> bool {
    let strict_or_dev =
        crate::config::env::joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled()
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

fn contains_scan_window_loop(stmts: &[ASTNode], limit_var: &str) -> bool {
    fn cond_is_j_plus_m_le_n(cond: &ASTNode, limit_var: &str) -> bool {
        matches!(
            cond,
            ASTNode::BinaryOp {
                operator: BinaryOperator::LessEqual,
                left,
                right,
                ..
            } if as_var_name(right.as_ref()) == Some(limit_var)
                && matches!(left.as_ref(), ASTNode::BinaryOp { operator: BinaryOperator::Add, left: l, right: r, .. }
                    if as_var_name(l.as_ref()).is_some() && as_var_name(r.as_ref()).is_some())
        )
    }

    fn walk(stmts: &[ASTNode], limit_var: &str) -> bool {
        for stmt in stmts {
            match stmt {
                ASTNode::Loop { condition, .. } => {
                    if cond_is_j_plus_m_le_n(condition.as_ref(), limit_var) {
                        return true;
                    }
                }
                ASTNode::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    if walk(then_body, limit_var) {
                        return true;
                    }
                    if else_body.as_ref().is_some_and(|b| walk(b, limit_var)) {
                        return true;
                    }
                }
                ASTNode::Program { statements, .. } => {
                    if walk(statements, limit_var) {
                        return true;
                    }
                }
                ASTNode::ScopeBox { body, .. } => {
                    if walk(body, limit_var) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    walk(stmts, limit_var)
}

fn contains_exit_outside_nested_loops(stmts: &[ASTNode]) -> bool {
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
                // NOTE: Exits inside nested loops are allowed; do not recurse.
                ASTNode::Loop { .. } | ASTNode::While { .. } | ASTNode::ForRange { .. } => {}
                _ => {}
            }
        }
        false
    }

    walk(stmts)
}

pub(in crate::mir::builder) fn try_extract_loop_scan_methods_v0_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopScanMethodsV0Facts>, Freeze> {
    let debug = crate::config::env::joinir_dev::debug_enabled();
    let debug_reject = |reason: &str| {
        if debug {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[plan/reject_detail] box=loop_scan_methods_v0 reason={}",
                reason
            ));
        }
    };

    if !planner_required_enabled() {
        debug_reject("planner_required_off");
        return Ok(None);
    }

    let Some((loop_var, limit_var)) = is_loop_cond_i_lt_n(condition) else {
        debug_reject("cond_not_i_lt_n");
        return Ok(None);
    };

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

    if !body.iter().any(|stmt| declares_local_var(stmt, &next_i_var)) {
        debug_reject("missing_next_i_local");
        return Ok(None);
    }

    if contains_exit_outside_nested_loops(body) {
        debug_reject("exit_outside_nested_loops");
        return Ok(None);
    }

    if !contains_scan_window_loop(body, &limit_var) {
        debug_reject("missing_window_loop_j_plus_m_le_n");
        return Ok(None);
    }

    const ALLOW_EXTENDED: bool = true;
    let mut segments = Vec::new();
    let mut linear = Vec::new();
    for (_idx, stmt) in body.iter().enumerate() {
        match stmt {
            ASTNode::Loop { .. } | ASTNode::While { .. } => {
                if !linear.is_empty() {
                    let Some(recipe) = try_build_no_exit_block_recipe(&linear, ALLOW_EXTENDED)
                    else {
                        debug_reject("linear_segment_not_no_exit");
                        return Ok(None);
                    };
                    segments.push(LoopScanSegment::Linear(recipe));
                    linear.clear();
                }
                let nested = match stmt {
                    ASTNode::Loop { condition, body, .. }
                    | ASTNode::While { condition, body, .. } => {
                        let cond_view = CondBlockView::from_expr(condition);
                        let body_stmt_only = try_build_stmt_only_block_recipe(body);
                        NestedLoopRecipe {
                            cond_view,
                            loop_stmt: stmt.clone(),
                            body: RecipeBody::new(body.to_vec()),
                            body_stmt_only,
                        }
                    }
                    _ => {
                        debug_reject("nested_loop_stmt_not_loop_or_while");
                        return Ok(None);
                    }
                };
                segments.push(LoopScanSegment::NestedLoop(nested));
            }
            _ => {
                linear.push(stmt.clone());
            }
        }
    }
    if !linear.is_empty() {
        let Some(recipe) = try_build_no_exit_block_recipe(&linear, ALLOW_EXTENDED) else {
            debug_reject("linear_tail_segment_not_no_exit");
            return Ok(None);
        };
        segments.push(LoopScanSegment::Linear(recipe));
    }

    Ok(Some(LoopScanMethodsV0Facts {
        loop_var,
        limit_var,
        condition: condition.clone(),
        body_lowering_policy: BodyLoweringPolicy::RecipeOnly,
        recipe: LoopScanMethodsV0Recipe {
            next_i_var,
            body: RecipeBody::new(body.to_vec()),
            segments,
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::try_extract_loop_scan_methods_v0_facts;
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
    fn policy_recipe_only_for_loop_scan_methods_v0() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(BinaryOperator::Less, var("i"), var("n"));
        let body = vec![
            local("next_i", Some(int(0))),
            local("j", Some(int(0))),
            local("m", Some(int(0))),
            ASTNode::Loop {
                condition: Box::new(binop(
                    BinaryOperator::LessEqual,
                    binop(BinaryOperator::Add, var("j"), var("m")),
                    var("n"),
                )),
                body: vec![assign(var("j"), binop(BinaryOperator::Add, var("j"), int(1)))],
                span: Span::unknown(),
            },
            ASTNode::If {
                condition: Box::new(binop(
                    BinaryOperator::LessEqual,
                    var("next_i"),
                    var("i"),
                )),
                then_body: vec![assign(var("next_i"), binop(BinaryOperator::Add, var("i"), int(1)))],
                else_body: None,
                span: Span::unknown(),
            },
            assign(var("i"), var("next_i")),
        ];

        let facts = try_extract_loop_scan_methods_v0_facts(&condition, &body)
            .expect("extract ok")
            .expect("facts");

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::RecipeOnly
        ));
    }
}
