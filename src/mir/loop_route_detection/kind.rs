/// Loop route-family classification based on structure.
///
/// Historical numbering remains available via `pattern_id()`, but runtime-facing
/// code should use the semantic route labels below.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopRouteKind {
    /// LoopSimpleWhile route family
    /// - No break, no continue
    /// - Single backedge
    LoopSimpleWhile,

    /// LoopBreak route family
    /// - Has break statement(s)
    /// - No continue statements
    LoopBreak,

    /// IfPhiJoin route family
    /// - Has if-else statement with PHI
    /// - No break, no continue
    /// - Multiple carrier variables
    IfPhiJoin,

    /// LoopContinueOnly route family
    /// - Has continue statement(s)
    /// - No break statements (for simplicity)
    LoopContinueOnly,

    /// LoopTrueEarlyExit route family
    /// - Infinite loop: condition is `loop(true)`
    /// - Has both break AND continue
    /// - Minimal carrier (1 counter-like variable)
    LoopTrueEarlyExit,

    /// NestedLoopMinimal route family
    /// - Outer loop: loop_simple_while-compatible
    /// - Inner loop: loop_simple_while-compatible
    /// - max_loop_depth == 2 exactly
    /// - No break/continue in either loop
    NestedLoopMinimal,

    /// Route family not recognized
    Unknown,
}

impl LoopRouteKind {
    /// Phase 193-3: Get human-readable route name.
    pub fn name(&self) -> &'static str {
        match self {
            LoopRouteKind::LoopSimpleWhile => "LoopSimpleWhile",
            LoopRouteKind::LoopBreak => "LoopBreak",
            LoopRouteKind::IfPhiJoin => "IfPhiJoin",
            LoopRouteKind::LoopContinueOnly => "LoopContinueOnly",
            LoopRouteKind::LoopTrueEarlyExit => "LoopTrueEarlyExit",
            LoopRouteKind::NestedLoopMinimal => "NestedLoopMinimal",
            LoopRouteKind::Unknown => "UnknownLoopShape",
        }
    }


    /// Phase 193-3: Get historical numeric route ID
    ///
    /// Returns the historical route number (1-6) or 0 for unknown.
    /// Useful for priority sorting.
    pub fn pattern_id(&self) -> u8 {
        match self {
            LoopRouteKind::LoopSimpleWhile => 1,
            LoopRouteKind::LoopBreak => 2,
            LoopRouteKind::IfPhiJoin => 3,
            LoopRouteKind::LoopContinueOnly => 4,
            LoopRouteKind::LoopTrueEarlyExit => 5,
            LoopRouteKind::NestedLoopMinimal => 6,
            LoopRouteKind::Unknown => 0,
        }
    }



}
