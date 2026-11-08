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
//! - Function CFG owns fragment outgoing completion
//! - Route-agnostic (all route knowledge is fixed in Normalizer)
//!
//! # Phase Comments
//!
//! - Phase 273 P3: Generalized fields (block_effects/phis/frag/final_values)
//! - Fragment outgoing completion is shared with If/Root

use super::LoopFrame;
use crate::mir::builder::control_flow::plan::lowering_context::PlanLoweringContext;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CoreLoopPlan};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};

// Loop preparation module (Phase 29bq+: extracted for modularity)
use super::loop_preparation;
// Block effect emission module (Phase 29bq+: extracted for modularity)
use super::block_effect_emission;
// Loop completion module (Phase 29bq+: extracted for modularity)
use super::loop_completion;

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
        };
        loop_stack.push(frame);
        let result = Self::lower_loop_generalized(builder, loop_plan, ctx, loop_stack, port);
        loop_stack.pop();
        result
    }

    /// Phase 273 P3: Generalized loop lowering (SSOT)
    ///
    /// # Phase 29bq+: Structural Lock
    /// - Uses the function CFG for fragment outgoing completion
    /// - emit_frag → session.emit_and_seal (手順 SSOT)
    fn lower_loop_generalized(
        builder: &mut MirBuilder,
        loop_plan: CoreLoopPlan,
        ctx: &dyn PlanLoweringContext,
        loop_stack: &mut Vec<LoopFrame>,
        port: &mut super::emission_port::CorePlanEffectEmissionPortV1<'_>,
    ) -> Result<Option<ValueId>, String> {
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

        let source_scope = port.prepare_source_loop_phis(builder, &loop_plan)?.is_some();
        let shared = std::rc::Rc::clone(&builder.function_state.canonical_control);
        let result = (|| {
            shared.borrow_mut().reserve_plan_phis(builder, &loop_plan, source_scope)?;
            block_effect_emission::emit_all_block_effects(
                builder, &loop_plan, &block_effects, body_effects, ctx, loop_stack, port,
            )?;
            builder.ensure_block_exists(loop_plan.after_bb)?;
            builder.ensure_block_exists(loop_plan.found_bb)?;
            // The common SSA consumes actual predecessor witnesses. All
            // terminators are written before it closes the original PHIs.
            loop_completion::emit_loop_frag(builder, &frag, ctx)?;
            shared.borrow_mut().complete_plan_phis(builder, &mut loop_plan)?;
            loop_completion::verify_loop_phi_inputs_if_debug(builder, &loop_plan)?;
            shared.borrow_mut().require_plan_patched(&loop_plan)?;
            Ok(())
        })();
        result.map_err(|error| shared.borrow_mut().abort(builder, error))?;

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
