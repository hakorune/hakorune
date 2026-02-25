use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::features::pattern5_infinite_early_exit_pipeline;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe, Pattern5InfiniteEarlyExitPlan};
use crate::mir::builder::MirBuilder;

impl super::PlanNormalizer {
    /// Pattern5InfiniteEarlyExit → CorePlan 変換（pipeline 経由）
    pub(in crate::mir::builder) fn normalize_pattern5_infinite_early_exit(
        builder: &mut MirBuilder,
        parts: Pattern5InfiniteEarlyExitPlan,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        pattern5_infinite_early_exit_pipeline::apply_pattern5_infinite_early_exit_pipeline(
            builder, parts, ctx,
        )
    }
}
