//! Exit-path predicates for RecipeBlock dispatch.
//!
//! These helpers classify already-built lowering plans. They must not lower or
//! mutate builder state.

use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};

pub(super) fn plans_exit_on_all_paths(plans: &[LoweredRecipe]) -> bool {
    plans.last().is_some_and(core_plan_exits_on_all_paths)
}

fn core_plan_exits_on_all_paths(plan: &LoweredRecipe) -> bool {
    match plan {
        CorePlan::Exit(_) => true,
        CorePlan::If(if_plan) => {
            plans_exit_on_all_paths(&if_plan.then_plans)
                && if_plan
                    .else_plans
                    .as_ref()
                    .is_some_and(|p| plans_exit_on_all_paths(p))
        }
        CorePlan::BranchN(branch) => {
            branch
                .arms
                .iter()
                .all(|arm| plans_exit_on_all_paths(&arm.plans))
                && branch
                    .else_plans
                    .as_ref()
                    .is_some_and(|p| plans_exit_on_all_paths(p))
        }
        CorePlan::Seq(inner) => plans_exit_on_all_paths(inner),
        CorePlan::Effect(_) | CorePlan::Loop(_) => false,
    }
}
