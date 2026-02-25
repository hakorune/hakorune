//! Phase 273 P3: PlanVerifier - CorePlan 不変条件検証 (fail-fast)
//!
//! # Responsibilities
//!
//! - Validate CorePlan invariants before lowering
//! - Fail fast on close-but-unsupported patterns
//! - Prevent silent miscompilation
//!
//! # Invariants (V2-V9)
//!
//! - V2: Condition validity (valid ValueId)
//! - V3: Exit validity (Return in function, Break/Continue in loop)
//! - V4: Seq non-empty
//! - V5: If completeness (then_plans non-empty)
//! - V6: ValueId validity (all ValueIds pre-generated)
//! - V7: PHI non-empty (loops require at least one carrier)
//! - V8: Frag entry matches header_bb
//! - V9: block_effects contains header_bb
//! - V10: body_bb effects go in loop_plan.body (block_effects[body_bb] must be empty)
//! - V11: Exit must be last in Seq/If branch (ExitMap alignment)
//! - V12: Loop.body must be Effect-only (Seq-of-effects allowed)
//! - Error format: "[Vx][reason=...]" with stable reason codes for diagnostics
//!
//! Phase 273 P3: V1 (Carrier completeness) removed with CoreCarrierInfo

use super::{
    CoreBranchNPlan, CoreEffectPlan, CoreExitPlan, CoreIfPlan, CoreLoopPlan, CorePlan,
};
use super::coreloop_body_contract::is_leaf_effect_plan;
use crate::mir::basic_block::EdgeArgs;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::ValueId;

/// Phase 273 P1: PlanVerifier - CorePlan 不変条件検証 (fail-fast)
pub(in crate::mir::builder) struct PlanVerifier;

impl PlanVerifier {
    /// Verify CorePlan invariants
    ///
    /// Returns Ok(()) if all invariants hold, Err with details otherwise.
    pub(in crate::mir::builder) fn verify(plan: &CorePlan) -> Result<(), String> {
        Self::verify_plan(plan, 0, 0)
    }

    fn verify_plan(plan: &CorePlan, depth: usize, loop_depth: usize) -> Result<(), String> {
        match plan {
            CorePlan::Seq(plans) => Self::verify_seq(plans, depth, loop_depth),
            CorePlan::Loop(loop_plan) => Self::verify_loop(loop_plan, depth, loop_depth),
            CorePlan::If(if_plan) => Self::verify_if(if_plan, depth, loop_depth),
            CorePlan::BranchN(branch_plan) => Self::verify_branch_n(branch_plan, depth, loop_depth),
            CorePlan::Effect(effect) => Self::verify_effect(effect, depth, loop_depth),
            CorePlan::Exit(exit) => Self::verify_exit(exit, depth, loop_depth),
        }
    }

    /// V4: Seq non-empty
    fn verify_seq(plans: &[CorePlan], depth: usize, loop_depth: usize) -> Result<(), String> {
        if plans.is_empty() {
            return Err(Self::err(
                "V4",
                "seq_empty",
                format!(
                    "Empty Seq at depth {} (Seq must have at least one plan)",
                    depth
                ),
            ));
        }

        Self::verify_exit_position(plans, depth, "Seq")?;

        for (i, plan) in plans.iter().enumerate() {
            Self::verify_plan(plan, depth + 1, loop_depth).map_err(|e| {
                format!("[Seq[{}]] {}", i, e)
            })?;
        }

        Ok(())
    }

