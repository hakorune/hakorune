//! Case-A Loop Lowering Shape Detection
//!
//! Phase 170-A: Structure-based routing for Case-A loops
//!
//! Replaces hardcoded function name matching with AST-based pattern detection.
//! Enables reuse of Case-A lowering functions for ANY loop with matching structure,
//! not just hardcoded function names like "Main.skip/1".

/// Recognized Case-A loop body patterns
///
/// Used to dispatch to appropriate lowerer when function name is unavailable.
///
/// Case-A loops are characterized by:
/// - Single exit group (one loop exit path)
/// - Progress carrier (monotonically increasing/decreasing variable that drives termination)
/// - Pinned parameters (loop-invariant values that don't change during iteration)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseALoweringShape {
    /// String examination loop: conditional carrier update
    ///
    /// Examples: Main.skip/1, FuncScannerBox.trim/1
    ///
    /// Shape:
    /// - Loop body: single if statement examining character
    /// - Condition: examines string character (e.g., ch == " ")
    /// - Then branch: updates carrier (e.g., i = i + 1)
    /// - Else branch: exits loop (break)
    /// - Carriers: 1 (progress variable i or e)
    /// - Return value: final progress carrier
    ///
    /// Signature: (StringBox, ...) -> Integer
    StringExamination,

    /// Array accumulation loop: linear iteration with collection mutation
    ///
    /// Examples: FuncScannerBox.append_defs/2
    ///
    /// Shape:
    /// - Loop body: access arr[i], mutate collection (push/add)
    /// - Condition: i < array.length()
    /// - Increment: i = i + 1
    /// - Exit: void or collection mutation
    /// - Carriers: 1 (progress variable i)
    /// - Pinned: 1-2 (array/collection, optional length)
    ///
    /// Signature: (CollectionBox, ...) -> Void or CollectionBox
    ArrayAccumulation,

    /// Iteration with accumulation: linear iteration + value accumulation
    ///
    /// Examples: Stage1UsingResolverBox.resolve_for_source/5
    ///
    /// Shape:
    /// - Loop body: access arr[i], update accumulator
    /// - Multiple carriers: progress (i) + result (prefix, sum, etc.)
    /// - Condition: i < array.length()
    /// - Exit: accumulated value
    /// - Carriers: 2+ (progress i + accumulator)
    /// - Pinned: 2+ (array, initial value, etc.)
    ///
    /// Signature: (CollectionBox, InitialValueBox, ...) -> ResultBox
    IterationWithAccumulation,

    /// Generic Case-A: falls outside recognized patterns
    ///
    /// Structure is Case-A (single exit + progress carrier) but body
    /// doesn't match any known pattern.
    ///
    /// Lowering: Should either:
    /// 1. Implement generic extraction from LoopForm (Phase 170-B+), or
    /// 2. Return None to trigger fallback to legacy LoopBuilder
    Generic,

    /// Unknown: not a Case-A loop at all
    ///
    /// Structural requirements not met:
    /// - Multiple exit groups, OR
    /// - No progress carrier
    ///
    /// Should use route-based lowering instead.
    NotCaseA,
}

impl CaseALoweringShape {
    /// Detect lowering shape from LoopFeatures.
    ///
    /// # Phase 170-A Design Principle
    ///
    /// **CaseALoweringShape does NOT look at function names.**
    /// Input: LoopFeatures / LoopRouteKind only (structure-based detection).
    ///
    /// This is the core architectural invariant that enables generic routing.
    ///
    /// # Heuristics
    /// - Carrier count alone is not enough to select a specialized lowerer.
    /// - has_break/has_continue → affects Case-A eligibility
    ///
    /// # Arguments
    /// * `features` - LoopFeatures (structure-based, name-agnostic)
    /// * `carrier_count` - Number of carrier variables from LoopScopeShape
    /// * `has_progress_carrier` - Whether progress carrier exists
    pub fn detect_from_features(
        features: &crate::mir::loop_route_detection::LoopFeatures,
        carrier_count: usize,
        has_progress_carrier: bool,
    ) -> Self {
        // Case-A requirement: must have a progress carrier
        if !has_progress_carrier {
            return CaseALoweringShape::NotCaseA;
        }

        // Case-A requirement: no complex control flow (continue)
        // Note: break is allowed (StringExamination patterns use break)
        if features.has_continue {
            return CaseALoweringShape::NotCaseA;
        }

        // Carrier count alone is not update-shape proof. Keep recognized
        // lowerer selection on observed update metadata or descriptor fallback.
        match carrier_count {
            0 => {
                // This shouldn't happen if has_progress_carrier is true, but be safe
                CaseALoweringShape::NotCaseA
            }
            _ => CaseALoweringShape::Generic,
        }
    }

    /// Get human-readable name for tracing/debugging
    pub fn name(&self) -> &'static str {
        match self {
            CaseALoweringShape::StringExamination => "StringExamination",
            CaseALoweringShape::ArrayAccumulation => "ArrayAccumulation",
            CaseALoweringShape::IterationWithAccumulation => "IterationWithAccumulation",
            CaseALoweringShape::Generic => "Generic",
            CaseALoweringShape::NotCaseA => "NotCaseA",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_display() {
        assert_eq!(
            CaseALoweringShape::StringExamination.name(),
            "StringExamination"
        );
        assert_eq!(
            CaseALoweringShape::ArrayAccumulation.name(),
            "ArrayAccumulation"
        );
        assert_eq!(
            CaseALoweringShape::IterationWithAccumulation.name(),
            "IterationWithAccumulation"
        );
        assert_eq!(CaseALoweringShape::Generic.name(), "Generic");
        assert_eq!(CaseALoweringShape::NotCaseA.name(), "NotCaseA");
    }

    #[test]
    fn case_a_carrier_count_without_update_proof_stays_generic() {
        let features = crate::mir::loop_route_detection::LoopFeatures {
            carrier_count: 2,
            ..Default::default()
        };

        assert_eq!(
            CaseALoweringShape::detect_from_features(&features, 2, true),
            CaseALoweringShape::Generic
        );
    }
}
