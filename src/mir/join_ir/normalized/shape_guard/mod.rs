#![cfg(feature = "normalized_dev")]

//! Shape guard system - Pattern detection and capability-based routing
//!
//! This module is organized into pattern-based submodules:
//! - `pattern2`: Simple break-loop patterns (P2 core)
//! - `pattern3`: If-sum patterns with conditional carrier updates
//! - `pattern4`: Continue patterns with loop-internal control flow
//! - `selfhost`: Selfhost-specific P2/P3 patterns
//! - `utils`: Shared utility functions

use crate::config::env::joinir_dev_enabled;
use crate::runtime::get_global_ring0;
use crate::mir::join_ir::normalized::dev_env;
use crate::mir::join_ir::{JoinFunction, JoinInst, JoinModule};

mod pattern2;
mod pattern3;
mod pattern4;
mod selfhost;
mod utils;

// Re-export all detector functions to maintain the existing API
pub(crate) use pattern2::{
    is_jsonparser_atoi_mini, is_jsonparser_atoi_real, is_jsonparser_parse_number_real,
    is_jsonparser_skip_ws_mini, is_jsonparser_skip_ws_real,
};
pub(crate) use pattern3::{
    is_pattern3_if_sum_json, is_pattern3_if_sum_minimal, is_pattern3_if_sum_multi,
};
pub(crate) use pattern4::{
    is_jsonparser_parse_array_continue_skip_ws, is_jsonparser_parse_object_continue_skip_ws,
    is_parse_string_composite_minimal, is_pattern4_continue_minimal,
    is_pattern_continue_return_minimal,
};
pub(crate) use selfhost::{
    is_selfhost_args_parse_p2, is_selfhost_detect_format_p3, is_selfhost_if_sum_p3,
    is_selfhost_if_sum_p3_ext, is_selfhost_stmt_count_p3, is_selfhost_token_scan_p2,
    is_selfhost_token_scan_p2_accum, is_selfhost_verify_schema_p2,
};

/// Phase 44: Shape capability kinds (capability-based routing)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeCapabilityKind {
    /// P2 Core: Simple mini patterns (i/acc/n etc)
    P2CoreSimple,

    /// P2 Core: skip_whitespace mini/real
    P2CoreSkipWs,

    /// P2 Core: _atoi mini/real
    P2CoreAtoi,

    /// P2 Mid: _parse_number real (p + num_str + result)
    P2MidParseNumber,

    /// P3 If-Sum family (minimal/multi/json)
    P3IfSum,

    /// P4 Continue (skip whitespace) family
    P4ContinueSkipWs,

    /// P4 Continue + Early Return family (Phase 89)
    P4ContinueEarlyReturn,

    /// Composite Parse String (continue + early return + variable step) (Phase 90)
    CompositeParseString,

    /// Selfhost P2 core (token scan)
    SelfhostP2Core,

    /// Selfhost P3 if-sum family
    SelfhostP3IfSum,
    // Future: Other P2 patterns
    // P2MidAtOfLoop,
    // P2HeavyString,
}

/// Phase 44: Shape capability descriptor
#[derive(Debug, Clone)]
pub struct ShapeCapability {
    pub kind: ShapeCapabilityKind,
    // Future extensibility fields (not all used yet):
    // pub pattern_kind: LoopPatternKind,
    // pub loop_param_count: usize,
    // pub carrier_roles: Vec<CarrierRole>,
    // pub method_calls: Vec<MethodCallSignature>,
}

