//! Phase 29ai P7: Trivial helper functions for planner
//!
//! This module contains simple inference/utility functions that don't belong
//! in the main orchestration logic.

use crate::mir::builder::control_flow::plan::facts::feature_facts::ExitUsageFacts;
use crate::mir::builder::control_flow::plan::facts::skeleton_facts::SkeletonKind;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;

/// Infer skeleton kind from facts (trivial accessor)
pub(super) fn infer_skeleton_kind(facts: &CanonicalLoopFacts) -> Option<SkeletonKind> {
    Some(facts.skeleton_kind)
}

/// Infer exit usage from facts (trivial accessor)
pub(super) fn infer_exit_usage(facts: &CanonicalLoopFacts) -> Option<ExitUsageFacts> {
    Some(facts.exit_usage.clone())
}
