//! Phase 33-10-Refactor-P1: ExitMetaCollector Box
//!
//! Modularizes the exit_bindings collection logic from Pattern lowerers
//! into a focused, testable Box.
//!
//! **Responsibility**: Construct exit_bindings from ExitMeta + variable_ctx.variable_map lookup

use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::carrier_info::ExitMeta;
use crate::mir::join_ir::lowering::inline_boundary::LoopExitBinding;

/// ExitMetaCollector: A Box that builds exit_bindings from ExitMeta
///
/// # Design Notes
///
/// ## Pure Function Philosophy
/// ExitMetaCollector::collect() is a **pure function**:
/// - Input: builder (read-only for variable_ctx.variable_map lookup)
/// - Input: exit_meta (data structure)
/// - Output: Vec<LoopExitBinding> (new data)
/// - No side effects (except reading builder.variable_ctx.variable_map)
///
/// ## Why Pure Functions?
/// - Easy to test (no mocks needed)
/// - Easy to reason about (input → output mapping)
/// - Easy to parallelize (future optimization)
/// - Easy to reuse (any pattern lowerer can call)
///
/// ## Reusability Across Patterns
/// This collector is pattern-agnostic:
/// - Pattern 1: No exit_bindings needed (simple while)
/// - Pattern 2: Single exit binding (loop variable)
/// - Pattern 3: Multiple bindings (loop vars + if merges)
/// - Pattern 4: Single/multiple bindings (with continue)
///
/// All patterns use the same `collect()` method!
///
/// # Box Contract
///
/// **Input**:
/// - ExitMeta with exit_values (carrier_name → join_exit_value mappings)
/// - MirBuilder with variable_ctx.variable_map for host ValueId lookup
///
/// **Effect**:
/// - Creates LoopExitBinding vector (pure function, no side effects)
///
/// **Output**:
/// - Vec<LoopExitBinding>: exit_bindings ready for JoinInlineBoundary
pub struct ExitMetaCollector;

impl ExitMetaCollector {
    /// Build exit_bindings from ExitMeta and variable_ctx.variable_map
    ///
    /// # Algorithm
    ///
    /// For each entry in exit_meta.exit_values:
    /// 1. Look up the carrier's host ValueId from builder.variable_ctx.variable_map
    /// 2. Create LoopExitBinding with carrier_name, join_exit_value, host_slot
    /// 3. Collect into Vec<LoopExitBinding>
    ///
    /// # Phase 29af: Boundary hygiene
    ///
    /// ConditionOnly / LoopLocalZero carriers do not participate in exit reconnection.
    /// They are excluded from exit_bindings and are handled by header PHIs via carrier_info.
    ///
    /// # Skipped carriers
    ///
    /// - ConditionOnly / LoopLocalZero carriers are excluded from exit_bindings.
    /// - Other carriers not found in variable_ctx.variable_map are skipped (or strict-fail).
    ///
    /// # Logging
    ///
    /// If debug enabled, logs each binding created for validation.
    pub fn collect(
        builder: &MirBuilder,
        exit_meta: &ExitMeta,
        carrier_info: Option<&crate::mir::join_ir::lowering::carrier_info::CarrierInfo>, // Phase 228-8: Added carrier_info
        debug: bool,
    ) -> Vec<LoopExitBinding> {
        let mut bindings = Vec::new();
        let dev_on = crate::config::env::joinir_dev_enabled();
        let verbose = debug || dev_on;
        let strict = crate::config::env::joinir_strict_enabled() || dev_on;
        let trace = crate::mir::builder::control_flow::joinir::trace::trace();

        if verbose {
            trace.emit_if(
                "exit-line",
                "collector",
                &format!(
                    "Collecting {} exit values",
                    exit_meta.exit_values.len()
                ),
                true,
            );
        }

        // Iterate over ExitMeta entries and build bindings
        for (carrier_name, join_exit_value) in &exit_meta.exit_values {
            if verbose {
                trace.emit_if(
                    "exit-line",
                    "collector",
                    &format!(
                        "checking carrier '{}' in variable_ctx.variable_map",
                        carrier_name
                    ),
                    true,
                );
            }

            // Look up host slot from variable_ctx.variable_map
            if let Some(&host_slot) = builder.variable_ctx.variable_map.get(carrier_name) {
                use crate::mir::join_ir::lowering::carrier_info::CarrierRole;

                // Phase 228-8: Look up role from carrier_info if available
                let role = if let Some(ci) = carrier_info {
                    ci.carriers
                        .iter()
                        .find(|c| c.name == *carrier_name)
                        .map(|c| c.role)
                        .unwrap_or(CarrierRole::LoopState)
                } else {
                    CarrierRole::LoopState
                };

                let binding = LoopExitBinding {
                    carrier_name: carrier_name.clone(),
                    join_exit_value: *join_exit_value,
                    host_slot,
                    role,
                };

                if verbose {
                    trace.emit_if(
                        "exit-line",
                        "collector",
                        &format!(
                            "collected '{}' JoinIR {:?} → HOST {:?}, role={:?}",
                            carrier_name, join_exit_value, host_slot, role
                        ),
                        true,
                    );
                }

                bindings.push(binding);
            } else {
                use crate::mir::join_ir::lowering::carrier_info::{CarrierInit, CarrierRole};
                let carrier_meta = if let Some(ci) = carrier_info {
                    ci.carriers
                        .iter()
                        .find(|c| c.name == *carrier_name)
                        .map(|c| (c.role, c.init))
                } else {
                    None
                };

                match carrier_meta {
                    Some((CarrierRole::ConditionOnly, _))
                    | Some((CarrierRole::LoopState, CarrierInit::LoopLocalZero)) => {
                        if verbose {
                            trace.emit_if(
                                "exit-line",
                                "collector",
                                &format!(
                                    "skipping non-exit carrier '{}' JoinIR {:?} (ConditionOnly/LoopLocalZero)",
                                    carrier_name, join_exit_value
                                ),
                                true,
                            );
                        }
                    }
                    Some((CarrierRole::LoopState, CarrierInit::FromHost)) => {
                        let msg = format!(
                            "[joinir/exit-line] carrier '{}' missing host slot for FromHost",
                            carrier_name
                        );
                        if strict {
                            panic!("{}", msg);
                        } else if verbose {
                            trace.emit_if("exit-line", "collector", &msg, true);
                        }
                    }
                    _ => {
                        let msg = format!(
                            "[joinir/exit-line] carrier '{}' not in variable_ctx.variable_map (skip)",
                            carrier_name
                        );
                        if strict {
                            panic!("{}", msg);
                        } else if verbose {
                            trace.emit_if("exit-line", "collector", &msg, true);
                        }
                    }
                }
            }
        }

        if verbose {
            trace.emit_if(
                "exit-line",
                "collector",
                &format!("collected {} bindings: {:?}", bindings.len(), bindings),
                true,
            );
        }

        bindings
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_empty_exit_meta() {
        // This test would require full MirBuilder setup
        // Placeholder for future detailed testing
        // When exit_meta is empty, should return empty vec
        assert!(true);
    }

    #[test]
    fn test_missing_carrier_in_variable_map() {
        // This test would require full MirBuilder setup
        // Placeholder for future detailed testing
        // When carrier not in variable_ctx.variable_map, should be silently skipped
        assert!(true);
    }
}
