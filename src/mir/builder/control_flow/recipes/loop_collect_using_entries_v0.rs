//! Recipe definition for loop_collect_using_entries_v0 (recipes-owned surface).
//!
//! This keeps the no-exit body contract under the shared recipes owner while
//! lowering stays in the owner-local family.

use crate::mir::builder::control_flow::facts::no_exit_block::NoExitBlockRecipe;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopCollectUsingEntriesV0Recipe {
    pub body_no_exit: NoExitBlockRecipe,
}
