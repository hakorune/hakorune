//! Recipe definition for loop_bundle_resolver_v0 (recipes-owned surface).
//!
//! This keeps the bundle-resolver body contract near other recipe vocabulary while
//! leaving lowering/pipeline logic under the owner-local family.

use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopBundleResolverV0Recipe {
    pub step_var: String,
    pub body_exit_allowed: ExitAllowedBlockRecipe,
}
