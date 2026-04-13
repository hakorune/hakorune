//! Phase 29bt P0: Feature deltas for CorePlan (apply-only).
//!
//! SSOT: docs/development/current/main/design/plan-dir-shallowing-ssot.md
//! shallowing: moved from subdirs (coreloop_skeleton/, loop_cond_*_pipeline/)

pub(in crate::mir::builder) mod body_view;
pub(in crate::mir::builder) mod carriers;
pub(in crate::mir::builder) mod edgecfg_stubs;
pub(in crate::mir::builder) mod exit_branch;
pub(in crate::mir::builder) mod exit_map;
pub(in crate::mir::builder) mod if_branch_lowering;
pub(in crate::mir::builder) mod if_join;
pub(in crate::mir::builder) mod loop_carriers;

// Flattened from coreloop_skeleton/
pub(in crate::mir::builder) mod coreloop_frame;

pub(in crate::mir::builder) mod carrier_merge;
pub(in crate::mir::builder) mod conditional_update_join;
pub(in crate::mir::builder) mod exit_if_map;
pub(in crate::mir::builder) mod generic_loop_body;
pub(in crate::mir::builder) mod generic_loop_pipeline;
pub(in crate::mir::builder) mod generic_loop_step;
pub(in crate::mir::builder) mod loop_cond_continue_with_return_pipeline;
pub(in crate::mir::builder) mod loop_cond_return_in_body_join;
pub(in crate::mir::builder) mod loop_cond_return_in_body_phi_materializer;
pub(in crate::mir::builder) mod loop_cond_return_in_body_pipeline;
pub(in crate::mir::builder) mod loop_true_break_continue_pipeline;
pub(in crate::mir::builder) mod nested_loop_depth1;
pub(in crate::mir::builder) mod step_mode;

// Flattened from loop_cond_break_continue_pipeline/
pub(in crate::mir::builder) mod loop_cond_bc;
pub(in crate::mir::builder) mod loop_cond_bc_continue_if;
pub(in crate::mir::builder) mod loop_cond_bc_else_patterns;
pub(in crate::mir::builder) mod loop_cond_bc_item;
pub(in crate::mir::builder) mod loop_cond_bc_item_stmt;
pub(in crate::mir::builder) mod loop_cond_bc_nested_carriers;
pub(in crate::mir::builder) mod loop_cond_bc_util;

// Flattened from loop_cond_continue_only_pipeline/
pub(in crate::mir::builder) mod loop_cond_co_block;
pub(in crate::mir::builder) mod loop_cond_co_continue_if;
pub(in crate::mir::builder) mod loop_cond_co_group_if;
pub(in crate::mir::builder) mod loop_cond_co_helpers;
pub(in crate::mir::builder) mod loop_cond_co_pipeline;
pub(in crate::mir::builder) mod loop_cond_co_stmt;

// Re-exports for flattened pipeline modules (backwards compatibility)
pub(in crate::mir::builder) use loop_cond_bc::lower_loop_cond_break_continue;
pub(in crate::mir::builder) use loop_cond_co_pipeline::lower_loop_cond_continue_only;
