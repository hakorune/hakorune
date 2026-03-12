//! MIR policy SSOT (shared by router/canonicalizer).
//!
//! Goal: avoid duplicated policy logic across layers.

#[derive(Debug, Clone)]
pub enum PolicyDecision<T> {
    Use(T),
    Reject(String),
    None,
}

pub mod balanced_depth_scan;
pub mod cond_profile;
pub mod generic_loop_overlap_policy;
pub mod generic_loop_v1_shape;
pub mod loop_body_lowering_policy;
pub mod post_loop_early_return_plan;
pub mod return_prelude_policy;

pub use cond_profile::{BoundExpr, CmpOp, CondParam, CondProfile, CondSkeleton, StepExpr};
pub use generic_loop_v1_shape::GenericLoopV1ShapeId;
pub use loop_body_lowering_policy::BodyLoweringPolicy;
