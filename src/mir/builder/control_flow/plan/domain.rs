/// Phase 286 P3.1: Step placement vocabulary for loop-break recipe loops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopBreakStepPlacement {
    /// Loop increment executes at the end of the iteration (default).
    Last,
    /// Loop increment executes before the break check in the body.
    BeforeBreak,
}

/// Phase 286 P3.2: Exit kind for loop(true) early-exit loops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopTrueEarlyExitKind {
    /// Early return from function
    Return,
    /// Break from loop
    Break,
}
