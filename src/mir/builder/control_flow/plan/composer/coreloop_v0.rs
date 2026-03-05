//! Phase 29ao P43: CoreLoopComposer v0 scaffold (unconnected).

use super::coreloop_single_entry::{
    try_compose_scan_with_init_unified, try_compose_split_scan_unified,
};
use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::composer::coreloop_gates::{
    coreloop_base_gate, exit_kinds_empty,
};
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::normalizer::build_pattern1_coreloop;
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

    let Some(pattern1) = facts.facts.pattern1_simplewhile.as_ref() else {
        return Ok(None);
    };

    let loop_plan = build_pattern1_coreloop(
        builder,
        &pattern1.loop_var,
        &pattern1.condition,
        &pattern1.loop_increment,
        ctx,
    )?;
    Ok(Some(CorePlan::Loop(loop_plan)))
}
