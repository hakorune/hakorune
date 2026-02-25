//! Minimal Normalized JoinIR model (Phase 26-H.B).
//!
//! テスト専用の極小サブセット。Pattern1 の while だけを Structured → Normalized に
//! 変換して遊ぶための足場だよ。本線の Structured→MIR 経路には影響しない。

use std::collections::BTreeMap;

use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinIrPhase,
    JoinModule, UnaryOp,
};
use crate::mir::ValueId;
#[cfg(feature = "normalized_dev")]
use std::collections::HashMap;
#[cfg(feature = "normalized_dev")]
use std::panic::{catch_unwind, AssertUnwindSafe};

#[cfg(feature = "normalized_dev")]
pub mod dev_env;
#[cfg(feature = "normalized_dev")]
pub mod dev_fixtures;
#[cfg(feature = "normalized_dev")]
pub mod fixtures;
#[cfg(feature = "normalized_dev")]
pub mod loop_step_inspector;
#[cfg(feature = "normalized_dev")]
pub mod shape_guard;

// Phase 286C: Box modularization - core normalization boxes
pub mod pattern1_normalizer;
pub mod pattern2_normalizer;
pub mod env_layout_builder;

#[cfg(feature = "normalized_dev")]
use crate::mir::join_ir::normalized::shape_guard::NormalizedDevShape;

/// 環境レイアウト（最小）。
#[derive(Debug, Clone)]
pub struct EnvLayout {
    pub id: u32,
    pub fields: Vec<EnvField>,
}

#[derive(Debug, Clone)]
pub struct EnvField {
    pub name: String,
    pub ty: Option<crate::mir::MirType>,
    pub value_id: Option<ValueId>,
}

/// 正規化済み関数 ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JpFuncId(pub u32);

/// 正規化済み関数（Kont 兼用、is_kont で区別）。
#[derive(Debug, Clone)]
pub struct JpFunction {
    pub id: JpFuncId,
    pub name: String,
    pub env_layout: Option<u32>,
    pub body: Vec<JpInst>,
    pub is_kont: bool,
}

/// 正規化済み命令（最小セット）。
#[derive(Debug, Clone)]
pub enum JpInst {
    Let {
        dst: ValueId,
        op: JpOp,
        args: Vec<ValueId>,
    },
    EnvLoad {
        dst: ValueId,
        env: ValueId,
        field: usize,
    },
    EnvStore {
        env: ValueId,
        field: usize,
        src: ValueId,
    },
    TailCallFn {
        target: JpFuncId,
        env: Vec<ValueId>,
    },
    TailCallKont {
        target: JpFuncId,
        env: Vec<ValueId>,
    },
    If {
        cond: ValueId,
        then_target: JpFuncId,
        else_target: JpFuncId,
        env: Vec<ValueId>,
    },
}

/// 演算（Let 用の最小サブセット）。
#[derive(Debug, Clone)]
pub enum JpOp {
    Const(ConstValue),
    BinOp(BinOpKind),
    Unary(UnaryOp),
    Compare(CompareOp),
    BoxCall {
        box_name: String,
        method: String,
    },
    /// 三項演算子（cond ? then : else）
    Select,
}

/// Normalized JoinIR モジュール（テスト専用）。
#[derive(Debug, Clone)]
pub struct NormalizedModule {
    pub functions: BTreeMap<JpFuncId, JpFunction>,
    pub entry: Option<JpFuncId>,
    pub env_layouts: Vec<EnvLayout>,
    pub phase: JoinIrPhase,
    /// Structured に戻すためのスナップショット（テスト専用）。
    pub structured_backup: Option<JoinModule>,
}

impl NormalizedModule {
    pub fn to_structured(&self) -> Option<JoinModule> {
        self.structured_backup.clone()
    }
}

// Phase 286C: Re-export from pattern1_normalizer box
#[cfg(feature = "normalized_dev")]
fn verify_normalized_pattern1(module: &NormalizedModule) -> Result<(), String> {
    pattern1_normalizer::Pattern1Normalizer::verify(module)
}

// Phase 286C: Re-export from pattern1_normalizer box
/// Pattern1 専用: Normalized → Structured への簡易逆変換。
pub fn normalized_pattern1_to_structured(norm: &NormalizedModule) -> JoinModule {
    pattern1_normalizer::Pattern1Normalizer::to_structured(norm)
}

