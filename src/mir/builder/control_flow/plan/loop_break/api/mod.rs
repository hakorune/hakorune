//! Phase 263 P0.2: loop_break promotion API (entry point SSOT)
//!
//! This module is the single entry point for loop_break promotion logic.
//! All callers should use this module's exports instead of accessing internals.
//!
//! # Usage
//!
//! ```ignore
//! use super::loop_break::api::{try_promote, PromoteDecision};
//!
//! match try_promote(builder, condition, body, inputs, debug, verbose)? {
//!     PromoteDecision::Promoted(result) => { /* ... */ }
//!     PromoteDecision::NotApplicable(result) => { /* continue without promotion */ }
//!     PromoteDecision::Freeze(reason) => { /* fail-fast */ }
//! }
//! ```

mod promote_decision;
mod promote_runner;