    /// Phase 273 P3: Verify loop with generalized fields
    ///
    /// Invariants:
    /// - V2: Condition validity (cond_loop, cond_match)
    /// - V7: PHI non-empty (at least one carrier)
    /// - V8: Frag entry matches header_bb
    /// - V9: block_effects contains header_bb
    fn verify_loop(loop_plan: &CoreLoopPlan, depth: usize, loop_depth: usize) -> Result<(), String> {
        // V2: Condition validity (basic check - ValueId should be non-zero for safety)
        Self::verify_value_id_basic(loop_plan.cond_loop, depth, "cond_loop")?;
        Self::verify_value_id_basic(loop_plan.cond_match, depth, "cond_match")?;

        // V7: PHI non-empty (loops must have at least one carrier)
        if loop_plan.phis.is_empty() {
            return Err(Self::err(
                "V7",
                "loop_phi_empty",
                format!(
                    "Loop at depth {} has no PHI nodes (loops require at least one carrier)",
                    depth
                ),
            ));
        }

        // V8: Frag entry matches header_bb (loop entry SSOT)
        if loop_plan.frag.entry != loop_plan.header_bb {
            return Err(Self::err(
                "V8",
                "loop_frag_entry_mismatch",
                format!(
                    "Loop at depth {} has frag.entry {:?} != header_bb {:?}",
                    depth, loop_plan.frag.entry, loop_plan.header_bb
                ),
            ));
        }

        // V9: block_effects contains header_bb
        let has_header = loop_plan.block_effects.iter().any(|(bb, _)| *bb == loop_plan.header_bb);
        if !has_header {
            return Err(Self::err(
                "V9",
                "loop_header_missing",
                format!(
                    "Loop at depth {} block_effects missing header_bb {:?}",
                    depth, loop_plan.header_bb
                ),
            ));
        }

        // V10: body_bb effects must be empty in block_effects (use loop_plan.body instead)
        // Phase 286 P2.7: lowerer emits loop_plan.body for body_bb, ignoring block_effects
        for (bb, effects) in loop_plan.block_effects.iter() {
            if *bb == loop_plan.body_bb && !effects.is_empty() {
                return Err(Self::err(
                    "V10",
                    "loop_body_bb_block_effects",
                    format!(
                        "Loop at depth {} has non-empty block_effects for body_bb {:?} ({} effects). \
                    Body effects must go in loop_plan.body instead.",
                        depth,
                        loop_plan.body_bb,
                        effects.len()
                    ),
                ));
            }
        }

        // Verify block_effects
        for (i, (bb, effects)) in loop_plan.block_effects.iter().enumerate() {
            for (j, effect) in effects.iter().enumerate() {
                Self::verify_effect(effect, depth, 0).map_err(|e| {
                    format!("[Loop.block_effects[{}={:?}][{}]] {}", i, bb, j, e)
                })?;
            }
        }

        // Verify body plans (loop_depth + 1)
        Self::verify_loop_body_tree(&loop_plan.body, depth, loop_depth + 1)?;

        // Verify PHIs
        for (i, phi) in loop_plan.phis.iter().enumerate() {
            Self::verify_value_id_basic(phi.dst, depth, &format!("phi[{}].dst", i))?;
            for (j, (_, val)) in phi.inputs.iter().enumerate() {
                Self::verify_value_id_basic(*val, depth, &format!("phi[{}].inputs[{}]", i, j))?;
            }
        }

        // Verify final_values
        for (i, (name, val)) in loop_plan.final_values.iter().enumerate() {
            if name.is_empty() {
                return Err(Self::err(
                    "V6",
                    "final_value_empty_name",
                    format!(
                        "final_values[{}] at depth {} has empty name",
                        i, depth
                    ),
                ));
            }
            Self::verify_value_id_basic(*val, depth, &format!("final_values[{}]", i))?;
        }

        // Verify EdgeArgs layout (V13)
        for (i, wire) in loop_plan.frag.wires.iter().enumerate() {
            Self::verify_edge_args_layout(&wire.args, depth, &format!("frag.wires[{}]", i))?;
        }
        for (kind, stubs) in loop_plan.frag.exits.iter() {
            for (i, stub) in stubs.iter().enumerate() {
                Self::verify_edge_args_layout(
                    &stub.args,
                    depth,
                    &format!("frag.exits[{}][{}]", kind, i),
                )?;
            }
        }
        for (i, branch) in loop_plan.frag.branches.iter().enumerate() {
            Self::verify_edge_args_layout(
                &branch.then_args,
                depth,
                &format!("frag.branches[{}].then", i),
            )?;
            Self::verify_edge_args_layout(
                &branch.else_args,
                depth,
                &format!("frag.branches[{}].else", i),
            )?;
        }

        Ok(())
    }

    // Phase 273 P3: verify_carrier() removed (CoreCarrierInfo replaced by CorePhiInfo)

    /// V5: If completeness
    fn verify_if(if_plan: &CoreIfPlan, depth: usize, loop_depth: usize) -> Result<(), String> {
        // V2: Condition validity
        Self::verify_value_id_basic(if_plan.condition, depth, "if.condition")?;

        // V5: then_plans non-empty
        if if_plan.then_plans.is_empty() {
            return Err(Self::err(
                "V5",
                "if_then_empty",
                format!("If at depth {} has empty then_plans", depth),
            ));
        }

        if let Some(else_plans) = &if_plan.else_plans {
            if else_plans.is_empty() {
                return Err(Self::err(
                    "V5",
                    "if_else_empty",
                    format!("If at depth {} has empty else_plans", depth),
                ));
            }
        }

        Self::verify_branch_plans(&if_plan.then_plans, depth, loop_depth, "If.then")?;
        if let Some(else_plans) = &if_plan.else_plans {
            Self::verify_branch_plans(else_plans, depth, loop_depth, "If.else")?;
        }

        Ok(())
    }

