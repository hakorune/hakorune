//! Route-local preheader freshness rewrite for `nested_loop_depth1`.
//!
//! Scope:
//! - apply fresh nested-loop preheaders after route/normalizer dispatch picked a plan
//! - keep recursive block-id/phi remap out of `nested_loop_depth1` entrypoints

use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn apply_nested_loop_preheader_freshness(
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
                .map(|plan| apply_nested_loop_preheader_freshness(builder, plan))
                .collect();
            CorePlan::Seq(plans)
        }
        CorePlan::If(mut if_plan) => {
            if_plan.then_plans = if_plan
                .then_plans
                .into_iter()
                .map(|plan| apply_nested_loop_preheader_freshness(builder, plan))
                .collect();
            if_plan.else_plans = if_plan.else_plans.map(|plans| {
                plans
                    .into_iter()
                    .map(|plan| apply_nested_loop_preheader_freshness(builder, plan))
                    .collect()
            });
            CorePlan::If(if_plan)
        }
        CorePlan::BranchN(mut branch_plan) => {
            for arm in branch_plan.arms.iter_mut() {
                arm.plans = arm
                    .plans
                    .drain(..)
                    .map(|plan| apply_nested_loop_preheader_freshness(builder, plan))
                    .collect();
            }
            if let Some(else_plans) = branch_plan.else_plans.as_mut() {
                *else_plans = else_plans
                    .drain(..)
                    .map(|plan| apply_nested_loop_preheader_freshness(builder, plan))
                    .collect();
            }
            CorePlan::BranchN(branch_plan)
        }
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::control_flow::edgecfg::api::Frag;
    use crate::mir::builder::control_flow::plan::{
        step_mode::extract_to_step_bb_explicit_step, CoreEffectPlan, CoreIfPlan, CoreLoopPlan,
        CorePhiInfo,
    };
    use crate::mir::{BasicBlockId, ConstValue, ValueId};

    fn dummy_phi(pred: BasicBlockId, dst: u32) -> CorePhiInfo {
        CorePhiInfo {
            block: BasicBlockId(1),
            dst: ValueId(dst),
            inputs: vec![(pred, ValueId(dst + 10))],
            tag: format!("phi_{dst}"),
        }
    }

    fn make_loop_plan(preheader_bb: BasicBlockId) -> CoreLoopPlan {
        let header_bb = BasicBlockId(preheader_bb.0 + 1);
        let body_bb = BasicBlockId(preheader_bb.0 + 2);
        let step_bb = BasicBlockId(preheader_bb.0 + 3);
        let after_bb = BasicBlockId(preheader_bb.0 + 4);
        let (step_mode, has_explicit_step) = extract_to_step_bb_explicit_step();

        CoreLoopPlan {
            preheader_bb,
            preheader_is_fresh: false,
            header_bb,
            body_bb,
            step_bb,
            continue_target: step_bb,
            after_bb,
            found_bb: after_bb,
            body: vec![CorePlan::Effect(CoreEffectPlan::Const {
                dst: ValueId(50),
                value: ConstValue::Integer(1),
            })],
            cond_loop: ValueId(100),
            cond_match: ValueId(101),
            block_effects: vec![
                (preheader_bb, vec![]),
                (header_bb, vec![]),
                (body_bb, vec![]),
                (step_bb, vec![]),
            ],
            phis: vec![dummy_phi(preheader_bb, 200)],
            frag: Frag::new(header_bb),
            final_values: vec![("i".to_string(), ValueId(200))],
            step_mode,
            has_explicit_step,
        }
    }

    #[test]
    fn nested_loop_depth1_preheader_rewrites_loop_preheader_and_phi_inputs() {
        let mut builder = MirBuilder::new();
        let old_preheader = BasicBlockId(10);
        let plan = CorePlan::Loop(make_loop_plan(old_preheader));

        let CorePlan::Loop(loop_plan) = apply_nested_loop_preheader_freshness(&mut builder, plan)
        else {
            panic!("expected loop plan");
        };

        assert_ne!(loop_plan.preheader_bb, old_preheader);
        assert!(loop_plan.preheader_is_fresh);
        assert_eq!(loop_plan.block_effects[0].0, loop_plan.preheader_bb);
        assert_eq!(loop_plan.phis[0].inputs[0].0, loop_plan.preheader_bb);
    }

    #[test]
    fn nested_loop_depth1_preheader_rewrites_nested_if_children() {
        let mut builder = MirBuilder::new();
        let inner_loop = CorePlan::Loop(make_loop_plan(BasicBlockId(20)));
        let plan = CorePlan::If(CoreIfPlan {
            condition: ValueId(1),
            then_plans: vec![inner_loop],
            else_plans: None,
            joins: vec![],
        });

        let CorePlan::If(if_plan) = apply_nested_loop_preheader_freshness(&mut builder, plan)
        else {
            panic!("expected if plan");
        };
        let CorePlan::Loop(loop_plan) = &if_plan.then_plans[0] else {
            panic!("expected nested loop plan");
        };

        assert!(loop_plan.preheader_is_fresh);
        assert_ne!(loop_plan.preheader_bb, BasicBlockId(20));
        assert_eq!(loop_plan.block_effects[0].0, loop_plan.preheader_bb);
    }
}
