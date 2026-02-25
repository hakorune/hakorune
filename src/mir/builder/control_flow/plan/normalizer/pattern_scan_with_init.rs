use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::features::scan_with_init_pipeline;
use crate::mir::builder::control_flow::plan::{LoweredRecipe, ScanWithInitPlan};
use crate::mir::builder::MirBuilder;

impl super::PlanNormalizer {
    /// ScanWithInit → CorePlan 変換（pipeline 経由）
    pub(in crate::mir::builder) fn normalize_scan_with_init(
        builder: &mut MirBuilder,
        parts: ScanWithInitPlan,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        scan_with_init_pipeline::apply_scan_with_init_pipeline(builder, parts, ctx)
    }
}
