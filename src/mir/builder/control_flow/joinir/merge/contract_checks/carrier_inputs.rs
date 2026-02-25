use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

/// Phase 286C-4.2: Contract check (Fail-Fast) - carrier_inputs must include all exit_bindings.
///
/// # Purpose
///
/// Validates that carrier_inputs collected during plan stage contains entries for all
/// exit_bindings that require PHI generation (non-ConditionOnly carriers).
///
/// This catches bugs where:
/// - CarrierInputsCollector fails to add a carrier due to missing header PHI
/// - plan_rewrites skips a carrier mistakenly
/// - exit_phi_builder receives incomplete carrier_inputs
///
/// # Contract
///
/// For each non-ConditionOnly exit_binding:
/// - `carrier_inputs[carrier_name]` must exist
///
/// # Returns
/// - `Ok(())`: All non-ConditionOnly carriers present in carrier_inputs
/// - `Err(String)`: Contract violation with [joinir/contract:C4] tag
pub(in crate::mir::builder::control_flow::joinir::merge) fn verify_carrier_inputs_complete(
    boundary: &JoinInlineBoundary,
    carrier_inputs: &BTreeMap<String, Vec<(BasicBlockId, ValueId)>>,
) -> Result<(), String> {
    use crate::mir::join_ir::lowering::carrier_info::CarrierRole;
    use crate::mir::join_ir::lowering::error_tags;

    for binding in &boundary.exit_bindings {
        // Skip ConditionOnly carriers (no PHI required)
        if binding.role == CarrierRole::ConditionOnly {
            continue;
        }

        // Check carrier_inputs has entry for this binding
        if !carrier_inputs.contains_key(&binding.carrier_name) {
            return Err(error_tags::freeze_with_hint(
                "joinir/contract:C4",
                &format!(
                    "exit_binding carrier '{}' (role={:?}) is missing from carrier_inputs",
                    binding.carrier_name, binding.role
                ),
                "ensure CarrierInputsCollector successfully collected from header PHI or DirectValue mode; check loop_header_phi_info has PHI dst for this carrier",
            ));
        }
    }

    Ok(())
}

// Phase 286C-4.2: Test helper for JoinInlineBoundary construction
#[cfg(test)]
fn make_boundary(
    exit_bindings: Vec<crate::mir::join_ir::lowering::inline_boundary::LoopExitBinding>,
) -> JoinInlineBoundary {
    use crate::mir::join_ir::lowering::carrier_info::ExitReconnectMode;
    use crate::mir::join_ir::lowering::JoinInlineBoundaryBuilder;

    let mut boundary = JoinInlineBoundaryBuilder::new()
        .with_inputs(vec![], vec![])
        .with_exit_bindings(exit_bindings)
        .build();

    boundary.exit_reconnect_mode = ExitReconnectMode::DirectValue;
    boundary
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::lowering::inline_boundary::LoopExitBinding;
    use crate::mir::join_ir::lowering::carrier_info::CarrierRole;

    #[test]
    fn test_verify_carrier_inputs_complete_missing_carrier() {
        // Setup: boundary with Accumulator carrier
        let boundary = make_boundary(vec![
            LoopExitBinding {
                carrier_name: "sum".to_string(),
                host_slot: ValueId(10),
                join_exit_value: ValueId(100),
                role: CarrierRole::LoopState,
            },
        ]);

        // Empty carrier_inputs (欠落シミュレート)
        let carrier_inputs = BTreeMap::new();

        // Verify: should fail
        let result = verify_carrier_inputs_complete(&boundary, &carrier_inputs);
        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("joinir/contract:C4"));
        assert!(err_msg.contains("sum"));
    }

    #[test]
    fn test_verify_carrier_inputs_complete_condition_only_skipped() {
        // Setup: ConditionOnly carrier (should be skipped)
        let boundary = make_boundary(vec![
            LoopExitBinding {
                carrier_name: "is_found".to_string(),
                role: CarrierRole::ConditionOnly,
                host_slot: ValueId(20),
                join_exit_value: ValueId(101),
            },
        ]);

        // Empty carrier_inputs (but OK because ConditionOnly)
        let carrier_inputs = BTreeMap::new();

        // Verify: should succeed
        let result = verify_carrier_inputs_complete(&boundary, &carrier_inputs);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_carrier_inputs_complete_valid() {
        // Setup: LoopState carrier with inputs
        let boundary = make_boundary(vec![
            LoopExitBinding {
                carrier_name: "count".to_string(),
                host_slot: ValueId(30),
                join_exit_value: ValueId(102),
                role: CarrierRole::LoopState,
            },
        ]);

        let mut carrier_inputs = BTreeMap::new();
        carrier_inputs.insert(
            "count".to_string(),
            vec![
                (BasicBlockId(1), ValueId(100)),
                (BasicBlockId(2), ValueId(200)),
            ],
        );

        // Verify: should succeed
        let result = verify_carrier_inputs_complete(&boundary, &carrier_inputs);
        assert!(result.is_ok());
    }
}
