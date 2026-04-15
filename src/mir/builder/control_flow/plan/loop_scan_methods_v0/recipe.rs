//! Compat wrapper/re-export for plan-local callers.
//!
//! Purpose:
//! - Keep existing plan-local imports working during phase-29bq cleanup.
//! - The recipe definition is now recipes-owned (see `recipes/loop_scan_methods_v0.rs`).
//! - Non-plan callers should depend on the recipes-owned module first.

pub(in crate::mir::builder) type LoopScanMethodsV0Recipe =
    crate::mir::builder::control_flow::recipes::loop_scan_methods_v0::LoopScanMethodsV0Recipe;
pub(in crate::mir::builder) type LoopScanSegment =
    crate::mir::builder::control_flow::recipes::loop_scan_methods_v0::LoopScanSegment;
pub(in crate::mir::builder) type NestedLoopRecipe =
    crate::mir::builder::control_flow::recipes::loop_scan_methods_v0::NestedLoopRecipe;
