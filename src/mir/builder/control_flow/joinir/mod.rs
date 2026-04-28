//! JoinIR integration for control flow
//!
//! This module contains JoinIR-related control flow logic:
//! - Route lowerers (active module surface and physical path: route_entry/)
//! - Routing logic (routing.rs) ✅
//! - Parity verification (parity_checker.rs) ✅ Phase 138
//! - Loop processing context (loop_context.rs) ✅ Phase 140-P5
//! - MIR block merging (merge/) ✅ Phase 4
//! - Unified tracing (trace.rs) ✅ Phase 195
//! - Control tree capability guard (control_tree_capability_guard.rs) ✅ Phase 112

pub(in crate::mir::builder) mod control_tree_capability_guard;
#[cfg(test)]
pub(in crate::mir::builder) mod loop_context;
pub(in crate::mir::builder) mod merge;
pub(in crate::mir::builder) mod parity_checker;
pub(in crate::mir::builder) mod route_entry;
pub(in crate::mir::builder) mod routing;
pub(in crate::mir::builder) mod trace;

// Phase 140-P4-A: Re-export skip_whitespace shape detection for loop_canonicalizer
pub(crate) use route_entry::detect_skip_whitespace_shape;

// Phase 104: Re-export read_digits(loop(true)) shape detection for loop_canonicalizer
pub(crate) use route_entry::detect_read_digits_loop_true_shape;

// Phase 142-P1: Re-export continue shape detection for loop_canonicalizer
pub(crate) use route_entry::detect_continue_shape;

// Phase 143-P0: Re-export parse_number / parse_string shape detection for loop_canonicalizer
pub(crate) use route_entry::detect_parse_number_shape;
pub(crate) use route_entry::detect_parse_string_shape;

// Phase 91 P5b: Re-export escape skip pattern detection for loop_canonicalizer
pub(crate) use route_entry::detect_escape_skip_shape;
