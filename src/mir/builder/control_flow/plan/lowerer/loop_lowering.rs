//! Phase 273 P3: Loop Lowering - CoreLoopPlan → MIR (8-step pipeline)
//!
//! # Responsibilities
//!
//! - Lower loop plans with frame management
//! - Execute 8-step generalized loop pipeline
//! - Emit blocks, effects, PHIs, and edge CFG
//! - Handle preheader setup and variable map finalization
//!
//! # Design
//!
//! - Frame setup with break/continue targets and PHI inputs
//! - 8-step pipeline for comprehensive loop lowering
//! - PlanBuildSession integration for structural lock
//! - Route-agnostic (all route knowledge is fixed in Normalizer)
//!
//! # Phase Comments
//!
//! - Phase 273 P3: Generalized fields (block_effects/phis/frag/final_values)
//! - Phase 29bq+: PlanBuildSession structural lock

use super::LoopFrame;
use crate::mir::builder::control_flow::plan::lowering_context::PlanLoweringContext;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CoreLoopPlan};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

// Loop preparation module (Phase 29bq+: extracted for modularity)
use super::loop_preparation;
// Block effect emission module (Phase 29bq+: extracted for modularity)
use super::block_effect_emission;
// Loop completion module (Phase 29bq+: extracted for modularity)
use super::loop_completion;
// PHI processing module (Phase 29bq+: extracted for modularity)
use super::phi_processing;

