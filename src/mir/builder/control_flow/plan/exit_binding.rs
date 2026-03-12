//! Phase 193-4 / Phase 222.5-C: Exit Binding Builder
//!
//! Connects JoinIR exit values back to host function's variable_map,
//! eliminating hardcoded variable names and ValueId assumptions.
//!
//! Phase 222.5-C: Refactored into modular architecture:
//! - Validator: Validates CarrierInfo and ExitMeta consistency
//! - Constructor: Builds exit bindings and allocates post-loop ValueIds
//! - Applicator: Applies bindings to JoinInlineBoundary
//!
//! This orchestrator coordinates the three modules for a complete workflow.

use crate::mir::join_ir::lowering::carrier_info::{CarrierInfo, ExitMeta};
use crate::mir::join_ir::lowering::inline_boundary::{JoinInlineBoundary, LoopExitBinding};
use crate::mir::ValueId;
use std::collections::BTreeMap; // Phase 222.5-D: HashMap → BTreeMap for determinism

// Phase 222.5-C: Import modular components
use super::exit_binding_applicator::{
    apply_exit_bindings_to_boundary, create_loop_var_exit_binding,
};
use super::exit_binding_constructor::build_loop_exit_bindings;
use super::exit_binding_validator::validate_exit_binding;

/// Builder for generating loop exit bindings
///
/// Phase 193-4: Fully boxifies exit binding generation.
/// Phase 222.5-C: Refactored into orchestrator pattern.
///
/// Eliminates hardcoded variable names and ValueId plumbing scattered across lowerers.
pub(crate) struct ExitBindingBuilder<'a> {
    carrier_info: &'a CarrierInfo,
    exit_meta: &'a ExitMeta,
    variable_map: &'a mut BTreeMap<String, ValueId>, // Phase 222.5-D: HashMap → BTreeMap for determinism
}

impl<'a> std::fmt::Debug for ExitBindingBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExitBindingBuilder")
            .field("carrier_info", self.carrier_info)
            .field("exit_meta", self.exit_meta)
            .field("variable_map", &"<BTreeMap>") // Phase 222.5-D: HashMap → BTreeMap for determinism
            .finish()
    }
}

impl<'a> ExitBindingBuilder<'a> {
    /// Create a new ExitBindingBuilder
    ///
    /// Phase 222.5-C: Delegates validation to validator module.
    ///
    /// # Arguments
    ///
    /// * `carrier_info` - Metadata about loop variables and carriers
    /// * `exit_meta` - Exit values from JoinIR lowering
    /// * `variable_map` - Host function's variable map (will be updated)
    ///
    /// # Returns
    ///
    /// ExitBindingBuilder instance, or error if metadata is inconsistent
    pub(crate) fn new(
        carrier_info: &'a CarrierInfo,
        exit_meta: &'a ExitMeta,
        variable_map: &'a mut BTreeMap<String, ValueId>, // Phase 222.5-D: HashMap → BTreeMap for determinism
    ) -> Result<Self, String> {
        // Phase 222.5-C: Delegate validation to validator module
        validate_exit_binding(carrier_info, exit_meta)?;

        Ok(Self {
            carrier_info,
            exit_meta,
            variable_map,
        })
    }

    /// Generate loop exit bindings
    ///
    /// Phase 222.5-C: Delegates to constructor module.
    ///
    /// Returns one LoopExitBinding per carrier, in sorted order.
    /// Updates variable_map with new post-loop ValueIds for each carrier.
    ///
    /// # Returns
    ///
    /// Vec of LoopExitBinding, one per carrier, sorted by carrier name
    pub(crate) fn build_loop_exit_bindings(&mut self) -> Result<Vec<LoopExitBinding>, String> {
        // Phase 222.5-C: Delegate to constructor module
        build_loop_exit_bindings(self.carrier_info, self.exit_meta, self.variable_map)
    }

