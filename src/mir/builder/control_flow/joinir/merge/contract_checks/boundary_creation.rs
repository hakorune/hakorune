use super::verify_boundary_hygiene;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;

/// Phase 286 P1: Boundary contract validation (B1/C2 invariants)
///
/// Validates boundary structure invariants BEFORE merge begins.
/// This catches boundary construction bugs early with clear diagnostics.
///
/// # Checks
/// - **B1**: join_inputs ValueIds are in non-colliding region (Param: 100-999)
/// - **C2**: condition_bindings join_values are in Param region
///
/// # Returns
/// - `Ok(())`: All invariants satisfied
/// - `Err(String)`: Contract violation with [joinir/contract:B*] tag
pub(in crate::mir::builder::control_flow::joinir) fn verify_boundary_contract_at_creation(
    boundary: &JoinInlineBoundary,
    context: &str,
) -> Result<(), String> {
    use crate::mir::join_ir::lowering::error_tags;
    use crate::mir::join_ir::lowering::join_value_space::{PARAM_MAX, PARAM_MIN};

    // Debug logging (HAKO_JOINIR_DEBUG=1)
    if crate::config::env::is_joinir_debug() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[joinir/boundary-contract] Validating boundary:");
        ring0
            .log
            .debug(&format!("  join_inputs: {:?}", boundary.join_inputs));
        ring0.log.debug(&format!(
            "  condition_bindings: {} bindings",
            boundary.condition_bindings.len()
        ));
        ring0.log.debug(&format!(
            "  exit_bindings: {} bindings",
            boundary.exit_bindings.len()
        ));

        // Debug: Print file/line info
        ring0
            .log
            .debug("  [DEBUG] verify_boundary_contract_at_creation called from:");
        ring0
            .log
            .debug("         (This should help identify which pattern is causing the issue)");
    }

    // Phase 29af P1: Boundary hygiene checks (strict/dev only)
    verify_boundary_hygiene(boundary)?;

    // B1: join_inputs in Param region (100-999)
    for (i, join_id) in boundary.join_inputs.iter().enumerate() {
        if !(PARAM_MIN..=PARAM_MAX).contains(&join_id.0) {
            // Debug: Print backtrace to identify which route lowering caused this
            if crate::config::env::is_joinir_debug() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!("[joinir/contract/B1] FAILED - join_inputs[{}] = ValueId({}) outside Param region", i, join_id.0));
                ring0.log.debug("  [DEBUG] This likely means alloc_local() was used instead of alloc_param() for function parameters");
                ring0.log.debug(
                    "  [DEBUG] Check route lowering code for function parameter allocations",
                );
            }

            return Err(error_tags::freeze_with_hint(
                "joinir/contract/B1",
                &format!(
                    "[{}] join_inputs[{}] = {:?} outside Param region (expected 100-999)",
                    context, i, join_id
                ),
                "use alloc_join_param() for function parameters",
            ));
        }
    }

    // C2: condition_bindings join_values in Param region
    for binding in &boundary.condition_bindings {
        if !(PARAM_MIN..=PARAM_MAX).contains(&binding.join_value.0) {
            return Err(error_tags::freeze_with_hint(
                "joinir/contract/C2",
                &format!(
                    "[{}] condition_binding '{}' join_value {:?} outside Param region",
                    context, binding.name, binding.join_value
                ),
                "use ConditionContext.alloc_value() for JoinIR ValueIds",
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::lowering::JoinInlineBoundaryBuilder;
    use crate::mir::ValueId;

    #[test]
    fn test_verify_boundary_contract_at_creation_valid() {
        // Case 1: Valid boundary with all ValueIds in Param region → OK
        let boundary = JoinInlineBoundaryBuilder::new()
            .with_inputs(
                vec![ValueId(100), ValueId(101), ValueId(102)], // Valid Param region
                vec![ValueId(4), ValueId(5), ValueId(6)],
            )
            .build();

        let result = verify_boundary_contract_at_creation(&boundary, "test_valid");
        assert!(result.is_ok(), "Valid boundary should pass: {:?}", result);
    }

    #[test]
    fn test_verify_boundary_contract_at_creation_invalid_b1_too_low() {
        // Case 2: join_inputs with ValueId below Param region → FAIL with B1 tag
        let mut boundary = JoinInlineBoundaryBuilder::new()
            .with_inputs(
                vec![ValueId(5)], // INVALID: Below Param region (100-999)
                vec![ValueId(5)],
            )
            .build();
        // Override join_inputs to invalid ValueId
        boundary.join_inputs = vec![ValueId(5)];

        let result = verify_boundary_contract_at_creation(&boundary, "test_b1_too_low");
        assert!(result.is_err(), "Invalid join_inputs should fail");

        let err = result.unwrap_err();
        assert!(
            err.contains("contract/B1"),
            "Error should have B1 tag: {}",
            err
        );
        assert!(
            err.contains("[test_b1_too_low]"),
            "Error should include context: {}",
            err
        );
        assert!(
            err.contains("outside Param region"),
            "Error should explain the issue: {}",
            err
        );
        assert!(
            err.contains("ValueId(5)"),
            "Error should show invalid ValueId: {}",
            err
        );
    }

    #[test]
    fn test_verify_boundary_contract_at_creation_invalid_b1_too_high() {
        // Case 3: join_inputs with ValueId above Param region → FAIL with B1 tag
        let mut boundary = JoinInlineBoundaryBuilder::new()
            .with_inputs(
                vec![ValueId(1000)], // INVALID: Above Param region (100-999)
                vec![ValueId(4)],
            )
            .build();
        boundary.join_inputs = vec![ValueId(1000)];

        let result = verify_boundary_contract_at_creation(&boundary, "test_b1_too_high");
        assert!(result.is_err(), "Invalid join_inputs should fail");

        let err = result.unwrap_err();
        assert!(
            err.contains("contract/B1"),
            "Error should have B1 tag: {}",
            err
        );
        assert!(
            err.contains("[test_b1_too_high]"),
            "Error should include context: {}",
            err
        );
    }
}
