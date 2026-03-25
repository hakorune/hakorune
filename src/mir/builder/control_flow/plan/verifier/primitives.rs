//! Primitive Verification Helpers
//!
//! **Responsibility**: Low-level validation functions and error formatting
//!
//! ## Functions
//!
//! - `verify_value_id_basic()` - V6 basic ValueId validity check (currently no-op placeholder)
//! - `verify_edge_args_layout()` - V13 EdgeArgs layout validation
//! - `debug_assert_value_join_invariants()` - Debug-only value join assertions
//! - `err()` - Standardized error message formatting `[Vx][reason=...] detail`
//!
//! ## Dependencies
//!
//! - None (pure helpers used by all other verifier modules)
//!
//! ## Called by
//!
//! - All verifier modules (position_validators, plan_validators, effect_validators, loop_validators, loop_body_validators)
//!
//! ## Phase Context
//!
//! - Phase 273 P3: PlanVerifier infrastructure
//! - Phase 29bq+: Cleanliness campaign - verifier.rs modularization (Step 1/7)

use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::EdgeArgs;
use crate::mir::ValueId;

/// V6: Basic ValueId validity check
///
/// Note: This is a basic check. Full validity would require builder context.
pub(super) fn verify_value_id_basic(
    value_id: ValueId,
    depth: usize,
    context: &str,
) -> Result<(), String> {
    // ValueId(0) might be valid in some contexts, so we don't check for zero
    // This is a placeholder for more sophisticated checks if needed
    let _ = (value_id, depth, context);
    Ok(())
}

/// V13: EdgeArgs layout validation
///
/// Validates that EdgeArgs with `ExprResultPlusCarriers` layout have at least one value.
pub(super) fn verify_edge_args_layout(
    args: &EdgeArgs,
    depth: usize,
    context: &str,
) -> Result<(), String> {
    if matches!(args.layout, JumpArgsLayout::ExprResultPlusCarriers) && args.values.is_empty() {
        return Err(err(
            "V13",
            "edge_args_missing_value",
            format!(
                "EdgeArgs at depth {} {} requires expr_result value",
                depth, context
            ),
        ));
    }
    Ok(())
}

/// Standardized error message formatting
///
/// Format: `[Vx][reason=...] detail`
/// Enables stable reason codes for diagnostics and filtering.
pub(super) fn err(
    code: &'static str,
    reason: &'static str,
    detail: impl std::fmt::Display,
) -> String {
    format!("[{}][reason={}] {}", code, reason, detail)
}

/// Debug assertion: value join invariants
///
/// Validates that if `value_join_needed` is true, at least one exit kind is present.
/// This is a sanity check for loop value join scenarios.
#[cfg(debug_assertions)]
pub(in crate::mir::builder) fn debug_assert_value_join_invariants(facts: &CanonicalLoopFacts) {
    if facts.value_join_needed {
        debug_assert!(
            !facts.exit_kinds_present.is_empty(),
            "value join requires at least one exit kind"
        );
    }
}

#[cfg(not(debug_assertions))]
pub(in crate::mir::builder) fn debug_assert_value_join_invariants(_facts: &CanonicalLoopFacts) {}
