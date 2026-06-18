//! Passive frontend AST crate scaffold.
//!
//! This crate is the future owner for parser-independent AST data. It must not
//! depend on parser, runtime, backend, MIR, or Box implementations.
//!
//! Current row is scaffold-only: the main crate still owns the active AST
//! implementation until a later compatibility facade wiring row.

pub const FRONTEND_AST_CRATE_READY: bool = true;

mod attrs;
mod build_predicate;
mod literal;
mod operators;
pub mod rune_profile;
mod span;
pub use attrs::{DeclarationAttrs, RuneAttr};
pub use build_predicate::BuildPredicate;
pub use literal::LiteralValue;
pub use operators::{BinaryOperator, UnaryOperator};
pub use span::Span;

/// Passive boundary marker for the first frontend AST split.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrontendAstBoundary;

impl FrontendAstBoundary {
    pub const fn name(self) -> &'static str {
        "hakorune-frontend-ast"
    }
}
