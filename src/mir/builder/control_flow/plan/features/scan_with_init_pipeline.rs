//! ScanWithInit pipeline (skeleton + features).

use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::features::scan_with_init_ops;
use crate::mir::builder::control_flow::plan::skeletons::scan_with_init::alloc_scan_with_init_skeleton;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe, ScanWithInitPlan};
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn apply_scan_with_init_pipeline(
    builder: &mut MirBuilder,
    parts: ScanWithInitPlan,
    ctx: &LoopPatternContext,
) -> Result<LoweredRecipe, String> {
    let skeleton = alloc_scan_with_init_skeleton(builder, &parts)?;
    let loop_plan = scan_with_init_ops::build_scan_with_init_plan(builder, &parts, ctx, &skeleton)?;
    Ok(CorePlan::Loop(loop_plan))
}
