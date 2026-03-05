//! Phase 193: AST Feature Extractor Box
//!
//! Phase 287 P1: Facade pattern - re-exports recognizers from pattern_recognizers/
//!
//! Modularized feature extraction from loop AST nodes.
//! Separated from router.rs to improve reusability and testability.
//!
//! This module provides pure functions for analyzing loop body AST to determine
//! structural characteristics (break/continue presence, if-else PHI patterns, carrier counts).
//!
//! # Design Philosophy
//!
//! - **Pure functions**: No side effects, only AST analysis
//! - **High reusability**: Used by router, future Pattern 5/6, and pattern analysis tools
//! - **Independent testability**: Can be unit tested without MirBuilder context
//! - **Extension-friendly**: Easy to add new feature detection methods
//! - **Facade pattern**: Re-exports from pattern_recognizers/ for backward compatibility
//!
//! # Phase 33-23: Refactoring
//!
//! - Break condition analysis moved to `break_condition_analyzer.rs`
//! - This module now focuses on high-level feature extraction
//! - Delegates to specialized analyzers for break/continue logic
//!
//! # Phase 287 P1: Modularization
//!
//! - Individual recognizers extracted to `pattern_recognizers/` subdirectory
//! - This file now acts as a facade, re-exporting public APIs
//! - Internal implementation moved to specialized modules
//!
//! # Boundary (Phase 110)
//!
//! - **Routing SSOT**: Pattern routing and feature classification use this module (and
//!   `BreakConditionAnalyzer`) as the SSOT in production code paths.
//! - **Structure SSOT**: `crate::mir::control_tree` (StepTree) describes *control structure only*
//!   and must not drive routing decisions yet; it is used for dev-only observation and parity checks.

use crate::ast::ASTNode;
use crate::mir::loop_pattern_detection::LoopFeatures;

// Phase 287 P1: Use recognizer modules from parent
use super::pattern_recognizers;

// Re-export continue/break/return detection
pub(crate) use pattern_recognizers::continue_break::{
    detect_break_in_body, detect_continue_in_body, detect_return_in_body,
    find_first_control_flow_stmt,
};

// Re-export infinite loop detection
use pattern_recognizers::infinite_loop::detect_infinite_loop;

// Re-export if-else phi detection
use pattern_recognizers::if_else_phi::detect_if_else_phi_in_body;

// Re-export carrier count estimation
use pattern_recognizers::carrier_count::count_carriers_in_body;

// Re-export parse_number pattern detection
pub use pattern_recognizers::parse_number::{
    detect_parse_number_pattern, detect_read_digits_loop_true_pattern,
};

// Re-export parse_string pattern detection
pub use pattern_recognizers::parse_string::{
    detect_continue_pattern, detect_parse_string_pattern,
};

// Re-export skip_whitespace pattern detection
pub use pattern_recognizers::skip_whitespace::detect_skip_whitespace_pattern;

// Re-export escape pattern recognizer (existing module, not moved in P1)
pub use super::escape_pattern_recognizer::detect_escape_skip_pattern;

/// Extract full feature set from loop body AST
///
/// This is the main entry point for feature extraction. It analyzes the loop body
/// to determine all relevant characteristics for pattern classification.
///
/// # Arguments
///
/// * `condition` - Loop condition AST node (Phase 131-11: for infinite loop detection)
/// * `body` - Loop body statements to analyze
/// * `has_continue` - Pre-computed continue presence (for optimization)
/// * `has_break` - Pre-computed break presence (for optimization)
///
/// # Returns
///
/// A LoopFeatures struct containing all detected structural characteristics
pub(crate) fn extract_features(
    condition: &ASTNode,
    body: &[ASTNode],
    has_continue: bool,
    has_break: bool,
) -> LoopFeatures {
    // Detect if-else statements with PHI pattern
    let has_if_else_phi = detect_if_else_phi_in_body(body);

    // Phase 264 P0: Use has_if_else_phi to prevent misclassification
    // Previously used detect_if_in_body() which returned true for ANY if statement.
    // This caused simple conditional assignments to be classified as if_phi_join
    // (legacy label: Pattern3IfPhi). Now we use has_if_else_phi which only returns
    // true for actual if-sum patterns.
    let has_if = has_if_else_phi;

    // Count carrier variables (approximation based on assignments)
    let carrier_count = count_carriers_in_body(body);

    // Phase 131-11: Detect infinite loop (condition == true)
    let is_infinite_loop = detect_infinite_loop(condition);

    LoopFeatures {
        has_break,
        has_continue,
        has_if,
        has_if_else_phi,
        carrier_count,
        break_count: if has_break { 1 } else { 0 },
        continue_count: if has_continue { 1 } else { 0 },
        is_infinite_loop,
        ..Default::default() // Phase 188.1: Use Default for nesting fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_body() {
        let empty: Vec<ASTNode> = vec![];
        assert!(!detect_continue_in_body(&empty));
        assert!(!detect_break_in_body(&empty));
        assert_eq!(count_carriers_in_body(&empty), 0);
    }
}
