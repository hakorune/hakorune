//! loop_scan_phi_vars_v0: one-shape outer loop plan for selfhost _collect_phi_vars (BoxCount).
//!
//! Goal: accept exactly the outer loop shape in PhiInjectorBox._collect_phi_vars/2
//! which contains nested loops with break and conditional nested loops.

pub(in crate::mir::builder) mod facts;
pub(in crate::mir::builder) mod facts_helpers;
pub(in crate::mir::builder) mod facts_shape_routes;
pub(in crate::mir::builder) mod facts_types;
pub(in crate::mir::builder) mod if_branch_scan;
pub(in crate::mir::builder) mod nested_loop_handoff;
pub(in crate::mir::builder) mod nested_loop_recipe_handoff;
pub(in crate::mir::builder) mod pipeline;
pub(in crate::mir::builder) mod recipe;
pub(in crate::mir::builder) mod segment_linear;
pub(in crate::mir::builder) mod segment_nested_loop;

#[allow(unused_imports)]
pub(in crate::mir::builder) use facts::try_extract_loop_scan_phi_vars_v0_facts;
pub(in crate::mir::builder) use facts_types::LoopScanPhiVarsV0Facts;
pub(in crate::mir::builder) use pipeline::lower_loop_scan_phi_vars_v0;
