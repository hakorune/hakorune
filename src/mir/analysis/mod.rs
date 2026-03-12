//! MIR analysis-only views (no AST rewrite).
//!
//! This module provides conservative matchers that *observe* AST shapes without
//! performing any normalization or equivalence rewrites.

pub mod expr_view;
