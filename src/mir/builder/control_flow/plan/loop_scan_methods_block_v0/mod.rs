//! loop_scan_methods_block_v0: scan_methods outer loop plan (BoxCount).
//!
//! Accepts a single one-shape loop(cond) where the scan window inner loop is wrapped
//! by a statement block `{ ... }` (ASTNode::Program / ScopeBox), which
//! `loop_scan_methods_v0` does not segmentize.

mod nested_loop_handoff;
mod nested_loop_stmt_only;
mod pipeline;
pub(in crate::mir::builder) mod recipe;
mod route_finalize;
mod segment_linear;
mod segment_nested_loop;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::facts::loop_scan_methods_block_v0::{
    try_extract_loop_scan_methods_block_v0_facts, LoopScanMethodsBlockV0Facts,
};
pub(in crate::mir::builder) use pipeline::lower_loop_scan_methods_block_v0;
#[allow(unused_imports)]
pub(in crate::mir::builder) use recipe::{LinearBlockRecipe, ScanSegment};
