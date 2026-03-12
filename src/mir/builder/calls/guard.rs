/*!
 * CalleeGuardBox - 構造ガード専用箱
 *
 * 箱理論の実践:
 * - 箱にする: 構造ガード機能を1箱に集約
 * - 境界を作る: 静的Box/ランタイムBoxの混線を構造的に防ぐ
 * - Fail-Fast: フォールバックより明示的エラー
 *
 * 責務:
 * - Callee::Methodの静的Box/ランタイムBox混線検出・正規化
 * - me-call判定（receiver型==box_name）
 * - receiver実体化の保証
 */

use crate::mir::definitions::call_unified::CalleeBoxKind;
use crate::mir::{Callee, MirType, ValueId};
use crate::runtime::core_method_aliases::canonical_method_name;
use std::collections::BTreeMap;

/// 構造ガード専用箱
///
/// 箱理論:
/// - 単一責務: Calleeの構造検証・正規化のみ
/// - 状態最小: value_typesのみ保持（型情報参照用）
/// - ピュア関数的: 入力Callee → 検証・変換 → 出力Callee
pub struct CalleeGuardBox<'a> {
    /// 型情報マップ（ValueId → MirType）
    value_types: &'a BTreeMap<ValueId, MirType>,
}

impl<'a> CalleeGuardBox<'a> {
    /// 新しいCalleeGuardBoxを作成
    pub fn new(value_types: &'a BTreeMap<ValueId, MirType>) -> Self {
        CalleeGuardBox { value_types }
    }

    /// 静的Box/ランタイムBox混線を検出・正規化
    ///
    /// 箱理論の「境界を作る」原則:
    /// - Stage-B/Stage-1コンパイラBoxとランタイムDataBoxを構造的に分離
    ///
    /// ロジック:
    /// 1. box_kind==StaticCompiler かつ receiver型==同一Box名
    ///    → me-call判定、静的メソッド降下に委ねる（そのまま通す）
    /// 2. box_kind==StaticCompiler かつ receiver型==異なるランタイムBox
    ///    → 正規化（MapBox/ArrayBoxなど実際のruntime型に修正）
    /// 3. それ以外 → そのまま通す
    ///
    /// 実例:
    /// - StageBArgsBox.resolve_src内のargs.get(i)がStage1UsingResolverBox.getに
    ///   化けるのを防ぐ（args型はMapBox/ArrayBox → 正規化）
    pub fn apply_static_runtime_guard(&self, callee: Callee) -> Result<Callee, String> {
        if let Callee::Method {
            ref box_name,
            ref method,
            receiver: Some(recv),
            certainty,
            box_kind,
        } = callee
        {
            // Only apply guard if box_kind is StaticCompiler
            if box_kind == CalleeBoxKind::StaticCompiler {
                let trace_enabled = crate::config::env::builder_callee_resolve_trace();

                // Check if receiver has a Box type
                if let Some(MirType::Box(receiver_box)) = self.value_types.get(&recv) {
                    // If receiver box type matches the static box name, this is a me-call
                    // Let it through for static method lowering (don't normalize)
                    if receiver_box == box_name {
                        if trace_enabled {
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0.log.debug("[static-runtime-guard] ME-CALL detected:");
                            ring0.log.debug(&format!(
                                "  {}.{} with receiver type: {} (same as box_name)",
                                box_name, method, receiver_box
                            ));
                            ring0.log.debug("  → Allowing for static method lowering");
                        }
                        return Ok(callee); // Pass through unchanged
                    }

                    // Otherwise, this is a true mix-up: runtime box with static box name
                    // Normalize to the runtime box type
                    if trace_enabled {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug("[static-runtime-guard] CORRECTING mix-up:");
                        ring0.log.debug(&format!(
                            "  Original: {}.{} (box_kind=StaticCompiler)",
                            box_name, method
                        ));
                        ring0.log.debug(&format!(
                            "  Receiver %{} has runtime type: {}",
                            recv.0, receiver_box
                        ));
                        ring0
                            .log
                            .debug(&format!("  Normalized: {}.{}", receiver_box, method));
                    }

                    return Ok(Callee::Method {
                        box_name: receiver_box.clone(),
                        method: method.clone(),
                        receiver: Some(recv),
                        certainty,
                        box_kind: CalleeBoxKind::RuntimeData, // Switch to runtime
                    });
                } else {
                    // StaticCompiler Method but receiver has NO Box type
                    // 🎯 Phase 3-C: Normalize StaticCompiler string methods to StringBox
                    //
                    // Common string methods that should be routed to StringBox:
                    // length, substring, charAt, indexOf, etc.
                    let canonical = canonical_method_name(method);
                    let is_string_method = matches!(
                        canonical,
                        "length"
                            | "substring"
                            | "charAt"
                            | "indexOf"
                            | "lastIndexOf"
                            | "toUpper"
                            | "toLower"
                            | "trim"
                            | "split"
                    );

                    if is_string_method {
                        if trace_enabled {
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0.log.debug("[static-runtime-guard] StaticCompiler string method normalization:");
                            ring0.log.debug(&format!(
                                "  {}.{} receiver %{} has no type info",
                                box_name, method, recv.0
                            ));
                            ring0
                                .log
                                .debug(&format!("  → Normalize: StringBox.{}", method));
                        }

                        // Normalize to StringBox method call
                        return Ok(Callee::Method {
                            box_name: "StringBox".to_string(),
                            method: method.clone(),
                            receiver: Some(recv),
                            certainty,
                            box_kind: CalleeBoxKind::RuntimeData,
                        });
                    } else {
                        // Non-string methods with StaticCompiler box_kind:
                        // Trust methodize - if it explicitly created a singleton receiver
                        // for a static box call, pass through unchanged.
                        // The VM's static box singleton path will handle it.
                        //
                        // 🎯 Phase 173-B Fix: Don't fallback to Global for using-imported static boxes
                        // Previously this converted to Global("JsonParserBox.parse") which failed
                        // because the function table expected "JsonParserBox.parse/1" with arity.
                        if trace_enabled {
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0
                                .log
                                .debug("[static-runtime-guard] StaticCompiler trusting methodize:");
                            ring0.log.debug(&format!(
                                "  {}.{} receiver %{} - passing through as static box call",
                                box_name, method, recv.0
                            ));
                        }

                        // Pass through unchanged - VM will handle via static_box_decls
                        return Ok(callee);
                    }
                }
            }
        }

        // No guard needed, return as-is
        Ok(callee)
    }

