/// Phase 231: Expression lowering context
///
/// Defines the context in which an expression is being lowered, which affects
/// what AST nodes are supported and how they're translated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprContext {
    /// Loop condition expression (limited subset: comparisons, logical ops)
    Condition,

    /// General expression (future: method calls, box ops, etc.)
    #[allow(dead_code)] // Phase 231: Not yet implemented
    General,
}

/// Phase 231: Expression lowering error
///
/// Explicit error types allow callers to handle different failure modes
/// (e.g., fall back to legacy path for unsupported nodes).
#[derive(Debug)]
pub enum ExprLoweringError {
    /// AST node type not supported in this context
    UnsupportedNode(String),

    /// Variable not found in any scope
    VariableNotFound(String),

    /// Type error during lowering (e.g., non-boolean in condition)
    TypeError(String),

    /// Internal lowering error (from condition_lowerer)
    LoweringError(String),
}

impl std::fmt::Display for ExprLoweringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprLoweringError::UnsupportedNode(msg) => write!(f, "Unsupported node: {}", msg),
            ExprLoweringError::VariableNotFound(name) => write!(f, "Variable not found: {}", name),
            ExprLoweringError::TypeError(msg) => write!(f, "Type error: {}", msg),
            ExprLoweringError::LoweringError(msg) => write!(f, "Lowering error: {}", msg),
        }
    }
}
