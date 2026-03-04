use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;

/// Phase 256 P1.5-DBG: Contract check - Entry function parameters match boundary join_inputs exactly
///
/// # Purpose
///
/// Validates that `boundary.join_inputs` and `JoinModule.entry.params` have the same order,
/// count, and ValueId mapping. This prevents ordering bugs like the Pattern6 loop_invariants
/// issue where `[s, ch]` → `[ch, s]` required manual debugging.
///
/// # Example Valid (Pattern6):
/// ```text
/// JoinModule.main.params:     [ValueId(100), ValueId(101), ValueId(102)]  (i, ch, s)
/// boundary.join_inputs:       [ValueId(100), ValueId(101), ValueId(102)]
/// Check: params[0]==join_inputs[0] ✓, params[1]==join_inputs[1] ✓, etc.
/// ```
///
/// # Example Invalid (ordering bug):
/// ```text
/// JoinModule.main.params:     [ValueId(100), ValueId(101), ValueId(102)]  (i, ch, s)
/// boundary.join_inputs:       [ValueId(100), ValueId(102), ValueId(101)]  (i, s, ch - WRONG!)
/// Error: "Entry param[1] in 'main': expected ValueId(101), but boundary.join_inputs[1] = ValueId(102)"
/// ```
///
/// # Contract
///
/// 1. Entry function params count must match `boundary.join_inputs` count
/// 2. Each `entry.params[i]` must equal `boundary.join_inputs[i]` (order + ValueId match)
/// 3. Optional: `join_inputs.len() == host_inputs.len()` (already asserted in constructor)
///
/// # Returns
///
/// - `Ok(())`: All parameters match correctly
/// - `Err(String)`: Mismatch found with clear diagnostic message
pub(in crate::mir::builder::control_flow::joinir) fn verify_boundary_entry_params(
    join_module: &crate::mir::join_ir::JoinModule,
    boundary: &JoinInlineBoundary,
) -> Result<(), String> {
    use crate::mir::join_ir::lowering::error_tags;

    // Get entry function (priority: join_module.entry → fallback to "main")
    let entry = get_entry_function(join_module)?;

    // Check 1: Count must match
    if entry.params.len() != boundary.join_inputs.len() {
        return Err(error_tags::freeze_with_hint(
            "phase1.5/boundary/entry_param_count",
            &format!(
                "Entry function '{}' has {} params, but boundary has {} join_inputs",
                entry.name,
                entry.params.len(),
                boundary.join_inputs.len()
            ),
            "ensure pattern lowerer sets boundary.join_inputs with one entry per parameter",
        ));
    }

    // Check 2: Each param must match in order
    for (i, (entry_param, join_input)) in entry
        .params
        .iter()
        .zip(boundary.join_inputs.iter())
        .enumerate()
    {
        if entry_param != join_input {
            return Err(error_tags::freeze_with_hint(
                "phase1.5/boundary/entry_param_mismatch",
                &format!(
                    "Entry param[{}] in '{}': expected {:?}, but boundary.join_inputs[{}] = {:?}",
                    i, entry.name, entry_param, i, join_input
                ),
                "parameter ValueId mismatch indicates boundary.join_inputs constructed in wrong order",
            ));
        }
    }

    // Check 3: Verify join_inputs.len() == host_inputs.len() (belt-and-suspenders)
    // (Already asserted in JoinInlineBoundary constructor, but reconfirm for safety)
    if boundary.join_inputs.len() != boundary.host_inputs.len() {
        return Err(error_tags::freeze_with_hint(
            "phase1.5/boundary/input_count_mismatch",
            &format!(
                "boundary.join_inputs ({}) and host_inputs ({}) have different lengths",
                boundary.join_inputs.len(),
                boundary.host_inputs.len()
            ),
            "BoundaryBuilder should prevent this - indicates constructor invariant violation",
        ));
    }

    Ok(())
}

/// Helper: Get entry function from JoinModule
///
/// Priority:
/// 1. Use `join_module.entry` if Some
/// 2. Fallback to function named "main"
/// 3. Fail with descriptive error if neither exists
fn get_entry_function(
    join_module: &crate::mir::join_ir::JoinModule,
) -> Result<&crate::mir::join_ir::JoinFunction, String> {
    use crate::mir::join_ir::lowering::error_tags;

    if let Some(entry_id) = join_module.entry {
        return join_module.functions.get(&entry_id).ok_or_else(|| {
            error_tags::freeze_with_hint(
                "phase1.5/boundary/entry_not_found",
                &format!("Entry function ID {:?} not found in module", entry_id),
                "ensure JoinModule.entry points to valid JoinFuncId",
            )
        });
    }

    // Fallback to "main"
    join_module
        .get_function_by_name(crate::mir::join_ir::lowering::canonical_names::MAIN)
        .ok_or_else(|| {
        error_tags::freeze_with_hint(
            "phase1.5/boundary/no_entry_function",
            "no entry function found (entry=None and no 'main' function)",
            "pattern lowerer must set join_module.entry OR create 'main' function",
        )
    })
}

/// Phase 256 P1.6: Run all JoinIR conversion pipeline contract checks
///
/// Thin aggregation function that runs all pre-conversion contract checks.
/// This simplifies the conversion_pipeline.rs call site and makes it easy to add new checks.
///
/// # Checks included:
/// - Phase 256 P1.5-DBG: Boundary entry parameter contract
/// - (Future checks can be added here)
///
/// # Debug logging:
/// - Enabled when `is_joinir_debug()` is true (HAKO_JOINIR_DEBUG=1)
pub(in crate::mir::builder) fn run_all_pipeline_checks(
    join_module: &crate::mir::join_ir::JoinModule,
    boundary: &JoinInlineBoundary,
) -> Result<(), String> {
    // Phase 256 P1.5-DBG: Boundary entry parameter contract
    verify_boundary_entry_params(join_module, boundary)?;

    // Debug logging (is_joinir_debug() only)
    if crate::config::env::is_joinir_debug() {
        debug_log_boundary_contract(join_module, boundary);
    }

    Ok(())
}

