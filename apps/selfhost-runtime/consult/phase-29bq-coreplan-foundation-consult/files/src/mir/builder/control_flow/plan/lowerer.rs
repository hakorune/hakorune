//! Phase 273 P3: PlanLowerer - CorePlan → MIR 生成 (SSOT)
//!
//! # Responsibilities
//!
//! - Receive CorePlan from PlanNormalizer
//! - Emit MIR instructions using pre-allocated ValueIds
//! - No pattern-specific knowledge (pattern-agnostic)
//!
//! # Key Design Decision
//!
//! Lowerer processes CorePlan ONLY. It does not know about scan, split, or
//! any other pattern-specific semantics. All pattern knowledge is in Normalizer.
//!
//! # Phase 273 P3: SSOT Finalization
//!
//! - Generalized fields (block_effects/phis/frag/final_values) are now REQUIRED
//! - Legacy fallback has been removed (Fail-Fast on missing fields)
//! - Pattern-specific emission functions (emit_scan_with_init_edgecfg) no longer used

use super::{
    CoreBranchNPlan, CoreEffectPlan, CoreExitPlan, CoreIfPlan, CoreLoopPlan, CorePlan,
};
use super::branchn::branchn_to_if_chain;
use crate::mir::builder::calls::CallTarget;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirInstruction, ValueId};

/// Phase 273 P1: PlanLowerer - CorePlan → MIR 生成 (SSOT)
pub(in crate::mir::builder) struct PlanLowerer;

#[derive(Debug, Clone)]
struct LoopFrame {
    break_target: BasicBlockId,
    continue_target: BasicBlockId,
}

impl PlanLowerer {
    /// CorePlan を受け取り、MIR を生成
    ///
    /// # Arguments
    ///
    /// * `builder` - MIR builder (mutable access for instruction emission)
    /// * `plan` - CorePlan from Normalizer (pre-allocated ValueIds)
    /// * `ctx` - Loop pattern context for debug/func_name
    pub(in crate::mir::builder) fn lower(
        builder: &mut MirBuilder,
        plan: CorePlan,
        ctx: &LoopPatternContext,
    ) -> Result<Option<ValueId>, String> {
        let mut loop_stack = Vec::new();
        Self::lower_with_stack(builder, plan, ctx, &mut loop_stack)
    }

    fn lower_with_stack(
        builder: &mut MirBuilder,
        plan: CorePlan,
        ctx: &LoopPatternContext,
        loop_stack: &mut Vec<LoopFrame>,
    ) -> Result<Option<ValueId>, String> {
        match plan {
            CorePlan::Seq(plans) => Self::lower_seq(builder, plans, ctx, loop_stack),
            CorePlan::Loop(loop_plan) => Self::lower_loop(builder, loop_plan, ctx, loop_stack),
            CorePlan::If(if_plan) => Self::lower_if(builder, if_plan, ctx, loop_stack),
            CorePlan::BranchN(branch_plan) => Self::lower_branchn(builder, branch_plan, ctx, loop_stack),
            CorePlan::Effect(effect) => {
                if loop_stack.is_empty() {
                    Self::emit_effect(builder, &effect)?;
                } else {
                    Self::emit_effect_in_loop(builder, &effect, loop_stack)?;
                }
                Ok(None)
            }
            CorePlan::Exit(exit) => Self::lower_exit(builder, exit, loop_stack),
        }
    }

    /// Seq: process plans in order
    fn lower_seq(
        builder: &mut MirBuilder,
        plans: Vec<CorePlan>,
        ctx: &LoopPatternContext,
        loop_stack: &mut Vec<LoopFrame>,
    ) -> Result<Option<ValueId>, String> {
        let mut result = None;
        for plan in plans {
            result = Self::lower_with_stack(builder, plan, ctx, loop_stack)?;
            if builder.is_current_block_terminated() {
                break;
            }
        }
        Ok(result)
    }