    /// Apply bindings to JoinInlineBoundary
    ///
    /// Phase 222.5-C: Delegates to applicator module.
    ///
    /// Sets exit_bindings based on loop_var + carriers.
    /// Must be called after build_loop_exit_bindings().
    ///
    /// # Arguments
    ///
    /// * `boundary` - JoinInlineBoundary to update
    ///
    /// # Returns
    ///
    /// Success or error if boundary cannot be updated
    pub(crate) fn apply_to_boundary(
        &self,
        boundary: &mut JoinInlineBoundary,
    ) -> Result<(), String> {
        // Phase 222.5-C: Delegate to applicator module
        apply_exit_bindings_to_boundary(
            self.carrier_info,
            self.exit_meta,
            self.variable_map,
            boundary,
        )
    }

    /// Get the loop variable exit binding
    ///
    /// Phase 222.5-C: Delegates to applicator module.
    ///
    /// The loop variable is always the first exit (index 0).
    pub fn loop_var_exit_binding(&self) -> LoopExitBinding {
        // Phase 222.5-C: Delegate to applicator module
        create_loop_var_exit_binding(self.carrier_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::lowering::carrier_info::CarrierVar;

    #[test]
    fn test_single_carrier_binding() {
        let carrier_info = CarrierInfo::with_carriers(
            "i".to_string(),
            ValueId(5),
            vec![CarrierVar {
                name: "sum".to_string(),
                host_id: ValueId(10),
                join_id: None,
                role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
                init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
            }],
        );

        let exit_meta = ExitMeta::single("sum".to_string(), ValueId(15));

        let mut variable_map = [
            ("i".to_string(), ValueId(5)),
            ("sum".to_string(), ValueId(10)),
        ]
        .iter()
        .cloned()
        .collect();

        let mut builder = ExitBindingBuilder::new(&carrier_info, &exit_meta, &mut variable_map)
            .expect("Failed to create builder");

        let bindings = builder
            .build_loop_exit_bindings()
            .expect("Failed to build bindings");

        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].carrier_name, "sum");
        assert_eq!(bindings[0].host_slot, ValueId(10));
        assert_eq!(bindings[0].join_exit_value, ValueId(15));

        // Check that variable_map was updated with new post-loop ValueId
        assert!(variable_map.contains_key("sum"));
        let post_loop_id = variable_map["sum"];
        assert!(post_loop_id.0 > 10); // Should be allocated after max of existing IDs
    }

    #[test]
    fn test_multi_carrier_binding() {
        let carrier_info = CarrierInfo::with_carriers(
            "i".to_string(),
            ValueId(5),
            vec![
                CarrierVar {
                    name: "printed".to_string(),
                    host_id: ValueId(11),
                    join_id: None,
                    role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
                    init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
                },
                CarrierVar {
                    name: "sum".to_string(),
                    host_id: ValueId(10),
                    join_id: None,
                    role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
                    init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
                },
            ],
        );

        let exit_meta = ExitMeta::multiple(vec![
            ("printed".to_string(), ValueId(14)),
            ("sum".to_string(), ValueId(15)),
        ]);

        let mut variable_map = [
            ("i".to_string(), ValueId(5)),
            ("sum".to_string(), ValueId(10)),
            ("printed".to_string(), ValueId(11)),
        ]
        .iter()
        .cloned()
        .collect();

        let mut builder = ExitBindingBuilder::new(&carrier_info, &exit_meta, &mut variable_map)
            .expect("Failed to create builder");

        let bindings = builder
            .build_loop_exit_bindings()
            .expect("Failed to build bindings");

        assert_eq!(bindings.len(), 2);
        // Bindings should be sorted by carrier name
        assert_eq!(bindings[0].carrier_name, "printed");
        assert_eq!(bindings[1].carrier_name, "sum");

        // Check post-loop ValueIds are allocated
        assert!(variable_map.contains_key("printed"));
        assert!(variable_map.contains_key("sum"));
    }

