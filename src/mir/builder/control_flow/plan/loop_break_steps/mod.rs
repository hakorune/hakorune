//! Phase 106: loop_break step boxes (SSOT)
//!
//! Goal: keep the loop_break lowering orchestrator thin by splitting the pipeline
//! into explicit steps with clear boundaries.
//!
//! Each step should have a single responsibility and fail-fast with a clear tag
//! at the step boundary.

mod apply_policy_inputs;
mod carrier_updates_helpers;
mod normalize_body_complex_addends;
pub(crate) mod apply_policy_step_box;
pub(crate) mod body_local_derived_step_box;
pub(crate) mod carrier_updates_step_box;
pub(crate) mod emit_joinir_step_box;
pub(crate) mod gather_facts_step_box;
pub(crate) mod merge_step_box;
pub(crate) mod normalize_body_step_box;
pub(crate) mod post_loop_early_return_step_box;
