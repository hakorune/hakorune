//! Ownership Analyzer for real AST (`crate::ast::ASTNode`)
//!
//! Phase 63: analysis-only helper for real AST input.

mod core;
mod loop_helper;
mod node_analysis;
#[cfg(test)]
mod tests;

pub use core::AstOwnershipAnalyzer;
pub use loop_helper::analyze_loop;
