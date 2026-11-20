//! Phase 273 P3: Body Processing - Loop body effects with control flow
//!
//! # Responsibilities
//!
//! - Process loop body effects with fallthrough tracking
//! - Lower IfEffect arms through the ordinary if writer
//! - Flatten Seq plans into effect lists
//! - Emit body effects with or without control flow
//!
//! # Design
//!
//! - Forward normal loop body completion to the step block
//! - Recursive calls to lower_with_stack for non-effect plans
//! - Control flow detection for optimization decisions

use super::LoopFrame;
use crate::mir::builder::control_flow::plan::lowering_context::PlanLoweringContext;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::if_form::IfBranchKindV1;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirInstruction, ValueId};
use std::collections::HashSet;

mod helpers;
use helpers::{
    debug_log_literal_plan, effect_defined_value, find_forward_def, plans_find_def,
    validate_effects_binop_operands,
};

impl super::PlanLowerer {
    fn emit_if_effect(
        builder: &mut MirBuilder,
        condition: ValueId,
        then_effects: &[CoreEffectPlan],
        else_effects: Option<&[CoreEffectPlan]>,
        port: &mut super::emission_port::CorePlanEffectEmissionPortV1<'_>,
    ) -> Result<(), String> {
        // The plan has already resolved each operand to a ValueId. Its variable
        // cache can name destinations that the following effects still define.
        let bindings = std::mem::take(&mut builder.function_state.variable_ctx.variable_map);
        let result = builder.lower_if_form_for_effects_v1(
            condition,
            None,
            else_effects.is_some(),
            |builder, branch| {
                let effects = match branch {
                    IfBranchKindV1::Then => then_effects,
                    IfBranchKindV1::Else => {
                        else_effects.expect("if_form visits explicit else arms only")
                    }
                };
                Self::emit_effects(builder, effects, port)
            },
            None,
        );
        builder.function_state.variable_ctx.variable_map = bindings;
        result
    }