    /// V5: BranchN completeness (arms >= 2)
    fn verify_branch_n(
        branch_plan: &CoreBranchNPlan,
        depth: usize,
        loop_depth: usize,
    ) -> Result<(), String> {
        if branch_plan.arms.len() < 2 {
            return Err(Self::err(
                "V5",
                "branchn_arms_lt2",
                format!("BranchN at depth {} has < 2 arms", depth),
            ));
        }

        for (i, arm) in branch_plan.arms.iter().enumerate() {
            Self::verify_value_id_basic(arm.condition, depth, &format!("BranchN.arm[{}].cond", i))?;
            if arm.plans.is_empty() {
                return Err(Self::err(
                    "V5",
                    "branchn_arm_empty",
                    format!(
                        "BranchN at depth {} has empty arm plans (index {})",
                        depth, i
                    ),
                ));
            }
            Self::verify_branch_plans(
                &arm.plans,
                depth,
                loop_depth,
                &format!("BranchN.arm[{}]", i),
            )?;
        }

        if let Some(else_plans) = &branch_plan.else_plans {
            if else_plans.is_empty() {
                return Err(Self::err(
                    "V5",
                    "branchn_else_empty",
                    format!("BranchN at depth {} has empty else_plans", depth),
                ));
            }
            Self::verify_branch_plans(else_plans, depth, loop_depth, "BranchN.else")?;
        }

        Ok(())
    }

    /// V6: Effect ValueId validity
    fn verify_effect(effect: &CoreEffectPlan, depth: usize, loop_depth: usize) -> Result<(), String> {
        match effect {
            CoreEffectPlan::MethodCall { dst, object, method, args, effects: _ } => {
                // P2: dst is now Option<ValueId>
                if let Some(dst_val) = dst {
                    Self::verify_value_id_basic(*dst_val, depth, "MethodCall.dst")?;
                }
                Self::verify_value_id_basic(*object, depth, "MethodCall.object")?;
                if method.is_empty() {
                    return Err(Self::err(
                        "V6",
                        "method_call_empty_name",
                        format!("MethodCall at depth {} has empty method name", depth),
                    ));
                }
                for (i, arg) in args.iter().enumerate() {
                    Self::verify_value_id_basic(*arg, depth, &format!("MethodCall.args[{}]", i))?;
                }
            }
            CoreEffectPlan::GlobalCall { dst, func, args } => {
                if let Some(dst_val) = dst {
                    Self::verify_value_id_basic(*dst_val, depth, "GlobalCall.dst")?;
                }
                if func.is_empty() {
                    return Err(Self::err(
                        "V6",
                        "global_call_empty_func",
                        format!("GlobalCall at depth {} has empty func", depth),
                    ));
                }
                for (i, arg) in args.iter().enumerate() {
                    Self::verify_value_id_basic(*arg, depth, &format!("GlobalCall.args[{}]", i))?;
                }
            }
            CoreEffectPlan::ExternCall {
                dst,
                iface_name,
                method_name,
                args,
                effects: _,
            } => {
                if let Some(dst_val) = dst {
                    Self::verify_value_id_basic(*dst_val, depth, "ExternCall.dst")?;
                }
                if iface_name.is_empty() || method_name.is_empty() {
                    return Err(Self::err(
                        "V6",
                        "extern_call_empty_name",
                        format!(
                            "ExternCall at depth {} has empty iface/method",
                            depth
                        ),
                    ));
                }
                for (i, arg) in args.iter().enumerate() {
                    Self::verify_value_id_basic(*arg, depth, &format!("ExternCall.args[{}]", i))?;
                }
            }
            CoreEffectPlan::BinOp { dst, lhs, op: _, rhs } => {
                Self::verify_value_id_basic(*dst, depth, "BinOp.dst")?;
                Self::verify_value_id_basic(*lhs, depth, "BinOp.lhs")?;
                Self::verify_value_id_basic(*rhs, depth, "BinOp.rhs")?;
            }
            CoreEffectPlan::Compare { dst, lhs, op: _, rhs } => {
                Self::verify_value_id_basic(*dst, depth, "Compare.dst")?;
                Self::verify_value_id_basic(*lhs, depth, "Compare.lhs")?;
                Self::verify_value_id_basic(*rhs, depth, "Compare.rhs")?;
            }
            CoreEffectPlan::Select {
                dst,
                cond,
                then_val,
                else_val,
            } => {
                Self::verify_value_id_basic(*dst, depth, "Select.dst")?;
                Self::verify_value_id_basic(*cond, depth, "Select.cond")?;
                Self::verify_value_id_basic(*then_val, depth, "Select.then_val")?;
                Self::verify_value_id_basic(*else_val, depth, "Select.else_val")?;
            }
            CoreEffectPlan::Const { dst, value: _ } => {
                Self::verify_value_id_basic(*dst, depth, "Const.dst")?;
            }
            CoreEffectPlan::ExitIf { cond, exit } => {
                if loop_depth == 0 {
                    return Err(Self::err(
                        "V12",
                        "exit_if_outside_loop",
                        format!("ExitIf outside loop body at depth {}", depth),
                    ));
                }
                Self::verify_value_id_basic(*cond, depth, "ExitIf.cond")?;
                match exit {
                    CoreExitPlan::Return(Some(value)) => {
                        Self::verify_value_id_basic(*value, depth, "ExitIf.return_value")?;
                    }
                    CoreExitPlan::Return(None) => {
                        return Err(Self::err(
                            "V12",
                            "exit_if_return_no_payload",
                            format!("ExitIf(Return) without payload at depth {}", depth),
                        ));
                    }
                    CoreExitPlan::Break(exit_depth) | CoreExitPlan::Continue(exit_depth) => {
                        Self::verify_exit_depth(*exit_depth, loop_depth, depth)?;
                    }
                }
            }
            CoreEffectPlan::IfEffect {
                cond,
                then_effects,
                else_effects,
            } => {
                if loop_depth == 0 {
                    return Err(Self::err(
                        "V12",
                        "if_effect_outside_loop",
                        format!("IfEffect outside loop body at depth {}", depth),
                    ));
                }
                Self::verify_value_id_basic(*cond, depth, "IfEffect.cond")?;
                Self::verify_if_effect_branch(then_effects, depth, loop_depth, "then_effects")?;
                if let Some(else_effects) = else_effects {
                    Self::verify_if_effect_branch(
                        else_effects,
                        depth,
                        loop_depth,
                        "else_effects",
                    )?;
                }
            }
        }
        Ok(())
    }