    /// Loop: emit blocks, effects, PHI, and edge CFG
    ///
    /// This is pattern-agnostic. All pattern knowledge is in Normalizer.
    /// Phase 273 P3: Generalized fields are now REQUIRED (Fail-Fast).
    fn lower_loop(
        builder: &mut MirBuilder,
        loop_plan: CoreLoopPlan,
        ctx: &LoopPatternContext,
        loop_stack: &mut Vec<LoopFrame>,
    ) -> Result<Option<ValueId>, String> {
        use crate::mir::builder::control_flow::joinir::trace;

        let trace_logger = trace::trace();
        let debug = ctx.debug;

        if debug {
            trace_logger.debug(
                "lowerer/loop",
                &format!(
                    "Phase 273 P3: Lowering CoreLoopPlan for {}",
                    ctx.func_name
                ),
            );
        }

        // Phase 273 P3: Generalized fields are now struct fields (not Option)
        // No validation needed - type system guarantees presence

        if debug {
            trace_logger.debug("lowerer/loop", "Processing generalized fields (SSOT)");
        }

        let frame = LoopFrame {
            break_target: loop_plan.after_bb,
            continue_target: loop_plan.step_bb,
        };
        loop_stack.push(frame);
        let result = Self::lower_loop_generalized(builder, loop_plan, ctx, loop_stack);
        loop_stack.pop();
        result
    }

    /// If: emit Branch and lower then/else plans (standalone)
    fn lower_if(
        builder: &mut MirBuilder,
        if_plan: CoreIfPlan,
        ctx: &LoopPatternContext,
        loop_stack: &mut Vec<LoopFrame>,
    ) -> Result<Option<ValueId>, String> {
        use crate::mir::builder::emission::branch::{emit_conditional, emit_jump};

        let _pre_branch_bb = builder
            .current_block
            .ok_or_else(|| "[lowerer] No current block for CorePlan::If".to_string())?;

        let then_bb = builder.next_block_id();
        let else_bb = builder.next_block_id();
        let merge_bb = builder.next_block_id();

        builder.ensure_block_exists(then_bb)?;
        builder.ensure_block_exists(else_bb)?;
        builder.ensure_block_exists(merge_bb)?;

        let mut condition_val = if_plan.condition;
        crate::mir::builder::ssa::local::finalize_branch_cond(builder, &mut condition_val);
        emit_conditional(builder, condition_val, then_bb, else_bb)?;

        // then
        builder.start_new_block(then_bb)?;
        for plan in if_plan.then_plans {
            Self::lower_with_stack(builder, plan, ctx, loop_stack)?;
            if builder.is_current_block_terminated() {
                break;
            }
        }
        let then_reaches_merge = !builder.is_current_block_terminated();
        if then_reaches_merge {
            emit_jump(builder, merge_bb)?;
        }

        // else
        builder.start_new_block(else_bb)?;
        if let Some(else_plans) = if_plan.else_plans {
            for plan in else_plans {
                Self::lower_with_stack(builder, plan, ctx, loop_stack)?;
                if builder.is_current_block_terminated() {
                    break;
                }
            }
        }
        let else_reaches_merge = !builder.is_current_block_terminated();
        if else_reaches_merge {
            emit_jump(builder, merge_bb)?;
        }

        // merge (may be unreachable if both branches terminate)
        builder.start_new_block(merge_bb)?;
        Ok(None)
    }

    /// BranchN: rewrite into nested If chain and reuse lower_if.
    fn lower_branchn(
        builder: &mut MirBuilder,
        branch_plan: CoreBranchNPlan,
        ctx: &LoopPatternContext,
        loop_stack: &mut Vec<LoopFrame>,
    ) -> Result<Option<ValueId>, String> {
        let if_chain = branchn_to_if_chain(branch_plan)?;
        Self::lower_with_stack(builder, if_chain, ctx, loop_stack)
    }

    /// Exit: emit Return (standalone); Break/Continue require loop context
    fn lower_exit(
        builder: &mut MirBuilder,
        exit: CoreExitPlan,
        loop_stack: &mut Vec<LoopFrame>,
    ) -> Result<Option<ValueId>, String> {
        match exit {
            CoreExitPlan::Return(opt_val) => {
                builder.emit_instruction(MirInstruction::Return { value: opt_val })?;
                Ok(opt_val)
            }
            CoreExitPlan::Break(depth) => {
                let frame = Self::resolve_loop_frame(loop_stack, depth)?;
                builder.ensure_block_exists(frame.break_target)?;
                builder.emit_instruction(MirInstruction::Jump {
                    target: frame.break_target,
                    edge_args: None,
                })?;
                Ok(None)
            }
            CoreExitPlan::Continue(depth) => {
                let frame = Self::resolve_loop_frame(loop_stack, depth)?;
                builder.ensure_block_exists(frame.continue_target)?;
                builder.emit_instruction(MirInstruction::Jump {
                    target: frame.continue_target,
                    edge_args: None,
                })?;
                Ok(None)
            }
        }
    }

