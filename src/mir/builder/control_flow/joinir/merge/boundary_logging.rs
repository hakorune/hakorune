//! Phase 287 P0.5: Boundary logging consolidation
//!
//! Consolidates all trace.stderr_if() calls related to boundary information
//! into a single module for consistent logging.
//!
//! SSOT Principle: Only use trace.stderr_if(..., debug/verbose) - NO constant logs
//! to avoid noise in quick smoke tests.

use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;

/// Log detailed boundary information (verbose mode only)
///
/// Logs:
/// - join_inputs / host_inputs
/// - exit_bindings (carrier mappings)
/// - condition_bindings (if any)
/// - carrier_info (if present)
pub(super) fn log_boundary_info(
    boundary: Option<&JoinInlineBoundary>,
    trace: &super::super::trace::JoinLoopTrace,
    verbose: bool,
) {
    if !verbose {
        return;
    }

    if let Some(boundary) = boundary {
        // Exit bindings summary
        let exit_summary: Vec<String> = boundary
            .exit_bindings
            .iter()
            .map(|b| {
                format!(
                    "{}: join {:?} → host {:?} ({:?})",
                    b.carrier_name, b.join_exit_value, b.host_slot, b.role
                )
            })
            .collect();

        // Condition bindings summary
        let cond_summary: Vec<String> = boundary
            .condition_bindings
            .iter()
            .map(|b| {
                format!(
                    "{}: host {:?} → join {:?}",
                    b.name, b.host_value, b.join_value
                )
            })
            .collect();

        // Log join_inputs / host_inputs
        trace.stderr_if(
            &format!(
                "[cf_loop/joinir] Boundary join_inputs={:?} host_inputs={:?}",
                boundary.join_inputs, boundary.host_inputs
            ),
            verbose,
        );

        // Log exit_bindings
        trace.stderr_if(
            &format!(
                "[cf_loop/joinir] Boundary exit_bindings ({}): {}",
                boundary.exit_bindings.len(),
                exit_summary.join(", ")
            ),
            verbose,
        );

        // Log condition_bindings (if any)
        if !cond_summary.is_empty() {
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir] Boundary condition_bindings ({}): {}",
                    cond_summary.len(),
                    cond_summary.join(", ")
                ),
                verbose,
            );
        }

        // Log carrier_info (if present)
        if let Some(ci) = &boundary.carrier_info {
            let carriers: Vec<String> = ci.carriers.iter().map(|c| c.name.clone()).collect();
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir] Boundary carrier_info: loop_var='{}', carriers={:?}",
                    ci.loop_var_name, carriers
                ),
                verbose,
            );
        }
    } else {
        trace.stderr_if("[cf_loop/joinir] No boundary provided", verbose);
    }
}

/// Log merge completion summary (debug mode only)
pub(super) fn log_merge_complete(
    function_count: usize,
    exit_block_id: crate::mir::BasicBlockId,
    trace: &super::super::trace::JoinLoopTrace,
    debug: bool,
) {
    trace.stderr_if(
        &format!(
            "[cf_loop/joinir] Phase 189: Merge complete: {} functions merged, continuing from {:?}",
            function_count, exit_block_id
        ),
        debug,
    );
}
