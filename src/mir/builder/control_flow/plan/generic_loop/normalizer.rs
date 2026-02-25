//! Phase 29bu P2: Generic structured loop normalizer (skeleton + features only).

use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::features::generic_loop_pipeline;
use crate::mir::builder::control_flow::plan::skeletons::generic_loop::alloc_generic_loop_v0_skeleton;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use super::facts_types::{GenericLoopV0Facts, GenericLoopV1Facts};

pub(in crate::mir::builder) fn normalize_generic_loop_v0(
    builder: &mut MirBuilder,
    facts: &GenericLoopV0Facts,
    ctx: &LoopPatternContext,
) -> Result<LoweredRecipe, String> {
    let mut skeleton = alloc_generic_loop_v0_skeleton(builder, &facts.loop_var)?;
    generic_loop_pipeline::apply_generic_loop_v0_pipeline(builder, facts, ctx, &mut skeleton)?;

    Ok(CorePlan::Loop(skeleton.plan))
}

pub(in crate::mir::builder) fn normalize_generic_loop_v1(
    builder: &mut MirBuilder,
    facts: &GenericLoopV1Facts,
    ctx: &LoopPatternContext,
) -> Result<LoweredRecipe, String> {
    let mut skeleton = alloc_generic_loop_v0_skeleton(builder, &facts.loop_var)?;
    generic_loop_pipeline::apply_generic_loop_v1_pipeline(builder, facts, ctx, &mut skeleton)?;

    Ok(CorePlan::Loop(skeleton.plan))
}