impl ShapeCapability {
    pub fn new(kind: ShapeCapabilityKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizedDevShape {
    Pattern1Mini,
    Pattern2Mini,
    JsonparserSkipWsMini,
    JsonparserSkipWsReal,
    JsonparserAtoiMini,
    JsonparserAtoiReal,
    JsonparserParseNumberReal,
    // Phase 47-A: Pattern3 (if-sum) minimal
    Pattern3IfSumMinimal,
    // Phase 47-B: Pattern3 extended (multi/json)
    Pattern3IfSumMulti,
    Pattern3IfSumJson,
    // Phase 48-A: Pattern4 (continue) minimal
    Pattern4ContinueMinimal,
    // Phase 48-B: Pattern4 (continue) JsonParser skip_ws (array/object)
    JsonparserParseArrayContinueSkipWs,
    JsonparserParseObjectContinueSkipWs,
    // Phase 50: selfhost P2/P3 dev shapes
    SelfhostTokenScanP2,
    SelfhostIfSumP3,
    // Phase 51: selfhost P2/P3 dev extensions
    SelfhostTokenScanP2Accum,
    SelfhostIfSumP3Ext,
    // Phase 53: selfhost P2/P3 practical variations
    SelfhostArgsParseP2,
    SelfhostStmtCountP3,
    // Phase 54: selfhost P2/P3 shape growth (structural axis expansion)
    SelfhostVerifySchemaP2,
    SelfhostDetectFormatP3,
    // Phase 89: Continue + Early Return pattern (dev-only)
    PatternContinueReturnMinimal,
    // Phase 90: Parse String Composite pattern (dev-only: continue + early return + variable step)
    ParseStringCompositeMinimal,
}

type Detector = fn(&JoinModule) -> bool;

const SHAPE_DETECTORS: &[(NormalizedDevShape, Detector)] = &[
    (NormalizedDevShape::Pattern1Mini, pattern2::is_pattern1_mini),
    (NormalizedDevShape::Pattern2Mini, pattern2::is_pattern2_mini),
    (
        NormalizedDevShape::JsonparserSkipWsMini,
        pattern2::is_jsonparser_skip_ws_mini,
    ),
    (
        NormalizedDevShape::JsonparserSkipWsReal,
        pattern2::is_jsonparser_skip_ws_real,
    ),
    (
        NormalizedDevShape::JsonparserAtoiMini,
        pattern2::is_jsonparser_atoi_mini,
    ),
    (
        NormalizedDevShape::JsonparserAtoiReal,
        pattern2::is_jsonparser_atoi_real,
    ),
    (
        NormalizedDevShape::JsonparserParseNumberReal,
        pattern2::is_jsonparser_parse_number_real,
    ),
    (
        NormalizedDevShape::SelfhostTokenScanP2,
        selfhost::is_selfhost_token_scan_p2,
    ),
    (
        NormalizedDevShape::SelfhostTokenScanP2Accum,
        selfhost::is_selfhost_token_scan_p2_accum,
    ),
    // Phase 47-A: Pattern3 if-sum minimal
    (
        NormalizedDevShape::Pattern3IfSumMinimal,
        pattern3::is_pattern3_if_sum_minimal,
    ),
    (
        NormalizedDevShape::Pattern3IfSumMulti,
        pattern3::is_pattern3_if_sum_multi,
    ),
    (
        NormalizedDevShape::Pattern3IfSumJson,
        pattern3::is_pattern3_if_sum_json,
    ),
    // Phase 48-A: Pattern4 continue minimal
    (
        NormalizedDevShape::Pattern4ContinueMinimal,
        pattern4::is_pattern4_continue_minimal,
    ),
    (
        NormalizedDevShape::JsonparserParseArrayContinueSkipWs,
        pattern4::is_jsonparser_parse_array_continue_skip_ws,
    ),
    (
        NormalizedDevShape::JsonparserParseObjectContinueSkipWs,
        pattern4::is_jsonparser_parse_object_continue_skip_ws,
    ),
    (
        NormalizedDevShape::SelfhostIfSumP3,
        selfhost::is_selfhost_if_sum_p3,
    ),
    (
        NormalizedDevShape::SelfhostIfSumP3Ext,
        selfhost::is_selfhost_if_sum_p3_ext,
    ),
    // Phase 53: selfhost P2/P3 practical variations
    (
        NormalizedDevShape::SelfhostArgsParseP2,
        selfhost::is_selfhost_args_parse_p2,
    ),
    (
        NormalizedDevShape::SelfhostStmtCountP3,
        selfhost::is_selfhost_stmt_count_p3,
    ),
    // Phase 54: selfhost P2/P3 shape growth
    (
        NormalizedDevShape::SelfhostVerifySchemaP2,
        selfhost::is_selfhost_verify_schema_p2,
    ),
    (
        NormalizedDevShape::SelfhostDetectFormatP3,
        selfhost::is_selfhost_detect_format_p3,
    ),
    // Phase 89: Continue + Early Return pattern
    (
        NormalizedDevShape::PatternContinueReturnMinimal,
        pattern4::is_pattern_continue_return_minimal,
    ),
    // Phase 90: Parse String Composite pattern
    (
        NormalizedDevShape::ParseStringCompositeMinimal,
        pattern4::is_parse_string_composite_minimal,
    ),
];

/// direct ブリッジで扱う shape（dev 限定）。
pub(crate) fn direct_shapes(module: &JoinModule) -> Vec<NormalizedDevShape> {
    let shapes = detect_shapes(module);
    log_shapes("direct", &shapes);
    shapes
}

/// Structured→Normalized の対象 shape（dev 限定）。
pub(crate) fn supported_shapes(module: &JoinModule) -> Vec<NormalizedDevShape> {
    let shapes = detect_shapes(module);
    log_shapes("roundtrip", &shapes);
    shapes
}

/// Phase 44: Map NormalizedDevShape to ShapeCapability
pub fn capability_for_shape(shape: &NormalizedDevShape) -> ShapeCapability {
    use NormalizedDevShape::*;
    use ShapeCapabilityKind::*;

    let kind = match shape {
        Pattern2Mini => P2CoreSimple,
        JsonparserSkipWsMini | JsonparserSkipWsReal => P2CoreSkipWs,
        JsonparserAtoiMini | JsonparserAtoiReal => P2CoreAtoi,
        JsonparserParseNumberReal => P2MidParseNumber,
        Pattern1Mini => P2CoreSimple, // Also core simple pattern
        // Phase 47-B: P3 if-sum family
        Pattern3IfSumMinimal | Pattern3IfSumMulti | Pattern3IfSumJson => P3IfSum,
        // Phase 48-A/B: P4 continue family
        Pattern4ContinueMinimal
        | JsonparserParseArrayContinueSkipWs
        | JsonparserParseObjectContinueSkipWs => P4ContinueSkipWs,
        // Phase 50: selfhost P2/P3 dev shapes
        SelfhostTokenScanP2 | SelfhostTokenScanP2Accum => SelfhostP2Core,
        SelfhostIfSumP3 | SelfhostIfSumP3Ext => SelfhostP3IfSum,
        // Phase 53: selfhost P2/P3 practical variations
        SelfhostArgsParseP2 => SelfhostP2Core,
        SelfhostStmtCountP3 => SelfhostP3IfSum,
        // Phase 54: selfhost P2/P3 shape growth
        SelfhostVerifySchemaP2 => SelfhostP2Core,
        SelfhostDetectFormatP3 => SelfhostP3IfSum,
        // Phase 89: Continue + Early Return pattern (dev-only, dedicated capability)
        PatternContinueReturnMinimal => P4ContinueEarlyReturn,
        // Phase 90: Parse String Composite pattern (dev-only, dedicated capability)
        ParseStringCompositeMinimal => CompositeParseString,
    };

    ShapeCapability::new(kind)
}

/// Phase 46+: Canonical shapes that ALWAYS use Normalized→MIR(direct)
/// regardless of feature flags or mode.
///
/// Canonical set (Phase 48-C):
/// - P2-Core: Pattern2Mini, JsonparserSkipWsMini, JsonparserSkipWsReal, JsonparserAtoiMini
/// - P2-Mid: JsonparserAtoiReal, JsonparserParseNumberReal
/// - P3: Pattern3 If-sum minimal/multi/json
/// - P4: Pattern4 continue minimal + JsonParser skip_ws (array/object)
pub fn is_canonical_shape(shape: &NormalizedDevShape) -> bool {
    use NormalizedDevShape::*;
    matches!(
        shape,
        Pattern2Mini
            | JsonparserSkipWsMini
            | JsonparserSkipWsReal
            | JsonparserAtoiMini
            // Phase 46: Add P2-Mid patterns
            | JsonparserAtoiReal
            | JsonparserParseNumberReal
            // Phase 47-C: P3 if-sum canonical set
            | Pattern3IfSumMinimal
            | Pattern3IfSumMulti
            | Pattern3IfSumJson
            // Phase 48-C: P4 continue canonical set
            | Pattern4ContinueMinimal
            | JsonparserParseArrayContinueSkipWs
            | JsonparserParseObjectContinueSkipWs
    )
}

/// Phase 44: Check if capability kind is in P2-Core family
///
/// This checks capability-level membership, not granular canonical status.
/// Use `is_canonical_shape()` for exact canonical filtering.
pub fn is_p2_core_capability(cap: &ShapeCapability) -> bool {
    use ShapeCapabilityKind::*;
    matches!(
        cap.kind,
        P2CoreSimple
            | P2CoreSkipWs
            | P2CoreAtoi
            | P2MidParseNumber
            | P3IfSum
            | P4ContinueSkipWs
            | P4ContinueEarlyReturn
            | CompositeParseString
            | SelfhostP2Core
            | SelfhostP3IfSum
    )
}

/// Phase 44: Check if capability is supported by Normalized dev
pub fn is_supported_by_normalized(cap: &ShapeCapability) -> bool {
    // Currently same as P2-Core family
    is_p2_core_capability(cap)
}

/// canonical（常時 Normalized 経路を通す）対象。
/// Phase 46: Extract canonical shapes from JoinModule.
///
/// Canonical set (P2-Core + P2-Mid):
/// - Pattern2Mini, skip_ws mini/real, atoi mini/real, parse_number real
///
/// These shapes ALWAYS use Normalized→MIR(direct) regardless of mode.
/// P3/P4 patterns are NOT canonical (future NORM-P3/NORM-P4 phases).
pub(crate) fn canonical_shapes(module: &JoinModule) -> Vec<NormalizedDevShape> {
    let shapes: Vec<_> = detect_shapes(module)
        .into_iter()
        .filter(|s| is_canonical_shape(s))
        .collect();
    log_shapes("canonical", &shapes);
    shapes
}

#[allow(dead_code)]
pub(crate) fn is_direct_supported(module: &JoinModule) -> bool {
    !detect_shapes(module).is_empty()
}

pub fn detect_shapes(module: &JoinModule) -> Vec<NormalizedDevShape> {
    let mut shapes: Vec<_> = SHAPE_DETECTORS
        .iter()
        .filter_map(|(shape, detector)| if detector(module) { Some(*shape) } else { None })
        .collect();

    // Pattern1 は「最小の後方互換」なので、より具体的な shape が見つかった場合は外しておく。
    if shapes.len() > 1 {
        shapes.retain(|s| *s != NormalizedDevShape::Pattern1Mini);
    }

    // selfhost shapesは canonical P2/P3 の generic 判定から分離する
    if shapes.contains(&NormalizedDevShape::SelfhostTokenScanP2)
        || shapes.contains(&NormalizedDevShape::SelfhostTokenScanP2Accum)
        || shapes.contains(&NormalizedDevShape::SelfhostArgsParseP2)
        || shapes.contains(&NormalizedDevShape::SelfhostVerifySchemaP2)
    {
        shapes.retain(|s| {
            *s != NormalizedDevShape::Pattern2Mini
                && *s != NormalizedDevShape::Pattern4ContinueMinimal
        });
    }
    if shapes.contains(&NormalizedDevShape::SelfhostIfSumP3)
        || shapes.contains(&NormalizedDevShape::SelfhostIfSumP3Ext)
        || shapes.contains(&NormalizedDevShape::SelfhostStmtCountP3)
        || shapes.contains(&NormalizedDevShape::SelfhostDetectFormatP3)
    {
        shapes.retain(|s| {
            !matches!(
                s,
                NormalizedDevShape::Pattern3IfSumMinimal
                    | NormalizedDevShape::Pattern3IfSumMulti
                    | NormalizedDevShape::Pattern3IfSumJson
                    | NormalizedDevShape::Pattern4ContinueMinimal
            )
        });
    }

    shapes
}

fn log_shapes(tag: &str, shapes: &[NormalizedDevShape]) {
    if shapes.is_empty() {
        return;
    }
    if dev_env::normalized_dev_logs_enabled() && joinir_dev_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/normalized-dev/shape] {}: {:?}",
            tag, shapes
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "normalized_dev")]
    #[test]
    fn test_detect_pattern3_if_sum_minimal_shape() {
        use crate::mir::join_ir::normalized::fixtures::build_pattern3_if_sum_min_structured_for_normalized_dev;

        let module = build_pattern3_if_sum_min_structured_for_normalized_dev();

        // Should detect Pattern3IfSumMinimal shape
        assert!(
            pattern3::is_pattern3_if_sum_minimal(&module),
            "pattern3_if_sum_minimal fixture should be detected"
        );

        let shapes = detect_shapes(&module);
        assert!(
            shapes.contains(&NormalizedDevShape::Pattern3IfSumMinimal),
            "detect_shapes() should include Pattern3IfSumMinimal, got: {:?}",
            shapes
        );
    }

    #[cfg(feature = "normalized_dev")]
    #[test]
    fn test_selfhost_p2_core_structural_candidate_signature() {
        use crate::mir::join_ir::normalized::fixtures::{
            build_jsonparser_skip_ws_structured_for_normalized_dev,
            build_pattern2_minimal_structured,
            build_selfhost_token_scan_p2_accum_structured_for_normalized_dev,
            build_selfhost_token_scan_p2_structured_for_normalized_dev,
        };

        let selfhost_p2 = build_selfhost_token_scan_p2_structured_for_normalized_dev();
        let selfhost_p2_accum = build_selfhost_token_scan_p2_accum_structured_for_normalized_dev();
        let json_p2 = build_jsonparser_skip_ws_structured_for_normalized_dev();
        let canonical_p2_min = build_pattern2_minimal_structured();

        assert!(
            selfhost::is_selfhost_p2_core_family_candidate(&selfhost_p2),
            "selfhost_token_scan_p2 should match structural candidate"
        );
        assert!(
            selfhost::is_selfhost_p2_core_family_candidate(&selfhost_p2_accum),
            "selfhost_token_scan_p2_accum should match structural candidate"
        );
        // Structural signature is intentionally ambiguous with JsonParser P2-mini family.
        assert!(
            selfhost::is_selfhost_p2_core_family_candidate(&json_p2),
            "jsonparser_skip_ws_mini should also match P2 core candidate"
        );
        assert!(
            !selfhost::is_selfhost_p2_core_family_candidate(&canonical_p2_min),
            "canonical Pattern2Mini fixture should not match selfhost P2 candidate"
        );
    }

    #[cfg(feature = "normalized_dev")]
    #[test]
    fn test_selfhost_p3_if_sum_structural_candidate_signature() {
        use crate::mir::join_ir::normalized::fixtures::{
            build_pattern3_if_sum_min_structured_for_normalized_dev,
            build_pattern3_if_sum_multi_min_structured_for_normalized_dev,
            build_selfhost_if_sum_p3_ext_structured_for_normalized_dev,
            build_selfhost_if_sum_p3_structured_for_normalized_dev,
        };

        let selfhost_p3 = build_selfhost_if_sum_p3_structured_for_normalized_dev();
        let selfhost_p3_ext = build_selfhost_if_sum_p3_ext_structured_for_normalized_dev();
        let canonical_p3_min = build_pattern3_if_sum_min_structured_for_normalized_dev();
        let canonical_p3_multi = build_pattern3_if_sum_multi_min_structured_for_normalized_dev();

        assert!(
            selfhost::is_selfhost_p3_if_sum_family_candidate(&selfhost_p3),
            "selfhost_if_sum_p3 should match structural candidate"
        );
        assert!(
            selfhost::is_selfhost_p3_if_sum_family_candidate(&selfhost_p3_ext),
            "selfhost_if_sum_p3_ext should match structural candidate"
        );
        assert!(
            !selfhost::is_selfhost_p3_if_sum_family_candidate(&canonical_p3_min),
            "canonical P3 minimal should not match selfhost P3 candidate"
        );
        assert!(
            !selfhost::is_selfhost_p3_if_sum_family_candidate(&canonical_p3_multi),
            "canonical P3 multi should not match selfhost P3 candidate"
        );
    }

    #[cfg(feature = "normalized_dev")]
    #[test]
    fn test_detect_selfhost_token_scan_p2_shape() {
        use crate::mir::join_ir::normalized::fixtures::build_selfhost_token_scan_p2_structured_for_normalized_dev;

        let module = build_selfhost_token_scan_p2_structured_for_normalized_dev();
        let shapes = detect_shapes(&module);

        assert!(
            shapes.contains(&NormalizedDevShape::SelfhostTokenScanP2),
            "selfhost_token_scan_p2 shape missing: {:?}",
            shapes
        );
        assert!(
            !shapes.contains(&NormalizedDevShape::Pattern2Mini),
            "selfhost_token_scan_p2 should not be treated as canonical Pattern2Mini: {:?}",
            shapes
        );
    }

    #[cfg(feature = "normalized_dev")]
    #[test]
    fn test_detect_selfhost_token_scan_p2_accum_shape() {
        use crate::mir::join_ir::normalized::fixtures::build_selfhost_token_scan_p2_accum_structured_for_normalized_dev;

        let module = build_selfhost_token_scan_p2_accum_structured_for_normalized_dev();
        let shapes = detect_shapes(&module);

        assert!(
            shapes.contains(&NormalizedDevShape::SelfhostTokenScanP2Accum),
            "selfhost_token_scan_p2_accum shape missing: {:?}",
            shapes
        );
        assert!(
            !shapes.contains(&NormalizedDevShape::Pattern2Mini),
            "selfhost_token_scan_p2_accum should not be treated as canonical Pattern2Mini: {:?}",
            shapes
        );
    }

    #[cfg(feature = "normalized_dev")]
    #[test]
    fn test_detect_selfhost_if_sum_p3_shape() {
        use crate::mir::join_ir::normalized::fixtures::build_selfhost_if_sum_p3_structured_for_normalized_dev;

        let module = build_selfhost_if_sum_p3_structured_for_normalized_dev();
        let shapes = detect_shapes(&module);

        assert!(
            shapes.contains(&NormalizedDevShape::SelfhostIfSumP3),
            "selfhost_if_sum_p3 shape missing: {:?}",
            shapes
        );
        assert!(
            !shapes
                .iter()
                .any(|s| matches!(s, NormalizedDevShape::Pattern3IfSumMinimal)),
            "selfhost_if_sum_p3 should not rely on canonical P3 minimal detection: {:?}",
            shapes
        );
    }

    #[cfg(feature = "normalized_dev")]
    #[test]
    fn test_detect_selfhost_if_sum_p3_ext_shape() {
        use crate::mir::join_ir::normalized::fixtures::build_selfhost_if_sum_p3_ext_structured_for_normalized_dev;

        let module = build_selfhost_if_sum_p3_ext_structured_for_normalized_dev();
        let shapes = detect_shapes(&module);

        assert!(
            shapes.contains(&NormalizedDevShape::SelfhostIfSumP3Ext),
            "selfhost_if_sum_p3_ext shape missing: {:?}",
            shapes
        );
        assert!(
            !shapes.iter().any(|s| matches!(
                s,
                NormalizedDevShape::Pattern3IfSumMinimal
                    | NormalizedDevShape::Pattern3IfSumMulti
                    | NormalizedDevShape::Pattern3IfSumJson
            )),
            "selfhost_if_sum_p3_ext should not rely on canonical P3 detection: {:?}",
            shapes
        );
    }

    #[cfg(feature = "normalized_dev")]
    #[test]
    fn test_detect_pattern4_continue_minimal_shape() {
        use crate::mir::join_ir::normalized::fixtures::build_pattern4_continue_min_structured_for_normalized_dev;

        let module = build_pattern4_continue_min_structured_for_normalized_dev();

        // Should detect Pattern4ContinueMinimal shape
        assert!(
            pattern4::is_pattern4_continue_minimal(&module),
            "pattern4_continue_minimal fixture should be detected"
        );

        let shapes = detect_shapes(&module);
        assert!(
            shapes.contains(&NormalizedDevShape::Pattern4ContinueMinimal),
            "detect_shapes() should include Pattern4ContinueMinimal, got: {:?}",
            shapes
        );
    }

    #[cfg(feature = "normalized_dev")]
    #[test]
    fn test_detect_pattern4_jsonparser_continue_shapes() {
        use crate::mir::join_ir::normalized::fixtures::{
            build_jsonparser_parse_array_continue_skip_ws_structured_for_normalized_dev,
            build_jsonparser_parse_object_continue_skip_ws_structured_for_normalized_dev,
        };

        let array = build_jsonparser_parse_array_continue_skip_ws_structured_for_normalized_dev();
        assert!(
            pattern4::is_jsonparser_parse_array_continue_skip_ws(&array),
            "array continue fixture should be detected"
        );
        let array_shapes = detect_shapes(&array);
        assert!(
            array_shapes.contains(&NormalizedDevShape::JsonparserParseArrayContinueSkipWs),
            "array continue shape missing, got {:?}",
            array_shapes
        );

        let object = build_jsonparser_parse_object_continue_skip_ws_structured_for_normalized_dev();
        assert!(
            pattern4::is_jsonparser_parse_object_continue_skip_ws(&object),
            "object continue fixture should be detected"
        );
        let object_shapes = detect_shapes(&object);
        assert!(
            object_shapes.contains(&NormalizedDevShape::JsonparserParseObjectContinueSkipWs),
            "object continue shape missing, got {:?}",
            object_shapes
        );
    }

    #[cfg(feature = "normalized_dev")]
    #[test]
    fn test_pattern4_detector_rejects_loop_with_return() {
        // Phase 89: Verify that Pattern4 detector does NOT match
        // modules with loop-internal return (continue + early return pattern)

        use crate::mir::join_ir::{JoinFuncId, JoinFunction, JoinModule};
        use crate::mir::ValueId;
        use std::collections::BTreeMap;

        // Minimal module with loop + continue + return
        // (this would be the ContinueReturn pattern, NOT Pattern4)
        let mut functions = BTreeMap::new();

        // Entry function
        let entry_func = JoinFunction {
            id: JoinFuncId::new(0),
            name: "loop_with_return_test".to_string(),
            params: vec![ValueId(0)],
            body: vec![
                JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Const {
                    dst: ValueId(1),
                    value: crate::mir::join_ir::ConstValue::Integer(0),
                }),
                JoinInst::Call {
                    func: JoinFuncId::new(1),
                    args: vec![ValueId(1), ValueId(1), ValueId(0)],
                    k_next: None,
                    dst: Some(ValueId(2)),
                },
                JoinInst::Ret {
                    value: Some(ValueId(2)),
                },
            ],
            exit_cont: None,
        };

        // loop_step function with TWO conditional Jumps (break + early return)
        let loop_step_func = JoinFunction {
            id: JoinFuncId::new(1),
            name: "loop_step".to_string(),
            params: vec![ValueId(0), ValueId(1), ValueId(2)],
            body: vec![
                // Compare for loop condition
                JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Compare {
                    dst: ValueId(10),
                    op: crate::mir::join_ir::CompareOp::Lt,
                    lhs: ValueId(0),
                    rhs: ValueId(2),
                }),
                JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Const {
                    dst: ValueId(11),
                    value: crate::mir::join_ir::ConstValue::Bool(false),
                }),
                JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Compare {
                    dst: ValueId(12),
                    op: crate::mir::join_ir::CompareOp::Eq,
                    lhs: ValueId(10),
                    rhs: ValueId(11),
                }),
                // First Jump: loop break
                JoinInst::Jump {
                    cont: JoinFuncId::new(2).as_cont(),
                    args: vec![ValueId(1)],
                    cond: Some(ValueId(12)),
                },
                // Compare for early return condition
                JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Compare {
                    dst: ValueId(20),
                    op: crate::mir::join_ir::CompareOp::Eq,
                    lhs: ValueId(0),
                    rhs: ValueId(2),
                }),
                // Second Jump: early return (THIS MAKES IT NOT PATTERN4)
                JoinInst::Jump {
                    cont: JoinFuncId::new(2).as_cont(),
                    args: vec![ValueId(1)],
                    cond: Some(ValueId(20)),
                },
                // Select (continue's core)
                JoinInst::Select {
                    dst: ValueId(30),
                    cond: ValueId(20),
                    then_val: ValueId(1),
                    else_val: ValueId(1),
                    type_hint: None,
                },
                // Tail call (loop back)
                JoinInst::Call {
                    func: JoinFuncId::new(1),
                    args: vec![ValueId(0), ValueId(30), ValueId(2)],
                    k_next: None,
                    dst: Some(ValueId(40)),
                },
                JoinInst::Ret {
                    value: Some(ValueId(40)),
                },
            ],
            exit_cont: None,
        };

        // k_exit function
        let k_exit_func = JoinFunction {
            id: JoinFuncId::new(2),
            name: "k_exit".to_string(),
            params: vec![ValueId(0)],
            body: vec![JoinInst::Ret {
                value: Some(ValueId(0)),
            }],
            exit_cont: None,
        };

        functions.insert(JoinFuncId::new(0), entry_func);
        functions.insert(JoinFuncId::new(1), loop_step_func);
        functions.insert(JoinFuncId::new(2), k_exit_func);

        let module = JoinModule {
            functions,
            entry: Some(JoinFuncId::new(0)),
            phase: crate::mir::join_ir::JoinIrPhase::Structured,
        };

        // Phase 89: This should NOT be detected as Pattern4ContinueMinimal
        // because it has TWO conditional Jumps (loop break + early return)
        assert!(
            !pattern4::is_pattern4_continue_minimal(&module),
            "Module with loop-internal return should NOT match Pattern4ContinueMinimal"
        );

        let shapes = detect_shapes(&module);
        assert!(
            !shapes.contains(&NormalizedDevShape::Pattern4ContinueMinimal),
            "Pattern4ContinueMinimal should not be detected for loop with return, got: {:?}",
            shapes
        );
    }
}