// Phase 286C: Re-export from pattern2_normalizer box
/// Pattern2 専用のミニ変換（最小サブセット: ループ変数1つ + break、acc などの LoopState を 1 個まで＋ホスト 1 個まで）。
///
/// 制約:
/// - structured.phase は Structured であること
/// - main/loop_step/k_exit の 3 関数構成（joinir_min_loop 相当）
pub fn normalize_pattern2_minimal(structured: &JoinModule) -> NormalizedModule {
    pattern2_normalizer::Pattern2Normalizer::normalize(structured)
}

#[cfg(feature = "normalized_dev")]
fn normalize_pattern2_shape(
    structured: &JoinModule,
    target_shape: NormalizedDevShape,
) -> Result<NormalizedModule, String> {
    if !structured.is_structured() {
        return Err("[normalize_p2] Not structured JoinIR".to_string());
    }

    let shapes = shape_guard::supported_shapes(structured);
    if !shapes.contains(&target_shape) {
        return Err(format!(
            "[normalize_p2] shape mismatch: expected {:?}, got {:?}",
            target_shape, shapes
        ));
    }

    Ok(normalize_pattern2_minimal(structured))
}

/// Phase 50: selfhost token-scan P2 を Normalized に載せる（dev-only）。
#[cfg(feature = "normalized_dev")]
pub fn normalize_selfhost_token_scan_p2(
    structured: &JoinModule,
) -> Result<NormalizedModule, String> {
    normalize_pattern2_shape(structured, NormalizedDevShape::SelfhostTokenScanP2)
}

/// Phase 51: selfhost token-scan P2（accum 拡張）を Normalized に載せる（dev-only）。
#[cfg(feature = "normalized_dev")]
pub fn normalize_selfhost_token_scan_p2_accum(
    structured: &JoinModule,
) -> Result<NormalizedModule, String> {
    normalize_pattern2_shape(structured, NormalizedDevShape::SelfhostTokenScanP2Accum)
}

/// Phase 47-A/B: Normalize Pattern3 if-sum shapes to Normalized JoinIR
#[cfg(feature = "normalized_dev")]
pub fn normalize_pattern3_if_sum_minimal(
    structured: &JoinModule,
) -> Result<NormalizedModule, String> {
    normalize_pattern3_if_sum_shape(structured, NormalizedDevShape::Pattern3IfSumMinimal)
}

/// Phase 47-B: Normalize Pattern3 if-sum multi-carrier (sum+count) shape.
#[cfg(feature = "normalized_dev")]
pub fn normalize_pattern3_if_sum_multi_minimal(
    structured: &JoinModule,
) -> Result<NormalizedModule, String> {
    normalize_pattern3_if_sum_shape(structured, NormalizedDevShape::Pattern3IfSumMulti)
}

/// Phase 47-B: Normalize JsonParser if-sum (mini) shape.
#[cfg(feature = "normalized_dev")]
pub fn normalize_pattern3_if_sum_json_minimal(
    structured: &JoinModule,
) -> Result<NormalizedModule, String> {
    normalize_pattern3_if_sum_shape(structured, NormalizedDevShape::Pattern3IfSumJson)
}

/// Phase 50: selfhost if-sum P3 を Normalized に載せる（dev-only）。
#[cfg(feature = "normalized_dev")]
pub fn normalize_selfhost_if_sum_p3(structured: &JoinModule) -> Result<NormalizedModule, String> {
    normalize_pattern3_if_sum_shape(structured, NormalizedDevShape::SelfhostIfSumP3)
}

/// Phase 51: selfhost if-sum P3（ext 拡張）を Normalized に載せる（dev-only）。
#[cfg(feature = "normalized_dev")]
pub fn normalize_selfhost_if_sum_p3_ext(
    structured: &JoinModule,
) -> Result<NormalizedModule, String> {
    normalize_pattern3_if_sum_shape(structured, NormalizedDevShape::SelfhostIfSumP3Ext)
}

#[cfg(feature = "normalized_dev")]
fn normalize_pattern3_if_sum_shape(
    structured: &JoinModule,
    target_shape: NormalizedDevShape,
) -> Result<NormalizedModule, String> {
    if !structured.is_structured() {
        return Err("[normalize_p3] Not structured JoinIR".to_string());
    }

    let shapes = shape_guard::supported_shapes(structured);
    if !shapes.contains(&target_shape) {
        return Err(format!(
            "[normalize_p3] shape mismatch: expected {:?}, got {:?}",
            target_shape, shapes
        ));
    }

    // Phase 47-B: P3 if-sum は既存の P2 ミニ正規化器で十分に表現できる
    // （Select/If/Compare/BinOp をそのまま JpInst に写す）。
    Ok(normalize_pattern2_minimal(structured))
}

