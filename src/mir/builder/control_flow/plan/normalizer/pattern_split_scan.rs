use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::features::split_scan_pipeline;
use crate::mir::builder::control_flow::plan::{LoweredRecipe, SplitScanPlan};
use crate::mir::builder::MirBuilder;

impl super::PlanNormalizer {
    /// SplitScan → CorePlan 変換（pipeline 経由）
    pub(in crate::mir::builder) fn normalize_split_scan(
        builder: &mut MirBuilder,
        parts: SplitScanPlan,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        split_scan_pipeline::apply_split_scan_pipeline(builder, parts, ctx)
    }
}
