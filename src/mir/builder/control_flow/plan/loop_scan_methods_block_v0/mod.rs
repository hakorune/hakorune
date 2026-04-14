//! loop_scan_methods_block_v0: scan_methods outer loop plan (BoxCount).
//!
//! Accepts a single one-shape loop(cond) where the scan window inner loop is wrapped
//! by a statement block `{ ... }` (ASTNode::Program / ScopeBox), which
//! `loop_scan_methods_v0` does not segmentize.

mod facts;
mod facts_helpers;
mod facts_recipe_builder;
mod facts_shape_routes;
mod facts_types;
mod nested_loop_handoff;
mod nested_loop_stmt_only;
mod pipeline;
mod recipe;
mod route_finalize;
mod segment_linear;
mod segment_nested_loop;

pub(in crate::mir::builder) use facts::try_extract_loop_scan_methods_block_v0_facts;
pub(in crate::mir::builder) use facts_types::LoopScanMethodsBlockV0Facts;
pub(in crate::mir::builder) use pipeline::lower_loop_scan_methods_block_v0;
pub(in crate::mir::builder) use recipe::{LinearBlockRecipe, ScanSegment};
