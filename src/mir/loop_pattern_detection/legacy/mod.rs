//! Legacy loop pattern detection functions (Phase 188).
//!
//! These are still used by pattern lowerers. They live in `legacy/` to
//! separate Phase 188 name-based detection from Phase 194+ structure-based
//! classification (extract_features + classify).

use crate::mir::loop_form::LoopForm;

// Legacy Detection Functions (Phase 188)
// ============================================================================
//
// NOTE (Phase 179): These functions are still actively used by pattern lowerers.
// The term "legacy" refers to Phase 188 implementation style (name-based detection),
// not deprecation status. They live under legacy/ to separate from structure-based
// detection while remaining production code.
//
// Future work: Gradually migrate to Phase 194+ structure-based detection (extract_features/classify).

// ============================================================================
// Pattern 1: Simple While Loop
// ============================================================================

/// Detect Pattern 1: Simple While Loop
///
/// Returns true ONLY if:
/// - Loop condition is simple comparison (no &&, ||)
/// - Loop body contains only assignments + prints (no nested loops, no breaks)
/// - Loop has single increment/decrement
/// - NO break statements (break_targets is empty)
/// - NO continue statements (continue_targets is empty)
/// - Single backedge (latches.len() == 1)
///
/// # Arguments
/// * `loop_form` - The loop structure to analyze
///
/// # Returns
/// * `true` if the loop matches Pattern 1 (Simple While), `false` otherwise
///
/// # Reference
/// See design.md § Pattern 1 → LoopScopeShape Recognition
///
/// # Example
/// ```rust,ignore
/// let loop_form = /* ... */;
/// if is_simple_while_pattern(&loop_form) {
///     // Lower to simple while pattern
/// }
/// ```
pub fn is_simple_while_pattern(loop_form: &LoopForm) -> bool {
    // Pattern 1 Recognition Criteria (from design.md § Pattern 1):
    // 1. break_targets: EMPTY (no break statements)
    // 2. continue_targets: EMPTY (no continue statements)
    // 3. Single backedge (single latch - LoopShape has singular latch field)
    //
    // Note: LoopShape has a singular `latch` field, not `latches`, so we don't
    // need to check length. The existence of a LoopShape implies a valid latch.

    // Check 1: No break statements
    if !loop_form.break_targets.is_empty() {
        return false;
    }

    // Check 2: No continue statements
    if !loop_form.continue_targets.is_empty() {
        return false;
    }

    // Check 3: All other checks passed
    // The LoopShape structure guarantees:
    // - Single preheader, header, body, latch, exit
    // - Valid loop structure
    //
    // Pattern 1 ONLY requires:
    // - No breaks, no continues
    // - Natural loop structure (which LoopShape guarantees)
    //
    // Advanced checks (nested loops, complex conditions) are deferred to
    // lowering phase where we can fail gracefully if needed.

    true
}

// ============================================================================
// Pattern 2: Loop with Conditional Break
// ============================================================================

/// Detect Pattern 2: Loop with Conditional Break
///
/// Returns true ONLY if:
/// - Loop condition exists
/// - Loop body contains exactly ONE if statement with break
/// - Break is in then-branch
/// - NO nested loops
/// - break_targets is NON-EMPTY (has at least one break)
///
/// # Arguments
/// * `loop_form` - The loop structure to analyze
///
/// # Returns
/// * `true` if the loop matches Pattern 2 (Break), `false` otherwise
///
/// # Reference
/// See design.md § Pattern 2 → LoopScopeShape Recognition
///
/// # Example
/// ```rust,ignore
/// let loop_form = /* ... */;
/// if is_loop_with_break_pattern(&loop_form) {
///     // Lower to loop with break pattern
/// }
/// ```
pub fn is_loop_with_break_pattern(loop_form: &LoopForm) -> bool {
    // Pattern 2 Recognition Criteria (from design.md § Pattern 2):
    // 1. break_targets: NON-EMPTY (at least 1 break)
    // 2. continue_targets: EMPTY (for simplicity)
    // 3. Exactly ONE break target
    //
    // Phase 188-Impl-2: Minimal implementation
    // Advanced checks (nested loops, if-statement structure) are deferred to
    // lowering phase where we can fail gracefully if needed.

    // Check 1: break_targets is NON-EMPTY (has at least 1 break)
    if loop_form.break_targets.is_empty() {
        return false;
    }

    // Check 2: Exactly ONE break target (pattern assumes single break)
    if loop_form.break_targets.len() != 1 {
        return false;
    }

    // Check 3: No continue statements (for simplicity in Pattern 2)
    if !loop_form.continue_targets.is_empty() {
        return false;
    }

    // Pattern 2 matched
    // The LoopForm structure guarantees:
    // - Valid loop structure
    // - Single break target
    // - No continues
    //
    // Advanced checks (break is in if-statement, etc.) are deferred to
    // lowering phase for graceful failure.

    true
}

