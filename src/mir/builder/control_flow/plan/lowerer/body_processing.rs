//! Phase 273 P3: Body Processing - Loop body effects with control flow
//!
//! # Responsibilities
//!
//! - Process loop body effects with fallthrough tracking
//! - Handle IfEffect and ExitIf in loop bodies
//! - Flatten Seq plans into effect lists
//! - Emit body effects with or without control flow
//!
//! # Design
//!
//! - Fallthrough state management for effect chains
//! - Recursive calls to lower_with_stack for non-effect plans
//! - Control flow detection for optimization decisions

use super::LoopFrame;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirInstruction, ValueId};
use std::collections::HashSet;

mod helpers;
use helpers::{
    debug_log_literal_plan, effect_defined_value, find_forward_def, plans_find_def,
    validate_effects_binop_operands,
};

impl super::PlanLowerer {
    pub(super) fn emit_if_effect(
        builder: &mut MirBuilder,
        cond: crate::mir::ValueId,
        then_effects: &[CoreEffectPlan],
        else_effects: Option<&[CoreEffectPlan]>,
        fallthrough_target: BasicBlockId,
        loop_stack: &mut [LoopFrame],
    ) -> Result<(), String> {
        use crate::mir::builder::emission::branch::emit_conditional;

        let _pre_branch_bb = builder
            .current_block
            .ok_or_else(|| "[lowerer] No current block for IfEffect".to_string())?;

        let then_bb = builder.next_block_id();
        builder.ensure_block_exists(then_bb)?;
        builder.ensure_block_exists(fallthrough_target)?;

        let mut cond_val = cond;
        crate::mir::builder::ssa::local::finalize_branch_cond(builder, &mut cond_val)?;

        let else_bb = if else_effects.is_some() {
            let else_bb = builder.next_block_id();
            builder.ensure_block_exists(else_bb)?;
            Some(else_bb)
        } else {
            None
        };
        emit_conditional(
            builder,
            cond_val,
            then_bb,
            else_bb.unwrap_or(fallthrough_target),
        )?;

        builder.start_new_block(then_bb)?;
        Self::emit_effects_with_fallthrough(
            builder,
            then_effects,
            fallthrough_target,
            loop_stack,
        )?;

        if let (Some(else_bb), Some(else_effects)) = (else_bb, else_effects) {
            builder.start_new_block(else_bb)?;
            Self::emit_effects_with_fallthrough(
                builder,
                else_effects,
                fallthrough_target,
                loop_stack,
            )?;
        }

        Ok(())
    }

    pub(super) fn try_flatten_body_effects(
        plans: &[LoweredRecipe],
    ) -> Result<Option<Vec<CoreEffectPlan>>, String> {
        let mut out = Vec::new();
        for plan in plans {
            match plan {
                CorePlan::Effect(effect) => out.push(effect.clone()),
                CorePlan::Seq(nested) => {
                    let nested = match Self::try_flatten_body_effects(nested)? {
                        Some(nested) => nested,
                        None => return Ok(None),
                    };
                    out.extend(nested);
                }
                _ => return Ok(None),
            }
        }
        Ok(Some(out))
    }

