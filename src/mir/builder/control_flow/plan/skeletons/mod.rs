//! Phase 29bt P0: CorePlan skeleton allocation helpers (no AST analysis).

pub(in crate::mir::builder) mod generic_loop;
pub(in crate::mir::builder) mod loop_true;
#[cfg(test)]
pub(in crate::mir::builder) mod scan_with_init;
#[cfg(test)]
pub(in crate::mir::builder) mod split_scan;
