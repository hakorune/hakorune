//! Phase 12: Unified nested loop depth1 module.
//!
//! This module consolidates 4 separate nested_loop_depth1 variants:
//! - nested_loop_depth1_break_continue_pure
//! - nested_loop_depth1_no_break_or_continue_pure
//! - nested_loop_depth1_methodcall
//! - nested_loop_depth1_no_break_or_continue
//!
//! The key difference between these variants is only the NestedLoopBodyProfile:
//!
//! | Kind                   | allow_calls | require_call | allow_break | allow_continue | trailing_continue |
//! |------------------------|-------------|--------------|-------------|----------------|-------------------|
//! | BreakContinuePure      | false       | false        | true        | true           | true              |
//! | NoBreakOrContinuePure  | false       | false        | false       | false          | false             |
//! | MethodCall             | true        | true         | true        | true           | false             |
//! | NoBreakOrContinue      | true        | true         | false       | false          | false             |

pub(in crate::mir::builder) mod facts;
pub(in crate::mir::builder) mod facts_helpers;
pub(in crate::mir::builder) mod facts_types;
pub(in crate::mir::builder) mod normalizer;

// Re-export the unified entry point
pub(in crate::mir::builder) use normalizer::try_lower_nested_loop_depth1;
