use crate::mir::builder::control_flow::plan::facts::no_exit_block::NoExitBlockRecipe;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopCollectUsingEntriesV0Recipe {
    pub body_no_exit: NoExitBlockRecipe,
}
