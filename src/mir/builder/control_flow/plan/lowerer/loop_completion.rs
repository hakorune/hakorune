//! Loop Completion Utilities
//!
//! This module contains loop completion logic for loop lowering.
//! Phase 29bq+: Extracted from loop_lowering.rs for better modularity.
//!
//! Responsibilities:
//! - Step 5: Frag emission with session structural lock
//! - Step 6: Update variable_map for final values
//! - Step 7: Setup after_bb for subsequent AST lowering
//! - Step 8: Return Void (pattern applied successfully)

use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::CoreLoopPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;

use crate::mir::builder::control_flow::plan::PlanBuildSession;

/// Emit loop frag (Step 5) with session structural lock
///
/// This function emits the CFG terminators using PlanBuildSession's
/// emit_and_seal method, which enforces structural locking.
pub fn emit_loop_frag(
    builder: &mut MirBuilder,
    session: &mut PlanBuildSession,
    frag: &Frag,
    loop_plan: &CoreLoopPlan,
    ctx: &LoopRouteContext,
) -> Result<(), String> {
    use crate::mir::builder::control_flow::joinir::trace;

    let trace_logger = trace::trace();
    let debug = ctx.debug;

    // Step 5: Emit Frag (terminators)
    // Phase 29bq+: Use session.emit_and_seal for structural lock
    // - from blocks auto-collected from frag.wires/branches
    // - assert_open before emit, seal after success
    if let Some(ref mut func) = builder.scope_ctx.current_function {
        session.emit_and_seal(func, frag)?;
    } else {
        return Err("[lowerer] current_function is None".to_string());
    }

    // Strict/dev+planner_required debug: Validate PHI inputs dominate predecessors
    if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
        let func = builder
            .scope_ctx
            .current_function
            .as_ref()
            .ok_or_else(|| "[lowerer] current_function is None".to_string())?;
        let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);
        let dominators = crate::mir::verification::utils::compute_dominators(func);
        for phi in &loop_plan.phis {
            for (pred, incoming) in &phi.inputs {
                let def_block = def_blocks.get(incoming).copied();
                let dominates = def_block
                    .map(|bb| dominators.dominates(bb, *pred))
                    .unwrap_or(false);
                if !dominates {
                    return Err(format!(
                        "[freeze:contract][loop_lowering/phi_input_not_available_in_pred] fn={} phi_tag={} phi_dst=%{} phi_block={:?} pred={:?} incoming=%{} incoming_def_bb={:?}",
                        func.signature.name,
                        phi.tag,
                        phi.dst.0,
                        phi.block,
                        pred,
                        incoming.0,
                        def_block
                    ));
                }
            }
        }
    }

    if debug {
        trace_logger.debug("lowerer/loop_generalized", "Frag emitted");
    }

    Ok(())
}

/// Finalize loop variables and return Void (Steps 6-8)
///
/// This function:
/// - Step 6: Updates variable_map with final values
/// - Step 7: Sets up after_bb for subsequent AST lowering
/// - Step 8: Returns Void value
pub fn finalize_loop_variables(
    builder: &mut MirBuilder,
    final_values: &[(String, ValueId)],
    after_bb: BasicBlockId,
    ctx: &LoopRouteContext,
) -> Result<Option<ValueId>, String> {
    use crate::mir::builder::control_flow::joinir::trace;
    use crate::mir::builder::emission::constant::emit_void;

    let trace_logger = trace::trace();
    let debug = ctx.debug;

    // Step 6: Update variable_map for final values
    for (name, value_id) in final_values {
        builder
            .variable_ctx
            .variable_map
            .insert(name.clone(), *value_id);
    }

    // Step 7: Setup after_bb for subsequent AST lowering
    builder.start_new_block(after_bb)?;

    // Step 8: Return Void (pattern applied successfully)
    let void_val = emit_void(builder)?;

    if debug {
        trace_logger.debug(
            "lowerer/loop_generalized",
            &format!("Loop complete, returning Void {:?}", void_val),
        );
    }

    Ok(Some(void_val))
}
