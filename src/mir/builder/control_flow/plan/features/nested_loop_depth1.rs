//! NestedLoopFeature (depth<=1) for loop(true) normalization.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1_route::dispatch_nested_loop_depth1_any;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn lower_nested_loop_depth1_any(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    error_prefix: &str,
) -> Result<LoweredRecipe, String> {
    let plan = dispatch_nested_loop_depth1_any(builder, condition, body, error_prefix)?;
    Ok(mark_nested_loop_preheader_fresh(builder, plan))
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
