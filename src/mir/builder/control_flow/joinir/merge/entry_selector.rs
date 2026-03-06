//! Phase 287 P0.3: Entry function selection (SSOT)
//!
//! Selects which JoinIR function to use as:
//! 1. Loop header (where PHIs are placed)
//! 2. Merge entry (where host jumps to)
//!
//! SSOT Principles:
//! - Prefer `boundary.loop_header_func_name` (explicit specification)
//! - Fallback: Exclude MAIN and continuation_func_ids, take first remaining
//! - Never use string-based heuristics like "k_exit" prefix matching

use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::{MirFunction, MirModule};
use std::collections::BTreeMap;

/// Select the loop_step function name (loop header)
///
/// SSOT Strategy:
/// 1. Use boundary.loop_header_func_name if specified (explicit)
/// 2. Find first function that is NOT:
///    - MAIN constant
///    - In boundary.continuation_func_ids (SSOT for continuations)
/// 3. Fallback to first function key (legacy compatibility)
pub(super) fn select_loop_step_func_name<'a>(
    mir_module: &'a MirModule,
    boundary: &'a JoinInlineBoundary,
) -> Result<&'a str, String> {
    boundary
        .loop_header_func_name
        .as_deref()
        .or_else(|| {
            // SSOT: Use boundary.continuation_func_ids, NOT string matching
            mir_module
                .functions
                .iter()
                .find(|(name, _)| {
                    let is_continuation = boundary.continuation_func_ids.contains(*name);
                    let is_main = *name == crate::mir::join_ir::lowering::canonical_names::MAIN;
                    !is_continuation && !is_main
                })
                .map(|(name, _)| name.as_str())
        })
        .or_else(|| mir_module.functions.keys().next().map(|s| s.as_str()))
        .ok_or_else(|| "JoinIR module has no functions (Phase 287 P0.3)".to_string())
}

/// Select the merge entry function (where host jumps to)
///
/// Strategy:
/// - If MAIN exists with matching params and condition_bindings: use MAIN
/// - Otherwise: use loop_step (header function)
///
/// This handles IfPhiJoin route's if-sum shape, where condition evaluation
/// happens in MAIN before entering the loop_step header.
pub(super) fn select_merge_entry_func<'a>(
    mir_module: &'a MirModule,
    boundary: &'a JoinInlineBoundary,
    loop_step_func_name: &'a str,
) -> Result<(&'a str, &'a MirFunction), String> {
    use crate::mir::join_ir::lowering::canonical_names as cn;

    // Try to get MAIN function
    if let Some(main) = mir_module.functions.get(cn::MAIN) {
        // Use MAIN if it has matching params and condition_bindings
        if main.params == boundary.join_inputs && !boundary.condition_bindings.is_empty() {
            return Ok((cn::MAIN, main));
        }
    }

    // Fallback: use loop_step
    let loop_step_func = mir_module
        .functions
        .get(loop_step_func_name)
        .ok_or_else(|| {
            format!(
                "loop_header_func_name '{}' not found in JoinIR module (Phase 287 P0.3)",
                loop_step_func_name
            )
        })?;

    Ok((loop_step_func_name, loop_step_func))
}

/// Get function from module with error handling
pub(super) fn get_function<'a>(
    mir_module: &'a BTreeMap<String, MirFunction>,
    func_name: &str,
) -> Result<&'a MirFunction, String> {
    mir_module.get(func_name).ok_or_else(|| {
        format!(
            "Function '{}' not found in JoinIR module (Phase 287 P0.3)",
            func_name
        )
    })
}
