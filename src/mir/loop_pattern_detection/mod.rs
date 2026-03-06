//! Loop Pattern Detection Module
//!
//! Phase 188 Task 188-4: Route-shape detection helpers for JoinIR loop lowering.
//!
//! This module provides detection functions for 4 loop route shapes:
//! - `LoopSimpleWhile` (foundational)
//! - `LoopBreak` (early exit)
//! - `IfPhiJoin` (variable mutation)
//! - `LoopContinueOnly` (skip iteration)
//!
//! Phase 194+: Structure-based detection using LoopFeatures.
//! Route shapes are classified based on CFG structure, not function names.
//!
//! # Architecture
//!
//! ```
//! LoopForm → extract_features() → LoopFeatures → classify() → LoopPatternKind
//! ```
//!
//! Reference: docs/private/roadmap2/phases/phase-188-joinir-loop-pattern-expansion/design.md
//!
//! Legacy detection (Phase 188 name-based) lives under `legacy/` and is
//! re-exported here for compatibility.

mod classify;
mod features;
mod kind;
pub mod legacy;

pub use classify::{classify, classify_with_diagnosis};
pub(crate) use features::extract_features;
pub use features::LoopFeatures;
pub use kind::LoopPatternKind;
pub use legacy::*;
