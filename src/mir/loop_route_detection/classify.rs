use super::features::LoopFeatures;
use super::kind::LoopRouteKind;

/// Classify flat loop route family based on feature vector.
///
/// This function implements flat route classification using structure-based
/// rules. It does NOT depend on function names or variable names like "sum".
/// NestedLoopMinimal is selected by the AST/StepTree route path, not by this
/// `LoopFeatures` classifier.
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
/// * `LoopRouteKind` - Classified route family
///
/// # Phase 183: Unified Detection
///
/// This is the single source of truth for route classification.
/// The AST route path uses this after its StepTree nested-loop precheck; the
/// LoopForm router uses it for LoopForm-observable flat route facts.
pub fn classify(features: &LoopFeatures) -> LoopRouteKind {
    // Phase 131-11: LoopTrueEarlyExit (highest priority - most specific)
    // MUST check before LoopContinueOnly to avoid misrouting break+continue cases
    if features.is_infinite_loop && features.has_break && features.has_continue {
        return LoopRouteKind::LoopTrueEarlyExit;
    }

    // LoopContinueOnly
    // Phase 131-11: Break+continue stays LoopTrueEarlyExit only for infinite loops
    if features.has_continue {
        return LoopRouteKind::LoopContinueOnly;
    }

    // IfPhiJoin (check before LoopSimpleWhile)
    // Phase 212.5: Structural if detection - route to IfPhiJoin if has_if && carrier_count >= 1
    if features.has_if
        && features.carrier_count >= 1
        && !features.has_break
        && !features.has_continue
    {
        return LoopRouteKind::IfPhiJoin;
    }

    // LoopBreak
    if features.has_break && !features.has_continue {
        return LoopRouteKind::LoopBreak;
    }

    // LoopSimpleWhile
    // Phase 212.5: Exclude loops with if statements (they go to IfPhiJoin)
    if !features.has_break && !features.has_continue && !features.has_if {
        return LoopRouteKind::LoopSimpleWhile;
    }

    // Unknown route family
    LoopRouteKind::Unknown
}
