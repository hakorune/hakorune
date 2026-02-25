//! Phase 3.5: Parameter-to-PHI Override Logic
//!
//! This module handles the complex logic of overriding remapper entries
//! for function parameters to use PHI destination ValueIds.
//!
//! # Background
//!
//! JoinIR generates separate parameter ValueIds for each function:
//! - main(): boundary.join_inputs slots for (i_init, carrier1_init, ...)
//! - loop_step(): loop_step params for (i_param, carrier1_param, ...)
//!
//! The loop body uses loop_step's parameters, so we need to remap THOSE
//! to the header PHI dsts, not main()'s parameters.

use super::super::{boundary_carrier_layout, loop_header_phi_info};
use crate::mir::builder::control_flow::joinir::trace;
use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::ValueId;
use std::collections::{BTreeMap, HashSet};

/// Execute Phase 3.5: Override remapper for function parameters to use PHI dsts
pub fn execute(
    boundary: &JoinInlineBoundary,
    function_params: &BTreeMap<String, Vec<ValueId>>,
    remapper: &mut JoinIrIdRemapper,
    loop_header_phi_info: &mut loop_header_phi_info::LoopHeaderPhiInfo,
    trace: &trace::JoinLoopTrace,
    debug: bool,
    verbose: bool,
) -> Result<(), String> {
    let Some(loop_var_name) = &boundary.loop_var_name else {
        return Ok(());
    };

    // Phase 177-3 fix: Protect condition-ONLY bindings from being overridden to PHI dsts
    //
    // Problem: condition_bindings may contain:
    // 1. True condition-only variables (e.g., 'limit' in loop(i < limit)) - NOT carriers
    // 2. Body-only carriers added by Phase 176-5 (e.g., 'result') - ARE carriers
    //
    // We must ONLY protect (1), not (2), because:
    // - Condition-only vars should keep their HOST mapping (e.g., limit = %8)
    // - Body-only carriers MUST be remapped to PHI dsts (e.g., result = %24)
    //
    // Solution: Protect condition_bindings that are NOT in exit_bindings (i.e., not carriers)
    let carrier_names: HashSet<&str> = boundary
        .exit_bindings
        .iter()
        .map(|eb| eb.carrier_name.as_str())
        .collect();

    let condition_binding_ids: HashSet<ValueId> = boundary
        .condition_bindings
        .iter()
        .filter(|cb| !carrier_names.contains(cb.name.as_str()))
        .map(|cb| cb.join_value)
        .collect();

    if !condition_binding_ids.is_empty() {
        trace.stderr_if(
            &format!(
                "[cf_loop/joinir] Phase 177-3: Protected ValueIds (condition-only, not carriers): {:?}",
                condition_binding_ids
            ),
            verbose,
        );
        for cb in &boundary.condition_bindings {
            let is_carrier = carrier_names.contains(cb.name.as_str());
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir] Phase 177-3:   '{}': JoinIR {:?} (carrier={})",
                    cb.name, cb.join_value, is_carrier
                ),
                verbose,
            );
        }
    }

    let canonical_main = crate::mir::join_ir::lowering::canonical_names::MAIN;
    let canonical_loop_step = crate::mir::join_ir::lowering::canonical_names::LOOP_STEP;
    let main_func_name = if function_params.contains_key(canonical_main) {
        canonical_main
    } else {
        "join_func_0"
    };
    let loop_step_func_name = if function_params.contains_key(canonical_loop_step) {
        canonical_loop_step
    } else {
        "join_func_1"
    };

    if function_params.get(main_func_name).is_none() {
        trace.stderr_if(
            &format!(
                "[cf_loop/joinir] WARNING: function_params.get('{}') returned None. Available keys: {:?}",
                main_func_name,
                function_params.keys().collect::<Vec<_>>()
            ),
            verbose,
        );
    }

    let layout = boundary_carrier_layout::BoundaryCarrierLayout::from_boundary(boundary);
    let layout_names = layout.ordered_names();

    // Process main function parameters
    process_main_params(
        function_params,
        main_func_name,
        &layout_names,
        loop_header_phi_info,
        &condition_binding_ids,
        remapper,
        trace,
        verbose,
    );

    // Phase 177-3-B: Handle body-only carriers
    process_body_only_carriers(
        boundary,
        loop_header_phi_info,
        &condition_binding_ids,
        remapper,
        trace,
        verbose,
    );

    // Process loop_step function parameters
    process_loop_step_params(
        boundary,
        function_params,
        loop_step_func_name,
        &layout_names,
        loop_header_phi_info,
        &condition_binding_ids,
        remapper,
        trace,
        verbose,
    );

    // Fallback if neither main nor loop_step found
    if function_params.get(main_func_name).is_none()
        && function_params.get(loop_step_func_name).is_none()
    {
        process_fallback(
            boundary,
            loop_var_name,
            &layout_names,
            loop_header_phi_info,
            &condition_binding_ids,
            remapper,
            trace,
            debug,
            verbose,
        );
    }

    // Phase 177-3 DEBUG: Check remapper after Phase 33-21 overrides
    log_remapper_after_overrides(boundary, remapper, trace, verbose);

    Ok(())
}

