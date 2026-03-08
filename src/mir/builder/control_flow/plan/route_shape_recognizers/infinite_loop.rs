//! Infinite Loop Detection
//!
//! Phase 287 P1: Extracted from ast_feature_extractor.rs
//!
//! This module detects infinite loop route shapes (condition == true).

use crate::ast::ASTNode;

/// Phase 131-11: Detect infinite loop (condition == Literal(Bool(true)))
///
/// # Arguments
///
/// * `condition` - Loop condition AST node
///
/// # Returns
///
/// `true` if condition is a boolean literal with value true
pub(crate) fn detect_infinite_loop(condition: &ASTNode) -> bool {
    matches!(
        condition,
        ASTNode::Literal {
            value: crate::ast::LiteralValue::Bool(true),
            ..
        }
    )
}
