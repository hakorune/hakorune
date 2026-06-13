//! Split from composer.rs (behavior-preserving module split).

use super::RecipeComposer;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

impl RecipeComposer {
    /// Compose loop_cond_break_continue facts into LoweredRecipe without normalizer.
    ///
    /// Used only in strict/dev + planner_required routing.
    pub fn compose_loop_cond_break_continue_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        let loop_cond_facts = facts
            .facts
            .loop_cond_break_continue
            .clone()
            .ok_or_else(|| {
                Freeze::contract(
                    "loop_cond_break_continue facts missing in compose_loop_cond_break_continue_recipe",
                )
            })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=loop_cond_break_continue path=direct_pipeline");
        }

        crate::mir::builder::control_flow::plan::features::loop_cond_bc::lower_loop_cond_break_continue(
            builder,
            loop_cond_facts,
            ctx,
        )
        .map_err(|e| {
            Freeze::contract(&format!(
                "loop_cond_break_continue recipe lower failed: {}",
                e
            ))
        })
    }

    /// Compose loop_cond_continue_only facts into LoweredRecipe without normalizer.
    ///
    /// Used only in strict/dev + planner_required routing.
    pub fn compose_loop_cond_continue_only_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        let loop_cond_facts = facts.facts.loop_cond_continue_only.clone().ok_or_else(|| {
            Freeze::contract(
                "loop_cond_continue_only facts missing in compose_loop_cond_continue_only_recipe",
            )
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=loop_cond_continue_only path=direct_pipeline");
        }

        crate::mir::builder::control_flow::plan::features::loop_cond_co_pipeline::lower_loop_cond_continue_only(
            builder,
            loop_cond_facts,
            ctx,
        )
        .map_err(|e| {
            Freeze::contract(&format!(
                "loop_cond_continue_only recipe lower failed: {}",
                e
            ))
        })
    }

    /// Compose loop_cond_continue_with_return facts into LoweredRecipe without normalizer.
    ///
    /// Used only in strict/dev + planner_required routing.
    pub fn compose_loop_cond_continue_with_return_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        let loop_cond_facts = facts
            .facts
            .loop_cond_continue_with_return
            .clone()
            .ok_or_else(|| {
                Freeze::contract(
                    "loop_cond_continue_with_return facts missing in compose_loop_cond_continue_with_return_recipe",
                )
            })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(
                "[recipe:compose] route=loop_cond_continue_with_return path=direct_pipeline",
            );
        }

        crate::mir::builder::control_flow::plan::features::loop_cond_continue_with_return_pipeline::lower_loop_cond_continue_with_return(
            builder,
            loop_cond_facts,
            ctx,
        )
        .map_err(|e| {
            Freeze::contract(&format!(
                "loop_cond_continue_with_return recipe lower failed: {}",
                e
            ))
        })
    }

    /// Compose loop_cond_return_in_body facts into LoweredRecipe without normalizer.
    ///
    /// Used only in strict/dev + planner_required routing.
    pub fn compose_loop_cond_return_in_body_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        let loop_cond_facts = facts
            .facts
            .loop_cond_return_in_body
            .clone()
            .ok_or_else(|| {
                Freeze::contract(
                    "loop_cond_return_in_body facts missing in compose_loop_cond_return_in_body_recipe",
                )
            })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=loop_cond_return_in_body path=direct_pipeline");
        }

        crate::mir::builder::control_flow::plan::features::loop_cond_return_in_body_pipeline::lower_loop_cond_return_in_body(
            builder,
            loop_cond_facts,
            ctx,
        )
        .map_err(|e| {
            Freeze::contract(&format!(
                "loop_cond_return_in_body recipe lower failed: {}",
                e
            ))
        })
    }
}
