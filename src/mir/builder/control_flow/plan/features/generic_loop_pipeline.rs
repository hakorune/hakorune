//! GenericLoop pipeline (ordered feature application).

use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::features::{generic_loop_body, generic_loop_handoff};
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::{
    GenericLoopV0Facts, GenericLoopV1Facts,
};
use crate::mir::builder::control_flow::plan::skeletons::generic_loop::GenericLoopSkeleton;
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn apply_generic_loop_v0_pipeline(
    builder: &mut MirBuilder,
    facts: &GenericLoopV0Facts,
    ctx: &LoopRouteContext,
    skeleton: &mut GenericLoopSkeleton,
) -> Result<(), String> {
    let pre_body_map = builder.variable_ctx.variable_map.clone();
    // Keep loop-step lowering anchored to the current header PHI in v0 route.
    // Without this rebinding, post_body_map can retain the init value and
    // step-only loops (e.g. i = i + k) become constant updates.
    builder
        .variable_ctx
        .variable_map
        .insert(facts.loop_var.clone(), skeleton.loop_var_current);

    let body_plans =
        generic_loop_body::lower_generic_loop_v0_body(builder, facts, &skeleton.phi_bindings, ctx)?;
    skeleton.plan.body = body_plans;

    let post_body_map = builder.variable_ctx.variable_map.clone();
    generic_loop_handoff::apply_generic_loop_v0_condition_step_handoff(
        builder,
        facts,
        skeleton,
        pre_body_map,
        post_body_map,
    )?;

    Ok(())
}

pub(in crate::mir::builder) fn apply_generic_loop_v1_pipeline(
    builder: &mut MirBuilder,
    facts: &GenericLoopV1Facts,
    ctx: &LoopRouteContext,
    skeleton: &mut GenericLoopSkeleton,
) -> Result<(), String> {
    crate::mir::builder::control_flow::joinir::trace::trace()
        .varmap("generic_loop_v1_pre", &builder.variable_ctx.variable_map);
    let pre_body_map = builder.variable_ctx.variable_map.clone();

    let mut carrier_orchestration = generic_loop_body::orchestrate_generic_loop_v1_carriers(
        builder,
        facts,
        skeleton.loop_var_current,
        ctx,
    )?;
    skeleton.plan.body = carrier_orchestration.take_body_plans();

    generic_loop_handoff::apply_generic_loop_v1_condition_step_handoff(
        builder,
        facts,
        skeleton,
        pre_body_map,
        &carrier_orchestration,
    )?;

    carrier_orchestration.finalize(
        builder,
        &mut skeleton.plan,
        &facts.loop_var,
        skeleton.loop_var_init,
        skeleton.loop_var_current,
    );

    Ok(())
}