    /// Phase 273 P3: Generalized loop lowering (SSOT)
    fn lower_loop_generalized(
        builder: &mut MirBuilder,
        loop_plan: CoreLoopPlan,
        ctx: &LoopPatternContext,
        loop_stack: &mut Vec<LoopFrame>,
    ) -> Result<Option<ValueId>, String> {
        use crate::mir::builder::control_flow::joinir::trace;
        use crate::mir::builder::control_flow::edgecfg::api::emit_frag;

        let trace_logger = trace::trace();
        let debug = ctx.debug;

        let mut loop_plan = loop_plan;

        if let Some(current_bb) = builder.current_block {
            if loop_plan.preheader_bb != current_bb {
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

        // Phase 273 P3: Direct access (not Option - type system guarantees presence)
        let block_effects = &loop_plan.block_effects;
        let phis = &loop_plan.phis;
        let final_values = &loop_plan.final_values;

        let body_effects = Self::try_flatten_body_effects(&loop_plan.body)?;
        let body_has_control_flow = match body_effects.as_ref() {
            Some(effects) => super::coreloop_body_contract::has_control_flow_effect(effects),
            None => true,
        };

        if body_has_control_flow {
            loop_plan
                .frag
                .wires
                .retain(|wire| wire.from != loop_plan.body_bb);
        }

        let frag = &loop_plan.frag;

        // Step 1: Emit Jump from current block to loop entry
        if builder.current_block.is_some() {
            builder.emit_instruction(MirInstruction::Jump {
                target: frag.entry,
                edge_args: None,
            })?;
        }

        // Step 2: Emit block effects in SSOT order (preheader, header, body, step)
        // Note: Body effects are handled separately via body CorePlan
        for (block_id, effects) in block_effects {
            builder.start_new_block(*block_id)?;

            // Special handling for body block: emit body CorePlan instead of effects
            if *block_id == loop_plan.body_bb {
                if let Some(ref effects) = body_effects {
                    Self::emit_body_effects(builder, effects, loop_plan.step_bb, loop_stack)?;
                } else {
                    Self::lower_loop_body_plans(
                        builder,
                        &loop_plan.body,
                        ctx,
                        loop_stack,
                        loop_plan.step_bb,
                    )?;
                }
            } else {
                // Normal block: emit effects
                for effect in effects {
                    Self::emit_effect(builder, effect)?;
                }
            }
        }

        if debug {
            trace_logger.debug(
                "lowerer/loop_generalized",
                &format!("Block effects emitted: {} blocks", block_effects.len()),
            );
        }

        // Step 3: Ensure non-effect blocks exist (after_bb, found_bb, etc.)
        builder.ensure_block_exists(loop_plan.after_bb)?;
        builder.ensure_block_exists(loop_plan.found_bb)?;

        // Step 4: Insert PHIs
        use crate::mir::builder::emission::phi::insert_loop_phi;

        for phi in phis {
            insert_loop_phi(
                builder,
                phi.block,
                phi.dst,
                phi.inputs.clone(),
                &phi.tag,
            )?;
        }

        if debug {
            trace_logger.debug(
                "lowerer/loop_generalized",
                &format!("PHI inserted: {} PHIs", phis.len()),
            );
        }

        // Step 5: Emit Frag (terminators)
        if let Some(ref mut func) = builder.scope_ctx.current_function {
            emit_frag(func, frag)?;
        } else {
            return Err("[lowerer] current_function is None".to_string());
        }

        if debug {
            trace_logger.debug("lowerer/loop_generalized", "Frag emitted");
        }

        // Step 6: Update variable_map for final values
        for (name, value_id) in final_values {
            builder
                .variable_ctx
                .variable_map
                .insert(name.clone(), *value_id);
        }

        // Step 7: Setup after_bb for subsequent AST lowering
        builder.start_new_block(loop_plan.after_bb)?;

        // Step 8: Return Void (pattern applied successfully)
        use crate::mir::builder::emission::constant::emit_void;
        let void_val = emit_void(builder);

        if debug {
            trace_logger.debug(
                "lowerer/loop_generalized",
                &format!("Loop complete, returning Void {:?}", void_val),
            );
        }

        Ok(Some(void_val))
    }

    // Phase 273 P3: lower_loop_legacy() has been REMOVED
    // All patterns must use generalized fields (block_effects/phis/frag/final_values)
    // Pattern-specific emission functions (emit_scan_with_init_edgecfg) are no longer used

    /// Emit a single CoreEffectPlan as MirInstruction
    fn emit_effect(builder: &mut MirBuilder, effect: &CoreEffectPlan) -> Result<(), String> {
        match effect {
            CoreEffectPlan::Const { dst, value } => {
                builder.emit_instruction(MirInstruction::Const {
                    dst: *dst,
                    value: value.clone(),
                })?;
            }
            CoreEffectPlan::MethodCall { dst, object, method, args, effects } => {
                // P2: dst and effects are now specified by Normalizer
                builder.emit_instruction(MirInstruction::BoxCall {
                    dst: *dst,
                    box_val: *object,
                    method: method.clone(),
                    method_id: None,
                    args: args.clone(),
                    effects: *effects,
                })?;
            }
            CoreEffectPlan::GlobalCall { dst, func, args } => {
                builder.emit_unified_call(*dst, CallTarget::Global(func.clone()), args.clone())?;
            }
            CoreEffectPlan::ExternCall {
                dst,
                iface_name,
                method_name,
                args,
                effects,
            } => {
                builder.emit_instruction(MirInstruction::ExternCall {
                    dst: *dst,
                    iface_name: iface_name.clone(),
                    method_name: method_name.clone(),
                    args: args.clone(),
                    effects: *effects,
                })?;
            }
            CoreEffectPlan::BinOp { dst, lhs, op, rhs } => {
                builder.emit_instruction(MirInstruction::BinOp {
                    dst: *dst,
                    lhs: *lhs,
                    op: *op,
                    rhs: *rhs,
                })?;
            }
            CoreEffectPlan::Compare { dst, lhs, op, rhs } => {
                builder.emit_instruction(MirInstruction::Compare {
                    dst: *dst,
                    lhs: *lhs,
                    op: *op,
                    rhs: *rhs,
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
        }
        Ok(())
    }

    fn emit_exit_if(
        builder: &mut MirBuilder,
        cond: ValueId,
        exit: &CoreExitPlan,
        fallthrough_target: BasicBlockId,
        loop_stack: &[LoopFrame],
    ) -> Result<(), String> {
        use crate::mir::builder::emission::branch::emit_conditional;

        let _pre_branch_bb = builder
            .current_block
            .ok_or_else(|| "[lowerer] No current block for ExitIf".to_string())?;

        builder.ensure_block_exists(fallthrough_target)?;
        let mut cond_val = cond;
        crate::mir::builder::ssa::local::finalize_branch_cond(builder, &mut cond_val);

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
            CoreExitPlan::Continue(depth) => {
                let frame = Self::resolve_loop_frame(loop_stack, *depth)?;
                builder.ensure_block_exists(frame.continue_target)?;
                emit_conditional(builder, cond_val, frame.continue_target, fallthrough_target)?;
            }
        }

        Ok(())
    }

    fn emit_if_effect(
        builder: &mut MirBuilder,
        cond: ValueId,
        then_effects: &[CoreEffectPlan],
        else_effects: Option<&[CoreEffectPlan]>,
        fallthrough_target: BasicBlockId,
        loop_stack: &[LoopFrame],
    ) -> Result<(), String> {
        use crate::mir::builder::emission::branch::emit_conditional;

        let _pre_branch_bb = builder
            .current_block
            .ok_or_else(|| "[lowerer] No current block for IfEffect".to_string())?;

        let then_bb = builder.next_block_id();
        builder.ensure_block_exists(then_bb)?;
        builder.ensure_block_exists(fallthrough_target)?;

        let mut cond_val = cond;
        crate::mir::builder::ssa::local::finalize_branch_cond(builder, &mut cond_val);

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

    fn try_flatten_body_effects(
        plans: &[CorePlan],
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

    fn emit_effects_with_fallthrough(
        builder: &mut MirBuilder,
        effects: &[CoreEffectPlan],
        fallthrough_target: BasicBlockId,
        loop_stack: &[LoopFrame],
    ) -> Result<(), String> {
        let mut idx = 0;
        let mut terminated = false;
        while idx < effects.len() {
            let effect = &effects[idx];
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
                    Self::emit_effect(builder, effect)?;
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

    fn emit_body_effects(
        builder: &mut MirBuilder,
        effects: &[CoreEffectPlan],
        fallthrough_target: BasicBlockId,
        loop_stack: &[LoopFrame],
    ) -> Result<(), String> {
        let has_control_flow =
            super::coreloop_body_contract::has_control_flow_effect(effects);

        if !has_control_flow {
            for effect in effects {
                Self::emit_effect(builder, effect)?;
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

    fn emit_effect_in_loop(
        builder: &mut MirBuilder,
        effect: &CoreEffectPlan,
        loop_stack: &[LoopFrame],
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
                Self::emit_effect(builder, effect)?;
            }
        }
        Ok(())
    }

    fn lower_loop_body_plans(
        builder: &mut MirBuilder,
        plans: &[CorePlan],
        ctx: &LoopPatternContext,
        loop_stack: &mut Vec<LoopFrame>,
        fallthrough_target: BasicBlockId,
    ) -> Result<(), String> {
        for plan in plans {
            Self::lower_with_stack(builder, plan.clone(), ctx, loop_stack)?;
            if builder.is_current_block_terminated() {
                return Ok(());
            }
        }

        builder.ensure_block_exists(fallthrough_target)?;
        builder.emit_instruction(MirInstruction::Jump {
            target: fallthrough_target,
            edge_args: None,
        })?;
        Ok(())
    }

    fn resolve_loop_frame<'a>(
        loop_stack: &'a [LoopFrame],
        depth: usize,
    ) -> Result<&'a LoopFrame, String> {
        if depth == 0 {
            return Err("[lowerer] Break/Continue depth must be >= 1".to_string());
        }
        if depth > loop_stack.len() {
            return Err(format!(
                "[lowerer] Break/Continue depth {} exceeds loop depth {}",
                depth,
                loop_stack.len()
            ));
        }
        let idx = loop_stack.len() - depth;
        Ok(&loop_stack[idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::control_flow::plan::branchn::CoreBranchArmPlan;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::edgecfg::api::Frag;
    use crate::mir::{ConstValue, MirInstruction};

    fn make_ctx<'a>(condition: &'a ASTNode, body: &'a [ASTNode]) -> LoopPatternContext<'a> {
        LoopPatternContext::new(condition, body, "test_coreplan", false, false)
    }

    #[test]
    fn test_lower_exit_return_sets_terminator() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_exit".to_string());

        let ret_val = builder.alloc_value_for_test();
        builder
            .emit_for_test(MirInstruction::Const {
                dst: ret_val,
                value: ConstValue::Integer(1),
            })
            .expect("emit const");

        let cond = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let body: Vec<ASTNode> = vec![];
        let ctx = make_ctx(&cond, &body);

        let plan = CorePlan::Exit(CoreExitPlan::Return(Some(ret_val)));
        let result = PlanLowerer::lower(&mut builder, plan, &ctx);
        assert!(result.is_ok());

        let entry = builder.current_block_for_test().expect("entry block");
        let func = builder.scope_ctx.current_function.as_ref().expect("function");
        let block = func.get_block(entry).expect("block");
        assert!(
            matches!(block.terminator, Some(MirInstruction::Return { value: Some(v) }) if v == ret_val),
            "expected Return terminator"
        );
    }

    #[test]
    fn test_lower_if_emits_branch() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_if".to_string());

        let entry = builder.current_block_for_test().expect("entry block");
        let cond_val = builder.alloc_value_for_test();
        builder
            .emit_for_test(MirInstruction::Const {
                dst: cond_val,
                value: ConstValue::Bool(true),
            })
            .expect("emit const");

        let then_val = builder.alloc_value_for_test();
        let if_plan = CoreIfPlan {
            condition: cond_val,
            then_plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                dst: then_val,
                value: ConstValue::Integer(2),
            })],
            else_plans: None,
        };

        let cond = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let body: Vec<ASTNode> = vec![];
        let ctx = make_ctx(&cond, &body);

        let result = PlanLowerer::lower(&mut builder, CorePlan::If(if_plan), &ctx);
        assert!(result.is_ok());

        let func = builder.scope_ctx.current_function.as_ref().expect("function");
        let block = func.get_block(entry).expect("entry block");
        assert!(
            matches!(block.terminator, Some(MirInstruction::Branch { .. })),
            "expected Branch terminator"
        );
    }

    #[test]
    fn test_lower_branchn_does_not_fail() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_branchn".to_string());

        let cond1 = builder.alloc_value_for_test();
        builder
            .emit_for_test(MirInstruction::Const {
                dst: cond1,
                value: ConstValue::Bool(true),
            })
            .expect("emit cond1");

        let cond2 = builder.alloc_value_for_test();
        builder
            .emit_for_test(MirInstruction::Const {
                dst: cond2,
                value: ConstValue::Bool(false),
            })
            .expect("emit cond2");

        let arm1_val = builder.alloc_value_for_test();
        let arm2_val = builder.alloc_value_for_test();

        let branch_plan = CoreBranchNPlan {
            arms: vec![
                CoreBranchArmPlan {
                    condition: cond1,
                    plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                        dst: arm1_val,
                        value: ConstValue::Integer(1),
                    })],
                },
                CoreBranchArmPlan {
                    condition: cond2,
                    plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                        dst: arm2_val,
                        value: ConstValue::Integer(2),
                    })],
                },
            ],
            else_plans: None,
        };

        let cond = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let body: Vec<ASTNode> = vec![];
        let ctx = make_ctx(&cond, &body);

        let result = PlanLowerer::lower(&mut builder, CorePlan::BranchN(branch_plan), &ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_loop_body_seq_flattens() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_loop_body_seq".to_string());

        let preheader_bb = builder.next_block_id();
        let header_bb = builder.next_block_id();
        let body_bb = builder.next_block_id();
        let step_bb = builder.next_block_id();
        let after_bb = builder.next_block_id();
        let found_bb = builder.next_block_id();

        let eff1 = CoreEffectPlan::Const {
            dst: builder.alloc_value_for_test(),
            value: ConstValue::Integer(1),
        };
        let eff2 = CoreEffectPlan::Const {
            dst: builder.alloc_value_for_test(),
            value: ConstValue::Integer(2),
        };
        let eff3 = CoreEffectPlan::Const {
            dst: builder.alloc_value_for_test(),
            value: ConstValue::Integer(3),
        };

        let loop_plan = CoreLoopPlan {
            preheader_bb,
            header_bb,
            body_bb,
            step_bb,
            after_bb,
            found_bb,
            body: vec![CorePlan::Seq(vec![
                CorePlan::Effect(eff1),
                CorePlan::Seq(vec![CorePlan::Effect(eff2)]),
                CorePlan::Effect(eff3),
            ])],
            cond_loop: builder.alloc_value_for_test(),
            cond_match: builder.alloc_value_for_test(),
            block_effects: vec![
                (preheader_bb, vec![]),
                (header_bb, vec![]),
                (body_bb, vec![]),
                (step_bb, vec![]),
            ],
            phis: vec![],
            frag: Frag::new(header_bb),
            final_values: vec![],
        };

        let cond = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let body: Vec<ASTNode> = vec![];
        let ctx = make_ctx(&cond, &body);

        let result = PlanLowerer::lower(&mut builder, CorePlan::Loop(loop_plan), &ctx);
        assert!(result.is_ok());

        let func = builder.scope_ctx.current_function.as_ref().expect("function");
        let block = func.get_block(body_bb).expect("body block");
        let const_count = block
            .instructions
            .iter()
            .filter(|inst| matches!(inst, MirInstruction::Const { .. }))
            .count();
        assert_eq!(const_count, 3, "expected 3 Const effects in body");
    }

    #[test]
    fn test_lower_loop_body_if_effect_ok() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_loop_body_if_effect".to_string());

        let cond_val = builder.alloc_value_for_test();
        builder
            .emit_for_test(MirInstruction::Const {
                dst: cond_val,
                value: ConstValue::Bool(true),
            })
            .expect("emit cond");

        let effect = CoreEffectPlan::IfEffect {
            cond: cond_val,
            then_effects: vec![CoreEffectPlan::Const {
                dst: builder.alloc_value_for_test(),
                value: ConstValue::Integer(1),
            }],
            else_effects: None,
        };

        let preheader_bb = builder.next_block_id();
        let header_bb = builder.next_block_id();
        let body_bb = builder.next_block_id();
        let step_bb = builder.next_block_id();
        let after_bb = builder.next_block_id();
        let found_bb = builder.next_block_id();

        let loop_plan = CoreLoopPlan {
            preheader_bb,
            header_bb,
            body_bb,
            step_bb,
            after_bb,
            found_bb,
            body: vec![CorePlan::Effect(effect)],
            cond_loop: builder.alloc_value_for_test(),
            cond_match: builder.alloc_value_for_test(),
            block_effects: vec![
                (preheader_bb, vec![]),
                (header_bb, vec![]),
                (body_bb, vec![]),
                (step_bb, vec![]),
            ],
            phis: vec![],
            frag: Frag::new(header_bb),
            final_values: vec![],
        };

        let cond = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let body: Vec<ASTNode> = vec![];
        let ctx = make_ctx(&cond, &body);

        let result = PlanLowerer::lower(&mut builder, CorePlan::Loop(loop_plan), &ctx);
        assert!(result.is_ok());
    }
}
