//! Local-variable analysis support for route planning.
//!
//! These modules are physically owned by `support/locals`; callers should keep
//! using this semantic owner path instead of legacy analyzer names.

/// Mutable accumulator analysis support.
pub mod mutable_accumulator;

/// Pinned local analysis support.
pub mod pinned;
