//! Phase 273 P3: Effect Emission - CoreEffectPlan → MIR
//!
//! # Responsibilities
//!
//! - Emit CoreEffectPlan variants to MIR instructions
//! - Handle 11 effect types (MethodCall, GlobalCall, ValueCall, ExternCall, NewBox, BinOp, Compare, Select, Const, Copy, ExitIf, IfEffect)
//! - ExitIf emission with conditional branching
//!
//! # Design
//!
//! - Effect emission is leaf operation (no recursive CorePlan processing)
//! - ExitIf requires loop context for Break/Continue target resolution
//! - Used by body_processing and loop_lowering

use super::{debug_ctx, debug_tags, LoopFrame};
use crate::mir::builder::calls::CallTarget;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CoreExitPlan};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirInstruction, ValueId};

impl super::PlanLowerer {
    /// Emit a single CoreEffectPlan as MirInstruction
    #[track_caller]
    pub(super) fn emit_effect(builder: &mut MirBuilder, effect: &CoreEffectPlan) -> Result<(), String> {
        match effect {
            CoreEffectPlan::Const { dst, value } => {
                if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
                    let fn_name = debug_ctx::current_fn_name(builder);
                    let next_value_id = builder
                        .scope_ctx
                        .current_function
                        .as_ref()
                        .map(|f| f.next_value_id)
                        .unwrap_or(0);
                    let file = builder
                        .metadata_ctx
                        .current_source_file()
                        .unwrap_or_else(|| "unknown".to_string());
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[lit/lower:emit] fn={} bb={:?} v=%{} lit={:?} span={} file={} next={} emit=ok",
                        fn_name,
                        builder.current_block,
                        dst.0,
                        value,
                        super::span_fmt::current_span_location(builder),
                        file,
                        next_value_id
                    ));
                }
                builder.emit_instruction(MirInstruction::Const {
                    dst: *dst,
                    value: value.clone(),
                })?;
            }
            CoreEffectPlan::MethodCall { dst, object, method, args, effects } => {
                // P2: dst and effects are now specified by Normalizer
                // LocalSSA: ensure receiver/args are materialized in the current block to avoid
                // cross-block undefined uses (dominance violations) when values were defined on
                // a different control-flow path.
                let box_val = builder.local_recv(*object);
                let args: Vec<ValueId> = args.iter().copied().map(|a| builder.local_arg(a)).collect();
                builder.emit_instruction(crate::mir::ssot::method_call::runtime_method_call(
                    *dst,
                    box_val,
                    "RuntimeDataBox",
                    method.clone(),
                    args,
                    *effects,
                    crate::mir::definitions::call_unified::TypeCertainty::Union,
                ))?;
            }
            CoreEffectPlan::GlobalCall { dst, func, args } => {
                builder.emit_unified_call(*dst, CallTarget::Global(func.clone()), args.clone())?;
            }
            CoreEffectPlan::ValueCall { dst, callee, args } => {
                builder.emit_unified_call(*dst, CallTarget::Value(*callee), args.clone())?;
            }
            CoreEffectPlan::ExternCall {
                dst,
                iface_name,
                method_name,
                args,
                effects,
            } => {
                builder.emit_extern_call_with_effects(
                    iface_name,
                    method_name,
                    args.clone(),
                    *dst,
                    *effects,
                )?;
            }
            CoreEffectPlan::NewBox { dst, box_type, args } => {
                builder.emit_instruction(MirInstruction::NewBox {
                    dst: *dst,
                    box_type: box_type.clone(),
                    args: args.clone(),
                })?;
            }
            CoreEffectPlan::BinOp { dst, lhs, op, rhs } => {
                let emit_caller = std::panic::Location::caller();
                // LocalSSA: arithmetic operands must be defined in the current function and materialized
                // in the current block. In strict/dev selfhost gates, do not silently pass through
                // undefined/foreign ValueIds; fail-fast in LocalSSA to shorten diagnosis distance.
                let (lhs, rhs) = if crate::config::env::joinir_dev::strict_enabled()
                    && crate::config::env::joinir_dev::planner_required_enabled()
                {
                    use crate::mir::builder::ssa::local::{try_ensure, LocalKind};
                    let lhs = try_ensure(builder, *lhs, LocalKind::Arg).map_err(|e| {
                        format!(
                            "{} use=CoreEffectPlan::BinOp operand=lhs dst=%{} op={:?} emit_caller={}",
                            e, dst.0, op, emit_caller
                        )
                    })?;
                    let rhs = try_ensure(builder, *rhs, LocalKind::Arg).map_err(|e| {
                        format!(
                            "{} use=CoreEffectPlan::BinOp operand=rhs dst=%{} op={:?} emit_caller={}",
                            e, dst.0, op, emit_caller
                        )
                    })?;
                    (lhs, rhs)
                } else {
                    (builder.local_arg(*lhs), builder.local_arg(*rhs))
                };
                builder.emit_instruction(MirInstruction::BinOp {
                    dst: *dst,
                    lhs,
                    op: *op,
                    rhs,
                })?;
            }
            CoreEffectPlan::Compare { dst, lhs, op, rhs } => {
                // LocalSSA: compare operands must be defined in the current block.
                let mut lhs = *lhs;
                let mut rhs = *rhs;
                crate::mir::builder::ssa::local::finalize_compare(builder, &mut lhs, &mut rhs)?;
                builder.emit_instruction(MirInstruction::Compare {
                    dst: *dst,
                    lhs,
                    op: *op,
                    rhs,
                })?;
            }
            CoreEffectPlan::Select {
                dst,
                cond,
                then_val,
                else_val,
            } => {
                builder.emit_instruction(MirInstruction::Select {
                    dst: *dst,
                    cond: *cond,
                    then_val: *then_val,
                    else_val: *else_val,
                })?;
            }
            CoreEffectPlan::ExitIf { .. } => {
                return Err("[lowerer] ExitIf requires loop body context".to_string());
            }
            CoreEffectPlan::IfEffect { .. } => {
                return Err("[lowerer] IfEffect requires loop body context".to_string());
            }
            CoreEffectPlan::Copy { dst, src } => {
                builder.emit_instruction(MirInstruction::Copy {
                    dst: *dst,
                    src: *src,
                })?;
            }
        }
        Ok(())
    }

    pub(super) fn emit_exit_if(
        builder: &mut MirBuilder,
        cond: ValueId,
        exit: &CoreExitPlan,
        fallthrough_target: BasicBlockId,
        loop_stack: &mut [LoopFrame],
    ) -> Result<(), String> {
        use crate::mir::builder::emission::branch::emit_conditional;

        let _pre_branch_bb = builder
            .current_block
            .ok_or_else(|| "[lowerer] No current block for ExitIf".to_string())?;

        builder.ensure_block_exists(fallthrough_target)?;
        let mut cond_val = cond;
        crate::mir::builder::ssa::local::finalize_branch_cond(builder, &mut cond_val)?;

        match exit {
            CoreExitPlan::Return(Some(value)) => {
                let return_bb = builder.next_block_id();
                builder.ensure_block_exists(return_bb)?;
                emit_conditional(builder, cond_val, return_bb, fallthrough_target)?;
                builder.start_new_block(return_bb)?;
                builder.emit_instruction(MirInstruction::Return { value: Some(*value) })?;
            }
            CoreExitPlan::Return(None) => {
                return Err("[lowerer] ExitIf(Return) requires payload".to_string());
            }
            CoreExitPlan::Break(depth) => {
                let frame = Self::resolve_loop_frame(loop_stack, *depth)?;
                builder.ensure_block_exists(frame.break_target)?;
                emit_conditional(builder, cond_val, frame.break_target, fallthrough_target)?;
            }
            CoreExitPlan::BreakWithPhiArgs { depth, phi_args } => {
                let pre_bb = builder
                    .current_block
                    .ok_or_else(|| "[lowerer] No current block for ExitIf(BreakWithPhiArgs)".to_string())?;
                let frame = Self::resolve_loop_frame_mut(loop_stack, *depth)?;
                builder.ensure_block_exists(frame.break_target)?;
                for (dst, src) in phi_args {
                    frame
                        .break_phi_inputs
                        .entry(*dst)
                        .or_default()
                        .insert(pre_bb, *src);
                }
                emit_conditional(builder, cond_val, frame.break_target, fallthrough_target)?;
            }
            CoreExitPlan::Continue(depth) => {
                let frame = Self::resolve_loop_frame(loop_stack, *depth)?;
                builder.ensure_block_exists(frame.continue_target)?;
                emit_conditional(builder, cond_val, frame.continue_target, fallthrough_target)?;
            }
            CoreExitPlan::ContinueWithPhiArgs { depth, phi_args } => {
                let pre_bb = builder
                    .current_block
                    .ok_or_else(|| "[lowerer] No current block for ExitIf(ContinueWithPhiArgs)".to_string())?;
                let frame = Self::resolve_loop_frame_mut(loop_stack, *depth)?;
                builder.ensure_block_exists(frame.continue_target)?;
                let debug_ctx = debug_ctx::build(builder);
                for (dst, src) in phi_args {
                    if let Some(debug_ctx) = &debug_ctx {
                        let incoming_def_bb = debug_ctx.def_blocks.get(src).copied();
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "{} fn={} origin=effect_emission pred_bb={:?} dst=%{} incoming=%{} incoming_def_bb={:?}",
                            debug_tags::TAG_STEP_PHI_INPUT_ADD,
                            debug_ctx.fn_name,
                            pre_bb,
                            dst.0,
                            src.0,
                            incoming_def_bb
                        ));
                    }
                    frame
                        .step_phi_inputs
                        .entry(*dst)
                        .or_default()
                        .insert(pre_bb, *src);
                }
                emit_conditional(builder, cond_val, frame.continue_target, fallthrough_target)?;
            }
        }

        Ok(())
    }
}
