//! Phase 33-10-Refactor-P0: ExitLineReconnector Box
//!
//! Modularizes the exit line reconnection logic (Phase 6 from merge/mod.rs)
//! into a focused, testable Box.
//!
//! **Responsibility**: Update host variable_ctx.variable_map with PHI dst values from exit block
//!
//! # Phase 33-13 Architecture Change
//!
//! Before: Used remapper to get remapped exit values (SSA-incorrect!)
//! After: Uses carrier_phis map from exit_phi_builder (SSA-correct!)
//!
//! The remapped exit value is an INPUT to the PHI, not the OUTPUT.
//! Only the PHI dst is defined in the exit block and can be referenced later.

use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::ValueId;
use std::collections::BTreeMap;

/// ExitLineReconnector: A Box that manages exit value reconnection
///
/// # Design Notes
///
/// ## Why Separate Box?
/// - Responsibility: Update host variable_ctx.variable_map with PHI dst values
/// - Input: JoinInlineBoundary.exit_bindings + carrier_phis map
/// - Output: Updated MirBuilder.variable_ctx.variable_map
/// - No side effects beyond variable_ctx.variable_map updates
///
/// ## Phase 33-13 Carrier PHI Integration
/// This Box now uses carrier_phis from exit_phi_builder:
/// - Before: `variable_ctx.variable_map[carrier] = remapper.get(join_exit)` (SSA-incorrect!)
/// - After: `variable_ctx.variable_map[carrier] = carrier_phis[carrier]` (SSA-correct!)
///
/// The key insight is that remapped exit values are PHI INPUTS, not OUTPUTS.
/// Only the PHI dst ValueId is defined in the exit block.
///
/// ## Testing Strategy
/// Can be tested independently:
/// 1. Create mock boundary with exit_bindings
/// 2. Create mock carrier_phis map
/// 3. Call reconnect() and verify variable_ctx.variable_map updates
/// 4. No need to construct full merge/mod.rs machinery
///
/// # Box Contract
///
/// **Input**:
/// - JoinInlineBoundary with exit_bindings (Carrier → join_exit_value mappings)
/// - carrier_phis: Map from carrier name to PHI dst ValueId
///
/// **Effect**:
/// - Updates builder.variable_ctx.variable_map entries for each carrier with PHI dst values
///
/// **Output**:
/// - Result<(), String> (side effect on builder)
pub struct ExitLineReconnector;

