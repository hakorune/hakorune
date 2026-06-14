//! Exit-path predicates for RecipeBlock dispatch.
//!
//! These helpers classify already-built lowering plans. They must not lower or
//! mutate builder state.

use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};

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
        CorePlan::Effect(effect) => effect_exits_on_all_paths(effect),
        CorePlan::Loop(_) => false,
    }
}

fn effect_exits_on_all_paths(effect: &CoreEffectPlan) -> bool {
    match effect {
        CoreEffectPlan::IfEffect {
            then_effects,
            else_effects,
            ..
        } => else_effects.as_ref().is_some_and(|else_effects| {
            effects_exit_on_all_paths(then_effects) && effects_exit_on_all_paths(else_effects)
        }),
        CoreEffectPlan::ExitIf { .. } => false,
        _ => false,
    }
}

fn effects_exit_on_all_paths(effects: &[CoreEffectPlan]) -> bool {
    effects.last().is_some_and(effect_item_exits_on_all_paths)
}

fn effect_item_exits_on_all_paths(effect: &CoreEffectPlan) -> bool {
    match effect {
        CoreEffectPlan::ExitIf { .. } => true,
        CoreEffectPlan::IfEffect {
            then_effects,
            else_effects,
            ..
        } => else_effects.as_ref().is_some_and(|else_effects| {
            effects_exit_on_all_paths(then_effects) && effects_exit_on_all_paths(else_effects)
        }),
        _ => false,
    }
}
