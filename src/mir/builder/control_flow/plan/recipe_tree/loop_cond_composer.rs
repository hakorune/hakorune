//! Split from composer.rs (behavior-preserving module split).

use super::RecipeComposer;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

impl RecipeComposer {
    /// Compose loop_scan_methods_v0 facts into LoweredRecipe.
    ///
    /// Phase C15: Recipe-first compose path for loop_scan_methods_v0.
    pub fn compose_loop_scan_methods_v0(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        let scan_facts = facts.facts.loop_scan_methods_v0.clone().ok_or_else(|| {
            Freeze::contract("loop_scan_methods_v0 facts missing in compose_loop_scan_methods_v0")
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=scan_methods_v0 path=recipe_first");
        }

        crate::mir::builder::control_flow::plan::loop_scan_methods_v0::pipeline::lower_loop_scan_methods_v0(
            builder, scan_facts, ctx,
        )
        .map_err(|e| Freeze::contract(&format!("loop_scan_methods_v0 normalize failed: {}", e)))
    }

    /// Compose loop_scan_methods_block_v0 facts into LoweredRecipe.
    ///
    /// Phase C15: Recipe-first compose path for loop_scan_methods_block_v0.
    pub fn compose_loop_scan_methods_block_v0(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        let scan_facts = facts
            .facts
            .loop_scan_methods_block_v0
            .clone()
            .ok_or_else(|| {
                Freeze::contract(
                "loop_scan_methods_block_v0 facts missing in compose_loop_scan_methods_block_v0",
            )
            })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=scan_methods_block_v0 path=recipe_first");
        }

        crate::mir::builder::control_flow::plan::loop_scan_methods_block_v0::lower_loop_scan_methods_block_v0(
            builder, scan_facts, ctx,
        )
        .map_err(|e| {
            Freeze::contract(&format!(
                "loop_scan_methods_block_v0 normalize failed: {}",
                e
            ))
        })
    }

    /// Compose loop_scan_phi_vars_v0 facts into LoweredRecipe.
    ///
    /// Phase C15: Recipe-first compose path for loop_scan_phi_vars_v0.
    pub fn compose_loop_scan_phi_vars_v0(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        let scan_facts = facts.facts.loop_scan_phi_vars_v0.clone().ok_or_else(|| {
            Freeze::contract("loop_scan_phi_vars_v0 facts missing in compose_loop_scan_phi_vars_v0")
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=scan_phi_vars_v0 path=recipe_first");
        }

        crate::mir::builder::control_flow::plan::loop_scan_phi_vars_v0::lower_loop_scan_phi_vars_v0(
            builder, scan_facts, ctx,
        )
        .map_err(|e| Freeze::contract(&format!("loop_scan_phi_vars_v0 normalize failed: {}", e)))
    }

    /// Compose loop_scan_v0 facts into LoweredRecipe.
    ///
    /// Phase C15: Recipe-first compose path for loop_scan_v0.
    pub fn compose_loop_scan_v0(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        let scan_facts = facts.facts.loop_scan_v0.clone().ok_or_else(|| {
            Freeze::contract("loop_scan_v0 facts missing in compose_loop_scan_v0")
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=scan_v0 path=recipe_first");
        }

        crate::mir::builder::control_flow::plan::loop_scan_v0::pipeline::lower_loop_scan_v0(
            builder, scan_facts, ctx,
        )
        .map_err(|e| Freeze::contract(&format!("loop_scan_v0 normalize failed: {}", e)))
    }

    /// Compose loop_collect_using_entries_v0 facts into LoweredRecipe.
    ///
    /// Phase C16: Recipe-first compose path for loop_collect_using_entries_v0.
    pub fn compose_loop_collect_using_entries_v0(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        let collect_facts = facts
            .facts
            .loop_collect_using_entries_v0
            .clone()
            .ok_or_else(|| {
                Freeze::contract(
                    "loop_collect_using_entries_v0 facts missing in compose_loop_collect_using_entries_v0",
                )
            })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=collect_using_entries_v0 path=recipe_first");
        }

        crate::mir::builder::control_flow::plan::loop_collect_using_entries_v0::lower_loop_collect_using_entries_v0(
            builder, collect_facts, ctx,
        )
        .map_err(|e| {
            Freeze::contract(&format!(
                "loop_collect_using_entries_v0 normalize failed: {}",
                e
            ))
        })
    }

    /// Compose loop_bundle_resolver_v0 facts into LoweredRecipe.
    ///
    /// Phase C16: Recipe-first compose path for loop_bundle_resolver_v0.
    pub fn compose_loop_bundle_resolver_v0(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        let bundle_facts = facts.facts.loop_bundle_resolver_v0.clone().ok_or_else(|| {
            Freeze::contract(
                "loop_bundle_resolver_v0 facts missing in compose_loop_bundle_resolver_v0",
            )
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=bundle_resolver_v0 path=recipe_first");
        }

        crate::mir::builder::control_flow::plan::loop_bundle_resolver_v0::lower_loop_bundle_resolver_v0(
            builder, bundle_facts, ctx,
        )
        .map_err(|e| {
            Freeze::contract(&format!(
                "loop_bundle_resolver_v0 normalize failed: {}",
                e
            ))
        })
    }

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

        crate::mir::builder::control_flow::plan::features::lower_loop_cond_break_continue(
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

        crate::mir::builder::control_flow::plan::features::lower_loop_cond_continue_only(
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
