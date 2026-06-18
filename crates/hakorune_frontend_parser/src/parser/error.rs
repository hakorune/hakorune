use crate::tokenizer::{TokenType, TokenizeError};
use thiserror::Error;

/// Parser error vocabulary.
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token {found:?}, expected {expected} at line {line}")]
    UnexpectedToken {
        found: TokenType,
        expected: String,
        line: usize,
    },

    #[error("Unexpected end of file")]
    UnexpectedEOF,

    #[error("Invalid expression at line {line}")]
    InvalidExpression { line: usize },

    #[error("Invalid statement at line {line}")]
    InvalidStatement { line: usize },

    #[error("Delegate lowering error: {message} at line {line}")]
    DelegateLowering { message: String, line: usize },

    #[error("Invalid match pattern: {detail} at line {line}")]
    InvalidMatchPattern { detail: String, line: usize },

    #[error("Unsupported identifier '{name}' at line {line}")]
    UnsupportedIdentifier { name: String, line: usize },

    #[error("Circular dependency detected between static boxes: {cycle}")]
    CircularDependency { cycle: String },

    #[error("🚨 Infinite loop detected in parser at {location} - token: {token:?} at line {line}")]
    InfiniteLoop {
        location: String,
        token: TokenType,
        line: usize,
    },

    #[error("🔥 Transparency system removed: {suggestion} at line {line}")]
    TransparencySystemRemoved { suggestion: String, line: usize },

    #[error(
        "Unsupported namespace '{name}' at line {line}. Only 'nyashstd' is supported in Phase 0."
    )]
    UnsupportedNamespace { name: String, line: usize },

    #[error("Expected identifier at line {line}")]
    ExpectedIdentifier { line: usize },

    #[error("Tokenize error: {0}")]
    TokenizeError(#[from] TokenizeError),

    #[error("Build cfg error: {message} at line {line}")]
    BuildCfg { message: String, line: usize },
}
