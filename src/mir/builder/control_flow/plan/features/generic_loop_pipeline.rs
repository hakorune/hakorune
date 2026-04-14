//! GenericLoop pipeline (ordered feature application).

use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::features::{generic_loop_body, generic_loop_step};
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::{
    GenericLoopV0Facts, GenericLoopV1Facts,
};
use crate::mir::builder::control_flow::plan::skeletons::generic_loop::GenericLoopSkeleton;
use crate::mir::builder::MirBuilder;

const GENERIC_LOOP_ERR: &str = "[normalizer] generic loop v0";

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
    builder.variable_ctx.variable_map = pre_body_map;
    generic_loop_step::apply_generic_loop_condition(
        builder,
        skeleton,
        &facts.condition,
        &facts.loop_var,
        GENERIC_LOOP_ERR,
    )?;
    builder.variable_ctx.variable_map = post_body_map.clone();
    generic_loop_step::apply_generic_loop_step(
        builder,
        skeleton,
        &facts.loop_increment,
        &facts.loop_var,
        GENERIC_LOOP_ERR,
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

    builder.variable_ctx.variable_map = pre_body_map;
    generic_loop_step::apply_generic_loop_condition(
        builder,
        skeleton,
        &facts.condition,
        &facts.loop_var,
        GENERIC_LOOP_ERR,
    )?;
    // Restore post-body bindings before the optional step lowering.
    builder.variable_ctx.variable_map = carrier_orchestration.post_body_map().clone();
    // Step evaluation must read a value that is already materialized on the
    // body->step path. Using the provisional step-in PHI directly can become
    // non-dominating depending on block layout, which may collapse to const
    // fallback (e.g. i=i+1 turning into 0+1).
    let loop_var_step_src =
        carrier_orchestration.loop_var_step_src(&facts.loop_var, skeleton.loop_var_current);
    builder
        .variable_ctx
        .variable_map
        .insert(facts.loop_var.clone(), loop_var_step_src);
    if carrier_orchestration.body_has_continue_edge() {
        generic_loop_step::apply_generic_loop_step(
            builder,
            skeleton,
            &facts.loop_increment,
            &facts.loop_var,
            GENERIC_LOOP_ERR,
        )?;
        crate::mir::builder::control_flow::joinir::trace::trace().varmap(
            "generic_loop_v1_post_step",
            &builder.variable_ctx.variable_map,
        );
    }

    carrier_orchestration.finalize(
        builder,
        &mut skeleton.plan,
        &facts.loop_var,
        skeleton.loop_var_init,
        skeleton.loop_var_current,
    );

    Ok(())
}
