//! Route-local acceptance / fallback dispatch for `nested_loop_depth1`.

use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::builder::control_flow::facts::extractors::common_helpers::{
    count_control_flow, ControlFlowDetector,
};
use crate::mir::builder::control_flow::facts::loop_cond_continue_only::try_extract_loop_cond_continue_only_facts;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::features::loop_true_break_continue_pipeline::lower_loop_true_break_continue_inner;
use crate::mir::builder::control_flow::plan::generic_loop::facts::extract::try_extract_generic_loop_v1_facts;
use crate::mir::builder::control_flow::plan::generic_loop::normalizer::normalize_generic_loop_v1;
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_entry::try_extract_loop_cond_break_continue_facts_for_nested;
use crate::mir::builder::control_flow::plan::loop_true_break_continue::facts::try_extract_loop_true_break_continue_facts;
use crate::mir::builder::control_flow::plan::nested_loop_depth1::try_lower_nested_loop_depth1;
use crate::mir::builder::control_flow::plan::nested_loop_plan::try_compose_loop_cond_continue_with_return_recipe;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::single_planner;
use crate::mir::builder::control_flow::plan::trace as plan_trace;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn dispatch_nested_loop_depth1_any(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    error_prefix: &str,
) -> Result<LoweredRecipe, String> {
    let debug_enabled = crate::config::env::joinir_dev::debug_enabled();
    let is_continue_only_ctx = error_prefix.contains("loop_cond_continue_only");
    let mut detector = ControlFlowDetector::default();
    detector.count_returns = true;
    let counts = count_control_flow(body, detector);
    let no_break_or_continue = counts.break_count == 0 && counts.continue_count == 0;
    let return_only_nested = no_break_or_continue && counts.return_count > 0;

    if no_break_or_continue && !return_only_nested {
        if let Some(plan) = try_lower_generic_loop_v1_nested(builder, condition, body) {
            return Ok(plan);
        }
    }

    let continue_only_ctx_no_break_return =
        is_continue_only_ctx && counts.break_count == 0 && counts.return_count == 0;
    let continue_only_candidate = continue_only_ctx_no_break_return && counts.continue_count > 0;
    if continue_only_candidate {
        if let Ok(Some(facts)) = try_extract_loop_cond_continue_only_facts(condition, body) {
            let ctx = LoopRouteContext::new(condition, body, "<nested>", false, false);
            let plan = PlanNormalizer::normalize_loop_cond_continue_only(builder, facts, &ctx)?;
            return Ok(plan);
        }
        if let Some(plan) = try_lower_nested_loop_depth1(builder, condition, body, error_prefix)? {
            return Ok(plan);
        }
    }

    if let Ok(Some(facts)) = try_extract_loop_true_break_continue_facts(condition, body) {
        if debug_enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[plan/trace:nested_loop_depth1] ctx={} path=loop_true_break_continue rule=loop_true_break_continue",
                error_prefix
            ));
        }
        return lower_loop_true_break_continue_inner(builder, facts);
    }

    let facts = match if continue_only_ctx_no_break_return {
        Ok(None)
    } else {
        try_extract_loop_cond_break_continue_facts_for_nested(condition, body)
    } {
        Ok(Some(facts)) => facts,
        Ok(None) => {
            if !return_only_nested {
                if let Some(plan) = try_lower_generic_loop_v1_nested(builder, condition, body) {
                    return Ok(plan);
                }
            }
            if !nested_loop_allows_single_planner(condition, body) {
                if debug_enabled {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[plan/trace:nested_loop_depth1] ctx={} path=nested_loop_unsupported rule=none",
                        error_prefix
                    ));
                }
                return Err(format!("{error_prefix}: nested loop unsupported"));
            }
            if debug_enabled {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[plan/trace:nested_loop_depth1] ctx={} path=single_planner rule=single_planner",
                    error_prefix
                ));
            }
            return lower_nested_loop_single_planner(builder, condition, body, error_prefix);
        }
        Err(err) => {
            if debug_enabled {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[plan/trace:nested_loop_depth1] ctx={} path=facts_error rule=loop_cond_break_continue",
                    error_prefix
                ));
            }
            return Err(format!("{error_prefix}: nested loop facts error: {err}"));
        }
    };

    let ctx = LoopRouteContext::new(condition, body, "<nested>", false, false);
    if debug_enabled {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[plan/trace:nested_loop_depth1] ctx={} path=loop_cond_break_continue rule=loop_cond_break_continue",
            error_prefix
        ));
    }
    PlanNormalizer::normalize_loop_cond_break_continue(builder, facts, &ctx)
}

fn try_lower_generic_loop_v1_nested(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<LoweredRecipe> {
    let Ok(Some(facts)) = try_extract_generic_loop_v1_facts(condition, body) else {
        return None;
    };
    let ctx = LoopRouteContext::new(condition, body, "<nested>", false, false);
    normalize_generic_loop_v1(builder, &facts, &ctx).ok()
}

fn lower_nested_loop_single_planner(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    error_prefix: &str,
) -> Result<LoweredRecipe, String> {
    let ctx = LoopRouteContext::new(condition, body, "<nested>", false, false);
    let outcome = single_planner::try_build_outcome(&ctx)?;
    plan_trace::trace_outcome_snapshot(
        "nested_loop_depth1::single_planner",
        false,
        outcome.facts.is_some(),
        outcome.recipe_contract.is_some(),
    );
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    if let Some(recipe) = try_compose_loop_cond_continue_with_return_recipe(
        builder,
        &outcome,
        &ctx,
        "nested_loop_depth1::single_planner",
        planner_required,
    )? {
        return Ok(recipe);
    }
    plan_trace::trace_outcome_path("nested_loop_depth1::single_planner", "freeze_no_plan");
    Err(format!("{error_prefix}: nested loop has no plan"))
}

fn nested_loop_allows_single_planner(condition: &ASTNode, body: &[ASTNode]) -> bool {
    let allows_condition = match condition {
        ASTNode::BinaryOp { operator, .. } => {
            matches!(operator, BinaryOperator::Less | BinaryOperator::LessEqual)
        }
        _ => false,
    };
    if !allows_condition || body.is_empty() {
        return false;
    }
    nested_loop_body_is_simple(body)
}

fn nested_loop_body_is_simple(body: &[ASTNode]) -> bool {
    for stmt in body {
        match stmt {
            ASTNode::Local { .. } | ASTNode::Assignment { .. } => {}
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                if !nested_loop_body_is_simple(then_body) {
                    return false;
                }
                if let Some(else_body) = else_body {
                    if !nested_loop_body_is_simple(else_body) {
                        return false;
                    }
                }
            }
            ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. } => {}
            ASTNode::Loop { .. } | ASTNode::While { .. } | ASTNode::ForRange { .. } => {
                return false
            }
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};

    fn span() -> Span {
        Span::unknown()
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: span(),
        }
    }

    fn less_cond() -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(var("i")),
            right: Box::new(int(10)),
            span: span(),
        }
    }

    #[test]
    fn nested_loop_depth1_route_allows_single_planner_for_simple_body() {
        let body = vec![ASTNode::Assignment {
            target: Box::new(var("x")),
            value: Box::new(int(1)),
            span: span(),
        }];
        assert!(nested_loop_allows_single_planner(&less_cond(), &body));
    }

    #[test]
    fn nested_loop_depth1_route_rejects_nested_loops_for_single_planner() {
        let body = vec![ASTNode::Loop {
            condition: Box::new(less_cond()),
            body: vec![],
            span: span(),
        }];
        assert!(!nested_loop_allows_single_planner(&less_cond(), &body));
    }
}
