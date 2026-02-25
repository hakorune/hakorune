//! Phase 263 P0.2: Pattern2 promotion API (entry point SSOT)
//!
//! This module is the single entry point for Pattern2 promotion logic.
//! All callers should use this module's exports instead of accessing internals.
//!
//! # Usage
//!
//! ```ignore
//! use super::pattern2::api::{try_promote, PromoteDecision};
//!
//! match try_promote(builder, condition, body, inputs, debug, verbose)? {
//!     PromoteDecision::Promoted(result) => { /* ... */ }
//!     PromoteDecision::NotApplicable(result) => { /* continue without promotion */ }
//!     PromoteDecision::Freeze(reason) => { /* fail-fast */ }
//! }
//! ```

mod promote_decision;
mod promote_runner;

// Re-export the SSOT types and functions
pub(in crate::mir::builder) use promote_decision::PromoteDecision;
pub(in crate::mir::builder) use promote_runner::try_promote;
