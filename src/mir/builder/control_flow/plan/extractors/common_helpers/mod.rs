//! Common extraction helpers for core loop routes.
//!
//! Phase 282 P9a: Extracted from loop route extractors
//! to eliminate common duplication.
//! Phase 29ai P10: Moved to plan-layer SSOT (JoinIR keeps a wrapper path).
//!
//! # Design Principles
//!
//! - **Pure Functions**: No side effects, no builder mutations
//! - **Fail-Fast**: Err for logic bugs, Ok(None) for non-matches
//! - **Configurability**: ControlFlowDetector for route-specific behavior
//! - **Scope-Limited**: Common detection only, route-specific logic excluded
//!
//! # Groups (P9a Scope-Limited)
//!
//! 1. Control Flow Counting (count_control_flow) - Universal counter
//! 2. Control Flow Detection (has_break_statement, has_continue_statement, etc.) - Common detection
//! 3. Condition Validation (extract_loop_variable, is_true_literal) - Condition helpers
//! 4. Loop Increment Extraction (extract_loop_increment_plan) - Common plan helper
//! 5. loop_true_early_exit-specific helpers
//!    validate_continue_at_end, validate_break_in_simple_if) - NOT generalized
//!
//! **IMPORTANT**: route-specific interpretation logic (e.g., if_phi_join nested_if)
//! is EXCLUDED.
//! Such logic remains in individual extractor files to maintain clear SSOT boundaries.

#![allow(dead_code)]

mod condition;
mod control_flow;
mod increment;
mod loop_true_early_exit;
mod walk;

#[allow(unused_imports)]
pub(crate) use condition::{extract_loop_variable, is_true_literal};
#[allow(unused_imports)]
pub(crate) use control_flow::{
    count_control_flow, find_if_else_statement, has_break_statement, has_continue_statement,
    has_control_flow_statement, has_if_else_statement, has_if_statement, has_return_statement,
    ControlFlowCounts, ControlFlowDetector,
};
pub(crate) use increment::extract_loop_increment_plan;
#[allow(unused_imports)]
pub(crate) use loop_true_early_exit::{validate_break_in_simple_if, validate_continue_at_end};
pub(crate) use walk::{flatten_stmt_list, strip_trailing_continue_view, walk_stmt_list};

#[cfg(test)]
mod tests;
