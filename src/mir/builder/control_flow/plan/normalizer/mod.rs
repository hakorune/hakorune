//! Phase 273 P1: PlanNormalizer - DomainPlan → CorePlan 変換 (SSOT)
//!
//! SSOT: docs/development/current/main/design/plan-dir-shallowing-ssot.md
//! Flattened: cond_lowering/ moved to normalizer/ root
//!
//! # Responsibilities
//!
//! - Convert DomainPlan to CorePlan (SSOT for pattern-specific knowledge)
//! - Generate ValueIds for CorePlan expressions
//! - Expand pattern-specific operations into generic CoreEffectPlan
//!
//! # Key Design Decision
//!
//! Normalizer is the ONLY place that knows pattern-specific semantics.
//! Lowerer processes CorePlan without any pattern knowledge.

pub(in crate::mir::builder) mod helpers;
mod pattern1_coreloop_builder;
mod pattern_is_integer;
mod pattern_starts_with;
mod pattern_int_to_str;
mod pattern_escape_map;
mod pattern_split_lines;
mod pattern_skip_ws;
mod pattern2_break;
mod pattern_scan_with_init;
mod pattern_split_scan;
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

use super::{CoreEffectPlan, CoreLoopPlan, CorePlan, DomainPlan, LoweredRecipe};
use crate::mir::builder::control_flow::plan::loop_cond::continue_only_facts::LoopCondContinueOnlyFacts;
use crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_facts::LoopCondContinueWithReturnFacts;
use crate::mir::builder::control_flow::plan::loop_cond::return_in_body_facts::LoopCondReturnInBodyFacts;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) use pattern1_coreloop_builder::build_pattern1_coreloop;
pub(in crate::mir::builder) use pattern_is_integer::normalize_is_integer_minimal;
pub(in crate::mir::builder) use pattern_starts_with::normalize_starts_with_minimal;
pub(in crate::mir::builder) use pattern_int_to_str::normalize_int_to_str_minimal;
pub(in crate::mir::builder) use pattern_escape_map::normalize_escape_map_minimal;
pub(in crate::mir::builder) use pattern_split_lines::normalize_split_lines_minimal;
pub(in crate::mir::builder) use pattern_skip_ws::normalize_skip_ws_minimal;
pub(in crate::mir::builder) use super::generic_loop::normalizer::{
    normalize_generic_loop_v0, normalize_generic_loop_v1,
};

/// Phase 273 P1: PlanNormalizer - DomainPlan → CorePlan 変換 (SSOT)
pub(in crate::mir::builder) struct PlanNormalizer;

use crate::mir::builder::control_flow::plan::loop_cond::break_continue_types::LoopCondBreakContinueFacts;

impl PlanNormalizer {
    /// Normalize DomainPlan to CorePlan
    ///
    /// This is the SSOT for pattern-specific knowledge expansion.
    /// All pattern semantics (scan, split, etc.) are expanded here.
    pub(in crate::mir::builder) fn normalize(
        builder: &mut MirBuilder,
        domain: DomainPlan,
        ctx: &LoopPatternContext,
    ) -> Result<LoweredRecipe, String> {
        use crate::mir::builder::control_flow::plan::DomainPlan;
        match domain {
            // Phase 29bq P2.x: LoopCondContinueWithReturn
            DomainPlan::LoopCondContinueWithReturn(plan) => {
                use crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_facts::LoopCondContinueWithReturnFacts;
                let facts = LoopCondContinueWithReturnFacts {
                    condition: plan.condition,
                    recipe: plan.recipe,
                };
                Self::normalize_loop_cond_continue_with_return(builder, facts, ctx)
            }
        }
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
