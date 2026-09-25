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

pub(in crate::mir::builder) mod add_result_representation;
pub(in crate::mir::builder) mod common;
pub(in crate::mir::builder) mod helpers;
pub(in crate::mir::builder) mod helpers_layout;
pub(in crate::mir::builder) mod helpers_pure_value;
pub(in crate::mir::builder) mod helpers_value;
mod helpers_value_state;
pub(in crate::mir::builder) mod newbox;

// Cond lowering modules (flattened from cond_lowering/)
pub(in crate::mir::builder) mod cond_lowering_entry;
pub(in crate::mir::builder) mod cond_lowering_freshen;
pub(in crate::mir::builder) mod cond_lowering_if_plan;
pub(in crate::mir::builder) mod cond_lowering_if_plan_port;
#[cfg(test)]
mod cond_lowering_if_plan_port_tests;
pub(in crate::mir::builder) mod cond_lowering_loop_header;
pub(in crate::mir::builder) mod cond_lowering_loop_header_port;
#[cfg(test)]
mod cond_lowering_loop_header_port_tests;
pub(in crate::mir::builder) mod cond_lowering_prelude;
pub(in crate::mir::builder) mod cond_lowering_value_expr;

pub(in crate::mir::builder) mod loop_body_lowering;
pub(in crate::mir::builder) mod loop_body_lowering_associated_input;
#[cfg(test)]
mod loop_body_lowering_associated_input_tests;
mod stmt_only_prelude_view;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod value_join_demo_if2;

use super::CoreEffectPlan;

/// Phase 273 P1: PlanNormalizer - facts/recipe contract → CorePlan 変換 (SSOT)
pub(in crate::mir::builder) struct PlanNormalizer;
