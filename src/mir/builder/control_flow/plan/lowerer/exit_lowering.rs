//! Phase 273 P3: Exit Lowering - Return/Break/Continue → MIR
//!
//! # Responsibilities
//!
//! - Lower Exit plans (Return, Break, Continue) to MIR instructions
//! - Resolve loop frames for Break/Continue depth
//! - Handle PHI args for BreakWithPhiArgs/ContinueWithPhiArgs
//!
//! # Design
//!
//! - Exit plans are leaf operations (no recursive CorePlan processing)
//! - Loop frame resolution validates depth and selects correct target
//! - PHI args are accumulated in LoopFrame for later merge

use super::{debug_ctx, debug_tags, LoopFrame};
use crate::mir::builder::control_flow::plan::CoreExitPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::{MirInstruction, ValueId};

impl super::PlanLowerer {
    /// Exit: emit Return (standalone); Break/Continue require loop context
    pub(super) fn lower_exit(
        builder: &mut MirBuilder,
        exit: CoreExitPlan,
        loop_stack: &mut Vec<LoopFrame>,
    ) -> Result<Option<ValueId>, String> {
        use crate::mir::builder::control_flow::joinir::trace;

        let trace_logger = trace::trace();
        let func_name = builder
            .scope_ctx
            .current_function
            .as_ref()
            .map(|func| func.signature.name.clone())
            .unwrap_or_else(|| "<unknown>".to_string());

        match exit {
            CoreExitPlan::Return(opt_val) => {
                builder.emit_instruction(MirInstruction::Return { value: opt_val })?;
                if let Some(current_bb) = builder.current_block {
                    trace_logger.debug(
                        "lowerer/term_set",
                        &format!(
                            "func={} bb={:?} term=Return",
                            func_name, current_bb
                        ),
                    );
                }
                Ok(opt_val)
            }
            CoreExitPlan::Break(depth) => {
                let frame = Self::resolve_loop_frame(loop_stack, depth)?;
                builder.ensure_block_exists(frame.break_target)?;
                builder.emit_instruction(MirInstruction::Jump {
                    target: frame.break_target,
                    edge_args: None,
                })?;
                if let Some(current_bb) = builder.current_block {
                    trace_logger.debug(
                        "lowerer/term_set",
                        &format!(
                            "func={} bb={:?} term=Jump target={:?}",
                            func_name, current_bb, frame.break_target
                        ),
                    );
                }
                Ok(None)
            }
            CoreExitPlan::BreakWithPhiArgs { depth, phi_args } => {
                let current_bb = builder
                    .current_block
                    .ok_or_else(|| "[lowerer] BreakWithPhiArgs without current block".to_string())?;
                let frame = Self::resolve_loop_frame_mut(loop_stack, depth)?;
                builder.ensure_block_exists(frame.break_target)?;
                let phi_args_len = phi_args.len();
                for (dst, src) in phi_args {
                    frame
                        .break_phi_inputs
                        .entry(dst)
                        .or_default()
                        .insert(current_bb, src);
                }
                builder.emit_instruction(MirInstruction::Jump {
                    target: frame.break_target,
                    edge_args: None,
                })?;
                trace_logger.debug(
                    "lowerer/term_set",
                    &format!(
                        "func={} bb={:?} term=Jump target={:?} phi_args.len={}",
                        func_name,
                        current_bb,
                        frame.break_target,
                        phi_args_len
                    ),
                );
                Ok(None)
            }
            CoreExitPlan::Continue(depth) => {
                let frame = Self::resolve_loop_frame(loop_stack, depth)?;
                builder.ensure_block_exists(frame.continue_target)?;
                builder.emit_instruction(MirInstruction::Jump {
                    target: frame.continue_target,
                    edge_args: None,
                })?;
                if let Some(current_bb) = builder.current_block {
                    trace_logger.debug(
                        "lowerer/term_set",
                        &format!(
                            "func={} bb={:?} term=Jump target={:?}",
                            func_name, current_bb, frame.continue_target
                        ),
                    );
                }
                Ok(None)
            }
            CoreExitPlan::ContinueWithPhiArgs { depth, phi_args } => {
                let current_bb = builder
                    .current_block
                    .ok_or_else(|| "[lowerer] ContinueWithPhiArgs without current block".to_string())?;
                let frame = Self::resolve_loop_frame_mut(loop_stack, depth)?;
                builder.ensure_block_exists(frame.continue_target)?;
                let phi_args_len = phi_args.len();
                let debug_ctx = debug_ctx::build(builder);
                for (dst, src) in phi_args {
                    // Always localize continue-phi incoming values at the emitting predecessor.
                    // This preserves dominance when `src` was defined on a sibling branch and
                    // allows pure defs (e.g. BinOp) to be rematerialized in-place.
                    let incoming = crate::mir::builder::ssa::local::try_ensure(
                        builder,
                        src,
                        crate::mir::builder::ssa::local::LocalKind::Arg,
                    )?;
                    if let Some(debug_ctx) = &debug_ctx {
                        let incoming_def_bb = debug_ctx.def_blocks.get(&incoming).copied();
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "{} fn={} origin=exit_lowering pred_bb={:?} dst=%{} incoming=%{} incoming_def_bb={:?}",
                            debug_tags::TAG_STEP_PHI_INPUT_ADD,
                            debug_ctx.fn_name,
                            current_bb,
                            dst.0,
                            incoming.0,
                            incoming_def_bb
                        ));
                    }
                    frame
                        .step_phi_inputs
                        .entry(dst)
                        .or_default()
                        .insert(current_bb, incoming);
                }
                builder.emit_instruction(MirInstruction::Jump {
                    target: frame.continue_target,
                    edge_args: None,
                })?;
                trace_logger.debug(
                    "lowerer/term_set",
                    &format!(
                        "func={} bb={:?} term=Jump target={:?} phi_args.len={}",
                        func_name,
                        current_bb,
                        frame.continue_target,
                        phi_args_len
                    ),
                );
                Ok(None)
            }
        }
    }

    pub(super) fn resolve_loop_frame<'a>(
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

    pub(super) fn resolve_loop_frame_mut<'a>(
        loop_stack: &'a mut [LoopFrame],
        depth: usize,
    ) -> Result<&'a mut LoopFrame, String> {
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
        Ok(&mut loop_stack[idx])
    }
}
