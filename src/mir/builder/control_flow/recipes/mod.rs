//! Top-level owner surface for control-flow recipe infrastructure.
//!
//! `RecipeBody` and shape refs are recipes-owned.
//! Non-`plan/` consumers should depend on this module first.

mod body;
pub(in crate::mir::builder) mod loop_bundle_resolver_v0;
pub(in crate::mir::builder) mod loop_collect_using_entries_v0;
pub(in crate::mir::builder) mod loop_cond_break_continue;
pub(in crate::mir::builder) mod loop_cond_continue_only;
pub(in crate::mir::builder) mod loop_cond_continue_with_return;
pub(in crate::mir::builder) mod loop_cond_return_in_body;
pub(in crate::mir::builder) mod loop_cond_shared;
pub(in crate::mir::builder) mod loop_scan_methods_block_v0;
pub(in crate::mir::builder) mod loop_scan_methods_v0;
pub(in crate::mir::builder) mod loop_scan_phi_vars_v0;
pub(in crate::mir::builder) mod refs;
pub(in crate::mir::builder) mod scan_loop_segments;

pub(in crate::mir::builder) use self::body::{RecipeBody, StmtIdx, StmtRange};