    fn verify_if_effect_branch(
        effects: &[CoreEffectPlan],
        depth: usize,
        loop_depth: usize,
        label: &str,
    ) -> Result<(), String> {
        if effects.is_empty() {
            return Err(Self::err(
                "V12",
                "if_effect_empty",
                format!("IfEffect at depth {} has empty {}", depth, label),
            ));
        }
        for (idx, effect) in effects.iter().enumerate() {
            let is_last = idx + 1 == effects.len();
            match effect {
                CoreEffectPlan::ExitIf {
                    cond: exit_cond,
                    exit: CoreExitPlan::Continue(exit_depth),
                } if is_last => {
                    Self::verify_value_id_basic(*exit_cond, depth, "IfEffect.continue.cond")?;
                    Self::verify_exit_depth(*exit_depth, loop_depth, depth)?;
                }
                CoreEffectPlan::ExitIf { .. } => {
                    return Err(Self::err(
                        "V12",
                        "if_effect_exit_not_allowed",
                        format!(
                            "IfEffect.{}[{}] has forbidden exit at depth {}",
                            label, idx, depth
                        ),
                    ));
                }
                CoreEffectPlan::IfEffect { .. } => {
                    Self::verify_effect(effect, depth + 1, loop_depth).map_err(|e| {
                        Self::err(
                            "V12",
                            "if_effect_invalid",
                            format!("IfEffect.{}[{}] invalid: {}", label, idx, e),
                        )
                    })?;
                }
                _ => {
                    if !is_leaf_effect_plan(effect) {
                        return Err(Self::err(
                            "V12",
                            "if_effect_non_leaf",
                            format!(
                                "IfEffect.{}[{}] has non-leaf effect at depth {}",
                                label, idx, depth
                            ),
                        ));
                    }
                    Self::verify_effect(effect, depth, loop_depth).map_err(|e| {
                        Self::err(
                            "V12",
                            "if_effect_invalid",
                            format!("IfEffect.{}[{}] invalid: {}", label, idx, e),
                        )
                    })?;
                }
            }
        }
        Ok(())
    }

