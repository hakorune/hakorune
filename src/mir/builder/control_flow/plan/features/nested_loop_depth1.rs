//! NestedLoopFeature (depth<=1) for loop(true) normalization.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    count_control_flow, ControlFlowDetector,
};
use crate::mir::builder::control_flow::plan::features::loop_true_break_continue_pipeline::lower_loop_true_break_continue_inner;
use crate::mir::builder::control_flow::plan::generic_loop::facts::extract::try_extract_generic_loop_v1_facts;
use crate::mir::builder::control_flow::plan::generic_loop::normalizer::normalize_generic_loop_v1;
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_entry::try_extract_loop_cond_break_continue_facts_for_nested;
use crate::mir::builder::control_flow::plan::loop_cond::continue_only_facts::try_extract_loop_cond_continue_only_facts;
use crate::mir::builder::control_flow::plan::loop_true_break_continue::facts::try_extract_loop_true_break_continue_facts;
use crate::mir::builder::control_flow::plan::nested_loop_depth1::try_lower_nested_loop_depth1;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::single_planner;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn lower_nested_loop_depth1_any(
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
    if no_break_or_continue {
        if let Ok(Some(facts)) = try_extract_generic_loop_v1_facts(condition, body) {
            let ctx = LoopPatternContext::new(condition, body, "<nested>", false, false);
            let plan = normalize_generic_loop_v1(builder, &facts, &ctx)?;
            return Ok(mark_nested_loop_preheader_fresh(builder, plan));
        }
    }
    let continue_only_ctx_no_break_return =
        is_continue_only_ctx && counts.break_count == 0 && counts.return_count == 0;
    let continue_only_candidate = continue_only_ctx_no_break_return && counts.continue_count > 0;
    if continue_only_candidate {
        if let Ok(Some(facts)) = try_extract_loop_cond_continue_only_facts(condition, body) {
            let ctx = LoopPatternContext::new(condition, body, "<nested>", false, false);
            let plan = PlanNormalizer::normalize_loop_cond_continue_only(builder, facts, &ctx)?;
            return Ok(mark_nested_loop_preheader_fresh(builder, plan));
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
        let plan = lower_loop_true_break_continue_inner(builder, facts)?;
        return Ok(mark_nested_loop_preheader_fresh(builder, plan));
    }
    let facts = match if continue_only_ctx_no_break_return {
        Ok(None)
    } else {
        try_extract_loop_cond_break_continue_facts_for_nested(condition, body)
    } {
        Ok(Some(facts)) => facts,
        Ok(None) => {
            if let Ok(Some(facts)) = try_extract_generic_loop_v1_facts(condition, body) {
                let ctx = LoopPatternContext::new(condition, body, "<nested>", false, false);
                let plan = normalize_generic_loop_v1(builder, &facts, &ctx)?;
                return Ok(mark_nested_loop_preheader_fresh(builder, plan));
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
            let plan = lower_nested_loop_single_planner(builder, condition, body, error_prefix)?;
            return Ok(mark_nested_loop_preheader_fresh(builder, plan));
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
    let ctx = LoopPatternContext::new(condition, body, "<nested>", false, false);
    if debug_enabled {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[plan/trace:nested_loop_depth1] ctx={} path=loop_cond_break_continue rule=loop_cond_break_continue",
            error_prefix
        ));
    }
    let plan = PlanNormalizer::normalize_loop_cond_break_continue(builder, facts, &ctx)?;
    Ok(mark_nested_loop_preheader_fresh(builder, plan))
}

fn lower_nested_loop_single_planner(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    error_prefix: &str,
) -> Result<LoweredRecipe, String> {
    let ctx = LoopPatternContext::new(condition, body, "<nested>", false, false);
    let (domain_plan, _outcome) = single_planner::try_build_domain_plan_with_outcome(&ctx)?;
    let Some(domain_plan) = domain_plan else {
        return Err(format!("{error_prefix}: nested loop has no plan"));
    };
    PlanNormalizer::normalize(builder, domain_plan, &ctx)
}

pub(in crate::mir::builder) fn mark_nested_loop_preheader_fresh(
    builder: &mut MirBuilder,
    plan: LoweredRecipe,
) -> LoweredRecipe {
    match plan {
        CorePlan::Loop(mut loop_plan) => {
            let old_preheader = loop_plan.preheader_bb;
            let new_preheader = builder.next_block_id();
            loop_plan.preheader_bb = new_preheader;
            loop_plan.preheader_is_fresh = true;
            for (block_id, _) in loop_plan.block_effects.iter_mut() {
                if *block_id == old_preheader {
                    *block_id = new_preheader;
                }
            }
            for phi in loop_plan.phis.iter_mut() {
                for (pred, _) in phi.inputs.iter_mut() {
                    if *pred == old_preheader {
                        *pred = new_preheader;
                    }
                }
            }
            CorePlan::Loop(loop_plan)
        }
        CorePlan::Seq(plans) => {
            let plans = plans
                .into_iter()
                .map(|plan| mark_nested_loop_preheader_fresh(builder, plan))
                .collect();
            CorePlan::Seq(plans)
        }
        CorePlan::If(mut if_plan) => {
            if_plan.then_plans = if_plan
                .then_plans
                .into_iter()
                .map(|plan| mark_nested_loop_preheader_fresh(builder, plan))
                .collect();
            if_plan.else_plans = if_plan.else_plans.map(|plans| {
                plans
                    .into_iter()
                    .map(|plan| mark_nested_loop_preheader_fresh(builder, plan))
                    .collect()
            });
            CorePlan::If(if_plan)
        }
        CorePlan::BranchN(mut branch_plan) => {
            for arm in branch_plan.arms.iter_mut() {
                arm.plans = arm
                    .plans
                    .drain(..)
                    .map(|plan| mark_nested_loop_preheader_fresh(builder, plan))
                    .collect();
            }
            if let Some(else_plans) = branch_plan.else_plans.as_mut() {
                *else_plans = else_plans
                    .drain(..)
                    .map(|plan| mark_nested_loop_preheader_fresh(builder, plan))
                    .collect();
            }
            CorePlan::BranchN(branch_plan)
        }
        other => other,
    }
}

fn nested_loop_allows_single_planner(condition: &ASTNode, body: &[ASTNode]) -> bool {
    use crate::ast::BinaryOperator;

    let allows_condition = match condition {
        ASTNode::BinaryOp { operator, .. } => {
            matches!(operator, BinaryOperator::Less | BinaryOperator::LessEqual)
        }
        _ => false,
    };
    if !allows_condition {
        return false;
    }

    if body.is_empty() {
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
            ASTNode::Break { .. } | ASTNode::Continue { .. } => {}
            ASTNode::Return { .. }
            | ASTNode::Loop { .. }
            | ASTNode::While { .. }
            | ASTNode::ForRange { .. } => return false,
            _ => return false,
        }
    }
    true
}
