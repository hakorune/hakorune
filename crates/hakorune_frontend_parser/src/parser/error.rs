use crate::tokenizer::{TokenType, TokenizeError};
use crate::{migration_transport::MigrationTransportKind, parser::GrammarProfile};
use thiserror::Error;

/// Parser error vocabulary.
#[derive(Error, Debug)]
pub enum ParseError {
    #[error(
        "[freeze:contract][{stable_reject_tag}] transport-only {transport_kind:?} under {profile:?} at line {line}"
    )]
    TransportOnly {
        row_id: &'static str,
        profile: GrammarProfile,
        transport_kind: MigrationTransportKind,
        stable_reject_tag: &'static str,
        line: usize,
    },

    #[error("[freeze:contract][{stable_reject_tag}] migration transport rejected at line {line}")]
    MigrationTransport {
        stable_reject_tag: &'static str,
        line: usize,
    },

    #[error("Unexpected token {found:?}, expected {expected} at line {line}")]
    UnexpectedToken {
        found: TokenType,
        expected: String,
        line: usize,
    },

    #[error(
        "Duplicate Box method '{name}' at line {duplicate_line}, column {duplicate_column}; first declared at line {first_line}, column {first_column}"
    )]
    DuplicateBoxMethod {
        name: String,
        first_line: usize,
        first_column: usize,
        duplicate_line: usize,
        duplicate_column: usize,
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

    #[error("[freeze:contract][{stable_reject_tag}] {detail} at line {line}")]
    GrammarContract {
        stable_reject_tag: &'static str,
        detail: String,
        line: usize,
    },

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
