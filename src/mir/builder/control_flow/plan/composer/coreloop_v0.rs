//! Phase 29ao P43: CoreLoopComposer v0 scaffold (unconnected).

use super::coreloop_single_entry::{
    try_compose_scan_with_init_unified, try_compose_split_scan_unified,
};
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::composer::coreloop_gates::{
    coreloop_base_gate, exit_kinds_empty,
};
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::normalizer::build_simple_while_coreloop;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn try_compose_core_loop_v0(
    builder: &mut MirBuilder,
    facts: &CanonicalLoopFacts,
    ctx: &LoopRouteContext,
) -> Result<Option<LoweredRecipe>, String> {
    if let Some(core) = try_compose_scan_with_init_unified(builder, facts, ctx)? {
        return Ok(Some(core));
    }
    if let Some(core) = try_compose_split_scan_unified(builder, facts, ctx)? {
        return Ok(Some(core));
    }
    if !coreloop_base_gate(facts) {
        return Ok(None);
    }
    if facts.value_join_needed {
        return Ok(None);
    }
    if !exit_kinds_empty(facts) {
        return Ok(None);
    }

    let Some(loop_simple_while) = facts.facts.loop_simple_while() else {
        return Ok(None);
    };

    let loop_plan = build_simple_while_coreloop(
        builder,
        &loop_simple_while.loop_var,
        &loop_simple_while.condition,
        &loop_simple_while.loop_increment,
        ctx,
    )?;
    Ok(Some(CorePlan::Loop(loop_plan)))
}
