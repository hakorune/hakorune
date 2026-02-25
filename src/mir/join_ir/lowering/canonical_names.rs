//! Phase 256 P1.7: Canonical JoinIR Function Names (SSOT)
//!
//! This module provides a Single Source of Truth (SSOT) for JoinIR function names
//! that are used throughout the codebase. By centralizing these names here, we:
//!
//! 1. **Eliminate magic strings**: No more scattered "k_exit", "loop_step" literals
//! 2. **Ensure consistency**: All code uses the same canonical names
//! 3. **Simplify refactoring**: Change the name in one place, not dozens
//! 4. **Improve readability**: Clear intent with named constants
//!
//! ## Usage
//!
//! ```rust
//! use crate::mir::join_ir::lowering::canonical_names as cn;
//!
//! // Instead of:
//! let func_name = "k_exit".to_string();
//!
//! // Use:
//! let func_name = cn::K_EXIT.to_string();
//! ```
//!
//! ## Design Note
//!
//! These names represent the canonical function names used in JoinModule.
//! The bridge uses `JoinFunction.name` as the MirModule function key,
//! not `join_func_name(id)`. This SSOT ensures all components agree on
//! the exact spelling of these critical function names.

/// Canonical name for loop exit/continuation function
///
/// Used in:
/// - Pattern 2, 3, 4, 5, 6, 7 (loop patterns with exit continuations)
/// - JoinInlineBoundary.continuation_funcs
/// - ExitLine/ExitMeta handling
///
/// Historical note: Some normalized shadow code uses "join_func_2" instead.
/// See K_EXIT_LEGACY for compatibility.
pub const K_EXIT: &str = "k_exit";

/// Legacy canonical name for k_exit in normalized shadow code
///
/// Used in:
/// - normalized_shadow/loop_true_break_once.rs (line 354, 460, 531)
///
/// TODO (Phase 256 P1.7): Unify with K_EXIT or keep as separate const
/// if semantic difference exists.
pub const K_EXIT_LEGACY: &str = "join_func_2";

/// Canonical name for loop step/body function
///
/// Used in:
/// - Pattern 1, 2, 3, 4, 5, 6, 7 (all loop patterns)
/// - LoopScopeShape inspection
/// - Normalized JoinIR validation
pub const LOOP_STEP: &str = "loop_step";

/// Canonical name for main entry function
///
/// Used in:
/// - Entry point detection
/// - JoinIR module main function naming
/// - MIR builder entry selection
pub const MAIN: &str = "main";

/// Canonical name for post-continuation function (if variant)
///
/// Used in:
/// - Pattern 3 with post-if computation (Phase 132-P4/133-P0)
/// - Normalized shadow exit routing
pub const POST_K: &str = "post_k";

/// Phase 284 P1: Canonical name for early return exit function
///
/// Used in:
/// - Pattern 4, 5 (loop patterns with early return)
/// - JoinInlineBoundary.continuation_funcs
/// - ExitLine/ExitMeta handling for return statements
pub const K_RETURN: &str = "k_return";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_names_are_not_empty() {
        assert!(!K_EXIT.is_empty());
        assert!(!K_EXIT_LEGACY.is_empty());
        assert!(!LOOP_STEP.is_empty());
        assert!(!MAIN.is_empty());
        assert!(!POST_K.is_empty());
        assert!(!K_RETURN.is_empty());
    }

    #[test]
    fn test_canonical_names_have_expected_values() {
        assert_eq!(K_EXIT, "k_exit");
        assert_eq!(K_EXIT_LEGACY, "join_func_2");
        assert_eq!(LOOP_STEP, "loop_step");
        assert_eq!(MAIN, "main");
        assert_eq!(POST_K, "post_k");
        assert_eq!(K_RETURN, "k_return");
    }

    #[test]
    fn test_k_exit_variants_differ() {
        // These should be different (historical reasons)
        assert_ne!(K_EXIT, K_EXIT_LEGACY);
    }
}