    /// V3: Exit validity
    fn verify_exit(exit: &CoreExitPlan, depth: usize, loop_depth: usize) -> Result<(), String> {
        match exit {
            CoreExitPlan::Return(opt_val) => {
                if let Some(val) = opt_val {
                    Self::verify_value_id_basic(*val, depth, "Return.value")?;
                }
                // Return is always valid (in function context)
            }
            CoreExitPlan::Break(exit_depth) => {
                if loop_depth == 0 {
                    return Err(Self::err(
                        "V3",
                        "break_outside_loop",
                        format!("Break at depth {} outside of loop", depth),
                    ));
                }
                Self::verify_exit_depth(*exit_depth, loop_depth, depth)?;
            }
            CoreExitPlan::Continue(exit_depth) => {
                if loop_depth == 0 {
                    return Err(Self::err(
                        "V3",
                        "continue_outside_loop",
                        format!("Continue at depth {} outside of loop", depth),
                    ));
                }
                Self::verify_exit_depth(*exit_depth, loop_depth, depth)?;
            }
        }
        Ok(())
    }

    fn verify_exit_position(plans: &[CorePlan], depth: usize, scope: &str) -> Result<(), String> {
        for (i, plan) in plans.iter().enumerate() {
            if matches!(plan, CorePlan::Exit(_)) && i + 1 != plans.len() {
                return Err(Self::err(
                    "V11",
                    "exit_not_last",
                    format!(
                        "Exit at depth {} in {} must be last (index {}, len {})",
                        depth,
                        scope,
                        i,
                        plans.len()
                    ),
                ));
            }
        }
        Ok(())
    }

    fn verify_branch_plans(
        plans: &[CorePlan],
        depth: usize,
        loop_depth: usize,
        scope: &str,
    ) -> Result<(), String> {
        Self::verify_exit_position(plans, depth, scope)?;
        for (i, plan) in plans.iter().enumerate() {
            Self::verify_plan(plan, depth + 1, loop_depth).map_err(|e| {
                format!("[{}[{}]] {}", scope, i, e)
            })?;
        }
        Ok(())
    }

    fn verify_loop_body_tree(
        plans: &[CorePlan],
        depth: usize,
        loop_depth: usize,
    ) -> Result<(), String> {
        for (i, plan) in plans.iter().enumerate() {
            let path = format!("Loop.body[{}]", i);
            Self::verify_body_plan_tree(plan, depth, loop_depth, &path)?;
        }
        Self::verify_exit_if_position(plans, depth, "Loop.body")?;
        Ok(())
    }

    fn verify_body_plan_tree(
        plan: &CorePlan,
        depth: usize,
        loop_depth: usize,
        path: &str,
    ) -> Result<(), String> {
        match plan {
            CorePlan::Effect(effect) => Self::verify_effect(effect, depth, loop_depth),
            CorePlan::Seq(plans) => {
                Self::verify_exit_position(plans, depth, "Loop.body.Seq")?;
                for (i, nested) in plans.iter().enumerate() {
                    let nested_path = format!("{}.Seq[{}]", path, i);
                    Self::verify_body_plan_tree(nested, depth + 1, loop_depth, &nested_path)?;
                }
                Ok(())
            }
            CorePlan::If(if_plan) => Self::verify_if(if_plan, depth + 1, loop_depth),
            CorePlan::Loop(loop_plan) => Self::verify_loop(loop_plan, depth + 1, loop_depth),
            CorePlan::Exit(exit) => Self::verify_exit(exit, depth, loop_depth),
            CorePlan::BranchN(_) => Err(Self::err(
                "V12",
                "loop_body_branchn",
                format!("Loop body contains BranchN at depth {} ({})", depth, path),
            )),
        }
    }

    fn verify_exit_if_position(
        plans: &[CorePlan],
        depth: usize,
        scope: &str,
    ) -> Result<(), String> {
        for (i, plan) in plans.iter().enumerate() {
            if let CorePlan::Seq(nested) = plan {
                let nested_scope = format!("{}.Seq[{}]", scope, i);
                Self::verify_exit_if_position(nested, depth + 1, &nested_scope)?;
            }
        }
        Ok(())
    }

    fn verify_exit_depth(
        exit_depth: usize,
        loop_depth: usize,
        depth: usize,
    ) -> Result<(), String> {
        if exit_depth == 0 || exit_depth > loop_depth {
            return Err(Self::err(
                "V3",
                "exit_depth_out_of_range",
                format!(
                    "Exit depth {} is out of range (loop_depth={}) at depth {}",
                    exit_depth, loop_depth, depth
                ),
            ));
        }
        Ok(())
    }

