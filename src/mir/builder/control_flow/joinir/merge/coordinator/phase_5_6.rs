//! Phase 5-6: Exit PHI Building and Boundary Reconnection
//!
//! This module handles:
//! - Phase 5: Build exit PHI (expr result only, not carrier PHIs)
//! - Phase 6: Reconnect boundary (if specified)

use super::super::{
    config::MergeConfig, contract_checks, exit_line, exit_phi_builder, merge_result,
};
use crate::mir::builder::control_flow::joinir::trace;
use crate::mir::join_ir::lowering::carrier_info::ExitReconnectMode;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Execute Phase 5-6: Build exit PHIs and reconnect boundary
///
/// Returns (exit_phi_result_id, exit_carrier_phis)
pub fn execute(
    builder: &mut crate::mir::builder::MirBuilder,
    boundary: Option<&JoinInlineBoundary>,
    merge_result: &merge_result::MergeResult,
    config: &MergeConfig,
    trace: &trace::JoinLoopTrace,
    debug: bool,
) -> Result<(Option<ValueId>, BTreeMap<String, ValueId>), String> {
    // Phase 5: Build exit PHI (expr result only, not carrier PHIs)
    // Phase 33-20: Carrier PHIs are now taken from header PHI info, not exit block
    // Phase 246-EX: REVERT Phase 33-20 - Use EXIT PHI dsts, not header PHI dsts!
    // Phase 131 P1.5: DirectValue mode completely skips PHI generation
    trace.stderr_if(
        &format!(
            "[cf_loop/joinir] Phase 131 P1.5 DEBUG: boundary={:?}, mode={:?}",
            boundary.is_some(),
            boundary.map(|b| b.exit_reconnect_mode)
        ),
        debug,
    );

    // Phase 131 P1.5: Check effective reconnect mode (config override > boundary mode)
    let boundary_mode = boundary.map(|b| b.exit_reconnect_mode);
    let effective_mode = config.exit_reconnect_mode.or(boundary_mode);
    let is_direct_value_mode = matches!(effective_mode, Some(ExitReconnectMode::DirectValue));

    // Phase 131 P1.5: Mode detection (dev-only visibility)
    trace.stderr_if(
        &format!(
            "[cf_loop/joinir] Phase 131 P1.5: boundary_mode={:?}, effective_mode={:?}, is_direct_value_mode={}",
            boundary_mode,
            effective_mode,
            is_direct_value_mode
        ),
        debug || config.dev_log,
    );

    let (exit_phi_result_id, exit_carrier_phis) = if is_direct_value_mode {
        // DirectValue mode: Skip PHI generation completely
        trace.stderr_if(
            "[cf_loop/joinir] Phase 131 P1.5: DirectValue mode - skipping exit PHI generation",
            debug,
        );
        (None, BTreeMap::new())
    } else {
        // Phi mode: Generate exit PHIs as usual
        trace.stderr_if(
            "[cf_loop/joinir] Phase 131 P1.5: Phi mode - generating exit PHIs",
            debug,
        );
        exit_phi_builder::build_exit_phi(
            builder,
            merge_result.exit_block_id,
            &merge_result.exit_phi_inputs,
            &merge_result.carrier_inputs,
            debug,
        )?
    };

    // Phase 118 P2: Contract check (Fail-Fast) - exit_bindings LoopState carriers must have exit PHIs.
    // Phase 131 P1.5: Skip this check in DirectValue mode
    if let Some(boundary) = boundary {
        if !is_direct_value_mode {
            contract_checks::verify_exit_bindings_have_exit_phis(boundary, &exit_carrier_phis)?;
        }
    }

    // Phase 118 P1: Dev-only carrier-phi SSOT logs (exit_bindings vs carrier_inputs vs exit_carrier_phis)
    // Phase 131 Task 6: Use config.dev_log instead of env check
    if config.dev_log {
        if let Some(boundary) = boundary {
            log_carrier_phi_ssot(boundary, merge_result, &exit_carrier_phis, trace);
        }
    }

    // Phase 246-EX: CRITICAL FIX - Use exit PHI dsts for variable_map reconnection
    //
    // **Why EXIT PHI, not HEADER PHI?**
    //
    // Header PHI represents the value at the BEGINNING of each iteration.
    // Exit PHI represents the FINAL value when leaving the loop (from any exit path).
    //
    // For Pattern 2 loops with multiple exit paths (natural exit + break):
    // - Header PHI: `%15 = phi [%3, bb7], [%42, bb14]` (loop variable at iteration start)
    // - Exit PHI:   `%5 = phi [%15, bb11], [%15, bb13]` (final value from exit paths)
    //
    // When we exit the loop, we want the FINAL value (%5), not the iteration-start value (%15).
    let carrier_phis = &exit_carrier_phis;

    trace.stderr_if(
        &format!(
            "[cf_loop/joinir] Phase 246-EX: Using EXIT PHI dsts for variable_map (not header): {:?}",
            carrier_phis
                .iter()
                .map(|(n, v)| (n.as_str(), v))
                .collect::<Vec<_>>()
        ),
        debug && !carrier_phis.is_empty(),
    );

    // Phase 6: Reconnect boundary (if specified)
    // Phase 197-B: Pass remapper to enable per-carrier exit value lookup
    // Phase 33-10-Refactor-P3: Delegate to ExitLineOrchestrator
    // Phase 246-EX: Now uses EXIT PHI dsts (reverted Phase 33-20)
    // Phase 131 P2: DirectValue mode SSOT uses MergeResult.remapped_exit_values
    let remapped_exit_values = merge_result.remapped_exit_values.clone();

    if let Some(boundary) = boundary {
        exit_line::ExitLineOrchestrator::execute(
            builder,
            boundary,
            carrier_phis,
            &remapped_exit_values, // Phase 131 P1.5: Now populated with exit PHI dsts
            debug,
        )?;
    }

    Ok((exit_phi_result_id, exit_carrier_phis))
}

/// Log carrier-phi SSOT information for debugging
fn log_carrier_phi_ssot(
    boundary: &JoinInlineBoundary,
    merge_result: &merge_result::MergeResult,
    exit_carrier_phis: &BTreeMap<String, ValueId>,
    trace: &trace::JoinLoopTrace,
) {
    let exit_binding_names: Vec<&str> = boundary
        .exit_bindings
        .iter()
        .map(|b| b.carrier_name.as_str())
        .collect();
    let carrier_input_names: Vec<&str> = merge_result
        .carrier_inputs
        .keys()
        .map(|s| s.as_str())
        .collect();
    let exit_phi_names: Vec<&str> = exit_carrier_phis.keys().map(|s| s.as_str()).collect();

    trace.stderr_if(
        &format!(
            "[joinir/phase118/dev] exit_bindings carriers={:?}",
            exit_binding_names
        ),
        true,
    );
    trace.stderr_if(
        &format!(
            "[joinir/phase118/dev] carrier_inputs keys={:?}",
            carrier_input_names
        ),
        true,
    );
    trace.stderr_if(
        &format!(
            "[joinir/phase118/dev] exit_carrier_phis keys={:?}",
            exit_phi_names
        ),
        true,
    );
}
