use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::facts::pattern_skip_ws_facts::PatternSkipWsFacts;
use crate::mir::builder::control_flow::plan::normalizer::build_pattern1_coreloop;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn normalize_skip_ws_minimal(
    builder: &mut MirBuilder,
    facts: &PatternSkipWsFacts,
    ctx: &LoopPatternContext,
) -> Result<LoweredRecipe, String> {
    let loop_plan = build_pattern1_coreloop(
        builder,
        &facts.loop_var,
        &facts.loop_condition,
        &facts.loop_increment,
        ctx,
    )?;
    Ok(CorePlan::Loop(loop_plan))
}