    pub(super) fn emit_effects_with_fallthrough(
        builder: &mut MirBuilder,
        effects: &[CoreEffectPlan],
        fallthrough_target: BasicBlockId,
        loop_stack: &mut [LoopFrame],
    ) -> Result<(), String> {
        let strict_planner_required = crate::config::env::joinir_dev::strict_enabled()
            && crate::config::env::joinir_dev::planner_required_enabled();
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
        let mut idx = 0;
        let mut terminated = false;
        while idx < effects.len() {
            let effect = &effects[idx];
            if let CoreEffectPlan::Const { dst, value } = effect {
                debug_log_literal_plan(builder, "body_fallthrough", *dst, value);
            }
            match effect {
                CoreEffectPlan::ExitIf { cond, exit } => {
                    let has_more = idx + 1 < effects.len();
                    let fallthrough = if has_more {
                        let next_bb = builder.next_block_id();
                        builder.ensure_block_exists(next_bb)?;
                        Some(next_bb)
                    } else {
                        None
                    };
                    Self::emit_exit_if(
                        builder,
                        *cond,
                        exit,
                        fallthrough.unwrap_or(fallthrough_target),
                        loop_stack,
                    )?;
                    if let Some(next_bb) = fallthrough {
                        builder.start_new_block(next_bb)?;
                        terminated = false;
                    } else {
                        terminated = true;
                    }
                }
                CoreEffectPlan::IfEffect {
                    cond,
                    then_effects,
                    else_effects,
                } => {
                    let has_more = idx + 1 < effects.len();
                    let fallthrough = if has_more {
                        let next_bb = builder.next_block_id();
                        builder.ensure_block_exists(next_bb)?;
                        Some(next_bb)
                    } else {
                        None
                    };
                    Self::emit_if_effect(
                        builder,
                        *cond,
                        then_effects,
                        else_effects.as_deref(),
                        fallthrough.unwrap_or(fallthrough_target),
                        loop_stack,
                    )?;
                    if let Some(next_bb) = fallthrough {
                        builder.start_new_block(next_bb)?;
                        terminated = false;
                    } else {
                        terminated = true;
                    }
                }
                _ => {
                    if strict_planner_required {
                        if let (Some(ref defined_values), CoreEffectPlan::BinOp { dst, lhs, op, rhs }) =
                            (&defined_values, effect)
                        {
                            if !defined_values.contains(lhs) {
                                if let Some((def_idx, def_kind)) =
                                    find_forward_def(effects, idx + 1, *lhs)
                                {
                                    return Err(format!(
                                        "[freeze:contract][loop_lowering/effect_forward_ref] fn={} bb={:?} use=%{} use_idx={} def_idx={} def_kind={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=lhs path=body_fallthrough",
                                        builder
                                            .scope_ctx
                                            .current_function
                                            .as_ref()
                                            .map(|f| f.signature.name.as_str())
                                            .unwrap_or("<unknown>"),
                                        builder.current_block,
                                        lhs.0,
                                        idx,
                                        def_idx,
                                        def_kind,
                                        dst.0,
                                        op
                                    ));
                                }
                                return Err(format!(
                                    "[freeze:contract][loop_lowering/effect_undefined_operand] fn={} bb={:?} use=%{} use_idx={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=lhs plan_def=none path=body_fallthrough",
                                    builder
                                        .scope_ctx
                                        .current_function
                                        .as_ref()
                                        .map(|f| f.signature.name.as_str())
                                        .unwrap_or("<unknown>"),
                                    builder.current_block,
                                    lhs.0,
                                    idx,
                                    dst.0,
                                    op
                                ));
                            }
                            if !defined_values.contains(rhs) {
                                if let Some((def_idx, def_kind)) =
                                    find_forward_def(effects, idx + 1, *rhs)
                                {
                                    return Err(format!(
                                        "[freeze:contract][loop_lowering/effect_forward_ref] fn={} bb={:?} use=%{} use_idx={} def_idx={} def_kind={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=rhs path=body_fallthrough",
                                        builder
                                            .scope_ctx
                                            .current_function
                                            .as_ref()
                                            .map(|f| f.signature.name.as_str())
                                            .unwrap_or("<unknown>"),
                                        builder.current_block,
                                        rhs.0,
                                        idx,
                                        def_idx,
                                        def_kind,
                                        dst.0,
                                        op
                                    ));
                                }
                                return Err(format!(
                                    "[freeze:contract][loop_lowering/effect_undefined_operand] fn={} bb={:?} use=%{} use_idx={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=rhs plan_def=none path=body_fallthrough",
                                    builder
                                        .scope_ctx
                                        .current_function
                                        .as_ref()
                                        .map(|f| f.signature.name.as_str())
                                        .unwrap_or("<unknown>"),
                                    builder.current_block,
                                    rhs.0,
                                    idx,
                                    dst.0,
                                    op
                                ));
                            }
                        }
                    }
                    Self::emit_effect(builder, effect)?;
                    if let Some(ref mut defined_values) = defined_values {
                        if let Some((def_value, _)) = effect_defined_value(effect) {
                            defined_values.insert(def_value);
                        }
                    }
                    terminated = false;
                }
            }
            idx += 1;
        }

        if !terminated {
            builder.emit_instruction(MirInstruction::Jump {
                target: fallthrough_target,
                edge_args: None,
            })?;
        }
        Ok(())
    }

