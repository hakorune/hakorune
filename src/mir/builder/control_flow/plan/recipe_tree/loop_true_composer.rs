//! Split from composer.rs (behavior-preserving module split).

use super::RecipeComposer;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

impl RecipeComposer {

    /// Compose loop_true_break_continue facts into LoweredRecipe without normalizer.
    ///
    /// Used only in strict/dev + planner_required routing.
    pub fn compose_loop_true_break_continue_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        let loop_true_facts = facts
            .facts
            .loop_true_break_continue
            .clone()
            .ok_or_else(|| {
                Freeze::contract(
                    "loop_true_break_continue facts missing in compose_loop_true_break_continue_recipe",
                )
            })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[recipe:compose] loop_true_break_continue: composing via direct pipeline path"
            ));
        }

        crate::mir::builder::control_flow::plan::features::loop_true_break_continue_pipeline::lower_loop_true_break_continue(
            builder,
            loop_true_facts,
            ctx,
        )
        .map_err(|e| {
            Freeze::contract(&format!(
                "loop_true_break_continue recipe lower failed: {}",
                e
            ))
        })
    }

}
