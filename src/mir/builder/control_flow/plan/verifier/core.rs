//! Phase 29bq+: Core verifier entry points
//!
//! # Responsibilities
//!
//! - Public API for CorePlan verification
//! - Top-level dispatcher to specialized validators
//!
//! # Architecture
//!
//! Orchestrates verification by dispatching to:
//! - `plan_validators::verify_seq()` for Seq
//! - `loop_validators::verify_loop()` for Loop
//! - `plan_validators::verify_if()` for If
//! - `plan_validators::verify_branch_n()` for BranchN
//! - `effect_validators::verify_effect()` for Effect
//! - `plan_validators::verify_exit()` for Exit

use super::super::{CorePlan, LoweredRecipe};

/// Phase 273 P1: PlanVerifier - CorePlan 不変条件検証 (fail-fast)
pub(in crate::mir::builder) struct PlanVerifier;

impl PlanVerifier {
    /// Verify CorePlan invariants
    ///
    /// Returns Ok(()) if all invariants hold, Err with details otherwise.
    pub(in crate::mir::builder) fn verify(plan: &LoweredRecipe) -> Result<(), String> {
        Self::verify_plan(plan, 0, 0)
    }

    /// Internal dispatcher - routes to specialized validators
    ///
    /// All validation logic modularized into 7 specialized modules (Steps 1-7 complete)
    pub(super) fn verify_plan(
        plan: &LoweredRecipe,
        depth: usize,
        loop_depth: usize,
    ) -> Result<(), String> {
        match plan {
            CorePlan::Seq(plans) => super::plan_validators::verify_seq(plans, depth, loop_depth),
            CorePlan::Loop(loop_plan) => {
                super::loop_validators::verify_loop(loop_plan, depth, loop_depth)
            }
            CorePlan::If(if_plan) => super::plan_validators::verify_if(if_plan, depth, loop_depth),
            CorePlan::BranchN(branch_plan) => {
                super::plan_validators::verify_branch_n(branch_plan, depth, loop_depth)
            }
            CorePlan::Effect(effect) => {
                super::effect_validators::verify_effect(effect, depth, loop_depth)
            }
            CorePlan::Exit(exit) => super::plan_validators::verify_exit(exit, depth, loop_depth),
        }
    }
}
