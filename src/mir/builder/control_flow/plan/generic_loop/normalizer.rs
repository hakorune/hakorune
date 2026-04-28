//! Phase 29bu P2: Generic structured loop normalizer (skeleton + features only).

use super::facts_types::GenericLoopV1Facts;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::features::generic_loop_pipeline;
use crate::mir::builder::control_flow::plan::skeletons::generic_loop::alloc_generic_loop_v0_skeleton;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn normalize_generic_loop_v1(
    builder: &mut MirBuilder,
    facts: &GenericLoopV1Facts,
    ctx: &LoopRouteContext,
) -> Result<LoweredRecipe, String> {
    let mut skeleton = alloc_generic_loop_v0_skeleton(builder, &facts.loop_var)?;
    generic_loop_pipeline::apply_generic_loop_v1_pipeline(builder, facts, ctx, &mut skeleton)?;

    Ok(CorePlan::Loop(skeleton.plan))
}