    /// V6: Basic ValueId validity check
    ///
    /// Note: This is a basic check. Full validity would require builder context.
    fn verify_value_id_basic(value_id: ValueId, depth: usize, context: &str) -> Result<(), String> {
        // ValueId(0) might be valid in some contexts, so we don't check for zero
        // This is a placeholder for more sophisticated checks if needed
        let _ = (value_id, depth, context);
        Ok(())
    }

    fn verify_edge_args_layout(
        args: &EdgeArgs,
        depth: usize,
        context: &str,
    ) -> Result<(), String> {
        if matches!(args.layout, JumpArgsLayout::ExprResultPlusCarriers)
            && args.values.is_empty()
        {
            return Err(Self::err(
                "V13",
                "edge_args_missing_value",
                format!(
                    "EdgeArgs at depth {} {} requires expr_result value",
                    depth, context
                ),
            ));
        }
        Ok(())
    }

    fn err(code: &'static str, reason: &'static str, detail: impl std::fmt::Display) -> String {
        format!("[{}][reason={}] {}", code, reason, detail)
    }
}

#[cfg(debug_assertions)]
pub(in crate::mir::builder) fn debug_assert_value_join_invariants(facts: &CanonicalLoopFacts) {
    if facts.value_join_needed {
        debug_assert!(
            !facts.exit_kinds_present.is_empty(),
            "value join requires at least one exit kind"
        );
    }
}