/// Process main function parameters
fn process_main_params(
    function_params: &BTreeMap<String, Vec<ValueId>>,
    main_func_name: &str,
    layout_names: &[&str],
    loop_header_phi_info: &loop_header_phi_info::LoopHeaderPhiInfo,
    condition_binding_ids: &HashSet<ValueId>,
    remapper: &mut JoinIrIdRemapper,
    trace: &trace::JoinLoopTrace,
    verbose: bool,
) {
    let Some(main_params) = function_params.get(main_func_name) else {
        return;
    };

    trace.stderr_if(
        &format!(
            "[DEBUG-177] Phase 33-21: main ({}) params: {:?}",
            main_func_name, main_params
        ),
        verbose,
    );
    trace.stderr_if(
        &format!(
            "[DEBUG-177] Phase 33-21: carrier_phis count: {}, names: {:?}",
            loop_header_phi_info.carrier_phis.len(),
            loop_header_phi_info
                .carrier_phis
                .iter()
                .map(|(n, _)| n.as_str())
                .collect::<Vec<_>>()
        ),
        verbose,
    );

    // Map main's parameters to header PHI dsts.
    for (idx, &main_param) in main_params.iter().enumerate() {
        let Some(carrier_name) = layout_names.get(idx) else {
            continue;
        };
        let Some(entry) = loop_header_phi_info.carrier_phis.get(*carrier_name) else {
            continue;
        };

        // Phase 177-3: Don't override condition_bindings
        if condition_binding_ids.contains(&main_param) {
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir] Phase 177-3: Skipping override for condition_binding {:?} ('{}')",
                    main_param, carrier_name
                ),
                verbose,
            );
            continue;
        }

        trace.stderr_if(
            &format!(
                "[DEBUG-177] Phase 33-21: REMAP main param[{}] {:?} -> {:?} (carrier '{}')",
                idx, main_param, entry.phi_dst, carrier_name
            ),
            verbose,
        );
        remapper.set_value(main_param, entry.phi_dst);
    }
}

/// Process body-only carriers (Phase 177-3-B)
fn process_body_only_carriers(
    boundary: &JoinInlineBoundary,
    loop_header_phi_info: &loop_header_phi_info::LoopHeaderPhiInfo,
    condition_binding_ids: &HashSet<ValueId>,
    remapper: &mut JoinIrIdRemapper,
    trace: &trace::JoinLoopTrace,
    verbose: bool,
) {
    // These are carriers in carrier_phis that are NOT in main function params.
    // They appear in condition_bindings (added by Phase 176-5) but need PHI remapping.
    for (carrier_name, entry) in &loop_header_phi_info.carrier_phis {
        // Check if this carrier has a condition_binding
        if let Some(binding) = boundary
            .condition_bindings
            .iter()
            .find(|cb| cb.name == *carrier_name)
        {
            // Skip if it's a true condition-only variable (already protected above)
            if condition_binding_ids.contains(&binding.join_value) {
                continue;
            }
            // This is a body-only carrier - remap it to PHI dst
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir] Phase 177-3-B: Body-only carrier '{}': JoinIR {:?} -> PHI {:?}",
                    carrier_name, binding.join_value, entry.phi_dst
                ),
                verbose,
            );
            remapper.set_value(binding.join_value, entry.phi_dst);
        }
    }
}

