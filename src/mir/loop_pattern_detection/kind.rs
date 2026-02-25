/// Loop pattern classification based on structure.
///
/// This enum represents the 6 main loop patterns we support.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopPatternKind {
    /// Pattern 1: Simple While Loop
    /// - No break, no continue
    /// - Single backedge
    Pattern1SimpleWhile,

    /// Pattern 2: Loop with Conditional Break
    /// - Has break statement(s)
    /// - No continue statements
    Pattern2Break,

    /// Pattern 3: Loop with If-Else PHI
    /// - Has if-else statement with PHI
    /// - No break, no continue
    /// - Multiple carrier variables
    Pattern3IfPhi,

    /// Pattern 4: Loop with Continue
    /// - Has continue statement(s)
    /// - No break statements (for simplicity)
    Pattern4Continue,

    /// Pattern 5: Infinite Loop with Early Exit (Phase 131-11)
    /// - Infinite loop: condition is `loop(true)`
    /// - Has both break AND continue
    /// - Minimal carrier (1 counter-like variable)
    InfiniteEarlyExit,

    /// Pattern 6: Nested Loop (1-level, minimal) - Phase 188.1
    /// - Outer loop: Pattern 1 (simple while)
    /// - Inner loop: Pattern 1 (simple while)
    /// - max_loop_depth == 2 exactly
    /// - No break/continue in either loop
    Pattern6NestedLoopMinimal,

    /// Pattern not recognized
    Unknown,
}

impl LoopPatternKind {
    /// Phase 193-3: Get human-readable pattern name
    ///
    /// Returns the friendly name for this pattern (e.g., "Pattern 1: Simple While")
    pub fn name(&self) -> &'static str {
        match self {
            LoopPatternKind::Pattern1SimpleWhile => "Pattern 1: Simple While Loop",
            LoopPatternKind::Pattern2Break => "Pattern 2: Loop with Conditional Break",
            LoopPatternKind::Pattern3IfPhi => "Pattern 3: Loop with If-Else PHI",
            LoopPatternKind::Pattern4Continue => "Pattern 4: Loop with Continue",
            LoopPatternKind::InfiniteEarlyExit => "Pattern 5: Infinite Loop with Early Exit",
            LoopPatternKind::Pattern6NestedLoopMinimal => "Pattern 6: Nested Loop (1-level minimal)",
            LoopPatternKind::Unknown => "Unknown Pattern",
        }
    }

    /// Phase 193-3: Get numeric pattern ID
    ///
    /// Returns the pattern number (1-5) or 0 for unknown.
    /// Useful for priority sorting.
    pub fn pattern_id(&self) -> u8 {
        match self {
            LoopPatternKind::Pattern1SimpleWhile => 1,
            LoopPatternKind::Pattern2Break => 2,
            LoopPatternKind::Pattern3IfPhi => 3,
            LoopPatternKind::Pattern4Continue => 4,
            LoopPatternKind::InfiniteEarlyExit => 5,
            LoopPatternKind::Pattern6NestedLoopMinimal => 6,
            LoopPatternKind::Unknown => 0,
        }
    }

    /// Phase 193-3: Check if this is a recognized pattern
    ///
    /// Returns false only for Unknown.
    pub fn is_recognized(&self) -> bool {
        !matches!(self, LoopPatternKind::Unknown)
    }

    /// Phase 193-3: Check if pattern has special control flow
    ///
    /// Returns true if pattern involves break or continue.
    pub fn has_special_control_flow(&self) -> bool {
        matches!(
            self,
            LoopPatternKind::Pattern2Break
                | LoopPatternKind::Pattern4Continue
                | LoopPatternKind::InfiniteEarlyExit
        )
    }

    /// Phase 193-3: Check if pattern involves PHI merging
    ///
    /// Returns true if pattern has if-else PHI merge.
    pub fn has_phi_merge(&self) -> bool {
        matches!(self, LoopPatternKind::Pattern3IfPhi)
    }
}
