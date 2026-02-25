//! Phase 33-17: Tail Call Classification
//!
//! Classifies tail calls in JoinIR loops into three semantic categories.
//!
//! Extracted from instruction_rewriter.rs (lines 14-69) for single responsibility.

/// Phase 33-16: Tail Call Classification
///
/// Classifies tail calls in JoinIR loops into three semantic categories:
///
/// 1. **LoopEntry**: First entry into the loop (main → loop_step)
///    - Occurs from the entry function's entry block
///    - Should jump directly to target (not redirect to header)
///    - Reason: The entry block IS the header block; redirecting creates self-loop
///
/// 2. **BackEdge**: Loop continuation (loop_step → loop_step)
///    - Occurs from loop body blocks (not entry function's entry block)
///    - MUST redirect to header block where PHI nodes are located
///    - Reason: PHI nodes need to merge values from all back edges
///
/// 3. **ExitJump**: Loop termination (→ k_exit)
///    - Occurs when jumping to continuation functions
///    - Handled separately via Return conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TailCallKind {
    /// First entry into loop - no redirection needed
    LoopEntry,
    /// Back edge in loop - redirect to header PHI
    BackEdge,
    /// Exit from loop - becomes Return conversion
    ExitJump,
}

/// Classifies a tail call based on context
///
/// # Arguments
/// * `is_source_entry_like` - True if this tail call originates from an entry-like block
/// * `has_loop_header_phis` - True if loop header PHI nodes exist
/// * `has_boundary` - True if JoinInlineBoundary exists (indicates loop context)
/// * `is_target_continuation` - True if the tail call target is a continuation function (k_exit)
/// * `is_target_loop_entry` - True if the tail call target is the loop entry function (loop_step)
///
/// # Returns
/// The classification of this tail call
pub fn classify_tail_call(
    is_source_entry_like: bool,
    has_loop_header_phis: bool,
    has_boundary: bool,
    is_target_continuation: bool,
    is_target_loop_entry: bool,
) -> TailCallKind {
    // Phase 256 P1.10: Continuation calls (k_exit) are always ExitJump
    // They should NOT be redirected to the header block.
    // k_exit body needs to execute before exiting.
    if is_target_continuation {
        return TailCallKind::ExitJump;
    }

    // Entry-like block jumping into the loop header is a LoopEntry (main → loop_step).
    // It should NOT be redirected to the header block, otherwise we create a self-loop.
    if is_source_entry_like && is_target_loop_entry {
        return TailCallKind::LoopEntry;
    }

    // Phase 287 P2: BackEdge is ONLY for target==loop_step (entry_func)
    // This prevents inner_step→inner_step from being classified as BackEdge,
    // which would incorrectly redirect it to the outer header (loop_step).
    // inner_step→inner_step should jump to inner_step's entry block, not outer header.
    if is_target_loop_entry && has_boundary && has_loop_header_phis {
        return TailCallKind::BackEdge;
    }

    // Otherwise, treat as exit (will be handled by Return conversion)
    TailCallKind::ExitJump
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_loop_entry() {
        let result = classify_tail_call(
            true,  // is_source_entry_like
            true,  // has_loop_header_phis
            true,  // has_boundary
            false, // is_target_continuation
            true,  // is_target_loop_entry
        );
        assert_eq!(result, TailCallKind::LoopEntry);
    }

    #[test]
    fn test_classify_back_edge() {
        let result = classify_tail_call(
            false, // is_source_entry_like
            true,  // has_loop_header_phis
            true,  // has_boundary
            false, // is_target_continuation
            true,  // is_target_loop_entry ← target is loop entry func
        );
        assert_eq!(result, TailCallKind::BackEdge);
    }

    #[test]
    fn test_classify_exit_jump() {
        let result = classify_tail_call(
            false, // is_source_entry_like
            false, // has_loop_header_phis (no header PHIs)
            true,  // has_boundary
            false, // is_target_continuation
            false, // is_target_loop_entry
        );
        assert_eq!(result, TailCallKind::ExitJump);
    }

    #[test]
    fn test_classify_no_boundary() {
        let result = classify_tail_call(
            false, // is_source_entry_like
            true,  // has_loop_header_phis
            false, // has_boundary (no boundary → exit)
            false, // is_target_continuation
            false, // is_target_loop_entry
        );
        assert_eq!(result, TailCallKind::ExitJump);
    }

    #[test]
    fn test_classify_continuation_target() {
        // Phase 256 P1.10: Continuation calls (k_exit) are always ExitJump
        // even when they would otherwise be classified as BackEdge
        let result = classify_tail_call(
            false, // is_source_entry_like
            true,  // has_loop_header_phis
            true,  // has_boundary
            true,  // is_target_continuation ← this makes it ExitJump
            true,  // is_target_loop_entry
        );
        assert_eq!(result, TailCallKind::ExitJump);
    }

    #[test]
    fn test_classify_inner_step_recursion() {
        // Phase 287 P2: inner_step→inner_step should NOT be BackEdge
        // even with boundary and header PHIs, because target is not loop entry func
        let result = classify_tail_call(
            false, // is_source_entry_like
            true,  // has_loop_header_phis
            true,  // has_boundary
            false, // is_target_continuation
            false, // is_target_loop_entry ← target is NOT loop entry (inner_step, not loop_step)
        );
        assert_eq!(result, TailCallKind::ExitJump);
    }

    #[test]
    fn test_classify_entry_like_but_not_loop_entry_target() {
        // Entry-like source does not imply LoopEntry unless the target is loop_step.
        let result = classify_tail_call(
            true,  // is_source_entry_like
            true,  // has_loop_header_phis
            true,  // has_boundary
            false, // is_target_continuation
            false, // is_target_loop_entry
        );
        assert_eq!(result, TailCallKind::ExitJump);
    }
}
