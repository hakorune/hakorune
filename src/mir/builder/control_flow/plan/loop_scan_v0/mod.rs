//! loop_scan_v0: one-shape scan loop plan (BoxCount).
//!
//! Goal: accept exactly one "scan comma / close-bracket" loop(cond) shape and
//! lower it via the existing loop skeleton + Parts.

pub(in crate::mir::builder) mod facts;
pub(in crate::mir::builder) mod helpers;
pub(in crate::mir::builder) mod nested_fallback_bridge;
pub(in crate::mir::builder) mod pipeline;
pub(in crate::mir::builder) mod recipe;
pub(in crate::mir::builder) mod route_finalize;

pub(in crate::mir::builder) use facts::{try_extract_loop_scan_v0_facts, LoopScanV0Facts};