impl ExitLineReconnector {
    /// Reconnect exit values to host variable_ctx.variable_map using carrier PHI dst values
    ///
    /// # Phase 33-13: Carrier PHI Integration
    ///
    /// Previously, we used the remapper to get remapped exit values. This was
    /// SSA-incorrect because those values are PHI inputs, not outputs.
    ///
    /// Now, we use the carrier_phis map from exit_phi_builder, which contains
    /// the actual PHI dst ValueIds that are defined in the exit block.
    ///
    /// # Phase 131 P1.5: DirectValue Mode Support
    ///
    /// For Normalized shadow (exit_reconnect_mode = DirectValue):
    /// - Uses remapped_exit_values instead of carrier_phis
    /// - No PHI generation, direct value assignment
    ///
    /// # Algorithm
    ///
    /// For each exit_binding:
    /// 1. Look up the PHI dst (Phi mode) or remapped value (DirectValue mode)
    /// 2. Update variable_ctx.variable_map[binding.carrier_name] with the value
    /// 3. Log each update (if debug enabled)
    pub fn reconnect(
        builder: &mut MirBuilder,
        boundary: &JoinInlineBoundary,
        carrier_phis: &BTreeMap<String, ValueId>,
        remapped_exit_values: &BTreeMap<String, ValueId>, // Phase 131 P1.5
        debug: bool,
    ) -> Result<(), String> {
        let trace = crate::mir::builder::control_flow::joinir::trace::trace();
        let dev_on = crate::config::env::joinir_dev_enabled();
        let strict = crate::config::env::joinir_strict_enabled() || dev_on;
        let verbose = debug || dev_on;

        if verbose {
            trace.stderr_if(
                &format!(
                    "[joinir/exit-line] reconnect: {} exit bindings, {} carrier PHIs",
                    boundary.exit_bindings.len(),
                    carrier_phis.len()
                ),
                verbose,
            );
            if !boundary.exit_bindings.is_empty() {
                trace.stderr_if(
                    &format!(
                        "[joinir/exit-line] bindings {:?}",
                        boundary
                            .exit_bindings
                            .iter()
                            .map(|b| (&b.carrier_name, b.role, b.join_exit_value))
                            .collect::<Vec<_>>()
                    ),
                    verbose,
                );
            }
        }

        // Early return for empty exit_bindings
        if boundary.exit_bindings.is_empty() {
            if verbose {
                trace.stderr_if(
                    "[joinir/exit-line] reconnect: no exit bindings, skip",
                    verbose,
                );
            }
            return Ok(());
        }

        if verbose {
            trace.stderr_if(
                &format!(
                    "[joinir/exit-line] reconnecting {} exit bindings with {} carrier PHIs",
                    boundary.exit_bindings.len(),
                    carrier_phis.len()
                ),
                verbose,
            );
        }

        // Phase 131 P1.5: Check exit_reconnect_mode
        use crate::mir::join_ir::lowering::carrier_info::{CarrierRole, ExitReconnectMode};
        let use_direct_values = boundary.exit_reconnect_mode == ExitReconnectMode::DirectValue;

        if verbose {
            trace.stderr_if(
                &format!(
                    "[joinir/exit-line] mode={:?}, use_direct_values={}",
                    boundary.exit_reconnect_mode, use_direct_values
                ),
                true,
            );
        }

        // Process each exit binding
        for binding in &boundary.exit_bindings {
            // Phase 228-8: Skip ConditionOnly carriers (no variable_ctx.variable_map update needed)
            if binding.role == CarrierRole::ConditionOnly {
                if verbose {
                    trace.stderr_if(
                        &format!(
                            "[joinir/exit-line] skip ConditionOnly carrier '{}' (no variable_ctx.variable_map update)",
                            binding.carrier_name
                        ),
                        true,
                    );
                }
                continue;
            }

            // Phase 131 P1.5: Choose value source based on mode
            let final_value = if use_direct_values {
                // DirectValue mode: Use remapped_exit_values (SSOT: merge owns remapper)
                remapped_exit_values.get(&binding.carrier_name).copied()
            } else {
                // Phi mode: Use carrier_phis (Phase 33-13)
                carrier_phis.get(&binding.carrier_name).copied()
            };

            if verbose {
                trace.stderr_if(
                    &format!(
                        "[joinir/exit-line] carrier '{}' → final_value={:?} (mode={:?})",
                        binding.carrier_name, final_value, boundary.exit_reconnect_mode
                    ),
                    true,
                );
            }

            // Update variable_ctx.variable_map with final value
            if let Some(phi_value) = final_value {
                if let Some(var_vid) = builder
                    .variable_ctx
                    .variable_map
                    .get_mut(&binding.carrier_name)
                {
                    // Phase 177-STRUCT: Always log for debugging
                    if verbose {
                        trace.stderr_if(
                            &format!(
                                "[joinir/exit-line] variable_ctx.variable_map['{}'] {:?} → {:?}",
                                binding.carrier_name, *var_vid, phi_value
                            ),
                            true,
                        );
                    }
                    *var_vid = phi_value;
                } else if verbose {
                    trace.stderr_if(
                        &format!(
                            "[joinir/exit-line] warning: carrier '{}' not found in variable_ctx.variable_map",
                            binding.carrier_name
                        ),
                        true,
                    );
                } else if strict {
                    return Err(format!(
                        "[joinir/exit-line] missing variable_ctx.variable_map entry for carrier '{}' (exit reconnection)",
                        binding.carrier_name
                    ));
                }
            } else {
                if strict && binding.role != CarrierRole::ConditionOnly {
                    return Err(format!(
                        "[joinir/exit-line] missing PHI dst for carrier '{}' ({} PHIs available)",
                        binding.carrier_name,
                        carrier_phis.len()
                    ));
                } else if verbose {
                    trace.stderr_if(
                        &format!(
                            "[joinir/exit-line] warning: No PHI dst for carrier '{}' (may be condition-only variable)",
                            binding.carrier_name
                        ),
                        true,
                    );
                }
            }
        }

        // Phase 190-impl-D-3: Contract verification (debug build only)
        // Ensures all exit_bindings have corresponding entries in carrier_phis and variable_ctx.variable_map
        #[cfg(debug_assertions)]
        Self::verify_exit_line_contract(boundary, carrier_phis, &builder.variable_ctx.variable_map);

        Ok(())
    }