    fn emit_effects(
        builder: &mut MirBuilder,
        effects: &[CoreEffectPlan],
        port: &mut super::emission_port::CorePlanEffectEmissionPortV1<'_>,
    ) -> Result<(), String> {
        let strict_planner_required = crate::config::env::joinir_dev::strict_enabled()
            && crate::config::env::joinir_dev::planner_required_enabled();
        let mut defined_values = if strict_planner_required {
            builder
                .function_state
                .current_function
                .as_ref()
                .map(|func| {
                    crate::mir::verification::utils::compute_def_blocks(func)
                        .keys()
                        .copied()
                        .collect::<HashSet<ValueId>>()
                })
        } else {
            None
        };
        for (idx, effect) in effects.iter().enumerate() {
            if let CoreEffectPlan::Const { dst, value } = effect {
                debug_log_literal_plan(builder, "body_fallthrough", *dst, value);
            }
            match effect {
                CoreEffectPlan::IfEffect {
                    cond,
                    then_effects,
                    else_effects,
                } => {
                    Self::emit_if_effect(
                        builder,
                        *cond,
                        then_effects,
                        else_effects.as_deref(),
                        port,
                    )?;
                }
                _ => {
                    if strict_planner_required {
                        if let (
                            Some(ref defined_values),
                            CoreEffectPlan::BinOp { dst, lhs, op, rhs },
                        ) = (&defined_values, effect)
                        {
                            if !defined_values.contains(lhs) {
                                if let Some((def_idx, def_kind)) =
                                    find_forward_def(effects, idx + 1, *lhs)
                                {
                                    return Err(format!(
                                        "[freeze:contract][loop_lowering/effect_forward_ref] fn={} bb={:?} use=%{} use_idx={} def_idx={} def_kind={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=lhs path=body_fallthrough",
                                        builder.function_state.current_function
                                            .as_ref()
                                            .map(|f| f.signature.name.as_str())
                                            .unwrap_or("<unknown>"),
                                        builder.function_state.current_block,
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
                                    builder.function_state.current_function
                                        .as_ref()
                                        .map(|f| f.signature.name.as_str())
                                        .unwrap_or("<unknown>"),
                                    builder.function_state.current_block,
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
                                        builder.function_state.current_function
                                            .as_ref()
                                            .map(|f| f.signature.name.as_str())
                                            .unwrap_or("<unknown>"),
                                        builder.function_state.current_block,
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
                                    builder.function_state.current_function
                                        .as_ref()
                                        .map(|f| f.signature.name.as_str())
                                        .unwrap_or("<unknown>"),
                                    builder.function_state.current_block,
                                    rhs.0,
                                    idx,
                                    dst.0,
                                    op
                                ));
                            }
                        }
                    }
                    port.emit_effect(builder, effect)?;
                    if let Some(ref mut defined_values) = defined_values {
                        if let Some((def_value, _)) = effect_defined_value(effect) {
                            defined_values.insert(def_value);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub(super) fn emit_body_effects(
        builder: &mut MirBuilder,
        effects: &[CoreEffectPlan],
        fallthrough_target: BasicBlockId,
        port: &mut super::emission_port::CorePlanEffectEmissionPortV1<'_>,
    ) -> Result<(), String> {
        let has_control_flow =
            crate::mir::builder::control_flow::verify::coreloop_body_contract::has_control_flow_effect(effects);

        if !has_control_flow {
            let strict_planner_required = crate::config::env::joinir_dev::strict_enabled()
                && crate::config::env::joinir_dev::planner_required_enabled();
            let mut defined_values = if strict_planner_required {
                builder
                    .function_state
                    .current_function
                    .as_ref()
                    .map(|func| {
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
                                    builder.function_state.current_function
                                        .as_ref()
                                        .map(|f| f.signature.name.as_str())
                                        .unwrap_or("<unknown>"),
                                    builder.function_state.current_block,
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
                                builder.function_state.current_function
                                    .as_ref()
                                    .map(|f| f.signature.name.as_str())
                                    .unwrap_or("<unknown>"),
                                builder.function_state.current_block,
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
                                    builder.function_state.current_function
                                        .as_ref()
                                        .map(|f| f.signature.name.as_str())
                                        .unwrap_or("<unknown>"),
                                    builder.function_state.current_block,
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
                                builder.function_state.current_function
                                    .as_ref()
                                    .map(|f| f.signature.name.as_str())
                                    .unwrap_or("<unknown>"),
                                builder.function_state.current_block,
                                rhs.0,
                                effect_idx,
                                dst.0,
                                op
                            ));
                        }
                    }
                }

                port.emit_effect(builder, effect)?;
                if let Some(ref mut defined_values) = defined_values {
                    if let Some((def_value, _)) = effect_defined_value(effect) {
                        defined_values.insert(def_value);
                    }
                }
            }
            return Ok(());
        }

        Self::emit_effects(builder, effects, port)?;
        if !builder.is_current_block_terminated() {
            builder.emit_instruction(MirInstruction::Jump {
                target: fallthrough_target,
                edge_args: None,
            })?;
        }
        Ok(())
    }

    pub(super) fn emit_effect_in_loop(
        builder: &mut MirBuilder,
        effect: &CoreEffectPlan,
        port: &mut super::emission_port::CorePlanEffectEmissionPortV1<'_>,
    ) -> Result<(), String> {
        match effect {
            CoreEffectPlan::IfEffect {
                cond,
                then_effects,
                else_effects,
            } => {
                validate_effects_binop_operands(builder, then_effects, "if_effect_then")?;
                if let Some(else_effects) = else_effects.as_deref() {
                    validate_effects_binop_operands(builder, else_effects, "if_effect_else")?;
                }
                Self::emit_if_effect(builder, *cond, then_effects, else_effects.as_deref(), port)?;
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
                        if let Some(func) = builder.function_state.current_function.as_ref() {
                            let def_blocks =
                                crate::mir::verification::utils::compute_def_blocks(func);
                            let span = builder.metadata_ctx.current_span();
                            let file = builder
                                .metadata_ctx
                                .current_source_file()
                                .unwrap_or_else(|| "unknown".to_string());
                            if !def_blocks.contains_key(lhs) {
                                let origin_span = builder
                                    .value_origin_span(*lhs)
                                    .map(|s| s.to_string())
                                    .unwrap_or_else(|| "unknown".to_string());
                                return Err(format!(
                                    "[freeze:contract][loop_lowering/effect_undefined_operand] fn={} bb={:?} use=%{} use_idx=0 use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=lhs plan_def=none path=effect_in_loop span={} span_start={} span_end={} file={} use_origin_span={}",
                                    func.signature.name,
                                    builder.function_state.current_block,
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
                                    .value_origin_span(*rhs)
                                    .map(|s| s.to_string())
                                    .unwrap_or_else(|| "unknown".to_string());
                                return Err(format!(
                                    "[freeze:contract][loop_lowering/effect_undefined_operand] fn={} bb={:?} use=%{} use_idx=0 use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=rhs plan_def=none path=effect_in_loop span={} span_start={} span_end={} file={} use_origin_span={}",
                                    func.signature.name,
                                    builder.function_state.current_block,
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
                port.emit_effect(builder, effect)?;
            }
        }
        Ok(())
    }

    pub(super) fn lower_loop_body_plans(
        builder: &mut MirBuilder,
        plans: &[LoweredRecipe],
        ctx: &dyn PlanLoweringContext,
        loop_stack: &mut Vec<LoopFrame>,
        fallthrough_target: BasicBlockId,
        port: &mut super::emission_port::CorePlanEffectEmissionPortV1<'_>,
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
                    if let Some(func) = builder.function_state.current_function.as_ref() {
                        let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);
                        if !def_blocks.contains_key(lhs) {
                            if let Some((def_idx, def_kind)) = plans_find_def(plans, idx + 1, *lhs)
                            {
                                return Err(format!(
                                    "[freeze:contract][loop_lowering/effect_cross_plan_forward_ref] fn={} use_idx={} use=%{} def_idx={} def_kind={} use_by=CorePlan::Effect dst=%{} op={:?} operand=lhs path=body_plan",
                                    ctx.function_name(), idx, lhs.0, def_idx, def_kind, dst.0, op
                                ));
                            }
                            return Err(format!(
                                "[freeze:contract][loop_lowering/effect_undefined_operand] fn={} bb={:?} use=%{} use_idx={} use_by=CorePlan::Effect dst=%{} op={:?} operand=lhs plan_def=none path=body_plan",
                                ctx.function_name(),
                                builder.function_state.current_block,
                                lhs.0,
                                idx,
                                dst.0,
                                op
                            ));
                        }
                        if !def_blocks.contains_key(rhs) {
                            if let Some((def_idx, def_kind)) = plans_find_def(plans, idx + 1, *rhs)
                            {
                                return Err(format!(
                                    "[freeze:contract][loop_lowering/effect_cross_plan_forward_ref] fn={} use_idx={} use=%{} def_idx={} def_kind={} use_by=CorePlan::Effect dst=%{} op={:?} operand=rhs path=body_plan",
                                    ctx.function_name(), idx, rhs.0, def_idx, def_kind, dst.0, op
                                ));
                            }
                            return Err(format!(
                                "[freeze:contract][loop_lowering/effect_undefined_operand] fn={} bb={:?} use=%{} use_idx={} use_by=CorePlan::Effect dst=%{} op={:?} operand=rhs plan_def=none path=body_plan",
                                ctx.function_name(),
                                builder.function_state.current_block,
                                rhs.0,
                                idx,
                                dst.0,
                                op
                            ));
                        }
                    }
                }
            }
            Self::lower_with_stack(builder, plan.clone(), ctx, loop_stack, port)?;
            if builder.is_current_block_terminated() {
                trace_logger.debug(
                    "lowerer/loop_body",
                    &format!("func={} terminated_at idx={}", ctx.function_name(), idx),
                );
                return Ok(());
            }
        }

        builder.ensure_block_exists(fallthrough_target)?;
        builder.emit_instruction(MirInstruction::Jump {
            target: fallthrough_target,
            edge_args: None,
        })?;
        if let Some(current_bb) = builder.function_state.current_block {
            trace_logger.debug(
                "lowerer/term_set",
                &format!(
                    "func={} bb={:?} term=Jump target={:?}",
                    ctx.function_name(),
                    current_bb,
                    fallthrough_target
                ),
            );
        }
        Ok(())
    }
}
