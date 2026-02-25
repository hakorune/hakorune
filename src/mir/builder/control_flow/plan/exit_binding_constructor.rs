//! Phase 222.5-C: Exit Binding Constructor
//!
//! Constructs loop exit bindings and allocates post-loop ValueIds.
//! Single-responsibility box for binding construction logic.

use crate::mir::join_ir::lowering::carrier_info::{CarrierInfo, ExitMeta};
use crate::mir::join_ir::lowering::inline_boundary::LoopExitBinding;
use crate::mir::ValueId;
use std::collections::BTreeMap; // Phase 222.5-D: HashMap → BTreeMap for determinism

/// Generate loop exit bindings
///
/// Returns one LoopExitBinding per carrier, in sorted order.
/// Updates variable_map with new post-loop ValueIds for each carrier.
///
/// Phase 222.5-C: Extracted from ExitBindingBuilder to separate construction concerns.
///
/// # Arguments
///
/// * `carrier_info` - Metadata about loop variables and carriers
/// * `exit_meta` - Exit values from JoinIR lowering
/// * `variable_map` - Host function's variable map (will be updated with post-loop ValueIds)
///
/// # Returns
///
/// Vec of LoopExitBinding, one per carrier, sorted by carrier name
pub(crate) fn build_loop_exit_bindings(
    carrier_info: &CarrierInfo,
    exit_meta: &ExitMeta,
    variable_map: &mut BTreeMap<String, ValueId>, // Phase 222.5-D: HashMap → BTreeMap for determinism
) -> Result<Vec<LoopExitBinding>, String> {
    let mut bindings = Vec::new();

    // Process each carrier in sorted order
    for carrier in &carrier_info.carriers {
        let join_exit_id = exit_meta
            .find_binding(&carrier.name)
            .ok_or_else(|| format!("Carrier '{}' missing in ExitMeta", carrier.name))?;

        bindings.push(LoopExitBinding {
            carrier_name: carrier.name.clone(),
            join_exit_value: join_exit_id,
            host_slot: carrier.host_id,
            role: carrier.role, // Phase 227: Propagate role from CarrierInfo
        });

        // Allocate new ValueId for post-loop carrier value
        // This represents the carrier variable's value after the loop completes
        let post_loop_id = allocate_new_value_id(variable_map);
        variable_map.insert(carrier.name.clone(), post_loop_id);
    }

    Ok(bindings)
}

/// Allocate a new ValueId for a post-loop carrier
///
/// Phase 222.5-C: Temporary sequential allocation strategy.
/// Future improvement: Delegate to MirBuilder's next_value_id() for proper allocation.
///
/// # Arguments
///
/// * `variable_map` - Current variable map to determine next available ValueId
///
/// # Returns
///
/// Newly allocated ValueId
pub(crate) fn allocate_new_value_id(variable_map: &BTreeMap<String, ValueId>) -> ValueId {
    // Phase 222.5-D: HashMap → BTreeMap for determinism
    // Find the maximum ValueId in current variable_map
    let max_id = variable_map.values().map(|v| v.0).max().unwrap_or(0);

    // Allocate next sequential ID
    // Note: This is a temporary strategy and should be replaced with
    // proper ValueId allocation from the builder
    ValueId(max_id + 1)
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
                #[cfg(feature = "normalized_dev")]
                binding_id: None,
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

        let bindings = build_loop_exit_bindings(&carrier_info, &exit_meta, &mut variable_map)
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
                    #[cfg(feature = "normalized_dev")]
                    binding_id: None,
                },
                CarrierVar {
                    name: "sum".to_string(),
                    host_id: ValueId(10),
                    join_id: None,
                    role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
                    init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
                    #[cfg(feature = "normalized_dev")]
                    binding_id: None,
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

        let bindings = build_loop_exit_bindings(&carrier_info, &exit_meta, &mut variable_map)
            .expect("Failed to build bindings");

        assert_eq!(bindings.len(), 2);
        // Bindings should be sorted by carrier name
        assert_eq!(bindings[0].carrier_name, "printed");
        assert_eq!(bindings[1].carrier_name, "sum");

        // Check post-loop ValueIds are allocated
        assert!(variable_map.contains_key("printed"));
        assert!(variable_map.contains_key("sum"));
    }
}