    #[test]
    fn test_carrier_name_mismatch_error() {
        let carrier_info = CarrierInfo::with_carriers(
            "i".to_string(),
            ValueId(5),
            vec![CarrierVar {
                name: "sum".to_string(),
                host_id: ValueId(10),
                join_id: None,
                role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
                init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
            }],
        );

        // ExitMeta with non-existent carrier
        let exit_meta = ExitMeta::single("foo".to_string(), ValueId(15));

        let mut variable_map = [
            ("i".to_string(), ValueId(5)),
            ("sum".to_string(), ValueId(10)),
        ]
        .iter()
        .cloned()
        .collect();

        let result = ExitBindingBuilder::new(&carrier_info, &exit_meta, &mut variable_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found in CarrierInfo"));
    }

    #[test]
    fn test_missing_carrier_in_exit_meta() {
        let carrier_info = CarrierInfo::with_carriers(
            "i".to_string(),
            ValueId(5),
            vec![CarrierVar {
                name: "sum".to_string(),
                host_id: ValueId(10),
                join_id: None,
                role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
                init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
            }],
        );

        // ExitMeta is empty
        let exit_meta = ExitMeta::empty();

        let mut variable_map = [
            ("i".to_string(), ValueId(5)),
            ("sum".to_string(), ValueId(10)),
        ]
        .iter()
        .cloned()
        .collect();

        let result = ExitBindingBuilder::new(&carrier_info, &exit_meta, &mut variable_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("missing in ExitMeta"));
    }

    #[test]
    fn test_loop_var_in_exit_meta_error() {
        let carrier_info = CarrierInfo::with_carriers(
            "i".to_string(),
            ValueId(5),
            vec![CarrierVar {
                name: "sum".to_string(),
                host_id: ValueId(10),
                join_id: None,
                role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
                init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
            }],
        );

        // ExitMeta incorrectly includes loop var
        let exit_meta = ExitMeta::multiple(vec![
            ("i".to_string(), ValueId(5)),
            ("sum".to_string(), ValueId(15)),
        ]);

        let mut variable_map = [
            ("i".to_string(), ValueId(5)),
            ("sum".to_string(), ValueId(10)),
        ]
        .iter()
        .cloned()
        .collect();

        let result = ExitBindingBuilder::new(&carrier_info, &exit_meta, &mut variable_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("should not be in exit_values"));
    }

    #[test]
    fn test_apply_to_boundary() {
        let carrier_info = CarrierInfo::with_carriers(
            "i".to_string(),
            ValueId(5),
            vec![CarrierVar {
                name: "sum".to_string(),
                host_id: ValueId(10),
                join_id: None,
                role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
                init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
            }],
        );

        let exit_meta = ExitMeta::single("sum".to_string(), ValueId(15));

        let mut variable_map = [
            ("i".to_string(), ValueId(5)),
            ("sum".to_string(), ValueId(10)),
        ]
        .iter()
        .cloned()
        .collect();

        let mut builder = ExitBindingBuilder::new(&carrier_info, &exit_meta, &mut variable_map)
            .expect("Failed to create builder");

        let _ = builder
            .build_loop_exit_bindings()
            .expect("Failed to build bindings");

        let mut boundary = JoinInlineBoundary {
            host_inputs: vec![],
            join_inputs: vec![],
            exit_bindings: vec![],      // Phase 171: Add missing field
            condition_bindings: vec![], // Phase 171-fix: Add missing field
            expr_result: None,          // Phase 33-14: Add missing field
            jump_args_layout:
                crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout::CarriersOnly,
            loop_var_name: None,         // Phase 33-16: Add missing field
            loop_header_func_name: None, // Phase 287 P2
            carrier_info: None,          // Phase 228: Add missing field
            loop_invariants: vec![],     // Phase 255 P2: Add missing field
            continuation_func_ids: std::collections::BTreeSet::from([
                "k_exit".to_string(), // Phase 256 P1.7: Use String instead of JoinFuncId
            ]),
            exit_reconnect_mode:
                crate::mir::join_ir::lowering::carrier_info::ExitReconnectMode::default(), // Phase 131 P1.5
        };

        builder
            .apply_to_boundary(&mut boundary)
            .expect("Failed to apply to boundary");

        // Should have loop_var + sum carrier in exit_bindings
        assert_eq!(boundary.exit_bindings.len(), 2);
        assert_eq!(boundary.exit_bindings[0].carrier_name, "i");
        assert_eq!(boundary.exit_bindings[0].host_slot, ValueId(5));
        assert_eq!(boundary.exit_bindings[0].join_exit_value, ValueId(5));

        assert_eq!(boundary.exit_bindings[1].carrier_name, "sum");
        // Post-loop carrier id is freshly allocated (10 -> 11)
        assert_eq!(boundary.exit_bindings[1].host_slot, ValueId(11));
        assert_eq!(boundary.exit_bindings[1].join_exit_value, ValueId(15));
    }
}
