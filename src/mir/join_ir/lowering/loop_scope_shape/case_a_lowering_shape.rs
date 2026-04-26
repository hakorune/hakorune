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
    /// Detect lowering shape from LoopFeatures (legacy, use detect_with_updates when possible)
    ///
    /// # Phase 170-A Design Principle
    ///
    /// **CaseALoweringShape does NOT look at function names.**
    /// Input: LoopFeatures / LoopRouteKind only (structure-based detection).
    ///
    /// This is the core architectural invariant that enables generic routing.
    ///
    /// # Heuristics
    /// - Single carrier (1) → likely StringExamination or ArrayAccumulation
    /// - Multiple carriers (2+) → likely IterationWithAccumulation
    /// - has_break/has_continue → affects Case-A eligibility
    ///
    /// # Phase 170-C Future Work
    /// - Use detect_with_updates() for better single-carrier classification
    ///
    /// # Arguments
    /// * `features` - LoopFeatures (structure-based, name-agnostic)
    /// * `carrier_count` - Number of carrier variables from LoopScopeShape
    /// * `has_progress_carrier` - Whether progress carrier exists
    #[allow(dead_code)]
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

        // Phase 170-A: Simple heuristic based on carrier count
        match carrier_count {
            0 => {
                // This shouldn't happen if has_progress_carrier is true, but be safe
                CaseALoweringShape::NotCaseA
            }
            1 => {
                // Single carrier: could be StringExamination or ArrayAccumulation
                // Further distinction requires analyzing loop body (Phase 170-C)
                // For now, return Generic to allow both paths to be tried
                CaseALoweringShape::Generic
            }
            2.. => {
                // Multiple carriers: likely Iteration with accumulation
                // (progress carrier + accumulator)
                CaseALoweringShape::IterationWithAccumulation
            }
        }
    }

    /// Phase 170-C-1: Carrier pattern を使った精度向上版
    ///
    /// carrier_count == 1 の場合に carrier 名パターンで StringExamination vs ArrayAccumulation を区別
    ///
    /// # Design Notes
    ///
    /// 本来は LoopUpdateAnalyzer を使って AST レベルの更新式を解析する予定だったが、
    /// loop_to_join.rs は MIR レベルで動作するため AST にアクセスできない。
    ///
    /// そのため、Phase 170-C-1 では以下の簡易的なヒューリスティックを使用：
    /// - progress carrier の名前が 'i', 'e', 'idx' などの典型的なインデックス名 → StringExamination
    /// - それ以外 → ArrayAccumulation
    ///
    /// # Future Work (Phase 170-C-2+)
    ///
    /// - MIR 命令を解析して更新パターンを抽出（BinOp の定数加算パターンなど）
    /// - より正確な分類を実現
    ///
    /// # Arguments
    /// * `features` - LoopFeatures (structure-based)
    /// * `carrier_count` - Number of carrier variables
    /// * `has_progress_carrier` - Whether progress carrier exists
    /// * `progress_carrier_name` - Name of progress carrier (if available)
    ///
    /// # Returns
    /// More precise CaseALoweringShape classification
    #[allow(dead_code)]
    pub fn detect_with_carrier_name(
        features: &crate::mir::loop_route_detection::LoopFeatures,
        carrier_count: usize,
        has_progress_carrier: bool,
        progress_carrier_name: Option<&str>,
    ) -> Self {
        // Case-A requirement: must have a progress carrier
        if !has_progress_carrier {
            return CaseALoweringShape::NotCaseA;
        }

        // Case-A requirement: no complex control flow (continue)
        if features.has_continue {
            return CaseALoweringShape::NotCaseA;
        }

        match carrier_count {
            0 => CaseALoweringShape::NotCaseA,
            1 => {
                // Phase 170-C-1: carrier 名パターンで StringExamination vs ArrayAccumulation を区別
                if let Some(name) = progress_carrier_name {
                    if Self::is_typical_index_name(name) {
                        // 'i', 'e', 'idx' などの典型的なインデックス名 → StringExamination (skip/trim 系)
                        CaseALoweringShape::StringExamination
                    } else {
                        // それ以外 → ArrayAccumulation
                        CaseALoweringShape::ArrayAccumulation
                    }
                } else {
                    // carrier 名が不明な場合は Generic（従来通り）
                    CaseALoweringShape::Generic
                }
            }
            2.. => CaseALoweringShape::IterationWithAccumulation,
        }
    }

    /// Phase 170-C-2b: LoopUpdateSummary を使った精度向上版
    ///
    /// `features.update_summary` から UpdateKind を取得して判定。
    /// これにより、carrier 名を直接見るコードを CaseALoweringShape から排除できる。
    ///
    /// # Arguments
    /// * `features` - LoopFeatures (update_summary 含む)
    /// * `carrier_count` - Number of carrier variables
    /// * `has_progress_carrier` - Whether progress carrier exists
    ///
    /// # Returns
    /// CaseALoweringShape classification based on UpdateSummary
    pub fn detect_with_updates(
        features: &crate::mir::loop_route_detection::LoopFeatures,
        carrier_count: usize,
        has_progress_carrier: bool,
    ) -> Self {
        use crate::mir::join_ir::lowering::loop_update_summary::UpdateKind;

        // Case-A requirement: must have a progress carrier
        if !has_progress_carrier {
            return CaseALoweringShape::NotCaseA;
        }

        // Case-A requirement: no complex control flow (continue)
        if features.has_continue {
            return CaseALoweringShape::NotCaseA;
        }

        // Phase 170-C-2b: Use UpdateSummary if available
        if let Some(ref summary) = features.update_summary {
            // Single carrier with CounterLike update → StringExamination
            if summary.carriers.len() == 1 && summary.carriers[0].kind == UpdateKind::CounterLike {
                return CaseALoweringShape::StringExamination;
            }

            // Any AccumulationLike carrier → ArrayAccumulation (for single carrier)
            // or IterationWithAccumulation (for multiple carriers)
            if summary
                .carriers
                .iter()
                .any(|c| c.kind == UpdateKind::AccumulationLike)
            {
                if carrier_count == 1 {
                    return CaseALoweringShape::ArrayAccumulation;
                } else {
                    return CaseALoweringShape::IterationWithAccumulation;
                }
            }

            // Multiple carriers without clear accumulation → IterationWithAccumulation
            if carrier_count >= 2 {
                return CaseALoweringShape::IterationWithAccumulation;
            }
        }

        // Fallback to carrier count heuristic
        match carrier_count {
            0 => CaseALoweringShape::NotCaseA,
            1 => CaseALoweringShape::Generic,
            2.. => CaseALoweringShape::IterationWithAccumulation,
        }
    }

    /// Typical index variable name detection
    ///
    /// StringExamination パターン（skip/trim）で使われる典型的なインデックス名：
    /// - 'i', 'e', 'idx', 'pos', 'start', 'end'
    ///
    /// ArrayAccumulation パターンは通常：
    /// - より意味のある名前 ('result', 'items', 'defs' など)
    #[allow(dead_code)]
    fn is_typical_index_name(name: &str) -> bool {
        matches!(
            name,
            "i" | "e" | "idx" | "index" | "pos" | "position" | "start" | "end"
        )
    }

    /// Legacy wrapper: Detect from LoopScopeShape (deprecated, use detect_from_features)
    ///
    /// Phase 170-A: Kept for backward compatibility during transition.
    #[deprecated(
        since = "Phase 170-A",
        note = "Use detect_from_features() with LoopFeatures instead"
    )]
    #[allow(dead_code)]
    pub fn detect(scope: &super::shape::LoopScopeShape) -> Self {
        // Construct minimal LoopFeatures from LoopScopeShape
        // Note: This loses some information (has_break, has_continue not available)
        let has_progress_carrier = scope.progress_carrier.is_some();
        let carrier_count = scope.carriers.len();

        // Create stub features without update_summary. Carrier names alone are
        // not an update-kind proof; only observed AST/MIR updates may populate
        // LoopFeatures.update_summary.
        let stub_features = crate::mir::loop_route_detection::LoopFeatures {
            carrier_count,
            ..Default::default() // Phase 188.1: Use Default for nesting fields
        };

        Self::detect_from_features(&stub_features, carrier_count, has_progress_carrier)
    }

    /// Is this a recognized lowering shape?
    #[allow(dead_code)]
    pub fn is_recognized(&self) -> bool {
        !matches!(
            self,
            CaseALoweringShape::NotCaseA | CaseALoweringShape::Generic
        )
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
    use crate::mir::BasicBlockId;
    use std::collections::{BTreeMap, BTreeSet};

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
    fn test_is_recognized() {
        assert!(CaseALoweringShape::StringExamination.is_recognized());
        assert!(CaseALoweringShape::ArrayAccumulation.is_recognized());
        assert!(CaseALoweringShape::IterationWithAccumulation.is_recognized());
        assert!(!CaseALoweringShape::Generic.is_recognized());
        assert!(!CaseALoweringShape::NotCaseA.is_recognized());
    }

    fn scope_with_carriers(
        carriers: &[&str],
        progress_carrier: Option<&str>,
    ) -> super::super::shape::LoopScopeShape {
        super::super::shape::LoopScopeShape {
            header: BasicBlockId::new(1),
            body: BasicBlockId::new(2),
            latch: BasicBlockId::new(3),
            exit: BasicBlockId::new(4),
            pinned: BTreeSet::new(),
            carriers: carriers.iter().map(|name| (*name).to_string()).collect(),
            body_locals: BTreeSet::new(),
            exit_live: BTreeSet::new(),
            progress_carrier: progress_carrier.map(str::to_string),
            variable_definitions: BTreeMap::new(),
        }
    }

    #[test]
    #[allow(deprecated)]
    fn case_a_update_summary_name_only_does_not_synthesize_accumulation() {
        let scope = scope_with_carriers(&["defs"], Some("defs"));

        assert_eq!(
            CaseALoweringShape::detect(&scope),
            CaseALoweringShape::Generic
        );
    }
}
