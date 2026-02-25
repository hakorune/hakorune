//! SplitScan pipeline (skeleton + features).

use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::features::split_scan_ops;
use crate::mir::builder::control_flow::plan::skeletons::split_scan::alloc_split_scan_skeleton;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe, SplitScanPlan};
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn apply_split_scan_pipeline(
    builder: &mut MirBuilder,
    parts: SplitScanPlan,
    ctx: &LoopPatternContext,
) -> Result<LoweredRecipe, String> {
    let skeleton = alloc_split_scan_skeleton(builder, &parts)?;
    let loop_plan = split_scan_ops::build_split_scan_plan(builder, &parts, ctx, &skeleton)?;
    Ok(CorePlan::Loop(loop_plan))
}