#[cfg(not(debug_assertions))]
pub(in crate::mir::builder) fn debug_assert_value_join_invariants(_facts: &CanonicalLoopFacts) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::control_flow::plan::branchn::CoreBranchArmPlan;
    use crate::mir::{BasicBlockId, ConstValue, ValueId};
    use crate::mir::basic_block::EdgeArgs;
    use crate::mir::builder::control_flow::edgecfg::api::{EdgeStub, ExitKind, Frag};
    use crate::mir::builder::control_flow::plan::CorePhiInfo;
    #[cfg(debug_assertions)]
    use crate::mir::builder::control_flow::plan::facts::feature_facts::{
        LoopFeatureFacts, ValueJoinFacts,
    };
    #[cfg(debug_assertions)]
    use crate::mir::builder::control_flow::plan::facts::loop_facts::LoopFacts;
    #[cfg(debug_assertions)]
    use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
        ConditionShape, StepShape,
    };
    #[cfg(debug_assertions)]
    use crate::mir::builder::control_flow::plan::facts::skeleton_facts::{
        SkeletonFacts, SkeletonKind,
    };
    #[cfg(debug_assertions)]
    use crate::mir::builder::control_flow::plan::normalize::canonicalize_loop_facts;
    use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
    use std::collections::BTreeMap;

    fn make_loop_plan(body: Vec<CorePlan>) -> CoreLoopPlan {
        let preheader_bb = BasicBlockId(0);
        let header_bb = BasicBlockId(1);
        let body_bb = BasicBlockId(2);
        let step_bb = BasicBlockId(3);
        let after_bb = BasicBlockId(4);

        CoreLoopPlan {
            preheader_bb,
            header_bb,
            body_bb,
            step_bb,
            after_bb,
            found_bb: after_bb,
            body,
            cond_loop: ValueId(100),
            cond_match: ValueId(101),
            block_effects: vec![
                (preheader_bb, vec![]),
                (header_bb, vec![]),
                (body_bb, vec![]),
                (step_bb, vec![]),
            ],
            phis: vec![CorePhiInfo {
                block: header_bb,
                dst: ValueId(102),
                inputs: vec![(preheader_bb, ValueId(103)), (step_bb, ValueId(104))],
                tag: "test_phi".to_string(),
            }],
            frag: Frag::new(header_bb),
            final_values: vec![("i".to_string(), ValueId(102))],
        }
    }

    #[test]
    fn test_verify_empty_seq_fails() {
        let plan = CorePlan::Seq(vec![]);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V4]"));
    }

    #[test]
    fn test_verify_break_outside_loop_fails() {
        let plan = CorePlan::Exit(CoreExitPlan::Break(1));
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V3]"));
    }

    #[test]
    fn test_verify_const_effect_succeeds() {
        let plan = CorePlan::Effect(CoreEffectPlan::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(42),
        });
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_loop_body_seq_effects_ok() {
        let body = vec![CorePlan::Seq(vec![
            CorePlan::Effect(CoreEffectPlan::Const {
                dst: ValueId(10),
                value: ConstValue::Integer(1),
            }),
            CorePlan::Seq(vec![CorePlan::Effect(CoreEffectPlan::Const {
                dst: ValueId(11),
                value: ConstValue::Integer(2),
            })]),
        ])];
        let plan = CorePlan::Loop(make_loop_plan(body));
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_loop_body_if_effect_ok() {
        let if_effect = CoreEffectPlan::IfEffect {
            cond: ValueId(1),
            then_effects: vec![CoreEffectPlan::Const {
                dst: ValueId(2),
                value: ConstValue::Integer(1),
            }],
            else_effects: None,
        };
        let plan = CorePlan::Loop(make_loop_plan(vec![CorePlan::Effect(if_effect)]));
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_loop_body_if_effect_exit_fails() {
        let if_effect = CoreEffectPlan::IfEffect {
            cond: ValueId(1),
            then_effects: vec![CoreEffectPlan::ExitIf {
                cond: ValueId(2),
                exit: CoreExitPlan::Return(Some(ValueId(3))),
            }],
            else_effects: None,
        };
        let plan = CorePlan::Loop(make_loop_plan(vec![CorePlan::Effect(if_effect)]));
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V12]"));
    }

    #[test]
    fn test_verify_loop_body_if_effect_continue_ok() {
        let if_effect = CoreEffectPlan::IfEffect {
            cond: ValueId(1),
            then_effects: vec![CoreEffectPlan::ExitIf {
                cond: ValueId(1),
                exit: CoreExitPlan::Continue(1),
            }],
            else_effects: None,
        };
        let plan = CorePlan::Loop(make_loop_plan(vec![CorePlan::Effect(if_effect)]));
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_loop_body_if_fails() {
        let if_plan = CoreIfPlan {
            condition: ValueId(1),
            then_plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                dst: ValueId(2),
                value: ConstValue::Integer(1),
            })],
            else_plans: None,
        };
        let plan = CorePlan::Loop(make_loop_plan(vec![CorePlan::If(if_plan)]));
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V12]"));
    }

    #[test]
    fn test_verify_loop_body_exit_fails() {
        let plan = CorePlan::Loop(make_loop_plan(vec![CorePlan::Exit(
            CoreExitPlan::Return(None),
        )]));
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V12]"));
    }

    #[test]
    fn test_verify_if_empty_else_fails() {
        let if_plan = CoreIfPlan {
            condition: ValueId(1),
            then_plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                dst: ValueId(2),
                value: ConstValue::Integer(1),
            })],
            else_plans: Some(vec![]),
        };
        let plan = CorePlan::If(if_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V5]"));
    }

    #[test]
    fn test_verify_exit_not_last_fails() {
        let plan = CorePlan::Seq(vec![
            CorePlan::Exit(CoreExitPlan::Return(None)),
            CorePlan::Effect(CoreEffectPlan::Const {
                dst: ValueId(1),
                value: ConstValue::Integer(0),
            }),
        ]);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V11]"));
    }

    #[test]
    fn test_verify_if_exit_not_last_fails() {
        let if_plan = CoreIfPlan {
            condition: ValueId(1),
            then_plans: vec![
                CorePlan::Exit(CoreExitPlan::Return(None)),
                CorePlan::Effect(CoreEffectPlan::Const {
                    dst: ValueId(2),
                    value: ConstValue::Integer(1),
                }),
            ],
            else_plans: Some(vec![CorePlan::Effect(CoreEffectPlan::Const {
                dst: ValueId(3),
                value: ConstValue::Integer(2),
            })]),
        };
        let plan = CorePlan::If(if_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V11]"));
    }

    #[test]
    fn test_verify_branchn_ok() {
        let branch_plan = CoreBranchNPlan {
            arms: vec![
                CoreBranchArmPlan {
                    condition: ValueId(1),
                    plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                        dst: ValueId(10),
                        value: ConstValue::Integer(1),
                    })],
                },
                CoreBranchArmPlan {
                    condition: ValueId(2),
                    plans: vec![CorePlan::Exit(CoreExitPlan::Return(None))],
                },
            ],
            else_plans: Some(vec![CorePlan::Effect(CoreEffectPlan::Const {
                dst: ValueId(11),
                value: ConstValue::Integer(2),
            })]),
        };
        let plan = CorePlan::BranchN(branch_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_branchn_single_arm_fails() {
        let branch_plan = CoreBranchNPlan {
            arms: vec![CoreBranchArmPlan {
                condition: ValueId(1),
                plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                    dst: ValueId(10),
                    value: ConstValue::Integer(1),
                })],
            }],
            else_plans: None,
        };
        let plan = CorePlan::BranchN(branch_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V5]"));
    }

    #[test]
    fn test_verify_branchn_exit_not_last_fails() {
        let branch_plan = CoreBranchNPlan {
            arms: vec![
                CoreBranchArmPlan {
                    condition: ValueId(1),
                    plans: vec![
                        CorePlan::Exit(CoreExitPlan::Return(None)),
                        CorePlan::Effect(CoreEffectPlan::Const {
                            dst: ValueId(10),
                            value: ConstValue::Integer(1),
                        }),
                    ],
                },
                CoreBranchArmPlan {
                    condition: ValueId(2),
                    plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                        dst: ValueId(11),
                        value: ConstValue::Integer(2),
                    })],
                },
            ],
            else_plans: None,
        };
        let plan = CorePlan::BranchN(branch_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V11]"));
    }

    #[test]
    fn test_verify_expr_result_plus_carriers_requires_value() {
        let mut loop_plan = make_loop_plan(vec![]);
        loop_plan.frag.wires = vec![EdgeStub {
            from: loop_plan.body_bb,
            kind: ExitKind::Normal,
            target: Some(loop_plan.step_bb),
            args: EdgeArgs {
                layout: JumpArgsLayout::ExprResultPlusCarriers,
                values: vec![],
            },
        }];
        let plan = CorePlan::Loop(loop_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V13]"));
    }

    #[test]
    fn test_verify_expr_result_plus_carriers_with_value_ok() {
        let mut loop_plan = make_loop_plan(vec![]);
        loop_plan.frag.wires = vec![EdgeStub {
            from: loop_plan.body_bb,
            kind: ExitKind::Normal,
            target: Some(loop_plan.step_bb),
            args: EdgeArgs {
                layout: JumpArgsLayout::ExprResultPlusCarriers,
                values: vec![ValueId(200)],
            },
        }];
        let plan = CorePlan::Loop(loop_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_ok());
    }

    #[cfg(debug_assertions)]
    #[test]
    fn debug_value_join_invariant_allows_empty_when_not_needed() {
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: LoopFeatureFacts::default(),
            scan_with_init: None,
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        debug_assert_value_join_invariants(&canonical);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn debug_value_join_invariant_panics_without_exit_kinds() {
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: LoopFeatureFacts {
                exit_usage: Default::default(),
                exit_map: None,
                value_join: Some(ValueJoinFacts { needed: true }),
                cleanup: None,
                nested_loop: false,
            },
            scan_with_init: None,
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        debug_assert_value_join_invariants(&canonical);
    }

    #[test]
    fn test_v10_body_bb_effects_in_block_effects_fails() {
        // V10: body_bb effects must be empty in block_effects
        // This test verifies that having effects in body_bb's block_effects fails validation
        let preheader_bb = BasicBlockId(0);
        let header_bb = BasicBlockId(1);
        let body_bb = BasicBlockId(2);
        let step_bb = BasicBlockId(3);
        let after_bb = BasicBlockId(4);

        let loop_plan = CoreLoopPlan {
            preheader_bb,
            header_bb,
            body_bb,
            step_bb,
            after_bb,
            found_bb: after_bb,
            body: vec![],
            cond_loop: ValueId(100),
            cond_match: ValueId(101),
            block_effects: vec![
                (preheader_bb, vec![]),
                (header_bb, vec![]),
                // V10 violation: body_bb has effects in block_effects
                (body_bb, vec![CoreEffectPlan::Const {
                    dst: ValueId(102),
                    value: ConstValue::Integer(42),
                }]),
                (step_bb, vec![]),
            ],
            phis: vec![CorePhiInfo {
                block: header_bb,
                dst: ValueId(103),
                inputs: vec![(preheader_bb, ValueId(104)), (step_bb, ValueId(105))],
                tag: "test_phi".to_string(),
            }],
            frag: Frag {
                entry: header_bb,
                block_params: BTreeMap::new(),
                exits: BTreeMap::new(),
                wires: vec![],
                branches: vec![],
            },
            final_values: vec![("i".to_string(), ValueId(103))],
        };

        let plan = CorePlan::Loop(loop_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("[V10]"), "Expected V10 error, got: {}", err);
        assert!(err.contains("body_bb"), "Expected body_bb in error, got: {}", err);
    }
}
