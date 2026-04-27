//! Explicit `plan::facts` residue still surfaced through `facts/`.
//!
//! Keep this file as a narrow allowlist for live non-`plan/` callers only.
//! Items that are no longer imported through `control_flow::facts` should be
//! removed here instead of accumulating as a broad compatibility bundle.

#![allow(unused_imports)]

pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::facts::{
    feature_facts, loop_types, reject_reason, scan_shapes, skeleton_facts, LoopFacts,
};
