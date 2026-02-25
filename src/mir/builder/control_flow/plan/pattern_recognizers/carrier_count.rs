//! Carrier Count Estimation
//!
//! Phase 287 P1: Extracted from ast_feature_extractor.rs
//!
//! This module provides heuristic-based carrier variable counting.

use crate::ast::ASTNode;

/// Count carrier variables (variables assigned in loop body)
///
/// This is a heuristic: counts assignment statements as a proxy for carriers.
/// A more precise implementation would track which specific variables are assigned.
///
/// # Arguments
///
/// * `body` - Loop body statements to analyze
///
/// # Returns
///
/// Count of distinct carrier variables (0 or 1 in current implementation)
///
/// # Notes
///
/// Current implementation returns 0 or 1 (at least one assignment present).
/// Future enhancement: track individual variable assignments for precise carrier count.
pub(crate) fn count_carriers_in_body(body: &[ASTNode]) -> usize {
    let mut count = 0;
    for node in body {
        match node {
            ASTNode::Assignment { .. } => count += 1,
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                count += count_carriers_in_body(then_body);
                if let Some(else_body) = else_body {
                    count += count_carriers_in_body(else_body);
                }
            }
            _ => {}
        }
    }
    // Return at least 1 if we have assignments, otherwise 0
    if count > 0 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_body() {
        let empty: Vec<ASTNode> = vec![];
        assert_eq!(count_carriers_in_body(&empty), 0);
    }
}
