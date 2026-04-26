//! Phase 121: StepTree → Normalized shadow lowering (dev-only)
//!
//! ## Purpose
//!
//! Establish minimal route from StepTree (structure SSOT) to Normalized form,
//! verifying parity with existing router for if-only patterns.
//!
//! ## Scope
//!
//! - **if-only**: Support if statements without loops
//! - **loop rejection**: Reject loops via capability guard
//!
//! ## Design Rules
//!
//! **Input SSOT**:
//! - `StepTree` + `StepTreeContract` (no facts re-analysis)
//! - Lowering decisions based only on contract information
//!
//! **Output**:
//! - `JoinModule` (Normalized dialect)
//! - Or "Normalized-equivalent intermediate" expressed in existing JoinIR types
//!
//! **Execution conditions**:
//! - dev-only: Only runs when `joinir_dev_enabled()` returns true
//! - strict: Only fail-fast on mismatch when `joinir_strict_enabled()` returns true
//!
//! **Prohibitions**:
//! - No fallback: Shadow conversion failure logs reason in dev-only, fail-fast in strict
//! - No direct env reads (must go through `src/config/env/*`)
//! - No hardcoding (no branching on fixture names or variable names)

pub mod anf;
pub mod available_inputs_collector; // Phase 126: available_inputs SSOT
pub mod builder;
pub mod common; // Phase 138: Common utilities (ReturnValueLowererBox)
pub mod contracts;
pub mod dev_pipeline;
pub mod env_layout;
pub mod exit_reconnector; // Phase 131 P1.5: Direct variable_map reconnection (Option B)
pub mod if_as_last_join_k;
pub mod legacy;
pub mod loop_true_break_once; // Phase 131: loop(true) break-once
mod loop_true_break_once_helpers;
pub mod loop_true_if_break_continue; // Phase 143 P0: loop(true) + if + break
pub mod normalized_verifier;
pub mod parity_contract;
pub mod post_if_post_k; // Phase 129-C: post-if with post_k continuation // Phase 145 P0: ANF (A-Normal Form) transformation
pub(crate) mod support;

pub use builder::StepTreeNormalizedShadowLowererBox;
pub use contracts::{CapabilityCheckResult, UnsupportedCapability};
pub use env_layout::EnvLayout;
pub use exit_reconnector::ExitReconnectorBox;
pub use parity_contract::{MismatchKind, ShadowParityResult}; // Phase 131 P1.5

pub const STEP_TREE_GATE_TAG: &str = "[phase132/gate]";
pub const STEP_TREE_DEBUG_TAG: &str = "[phase132/debug]";

#[inline]
pub fn log_step_tree_gate_root(func_name: &str) {
    if crate::config::env::joinir_dev::strict_planner_required_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        let msg = format!("{} StepTree root for '{}'", STEP_TREE_GATE_TAG, func_name);
        let _ = ring0.io.stderr_write(format!("{}\n", msg).as_bytes());
    }
}

#[cfg(test)]
mod tests; // Phase 143 R0: Separated unit tests
