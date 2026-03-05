/// Phase 273 P0: Scan direction for forward/reverse scan
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(test)]
pub(in crate::mir::builder) enum ScanDirection {
    /// Forward scan: i < s.length(), i = i + 1
    Forward,
    /// Reverse scan: i >= 0, i = i - 1
    Reverse,
}

#[cfg(test)]
pub(in crate::mir::builder) fn scan_direction_from_step_lit(
    step_lit: i64,
) -> Option<ScanDirection> {
    match step_lit {
        1 => Some(ScanDirection::Forward),
        -1 => Some(ScanDirection::Reverse),
        _ => None,
    }
}

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
