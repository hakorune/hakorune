//! Phase 29bq+: Loop body tree validation logic
//!
//! # Responsibilities
//! - Verify loop body structure (Effect-only tree)
//! - Enforce V12 invariant (loop body cannot contain If/BranchN/Exit plans)
//! - Validate nested structures within loop bodies
//!
//! # Invariants
//! - V12: Loop.body must be Effect-only (Seq-of-effects allowed)
//!   - Allowed: Effect, Seq, Loop (nested loops)
//!   - Forbidden: If, BranchN, Exit (control flow exits)
//!   - Exception: ExitIf within IfEffect is allowed (leaf-level exit)

use super::{effect_validators, plan_validators, position_validators, primitives};
use crate::mir::builder::control_flow::lower::{CorePlan, LoweredRecipe};

pub(super) fn verify_loop_body_tree(
    plans: &[LoweredRecipe],
    depth: usize,
    loop_depth: usize,
) -> Result<(), String> {
    for (i, plan) in plans.iter().enumerate() {
        let path = format!("Loop.body[{}]", i);
        verify_body_plan_tree(plan, depth, loop_depth, &path)?;
    }
    position_validators::verify_exit_if_position(plans, depth, "Loop.body")?;
    Ok(())
}

fn verify_body_plan_tree(
    plan: &LoweredRecipe,
    depth: usize,
    loop_depth: usize,
    path: &str,
) -> Result<(), String> {
    match plan {
        CorePlan::Effect(effect) => effect_validators::verify_effect(effect, depth, loop_depth),
        CorePlan::Seq(plans) => {
            position_validators::verify_exit_position(plans, depth, "Loop.body.Seq")?;
            for (i, nested) in plans.iter().enumerate() {
                let nested_path = format!("{}.Seq[{}]", path, i);
                verify_body_plan_tree(nested, depth + 1, loop_depth, &nested_path)?;
            }
            Ok(())
        }
        CorePlan::If(if_plan) => plan_validators::verify_if(if_plan, depth + 1, loop_depth),
        CorePlan::Loop(loop_plan) => {
            super::loop_validators::verify_loop(loop_plan, depth + 1, loop_depth)
        }
        CorePlan::Exit(exit) => plan_validators::verify_exit(exit, depth, loop_depth),
        CorePlan::BranchN(_) => Err(primitives::err(
            "V12",
            "loop_body_branchn",
            format!("Loop body contains BranchN at depth {} ({})", depth, path),
        )),
    }
}
