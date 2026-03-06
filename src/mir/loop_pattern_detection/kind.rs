/// Loop route-family classification based on structure.
///
/// Historical pattern numbering remains available via `pattern_id()`, but the
/// runtime-facing enum names should use semantic route labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopPatternKind {
    /// LoopSimpleWhile route family (historical Pattern 1)
    /// - No break, no continue
    /// - Single backedge
    LoopSimpleWhile,

    /// LoopBreak route family (historical Pattern 2)
    /// - Has break statement(s)
    /// - No continue statements
    LoopBreak,

    /// Pattern 3: Loop with If-Else PHI
    /// - Has if-else statement with PHI
    /// - No break, no continue
    /// - Multiple carrier variables
    IfPhiJoin,

    /// Pattern 4: Loop with Continue
    /// - Has continue statement(s)
    /// - No break statements (for simplicity)
    LoopContinueOnly,

    /// Pattern 5: Infinite Loop with Early Exit (Phase 131-11)
    /// - Infinite loop: condition is `loop(true)`
    /// - Has both break AND continue
    /// - Minimal carrier (1 counter-like variable)
    InfiniteEarlyExit,

    /// NestedLoopMinimal route family (historical Pattern 6) - Phase 188.1
    /// - Outer loop: loop_simple_while-compatible
    /// - Inner loop: loop_simple_while-compatible
    /// - max_loop_depth == 2 exactly
    /// - No break/continue in either loop
    NestedLoopMinimal,

    /// Pattern not recognized
    Unknown,
}

impl LoopPatternKind {
    /// Phase 193-3: Get human-readable route name.
    pub fn name(&self) -> &'static str {
        match self {
            LoopPatternKind::LoopSimpleWhile => "Pattern 1: Simple While Loop",
            LoopPatternKind::LoopBreak => "LoopBreak",
            LoopPatternKind::IfPhiJoin => "Pattern 3: Loop with If-Else PHI",
            LoopPatternKind::LoopContinueOnly => "Pattern 4: Loop with Continue",
            LoopPatternKind::InfiniteEarlyExit => "Pattern 5: Infinite Loop with Early Exit",
            LoopPatternKind::NestedLoopMinimal => "Pattern 6: Nested Loop (1-level minimal)",
            LoopPatternKind::Unknown => "Unknown Loop Shape",
        }
    }

    /// Semantic label without pattern numbering.
    ///
    /// Preferred for runtime diagnostics in planner/router paths.
    pub fn semantic_label(&self) -> &'static str {
        match self {
            LoopPatternKind::LoopSimpleWhile => "LoopSimpleWhile",
            LoopPatternKind::LoopBreak => "LoopBreakRecipe",
            LoopPatternKind::IfPhiJoin => "IfPhiJoin",
            LoopPatternKind::LoopContinueOnly => "LoopContinueOnly",
            LoopPatternKind::InfiniteEarlyExit => "LoopTrueEarlyExit",
            LoopPatternKind::NestedLoopMinimal => "NestedLoopMinimal",
            LoopPatternKind::Unknown => "UnknownLoopShape",
        }
    }

    /// Phase 193-3: Get numeric pattern ID
    ///
    /// Returns the pattern number (1-5) or 0 for unknown.
    /// Useful for priority sorting.
    pub fn pattern_id(&self) -> u8 {
        match self {
            LoopPatternKind::LoopSimpleWhile => 1,
            LoopPatternKind::LoopBreak => 2,
            LoopPatternKind::IfPhiJoin => 3,
            LoopPatternKind::LoopContinueOnly => 4,
            LoopPatternKind::InfiniteEarlyExit => 5,
            LoopPatternKind::NestedLoopMinimal => 6,
            LoopPatternKind::Unknown => 0,
        }
    }

    /// Phase 193-3: Check if this is a recognized pattern
    ///
    /// Returns false only for Unknown.
    pub fn is_recognized(&self) -> bool {
        !matches!(self, LoopPatternKind::Unknown)
    }

    /// Phase 193-3: Check if route family has special control flow
    ///
    /// Returns true if pattern involves break or continue.
    pub fn has_special_control_flow(&self) -> bool {
        matches!(
            self,
            LoopPatternKind::LoopBreak
                | LoopPatternKind::LoopContinueOnly
                | LoopPatternKind::InfiniteEarlyExit
        )
    }

    /// Phase 193-3: Check if route family involves PHI merging
    ///
    /// Returns true if pattern has if-else PHI merge.
    pub fn has_phi_merge(&self) -> bool {
        matches!(self, LoopPatternKind::IfPhiJoin)
    }
}
