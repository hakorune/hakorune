//! Phase 106: Pattern2 Step Boxes (SSOT)
//!
//! Goal: keep the Pattern2 lowering orchestrator thin by splitting the pipeline
//! into explicit steps with clear boundaries.
//!
//! Each step should have a single responsibility and fail-fast with a clear tag
//! at the step boundary.

pub(crate) mod apply_policy_step_box;
pub(crate) mod body_local_derived_step_box;
pub(crate) mod carrier_updates_step_box;
pub(crate) mod emit_joinir_step_box;
pub(crate) mod gather_facts_step_box;
pub(crate) mod merge_step_box;
pub(crate) mod normalize_body_step_box;
pub(crate) mod post_loop_early_return_step_box;
