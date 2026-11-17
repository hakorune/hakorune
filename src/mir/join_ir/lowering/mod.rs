//! Shared JoinIR boundary and source-analysis helpers.
//!
//! The old MIR-to-JoinIR oracle entry points were retired in cleanup 5.
//! Current inline boundaries, source shape checks and value spaces remain.

pub mod canonical_names; // Phase 256 P1.7: SSOT for JoinIR function names (k_exit, loop_step, main)
pub mod carrier_info; // Phase 196: Carrier metadata for loop lowering
pub mod condition_env; // Phase 171-fix: Condition expression environment
pub mod error_tags; // Phase 86: Centralized error message formatting
pub(crate) mod exit_args_resolver; // Internal exit argument resolution
pub mod inline_boundary; // Phase 188-Impl-3: JoinIR→Host boundary
#[cfg(test)]
pub mod inline_boundary_builder; // Test-only builder pattern for JoinInlineBoundary
pub mod join_value_space; // Phase 201: Unified JoinIR ValueId allocation
pub mod loop_scope_shape;
pub mod loop_update_analyzer; // Phase 197: Update expression analyzer for carrier semantics
pub(crate) mod value_id_ranges; // Internal ValueId range management

// Test-only inline boundary constructor
#[cfg(test)]
pub use inline_boundary_builder::JoinInlineBoundaryBuilder;