    /// receiver型の検証（ヘルパー）
    ///
    /// 指定されたreceiverがBox型を持っているか確認
    #[allow(dead_code)]
    pub fn has_box_type(&self, receiver: ValueId) -> bool {
        matches!(self.value_types.get(&receiver), Some(MirType::Box(_)))
    }

    /// receiver型の取得（ヘルパー）
    ///
    /// 指定されたreceiverのBox型名を返す（存在しない場合はNone）
    #[allow(dead_code)]
    pub fn get_box_type(&self, receiver: ValueId) -> Option<&String> {
        match self.value_types.get(&receiver) {
            Some(MirType::Box(box_name)) => Some(box_name),
            _ => None,
        }
    }

    /// me-call判定
    ///
    /// box_name と receiver型が一致するか判定
    /// （静的メソッド呼び出しの検出用）
    #[allow(dead_code)]
    pub fn is_me_call(&self, box_name: &str, receiver: ValueId) -> bool {
        match self.get_box_type(receiver) {
            Some(recv_box) => recv_box == box_name,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_me_call_detection() {
        let mut value_types = BTreeMap::new();
        value_types.insert(ValueId::new(10), MirType::Box("StageBArgsBox".to_string()));

        let guard = CalleeGuardBox::new(&value_types);

        // Same box name → me-call
        assert!(guard.is_me_call("StageBArgsBox", ValueId::new(10)));

        // Different box name → not me-call
        assert!(!guard.is_me_call("Stage1UsingResolverBox", ValueId::new(10)));

        // No type info → not me-call
        assert!(!guard.is_me_call("StageBArgsBox", ValueId::new(999)));
    }

    #[test]
    fn test_static_runtime_guard_me_call() {
        use crate::mir::definitions::call_unified::TypeCertainty;

        let mut value_types = BTreeMap::new();
        value_types.insert(ValueId::new(10), MirType::Box("StageBArgsBox".to_string()));

        let guard = CalleeGuardBox::new(&value_types);

        let callee = Callee::Method {
            box_name: "StageBArgsBox".to_string(),
            method: "process".to_string(),
            receiver: Some(ValueId::new(10)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::StaticCompiler,
        };

        // me-call → should pass through unchanged
        let result = guard.apply_static_runtime_guard(callee.clone()).unwrap();
        assert_eq!(result, callee);
    }

    #[test]
    fn test_static_runtime_guard_normalization() {
        use crate::mir::definitions::call_unified::TypeCertainty;

        let mut value_types = BTreeMap::new();
        value_types.insert(ValueId::new(10), MirType::Box("MapBox".to_string()));

        let guard = CalleeGuardBox::new(&value_types);

        let callee = Callee::Method {
            box_name: "Stage1UsingResolverBox".to_string(),
            method: "get".to_string(),
            receiver: Some(ValueId::new(10)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::StaticCompiler,
        };

        // Mix-up → should normalize to MapBox
        let result = guard.apply_static_runtime_guard(callee).unwrap();
        match result {
            Callee::Method {
                box_name, box_kind, ..
            } => {
                assert_eq!(box_name, "MapBox");
                assert_eq!(box_kind, CalleeBoxKind::RuntimeData);
            }
            _ => panic!("Expected Method callee"),
        }
    }
}
