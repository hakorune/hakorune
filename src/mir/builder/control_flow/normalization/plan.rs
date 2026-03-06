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

    /// Whether the detected shape includes an explicit return (for unreachable detection)
    pub requires_return: bool,
}

/// Kind of normalization shape detected
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanKind {
    /// Phase 131: loop(true) { ... break } alone
    ///
    /// Shape: Single loop statement
    /// Example: `loop(true) { x = 1; break }`
    LoopOnly,
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
}
