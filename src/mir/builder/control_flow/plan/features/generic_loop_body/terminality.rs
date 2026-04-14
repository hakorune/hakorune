//! Route-local terminality / continue-edge classification for `GenericLoopV1`.
//!
//! Scope:
//! - detect whether a lowered body exits on all paths
//! - detect whether a lowered body still needs a continue edge

use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePlan, LoweredRecipe};

pub(in crate::mir::builder) fn body_plans_exit_on_all_paths(plans: &[LoweredRecipe]) -> bool {
    plans.last().is_some_and(plan_exits_on_all_paths)
}

pub(in crate::mir::builder) fn plans_require_continue_edge(plans: &[LoweredRecipe]) -> bool {
    plans.iter().any(plan_requires_continue_edge)
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

fn plan_requires_continue_edge(plan: &LoweredRecipe) -> bool {
    match plan {
        CorePlan::Exit(CoreExitPlan::ContinueWithPhiArgs { .. } | CoreExitPlan::Continue(_)) => {
            true
        }
        CorePlan::If(if_plan) => {
            plans_require_continue_edge(&if_plan.then_plans)
                || if_plan
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| plans_require_continue_edge(plans))
        }
        CorePlan::BranchN(branch) => {
            branch
                .arms
                .iter()
                .any(|arm| plans_require_continue_edge(&arm.plans))
                || branch
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| plans_require_continue_edge(plans))
        }
        CorePlan::Seq(inner) => plans_require_continue_edge(inner),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::control_flow::plan::CoreEffectPlan;
    use crate::mir::{ConstValue, ValueId};

    #[test]
    fn generic_loop_body_term_accepts_tail_exit() {
        let plans = vec![CorePlan::Exit(CoreExitPlan::Break(1))];
        assert!(body_plans_exit_on_all_paths(&plans));
    }

    #[test]
    fn generic_loop_body_term_rejects_tail_effect() {
        let plans = vec![CorePlan::Effect(CoreEffectPlan::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(1),
        })];
        assert!(!body_plans_exit_on_all_paths(&plans));
    }

    #[test]
    fn generic_loop_body_term_detects_nested_continue_edge() {
        let plans = vec![CorePlan::Seq(vec![CorePlan::Exit(
            CoreExitPlan::ContinueWithPhiArgs {
                depth: 1,
                phi_args: vec![(ValueId(2), ValueId(3))],
            },
        )])];
        assert!(plans_require_continue_edge(&plans));
    }
}
