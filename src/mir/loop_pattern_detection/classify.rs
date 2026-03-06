use super::features::LoopFeatures;
use super::kind::LoopPatternKind;

/// Classify loop route family based on feature vector.
///
/// This function implements the route classification logic using
/// structure-based rules. It does NOT depend on function names or
/// variable names like "sum".
///
/// # Route Classification Rules (Phase 131-11: LoopTrueEarlyExit added)
///
/// 1. **LoopTrueEarlyExit**: `is_infinite_loop && has_break && has_continue`
///    - Priority: Check first (most specific - infinite loop with both break and continue)
///    - Phase 131-11: Route for `loop(true) { break + continue }`
///
/// 2. **LoopContinueOnly**: `has_continue && !has_break`
///    - Priority: Check second (has continue but no break)
///    - Phase 131-11: Narrowed to exclude break+continue cases
///
/// 3. **IfPhiJoin**: `has_if && carrier_count >= 1 && !has_break && !has_continue`
///    - Phase 212.5: Changed from carrier_count > 1 to structural if detection
///    - Includes single-carrier if-update patterns (e.g., if-sum with 1 carrier)
///
/// 4. **LoopBreak**: `has_break && !has_continue`
///
/// 5. **LoopSimpleWhile**: `!has_break && !has_continue && !has_if`
///    - Phase 212.5: Exclude loops with if statements
///    - No control flow alterations
///
/// # Arguments
/// * `features` - Feature vector from extract_features()
///
/// # Returns
/// * `LoopPatternKind` - Classified route family
///
/// # Phase 183: Unified Detection
///
/// This is the single source of truth for route classification.
/// Both routers (`router.rs` and `loop_pattern_router.rs`) use this
/// function to avoid duplicate detection logic.
pub fn classify(features: &LoopFeatures) -> LoopPatternKind {
    // Phase 188.1: NestedLoopMinimal (1-level only, check first after depth validation)
    // Reject 2+ level nesting (explicit error) BEFORE any pattern matching
    if features.max_loop_depth > 2 {
        // Return Unknown to trigger explicit error in router
        return LoopPatternKind::Unknown;
    }

    // NestedLoopMinimal: 1-level nested, simple-while-compatible inner/outer loops
    if features.max_loop_depth == 2
        && features.has_inner_loops
        && !features.has_break
        && !features.has_continue
    {
        return LoopPatternKind::NestedLoopMinimal;
    }

    // Phase 131-11: LoopTrueEarlyExit (highest priority - most specific)
    // MUST check before LoopContinueOnly to avoid misrouting break+continue cases
    if features.is_infinite_loop && features.has_break && features.has_continue {
        return LoopPatternKind::LoopTrueEarlyExit;
    }

    // LoopContinueOnly
    // Phase 131-11: Break+continue stays LoopTrueEarlyExit only for infinite loops
    if features.has_continue {
        return LoopPatternKind::LoopContinueOnly;
    }

    // IfPhiJoin (check before LoopSimpleWhile)
    // Phase 212.5: Structural if detection - route to IfPhiJoin if has_if && carrier_count >= 1
    if features.has_if
        && features.carrier_count >= 1
        && !features.has_break
        && !features.has_continue
    {
        return LoopPatternKind::IfPhiJoin;
    }

    // LoopBreak
    if features.has_break && !features.has_continue {
        return LoopPatternKind::LoopBreak;
    }

    // LoopSimpleWhile
    // Phase 212.5: Exclude loops with if statements (they go to IfPhiJoin)
    if !features.has_break && !features.has_continue && !features.has_if {
        return LoopPatternKind::LoopSimpleWhile;
    }

    // Unknown route family
    LoopPatternKind::Unknown
}

/// Phase 193-3: Diagnose route classification with details
///
/// This function performs classification AND generates diagnostic information.
/// Useful for debugging and logging.
///
/// # Returns
/// * `(LoopPatternKind, String)` - The classified route family and a diagnostic message
pub fn classify_with_diagnosis(features: &LoopFeatures) -> (LoopPatternKind, String) {
    let route_kind = classify(features);
    let reason = match route_kind {
        LoopPatternKind::LoopContinueOnly => {
            format!(
                "Has continue statement (continue_count={})",
                features.continue_count
            )
        }
        LoopPatternKind::IfPhiJoin => {
            format!(
                "Has if-else PHI with {} carriers, no break/continue",
                features.carrier_count
            )
        }
        LoopPatternKind::LoopBreak => {
            format!(
                "Has break statement (break_count={}), no continue",
                features.break_count
            )
        }
        LoopPatternKind::LoopSimpleWhile => {
            "Simple while loop with no special control flow".to_string()
        }
        LoopPatternKind::NestedLoopMinimal => {
            format!(
                "Nested loop (1-level, max_loop_depth={}) with no break/continue",
                features.max_loop_depth
            )
        }
        LoopPatternKind::LoopTrueEarlyExit => {
            format!(
                "Infinite loop (loop(true)) with both break and continue (break_count={}, continue_count={})",
                features.break_count, features.continue_count
            )
        }
        LoopPatternKind::Unknown => {
            format!("Unknown route: {}", features.debug_stats())
        }
    };

    (route_kind, reason)
}
