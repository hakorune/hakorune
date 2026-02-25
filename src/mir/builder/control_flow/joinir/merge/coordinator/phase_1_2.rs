//! Phase 1-2: Block allocation and value collection
//!
//! This module handles the initial phases of JoinIR merge:
//! - Phase 1: Allocate block IDs for all functions
//! - Phase 2: Collect values from all functions and handle condition/exit bindings

use super::super::{block_allocator, value_collector};
use crate::mir::builder::control_flow::joinir::trace;
use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::{BasicBlockId, MirModule, ValueId};
use std::collections::{BTreeMap, BTreeSet, HashSet};

/// Result of Phase 1-2 execution
pub struct Phase1_2Result {
    pub remapper: JoinIrIdRemapper,
    pub exit_block_id: BasicBlockId,
    pub used_values: BTreeSet<ValueId>,
    pub function_params: BTreeMap<String, Vec<ValueId>>,
}

/// Execute Phase 1-2: Block allocation and value collection
pub fn execute(
    builder: &mut crate::mir::builder::MirBuilder,
    mir_module: &MirModule,
    boundary: Option<&JoinInlineBoundary>,
    trace: &trace::JoinLoopTrace,
    debug: bool,
    verbose: bool,
) -> Result<Phase1_2Result, String> {
    // Phase 1: Allocate block IDs for all functions
    // Phase 177-3: block_allocator now returns exit_block_id to avoid conflicts
    let (mut remapper, exit_block_id) =
        block_allocator::allocate_blocks(builder, mir_module, debug)?;

    // Phase 2: Collect values from all functions
    let (mut used_values, _value_to_func_name, function_params) =
        value_collector::collect_values(mir_module, &remapper, debug)?;

    // Phase 171-fix + Phase 256.7-fix: Add condition_bindings' join_values to used_values for remapping
    // UNLESS they are function params. Params should NOT be remapped (they're defined
    // by boundary Copies and used directly in JoinIR body).
    if let Some(boundary) = boundary {
        // Build all_params set for checking (moved before condition_bindings loop)
        let all_params: HashSet<ValueId> = function_params
            .values()
            .flat_map(|params| params.iter().copied())
            .collect();

        // Phase 283 P0 DEBUG: Log condition_bindings count
        trace.stderr_if(
            &format!(
                "[cf_loop/joinir] Phase 283 P0 DEBUG: Processing {} condition_bindings",
                boundary.condition_bindings.len()
            ),
            debug,
        );

        for binding in &boundary.condition_bindings {
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir] Phase 283 P0 DEBUG: Checking binding '{}' join={:?}",
                    binding.name, binding.join_value
                ),
                debug,
            );

            if all_params.contains(&binding.join_value) {
                trace.stderr_if(
                    &format!(
                        "[cf_loop/joinir] Phase 256.7-fix: Skipping condition binding '{}' (JoinIR {:?} is a param)",
                        binding.name, binding.join_value
                    ),
                    debug,
                );
            } else {
                // Phase 283 P0 FIX: Ensure remapper has valid mapping (Fail-Fast)
                if let Some(host_id) = builder.variable_ctx.variable_map.get(&binding.name) {
                    // Variable exists in host context - map join_value to existing host_id
                    trace.stderr_if(
                        &format!(
                            "[cf_loop/joinir] Phase 283 P0: Condition binding '{}' JoinIR {:?} -> host {:?}",
                            binding.name, binding.join_value, host_id
                        ),
                        debug,
                    );
                    remapper.set_value(binding.join_value, *host_id);
                    used_values.insert(binding.join_value);
                } else {
                    // Fail-Fast: No host ValueId found -> surface root cause immediately
                    return Err(format!(
                        "[merge/phase2.1] Condition variable '{}' (join={:?}) has no host ValueId in variable_map. \
                        This indicates the value was not properly supplied by boundary builder or cond_env. \
                        Check: (1) boundary builder supplies all condition vars, (2) cond_env correctly tracks host ValueIds.",
                        binding.name, binding.join_value
                    ));
                }
            }
        }

        // Phase 172-3 + Phase 256 P1.10: Add exit_bindings' join_exit_values to used_values
        // UNLESS they are function params. Params should NOT be remapped (they're defined
        // by call site Copies and used directly in k_exit body).
        for binding in &boundary.exit_bindings {
            if all_params.contains(&binding.join_exit_value) {
                trace.stderr_if(
                    &format!(
                        "[cf_loop/joinir] Phase 256 P1.10: Skipping exit binding '{}' (JoinIR {:?} is a param)",
                        binding.carrier_name, binding.join_exit_value
                    ),
                    debug,
                );
            } else {
                trace.stderr_if(
                    &format!(
                        "[cf_loop/joinir] Phase 172-3: Adding exit binding '{}' JoinIR {:?} to used_values",
                        binding.carrier_name, binding.join_exit_value
                    ),
                    debug,
                );
                used_values.insert(binding.join_exit_value);
            }
        }
    }

    // Suppress unused variable warning
    let _ = verbose;

    Ok(Phase1_2Result {
        remapper,
        exit_block_id,
        used_values,
        function_params,
    })
}
