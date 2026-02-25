//! Ownership Analyzer for real AST (`crate::ast::ASTNode`)
//!
//! Phase 63: analysis-only (dev-only via `normalized_dev` feature).

mod core;
mod node_analysis;
#[cfg(feature = "normalized_dev")]
mod loop_helper;
#[cfg(test)]
mod tests;

pub use core::AstOwnershipAnalyzer;
#[cfg(feature = "normalized_dev")]
pub use loop_helper::analyze_loop;
