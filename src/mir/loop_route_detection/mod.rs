//! Loop route-shape detection module.
//!
//! Current module declaration: `crate::mir::loop_route_detection`
//! Historical module token: `crate::mir::loop_pattern_detection`
//! Current physical path: `src/mir/loop_route_detection/`
//! Historical physical path token: `src/mir/loop_pattern_detection/`
//!
//! Route-shape classification helpers for JoinIR loop lowering.
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
//! Implementation-backed support modules live under stable semantic owner paths
//! in `support/`.
//!
//! Current route selection should use `classify`, `LoopFeatures`, and
//! `LoopRouteKind`, not legacy route-shape function entry points.

mod classify;
mod features;
mod kind;
pub mod support;

pub use classify::classify;
pub(crate) use features::extract_features;
pub use features::LoopFeatures;
pub use kind::LoopRouteKind;
