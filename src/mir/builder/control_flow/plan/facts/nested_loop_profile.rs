//! Nested-loop body profile (analysis-only, no AST rewrite).
//!
//! ## Cluster count SSOT (Phase 29bq clusterN consolidation)
//!
//! cluster3/4/5 等の「数だけで増える」箱の追加点を1箇所に集約する。
//! planner outcome は facts/recipe 契約のみを扱う（挙動不変）。

// ============================================================================
// Cluster count configuration SSOT
// ============================================================================

/// Cluster count configuration for loop_cond_break_continue variants.
#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct ClusterCountProfile {
    /// Exact number of nested loops required for this cluster
    pub required_count: u8,
}

/// SSOT: Supported cluster configurations (優先順: 大きい数から)
///
/// cluster6+ 追加時は、この配列に1行追加するだけ。
pub(in crate::mir::builder) const CLUSTER_PROFILES: &[ClusterCountProfile] = &[
    ClusterCountProfile { required_count: 5 },
    ClusterCountProfile { required_count: 4 },
    ClusterCountProfile { required_count: 3 },
];

// ============================================================================
// Nested-loop body profile (existing functionality)
// ============================================================================

