use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopBundleResolverV0Recipe {
    pub step_var: String,
    pub body_exit_allowed: ExitAllowedBlockRecipe,
}