impl super::PlanLowerer {
    /// Loop: emit blocks, effects, PHI, and edge CFG
    ///
    /// This is route-agnostic. All route knowledge is in Normalizer.
    /// Phase 273 P3: Generalized fields are now REQUIRED (Fail-Fast).
    pub(super) fn lower_loop(
        builder: &mut MirBuilder,
        loop_plan: CoreLoopPlan,
        ctx: &dyn PlanLoweringContext,
        loop_stack: &mut Vec<LoopFrame>,
        port: &mut super::emission_port::CorePlanEffectEmissionPortV1<'_>,
    ) -> Result<Option<ValueId>, String> {
        use crate::mir::builder::control_flow::joinir::trace;

        let trace_logger = trace::trace();
        let debug = ctx.debug_enabled();

        if debug {
            trace_logger.debug(
                "lowerer/loop",
                &format!(
                    "Phase 273 P3: Lowering CoreLoopPlan for {}",
                    ctx.function_name()
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
            continue_target: loop_plan.continue_target,
            step_phi_inputs: BTreeMap::new(),
            break_phi_inputs: BTreeMap::new(),
        };
        loop_stack.push(frame);
        let result = Self::lower_loop_generalized(builder, loop_plan, ctx, loop_stack, port);
        loop_stack.pop();
        result
    }

    /// Phase 273 P3: Generalized loop lowering (SSOT)
    ///
    /// # Phase 29bq+: Structural Lock
    /// - Uses PlanBuildSession for sealing enforce
    /// - emit_frag → session.emit_and_seal (手順 SSOT)
    fn lower_loop_generalized(
        builder: &mut MirBuilder,
        loop_plan: CoreLoopPlan,
        ctx: &dyn PlanLoweringContext,
        loop_stack: &mut Vec<LoopFrame>,
        port: &mut super::emission_port::CorePlanEffectEmissionPortV1<'_>,
    ) -> Result<Option<ValueId>, String> {
        use super::super::PlanBuildSession;
        use crate::mir::builder::control_flow::joinir::trace;


        // Phase 29bq+: Create session for structural lock
        let mut session = PlanBuildSession::new();

        let trace_logger = trace::trace();
        let debug = ctx.debug_enabled();
        let pre_loop_map = builder.function_state.variable_ctx.variable_map.clone();

        // Phase 6: Prepare loop entry (preheader, body flattening, jump to entry)
        // SSOT: Delegated to loop_preparation::prepare_loop_entry()
        let (frag, body_effects, mut loop_plan) =
            loop_preparation::prepare_loop_entry(builder, loop_plan, ctx)?;

        // Phase 273 P3: Direct access (not Option - type system guarantees presence)
        // Clone block_effects to avoid borrow conflict with mutable loop_plan access in closure
        let block_effects: Vec<(BasicBlockId, Vec<CoreEffectPlan>)> =
            loop_plan.block_effects.clone();
        // Clone final_values to avoid borrow conflict with mutable loop_plan access in closure
        let final_values = loop_plan.final_values.clone();

        // A source session lends its transaction; otherwise this scope owns one.
        let source_transaction = port.prepare_source_loop_phis(builder, &loop_plan)?.is_some();
        let mut transaction = (!source_transaction).then(||
            crate::mir::builder::emission::phi_lifecycle::PhiTxn::begin("loop_lowerer"));
        let res: Result<(), String> = (|| {
            if let Some(txn) = transaction.as_mut() {
                for phi in &loop_plan.phis { builder.ensure_block_exists(phi.block)?; }
                for phi in &loop_plan.phis {
                    txn.define_provisional_phi(builder, phi.block, phi.dst,
                        &format!("loop_lowerer:step1.5:{}", phi.tag))?;
                }
            }
            // Step 2: Emit block effects in SSOT order (preheader, header, body, step)
            // SSOT: Delegated to block_effect_emission::emit_all_block_effects()
            block_effect_emission::emit_all_block_effects(
                builder,
                &loop_plan,
                &block_effects,
                body_effects,
                ctx,
                loop_stack,
                port,
            )?;

            // Step 3: Ensure non-effect blocks exist (after_bb, found_bb, etc.)
            builder.ensure_block_exists(loop_plan.after_bb)?;
            builder.ensure_block_exists(loop_plan.found_bb)?;

            // Step 3.5: Merge deferred continue-phi inputs into continue_target PHIs (strict/dev helper).
            // This supports multi-continue edges where the "next" carrier values differ per edge.
            // SSOT: Delegated to phi_processing::merge_deferred_phi_inputs()
            let debug_step_inputs =
                crate::config::env::joinir_dev::strict_planner_required_debug_enabled();
            let (debug_fn_name, debug_def_blocks) = if debug_step_inputs {
                let func = builder.function_state.current_function.as_ref();
                let fn_name = func
                    .map(|f| f.signature.name.as_str())
                    .unwrap_or("<none>")
                    .to_string();
                let def_blocks = func
                    .map(crate::mir::verification::utils::compute_def_blocks)
                    .unwrap_or_default();
                (Some(fn_name), Some(def_blocks))
            } else {
                (None, None)
            };
            phi_processing::merge_deferred_phi_inputs(
                &mut loop_plan,
                loop_stack,
                debug_step_inputs,
                debug_fn_name.as_deref(),
                debug_def_blocks.as_ref(),
            )?;

            // Step 4: Update PHI inputs (patch provisional PHIs)
            // PHIs were inserted provisionally in Step 1.5 with empty inputs.
            // Now update them with the actual inputs merged in Step 3.5.
            // SSOT: Delegated to phi_processing::patch_all_phis()
            port.patch_loop_phis(builder, &loop_plan, transaction.as_mut())?;

            if debug {
                trace_logger.debug(
                    "lowerer/loop_generalized",
                    &format!("PHI patched: {} PHIs", loop_plan.phis.len()),
                );
            }

            Ok(())
        })();

        if let Some(txn) = transaction {
            let labels: Vec<_> = loop_plan.phis.iter().map(|phi|
                (phi.block, phi.dst, phi.tag.as_str())).collect();
            let strict = crate::config::env::joinir_dev::strict_planner_required_enabled()
                || (crate::config::env::joinir_dev_enabled()
                    && crate::config::env::joinir_dev::planner_required_enabled());
            txn.finish_plan_scope(builder, res, &labels, strict)?;
        } else { res?; }

        // Step 5: Emit Frag (terminators)
        // SSOT: Delegated to loop_completion::emit_loop_frag()
        loop_completion::emit_loop_frag(builder, &mut session, &frag, &loop_plan, ctx)?;

        port.completed_loop(&loop_plan, builder.function_state.current_function.as_ref()
            .ok_or("[freeze:contract][loop-cond-after-role/function-missing]")?)?;

        // Steps 6-8: Finalize variables and return Void
        // Restore the outer lexical map before reapplying loop final_values.
        // Loop-body locals must not leak past after_bb just because their defs
        // were materialized while lowering the loop body.
        builder.function_state.variable_ctx.variable_map = pre_loop_map;
        // SSOT: Delegated to loop_completion::finalize_loop_variables()
        let out = loop_completion::finalize_loop_variables(
            builder,
            &final_values,
            loop_plan.after_bb,
            ctx,
        )?;
        crate::mir::builder::emission::value_lifecycle::verify_typed_values_are_defined(
            builder,
            "loop_lowerer:after_finalize_loop_variables",
        )?;
        Ok(out)
    }

    // Phase 273 P3: lower_loop_legacy() has been REMOVED
    // All loop routes must use generalized fields (block_effects/phis/frag/final_values)
    // Route-specific emission functions (emit_scan_with_init_edgecfg) are no longer used
}
