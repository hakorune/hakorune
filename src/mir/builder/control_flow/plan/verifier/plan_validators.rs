//! Phase 29bq+: Plan variant validators
//!
//! # Responsibilities
//!
//! - Validate Seq, If, BranchN, Exit plan variants
//! - Check invariants V2-V5 (Condition validity, Exit validity, Completeness)
//!
//! # Invariants
//!
//! - V2: Condition validity (valid ValueId)
//! - V3: Exit validity (Return in function, Break/Continue in loop)
//! - V4: Seq may be empty (no-op)
//! - V5: If/BranchN completeness (then_plans non-empty)

use super::super::{CoreBranchNPlan, CoreExitPlan, CoreIfPlan, LoweredRecipe};
use super::{position_validators, primitives};

/// V4: Seq may be empty (no-op)
pub(super) fn verify_seq(
    plans: &[LoweredRecipe],
    depth: usize,
    loop_depth: usize,
) -> Result<(), String> {
    if plans.is_empty() {
        return Ok(());
    }

    position_validators::verify_exit_position(plans, depth, "Seq")?;

    for (i, plan) in plans.iter().enumerate() {
        super::core::PlanVerifier::verify_plan(plan, depth + 1, loop_depth)
            .map_err(|e| format!("[Seq[{}]] {}", i, e))?;
    }

    Ok(())
}

/// V5: If completeness
pub(super) fn verify_if(
    if_plan: &CoreIfPlan,
    depth: usize,
    loop_depth: usize,
) -> Result<(), String> {
    // V2: Condition validity
    primitives::verify_value_id_basic(if_plan.condition, depth, "if.condition")?;

    // V5: then/else non-empty unless join-only branch (joins present)
    if if_plan.joins.is_empty() {
        if if_plan.then_plans.is_empty() {
            return Err(primitives::err(
                "V5",
                "if_then_empty",
                format!("If at depth {} has empty then_plans", depth),
            ));
        }

        if let Some(else_plans) = &if_plan.else_plans {
            if else_plans.is_empty() {
                return Err(primitives::err(
                    "V5",
                    "if_else_empty",
                    format!("If at depth {} has empty else_plans", depth),
                ));
            }
        }
    }

    position_validators::verify_branch_plans(
        &if_plan.then_plans,
        depth,
        loop_depth,
        "If.then",
        |p, d, l| super::core::PlanVerifier::verify_plan(p, d, l),
    )?;
    if let Some(else_plans) = &if_plan.else_plans {
        position_validators::verify_branch_plans(
            else_plans,
            depth,
            loop_depth,
            "If.else",
            |p, d, l| super::core::PlanVerifier::verify_plan(p, d, l),
        )?;
    }

    for (idx, join) in if_plan.joins.iter().enumerate() {
        primitives::verify_value_id_basic(join.dst, depth, &format!("if.join[{}].dst", idx))?;
        primitives::verify_value_id_basic(join.then_val, depth, &format!("if.join[{}].then", idx))?;
        primitives::verify_value_id_basic(join.else_val, depth, &format!("if.join[{}].else", idx))?;
    }

    Ok(())
}

/// V5: BranchN completeness (arms >= 2)
pub(super) fn verify_branch_n(
    branch_plan: &CoreBranchNPlan,
    depth: usize,
    loop_depth: usize,
) -> Result<(), String> {
    if branch_plan.arms.len() < 2 {
        return Err(primitives::err(
            "V5",
            "branchn_arms_lt2",
            format!("BranchN at depth {} has < 2 arms", depth),
        ));
    }

    for (i, arm) in branch_plan.arms.iter().enumerate() {
        primitives::verify_value_id_basic(
            arm.condition,
            depth,
            &format!("BranchN.arm[{}].cond", i),
        )?;
        if arm.plans.is_empty() {
            return Err(primitives::err(
                "V5",
                "branchn_arm_empty",
                format!(
                    "BranchN at depth {} has empty arm plans (index {})",
                    depth, i
                ),
            ));
        }
        position_validators::verify_branch_plans(
            &arm.plans,
            depth,
            loop_depth,
            &format!("BranchN.arm[{}]", i),
            |p, d, l| super::core::PlanVerifier::verify_plan(p, d, l),
        )?;
    }

    if let Some(else_plans) = &branch_plan.else_plans {
        if else_plans.is_empty() {
            return Err(primitives::err(
                "V5",
                "branchn_else_empty",
                format!("BranchN at depth {} has empty else_plans", depth),
            ));
        }
        position_validators::verify_branch_plans(
            else_plans,
            depth,
            loop_depth,
            "BranchN.else",
            |p, d, l| super::core::PlanVerifier::verify_plan(p, d, l),
        )?;
    }

    Ok(())
}

/// V3: Exit validity
pub(super) fn verify_exit(
    exit: &CoreExitPlan,
    depth: usize,
    loop_depth: usize,
) -> Result<(), String> {
    match exit {
        CoreExitPlan::Return(opt_val) => {
            if let Some(val) = opt_val {
                primitives::verify_value_id_basic(*val, depth, "Return.value")?;
            }
            // Return is always valid (in function context)
        }
        CoreExitPlan::Break(exit_depth) => {
            if loop_depth == 0 {
                return Err(primitives::err(
                    "V3",
                    "break_outside_loop",
                    format!("Break at depth {} outside of loop", depth),
                ));
            }
            position_validators::verify_exit_depth(*exit_depth, loop_depth, depth)?;
        }
        CoreExitPlan::BreakWithPhiArgs {
            depth: exit_depth,
            phi_args,
        } => {
            if loop_depth == 0 {
                return Err(primitives::err(
                    "V3",
                    "break_outside_loop",
                    format!("BreakWithPhiArgs at depth {} outside of loop", depth),
                ));
            }
            position_validators::verify_exit_depth(*exit_depth, loop_depth, depth)?;
            if phi_args.is_empty() {
                return Err(primitives::err(
                    "V3",
                    "break_phi_args_empty",
                    format!("BreakWithPhiArgs has empty phi_args at depth {}", depth),
                ));
            }
            for (dst, src) in phi_args {
                primitives::verify_value_id_basic(*dst, depth, "BreakWithPhiArgs.phi_dst")?;
                primitives::verify_value_id_basic(*src, depth, "BreakWithPhiArgs.phi_src")?;
            }
        }
        CoreExitPlan::Continue(exit_depth) => {
            if loop_depth == 0 {
                return Err(primitives::err(
                    "V3",
                    "continue_outside_loop",
                    format!("Continue at depth {} outside of loop", depth),
                ));
            }
            position_validators::verify_exit_depth(*exit_depth, loop_depth, depth)?;
        }
        CoreExitPlan::ContinueWithPhiArgs {
            depth: exit_depth,
            phi_args,
        } => {
            if loop_depth == 0 {
                return Err(primitives::err(
                    "V3",
                    "continue_outside_loop",
                    format!("ContinueWithPhiArgs at depth {} outside of loop", depth),
                ));
            }
            position_validators::verify_exit_depth(*exit_depth, loop_depth, depth)?;
            if phi_args.is_empty() {
                return Err(primitives::err(
                    "V3",
                    "continue_phi_args_empty",
                    format!("ContinueWithPhiArgs has empty phi_args at depth {}", depth),
                ));
            }
            for (dst, src) in phi_args {
                primitives::verify_value_id_basic(*dst, depth, "ContinueWithPhiArgs.phi_dst")?;
                primitives::verify_value_id_basic(*src, depth, "ContinueWithPhiArgs.phi_src")?;
            }
        }
    }
    Ok(())
}
