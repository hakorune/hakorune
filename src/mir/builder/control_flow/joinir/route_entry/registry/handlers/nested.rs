use crate::mir::builder::control_flow::joinir::route_entry::router::{
    lower_verified_core_plan, LoopRouteContext,
};
use crate::mir::builder::control_flow::plan::composer;
use crate::mir::builder::control_flow::lower::PlanBuildOutcome;
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::FlowboxVia;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use super::common::debug_log_recipe_entry;
use super::super::types::{route_labels, RouterEnv};

pub(crate) fn route_nested_loop_minimal(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    debug_log_recipe_entry(route_labels::NESTED_LOOP_MINIMAL, env);
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    if facts.facts.nested_loop_minimal().is_none() {
        return Ok(None);
    }

    let Some(core_plan) = composer::try_compose_core_loop_v2_nested_minimal(builder, facts, ctx)?
    else {
        if env.strict_or_dev {
            return Err(
                "nested_loop_minimal strict/dev route failed: compose rejected".to_string(),
            );
        }
        return Ok(None);
    };

    let via = if env.strict_or_dev {
        FlowboxVia::Shadow
    } else {
        FlowboxVia::Release
    };
    lower_verified_core_plan(
        builder,
        ctx,
        env.strict_or_dev,
        outcome.facts.as_ref(),
        core_plan,
        via,
    )
}