/// Debug logging for boundary entry parameter contract validation
///
/// Only enabled when `is_joinir_debug()` is true (HAKO_JOINIR_DEBUG=1).
///
/// Outputs each parameter with OK/MISMATCH status for easy diagnosis.
///
/// # Example Output
///
/// ```text
/// [joinir/boundary-contract] Entry function 'main' params:
///   [0] entry=ValueId(100) join_input=ValueId(100) OK
///   [1] entry=ValueId(101) join_input=ValueId(101) OK
///   [2] entry=ValueId(102) join_input=ValueId(102) OK
/// ```
#[cfg(debug_assertions)]
fn debug_log_boundary_contract(
    join_module: &crate::mir::join_ir::JoinModule,
    boundary: &JoinInlineBoundary,
) {
    // Get entry function (priority: join_module.entry → fallback to "main")
    let entry = if let Some(entry_id) = join_module.entry {
        join_module.functions.get(&entry_id)
    } else {
        join_module.get_function_by_name("main")
    };

    if let Some(entry) = entry {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[joinir/boundary-contract] Entry function '{}' params:",
            entry.name
        ));
        for (i, (entry_param, join_input)) in entry
            .params
            .iter()
            .zip(boundary.join_inputs.iter())
            .enumerate()
        {
            let status = if entry_param == join_input {
                "OK"
            } else {
                "MISMATCH"
            };
            ring0.log.debug(&format!(
                "  [{}] entry={:?} join_input={:?} {}",
                i, entry_param, join_input, status
            ));
        }
    }
}

#[cfg(not(debug_assertions))]
fn debug_log_boundary_contract(
    _join_module: &crate::mir::join_ir::JoinModule,
    _boundary: &JoinInlineBoundary,
) {
    // No-op in release mode
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::{JoinFunction, JoinFuncId, JoinModule};
    use crate::mir::join_ir::lowering::JoinInlineBoundaryBuilder;
    use crate::mir::ValueId;

    #[test]
    fn test_verify_boundary_entry_params_matches() {
        // Case 1: All parameters match → OK
        let mut join_module = JoinModule::new();
        let main_func = JoinFunction::new(
            JoinFuncId::new(0),
            "main".to_string(),
            vec![ValueId(100), ValueId(101), ValueId(102)],
        );
        join_module.add_function(main_func);
        join_module.entry = Some(JoinFuncId::new(0));

        let boundary = JoinInlineBoundaryBuilder::new()
            .with_inputs(
                vec![ValueId(100), ValueId(101), ValueId(102)],
                vec![ValueId(4), ValueId(5), ValueId(6)],
            )
            .build();

        let result = verify_boundary_entry_params(&join_module, &boundary);
        assert!(result.is_ok(), "Matching params should pass: {:?}", result);
    }

    #[test]
    fn test_verify_boundary_entry_params_order_mismatch() {
        // Case 2: Parameters in wrong order → FAIL with specific error
        let mut join_module = JoinModule::new();
        let main_func = JoinFunction::new(
            JoinFuncId::new(0),
            "main".to_string(),
            vec![ValueId(100), ValueId(101), ValueId(102)], // i, ch, s
        );
        join_module.add_function(main_func);
        join_module.entry = Some(JoinFuncId::new(0));

        // Wrong order: [i, s, ch] instead of [i, ch, s]
        let boundary = JoinInlineBoundaryBuilder::new()
            .with_inputs(
                vec![ValueId(100), ValueId(102), ValueId(101)], // WRONG ORDER
                vec![ValueId(4), ValueId(6), ValueId(5)],
            )
            .build();

        let result = verify_boundary_entry_params(&join_module, &boundary);
        assert!(result.is_err(), "Order mismatch should fail");

        let err = result.unwrap_err();
        assert!(
            err.contains("param[1]"),
            "Error should mention param[1]: {}",
            err
        );
        assert!(
            err.contains("expected ValueId(101)"),
            "Error should show expected ValueId(101): {}",
            err
        );
        assert!(
            err.contains("ValueId(102)"),
            "Error should show actual ValueId(102): {}",
            err
        );
    }

    #[test]
    fn test_verify_boundary_entry_params_count_mismatch() {
        // Case 3: Different count of parameters → FAIL
        let mut join_module = JoinModule::new();
        let main_func = JoinFunction::new(
            JoinFuncId::new(0),
            "main".to_string(),
            vec![ValueId(100), ValueId(101), ValueId(102)], // 3 params
        );
        join_module.add_function(main_func);
        join_module.entry = Some(JoinFuncId::new(0));

        // Boundary has only 2 inputs (count mismatch)
        let boundary = JoinInlineBoundaryBuilder::new()
            .with_inputs(
                vec![ValueId(100), ValueId(101)], // Only 2 inputs
                vec![ValueId(4), ValueId(5)],
            )
            .build();

        let result = verify_boundary_entry_params(&join_module, &boundary);
        assert!(result.is_err(), "Count mismatch should fail");

        let err = result.unwrap_err();
        assert!(
            err.contains("has 3 params"),
            "Error should mention 3 params: {}",
            err
        );
        assert!(
            err.contains("2 join_inputs"),
            "Error should mention 2 join_inputs: {}",
            err
        );
    }
}
