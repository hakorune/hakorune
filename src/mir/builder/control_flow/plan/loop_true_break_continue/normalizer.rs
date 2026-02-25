use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::features::loop_true_break_continue_pipeline::lower_loop_true_break_continue;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::loop_cond::true_break_continue::LoopTrueBreakContinueFacts;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

impl PlanNormalizer {
    pub(in crate::mir::builder) fn normalize_loop_true_break_continue(
        builder: &mut MirBuilder,
        facts: LoopTrueBreakContinueFacts,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        lower_loop_true_break_continue(builder, facts, ctx)
    }
}