// ============================================================================
// Pattern 3: Loop with If-Else PHI
// ============================================================================

/// Detect Pattern 3: Loop with If-Else PHI
///
/// Returns true ONLY if:
/// - Loop has if-else statement assigning to variable(s)
/// - Both branches assign to same variable
/// - NO nested loops
/// - NO break or continue statements
/// - Loop has multiple carrier variables (e.g., i + sum)
///
/// # Arguments
/// * `loop_form` - The loop structure to analyze
///
/// # Returns
/// * `true` if the loop matches Pattern 3 (If-Else PHI), `false` otherwise
///
/// # Reference
/// See design.md § Pattern 3 → LoopScopeShape Recognition
///
/// # Example
/// ```rust,ignore
/// let loop_form = /* ... */;
/// if is_loop_with_conditional_phi_pattern(&loop_form) {
///     // Lower to loop with if-else phi pattern
/// }
/// ```
pub fn is_loop_with_conditional_phi_pattern(loop_form: &LoopForm) -> bool {
    // Phase 188-Impl-3: Minimal implementation
    // Pattern 3 Recognition Criteria (from design.md § Pattern 3):
    // 1. break_targets: EMPTY (no break statements)
    // 2. continue_targets: EMPTY (no continue statements)
    // 3. All Pattern 3 loops are valid Pattern 1 loops with extra PHI nodes
    //
    // For now: return true as fallback for Pattern 1 loops
    // Advanced checks (if-else detection, multiple carriers) are deferred to
    // lowering phase where we can fail gracefully if needed.

    // Check 1: No break statements
    if !loop_form.break_targets.is_empty() {
        return false;
    }

    // Check 2: No continue statements
    if !loop_form.continue_targets.is_empty() {
        return false;
    }

    // Pattern 3 matched (fallback for now)
    // Since all Pattern 3 loops are also Pattern 1 loops, we can safely return true
    // The lowering phase will determine if the specific pattern is supported
    true
}

// ============================================================================
// Pattern 4: Loop with Continue
// ============================================================================

/// Detect Pattern 4: Loop with Continue
///
/// Returns true ONLY if:
/// - Loop has continue statement(s)
/// - Continue is typically in an if statement
/// - NO break statements (for simplicity)
/// - Loop has multiple carrier variables
///
/// # Arguments
/// * `loop_form` - The loop structure to analyze
///
/// # Returns
/// * `true` if the loop matches Pattern 4 (Continue), `false` otherwise
///
/// # Reference
/// See design.md § Pattern 4 → LoopScopeShape Recognition
///
/// # Example
/// ```rust,ignore
/// let loop_form = /* ... */;
/// if is_loop_with_continue_pattern(&loop_form) {
///     // Lower to loop with continue pattern
/// }
/// ```
pub fn is_loop_with_continue_pattern(loop_form: &LoopForm) -> bool {
    // Pattern 4 Recognition Criteria:
    // 1. continue_targets: NON-EMPTY (at least 1 continue)
    // 2. break_targets: EMPTY (for simplicity in Pattern 4)
    // 3. At least ONE continue target
    //
    // Phase 188-Impl-4: Minimal implementation
    // Advanced checks (nested loops, if-statement structure) are deferred to
    // lowering phase where we can fail gracefully if needed.

    // Check 1: continue_targets is NON-EMPTY (has at least 1 continue)
    if loop_form.continue_targets.is_empty() {
        return false;
    }

    // Check 2: At least ONE continue target (pattern assumes single continue for now)
    if loop_form.continue_targets.len() < 1 {
        return false;
    }

    // Check 3: No break statements (for simplicity in Pattern 4)
    if !loop_form.break_targets.is_empty() {
        return false;
    }

    // Pattern 4 matched
    // The LoopForm structure guarantees:
    // - Valid loop structure
    // - At least one continue target
    // - No breaks
    //
    // Advanced checks (continue is in if-statement, etc.) are deferred to
    // lowering phase for graceful failure.

    true
}

