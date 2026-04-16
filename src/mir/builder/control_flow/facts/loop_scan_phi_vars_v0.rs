//! Facts owner surface for loop_scan_phi_vars_v0.
//!
//! The route remains family-local under `plan/loop_scan_phi_vars_v0`, but the
//! extracted facts contract is owned here so non-`plan/` callers can depend on
//! a top-level facts surface.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::recipes::loop_scan_phi_vars_v0::{
    LoopScanPhiSegment, LoopScanPhiVarsV0Recipe,
};
use crate::mir::builder::control_flow::plan::loop_scan_phi_vars_v0::facts_helpers::{
    build_nested_loop_recipe, contains_exit_outside_nested_loops, is_loop_cond_var_lt_var,
    release_enabled,
};
use crate::mir::builder::control_flow::plan::loop_scan_phi_vars_v0::facts_shape_routes::{
    try_match_loop_scan_phi_vars_ext_shape01, try_match_loop_scan_phi_vars_len7_shape,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::policies::BodyLoweringPolicy;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopScanPhiVarsV0Facts {
    pub loop_var: String,
    pub limit_var: String,
    pub condition: ASTNode,
    pub body_lowering_policy: BodyLoweringPolicy,
    pub recipe: LoopScanPhiVarsV0Recipe,
    pub segments: Vec<LoopScanPhiSegment>,
}

pub(in crate::mir::builder) fn try_extract_loop_scan_phi_vars_v0_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopScanPhiVarsV0Facts>, Freeze> {
    let debug = crate::config::env::joinir_dev::debug_enabled();
    let debug_reject = |reason: &str| {
        if debug {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[plan/reject_detail] box=loop_scan_phi_vars_v0 reason={}",
                reason
            ));
        }
    };

    if !release_enabled() {
        debug_reject("planner_required_off");
        return Ok(None);
    }

    let Some((loop_var, limit_var)) = is_loop_cond_var_lt_var(condition) else {
        debug_reject("cond_not_var_lt_var");
        return Ok(None);
    };

    let shape_match = match body.len() {
        7 => match try_match_loop_scan_phi_vars_len7_shape(body, &loop_var) {
            Ok(shape_match) => shape_match,
            Err(reason) => {
                debug_reject(reason);
                return Ok(None);
            }
        },
        4 => match try_match_loop_scan_phi_vars_ext_shape01(body, &loop_var) {
            Ok(shape_match) => shape_match,
            Err(reason) => {
                debug_reject(reason);
                return Ok(None);
            }
        },
        _ => {
            debug_reject(&format!("body_len={} expected=7_or_4", body.len()));
            return Ok(None);
        }
    };

    if contains_exit_outside_nested_loops(body) {
        debug_reject("exit_outside_nested_loops");
        return Ok(None);
    }

    const ALLOW_EXTENDED: bool = true;

    let Some(prefix_linear) =
        try_build_no_exit_block_recipe(&body[..shape_match.prefix_end], ALLOW_EXTENDED)
    else {
        debug_reject("segments_prefix_not_no_exit");
        return Ok(None);
    };

    let Some(nested_loop_search) = build_nested_loop_recipe(&body[shape_match.nested_idx]) else {
        debug_reject("segments_inner_loop_not_loop");
        return Ok(None);
    };

    let Some(step_linear) =
        try_build_no_exit_block_recipe(&body[shape_match.step_start..], ALLOW_EXTENDED)
    else {
        debug_reject("segments_step_not_no_exit");
        return Ok(None);
    };

    Ok(Some(LoopScanPhiVarsV0Facts {
        loop_var,
        limit_var,
        condition: condition.clone(),
        body_lowering_policy: BodyLoweringPolicy::RecipeOnly,
        recipe: shape_match.recipe,
        segments: vec![
            LoopScanPhiSegment::Linear(prefix_linear),
            LoopScanPhiSegment::NestedLoop(nested_loop_search),
            LoopScanPhiSegment::Linear(step_linear),
        ],
    }))
}