/// Phase 48-A: Pattern4 (continue) minimal ループの正規化。
///
/// ガード:
/// - structured.phase は Structured であること
/// - 対象は Pattern4ContinueMinimal のシンプル continue パターン
///
/// 実装方針:
/// - Phase 48-A minimal: P2 正規化に委譲（P4 は P2 の逆制御フローなので同じインフラ利用可）
/// - TODO (Phase 48-B): P4 固有の正規化実装
///   - EnvLayout for i, count carriers
///   - HeaderCond → ContinueCheck → Updates → Tail step sequence
///   - continue = early TailCallFn (skip Updates)
#[cfg(feature = "normalized_dev")]
pub fn normalize_pattern4_continue_minimal(
    structured: &JoinModule,
) -> Result<NormalizedModule, String> {
    normalize_pattern4_continue_shape(structured, NormalizedDevShape::Pattern4ContinueMinimal)
}

/// Phase 48-B: JsonParser _parse_array continue skip_ws を Normalized に載せる（dev-only）。
#[cfg(feature = "normalized_dev")]
pub fn normalize_jsonparser_parse_array_continue_skip_ws(
    structured: &JoinModule,
) -> Result<NormalizedModule, String> {
    normalize_pattern4_continue_shape(
        structured,
        NormalizedDevShape::JsonparserParseArrayContinueSkipWs,
    )
}

/// Phase 48-B: JsonParser _parse_object continue skip_ws を Normalized に載せる（dev-only）。
#[cfg(feature = "normalized_dev")]
pub fn normalize_jsonparser_parse_object_continue_skip_ws(
    structured: &JoinModule,
) -> Result<NormalizedModule, String> {
    normalize_pattern4_continue_shape(
        structured,
        NormalizedDevShape::JsonparserParseObjectContinueSkipWs,
    )
}

#[cfg(feature = "normalized_dev")]
fn normalize_pattern4_continue_shape(
    structured: &JoinModule,
    target_shape: NormalizedDevShape,
) -> Result<NormalizedModule, String> {
    if !structured.is_structured() {
        return Err("[normalize_p4] Not structured JoinIR".to_string());
    }

    // Use shape detection to verify P4 shape
    let shapes = shape_guard::supported_shapes(structured);
    if !shapes.contains(&target_shape) {
        return Err(format!(
            "[normalize_p4] shape mismatch: expected {:?}, got {:?}",
            target_shape, shapes
        ));
    }

    // Phase 48-B: reuse Pattern2 minimal normalizer (continue is early tail-call).
    Ok(normalize_pattern2_minimal(structured))
}

// Phase 286C: Re-export from pattern2_normalizer box
/// Pattern2 専用: Normalized → Structured への簡易逆変換。
pub fn normalized_pattern2_to_structured(norm: &NormalizedModule) -> JoinModule {
    pattern2_normalizer::Pattern2Normalizer::to_structured(norm)
}

// Phase 286C: Re-export from pattern2_normalizer box
#[cfg(feature = "normalized_dev")]
fn verify_normalized_pattern2(
    module: &NormalizedModule,
    max_env_fields: usize,
) -> Result<(), String> {
    pattern2_normalizer::Pattern2Normalizer::verify(module, max_env_fields)
}

// Phase 286C: Re-export from pattern1_normalizer box
/// Pattern1 専用のミニ変換。
///
/// 制約:
/// - structured.phase は Structured であること
/// - 対象は Pattern1 のシンプル while（break/continue なし）
pub fn normalize_pattern1_minimal(structured: &JoinModule) -> NormalizedModule {
    pattern1_normalizer::Pattern1Normalizer::normalize(structured)
}

