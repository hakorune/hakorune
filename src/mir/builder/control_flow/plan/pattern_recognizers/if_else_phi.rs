//! If-Else PHI Pattern Detection
//!
//! Phase 287 P1: Extracted from ast_feature_extractor.rs
//!
//! This module detects if-else statements with potential PHI patterns.

use crate::ast::ASTNode;

/// Detect if-else statements with potential PHI pattern
///
/// Looks for if-else statements where both branches contain assignments.
/// This is a heuristic indicating a potential PHI merge point.
///
/// # Arguments
///
/// * `body` - Loop body statements to analyze
///
/// # Returns
///
/// `true` if at least one if-else statement with assignments in both branches is found
///
/// # Phase 264 P0: Conservative Implementation
///
/// Previously returned true if both if/else branches had assignments.
/// This was too broad - it caught simple conditional assignments like:
///   `if x then seg = "A" else seg = "B"`
///
/// Pattern3 is designed for if-sum patterns with arithmetic accumulation:
///   `sum = sum + (if x then 1 else 0)`
///
/// Phase 264 P0: Return false to prevent misclassification.
/// Effect: Loops with conditional assignment fall through to Pattern1.
///
/// Phase 264 P1: TODO - Implement accurate if-sum signature detection.
pub(crate) fn detect_if_else_phi_in_body(body: &[ASTNode]) -> bool {
    // Phase 282 P5: Proper if-else PHI detection (re-enabled with ExtractionBased safety)
    //
    // This function provides initial classification for Pattern3IfPhi.
    // The actual validation is done by extractors::pattern3::extract_loop_with_if_phi_parts()
    // which performs deep checks (PHI assignments, no control flow, etc.)
    //
    // Here we just check: Does the loop body contain an if-else statement?
    // This allows Pattern3 to be attempted, and extraction will validate.

    for stmt in body {
        if matches!(stmt, ASTNode::If { else_body: Some(_), .. }) {
            return true;  // Found if-else
        }
    }
    false  // No if-else found
}

/// Phase 212.5: Detect ANY if statement in loop body (structural detection)
///
/// This function detects any if statement, regardless of whether it has an else branch.
/// Used for routing single-carrier if-update patterns to Pattern 3.
///
/// # Arguments
///
/// * `body` - Loop body statements to analyze
///
/// # Returns
///
/// `true` if at least one if statement is found (with or without else)
#[allow(dead_code)]
fn detect_if_in_body(body: &[ASTNode]) -> bool {
    for node in body {
        if let ASTNode::If { .. } = node {
            return true;
        }
    }
    false
}
