//! Phase 29ai P7: Single Planner (Facts/recipe gating)

pub(in crate::mir::builder) mod context;
pub(in crate::mir::builder) mod freeze;
pub(in crate::mir::builder) mod outcome;
pub(in crate::mir::builder) mod tags;

pub(in crate::mir::builder) use context::PlannerContext;
pub(in crate::mir::builder) use freeze::Freeze;
pub(in crate::mir::builder) use outcome::{build_plan_with_facts_ctx, PlanBuildOutcome};