/// Dev helper: Structured → Normalized → Structured roundtrip (Pattern1/2 minis only).
#[cfg(feature = "normalized_dev")]
pub(crate) fn normalized_dev_roundtrip_structured(
    module: &JoinModule,
) -> Result<JoinModule, String> {
    if !module.is_structured() {
        return Err("[joinir/normalized-dev] expected Structured JoinModule".to_string());
    }

    let shapes = shape_guard::supported_shapes(module);
    if shapes.is_empty() {
        return Err(
            "[joinir/normalized-dev] module shape is not supported by normalized_dev".into(),
        );
    }

    let debug = dev_env::normalized_dev_logs_enabled() && crate::config::env::joinir_dev_enabled();

    for shape in shapes {
        if debug {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[joinir/normalized-dev/roundtrip] attempting {:?} normalization",
                shape
            ));
        }

        let attempt = match shape {
            NormalizedDevShape::Pattern1Mini => catch_unwind(AssertUnwindSafe(|| {
                let norm = normalize_pattern1_minimal(module);
                normalized_pattern1_to_structured(&norm)
            })),
            NormalizedDevShape::Pattern2Mini
            | NormalizedDevShape::JsonparserSkipWsMini
            | NormalizedDevShape::JsonparserSkipWsReal
            | NormalizedDevShape::JsonparserAtoiMini
            | NormalizedDevShape::JsonparserAtoiReal
            | NormalizedDevShape::JsonparserParseNumberReal => {
                catch_unwind(AssertUnwindSafe(|| {
                    let norm = normalize_pattern2_minimal(module);
                    normalized_pattern2_to_structured(&norm)
                }))
            }
            NormalizedDevShape::SelfhostTokenScanP2 => catch_unwind(AssertUnwindSafe(|| {
                let norm = normalize_selfhost_token_scan_p2(module)
                    .expect("selfhost P2 normalization failed");
                normalized_pattern2_to_structured(&norm)
            })),
            NormalizedDevShape::SelfhostTokenScanP2Accum => catch_unwind(AssertUnwindSafe(|| {
                let norm = normalize_selfhost_token_scan_p2_accum(module)
                    .expect("selfhost P2 accum normalization failed");
                normalized_pattern2_to_structured(&norm)
            })),
            // Phase 47-A: P3 minimal (delegates to P2 for now, but uses proper guard)
            NormalizedDevShape::Pattern3IfSumMinimal => catch_unwind(AssertUnwindSafe(|| {
                let norm =
                    normalize_pattern3_if_sum_minimal(module).expect("P3 normalization failed");
                normalized_pattern2_to_structured(&norm)
            })),
            NormalizedDevShape::Pattern3IfSumMulti => catch_unwind(AssertUnwindSafe(|| {
                let norm = normalize_pattern3_if_sum_multi_minimal(module)
                    .expect("P3 multi normalization failed");
                normalized_pattern2_to_structured(&norm)
            })),
            NormalizedDevShape::Pattern3IfSumJson => catch_unwind(AssertUnwindSafe(|| {
                let norm = normalize_pattern3_if_sum_json_minimal(module)
                    .expect("P3 json normalization failed");
                normalized_pattern2_to_structured(&norm)
            })),
            NormalizedDevShape::SelfhostIfSumP3 => catch_unwind(AssertUnwindSafe(|| {
                let norm =
                    normalize_selfhost_if_sum_p3(module).expect("selfhost P3 normalization failed");
                normalized_pattern2_to_structured(&norm)
            })),
            NormalizedDevShape::SelfhostIfSumP3Ext => catch_unwind(AssertUnwindSafe(|| {
                let norm = normalize_selfhost_if_sum_p3_ext(module)
                    .expect("selfhost P3 ext normalization failed");
                normalized_pattern2_to_structured(&norm)
            })),
            // Phase 53: selfhost P2/P3 practical variations (delegate to existing normalizers)
            NormalizedDevShape::SelfhostArgsParseP2 => catch_unwind(AssertUnwindSafe(|| {
                let norm = normalize_pattern2_minimal(module);
                normalized_pattern2_to_structured(&norm)
            })),
            NormalizedDevShape::SelfhostStmtCountP3 => catch_unwind(AssertUnwindSafe(|| {
                let norm = normalize_selfhost_if_sum_p3_ext(module)
                    .expect("selfhost stmt_count P3 normalization failed");
                normalized_pattern2_to_structured(&norm)
            })),
            // Phase 54: selfhost P2/P3 shape growth (delegate to existing normalizers)
            NormalizedDevShape::SelfhostVerifySchemaP2 => catch_unwind(AssertUnwindSafe(|| {
                let norm = normalize_pattern2_minimal(module);
                normalized_pattern2_to_structured(&norm)
            })),
            NormalizedDevShape::SelfhostDetectFormatP3 => catch_unwind(AssertUnwindSafe(|| {
                let norm = normalize_selfhost_if_sum_p3_ext(module)
                    .expect("selfhost detect_format P3 normalization failed");
                normalized_pattern2_to_structured(&norm)
            })),
            // Phase 48-A: P4 minimal (delegates to P2 for now, but uses proper guard)
            NormalizedDevShape::Pattern4ContinueMinimal => catch_unwind(AssertUnwindSafe(|| {
                let norm =
                    normalize_pattern4_continue_minimal(module).expect("P4 normalization failed");
                normalized_pattern2_to_structured(&norm)
            })),
            NormalizedDevShape::JsonparserParseArrayContinueSkipWs => {
                catch_unwind(AssertUnwindSafe(|| {
                    let norm = normalize_jsonparser_parse_array_continue_skip_ws(module)
                        .expect("P4 array normalization failed");
                    normalized_pattern2_to_structured(&norm)
                }))
            }
            NormalizedDevShape::JsonparserParseObjectContinueSkipWs => {
                catch_unwind(AssertUnwindSafe(|| {
                    let norm = normalize_jsonparser_parse_object_continue_skip_ws(module)
                        .expect("P4 object normalization failed");
                    normalized_pattern2_to_structured(&norm)
                }))
            }
            // Phase 89: Continue + Early Return pattern (dev-only, delegates to P2 for now)
            NormalizedDevShape::PatternContinueReturnMinimal => {
                catch_unwind(AssertUnwindSafe(|| {
                    let norm = normalize_pattern2_minimal(module);
                    normalized_pattern2_to_structured(&norm)
                }))
            }
            // Phase 90: Parse String Composite pattern (dev-only, delegates to P2 for now)
            NormalizedDevShape::ParseStringCompositeMinimal => {
                catch_unwind(AssertUnwindSafe(|| {
                    let norm = normalize_pattern2_minimal(module);
                    normalized_pattern2_to_structured(&norm)
                }))
            }
        };

        match attempt {
            Ok(structured) => {
                if debug {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[joinir/normalized-dev/roundtrip] {:?} normalization succeeded (functions={})",
                        shape,
                        structured.functions.len()
                    ));
                }
                return Ok(structured);
            }
            Err(_) => {
                if debug {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[joinir/normalized-dev/roundtrip] {:?} normalization failed (unsupported)",
                        shape
                    ));
                }
            }
        }
    }

    Err("[joinir/normalized-dev] all normalization attempts failed".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::{JoinContId, JoinFuncId, JoinFunction, JoinInst, MirLikeInst};

    fn build_structured_pattern1() -> JoinModule {
        let mut module = JoinModule::new();
        let mut loop_fn = JoinFunction::new(
            JoinFuncId::new(1),
            "loop_step".to_string(),
            vec![ValueId(10)],
        );

        loop_fn.body.push(JoinInst::Compute(MirLikeInst::Const {
            dst: ValueId(11),
            value: ConstValue::Integer(0),
        }));
        loop_fn.body.push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: ValueId(12),
            op: BinOpKind::Add,
            lhs: ValueId(10),
            rhs: ValueId(11),
        }));
        loop_fn.body.push(JoinInst::Jump {
            cont: JoinContId(2),
            args: vec![ValueId(12)],
            cond: Some(ValueId(12)), // dummy
        });

        let mut k_exit =
            JoinFunction::new(JoinFuncId::new(2), "k_exit".to_string(), vec![ValueId(12)]);
        k_exit.body.push(JoinInst::Ret {
            value: Some(ValueId(12)),
        });

        module.entry = Some(loop_fn.id);
        module.add_function(loop_fn);
        module.add_function(k_exit);
        module
    }

    #[test]
    fn normalized_pattern1_minimal_smoke() {
        let structured = build_structured_pattern1();
        let normalized = normalize_pattern1_minimal(&structured);
        assert_eq!(normalized.phase, JoinIrPhase::Normalized);
        assert!(!normalized.env_layouts.is_empty());
        assert!(!normalized.functions.is_empty());

        #[cfg(feature = "normalized_dev")]
        {
            verify_normalized_pattern1(&normalized).expect("verifier should pass");
        }

        let restored = normalized.to_structured().expect("backup");
        assert!(restored.is_structured());
        assert_eq!(restored.functions.len(), structured.functions.len());
    }

    #[test]
    fn normalized_pattern1_roundtrip_structured_equivalent() {
        let structured = build_structured_pattern1();
        let normalized = normalize_pattern1_minimal(&structured);
        let reconstructed = normalized_pattern1_to_structured(&normalized);

        assert!(reconstructed.is_structured());
        assert_eq!(reconstructed.functions.len(), structured.functions.len());
    }
}
