//! Split from composer.rs (behavior-preserving module split).

use super::RecipeComposer;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::features::generic_loop_pipeline;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::skeletons::generic_loop::alloc_generic_loop_v0_skeleton;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;

impl RecipeComposer {
    /// Compose generic_loop_v0 facts into LoweredRecipe without the normalizer.
    ///
    /// Used only in strict/dev + planner_required routing.
    pub fn compose_generic_loop_v0_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        let generic_loop_v0 = facts.facts.generic_loop_v0.as_ref().ok_or_else(|| {
            Freeze::contract("generic_loop_v0 facts missing in compose_generic_loop_v0_recipe")
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=generic_loop_v0 path=direct_pipeline");
        }

        let mut skeleton = alloc_generic_loop_v0_skeleton(builder, &generic_loop_v0.loop_var)
            .map_err(|e| Freeze::contract(&format!("generic_loop_v0 skeleton failed: {}", e)))?;

        generic_loop_pipeline::apply_generic_loop_v0_pipeline(
            builder,
            generic_loop_v0,
            ctx,
            &mut skeleton,
        )
        .map_err(|e| Freeze::contract(&format!("generic_loop_v0 pipeline failed: {}", e)))?;

        Ok(CorePlan::Loop(skeleton.plan))
    }

    /// Compose generic_loop_v1 facts into LoweredRecipe without the normalizer.
    ///
    /// Used only in strict/dev + planner_required routing.
    pub fn compose_generic_loop_v1_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        let generic_loop_v1 = facts.facts.generic_loop_v1.as_ref().ok_or_else(|| {
            Freeze::contract("generic_loop_v1 facts missing in compose_generic_loop_v1_recipe")
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=generic_loop_v1 path=direct_pipeline");
        }

        // Planner-required contracts expect a recipe to exist before lowering.
        if generic_loop_v1.body_exit_allowed.is_none() {
            return Err(Freeze::contract(
                "generic_loop_v1 recipe route requires body_exit_allowed",
            ));
        }

        let mut skeleton = alloc_generic_loop_v0_skeleton(builder, &generic_loop_v1.loop_var)
            .map_err(|e| Freeze::contract(&format!("generic_loop_v1 skeleton failed: {}", e)))?;

        generic_loop_pipeline::apply_generic_loop_v1_pipeline(
            builder,
            generic_loop_v1,
            ctx,
            &mut skeleton,
        )
        .map_err(|e| Freeze::contract(&format!("generic_loop_v1 pipeline failed: {}", e)))?;

        Ok(CorePlan::Loop(skeleton.plan))
    }
}
