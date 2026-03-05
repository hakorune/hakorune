//! JoinIR integration for control flow
//!
//! This module contains JoinIR-related control flow logic:
//! - Route lowerers (patterns/)
//! - Routing logic (routing.rs) ✅
//! - Parity verification (parity_checker.rs) ✅ Phase 138
//! - Loop processing context (loop_context.rs) ✅ Phase 140-P5
//! - MIR block merging (merge/) ✅ Phase 4
//! - Unified tracing (trace.rs) ✅ Phase 195
//! - Control tree capability guard (control_tree_capability_guard.rs) ✅ Phase 112

pub(in crate::mir::builder) mod control_tree_capability_guard;
pub(in crate::mir::builder) mod api;
#[cfg(test)]
pub(in crate::mir::builder) mod loop_context;
pub(in crate::mir::builder) mod merge;
pub(in crate::mir::builder) mod parity_checker;
pub(in crate::mir::builder) mod patterns;
pub(in crate::mir::builder) mod routing;
pub(in crate::mir::builder) mod trace;

// Phase 140-P4-A: Re-export for loop_canonicalizer SSOT (crate-wide visibility)
pub(crate) use patterns::detect_skip_whitespace_pattern;

// Phase 104: Re-export read_digits(loop(true)) detection for loop_canonicalizer
pub(crate) use patterns::detect_read_digits_loop_true_pattern;

// Phase 142-P1: Re-export continue pattern detection for loop_canonicalizer
pub(crate) use patterns::detect_continue_pattern;

// Phase 143-P0: Re-export parse_number pattern detection for loop_canonicalizer
pub(crate) use patterns::detect_parse_number_pattern;
pub(crate) use patterns::detect_parse_string_pattern;

// Phase 91 P5b: Re-export escape skip pattern detection for loop_canonicalizer
pub(crate) use patterns::detect_escape_skip_pattern;
