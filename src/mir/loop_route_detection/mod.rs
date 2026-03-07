//! Loop route-shape detection module.
//!
//! Current module declaration: `crate::mir::loop_route_detection`
//! Historical module token: `crate::mir::loop_pattern_detection`
//! Current physical path: `src/mir/loop_route_detection/`
//! Historical physical path token: `src/mir/loop_pattern_detection/`
//!
//! Phase 188 Task 188-4: Route-shape detection helpers for JoinIR loop lowering.
//!
//! This module provides detection functions for the current loop route families:
//! - `LoopSimpleWhile` (foundational)
//! - `LoopBreak` (early exit)
//! - `IfPhiJoin` (variable mutation)
//! - `LoopContinueOnly` (skip iteration)
//! - `LoopTrueEarlyExit` (`loop(true)` + early exit)
//! - `NestedLoopMinimal` (1-level nested route)
//!
//! Phase 194+: Structure-based detection using LoopFeatures.
//! Route shapes are classified based on CFG structure, not function names.
//!
//! # Architecture
//!
//! ```
//! LoopForm → extract_features() → LoopFeatures → classify() → LoopRouteKind
//! ```
//!
//! Reference: docs/private/roadmap2/phases/phase-188-joinir-loop-pattern-expansion/design.md
//!
//! Legacy detection (Phase 188 name-based) lives under `legacy/`.
//! Current code should use the `crate::mir::loop_route_detection::*` module surface.

mod classify;
mod features;
mod kind;
pub mod legacy;

pub use classify::{classify, classify_with_diagnosis};
pub(crate) use features::extract_features;
pub use features::LoopFeatures;
pub use kind::LoopRouteKind;
pub use legacy::*;
