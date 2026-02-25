use super::pattern1_coreloop_builder::build_pattern1_coreloop;
use super::{CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::plan::Pattern1SimpleWhilePlan;
use crate::mir::builder::control_flow::plan::step_mode::inline_in_body_explicit_step;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::MirBuilder;

impl super::PlanNormalizer {
    /// Phase 286 P2.1: Pattern1SimpleWhile → CorePlan 変換
    ///
    /// Expands Pattern1 (Simple While Loop) semantics into generic CorePlan:
    /// - CFG structure: preheader → header → body → step → header (back-edge)
    /// - 1 PHI for loop variable in header
    /// - No 2-step branching (simpler than Pattern4)
    pub(in crate::mir::builder) fn normalize_pattern1_simple_while(
        builder: &mut MirBuilder,
        parts: Pattern1SimpleWhilePlan,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        use crate::mir::builder::control_flow::joinir::trace;

        let trace_logger = trace::trace();
        let debug = ctx.debug;

        if debug {
            trace_logger.debug(
                "normalizer/pattern1_simple_while",
                &format!(
                    "Phase 286 P2.1: Normalizing Pattern1SimpleWhile for {} (loop_var: {})",
                    ctx.func_name, parts.loop_var
                ),
            );
        }

        let mut loop_plan = build_pattern1_coreloop(
            builder,
            &parts.loop_var,
            &parts.condition,
            &parts.loop_increment,
            ctx,
        )?;

        if inline_explicit_step_enabled() {
            promote_inline_explicit_step_for_pattern1(&mut loop_plan)?;
        }

        if debug {
            trace_logger.debug(
                "normalizer/pattern1_simple_while",
                "CorePlan construction complete (4 blocks, 1 PHI)",
            );
        }

        Ok(CorePlan::Loop(loop_plan))
    }
}

fn inline_explicit_step_enabled() -> bool {
    let strict_or_dev =
        crate::config::env::joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled()
}

fn promote_inline_explicit_step_for_pattern1(
    loop_plan: &mut crate::mir::builder::control_flow::plan::CoreLoopPlan,
) -> Result<(), String> {
    let step_effects = {
        let Some((_, effects)) = loop_plan
            .block_effects
            .iter_mut()
            .find(|(bb, _)| *bb == loop_plan.step_bb)
        else {
            return Err("[normalizer] pattern1 step_bb missing from block_effects".to_string());
        };
        std::mem::take(effects)
    };

    if step_effects.is_empty() {
        return Err("[normalizer] pattern1 step_bb has no effects".to_string());
    }

    if let Some((_, effects)) = loop_plan
        .block_effects
        .iter()
        .find(|(bb, _)| *bb == loop_plan.step_bb)
    {
        if !effects.is_empty() {
            return Err("[normalizer] pattern1 step_bb must be empty after inline promotion".to_string());
        }
    }

    loop_plan.body.extend(effects_to_plans(step_effects));
    let (step_mode, has_explicit_step) = inline_in_body_explicit_step();
    loop_plan.step_mode = step_mode;
    loop_plan.has_explicit_step = has_explicit_step;
    Ok(())
}
