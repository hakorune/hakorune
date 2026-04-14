//! loop_scan_methods_v0: one-shape scan_methods outer loop plan (BoxCount).
//!
//! Goal: accept exactly one "FuncScannerBox._scan_methods outer loop" shape and
//! lower it via a standard loop skeleton + recursive statement lowering.

pub(in crate::mir::builder) mod facts;
pub(in crate::mir::builder) mod nested_loop_handoff;
pub(in crate::mir::builder) mod nested_loop_stmt_only;
pub(in crate::mir::builder) mod pipeline;
pub(in crate::mir::builder) mod recipe;
pub(in crate::mir::builder) mod segment_linear;
pub(in crate::mir::builder) mod segment_nested_loop;

pub(in crate::mir::builder) use facts::{
    try_extract_loop_scan_methods_v0_facts, LoopScanMethodsV0Facts,
};
