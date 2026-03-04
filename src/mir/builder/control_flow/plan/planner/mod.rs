//! Phase 29ai P7: Single Planner (Facts → DomainPlan)
//!
//! P0 goal: expose a single planner entrypoint (`build_plan_from_facts_ctx`) and hide
//! pattern-name branching behind internal enums.

#![allow(unused_imports)]

pub(in crate::mir::builder) mod build;
pub(in crate::mir::builder) mod candidates;
pub(in crate::mir::builder) mod context;
pub(in crate::mir::builder) mod freeze;
pub(in crate::mir::builder) mod helpers;
pub(in crate::mir::builder) mod outcome;
mod pattern_shadow;
pub(in crate::mir::builder) mod tags;
pub(in crate::mir::builder) mod validators;

// Tests are in a separate module for maintainability
#[cfg(test)]
mod build_tests;

pub(in crate::mir::builder) use build::build_plan_from_facts_ctx;
pub(in crate::mir::builder) use context::PlannerContext;
pub(in crate::mir::builder) use freeze::Freeze;
pub(in crate::mir::builder) use outcome::{
    build_plan_with_facts, build_plan_with_facts_ctx, PlanBuildOutcome,
};