    pub(super) fn emit_body_effects(
        builder: &mut MirBuilder,
        effects: &[CoreEffectPlan],
        fallthrough_target: BasicBlockId,
        loop_stack: &mut [LoopFrame],
    ) -> Result<(), String> {
        let has_control_flow =
            super::super::coreloop_body_contract::has_control_flow_effect(effects);

        if !has_control_flow {
            let strict_planner_required = crate::config::env::joinir_dev::strict_enabled()
                && crate::config::env::joinir_dev::planner_required_enabled();
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
                if let CoreEffectPlan::Const { dst, value } = effect {
                    debug_log_literal_plan(builder, "body_effects", *dst, value);
                }
                if strict_planner_required {
                    if let (Some(ref defined_values), CoreEffectPlan::BinOp { dst, lhs, op, rhs }) =
                        (&defined_values, effect)
                    {
                        if !defined_values.contains(lhs) {
                            if let Some((def_idx, def_kind)) =
                                find_forward_def(effects, effect_idx + 1, *lhs)
                            {
                                return Err(format!(
                                    "[freeze:contract][loop_lowering/effect_forward_ref] fn={} bb={:?} use=%{} use_idx={} def_idx={} def_kind={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=lhs path=body_effects",
                                    builder
                                        .scope_ctx
                                        .current_function
                                        .as_ref()
                                        .map(|f| f.signature.name.as_str())
                                        .unwrap_or("<unknown>"),
                                    builder.current_block,
                                    lhs.0,
                                    effect_idx,
                                    def_idx,
                                    def_kind,
                                    dst.0,
                                    op
                                ));
                            }
                            return Err(format!(
                                "[freeze:contract][loop_lowering/effect_undefined_operand] fn={} bb={:?} use=%{} use_idx={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=lhs plan_def=none path=body_effects",
                                builder
                                    .scope_ctx
                                    .current_function
                                    .as_ref()
                                    .map(|f| f.signature.name.as_str())
                                    .unwrap_or("<unknown>"),
                                builder.current_block,
                                lhs.0,
                                effect_idx,
                                dst.0,
                                op
                            ));
                        }
                        if !defined_values.contains(rhs) {
                            if let Some((def_idx, def_kind)) =
                                find_forward_def(effects, effect_idx + 1, *rhs)
                            {
                                return Err(format!(
                                    "[freeze:contract][loop_lowering/effect_forward_ref] fn={} bb={:?} use=%{} use_idx={} def_idx={} def_kind={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=rhs path=body_effects",
                                    builder
                                        .scope_ctx
                                        .current_function
                                        .as_ref()
                                        .map(|f| f.signature.name.as_str())
                                        .unwrap_or("<unknown>"),
                                    builder.current_block,
                                    rhs.0,
                                    effect_idx,
                                    def_idx,
                                    def_kind,
                                    dst.0,
                                    op
                                ));
                            }
                            return Err(format!(
                                "[freeze:contract][loop_lowering/effect_undefined_operand] fn={} bb={:?} use=%{} use_idx={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=rhs plan_def=none path=body_effects",
                                builder
                                    .scope_ctx
                                    .current_function
                                    .as_ref()
                                    .map(|f| f.signature.name.as_str())
                                    .unwrap_or("<unknown>"),
                                builder.current_block,
                                rhs.0,
                                effect_idx,
                                dst.0,
                                op
                            ));
                        }
                    }
                }

                Self::emit_effect(builder, effect)?;
                if let Some(ref mut defined_values) = defined_values {
                    if let Some((def_value, _)) = effect_defined_value(effect) {
                        defined_values.insert(def_value);
                    }
                }
            }
            return Ok(());
        }

