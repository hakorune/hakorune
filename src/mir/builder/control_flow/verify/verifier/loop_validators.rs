//! Phase 29bq+: Loop validation logic
//!
//! # Responsibilities
//! - Verify loop structure invariants (V2, V7-V10, V10b, V14)
//! - Validate PHI nodes and carrier completeness
//! - Check loop wiring and pipeline correctness
//!
//! # Invariants
//! - V2: Condition validity (cond_loop, cond_match must be valid ValueIds)
//! - V7: PHI non-empty (loops must have at least one carrier)
//! - V8: Frag entry matches header_bb
//! - V9: block_effects contains header_bb
//! - V10: body_bb effects must be empty in block_effects (use loop_plan.body instead)
//! - V10b: InlineInBody requires empty step_bb effects
//! - V14: Continue target must be in frag wiring (pipeline invariant)

use super::{effect_validators, primitives};
use crate::mir::builder::control_flow::lower::{
    CoreEffectPlan, CoreExitPlan, CoreLoopPlan, CorePlan, LoopStepMode, LoweredRecipe,
    ExitKind,
};

/// Phase 273 P3: Verify loop with generalized fields
///
/// Invariants:
/// - V2: Condition validity (cond_loop, cond_match)
/// - V7: PHI non-empty (at least one carrier)
/// - V8: Frag entry matches header_bb
/// - V9: block_effects contains header_bb
pub(super) fn verify_loop(
    loop_plan: &CoreLoopPlan,
    depth: usize,
    loop_depth: usize,
) -> Result<(), String> {
    // V2: Condition validity (basic check - ValueId should be non-zero for safety)
    primitives::verify_value_id_basic(loop_plan.cond_loop, depth, "cond_loop")?;
    primitives::verify_value_id_basic(loop_plan.cond_match, depth, "cond_match")?;

    // V7: PHI non-empty (loops must have at least one carrier)
    if loop_plan.phis.is_empty() {
        return Err(primitives::err(
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
        return Err(primitives::err(
            "V8",
            "loop_frag_entry_mismatch",
            format!(
                "Loop at depth {} has frag.entry {:?} != header_bb {:?}",
                depth, loop_plan.frag.entry, loop_plan.header_bb
            ),
        ));
    }

    // V9: block_effects contains header_bb
    let has_header = loop_plan
        .block_effects
        .iter()
        .any(|(bb, _)| *bb == loop_plan.header_bb);
    if !has_header {
        return Err(primitives::err(
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
            return Err(primitives::err(
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

    // V10b: InlineInBody requires empty step_bb effects
    if matches!(loop_plan.step_mode, LoopStepMode::InlineInBody) {
        let step_effects = loop_plan
            .block_effects
            .iter()
            .find(|(bb, _)| *bb == loop_plan.step_bb)
            .map(|(_, effects)| effects.len())
            .unwrap_or(0);
        if step_effects != 0 {
            return Err(primitives::err(
                "V10b",
                "loop_step_bb_has_effects",
                format!(
                    "Loop at depth {} InlineInBody but step_bb {:?} has {} effects",
                    depth, loop_plan.step_bb, step_effects
                ),
            ));
        }
    }

    // V10c/V10d: InlineInBody + explicit-step contract (strict/dev-only producer path)
    if matches!(loop_plan.step_mode, LoopStepMode::InlineInBody) && loop_plan.has_explicit_step {
        if contains_depth1_continue_at_current_loop(&loop_plan.body) {
            return Err(primitives::err(
                "V10c",
                "loop_inline_explicit_continue_depth1",
                format!(
                    "Loop at depth {} InlineInBody(explicit-step) forbids Continue(depth=1) in body",
                    depth
                ),
            ));
        }
        verify_inline_explicit_step_backedge_contract(loop_plan, depth)?;
    }

    verify_loop_pipeline_invariants(loop_plan, depth)?;

    // Verify block_effects
    for (i, (bb, effects)) in loop_plan.block_effects.iter().enumerate() {
        for (j, effect) in effects.iter().enumerate() {
            effect_validators::verify_effect(effect, depth, 0)
                .map_err(|e| format!("[Loop.block_effects[{}={:?}][{}]] {}", i, bb, j, e))?;
        }
    }

    // Verify body plans (loop_depth + 1) - delegated to loop_body_validators
    super::loop_body_validators::verify_loop_body_tree(&loop_plan.body, depth, loop_depth + 1)?;

    // Verify PHIs
    for (i, phi) in loop_plan.phis.iter().enumerate() {
        primitives::verify_value_id_basic(phi.dst, depth, &format!("phi[{}].dst", i))?;
        for (j, (_, val)) in phi.inputs.iter().enumerate() {
            primitives::verify_value_id_basic(*val, depth, &format!("phi[{}].inputs[{}]", i, j))?;
        }
    }

    // Verify final_values
    for (i, (name, val)) in loop_plan.final_values.iter().enumerate() {
        if name.is_empty() {
            return Err(primitives::err(
                "V6",
                "final_value_empty_name",
                format!("final_values[{}] at depth {} has empty name", i, depth),
            ));
        }
        primitives::verify_value_id_basic(*val, depth, &format!("final_values[{}]", i))?;
    }

    // Verify EdgeArgs layout (V13)
    for (i, wire) in loop_plan.frag.wires.iter().enumerate() {
        primitives::verify_edge_args_layout(&wire.args, depth, &format!("frag.wires[{}]", i))?;
    }
    for (kind, stubs) in loop_plan.frag.exits.iter() {
        for (i, stub) in stubs.iter().enumerate() {
            primitives::verify_edge_args_layout(
                &stub.args,
                depth,
                &format!("frag.exits[{}][{}]", kind, i),
            )?;
        }
    }
    for (i, branch) in loop_plan.frag.branches.iter().enumerate() {
        primitives::verify_edge_args_layout(
            &branch.then_args,
            depth,
            &format!("frag.branches[{}].then", i),
        )?;
        primitives::verify_edge_args_layout(
            &branch.else_args,
            depth,
            &format!("frag.branches[{}].else", i),
        )?;
    }

    Ok(())
}

fn verify_loop_pipeline_invariants(loop_plan: &CoreLoopPlan, depth: usize) -> Result<(), String> {
    let continue_target = loop_plan.continue_target;
    let mut continue_in_frag = loop_plan.frag.entry == continue_target;
    if !continue_in_frag {
        continue_in_frag = loop_plan
            .frag
            .wires
            .iter()
            .any(|wire| wire.from == continue_target || wire.target == Some(continue_target))
            || loop_plan.frag.branches.iter().any(|branch| {
                branch.from == continue_target
                    || branch.then_target == continue_target
                    || branch.else_target == continue_target
            })
            || loop_plan.frag.exits.values().any(|stubs| {
                stubs.iter().any(|stub| {
                    stub.from == continue_target || stub.target == Some(continue_target)
                })
            })
            || loop_plan
                .block_effects
                .iter()
                .any(|(bb, _)| *bb == continue_target);
    }
    if !continue_in_frag {
        return Err(primitives::err(
            "V14",
            "loop_continue_target_missing",
            format!(
                "Loop at depth {} continue_target {:?} not found in frag wiring (pipeline)",
                depth, continue_target
            ),
        ));
    }

    Ok(())
}

fn contains_depth1_continue_at_current_loop(plans: &[LoweredRecipe]) -> bool {
    plans
        .iter()
        .any(plan_contains_depth1_continue_at_current_loop)
}

fn plan_contains_depth1_continue_at_current_loop(plan: &LoweredRecipe) -> bool {
    match plan {
        CorePlan::Seq(plans) => contains_depth1_continue_at_current_loop(plans),
        CorePlan::If(if_plan) => {
            contains_depth1_continue_at_current_loop(&if_plan.then_plans)
                || if_plan
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| contains_depth1_continue_at_current_loop(plans))
        }
        CorePlan::BranchN(branch_plan) => {
            branch_plan
                .arms
                .iter()
                .any(|arm| contains_depth1_continue_at_current_loop(&arm.plans))
                || branch_plan
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| contains_depth1_continue_at_current_loop(plans))
        }
        CorePlan::Loop(_) => false,
        CorePlan::Effect(effect) => effect_contains_depth1_continue(effect),
        CorePlan::Exit(CoreExitPlan::Continue(1))
        | CorePlan::Exit(CoreExitPlan::ContinueWithPhiArgs { depth: 1, .. }) => true,
        CorePlan::Exit(_) => false,
    }
}

fn effect_contains_depth1_continue(effect: &CoreEffectPlan) -> bool {
    match effect {
        CoreEffectPlan::ExitIf {
            exit: CoreExitPlan::Continue(1),
            ..
        }
        | CoreEffectPlan::ExitIf {
            exit: CoreExitPlan::ContinueWithPhiArgs { depth: 1, .. },
            ..
        } => true,
        CoreEffectPlan::IfEffect {
            then_effects,
            else_effects,
            ..
        } => {
            then_effects.iter().any(effect_contains_depth1_continue)
                || else_effects
                    .as_ref()
                    .is_some_and(|effects| effects.iter().any(effect_contains_depth1_continue))
        }
        _ => false,
    }
}

fn verify_inline_explicit_step_backedge_contract(
    loop_plan: &CoreLoopPlan,
    depth: usize,
) -> Result<(), String> {
    let continue_target = loop_plan.continue_target;
    let mut body_normal_wires_to_continue = 0usize;

    for wire in &loop_plan.frag.wires {
        if wire.target != Some(continue_target) {
            continue;
        }
        if wire.from == loop_plan.body_bb && wire.kind == ExitKind::Normal {
            body_normal_wires_to_continue += 1;
            continue;
        }
        return Err(primitives::err(
            "V10d",
            "loop_inline_explicit_backedge_shape",
            format!(
                "Loop at depth {} InlineInBody(explicit-step) has non-body/non-normal wire to continue_target {:?} (from={:?}, kind={:?})",
                depth, continue_target, wire.from, wire.kind
            ),
        ));
    }

    for branch in &loop_plan.frag.branches {
        if branch.then_target == continue_target || branch.else_target == continue_target {
            return Err(primitives::err(
                "V10d",
                "loop_inline_explicit_backedge_branch",
                format!(
                    "Loop at depth {} InlineInBody(explicit-step) forbids branch edges to continue_target {:?}",
                    depth, continue_target
                ),
            ));
        }
    }

    for stubs in loop_plan.frag.exits.values() {
        for stub in stubs {
            if stub.target == Some(continue_target) {
                return Err(primitives::err(
                    "V10d",
                    "loop_inline_explicit_backedge_exit",
                    format!(
                        "Loop at depth {} InlineInBody(explicit-step) forbids exit stubs to continue_target {:?}",
                        depth, continue_target
                    ),
                ));
            }
        }
    }

    if body_normal_wires_to_continue != 1 {
        return Err(primitives::err(
            "V10d",
            "loop_inline_explicit_backedge_count",
            format!(
                "Loop at depth {} InlineInBody(explicit-step) requires exactly one body->continue normal wire, got {}",
                depth, body_normal_wires_to_continue
            ),
        ));
    }

    Ok(())
}
