//! Finalize: Jump, Block Switch, and Return Handling
//!
//! This module handles the final steps of JoinIR merge:
//! - Jump to entry block
//! - Switch to exit block for subsequent code
//! - Debug contract verification
//! - Expression result resolution and return

use super::super::{
    boundary_logging, expr_result_resolver, loop_header_phi_info, merge_result,
};
use crate::mir::builder::control_flow::joinir::trace;
use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

#[cfg(debug_assertions)]
use super::super::debug_assertions;

/// Execute finalization: Jump to entry, switch to exit, handle return
pub fn execute(
    builder: &mut crate::mir::builder::MirBuilder,
    boundary: Option<&JoinInlineBoundary>,
    merge_result: &merge_result::MergeResult,
    loop_header_phi_info: &loop_header_phi_info::LoopHeaderPhiInfo,
    merge_entry_block: BasicBlockId,
    exit_phi_result_id: Option<ValueId>,
    carrier_phis: &BTreeMap<String, ValueId>,
    remapper: &JoinIrIdRemapper,
    trace: &trace::JoinLoopTrace,
    debug: bool,
) -> Result<Option<ValueId>, String> {
    let exit_block_id = merge_result.exit_block_id;

    // Phase 256.7-fix: Use merge_entry_block for the Jump
    // This is the block where boundary Copies are injected (main's entry when condition_bindings exist).
    // The host should Jump here first, then main's tail call jumps to the loop header.
    let entry_block = merge_entry_block;

    trace.stderr_if(
        &format!(
            "[cf_loop/joinir] Phase 256.7-fix: Entry block (merge_entry_block): {:?}, loop_header={:?}",
            entry_block, loop_header_phi_info.header_block
        ),
        debug,
    );
    trace.stderr_if(
        &format!(
            "[cf_loop/joinir]   Current block before emit_jump: {:?}",
            builder.current_block
        ),
        debug,
    );
    trace.stderr_if(
        &format!(
            "[cf_loop/joinir]   Jumping to entry block: {:?}",
            entry_block
        ),
        debug,
    );

    crate::mir::builder::emission::branch::emit_jump(builder, entry_block)?;

    trace.stderr_if(
        &format!(
            "[cf_loop/joinir]   After emit_jump, current_block: {:?}",
            builder.current_block
        ),
        debug,
    );

    // Switch to exit block for subsequent code
    builder.start_new_block(exit_block_id)?;

    // Phase 287 P0.5: Delegated to boundary_logging module
    boundary_logging::log_merge_complete(
        boundary
            .map(|b| b.continuation_func_ids.len() + 1)
            .unwrap_or(1),
        exit_block_id,
        trace,
        debug,
    );

    // Phase 200-3: Verify JoinIR contracts (debug only)
    #[cfg(debug_assertions)]
    {
        if let Some(boundary) = boundary {
            if let Some(ref func) = builder.scope_ctx.current_function {
                debug_assertions::verify_joinir_contracts(
                    func,
                    loop_header_phi_info.header_block,
                    exit_block_id,
                    loop_header_phi_info,
                    boundary,
                );
            }
            trace.stderr_if(
                "[cf_loop/joinir] Phase 200-3: Contract verification passed",
                debug,
            );
        }
    }

    // Resolve and return expression result
    resolve_expr_result(
        boundary,
        exit_phi_result_id,
        carrier_phis,
        remapper,
        trace,
        debug,
    )
}

/// Resolve expression result value
///
/// Phase 246-EX-FIX: Handle loop variable expr_result separately from carrier expr_result
///
/// The loop variable (e.g., 'i') is returned via exit_phi_result_id, not carrier_phis.
/// Other carriers use carrier_phis. We need to check which case we're in.
fn resolve_expr_result(
    boundary: Option<&JoinInlineBoundary>,
    exit_phi_result_id: Option<ValueId>,
    carrier_phis: &BTreeMap<String, ValueId>,
    remapper: &JoinIrIdRemapper,
    trace: &trace::JoinLoopTrace,
    debug: bool,
) -> Result<Option<ValueId>, String> {
    let expr_result_value = if let Some(b) = boundary {
        if let Some(expr_result_id) = b.expr_result {
            // Check if expr_result is the loop variable
            if let Some(loop_var_name) = &b.loop_var_name {
                // Find the exit binding for the loop variable
                let loop_var_binding = b
                    .exit_bindings
                    .iter()
                    .find(|binding| binding.carrier_name == *loop_var_name);

                if let Some(binding) = loop_var_binding {
                    if binding.join_exit_value == expr_result_id {
                        // expr_result is the loop variable! Use exit_phi_result_id
                        trace.stderr_if(
                            &format!(
                                "[cf_loop/joinir] Phase 246-EX-FIX: expr_result {:?} is loop variable '{}', using exit_phi_result_id {:?}",
                                expr_result_id, loop_var_name, exit_phi_result_id
                            ),
                            debug,
                        );
                        exit_phi_result_id
                    } else {
                        // expr_result is not the loop variable, resolve as carrier
                        expr_result_resolver::ExprResultResolver::resolve(
                            Some(expr_result_id),
                            b.exit_bindings.as_slice(),
                            carrier_phis,
                            remapper,
                            debug,
                        )?
                    }
                } else {
                    // No loop variable binding, resolve normally
                    expr_result_resolver::ExprResultResolver::resolve(
                        Some(expr_result_id),
                        b.exit_bindings.as_slice(),
                        carrier_phis,
                        remapper,
                        debug,
                    )?
                }
            } else {
                // No loop variable name, resolve normally
                expr_result_resolver::ExprResultResolver::resolve(
                    Some(expr_result_id),
                    b.exit_bindings.as_slice(),
                    carrier_phis,
                    remapper,
                    debug,
                )?
            }
        } else {
            None
        }
    } else {
        None
    };

    // Return expr_result if present, otherwise fall back to exit_phi_result_id
    if let Some(resolved) = expr_result_value {
        trace.stderr_if(
            &format!(
                "[cf_loop/joinir] Phase 246-EX-FIX: Returning expr_result_value {:?}",
                resolved
            ),
            debug,
        );
        Ok(Some(resolved))
    } else {
        // Fallback: return exit_phi_result_id (for legacy patterns or carrier-only loops)
        trace.stderr_if(
            &format!(
                "[cf_loop/joinir] Phase 221-R: Returning exit_phi_result_id (fallback): {:?}",
                exit_phi_result_id
            ),
            debug && exit_phi_result_id.is_some(),
        );
        Ok(exit_phi_result_id)
    }
}
