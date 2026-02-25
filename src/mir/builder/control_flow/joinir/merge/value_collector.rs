//! JoinIR Value Collector
//!
//! Collects all ValueIds used in JoinIR functions for remapping.
//! Also builds auxiliary maps for Call→Jump conversion.
//!
//! Phase 4 Extraction: Separated from merge_joinir_mir_blocks (lines 202-246)

use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::{MirInstruction, MirModule, ValueId};
use std::collections::BTreeMap;
use std::collections::BTreeSet; // Phase 222.5-E: HashMap → BTreeMap for determinism // Phase 222.5-E: HashMap → BTreeMap for determinism

/// Phase 2: Collect all ValueIds used across ALL functions (Phase 189)
///
/// Also build:
/// - value_to_func_name: Map of ValueId → function name (for Call→Jump conversion)
/// - function_params: Map of function name → parameter ValueIds (for tail call conversion)
pub(super) fn collect_values(
    mir_module: &MirModule,
    remapper: &JoinIrIdRemapper,
    debug: bool,
) -> Result<
    (
        BTreeSet<ValueId>,
        BTreeMap<ValueId, String>, // Phase 222.5-E: HashMap → BTreeMap for determinism
        BTreeMap<String, Vec<ValueId>>, // Phase 222.5-E: HashMap → BTreeMap for determinism
    ),
    String,
> {
    let trace = crate::mir::builder::control_flow::joinir::trace::trace();
    if debug {
        trace.stderr_if(
            "[cf_loop/joinir] Phase 189: Collecting value IDs from all functions",
            true,
        );
    }

    let mut used_values: BTreeSet<ValueId> = BTreeSet::new();
    // Phase 222.5-E: HashMap → BTreeMap for determinism
    let mut value_to_func_name: BTreeMap<ValueId, String> = BTreeMap::new();
    let mut function_params: BTreeMap<String, Vec<ValueId>> = BTreeMap::new();

    // Build function_entry_map for tracking function names
    // Phase 222.5-E: HashMap → BTreeMap for determinism
    let function_entry_map: BTreeMap<String, ()> = mir_module
        .functions
        .keys()
        .map(|name| (name.clone(), ()))
        .collect();

    for (func_name, func) in &mir_module.functions {
        // Phase 188-Impl-3: Collect function parameters for tail call conversion
        function_params.insert(func_name.clone(), func.params.clone());

        // Phase 256 P1.10 DEBUG: function params log (guarded)
        trace.stderr_if(
            &format!(
                "[cf_loop/joinir] Phase 256 P1.10 DEBUG: Function '{}' params: {:?}",
                func_name, func.params
            ),
            debug,
        );

        for block in func.blocks.values() {
            // Phase 189: Use remapper to collect values
            let block_values = remapper.collect_values_in_block(block);
            used_values.extend(block_values);

            // Phase 189: Track Const String instructions that define function names
            for inst in &block.instructions {
                if let MirInstruction::Const { dst, value } = inst {
                    if let crate::mir::types::ConstValue::String(s) = value {
                        if function_entry_map.contains_key(s) {
                            value_to_func_name.insert(*dst, s.clone());
                            // Phase 189 FIX: Also add to used_values so it gets remapped!
                            // Without this, subsequent instructions referencing dst will fail
                            used_values.insert(*dst);
                            if debug {
                                trace.stderr_if(
                                    &format!(
                                        "[cf_loop/joinir]   Found function name constant: {:?} = '{}'",
                                        dst, s
                                    ),
                                    true,
                                );
                            }
                        }
                    }
                }
            }
        }

        // Phase 33-15 / Phase 256 P1.10: DO NOT remap parameter ValueIds
        //
        // Reasoning: Parameters are implicitly defined at function entry in JoinIR.
        // When inlined into host MIR, if parameters are remapped, the remapped ValueIds
        // have no SSA definition (causing SSA-undef errors).
        //
        // Instead, parameters should remain with their original JoinIR ValueIds.
        // These are properly defined via:
        // 1. BoundaryInjector Copy (for entry function: host_input → join_param)
        // 2. Tail call Copy (for recursive calls: call_arg → param)
        //
        // Phase 256 P1.10: We must REMOVE params from used_values after block collection,
        // because collect_values_in_block() adds all operand ValueIds including params.
        // Without this removal, params get remapped and their remapped values are undefined.
        for param in &func.params {
            used_values.remove(param);
        }
    }

    if debug {
        trace.stderr_if(
            &format!(
                "[cf_loop/joinir]   Collected {} unique values",
                used_values.len()
            ),
            true,
        );
    }

    Ok((used_values, value_to_func_name, function_params))
}
