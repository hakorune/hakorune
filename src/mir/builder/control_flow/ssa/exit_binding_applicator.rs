//! Phase 222.5-C: Exit Binding Applicator
//!
//! Applies exit bindings to JoinInlineBoundary.
//! Single-responsibility box for boundary application logic.

#![allow(dead_code)] // Phase 291x-126: staged exit-binding owner seam.

use crate::mir::join_ir::lowering::carrier_info::{CarrierInfo, ExitMeta};
use crate::mir::join_ir::lowering::inline_boundary::{JoinInlineBoundary, LoopExitBinding};
use crate::mir::ValueId;
use std::collections::BTreeMap; // Phase 222.5-D: HashMap → BTreeMap for determinism

/// Apply bindings to JoinInlineBoundary
///
/// Sets exit_bindings based on loop_var + carriers.
/// Must be called after build_loop_exit_bindings().
///
/// Phase 222.5-C: Extracted from ExitBindingBuilder to separate application concerns.
///
/// # Arguments
///
/// * `carrier_info` - Metadata about loop variables and carriers
/// * `exit_meta` - Exit values from JoinIR lowering
/// * `variable_map` - Host function's variable map (must contain post-loop ValueIds)
/// * `boundary` - JoinInlineBoundary to update
///
/// # Returns
///
/// Success or error if boundary cannot be updated
pub(crate) fn apply_exit_bindings_to_boundary(
    carrier_info: &CarrierInfo,
    exit_meta: &ExitMeta,
    variable_map: &BTreeMap<String, ValueId>, // Phase 222.5-D: HashMap → BTreeMap for determinism
    boundary: &mut JoinInlineBoundary,
) -> Result<(), String> {
    // Build explicit exit bindings (loop var + carriers)
    let mut bindings = Vec::new();
    bindings.push(create_loop_var_exit_binding(carrier_info));

    for carrier in &carrier_info.carriers {
        let post_loop_id = variable_map
            .get(&carrier.name)
            .copied()
            .ok_or_else(|| format!("Post-loop ValueId not found for carrier '{}'", carrier.name))?;

        let join_exit_id = exit_meta
            .find_binding(&carrier.name)
            .ok_or_else(|| format!("Exit value not found for carrier '{}'", carrier.name))?;

        bindings.push(LoopExitBinding {
            carrier_name: carrier.name.clone(),
            host_slot: post_loop_id,
            join_exit_value: join_exit_id,
            role: carrier.role, // Phase 227: Propagate role from CarrierInfo
        });
    }

    boundary.exit_bindings = bindings;

    Ok(())
}

/// Create the loop variable exit binding
///
/// The loop variable is always the first exit (index 0).
///
/// Phase 222.5-C: Extracted from ExitBindingBuilder for single-purpose function.
///
/// # Arguments
///
/// * `carrier_info` - Metadata about loop variables and carriers
///
/// # Returns
///
/// LoopExitBinding for the loop variable
pub(crate) fn create_loop_var_exit_binding(carrier_info: &CarrierInfo) -> LoopExitBinding {
    use crate::mir::join_ir::lowering::carrier_info::CarrierRole;
    LoopExitBinding {
        carrier_name: carrier_info.loop_var_name.clone(),
        join_exit_value: carrier_info.loop_var_id, // Loop var maps to itself
        host_slot: carrier_info.loop_var_id,
        role: CarrierRole::LoopState, // Phase 227: Loop var is always LoopState
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::lowering::carrier_info::CarrierVar;

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

        // Simulate post-loop ValueId allocation (as done by constructor)
        let variable_map = [
            ("i".to_string(), ValueId(5)),
            ("sum".to_string(), ValueId(11)), // Post-loop ValueId
        ]
        .iter()
        .cloned()
        .collect();

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

        apply_exit_bindings_to_boundary(&carrier_info, &exit_meta, &variable_map, &mut boundary)
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

    #[test]
    fn test_loop_var_exit_binding() {
        let carrier_info = CarrierInfo::with_carriers("i".to_string(), ValueId(5), vec![]);

        let binding = create_loop_var_exit_binding(&carrier_info);
        assert_eq!(binding.carrier_name, "i");
        assert_eq!(binding.host_slot, ValueId(5));
        assert_eq!(binding.join_exit_value, ValueId(5));
    }
}
