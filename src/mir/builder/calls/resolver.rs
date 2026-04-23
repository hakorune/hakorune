/*!
 * CalleeResolverBox - Callee解決専用箱
 *
 * 箱理論の実践:
 * - 箱にする: Callee解決ロジックを1箱に集約
 * - 境界を作る: 型情報を保持し、効率的な解決を実現
 * - 状態最小: 型情報参照のみ（変更なし）
 *
 * 責務:
 * - CallTarget → Callee への解決
 * - Box種別分類（StaticCompiler/RuntimeData/UserDefined）
 * - Call引数検証
 */

use super::method_resolution;
use crate::mir::builder::type_registry::TypeRegistry;
use crate::mir::builder::CallTarget;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{Callee, MirType, ValueId};
use std::collections::BTreeMap;

/// Callee解決専用箱
///
/// 箱理論:
/// - 単一責務: CallTarget → Callee の型安全な解決のみ
/// - 状態保持: 型情報参照を保持して効率化
/// - ピュア解決器: 入力CallTarget → 解決・検証 → 出力Callee
pub struct CalleeResolverBox<'a> {
    /// 変数のnewbox起源マップ（ValueId → Box名）
    value_origin_newbox: &'a BTreeMap<ValueId, String>,
    /// 型情報マップ（ValueId → MirType）
    value_types: &'a BTreeMap<ValueId, MirType>,
    /// 型レジストリ（オプショナル）
    type_registry: Option<&'a TypeRegistry>,
}

impl<'a> CalleeResolverBox<'a> {
    /// 新しいCalleeResolverBoxを作成
    pub fn new(
        value_origin_newbox: &'a BTreeMap<ValueId, String>,
        value_types: &'a BTreeMap<ValueId, MirType>,
        type_registry: Option<&'a TypeRegistry>,
    ) -> Self {
        CalleeResolverBox {
            value_origin_newbox,
            value_types,
            type_registry,
        }
    }

