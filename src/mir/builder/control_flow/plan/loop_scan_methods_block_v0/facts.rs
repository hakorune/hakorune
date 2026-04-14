//! Facts for loop_scan_methods_block_v0 (one-shape, planner-required only).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
use crate::mir::policies::BodyLoweringPolicy;

use super::facts_helpers::{
    declares_local_var, extract_next_i_from_tail, flatten_stmt_list, is_loop_cond_i_lt_n,
    match_next_i_guard, match_scan_window_block, release_enabled, scan_window_substring_receiver,
    try_segmentize_stmt_list,
};
use super::facts_types::LoopScanMethodsBlockV0Facts;
use super::recipe::LoopScanMethodsBlockV0Recipe;

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
