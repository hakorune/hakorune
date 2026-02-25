/*!
 * EffectsAnalyzerBox - エフェクト解析専用箱
 *
 * 箱理論の実践:
 * - 箱にする: エフェクト解析ロジックを1箱に集約
 * - 境界を作る: Pure/IO/Alloc/Control等の効果分類を一元管理
 * - 状態レス: すべて静的解析（実行時状態不要）
 *
 * 責務:
 * - compute_call_effects: Calleeから副作用マスクを計算
 * - is_pure_method: メソッドがPure（副作用なし）か判定
 * - 既知の関数・メソッドのエフェクト知識を集約
 */

use super::extern_calls;
use crate::mir::builder::{Effect, EffectMask};
use crate::mir::definitions::call_unified::Callee;

/// エフェクト解析専用箱
///
/// 箱理論:
/// - 単一責務: Calleeのエフェクト解析のみ
/// - 状態レス: すべて静的関数（Callee情報のみで判定）
/// - 知識集約: 既知の関数・メソッドのエフェクト知識を一元管理
pub struct EffectsAnalyzerBox;

impl EffectsAnalyzerBox {
    /// Compute effects for a call based on its callee
    ///
    /// エフェクト分類:
    /// - PURE: 副作用なし（純粋計算）
    /// - READ: ヒープ読み取りのみ
    /// - IO: 入出力あり
    /// - Alloc: メモリ確保
    /// - Control: 制御フロー変更（panic/exit）
    /// - WriteHeap: ヒープ書き込み
    pub fn compute_call_effects(callee: &Callee) -> EffectMask {
        match callee {
            Callee::Global(name) => match name.as_str() {
                "print" | "error" => EffectMask::IO,
                "panic" | "exit" => EffectMask::IO.add(Effect::Control),
                "gc_collect" => EffectMask::IO.add(Effect::Alloc),
                _ => EffectMask::IO,
            },

            Callee::Method {
                method, box_name, ..
            } => {
                match method.as_str() {
                    "birth" => EffectMask::PURE.add(Effect::Alloc),
                    "get" | "length" | "size" => EffectMask::READ,
                    "set" | "push" | "pop" => EffectMask::READ.add(Effect::WriteHeap),
                    _ => {
                        // Check if it's a known pure method
                        if Self::is_pure_method(box_name, method) {
                            EffectMask::PURE
                        } else {
                            EffectMask::READ
                        }
                    }
                }
            }

            Callee::Constructor { .. } => EffectMask::PURE.add(Effect::Alloc),

            Callee::Closure { .. } => EffectMask::PURE.add(Effect::Alloc),

            Callee::Extern(name) => {
                let (iface, method) = extern_calls::parse_extern_name(name);
                extern_calls::compute_extern_effects(&iface, &method)
            }

            Callee::Value(_) => EffectMask::IO, // Conservative for dynamic calls
        }
    }

    /// Check if a method is known to be pure (no side effects)
    ///
    /// Pure メソッドの条件:
    /// - 副作用なし（ヒープ変更なし、I/Oなし）
    /// - 同じ入力に対して同じ出力を返す
    /// - プログラムの状態を変更しない
    ///
    /// 既知のPureメソッド:
    /// - StringBox: upper, lower, trim, length
    /// - IntegerBox: abs, toString
    /// - FloatBox: round, floor, ceil
    /// - BoolBox: not
    pub fn is_pure_method(box_name: &str, method: &str) -> bool {
        match (box_name, method) {
            ("StringBox", m) => matches!(m, "upper" | "lower" | "trim" | "length"),
            ("IntegerBox", m) => matches!(m, "abs" | "toString"),
            ("FloatBox", m) => matches!(m, "round" | "floor" | "ceil"),
            ("BoolBox", "not") => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::ValueId;

    #[test]
    fn test_compute_effects_global() {
        let callee = Callee::Global("print".to_string());
        let effects = EffectsAnalyzerBox::compute_call_effects(&callee);
        assert_eq!(effects, EffectMask::IO);
    }

    #[test]
    fn test_compute_effects_method_pure() {
        let callee = Callee::Method {
            box_name: "StringBox".to_string(),
            method: "upper".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        };
        let effects = EffectsAnalyzerBox::compute_call_effects(&callee);
        assert_eq!(effects, EffectMask::PURE);
    }

    #[test]
    fn test_compute_effects_method_read() {
        let callee = Callee::Method {
            box_name: "ArrayBox".to_string(),
            method: "get".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        };
        let effects = EffectsAnalyzerBox::compute_call_effects(&callee);
        assert_eq!(effects, EffectMask::READ);
    }

    #[test]
    fn test_compute_effects_constructor() {
        let callee = Callee::Constructor {
            box_type: "StringBox".to_string(),
        };
        let effects = EffectsAnalyzerBox::compute_call_effects(&callee);
        // Constructor should have PURE + Alloc
        assert_eq!(
            effects,
            EffectMask::PURE.add(crate::mir::builder::Effect::Alloc)
        );
    }

    #[test]
    fn test_is_pure_method() {
        assert!(EffectsAnalyzerBox::is_pure_method("StringBox", "upper"));
        assert!(EffectsAnalyzerBox::is_pure_method("IntegerBox", "abs"));
        assert!(EffectsAnalyzerBox::is_pure_method("BoolBox", "not"));
        assert!(!EffectsAnalyzerBox::is_pure_method("ArrayBox", "push"));
    }
}
