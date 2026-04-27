//! Facts for loop_scan_v0 (one-shape, planner-required only).

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::facts::scan_common_predicates::{
    as_var_name, is_int_lit, is_var_plus_one,
};
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::policies::BodyLoweringPolicy;

use super::recipe::{LoopScanSegment, LoopScanV0Recipe};

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopScanV0Facts {
    pub loop_var: String,
    pub limit_var: String,
    pub condition: ASTNode,
    pub body_lowering_policy: BodyLoweringPolicy,
    pub recipe: LoopScanV0Recipe,
    pub segments: Vec<LoopScanSegment>,
}

fn release_enabled() -> bool {
    true
}

fn as_var_minus_one(ast: &ASTNode) -> Option<&str> {
    match ast {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Subtract,
            left,
            right,
            ..
        } if is_int_lit(right.as_ref(), 1) => as_var_name(left.as_ref()),
        _ => None,
    }
}

fn is_eq_string(ast: &ASTNode, var: &str, s: &str) -> bool {
    matches!(
        ast,
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left,
            right,
            ..
        } if as_var_name(left.as_ref()) == Some(var)
            && matches!(right.as_ref(), ASTNode::Literal { value: LiteralValue::String(v), .. } if v == s)
    )
}

fn is_loop_scan_range_cond(ast: &ASTNode) -> Option<(String, String)> {
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
        ASTNode::BinaryOp {
            operator: BinaryOperator::LessEqual,
            left,
            right,
            ..
        } => Some((
            as_var_name(left.as_ref())?.to_string(),
            as_var_minus_one(right.as_ref())?.to_string(),
        )),
        _ => None,
    }
}

fn is_local_ch_substring_i_i1(stmt: &ASTNode, loop_var: &str) -> Option<(String, ASTNode)> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return None;
    };
    if variables.len() != 1 || initial_values.len() != 1 {
        return None;
    }
    let ch_name = variables[0].clone();
    let init = initial_values[0].as_ref()?.as_ref();
    let (object, method, arguments): (&ASTNode, &str, &[ASTNode]) = match init {
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => (object.as_ref(), method.as_str(), arguments.as_slice()),
        ASTNode::Call {
            callee, arguments, ..
        } => {
            let ASTNode::FieldAccess { object, field, .. } = callee.as_ref() else {
                return None;
            };
            (object.as_ref(), field.as_str(), arguments.as_slice())
        }
        _ => return None,
    };

    if as_var_name(object).is_none() {
        return None;
    }
    if method != "substring" {
        return None;
    }
    if arguments.len() != 2 {
        return None;
    }
    if as_var_name(&arguments[0]) != Some(loop_var) {
        return None;
    }
    if !is_var_plus_one(&arguments[1], loop_var) {
        return None;
    }
    Some((ch_name, stmt.clone()))
}

fn is_inc_stmt(stmt: &ASTNode, loop_var: &str) -> Option<ASTNode> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    if as_var_name(target.as_ref()) != Some(loop_var) {
        return None;
    }
    if !is_var_plus_one(value.as_ref(), loop_var) {
        return None;
    }
    Some(stmt.clone())
}

fn is_comma_continue_if(
    stmt: &ASTNode,
    loop_var: &str,
    ch_var: &str,
) -> Option<(ASTNode, ASTNode)> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return None;
    };
    if else_body.is_some() {
        return None;
    }
    if then_body.len() != 2 {
        return None;
    }
    if !is_eq_string(condition.as_ref(), ch_var, ",") {
        return None;
    }
    let inc = is_inc_stmt(&then_body[0], loop_var)?;
    if !matches!(then_body[1], ASTNode::Continue { .. }) {
        return None;
    }
    Some(((*condition.clone()), inc))
}

fn is_close_break_if(stmt: &ASTNode, ch_var: &str) -> Option<ASTNode> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return None;
    };
    if else_body.is_some() {
        return None;
    }
    if then_body.len() != 1 {
        return None;
    }
    if !is_eq_string(condition.as_ref(), ch_var, "]") {
        return None;
    }
    if !matches!(then_body[0], ASTNode::Break { .. }) {
        return None;
    }
    Some(*condition.clone())
}

