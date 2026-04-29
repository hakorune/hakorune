//! Phase 273 P1: PlanNormalizer - facts/recipe contract → CorePlan 変換 (SSOT)
//!
//! SSOT: docs/development/current/main/design/plan-dir-shallowing-ssot.md
//! Flattened: cond_lowering/ moved to normalizer/ root
//!
//! # Responsibilities
//!
//! - Convert accepted facts/recipe contracts to CorePlan in legacy/analysis-only lanes
//! - Generate ValueIds for CorePlan expressions
//! - Expand route-specific operations into generic CoreEffectPlan
//!
//! # Key Design Decision
//!
//! Legacy route labels stay boxed inside this module.
//! Composer/entry runtime paths should prefer semantic helpers or feature lowerers,
//! and Lowerer processes CorePlan without route-specific knowledge.

pub(in crate::mir::builder) mod common;
pub(in crate::mir::builder) mod helpers;
pub(in crate::mir::builder) mod helpers_layout;
pub(in crate::mir::builder) mod helpers_pure_value;
pub(in crate::mir::builder) mod helpers_value;
#[cfg(test)]
mod loop_break;
pub(in crate::mir::builder) mod simple_while_coreloop_builder;

// Cond lowering modules (flattened from cond_lowering/)
pub(in crate::mir::builder) mod cond_lowering_entry;
pub(in crate::mir::builder) mod cond_lowering_freshen;
pub(in crate::mir::builder) mod cond_lowering_if_plan;
pub(in crate::mir::builder) mod cond_lowering_loop_header;
pub(in crate::mir::builder) mod cond_lowering_prelude;
pub(in crate::mir::builder) mod cond_lowering_value_expr;

pub(in crate::mir::builder) mod loop_body_lowering;
#[cfg(test)]
mod value_join_demo_if2;

use super::{CoreEffectPlan, LoweredRecipe};
use crate::mir::builder::control_flow::facts::loop_cond_break_continue::LoopCondBreakContinueFacts;
use crate::mir::builder::control_flow::facts::loop_cond_continue_only::LoopCondContinueOnlyFacts;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::MirBuilder;

/// Phase 273 P1: PlanNormalizer - facts/recipe contract → CorePlan 変換 (SSOT)
pub(in crate::mir::builder) struct PlanNormalizer;

impl PlanNormalizer {
    // Delegators to pipeline lowerers (unified loop_cond_* normalizers)

    pub(in crate::mir::builder) fn normalize_loop_cond_break_continue(
        builder: &mut MirBuilder,
        facts: LoopCondBreakContinueFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, String> {
        crate::mir::builder::control_flow::plan::features::loop_cond_bc::lower_loop_cond_break_continue(
            builder, facts, ctx,
        )
    }

    pub(in crate::mir::builder) fn normalize_loop_cond_continue_only(
        builder: &mut MirBuilder,
        facts: LoopCondContinueOnlyFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, String> {
        crate::mir::builder::control_flow::plan::features::loop_cond_co_pipeline::lower_loop_cond_continue_only(
            builder, facts, ctx,
        )
    }
}
