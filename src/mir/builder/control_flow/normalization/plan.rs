//! Normalization plan data structures (Phase 134 P0)
//!
//! Defines what to normalize and how many statements to consume.

/// Plan for normalizing a block suffix
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizationPlan {
    /// Number of statements to consume from remaining block
    pub consumed: usize,

    /// What kind of normalization to perform
    pub kind: PlanKind,

    /// Whether the pattern includes an explicit return (for unreachable detection)
    pub requires_return: bool,
}

/// Kind of normalization pattern detected
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanKind {
    /// Phase 131: loop(true) { ... break } alone
    ///
    /// Pattern: Single loop statement
    /// Example: `loop(true) { x = 1; break }`
    LoopOnly,

    /// Phase 132-133: loop(true) + post assignments + return
    ///
    /// **DEPRECATED in Phase 142 P0**: This variant is no longer created by PlanBox.
    /// Normalization unit changed from "block suffix" to "statement (loop only)".
    /// Post-loop assignments are now handled by normal MIR lowering, not normalization.
    ///
    /// Migration: Use `LoopOnly` instead. Process subsequent statements separately.
    ///
    /// This variant is kept for backward compatibility but will be removed
    /// once all match arms and tests migrate to statement-level normalization.
    ///
    /// Pattern: loop + N assignments + return
    /// Example: `loop(true) { x = 1; break }; x = x + 2; return x`
    #[cfg(test)]
    #[deprecated(
        since = "Phase 142 P0",
        note = "Use LoopOnly instead. Normalization unit is now statement-level."
    )]
    LoopWithPost {
        /// Number of post-loop assignment statements
        post_assign_count: usize,
    },
}

impl NormalizationPlan {
    /// Create a Phase 131 plan (loop-only)
    pub fn loop_only() -> Self {
        Self {
            consumed: 1,
            kind: PlanKind::LoopOnly,
            requires_return: false,
        }
    }

    /// Create a Phase 132-133 plan (loop + post assignments + return)
    ///
    /// **DEPRECATED in Phase 142 P0**: Use `loop_only()` instead.
    /// Statement-level normalization makes this obsolete.
    #[deprecated(
        since = "Phase 142 P0",
        note = "Use loop_only() instead. Statement-level normalization makes this obsolete."
    )]
    #[cfg(test)]
    #[allow(deprecated)]
    pub fn loop_with_post(post_assign_count: usize) -> Self {
        // consumed = 1 (loop) + N (assigns) + 1 (return)
        let consumed = 1 + post_assign_count + 1;

        Self {
            consumed,
            kind: PlanKind::LoopWithPost { post_assign_count },
            requires_return: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_only_plan() {
        let plan = NormalizationPlan::loop_only();
        assert_eq!(plan.consumed, 1);
        assert_eq!(plan.kind, PlanKind::LoopOnly);
        assert!(!plan.requires_return);
    }

    // Phase 142 P0: Removed test_loop_with_post_* tests
    // The loop_with_post() function and LoopWithPost variant are deprecated.
    // Statement-level normalization (Phase 142 P0) obsoletes the "loop + post assigns + return" pattern.
    // See docs/development/current/main/phases/phase-142/ for details.
}
