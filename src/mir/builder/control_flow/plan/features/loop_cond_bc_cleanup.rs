//! Route-local cleanup for `LoopCondBreakContinue`.
//!
//! Scope:
//! - detect whether all body paths exit
//! - append the route-local fallthrough continue exit when needed

use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use std::collections::{BTreeMap, BTreeSet};

pub(in crate::mir::builder) struct LoopCondBreakContinueCleanupResult {
    body_exits_all_paths: bool,
}

impl LoopCondBreakContinueCleanupResult {
    pub(in crate::mir::builder) fn body_exits_all_paths(&self) -> bool {
        self.body_exits_all_paths
    }
}

pub(in crate::mir::builder) fn apply_loop_cond_break_continue_cleanup(
    builder: &mut MirBuilder,
    body_plans: &mut Vec<LoweredRecipe>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    current_bindings: &BTreeMap<String, ValueId>,
    body_entry_bindings: &BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<LoopCondBreakContinueCleanupResult, String> {
    let body_exits_all_paths = body_plans_exit_on_all_paths(body_plans);
    if !body_exits_all_paths {
        let body_entry_values: BTreeSet<ValueId> = body_entry_bindings.values().copied().collect();
        let mut body_defined_values = BTreeSet::new();
        collect_defined_values_from_plans(body_plans, &mut body_defined_values);

        let mut fallthrough_bindings = current_bindings.clone();
        for name in carrier_step_phis.keys() {
            let selected = builder
                .variable_ctx
                .variable_map
                .get(name)
                .copied()
                .and_then(|candidate| {
                    if body_defined_values.contains(&candidate)
                        || body_entry_values.contains(&candidate)
                    {
                        Some(candidate)
                    } else {
                        body_entry_bindings.get(name).copied()
                    }
                })
                .or_else(|| body_entry_bindings.get(name).copied());
            if let Some(value_id) = selected {
                fallthrough_bindings.insert(name.clone(), value_id);
            }
        }

        let exit =
            crate::mir::builder::control_flow::plan::parts::exit::build_continue_with_phi_args(
                builder,
                carrier_step_phis,
                &fallthrough_bindings,
                error_prefix,
            )?;
        body_plans.push(CorePlan::Exit(exit));
    }

    Ok(LoopCondBreakContinueCleanupResult {
        body_exits_all_paths,
    })
}

fn body_plans_exit_on_all_paths(plans: &[LoweredRecipe]) -> bool {
    plans.last().is_some_and(plan_exits_on_all_paths)
}

fn plan_exits_on_all_paths(plan: &LoweredRecipe) -> bool {
    match plan {
        CorePlan::Exit(_) => true,
        CorePlan::If(if_plan) => {
            body_plans_exit_on_all_paths(&if_plan.then_plans)
                && if_plan
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| body_plans_exit_on_all_paths(plans))
        }
        CorePlan::BranchN(branch) => {
            branch
                .arms
                .iter()
                .all(|arm| body_plans_exit_on_all_paths(&arm.plans))
                && branch
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| body_plans_exit_on_all_paths(plans))
        }
        CorePlan::Seq(inner) => body_plans_exit_on_all_paths(inner),
        CorePlan::Effect(_) | CorePlan::Loop(_) => false,
    }
}

fn collect_defined_values_from_plans(plans: &[LoweredRecipe], out: &mut BTreeSet<ValueId>) {
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
        | CoreEffectPlan::FieldGet { dst, .. }
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
        CoreEffectPlan::FieldSet { .. } | CoreEffectPlan::ExitIf { .. } => {}
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::control_flow::plan::CoreExitPlan;

    #[test]
    fn cleanup_marks_all_exit_when_tail_is_exit() {
        let mut builder = MirBuilder::new();
        let mut body_plans = vec![CorePlan::Exit(CoreExitPlan::Break(1))];

        let result = apply_loop_cond_break_continue_cleanup(
            &mut builder,
            &mut body_plans,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            "[test] loop_cond_break_continue",
        )
        .expect("cleanup result");

        assert!(result.body_exits_all_paths());
        assert!(matches!(
            body_plans.last(),
            Some(CorePlan::Exit(CoreExitPlan::Break(_)))
        ));
    }

    #[test]
    fn cleanup_appends_continue_for_fallthrough_tail() {
        let mut builder = MirBuilder::new();
        let mut body_plans = vec![CorePlan::Effect(CoreEffectPlan::Copy {
            dst: ValueId(1),
            src: ValueId(2),
        })];
        let carrier_step_phis = BTreeMap::from([("i".to_string(), ValueId(3))]);
        let current_bindings = BTreeMap::from([("i".to_string(), ValueId(4))]);
        let body_entry_bindings = current_bindings.clone();

        let result = apply_loop_cond_break_continue_cleanup(
            &mut builder,
            &mut body_plans,
            &carrier_step_phis,
            &current_bindings,
            &body_entry_bindings,
            "[test] loop_cond_break_continue",
        )
        .expect("cleanup result");

        assert!(!result.body_exits_all_paths());
        assert!(matches!(
            body_plans.last(),
            Some(CorePlan::Exit(CoreExitPlan::ContinueWithPhiArgs { .. }))
        ));
    }
}
