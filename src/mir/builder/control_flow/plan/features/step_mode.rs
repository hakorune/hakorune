//! StepMode helper boundary for feature pipelines/ops.
//!
//! Delegates to plan-wide SSOT (`plan::step_mode`) to keep one definition.

pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::step_mode::inline_in_body_no_explicit_step;