    /// CallTarget → Callee への型安全な解決
    ///
    /// 箱理論の「境界を作る」原則:
    /// - Global/Method/Constructor/Extern/Value/Closureを明確に分類
    /// - 型情報を活用してbox_nameとbox_kindを決定
    ///
    /// 🎯 TypeRegistry対応: NYASH_USE_TYPE_REGISTRY=1 で registry 優先
    pub fn resolve(&self, target: CallTarget) -> Result<Callee, String> {
        let use_registry = crate::config::env::builder_use_type_registry();

        match target {
            CallTarget::Global(name) => {
                // Prefer explicit categories; otherwise treat as module-global function
                if method_resolution::is_builtin_function(&name) {
                    Ok(Callee::Global(name))
                } else if method_resolution::is_extern_function(&name) {
                    Ok(Callee::Extern(name))
                } else {
                    // Module-local or static lowered function (e.g., "Box.method/N")
                    Ok(Callee::Global(name))
                }
            }

            CallTarget::Method {
                box_type,
                method,
                receiver,
            } => {
                // 🔍 Debug: trace box_name resolution
                let trace_enabled = crate::config::env::builder_callee_resolve_trace();
                if trace_enabled {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[callee-resolve] receiver=%{} method={}",
                        receiver.0, method
                    ));
                    ring0.log.debug(&format!(
                        "[callee-resolve]   explicit box_type: {:?}",
                        box_type
                    ));
                    ring0.log.debug(&format!(
                        "[callee-resolve]   use_registry: {}",
                        use_registry
                    ));
                }

                let inferred_box_type =
                    self.infer_box_type(receiver, box_type, trace_enabled, use_registry);

                // Certainty is Known when we have explicit origin or Box型の型情報を持つ場合
                let has_runtime_receiver_type = self
                    .value_types
                    .get(&receiver)
                    .map(|t| matches!(t, MirType::Box(_) | MirType::String | MirType::Array(_)))
                    .unwrap_or(false);
                let certainty = if self.value_origin_newbox.contains_key(&receiver)
                    || has_runtime_receiver_type
                {
                    TypeCertainty::Known
                } else {
                    TypeCertainty::Union
                };

                // Classify box kind to prevent static/runtime mixing
                let box_kind = self.classify_box_kind(&inferred_box_type);

                if trace_enabled {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[callee-resolve]   inferred_box_name: {}",
                        inferred_box_type
                    ));
                    ring0
                        .log
                        .debug(&format!("[callee-resolve]   box_kind: {:?}", box_kind));
                }

                Ok(Callee::Method {
                    box_name: inferred_box_type,
                    method,
                    receiver: Some(receiver),
                    certainty,
                    box_kind,
                })
            }

            CallTarget::Constructor(box_type) => Ok(Callee::Constructor { box_type }),

            CallTarget::Extern(name) => Ok(Callee::Extern(name)),

            CallTarget::Value(func_val) => Ok(Callee::Value(func_val)),

            CallTarget::Closure {
                params,
                captures,
                me_capture,
            } => Ok(Callee::Closure {
                params,
                captures,
                me_capture,
            }),
        }
    }

    /// Box種別の分類
    ///
    /// 箱理論の「箱にする」原則:
    /// - 静的コンパイラBox群を明示的に列挙（1箇所に集約）
    /// - ランタイムDataBox群を明示的に列挙
    /// - ユーザー定義Boxをデフォルト扱い
    pub fn classify_box_kind(&self, box_name: &str) -> CalleeBoxKind {
        // LoopSSA / Exit PHI analyzers are still resolver-local keep until the
        // broader classifier table is collapsed into one SSOT.
        match box_name {
            "BreakFinderBox" | "PhiInjectorBox" | "LoopSSA" => CalleeBoxKind::StaticCompiler,
            _ => super::call_unified::classify_box_kind(box_name),
        }
    }

    /// Call引数の検証
    ///
    /// 既知の関数/メソッドについてarity等を検証
    pub fn validate_args(&self, callee: &Callee, args: &[ValueId]) -> Result<(), String> {
        match callee {
            Callee::Global(name) => {
                // Check known global functions
                match name.as_str() {
                    "print" | "error" | "panic" => {
                        if args.is_empty() {
                            return Err(format!("{} requires at least one argument", name));
                        }
                    }
                    "exit" => {
                        if args.len() != 1 {
                            return Err(
                                "exit requires exactly one argument (exit code)".to_string()
                            );
                        }
                    }
                    _ => {} // Unknown functions pass through
                }
            }

            Callee::Method {
                box_name, method, ..
            } => {
                // Validate known methods
                match (box_name.as_str(), method.as_str()) {
                    ("ArrayBox", "get") | ("ArrayBox", "set") => {
                        if args.is_empty() {
                            return Err(format!("ArrayBox.{} requires an index", method));
                        }
                    }
                    _ => {} // Unknown methods pass through
                }
            }

            _ => {} // Other callee types don't have validation yet
        }

        Ok(())
    }

    /// Box型の推論（内部ヘルパー）
    ///
    /// 優先順位:
    /// 1. TypeRegistry（NYASH_USE_TYPE_REGISTRY=1の場合）
    /// 2. value_types（MirType::Box）
    /// 3. value_origin_newbox（起源情報）
    /// 4. "UnknownBox"（フォールバック）
    fn infer_box_type(
        &self,
        receiver: ValueId,
        explicit_box_type: Option<String>,
        trace_enabled: bool,
        use_registry: bool,
    ) -> String {
        explicit_box_type.unwrap_or_else(|| {
            // 🎯 TypeRegistry 対応: 優先して registry から推論
            if use_registry {
                if let Some(reg) = self.type_registry {
                    let inferred = reg.infer_class(receiver, None);
                    if trace_enabled {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0
                            .log
                            .debug(&format!("[callee-resolve]   from_registry: {}", inferred));
                        // トレースチェーン表示
                        let chain = reg.trace_origin(receiver);
                        if !chain.is_empty() {
                            ring0
                                .log
                                .debug(&format!("[callee-resolve]   trace_chain: {:?}", chain));
                        }
                    }
                    return inferred;
                }
            }

            // 従来: BTreeMap から推論（型情報を優先し、origin は補助とする）
            let from_type = self.value_types.get(&receiver).and_then(|t| match t {
                MirType::Box(box_name) => Some(box_name.clone()),
                MirType::String => Some("StringBox".to_string()),
                MirType::Array(_) => Some("ArrayBox".to_string()),
                _ => None,
            });
            let from_origin = self.value_origin_newbox.get(&receiver).cloned();

            if trace_enabled {
                let ring0 = crate::runtime::get_global_ring0();
                ring0
                    .log
                    .debug(&format!("[callee-resolve]   from_type: {:?}", from_type));
                ring0.log.debug(&format!(
                    "[callee-resolve]   from_origin: {:?}",
                    from_origin
                ));
            }

            // 型情報（MirType）がある場合はそれを優先し、無い場合のみ origin にフォールバックする。
            from_type.or(from_origin).unwrap_or_else(|| {
                if trace_enabled {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug("[callee-resolve]   FALLBACK: UnknownBox");
                }
                "UnknownBox".to_string()
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_static_compiler_boxes() {
        let value_origin = BTreeMap::new();
        let value_types = BTreeMap::new();
        let resolver = CalleeResolverBox::new(&value_origin, &value_types, None);

        // Stage-B boxes
        assert_eq!(
            resolver.classify_box_kind("StageBArgsBox"),
            CalleeBoxKind::StaticCompiler
        );
        assert_eq!(
            resolver.classify_box_kind("StageBDriverBox"),
            CalleeBoxKind::StaticCompiler
        );

        // Stage-1 boxes
        assert_eq!(
            resolver.classify_box_kind("Stage1UsingResolverBox"),
            CalleeBoxKind::StaticCompiler
        );

        // Parser boxes
        assert_eq!(
            resolver.classify_box_kind("ParserBox"),
            CalleeBoxKind::StaticCompiler
        );
    }

    #[test]
    fn test_classify_runtime_data_boxes() {
        let value_origin = BTreeMap::new();
        let value_types = BTreeMap::new();
        let resolver = CalleeResolverBox::new(&value_origin, &value_types, None);

        assert_eq!(
            resolver.classify_box_kind("MapBox"),
            CalleeBoxKind::RuntimeData
        );
        assert_eq!(
            resolver.classify_box_kind("ArrayBox"),
            CalleeBoxKind::RuntimeData
        );
        assert_eq!(
            resolver.classify_box_kind("StringBox"),
            CalleeBoxKind::RuntimeData
        );
        assert_eq!(
            resolver.classify_box_kind("UnknownBox"),
            CalleeBoxKind::RuntimeData
        );
    }

    #[test]
    fn test_classify_user_defined_boxes() {
        let value_origin = BTreeMap::new();
        let value_types = BTreeMap::new();
        let resolver = CalleeResolverBox::new(&value_origin, &value_types, None);

        assert_eq!(
            resolver.classify_box_kind("MyCustomBox"),
            CalleeBoxKind::UserDefined
        );
        assert_eq!(
            resolver.classify_box_kind("PersonBox"),
            CalleeBoxKind::UserDefined
        );
    }

    #[test]
    fn test_resolve_global() {
        let value_origin = BTreeMap::new();
        let value_types = BTreeMap::new();
        let resolver = CalleeResolverBox::new(&value_origin, &value_types, None);

        let target = CallTarget::Global("print".to_string());
        let result = resolver.resolve(target).unwrap();

        match result {
            Callee::Global(name) => assert_eq!(name, "print"),
            _ => panic!("Expected Global callee"),
        }
    }

    #[test]
    fn test_resolve_constructor() {
        let value_origin = BTreeMap::new();
        let value_types = BTreeMap::new();
        let resolver = CalleeResolverBox::new(&value_origin, &value_types, None);

        let target = CallTarget::Constructor("StringBox".to_string());
        let result = resolver.resolve(target).unwrap();

        match result {
            Callee::Constructor { box_type } => assert_eq!(box_type, "StringBox"),
            _ => panic!("Expected Constructor callee"),
        }
    }

    #[test]
    fn test_resolve_string_value_receiver_without_newbox_origin() {
        let value_origin = BTreeMap::new();
        let mut value_types = BTreeMap::new();
        value_types.insert(ValueId(1), MirType::String);
        let resolver = CalleeResolverBox::new(&value_origin, &value_types, None);

        let target = CallTarget::Method {
            box_type: None,
            method: "length".to_string(),
            receiver: ValueId(1),
        };
        let result = resolver.resolve(target).unwrap();

        match result {
            Callee::Method {
                box_name,
                certainty,
                box_kind,
                ..
            } => {
                assert_eq!(box_name, "StringBox");
                assert_eq!(certainty, TypeCertainty::Known);
                assert_eq!(box_kind, CalleeBoxKind::RuntimeData);
            }
            _ => panic!("Expected Method callee"),
        }
    }
}
