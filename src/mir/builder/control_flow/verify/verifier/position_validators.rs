//! Position Validators - Exit position validation (V11)
//!
//! **Responsibility**: V11 invariant - Exit must be last in Seq/If branch
//!
//! ## Functions
//!
//! - `verify_exit_position()` - Exit must be last in sequence
//! - `verify_branch_plans()` - Branch plans validation with exit position check
//! - `verify_exit_if_position()` - Recursive exit position check in Seq nesting
//! - `verify_exit_depth()` - V3 exit depth range validation
//!
//! ## Dependencies
//!
//! - primitives::err() - Error formatting
//!
//! ## Called by
//!
//! - plan_validators (verify_seq, verify_if, verify_branch_n, verify_exit)
//! - loop_body_validators (verify_loop_body_tree, verify_body_plan_tree)
//! - effect_validators (verify_effect, verify_if_effect_branch)
//!
//! ## Phase Context
//!
//! - Phase 273 P3: PlanVerifier infrastructure
//! - Phase 29bq+: Cleanliness campaign - verifier.rs modularization (Step 2/7)

use super::primitives;
use crate::mir::builder::control_flow::lower::{CorePlan, LoweredRecipe};

/// V11: Verify that Exit (if present) is last in sequence
///
/// Exit nodes must be the final plan in a sequence to ensure proper control flow.
/// This aligns with ExitMap lowering expectations.
pub(super) fn verify_exit_position(
    plans: &[LoweredRecipe],
    depth: usize,
    scope: &str,
) -> Result<(), String> {
    for (i, plan) in plans.iter().enumerate() {
        if matches!(plan, CorePlan::Exit(_)) && i + 1 != plans.len() {
            return Err(primitives::err(
                "V11",
                "exit_not_last",
                format!(
                    "Exit at depth {} in {} must be last (index {}, len {})",
                    depth,
                    scope,
                    i,
                    plans.len()
                ),
            ));
        }
    }
    Ok(())
}

/// Verify branch plans with exit position check
///
/// Validates a sequence of plans in a branch context (If.then/else, BranchN arms).
/// Ensures Exit nodes are properly positioned and recursively validates all nested plans.
pub(super) fn verify_branch_plans(
    plans: &[LoweredRecipe],
    depth: usize,
    loop_depth: usize,
    scope: &str,
    verify_plan_fn: impl Fn(&LoweredRecipe, usize, usize) -> Result<(), String>,
) -> Result<(), String> {
    verify_exit_position(plans, depth, scope)?;
    for (i, plan) in plans.iter().enumerate() {
        verify_plan_fn(plan, depth + 1, loop_depth)
            .map_err(|e| format!("[{}[{}]] {}", scope, i, e))?;
    }
    Ok(())
}

/// V11: Recursive exit position check in Seq nesting
///
/// Ensures that ExitIf nodes in loop bodies are properly positioned even in nested Seq.
/// This is a specialized check for loop body validation.
pub(super) fn verify_exit_if_position(
    plans: &[LoweredRecipe],
    depth: usize,
    scope: &str,
) -> Result<(), String> {
    for (i, plan) in plans.iter().enumerate() {
        if let CorePlan::Seq(nested) = plan {
            let nested_scope = format!("{}.Seq[{}]", scope, i);
            verify_exit_if_position(nested, depth + 1, &nested_scope)?;
        }
    }
    Ok(())
}

/// V3: Exit depth range validation
///
/// Ensures that Break/Continue exit depth is within valid range (1..=loop_depth).
/// Exit depth must be positive and not exceed current loop nesting level.
pub(super) fn verify_exit_depth(
    exit_depth: usize,
    loop_depth: usize,
    depth: usize,
) -> Result<(), String> {
    if exit_depth == 0 || exit_depth > loop_depth {
        return Err(primitives::err(
            "V3",
            "exit_depth_out_of_range",
            format!(
                "Exit depth {} is out of range (loop_depth={}) at depth {}",
                exit_depth, loop_depth, depth
            ),
        ));
    }
    Ok(())
}