/// Process loop_step function parameters
fn process_loop_step_params(
    boundary: &JoinInlineBoundary,
    function_params: &BTreeMap<String, Vec<ValueId>>,
    loop_step_func_name: &str,
    layout_names: &[&str],
    loop_header_phi_info: &loop_header_phi_info::LoopHeaderPhiInfo,
    condition_binding_ids: &HashSet<ValueId>,
    remapper: &mut JoinIrIdRemapper,
    trace: &trace::JoinLoopTrace,
    verbose: bool,
) {
    // DEBUG-177: Always log function_params keys to diagnose multi-carrier issue
    trace.stderr_if(
        &format!(
            "[DEBUG-177] Phase 33-21: function_params keys: {:?}",
            function_params.keys().collect::<Vec<_>>()
        ),
        verbose,
    );

    if function_params.get(loop_step_func_name).is_none() {
        trace.stderr_if(
            &format!(
                "[cf_loop/joinir] WARNING: function_params.get('{}') returned None. Available keys: {:?}",
                loop_step_func_name,
                function_params.keys().collect::<Vec<_>>()
            ),
            verbose,
        );
    }

    let Some(loop_step_params) = function_params.get(loop_step_func_name) else {
        return;
    };

    // DEBUG-177: Always log loop_step params
    trace.stderr_if(
        &format!(
            "[DEBUG-177] Phase 33-21: loop_step ({}) params: {:?}",
            loop_step_func_name, loop_step_params
        ),
        verbose,
    );

    // Phase 177-FIX: Process loop_step params but skip if already mapped
    for loop_step_param in loop_step_params {
        // Phase 177-3: Don't override condition_bindings
        if condition_binding_ids.contains(loop_step_param) {
            trace.stderr_if(
                &format!(
                    "[DEBUG-177] Phase 177-FIX: Skipping condition_binding {:?}",
                    loop_step_param
                ),
                verbose,
            );
            continue;
        }

        // Check if this param was already handled by Phase 177-3-B
        let already_mapped = boundary.condition_bindings.iter().any(|cb| {
            cb.join_value == *loop_step_param
                && loop_header_phi_info
                    .carrier_phis
                    .iter()
                    .any(|(name, _)| name == &cb.name)
        });
        if already_mapped {
            trace.stderr_if(
                &format!(
                    "[DEBUG-177] Phase 177-FIX: Skipping {:?} (already mapped by Phase 177-3-B)",
                    loop_step_param
                ),
                verbose,
            );
            continue;
        }

        // Phase 29ag P0: Use BoundaryCarrierLayout for index-based matching.
        if let Some(param_idx) = loop_step_params.iter().position(|p| p == loop_step_param) {
            // Map params[i] to carrier_order[i]
            if let Some(carrier_name) = layout_names.get(param_idx) {
                let Some(entry) = loop_header_phi_info.carrier_phis.get(*carrier_name) else {
                    continue;
                };
                trace.stderr_if(
                    &format!(
                        "[DEBUG-177] Phase 177-STRUCT-2: REMAP loop_step param[{}] {:?} -> {:?} (carrier '{}')",
                        param_idx, loop_step_param, entry.phi_dst, carrier_name
                    ),
                    verbose,
                );
                remapper.set_value(*loop_step_param, entry.phi_dst);
            }
        }
    }
}

/// Fallback: Use boundary.join_inputs for remap when function params not found
fn process_fallback(
    boundary: &JoinInlineBoundary,
    loop_var_name: &str,
    layout_names: &[&str],
    loop_header_phi_info: &loop_header_phi_info::LoopHeaderPhiInfo,
    condition_binding_ids: &HashSet<ValueId>,
    remapper: &mut JoinIrIdRemapper,
    trace: &trace::JoinLoopTrace,
    debug: bool,
    verbose: bool,
) {
    if let Some(phi_dst) = loop_header_phi_info.get_carrier_phi(loop_var_name) {
        let join_ids = boundary.join_inputs.as_slice();
        let loop_var_idx = layout_names
            .iter()
            .position(|name| *name == loop_var_name);
        if let Some(idx) = loop_var_idx {
            if let Some(&join_value_id) = join_ids.get(idx) {
                // Phase 177-3: Don't override condition_bindings
                if !condition_binding_ids.contains(&join_value_id) {
                    remapper.set_value(join_value_id, phi_dst);
                    trace.stderr_if(
                        &format!(
                            "[cf_loop/joinir] Phase 33-16 fallback: Override remap {:?} -> {:?} (PHI dst)",
                            join_value_id, phi_dst
                        ),
                        debug,
                    );
                } else {
                    trace.stderr_if(
                        &format!(
                            "[cf_loop/joinir] Phase 177-3 fallback: Skipping override for condition_binding {:?}",
                            join_value_id
                        ),
                        verbose,
                    );
                }
            }
        }
    }

    // Phase 29ag P1: Use boundary.join_inputs for remap instead of ValueId(idx).
    let join_ids = boundary.join_inputs.as_slice();
    for (idx, carrier_name) in layout_names.iter().enumerate() {
        if *carrier_name == loop_var_name {
            continue;
        }
        let Some(entry) = loop_header_phi_info.carrier_phis.get(*carrier_name) else {
            continue;
        };
        let Some(&join_value_id) = join_ids.get(idx) else {
            continue;
        };
        // Phase 177-3: Don't override condition_bindings
        if !condition_binding_ids.contains(&join_value_id) {
            remapper.set_value(join_value_id, entry.phi_dst);
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir] Phase 33-20 fallback: Override remap {:?} -> {:?} (carrier '{}' PHI dst)",
                    join_value_id, entry.phi_dst, carrier_name
                ),
                debug,
            );
        } else {
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir] Phase 177-3 fallback: Skipping override for condition_binding {:?} ('{}')",
                    join_value_id, carrier_name
                ),
                verbose,
            );
        }
    }
}

/// Log remapper state after Phase 33-21 overrides
fn log_remapper_after_overrides(
    boundary: &JoinInlineBoundary,
    remapper: &JoinIrIdRemapper,
    trace: &trace::JoinLoopTrace,
    verbose: bool,
) {
    trace.stderr_if(
        "[DEBUG-177] === Remapper state after Phase 33-21 ===",
        verbose,
    );
    for binding in &boundary.condition_bindings {
        let lookup_result = remapper.get_value(binding.join_value);
        trace.stderr_if(
            &format!(
                "[DEBUG-177]   '{}': JoinIR {:?} -> {:?} (after 33-21)",
                binding.name, binding.join_value, lookup_result
            ),
            verbose,
        );
    }
}
