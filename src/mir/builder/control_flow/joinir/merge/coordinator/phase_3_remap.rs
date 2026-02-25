//! Phase 3: Value remapping with debug logging
//!
//! This module handles ValueId remapping with reserved PHI dst protection
//! and provides debug logging for remapper state verification.

use super::super::value_remapper;
use crate::mir::builder::control_flow::joinir::trace;
use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::ValueId;
use std::collections::{BTreeSet, HashSet};

/// Execute Phase 3: Remap ValueIds with debug logging
pub fn execute(
    builder: &mut crate::mir::builder::MirBuilder,
    used_values: &BTreeSet<ValueId>,
    remapper: &mut JoinIrIdRemapper,
    reserved_value_ids: &HashSet<ValueId>,
    boundary: Option<&JoinInlineBoundary>,
    trace: &trace::JoinLoopTrace,
    debug: bool,
    verbose: bool,
) -> Result<(), String> {
    // Phase 3: Remap ValueIds (with reserved PHI dsts protection)
    // Phase 287 P0.2: Delegated to value_remapper module
    value_remapper::remap_values(builder, used_values, remapper, reserved_value_ids, debug)?;

    // Phase 177-3 DEBUG: Verify remapper state after Phase 3
    log_remapper_state_after_phase_3(used_values, remapper, boundary, trace, verbose);

    Ok(())
}

/// Log remapper state after Phase 3 for debugging
fn log_remapper_state_after_phase_3(
    used_values: &BTreeSet<ValueId>,
    remapper: &JoinIrIdRemapper,
    boundary: Option<&JoinInlineBoundary>,
    trace: &trace::JoinLoopTrace,
    verbose: bool,
) {
    trace.stderr_if("[DEBUG-177] === Remapper state after Phase 3 ===", verbose);
    trace.stderr_if(
        &format!("[DEBUG-177] used_values count: {}", used_values.len()),
        verbose,
    );

    for value_id in used_values {
        if let Some(remapped) = remapper.get_value(*value_id) {
            trace.stderr_if(
                &format!("[DEBUG-177]   JoinIR {:?} -> Host {:?}", value_id, remapped),
                verbose,
            );
        } else {
            trace.stderr_if(
                &format!("[DEBUG-177]   JoinIR {:?} -> NOT FOUND", value_id),
                verbose,
            );
        }
    }

    // Check condition_bindings specifically
    if let Some(boundary) = boundary {
        trace.stderr_if("[DEBUG-177] === Condition bindings check ===", verbose);
        for binding in &boundary.condition_bindings {
            let lookup_result = remapper.get_value(binding.join_value);
            trace.stderr_if(
                &format!(
                    "[DEBUG-177]   '{}': JoinIR {:?} -> {:?}",
                    binding.name, binding.join_value, lookup_result
                ),
                verbose,
            );
        }
    }
    trace.stderr_if("[DEBUG-177] ==============================", verbose);
}
