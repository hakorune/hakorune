//! Loop Preparation Utilities
//!
//! This module contains loop entry preparation logic for loop lowering.
//! Phase 29bq+: Extracted from loop_lowering.rs for better modularity.
//!
//! Responsibilities:
//! - Preheader block handling and rewiring
//! - Body effects flattening and control flow analysis
//! - Jump to loop entry
//! - Provisional PHI insertion (Step 1.5)

use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CoreLoopPlan, LoweredRecipe};
use crate::mir::builder::emission::phi_lifecycle;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirInstruction, ValueId};

/// Prepare loop entry: handle preheader, flatten body, jump to entry
///
/// This function:
/// 1. Handles preheader block rewiring if current block != preheader_bb
/// 2. Flattens body effects and analyzes control flow
/// 3. Emits jump from current block to loop entry
/// 4. Returns frag, body_effects (Option-wrapped), and loop_plan (mutated for preheader rewiring)
pub fn prepare_loop_entry(
    builder: &mut MirBuilder,
    mut loop_plan: CoreLoopPlan,
    ctx: &LoopRouteContext,
) -> Result<(Frag, Option<Vec<CoreEffectPlan>>, CoreLoopPlan), String> {
    use crate::mir::builder::control_flow::joinir::trace;

    let trace_logger = trace::trace();
    let debug = ctx.debug;

    // Preheader handling: if current block != preheader_bb, either:
    // - Jump to preheader (if fresh)
    // - Rewire preheader_bb to current_bb (if not fresh)
    if let Some(current_bb) = builder.current_block {
        if loop_plan.preheader_bb != current_bb {
            if loop_plan.preheader_is_fresh {
                builder.ensure_block_exists(loop_plan.preheader_bb)?;
                builder.emit_instruction(MirInstruction::Jump {
                    target: loop_plan.preheader_bb,
                    edge_args: None,
                })?;
                if debug {
                    trace_logger.debug(
                        "lowerer/term_set",
                        &format!(
                            "func={} bb={:?} term=Jump target={:?}",
                            ctx.func_name, current_bb, loop_plan.preheader_bb
                        ),
                    );
                }
                builder.start_new_block(loop_plan.preheader_bb)?;
            } else {
                let old_preheader = loop_plan.preheader_bb;
                loop_plan.preheader_bb = current_bb;
                for (block_id, _) in loop_plan.block_effects.iter_mut() {
                    if *block_id == old_preheader {
                        *block_id = current_bb;
                    }
                }
                for phi in loop_plan.phis.iter_mut() {
                    for (pred, _) in phi.inputs.iter_mut() {
                        if *pred == old_preheader {
                            *pred = current_bb;
                        }
                    }
                }
            }
        }
    }

    // Flatten body effects and check for control flow
    let body_effects = try_flatten_body_effects(&loop_plan.body)?;
    let body_has_control_flow = match body_effects.as_ref() {
        Some(effects) => has_control_flow_effect(effects),
        None => true,
    };

    // If body has control flow, retain wires that don't originate from body_bb
    if body_has_control_flow {
        loop_plan
            .frag
            .wires
            .retain(|wire| wire.from != loop_plan.body_bb);
    }

    let frag = loop_plan.frag.clone();

    // Step 1: Emit Jump from current block to loop entry
    if builder.current_block.is_some() {
        builder.emit_instruction(MirInstruction::Jump {
            target: frag.entry,
            edge_args: None,
        })?;
        if debug {
            if let Some(current_bb) = builder.current_block {
                trace_logger.debug(
                    "lowerer/term_set",
                    &format!(
                        "func={} bb={:?} term=Jump target={:?}",
                        ctx.func_name, current_bb, frag.entry
                    ),
                );
            }
        }
    }

    Ok((frag, body_effects, loop_plan))
}

/// Insert provisional PHIs (empty inputs) to define PHI dsts early
///
/// This ensures PHI dsts are in def_blocks before body instructions are emitted.
/// Inputs will be patched in Step 4 after Step 3.5 merges deferred inputs.
///
/// Returns list of (phi_dst, phi_bb, phi_tag) for error path validation.
pub fn insert_provisional_phis(
    builder: &mut MirBuilder,
    loop_plan: &CoreLoopPlan,
) -> Result<Vec<(ValueId, BasicBlockId, String)>, String> {
    // Step 1.5a: Ensure PHI blocks exist before provisional PHI insertion
    // Block Existence Contract: callsite is responsible for ensuring blocks exist
    for phi in &loop_plan.phis {
        builder.ensure_block_exists(phi.block)?;
    }

    // Step 1.5: Insert provisional PHIs with empty inputs
    let mut provisional_phis: Vec<(ValueId, BasicBlockId, String)> = Vec::new();
    for phi in &loop_plan.phis {
        phi_lifecycle::define_provisional_phi(
            builder,
            phi.block,
            phi.dst,
            &format!("loop_lowerer:step1.5:{}", phi.tag),
        )?;
        // Track for potential cleanup on error
        provisional_phis.push((phi.dst, phi.block, phi.tag.clone()));
    }

    Ok(provisional_phis)
}

/// Flatten nested Seq plans in body into a single Vec
fn try_flatten_body_effects(body: &[LoweredRecipe]) -> Result<Option<Vec<CoreEffectPlan>>, String> {
    use crate::mir::builder::control_flow::plan::CorePlan;

    if body.is_empty() {
        return Ok(None);
    }

    let mut effects = Vec::new();
    for plan in body {
        match plan {
            CorePlan::Effect(effect) => {
                effects.push(effect.clone());
            }
            CorePlan::Seq(inner_plans) => {
                for inner_plan in inner_plans {
                    if let CorePlan::Effect(inner_effect) = inner_plan {
                        effects.push(inner_effect.clone());
                    } else {
                        // Nested non-effect plan found - return None for complex case
                        return Ok(None);
                    }
                }
            }
            // Non-effect, non-seq plan found - use full body lowering
            _ => return Ok(None),
        }
    }

    Ok(Some(effects))
}

/// Check if effects contain control flow (ExitIf/IfEffect)
fn has_control_flow_effect(effects: &[CoreEffectPlan]) -> bool {
    use crate::mir::builder::control_flow::plan::CoreEffectPlan;

    effects.iter().any(|e| {
        matches!(
            e,
            CoreEffectPlan::ExitIf { .. } | CoreEffectPlan::IfEffect { .. }
        )
    })
}
