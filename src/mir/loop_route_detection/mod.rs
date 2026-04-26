//! Loop route-shape detection module.
//!
//! Current module declaration: `crate::mir::loop_route_detection`
//! Historical module token: `crate::mir::loop_pattern_detection`
//! Current physical path: `src/mir/loop_route_detection/`
//! Historical physical path token: `src/mir/loop_pattern_detection/`
//!
//! Phase 188 Task 188-4: Route-shape detection helpers for JoinIR loop lowering.
//!
//! This module provides the flat `LoopFeatures -> classify -> LoopRouteKind`
//! route-classification surface for the current loop route families:
//! - `LoopSimpleWhile` (foundational)
//! - `LoopBreak` (early exit)
//! - `IfPhiJoin` (variable mutation)
//! - `LoopContinueOnly` (skip iteration)
//! - `LoopTrueEarlyExit` (`loop(true)` + early exit)
//!
//! `NestedLoopMinimal` remains a route kind, but its live selection is owned by
//! the AST/StepTree routing path rather than the `LoopFeatures` classifier.
//!
//! Phase 194+: Structure-based flat route detection using LoopFeatures.
//! Flat route shapes are classified based on CFG structure, not function names.
//!
//! # Architecture
//!
//! ```
//! LoopForm → extract_features() → LoopFeatures → classify() → flat LoopRouteKind
//! ```
//!
//! Reference: docs/private/roadmap2/phases/phase-188-joinir-loop-pattern-expansion/design.md
//!
//! Legacy detection (Phase 188 name-based) lives under `legacy/`.
//! Current code should use the `crate::mir::loop_route_detection::*` module surface.

mod classify;
mod features;
mod kind;
mod legacy;

pub use classify::classify;
pub(crate) use features::extract_features;
pub use features::LoopFeatures;
pub use kind::LoopRouteKind;
pub use legacy::{
    break_condition_analyzer, function_scope_capture, loop_body_carrier_promoter,
    loop_body_cond_promoter, loop_condition_scope, mutable_accumulator_analyzer,
    pinned_local_analyzer, trim_loop_helper,
};