        Self::emit_effects_with_fallthrough(
            builder,
            effects,
            fallthrough_target,
            loop_stack,
        )
    }

    pub(super) fn emit_effect_in_loop(
        builder: &mut MirBuilder,
        effect: &CoreEffectPlan,
        loop_stack: &mut [LoopFrame],
    ) -> Result<(), String> {
        match effect {
            CoreEffectPlan::ExitIf { cond, exit } => {
                let fallthrough_target = builder.next_block_id();
                builder.ensure_block_exists(fallthrough_target)?;
                Self::emit_exit_if(builder, *cond, exit, fallthrough_target, loop_stack)?;
                builder.start_new_block(fallthrough_target)?;
            }
            CoreEffectPlan::IfEffect {
                cond,
                then_effects,
                else_effects,
            } => {
                validate_effects_binop_operands(builder, then_effects, "if_effect_then")?;
                if let Some(else_effects) = else_effects.as_deref() {
                    validate_effects_binop_operands(builder, else_effects, "if_effect_else")?;
                }
                let fallthrough_target = builder.next_block_id();
                builder.ensure_block_exists(fallthrough_target)?;
                Self::emit_if_effect(
                    builder,
                    *cond,
                    then_effects,
                    else_effects.as_deref(),
                    fallthrough_target,
                    loop_stack,
                )?;
                builder.start_new_block(fallthrough_target)?;
            }
            _ => {
                // This path emits a single effect (not a list). If a BinOp operand is undefined here,
                // it's a "dropped def/effect attach" bug upstream rather than a forward-ref within an effects list.
                let strict_planner_required = crate::config::env::joinir_dev::strict_enabled()
                    && crate::config::env::joinir_dev::planner_required_enabled();
                if strict_planner_required {
                    if let CoreEffectPlan::Copy { dst, src } = effect {
                        let src_local = crate::mir::builder::ssa::local::try_ensure(
                            builder,
                            *src,
                            crate::mir::builder::ssa::local::LocalKind::Arg,
                        )?;
                        builder.emit_instruction(MirInstruction::Copy {
                            dst: *dst,
                            src: src_local,
                        })?;
                        return Ok(());
                    }
                }
                if strict_planner_required {
                    if let CoreEffectPlan::BinOp { dst, lhs, op, rhs } = effect {
                        if let Some(func) = builder.scope_ctx.current_function.as_ref() {
                            let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);
                            let span = builder.metadata_ctx.current_span();
                            let file = builder
                                .metadata_ctx
                                .current_source_file()
                                .unwrap_or_else(|| "unknown".to_string());
                            if !def_blocks.contains_key(lhs) {
                                let origin_span = builder
                                    .metadata_ctx
                                    .value_span(*lhs)
                                    .map(|s| s.to_string())
                                    .unwrap_or_else(|| "unknown".to_string());
                                return Err(format!(
                                    "[freeze:contract][loop_lowering/effect_undefined_operand] fn={} bb={:?} use=%{} use_idx=0 use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=lhs plan_def=none path=effect_in_loop span={} span_start={} span_end={} file={} use_origin_span={}",
                                    func.signature.name,
                                    builder.current_block,
                                    lhs.0,
                                    dst.0,
                                    op,
                                    super::span_fmt::current_span_location(builder),
                                    span.start,
                                    span.end,
                                    file,
                                    origin_span
                                ));
                            }
                            if !def_blocks.contains_key(rhs) {
                                let origin_span = builder
                                    .metadata_ctx
                                    .value_span(*rhs)
                                    .map(|s| s.to_string())
                                    .unwrap_or_else(|| "unknown".to_string());
                                return Err(format!(
                                    "[freeze:contract][loop_lowering/effect_undefined_operand] fn={} bb={:?} use=%{} use_idx=0 use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=rhs plan_def=none path=effect_in_loop span={} span_start={} span_end={} file={} use_origin_span={}",
                                    func.signature.name,
                                    builder.current_block,
                                    rhs.0,
                                    dst.0,
                                    op,
                                    super::span_fmt::current_span_location(builder),
                                    span.start,
                                    span.end,
                                    file,
                                    origin_span
                                ));
                            }
                        }
                    }
                }
                Self::emit_effect(builder, effect)?;
            }
        }
        Ok(())
    }

    pub(super) fn lower_loop_body_plans(
        builder: &mut MirBuilder,
        plans: &[LoweredRecipe],
        ctx: &LoopRouteContext,
        loop_stack: &mut Vec<LoopFrame>,
        fallthrough_target: BasicBlockId,
    ) -> Result<(), String> {
        use crate::mir::builder::control_flow::joinir::trace;

        let trace_logger = trace::trace();
        let strict_planner_required = crate::config::env::joinir_dev::strict_enabled()
            && crate::config::env::joinir_dev::planner_required_enabled();
        for (idx, plan) in plans.iter().enumerate() {
            if let CorePlan::Effect(CoreEffectPlan::Const { dst, value }) = plan {
                debug_log_literal_plan(builder, "body_plan", *dst, value);
            }
            if strict_planner_required {
                if let CorePlan::Effect(CoreEffectPlan::BinOp { dst, lhs, op, rhs }) = plan {
                    if let Some(func) = builder.scope_ctx.current_function.as_ref() {
                        let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);
                        if !def_blocks.contains_key(lhs) {
                            if let Some((def_idx, def_kind)) =
                                plans_find_def(plans, idx + 1, *lhs)
                            {
                                return Err(format!(
                                    "[freeze:contract][loop_lowering/effect_cross_plan_forward_ref] fn={} use_idx={} use=%{} def_idx={} def_kind={} use_by=CorePlan::Effect dst=%{} op={:?} operand=lhs path=body_plan",
                                    ctx.func_name, idx, lhs.0, def_idx, def_kind, dst.0, op
                                ));
                            }
                            return Err(format!(
                                "[freeze:contract][loop_lowering/effect_undefined_operand] fn={} bb={:?} use=%{} use_idx={} use_by=CorePlan::Effect dst=%{} op={:?} operand=lhs plan_def=none path=body_plan",
                                ctx.func_name,
                                builder.current_block,
                                lhs.0,
                                idx,
                                dst.0,
                                op
                            ));
                        }
                        if !def_blocks.contains_key(rhs) {
                            if let Some((def_idx, def_kind)) =
                                plans_find_def(plans, idx + 1, *rhs)
                            {
                                return Err(format!(
                                    "[freeze:contract][loop_lowering/effect_cross_plan_forward_ref] fn={} use_idx={} use=%{} def_idx={} def_kind={} use_by=CorePlan::Effect dst=%{} op={:?} operand=rhs path=body_plan",
                                    ctx.func_name, idx, rhs.0, def_idx, def_kind, dst.0, op
                                ));
                            }
                            return Err(format!(
                                "[freeze:contract][loop_lowering/effect_undefined_operand] fn={} bb={:?} use=%{} use_idx={} use_by=CorePlan::Effect dst=%{} op={:?} operand=rhs plan_def=none path=body_plan",
                                ctx.func_name,
                                builder.current_block,
                                rhs.0,
                                idx,
                                dst.0,
                                op
                            ));
                        }
                    }
                }
            }
            Self::lower_with_stack(builder, plan.clone(), ctx, loop_stack)?;
            if builder.is_current_block_terminated() {
                trace_logger.debug(
                    "lowerer/loop_body",
                    &format!("func={} terminated_at idx={}", ctx.func_name, idx),
                );
                return Ok(());
            }
        }

        builder.ensure_block_exists(fallthrough_target)?;
        builder.emit_instruction(MirInstruction::Jump {
            target: fallthrough_target,
            edge_args: None,
        })?;
        if let Some(current_bb) = builder.current_block {
            trace_logger.debug(
                "lowerer/term_set",
                &format!(
                    "func={} bb={:?} term=Jump target={:?}",
                    ctx.func_name, current_bb, fallthrough_target
                ),
            );
        }
        Ok(())
    }
}
