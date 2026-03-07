//! Block Effect Emission Utilities
//!
//! This module contains block effects emission logic for loop lowering.
//! Phase 29bq+: Extracted from loop_lowering.rs for better modularity.
//!
//! Responsibilities:
//! - Step 2: Emit block effects in SSOT order (preheader, header, body, step)
//! - Strict planner validation for undefined operands and forward references
//! - Body block vs normal block handling

use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::{HashMap, HashSet};

use super::{LoopFrame, loop_validation};
use crate::mir::builder::control_flow::joinir::trace;

/// Emit all block effects for loop lowering (Step 2)
///
/// This is the main entry point for block effect emission. It handles:
/// - Strict planner validation setup
/// - Block effects iteration in SSOT order
/// - Body block vs normal block handling
///
/// Returns Ok(()) on success, or Err with freeze contract violation details.
pub fn emit_all_block_effects(
    builder: &mut MirBuilder,
    loop_plan: &CoreLoopPlan,
    block_effects: &[(BasicBlockId, Vec<CoreEffectPlan>)],
    body_effects: Option<Vec<CoreEffectPlan>>,
    ctx: &LoopRouteContext,
    loop_stack: &mut Vec<LoopFrame>,
) -> Result<(), String> {
    let trace_logger = trace::trace();
    let debug = ctx.debug;

    // Strict planner setup for validation
    let strict_planner_required = crate::config::env::joinir_dev::strict_enabled()
        && crate::config::env::joinir_dev::planner_required_enabled();
    let planned_defs = if strict_planner_required {
        let mut defs: HashMap<ValueId, (BasicBlockId, usize, &'static str)> = HashMap::new();
        for (block_id, effects) in block_effects {
            for (effect_idx, effect) in effects.iter().enumerate() {
                if let Some((def_value, def_kind)) = loop_validation::effect_defined_value(effect) {
                    defs.entry(def_value).or_insert((*block_id, effect_idx, def_kind));
                }
            }
        }
        Some(defs)
    } else {
        None
    };

    // Emit block effects in SSOT order
    for (block_id, effects) in block_effects {
        builder.start_new_block(*block_id)?;
        let term = builder
            .scope_ctx
            .current_function
            .as_ref()
            .and_then(|func| func.get_block(*block_id))
            .and_then(|block| block.terminator.as_ref())
            .map(|terminator| format!("{:?}", terminator))
            .unwrap_or_else(|| "None".to_string());
        trace_logger.debug(
            "lowerer/block_enter",
            &format!(
                "func={} bb={:?} terminated={} term={}",
                ctx.func_name,
                block_id,
                builder.is_current_block_terminated(),
                term
            ),
        );

        // Special handling for body block: emit body CorePlan instead of effects
        if *block_id == loop_plan.body_bb {
            if builder.is_current_block_terminated() {
                return Err(format!(
                    "[lowerer] loop body block already terminated (bb={:?}, term={})",
                    block_id, term
                ));
            }
            if let Some(ref effects) = body_effects {
                emit_body_effects_from_lowerer(builder, effects, loop_plan.step_bb, loop_stack)?;
            } else {
                emit_loop_body_plans(
                    builder,
                    &loop_plan.body,
                    ctx,
                    loop_stack,
                    loop_plan.step_bb,
                )?;
            }
        } else {
            // Normal block: emit effects with strict validation
            emit_block_effects_strict(
                builder,
                *block_id,
                effects,
                ctx,
                &planned_defs,
                strict_planner_required,
            )?;
        }
    }

    if debug {
        trace_logger.debug(
            "lowerer/loop_generalized",
            &format!("Block effects emitted: {} blocks", block_effects.len()),
        );
    }

    Ok(())
}

/// Emit block effects with strict planner validation
///
/// This function handles normal (non-body) blocks with full validation:
/// - Checks for undefined operands
/// - Validates no forward references within block
/// - Validates no cross-block forward references
fn emit_block_effects_strict(
    builder: &mut MirBuilder,
    block_id: BasicBlockId,
    effects: &[CoreEffectPlan],
    ctx: &LoopRouteContext,
    planned_defs: &Option<HashMap<ValueId, (BasicBlockId, usize, &'static str)>>,
    strict_planner_required: bool,
) -> Result<(), String> {
    let mut defined_values = if strict_planner_required {
        builder.scope_ctx.current_function.as_ref().map(|func| {
            crate::mir::verification::utils::compute_def_blocks(func)
                .keys()
                .copied()
                .collect::<HashSet<ValueId>>()
        })
    } else {
        None
    };

    for (effect_idx, effect) in effects.iter().enumerate() {
        // Debug logging for Const effects
        if let CoreEffectPlan::Const { dst, value } = effect {
            loop_validation::debug_log_literal_plan(builder, "block_effects", *dst, value);
        }

        // Strict planner validation for BinOp
        if strict_planner_required {
            if let (Some(ref defined_values), CoreEffectPlan::BinOp { dst, lhs, op, rhs }) =
                (&defined_values, effect)
            {
                let op_str = format!("{:?}", op);
                validate_binop_operands(
                    defined_values,
                    planned_defs,
                    effects,
                    effect_idx,
                    *lhs,
                    *dst,
                    &op_str,
                    block_id,
                    ctx.func_name,
                    "lhs",
                )?;
                validate_binop_operands(
                    defined_values,
                    planned_defs,
                    effects,
                    effect_idx,
                    *rhs,
                    *dst,
                    &op_str,
                    block_id,
                    ctx.func_name,
                    "rhs",
                )?;
            }
        }

        // Emit the effect
        emit_effect_from_lowerer(builder, effect)?;

        // Track defined values
        if let Some(ref mut defined_values) = defined_values {
            if let Some((def_value, _)) = loop_validation::effect_defined_value(effect) {
                defined_values.insert(def_value);
            }
        }
    }

    Ok(())
}

/// Validate BinOp operands are defined (strict planner check)
fn validate_binop_operands(
    defined_values: &HashSet<ValueId>,
    planned_defs: &Option<HashMap<ValueId, (BasicBlockId, usize, &'static str)>>,
    effects: &[CoreEffectPlan],
    effect_idx: usize,
    operand: ValueId,
    dst: ValueId,
    op: &str,
    block_id: BasicBlockId,
    func_name: &str,
    operand_name: &str,
) -> Result<(), String> {
    if !defined_values.contains(&operand) {
        // Check for forward definition within same block
        if let Some((def_idx, def_kind)) = loop_validation::find_forward_def(effects, effect_idx + 1, operand) {
            return Err(format!(
                "[freeze:contract][loop_lowering/effect_forward_ref] fn={} bb={:?} use=%{} use_idx={} def_idx={} def_kind={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand={}",
                func_name, block_id, operand.0, effect_idx, def_idx, def_kind, dst.0, op, operand_name
            ));
        }
        // Check planned definitions across blocks
        if let Some(ref planned_defs) = planned_defs {
            if let Some((def_bb, def_idx, def_kind)) = planned_defs.get(&operand) {
                if def_bb != &block_id {
                    return Err(format!(
                        "[freeze:contract][loop_lowering/effect_cross_block_forward_ref] fn={} use_bb={:?} use=%{} use_idx={} def_bb={:?} def_idx={} def_kind={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand={}",
                        func_name, block_id, operand.0, effect_idx, def_bb, def_idx, def_kind, dst.0, op, operand_name
                    ));
                }
            } else {
                return Err(format!(
                    "[freeze:contract][loop_lowering/effect_undefined_operand] fn={} bb={:?} use=%{} use_idx={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand={} plan_def=none",
                    func_name, block_id, operand.0, effect_idx, dst.0, op, operand_name
                ));
            }
        }
    }
    Ok(())
}

/// Lower loop body plans (used when body_effects is None)
///
/// This delegates to the PlanLowerer's lower_loop_body_plans method.
fn emit_loop_body_plans(
    builder: &mut MirBuilder,
    body: &[LoweredRecipe],
    ctx: &LoopRouteContext,
    loop_stack: &mut Vec<LoopFrame>,
    step_bb: BasicBlockId,
) -> Result<(), String> {
    use crate::mir::builder::control_flow::plan::lowerer::PlanLowerer;

    PlanLowerer::lower_loop_body_plans(builder, body, ctx, loop_stack, step_bb)
}

/// Emit body effects (delegates to PlanLowerer::emit_body_effects)
fn emit_body_effects_from_lowerer(
    builder: &mut MirBuilder,
    effects: &[CoreEffectPlan],
    step_bb: BasicBlockId,
    loop_stack: &mut Vec<LoopFrame>,
) -> Result<(), String> {
    use crate::mir::builder::control_flow::plan::lowerer::PlanLowerer;

    PlanLowerer::emit_body_effects(builder, effects, step_bb, loop_stack)
}

/// Emit effect (delegates to PlanLowerer::emit_effect)
fn emit_effect_from_lowerer(
    builder: &mut MirBuilder,
    effect: &CoreEffectPlan,
) -> Result<(), String> {
    use crate::mir::builder::control_flow::plan::lowerer::PlanLowerer;

    PlanLowerer::emit_effect(builder, effect)
}
