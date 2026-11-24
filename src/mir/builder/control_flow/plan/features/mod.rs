//! Phase 29bt P0: Feature deltas for CorePlan (apply-only).
//!
//! SSOT: docs/development/current/main/design/plan-dir-shallowing-ssot.md
//! shallowing: moved from subdirs (coreloop_skeleton/, loop_cond_*_pipeline/)

pub(in crate::mir::builder) mod body_view;
pub(in crate::mir::builder) mod carriers;
pub(in crate::mir::builder) mod edgecfg_stubs;
pub(in crate::mir::builder) mod if_branch_lowering;
pub(in crate::mir::builder) mod if_join;
pub(in crate::mir::builder) mod loop_carriers;

// Flattened from coreloop_skeleton/
pub(in crate::mir::builder) mod coreloop_frame;

pub(in crate::mir::builder) mod carrier_merge;
pub(in crate::mir::builder) mod exit_if_map;
pub(in crate::mir::builder) mod loop_break_composite_source;
pub(in crate::mir::builder) mod step_mode;

// Flattened from loop_cond_break_continue_pipeline/
pub(in crate::mir::builder) mod loop_cond_bc_util;