pub(in crate::mir::builder) fn try_extract_loop_scan_v0_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopScanV0Facts>, Freeze> {
    let debug = crate::config::env::joinir_dev::debug_enabled();
    let debug_reject = |reason: &str| {
        if debug {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[plan/reject_detail] box=loop_scan_v0 reason={}",
                reason
            ));
        }
    };

    if !release_enabled() {
        debug_reject("planner_required_off");
        return Ok(None);
    }

    let Some((loop_var, limit_var)) = is_loop_scan_range_cond(condition) else {
        debug_reject("cond_not_loop_scan_range_v0");
        return Ok(None);
    };

    if body.len() != 4 {
        debug_reject("body_len");
        return Ok(None);
    }

    let Some((ch_var, local_ch_stmt)) = is_local_ch_substring_i_i1(&body[0], &loop_var) else {
        debug_reject("local_ch_not_substring_i_i1");
        return Ok(None);
    };
    let Some((comma_if_cond, comma_inc_stmt)) = is_comma_continue_if(&body[1], &loop_var, &ch_var)
    else {
        debug_reject("comma_if_shape");
        return Ok(None);
    };
    let Some(close_if_cond) = is_close_break_if(&body[2], &ch_var) else {
        debug_reject("close_if_shape");
        return Ok(None);
    };
    let Some(step_inc_stmt) = is_inc_stmt(&body[3], &loop_var) else {
        debug_reject("step_inc_shape");
        return Ok(None);
    };

    let (body_lowering_policy, segments) = match try_build_exit_allowed_block_recipe(body, true) {
        Some(recipe) => (
            BodyLoweringPolicy::ExitAllowed {
                allow_join_if: false,
            },
            vec![LoopScanSegment::Linear(recipe)],
        ),
        None => {
            debug_reject("segments_exit_allowed_missing");
            (BodyLoweringPolicy::RecipeOnly, Vec::new())
        }
    };

    Ok(Some(LoopScanV0Facts {
        loop_var,
        limit_var,
        condition: condition.clone(),
        body_lowering_policy,
        recipe: LoopScanV0Recipe {
            local_ch_stmt,
            comma_if_cond,
            comma_inc_stmt,
            close_if_cond,
            step_inc_stmt,
        },
        segments,
    }))
}

#[cfg(test)]
mod tests {
    use super::try_extract_loop_scan_v0_facts;
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

    fn local(name: &str, init: ASTNode) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![Some(Box::new(init))],
            span: Span::unknown(),
        }
    }

    fn local_substring_ch(loop_var: &str, ch_var: &str) -> ASTNode {
        let substring_call = ASTNode::MethodCall {
            object: Box::new(var("s")),
            method: "substring".to_string(),
            arguments: vec![
                var(loop_var),
                binop(BinaryOperator::Add, var(loop_var), int(1)),
            ],
            span: Span::unknown(),
        };
        local(ch_var, substring_call)
    }

    fn loop_scan_body(loop_var: &str, ch_var: &str) -> Vec<ASTNode> {
        vec![
            local_substring_ch(loop_var, ch_var),
            ASTNode::If {
                condition: Box::new(binop(BinaryOperator::Equal, var(ch_var), string(","))),
                then_body: vec![
                    assign(
                        var(loop_var),
                        binop(BinaryOperator::Add, var(loop_var), int(1)),
                    ),
                    ASTNode::Continue {
                        span: Span::unknown(),
                    },
                ],
                else_body: None,
                span: Span::unknown(),
            },
            ASTNode::If {
                condition: Box::new(binop(BinaryOperator::Equal, var(ch_var), string("]"))),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            assign(
                var(loop_var),
                binop(BinaryOperator::Add, var(loop_var), int(1)),
            ),
        ]
    }

    #[test]
    fn policy_exit_allowed_for_loop_scan_v0_shape() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(BinaryOperator::Less, var("i"), var("n"));
        let body = loop_scan_body("i", "ch");

        let facts = try_extract_loop_scan_v0_facts(&condition, &body)
            .expect("extract ok")
            .expect("facts");

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::ExitAllowed { .. }
        ));
        assert_eq!(facts.segments.len(), 1);
    }

    #[test]
    fn loop_scan_v0_accepts_lte_n_minus_one_shape() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(
            BinaryOperator::LessEqual,
            var("i"),
            binop(BinaryOperator::Subtract, var("n"), int(1)),
        );
        let body = loop_scan_body("i", "ch");

        let facts = try_extract_loop_scan_v0_facts(&condition, &body)
            .expect("extract ok")
            .expect("facts");

        assert_eq!(facts.loop_var, "i");
        assert_eq!(facts.limit_var, "n");
        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::ExitAllowed { .. }
        ));
    }

    #[test]
    fn loop_scan_v0_rejects_lte_n_without_minus_one() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(BinaryOperator::LessEqual, var("i"), var("n"));
        let body = loop_scan_body("i", "ch");

        let facts = try_extract_loop_scan_v0_facts(&condition, &body).expect("extract ok");
        assert!(
            facts.is_none(),
            "loop_scan_v0 must reject `i <= n` (only `i < n` or `i <= n - 1` are accepted)"
        );
    }
}
