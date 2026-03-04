//! Pattern5InfiniteEarlyExit pipeline (skeleton + features).

use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::domain::Pattern5InfiniteEarlyExitPlan;
use crate::mir::builder::control_flow::plan::features::pattern5_infinite_early_exit_ops;
use crate::mir::builder::control_flow::plan::skeletons::loop_true::alloc_loop_true_skeleton;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn apply_pattern5_infinite_early_exit_pipeline(
    builder: &mut MirBuilder,
    parts: Pattern5InfiniteEarlyExitPlan,
    ctx: &LoopPatternContext,
) -> Result<LoweredRecipe, String> {
    let skeleton = alloc_loop_true_skeleton(builder)?;
    let loop_plan = pattern5_infinite_early_exit_ops::build_pattern5_infinite_early_exit_plan(
        builder, &parts, ctx, &skeleton,
    )?;
    Ok(CorePlan::Loop(loop_plan))
}
