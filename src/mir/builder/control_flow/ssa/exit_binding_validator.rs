//! Phase 222.5-C: Exit Binding Validator
//!
//! Validates consistency between CarrierInfo and ExitMeta.
//! Single-responsibility box for validation logic.

use crate::mir::join_ir::lowering::carrier_info::{CarrierInfo, ExitMeta};

/// Validate CarrierInfo and ExitMeta consistency
///
/// Checks:
/// 1. Loop variable is NOT in exit_values
/// 2. All carriers in ExitMeta exist in CarrierInfo
/// 3. All carriers in CarrierInfo have exit values in ExitMeta
///
/// # Arguments
///
/// * `carrier_info` - Metadata about loop variables and carriers
/// * `exit_meta` - Exit values from JoinIR lowering
///
/// # Returns
///
/// Ok(()) if validation passes, Err with descriptive message if validation fails
pub(crate) fn validate_exit_binding(
    carrier_info: &CarrierInfo,
    exit_meta: &ExitMeta,
) -> Result<(), String> {
    // Validate that all carriers in ExitMeta exist in CarrierInfo
    for (carrier_name, _) in &exit_meta.exit_values {
        if carrier_name == &carrier_info.loop_var_name {
            return Err(format!(
                "Loop variable '{}' should not be in exit_values",
                carrier_name
            ));
        }

        if !carrier_info.find_carrier(carrier_name).is_some() {
            return Err(format!(
                "Exit carrier '{}' not found in CarrierInfo",
                carrier_name
            ));
        }
    }

    // Validate that all carriers in CarrierInfo have exit values
    for carrier in &carrier_info.carriers {
        if exit_meta.find_binding(&carrier.name).is_none() {
            return Err(format!("Carrier '{}' missing in ExitMeta", carrier.name));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::lowering::carrier_info::CarrierVar;
    use crate::mir::ValueId;

    #[test]
    fn test_validate_single_carrier() {
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

        let result = validate_exit_binding(&carrier_info, &exit_meta);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_multi_carrier() {
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

        let result = validate_exit_binding(&carrier_info, &exit_meta);
        assert!(result.is_ok());
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

        let result = validate_exit_binding(&carrier_info, &exit_meta);
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

        let result = validate_exit_binding(&carrier_info, &exit_meta);
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

        let result = validate_exit_binding(&carrier_info, &exit_meta);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("should not be in exit_values"));
    }
}
