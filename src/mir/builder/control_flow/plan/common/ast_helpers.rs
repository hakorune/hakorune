//! Phase 255 P2: Common AST helpers for pattern lowering

use crate::ast::{ASTNode, Span};

/// Create Variable ASTNode with unknown span
///
/// # Phase 255 P2
///
/// This helper function eliminates duplicate var() implementations across
/// pattern lowering code. Previously, there were 7 copies of this function
/// scattered across different files.
///
/// # Usage
///
/// ```ignore
/// use super::common::var;
///
/// let i_var = var("i");  // ASTNode::Variable { name: "i", span: Span::unknown() }
/// ```
pub(crate) fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}
