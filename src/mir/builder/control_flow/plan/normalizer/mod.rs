//! Phase 273 P1: PlanNormalizer - loop plan payload → CorePlan 変換 (SSOT)
//!
//! SSOT: docs/development/current/main/design/plan-dir-shallowing-ssot.md
//! Flattened: cond_lowering/ moved to normalizer/ root
//!
//! # Responsibilities
//!
//! - Convert loop plan payload to CorePlan (SSOT for pattern-specific knowledge)
//! - Generate ValueIds for CorePlan expressions
//! - Expand pattern-specific operations into generic CoreEffectPlan
//!
//! # Key Design Decision
//!
//! Normalizer is the ONLY place that knows pattern-specific semantics.
//! Lowerer processes CorePlan without any pattern knowledge.

pub(in crate::mir::builder) mod helpers;
pub(in crate::mir::builder) mod pattern1_coreloop_builder;
pub(in crate::mir::builder) mod pattern_is_integer;
pub(in crate::mir::builder) mod pattern_starts_with;
pub(in crate::mir::builder) mod pattern_int_to_str;
pub(in crate::mir::builder) mod pattern_escape_map;
pub(in crate::mir::builder) mod pattern_split_lines;
pub(in crate::mir::builder) mod pattern_skip_ws;
#[cfg(test)]
mod pattern2_break;
mod value_join_args;
pub(in crate::mir::builder) mod common;

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

use super::{CoreEffectPlan, CoreLoopPlan, CorePlan, LoopCondContinueWithReturnPlan, LoweredRecipe};
use crate::mir::builder::control_flow::plan::loop_cond::continue_only_facts::LoopCondContinueOnlyFacts;
use crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_facts::LoopCondContinueWithReturnFacts;
use crate::mir::builder::control_flow::plan::loop_cond::return_in_body_facts::LoopCondReturnInBodyFacts;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) use pattern1_coreloop_builder::build_pattern1_coreloop;
pub(in crate::mir::builder) use super::generic_loop::normalizer::{
    normalize_generic_loop_v0, normalize_generic_loop_v1,
};

/// Phase 273 P1: PlanNormalizer - loop plan payload → CorePlan 変換 (SSOT)
pub(in crate::mir::builder) struct PlanNormalizer;

use crate::mir::builder::control_flow::plan::loop_cond::break_continue_types::LoopCondBreakContinueFacts;

impl PlanNormalizer {
    /// Normalize loop plan payload to CorePlan
    ///
    /// This is the SSOT for pattern-specific knowledge expansion.
    /// All pattern semantics (scan, split, etc.) are expanded here.
    pub(in crate::mir::builder) fn normalize(
        builder: &mut MirBuilder,
        domain: LoopCondContinueWithReturnPlan,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        // Phase 29bq P2.x: current planner payload is LoopCondContinueWithReturn.
        let facts = LoopCondContinueWithReturnFacts {
            condition: domain.condition,
            recipe: domain.recipe,
        };
        Self::normalize_loop_cond_continue_with_return(builder, facts, ctx)
    }

    // Delegators to pipeline lowerers (unified loop_cond_* normalizers)

    pub(in crate::mir::builder) fn normalize_loop_cond_break_continue(
        builder: &mut MirBuilder,
        facts: LoopCondBreakContinueFacts,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        crate::mir::builder::control_flow::plan::features::lower_loop_cond_break_continue(builder, facts, ctx)
    }

    pub(in crate::mir::builder) fn normalize_loop_cond_continue_only(
        builder: &mut MirBuilder,
        facts: LoopCondContinueOnlyFacts,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        crate::mir::builder::control_flow::plan::features::lower_loop_cond_continue_only(builder, facts, ctx)
    }

    pub(in crate::mir::builder) fn normalize_loop_cond_continue_with_return(
        builder: &mut MirBuilder,
        facts: LoopCondContinueWithReturnFacts,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        crate::mir::builder::control_flow::plan::features::loop_cond_continue_with_return_pipeline::lower_loop_cond_continue_with_return(builder, facts, ctx)
    }

    pub(in crate::mir::builder) fn normalize_loop_cond_return_in_body(
        builder: &mut MirBuilder,
        facts: LoopCondReturnInBodyFacts,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        crate::mir::builder::control_flow::plan::features::loop_cond_return_in_body_pipeline::lower_loop_cond_return_in_body(builder, facts, ctx)
    }
}

// Re-export cond_lowering types (maintains backward compatibility)
pub(in crate::mir::builder) use cond_lowering_entry::{lower_bool_expr_value_id, lower_cond_branch, lower_cond_value};
pub(in crate::mir::builder) use cond_lowering_loop_header::lower_loop_header_cond;
