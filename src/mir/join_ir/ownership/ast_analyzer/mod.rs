//! Ownership Analyzer for real AST (`crate::ast::ASTNode`)
//!
//! Phase 63: analysis-only helper for real AST input.

mod core;
mod node_analysis;
mod loop_helper;
#[cfg(test)]
mod tests;

pub use core::AstOwnershipAnalyzer;
pub use loop_helper::analyze_loop;
