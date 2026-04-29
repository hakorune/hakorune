//! Phase 176-2 / Phase 179 / Phase 184: Carrier Update Emission
//!
//! Converts UpdateExpr (from LoopUpdateAnalyzer) into JoinIR instructions
//! that compute the updated carrier value.
//!
//! This module is extracted from loop_with_break_minimal.rs to improve
//! modularity and single responsibility.
//!
//! Phase 184: Added UpdateEnv support for body-local variable resolution.

#[cfg(test)]
mod tests;
mod with_env;

pub use with_env::emit_carrier_update_with_env;