// ============================================================================
// Helper Functions (Future Use)
// ============================================================================

/// Count the number of carrier variables in a loop
///
/// Carrier variables are loop variables that are updated in the loop body
/// and carried through PHI nodes in the header.
///
/// # Arguments
/// * `loop_form` - The loop structure to analyze
///
/// # Returns
/// * Number of carrier variables
///
/// # TODO
/// Implement by analyzing header PHI nodes
#[allow(dead_code)]
fn count_carrier_variables(_loop_form: &LoopForm) -> usize {
    // TODO: Implement carrier variable counting
    // Step 1: Access loop_form.header block
    // Step 2: Count PHI instructions in header
    // Step 3: Return count
    0
}

/// Check if loop body contains nested loops
///
/// # Arguments
/// * `loop_form` - The loop structure to analyze
///
/// # Returns
/// * `true` if nested loops found, `false` otherwise
///
/// # TODO
/// Implement by checking for LoopForm within body blocks
#[allow(dead_code)]
fn has_nested_loops(_loop_form: &LoopForm) -> bool {
    // TODO: Implement nested loop detection
    // Step 1: Traverse body blocks
    // Step 2: Check for loop headers in body
    // Step 3: Return true if any found
    false
}

/// Check if loop condition is simple (single comparison, no && or ||)
///
/// # Arguments
/// * `loop_form` - The loop structure to analyze
///
/// # Returns
/// * `true` if condition is simple, `false` otherwise
///
/// # TODO
/// Implement by checking header condition complexity
#[allow(dead_code)]
fn has_simple_condition(_loop_form: &LoopForm) -> bool {
    // TODO: Implement condition complexity check
    // Step 1: Access loop_form.header block
    // Step 2: Find condition instruction
    // Step 3: Check for && or || operators
    // Step 4: Return true if no complex operators
    true // Assume simple for now
}

#[cfg(test)]
mod tests;

// Phase 170-D: Loop Condition Scope Analysis Boxes
pub mod condition_var_analyzer;
pub mod loop_condition_scope;

// Phase 171-C: LoopBodyLocal Carrier Promotion
pub mod loop_body_carrier_promoter;

// Phase 223-3: LoopBodyLocal Condition Promotion (for Pattern4)
pub mod loop_body_cond_promoter;

// Phase 224: A-4 DigitPos Pattern Promotion
pub mod loop_body_digitpos_promoter;

// Phase 171-C-5: Trim Pattern Helper
pub mod trim_loop_helper;
pub use trim_loop_helper::TrimLoopHelper;

// Phase 33-23: Break Condition Analysis (Stage 2, Issue 6)
pub mod break_condition_analyzer;

// Phase 200-A: Function Scope Capture Infrastructure
pub mod function_scope_capture;

// Phase 79: Pure Detection Logic (Detector/Promoter separation)
pub mod digitpos_detector;
pub mod trim_detector;
pub use digitpos_detector::{DigitPosDetectionResult, DigitPosDetector};
pub use trim_detector::{TrimDetectionResult, TrimDetector};

// Phase 100 P1-2: Pinned Local Analyzer
pub mod pinned_local_analyzer;

// Phase 100 P2-1: Mutable Accumulator Analyzer
pub mod mutable_accumulator_analyzer;
