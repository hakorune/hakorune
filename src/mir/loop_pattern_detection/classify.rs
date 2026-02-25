use super::features::LoopFeatures;
use super::kind::LoopPatternKind;

/// Classify loop pattern based on feature vector.
///
/// This function implements the pattern classification logic using
/// structure-based rules. It does NOT depend on function names or
/// variable names like "sum".
///
/// # Pattern Classification Rules (Phase 131-11: InfiniteEarlyExit added)
///
/// 1. **Pattern 5 (InfiniteEarlyExit)**: `is_infinite_loop && has_break && has_continue`
///    - Priority: Check first (most specific - infinite loop with both break and continue)
///    - Phase 131-11: New pattern for `loop(true) { break + continue }`
///
/// 2. **Pattern 4 (Continue)**: `has_continue && !has_break`
///    - Priority: Check second (has continue but no break)
///    - Phase 131-11: Narrowed to exclude break+continue cases
///
/// 3. **Pattern 3 (If-PHI)**: `has_if && carrier_count >= 1 && !has_break && !has_continue`
///    - Phase 212.5: Changed from carrier_count > 1 to structural if detection
///    - Includes single-carrier if-update patterns (e.g., if-sum with 1 carrier)
///
/// 4. **Pattern 2 (Break)**: `has_break && !has_continue`
///    - Has break but no continue
///
/// 5. **Pattern 1 (Simple While)**: `!has_break && !has_continue && !has_if`
///    - Phase 212.5: Exclude loops with if statements
///    - No control flow alterations
///
/// # Arguments
/// * `features` - Feature vector from extract_features()
///
/// # Returns
/// * `LoopPatternKind` - Classified pattern
///
/// # Phase 183: Unified Detection
///
/// This is the single source of truth for pattern classification.
/// Both routers (`router.rs` and `loop_pattern_router.rs`) use this
/// function to avoid duplicate detection logic.
pub fn classify(features: &LoopFeatures) -> LoopPatternKind {
    // Phase 188.1: Pattern 6: NestedLoop (1-level only, check first after depth validation)
    // Reject 2+ level nesting (explicit error) BEFORE any pattern matching
    if features.max_loop_depth > 2 {
        // Return Unknown to trigger explicit error in router
        return LoopPatternKind::Unknown;
    }

    // Pattern 6: NestedLoop Minimal (1-level nested, simple while inside simple while)
    if features.max_loop_depth == 2
        && features.has_inner_loops
        && !features.has_break
        && !features.has_continue
    {
        return LoopPatternKind::Pattern6NestedLoopMinimal;
    }

    // Phase 131-11: Pattern 5: InfiniteEarlyExit (highest priority - most specific)
    // MUST check before Pattern 4 to avoid misrouting break+continue cases
    if features.is_infinite_loop && features.has_break && features.has_continue {
        return LoopPatternKind::InfiniteEarlyExit;
    }

    // Pattern 4: Continue
    // Phase 131-11: Break+continue stays Pattern5 only for infinite loops
    if features.has_continue {
        return LoopPatternKind::Pattern4Continue;
    }

    // Pattern 3: If-PHI (check before Pattern 1)
    // Phase 212.5: Structural if detection - route to P3 if has_if && carrier_count >= 1
    if features.has_if
        && features.carrier_count >= 1
        && !features.has_break
        && !features.has_continue
    {
        return LoopPatternKind::Pattern3IfPhi;
    }

    // Pattern 2: Break
    if features.has_break && !features.has_continue {
        return LoopPatternKind::Pattern2Break;
    }

    // Pattern 1: Simple While
    // Phase 212.5: Exclude loops with if statements (they go to P3)
    if !features.has_break && !features.has_continue && !features.has_if {
        return LoopPatternKind::Pattern1SimpleWhile;
    }

    // Unknown pattern
    LoopPatternKind::Unknown
}

/// Phase 193-3: Diagnose pattern classification with details
///
/// This function performs classification AND generates diagnostic information.
/// Useful for debugging and logging.
///
/// # Returns
/// * `(LoopPatternKind, String)` - The classified pattern and a diagnostic message
pub fn classify_with_diagnosis(features: &LoopFeatures) -> (LoopPatternKind, String) {
    let pattern = classify(features);
    let reason = match pattern {
        LoopPatternKind::Pattern4Continue => {
            format!(
                "Has continue statement (continue_count={})",
                features.continue_count
            )
        }
        LoopPatternKind::Pattern3IfPhi => {
            format!(
                "Has if-else PHI with {} carriers, no break/continue",
                features.carrier_count
            )
        }
        LoopPatternKind::Pattern2Break => {
            format!(
                "Has break statement (break_count={}), no continue",
                features.break_count
            )
        }
        LoopPatternKind::Pattern1SimpleWhile => {
            "Simple while loop with no special control flow".to_string()
        }
        LoopPatternKind::Pattern6NestedLoopMinimal => {
            format!(
                "Nested loop (1-level, max_loop_depth={}) with no break/continue",
                features.max_loop_depth
            )
        }
        LoopPatternKind::InfiniteEarlyExit => {
            format!(
                "Infinite loop (loop(true)) with both break and continue (break_count={}, continue_count={})",
                features.break_count, features.continue_count
            )
        }
        LoopPatternKind::Unknown => {
            format!("Unknown pattern: {}", features.debug_stats())
        }
    };

    (pattern, reason)
}
