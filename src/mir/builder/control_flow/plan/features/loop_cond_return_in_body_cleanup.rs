//! Route-local cleanup for `LoopCondReturnInBody`.
//!
//! Scope:
//! - body exit analysis
//! - fallthrough continue-exit closure

use crate::mir::builder::control_flow::plan::features::loop_cond_return_in_body_phi_materializer::LoopCondReturnInBodyPhiClosure;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};

pub(in crate::mir::builder) fn body_exits_all_paths(plans: &[LoweredRecipe]) -> bool {
    plans.last().is_some_and(plan_exits_on_all_paths)
}

pub(in crate::mir::builder) fn apply_fallthrough_continue_exit(
    plans: &mut Vec<LoweredRecipe>,
    phi_closure: &LoopCondReturnInBodyPhiClosure,
) {
    if let Some(continue_exit) = phi_closure.continue_exit() {
        plans.push(CorePlan::Exit(continue_exit));
    }
}

fn plan_exits_on_all_paths(plan: &LoweredRecipe) -> bool {
    match plan {
        CorePlan::Exit(_) => true,
        CorePlan::If(if_plan) => {
            body_exits_all_paths(&if_plan.then_plans)
                && if_plan
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| body_exits_all_paths(plans))
        }
        CorePlan::BranchN(branch) => {
            branch
                .arms
                .iter()
                .all(|arm| body_exits_all_paths(&arm.plans))
                && branch
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| body_exits_all_paths(plans))
        }
        CorePlan::Seq(inner) => body_exits_all_paths(inner),
        CorePlan::Effect(_) | CorePlan::Loop(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::control_flow::plan::{
        CoreEffectPlan, CoreExitPlan, CoreIfPlan, CorePhiInfo,
    };
    use crate::mir::{BasicBlockId, ValueId};

    fn sample_phi_closure_with_continue() -> LoopCondReturnInBodyPhiClosure {
        LoopCondReturnInBodyPhiClosure::new(
            Some(CoreExitPlan::ContinueWithPhiArgs {
                depth: 1,
                phi_args: vec![(ValueId(3), ValueId(4))],
            }),
            vec![CorePhiInfo {
                block: BasicBlockId(1),
                dst: ValueId(2),
                inputs: vec![],
                tag: "phi".to_string(),
            }],
            vec![("i".to_string(), ValueId(2))],
            BasicBlockId(12),
        )
    }

    #[test]
    fn cleanup_detects_all_exit_if_tree() {
        let plans = vec![CorePlan::If(CoreIfPlan {
            condition: ValueId(9),
            then_plans: vec![CorePlan::Exit(CoreExitPlan::Return(Some(ValueId(1))))],
            else_plans: Some(vec![CorePlan::Exit(CoreExitPlan::Return(Some(ValueId(2))))]),
            joins: vec![],
        })];

        assert!(body_exits_all_paths(&plans));
    }

    #[test]
    fn cleanup_appends_fallthrough_continue_exit() {
        let mut plans = vec![CorePlan::Effect(CoreEffectPlan::Copy {
            dst: ValueId(1),
            src: ValueId(2),
        })];
        let phi_closure = sample_phi_closure_with_continue();

        apply_fallthrough_continue_exit(&mut plans, &phi_closure);

        assert!(matches!(
            plans.last(),
            Some(CorePlan::Exit(CoreExitPlan::ContinueWithPhiArgs { .. }))
        ));
    }
}
