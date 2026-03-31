use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::ValueId;
use std::collections::BTreeSet;

pub(super) fn collect_defined_values_from_plans(plans: &[LoweredRecipe], out: &mut BTreeSet<ValueId>) {
    for plan in plans {
        match plan {
            CorePlan::Effect(effect) => collect_defined_values_from_effect(effect, out),
            CorePlan::If(if_plan) => {
                let then_has_exit = plans_have_non_local_exit(&if_plan.then_plans);
                let else_has_exit = if_plan
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| plans_have_non_local_exit(plans));
                if !then_has_exit && !else_has_exit {
                    for join in &if_plan.joins {
                        out.insert(join.dst);
                    }
                }
                collect_defined_values_from_plans(&if_plan.then_plans, out);
                if let Some(else_plans) = &if_plan.else_plans {
                    collect_defined_values_from_plans(else_plans, out);
                }
            }
            CorePlan::BranchN(branch) => {
                for arm in &branch.arms {
                    collect_defined_values_from_plans(&arm.plans, out);
                }
                if let Some(else_plans) = &branch.else_plans {
                    collect_defined_values_from_plans(else_plans, out);
                }
            }
            CorePlan::Seq(inner) => collect_defined_values_from_plans(inner, out),
            CorePlan::Loop(loop_plan) => collect_defined_values_from_plans(&loop_plan.body, out),
            CorePlan::Exit(_) => {}
        }
    }
}

fn plans_have_non_local_exit(plans: &[LoweredRecipe]) -> bool {
    plans.iter().any(plan_has_non_local_exit)
}

fn plan_has_non_local_exit(plan: &LoweredRecipe) -> bool {
    match plan {
        CorePlan::Exit(_) => true,
        CorePlan::Effect(effect) => effect_has_non_local_exit(effect),
        CorePlan::If(if_plan) => {
            plans_have_non_local_exit(&if_plan.then_plans)
                || if_plan
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| plans_have_non_local_exit(plans))
        }
        CorePlan::BranchN(branch) => {
            branch
                .arms
                .iter()
                .any(|arm| plans_have_non_local_exit(&arm.plans))
                || branch
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| plans_have_non_local_exit(plans))
        }
        CorePlan::Seq(inner) => plans_have_non_local_exit(inner),
        CorePlan::Loop(loop_plan) => {
            plans_have_non_local_exit(&loop_plan.body)
                || loop_plan
                    .block_effects
                    .iter()
                    .any(|(_, effects)| effects.iter().any(effect_has_non_local_exit))
        }
    }
}

fn effect_has_non_local_exit(effect: &CoreEffectPlan) -> bool {
    match effect {
        CoreEffectPlan::ExitIf { .. } => true,
        CoreEffectPlan::IfEffect {
            then_effects,
            else_effects,
            ..
        } => {
            then_effects.iter().any(effect_has_non_local_exit)
                || else_effects
                    .as_ref()
                    .is_some_and(|effects| effects.iter().any(effect_has_non_local_exit))
        }
        _ => false,
    }
}

fn collect_defined_values_from_effect(effect: &CoreEffectPlan, out: &mut BTreeSet<ValueId>) {
    match effect {
        CoreEffectPlan::MethodCall { dst, .. }
        | CoreEffectPlan::GlobalCall { dst, .. }
        | CoreEffectPlan::ValueCall { dst, .. }
        | CoreEffectPlan::ExternCall { dst, .. } => {
            if let Some(dst) = dst {
                out.insert(*dst);
            }
        }
        CoreEffectPlan::NewBox { dst, .. }
        | CoreEffectPlan::BinOp { dst, .. }
        | CoreEffectPlan::Compare { dst, .. }
        | CoreEffectPlan::Select { dst, .. }
        | CoreEffectPlan::Const { dst, .. }
        | CoreEffectPlan::Copy { dst, .. } => {
            out.insert(*dst);
        }
        CoreEffectPlan::IfEffect {
            then_effects,
            else_effects,
            ..
        } => {
            for nested in then_effects {
                collect_defined_values_from_effect(nested, out);
            }
            if let Some(else_effects) = else_effects {
                for nested in else_effects {
                    collect_defined_values_from_effect(nested, out);
                }
            }
        }
        CoreEffectPlan::ExitIf { .. } => {}
    }
}