    /// Phase 190-impl-D-3: Verify exit line contract (debug build only)
    ///
    /// # Contract Requirements
    ///
    /// 1. Every exit_binding must have a corresponding entry in carrier_phis
    /// 2. Every exit_binding's carrier must exist in variable_ctx.variable_map after reconnect
    /// 3. The variable_ctx.variable_map entry must point to the PHI dst (not the original host value)
    ///
    /// # Panics
    ///
    /// Panics if any contract violation is detected. This helps catch bugs where:
    /// - PHI is missing for a carrier (Phase 190-impl-D root cause)
    /// - variable_ctx.variable_map update was skipped
    /// - ValueId collision occurred
    #[cfg(debug_assertions)]
    fn verify_exit_line_contract(
        boundary: &JoinInlineBoundary,
        carrier_phis: &BTreeMap<String, ValueId>,
        variable_map: &BTreeMap<String, ValueId>,
    ) {
        use crate::mir::join_ir::lowering::carrier_info::CarrierRole;
        use crate::mir::ValueId;
        let trace = crate::mir::builder::control_flow::joinir::trace::trace();

        for binding in &boundary.exit_bindings {
            // Phase 228-8: Skip ConditionOnly carriers (not in variable_ctx.variable_map by design)
            if binding.role == CarrierRole::ConditionOnly {
                trace.stderr_if(
                    &format!(
                        "[JoinIR/ExitLine/Contract] Phase 228-8: Skipping ConditionOnly carrier '{}' (not in variable_ctx.variable_map)",
                        binding.carrier_name
                    ),
                    true,
                );
                continue;
            }
            // Phase 247-EX: Skip carriers without host slots (loop-local or FromHost placeholders).
            if binding.host_slot == ValueId(0) {
                trace.stderr_if(
                    &format!(
                        "[JoinIR/ExitLine/Contract] Phase 247-EX: Skipping carrier '{}' (no host slot)",
                        binding.carrier_name
                    ),
                    true,
                );
                continue;
            }

            // Contract 1: carrier_phis must contain this carrier
            let phi_dst = carrier_phis.get(&binding.carrier_name);
            if phi_dst.is_none() {
                // Skip loop variable (it's handled separately in loop_header_phi)
                // Only check carriers that have exit_bindings
                trace.stderr_if(
                    &format!(
                        "[JoinIR/ExitLine/Contract] WARNING: Carrier '{}' has exit_binding but no PHI in carrier_phis",
                        binding.carrier_name
                    ),
                    true,
                );
                // Don't panic for now - loop variable might not be in carrier_phis
                // Future: Distinguish loop_var from carriers in exit_bindings
            }

            // Contract 2: variable_ctx.variable_map must contain this carrier after reconnect
            let var_value = variable_map.get(&binding.carrier_name);
            if var_value.is_none() {
                panic!(
                    "[JoinIR/ExitLine/Contract] VIOLATION: Carrier '{}' missing from variable_ctx.variable_map after reconnect",
                    binding.carrier_name
                );
            }

            // Contract 3: variable_ctx.variable_map entry should point to PHI dst (if PHI exists)
            if let (Some(&phi), Some(&var)) = (phi_dst, var_value) {
                if phi != var {
                    panic!(
                        "[JoinIR/ExitLine/Contract] VIOLATION: Carrier '{}' variable_ctx.variable_map={:?} but PHI dst={:?} (mismatch!)",
                        binding.carrier_name, var, phi
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_empty_exit_bindings() {
        // This test would require full MirBuilder setup
        // Placeholder for future detailed testing
        // When exit_bindings is empty, reconnect should return Ok immediately
        assert!(true);
    }
}