#[cfg(test)]
mod tests {
    use super::try_extract_loop_scan_phi_vars_v0_facts;
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

    fn bool_lit(value: bool) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Bool(value),
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

    fn method_call(object: ASTNode, method: &str, arguments: Vec<ASTNode>) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(object),
            method: method.to_string(),
            arguments,
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
    fn policy_recipe_only_for_loop_scan_phi_vars_v0() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(BinaryOperator::Less, var("i"), var("n"));
        let body = vec![
            local("var_name", Some(int(0))),
            local("j", Some(int(0))),
            local("m", Some(int(1))),
            local("found", Some(int(0))),
            ASTNode::Loop {
                condition: Box::new(binop(BinaryOperator::Less, var("j"), var("m"))),
                body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
            ASTNode::If {
                condition: Box::new(bool_lit(true)),
                then_body: vec![assign(var("found"), int(1))],
                else_body: None,
                span: Span::unknown(),
            },
            assign(var("i"), binop(BinaryOperator::Add, var("i"), int(1))),
        ];

        let facts = try_extract_loop_scan_phi_vars_v0_facts(&condition, &body)
            .expect("extract ok")
            .expect("facts");

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::RecipeOnly
        ));
        assert!(facts.recipe.found_if_stmt.is_some());
    }

    #[test]
    fn accepts_ext_shape01_body_len4_nested_no_exit_nonconst_var_step() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(BinaryOperator::Less, var("i"), var("n"));
        let body = vec![
            local("j", Some(int(0))),
            local("m", Some(method_call(var("arr"), "length", vec![]))),
            ASTNode::Loop {
                condition: Box::new(binop(BinaryOperator::Less, var("j"), var("m"))),
                body: vec![assign(
                    var("j"),
                    binop(BinaryOperator::Add, var("j"), int(1)),
                )],
                span: Span::unknown(),
            },
            assign(
                var("i"),
                binop(
                    BinaryOperator::Add,
                    var("i"),
                    method_call(var("arr"), "get", vec![int(0)]),
                ),
            ),
        ];

        let facts = try_extract_loop_scan_phi_vars_v0_facts(&condition, &body)
            .expect("extract ok")
            .expect("facts");

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::RecipeOnly
        ));
        assert_eq!(facts.segments.len(), 3);
        assert!(facts.recipe.found_if_stmt.is_none());
    }

    #[test]
    fn rejects_ext_shape01_when_nested_loop_contains_exit() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(BinaryOperator::Less, var("i"), var("n"));
        let body = vec![
            local("j", Some(int(0))),
            local("m", Some(method_call(var("arr"), "length", vec![]))),
            ASTNode::Loop {
                condition: Box::new(binop(BinaryOperator::Less, var("j"), var("m"))),
                body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
            assign(
                var("i"),
                binop(
                    BinaryOperator::Add,
                    var("i"),
                    method_call(var("arr"), "get", vec![int(0)]),
                ),
            ),
        ];

        let facts = try_extract_loop_scan_phi_vars_v0_facts(&condition, &body).expect("extract ok");
        assert!(facts.is_none());
    }

    #[test]
    fn rejects_ext_shape01_when_step_is_const_plus_one() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(BinaryOperator::Less, var("i"), var("n"));
        let body = vec![
            local("j", Some(int(0))),
            local("m", Some(method_call(var("arr"), "length", vec![]))),
            ASTNode::Loop {
                condition: Box::new(binop(BinaryOperator::Less, var("j"), var("m"))),
                body: vec![assign(
                    var("j"),
                    binop(BinaryOperator::Add, var("j"), int(1)),
                )],
                span: Span::unknown(),
            },
            assign(var("i"), binop(BinaryOperator::Add, var("i"), int(1))),
        ];

        let facts = try_extract_loop_scan_phi_vars_v0_facts(&condition, &body).expect("extract ok");
        assert!(facts.is_none());
    }
}
