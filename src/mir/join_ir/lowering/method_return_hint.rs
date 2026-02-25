//! Phase 83: P3-D 既知メソッド戻り値型推論箱
//!
//! lifecycle.rs の return 型推論で、既知メソッドの戻り値型を
//! TypeAnnotationBox のマッピングを利用して推論する。
//!
//! # 責務
//!
//! - ret_val の定義元 Call/TypeOp を探索
//! - メソッド名から TypeAnnotationBox::infer_return_type() で型を取得
//! - P3-D として lifecycle.rs から呼ばれる
//!
//! # 設計原則（箱理論）
//!
//! - **単一責務**: 既知メソッド戻り値型推論のみ
//! - **TypeAnnotationBox 薄ラップ**: 型マッピングの SSOT は TypeAnnotationBox
//! - **将来移行性**: MethodRegistry 導入時も API 不変

use crate::mir::{MirFunction, MirInstruction, MirType, TypeOpKind, ValueId};
use std::collections::BTreeMap;

/// Phase 83: 既知メソッド戻り値型推論箱
///
/// # 対象パターン（P3-D）
///
/// - `StringBox.length()` → i64
/// - `ArrayBox.push()` → void
/// - その他 TypeAnnotationBox でカバーされるメソッド
///
/// # Phase 82 との関係
///
/// Phase 82 で判明した「残り 20 件の Case D」のうち、
/// P3-C (GenericTypeResolver) 対象外のものを処理する。
///
/// # 呼び出しフロー
///
/// lifecycle.rs → MethodReturnHintBox → TypeAnnotationBox
pub struct MethodReturnHintBox;

impl MethodReturnHintBox {
    /// P3-D: 既知メソッドの戻り値型を返す
    ///
    /// # アルゴリズム
    ///
    /// 1. value_types に既に型が入っていればそれを返す
    /// 2. ret_val の定義元 instruction を探す
    /// 3. Call(Method) なら method 名から型を推論
    ///
    /// # 引数
    ///
    /// - `function`: MIR 関数
    /// - `ret_val`: return 値の ValueId
    /// - `value_types`: 既知の型マップ
    ///
    /// # 戻り値
    ///
    /// - `Some(MirType)`: 既知メソッドの戻り値型が判明した場合
    /// - `None`: 未知のメソッド、または Call(Method) 以外
    pub fn resolve_for_return(
        function: &MirFunction,
        ret_val: ValueId,
        value_types: &BTreeMap<ValueId, MirType>,
    ) -> Option<MirType> {
        // 1. value_types に既に型が入っていればそれを返す
        if let Some(t) = value_types.get(&ret_val) {
            return Some(t.clone());
        }

        // 2. ret_val の定義元 instruction を探す
        for (_bid, bb) in function.blocks.iter() {
            for inst in bb.instructions.iter() {
                match inst {
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee:
                            Some(crate::mir::Callee::Method {
                                receiver: Some(box_val),
                                method,
                                ..
                            }),
                        ..
                    } if *dst == ret_val => {
                        // 3. Call(Method) の method から型を推論
                        let box_type = value_types.get(box_val);
                        if let Some(mt) = Self::infer_from_boxcall(box_type, method) {
                            return Some(mt);
                        }
                    }
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(callee),
                        ..
                    } if *dst == ret_val => {
                        // Global/Extern/other callee paths
                        if let Some(mt) = Self::infer_from_callee(callee) {
                            return Some(mt);
                        }
                    }
                    // TypeOp: .is() 型チェック → Bool, .as() 型キャスト → 対象型
                    MirInstruction::TypeOp { dst, op, ty, .. } if *dst == ret_val => {
                        return Some(Self::infer_from_typeop(*op, ty));
                    }
                    _ => {}
                }
            }
        }

        None
    }

    /// MethodCall から戻り値型を推論
    fn infer_from_boxcall(box_type: Option<&MirType>, method: &str) -> Option<MirType> {
        // TypeAnnotationBox と同じロジックを適用
        // メソッド名だけで判定できるケース
        if method == "length" || method == "size" || method == "len" {
            return Some(MirType::Integer);
        }
        if method == "str" {
            return Some(MirType::String);
        }
        if method == "substring" {
            return Some(MirType::String);
        }
        if method == "esc_json" {
            return Some(MirType::String);
        }
        if method == "indexOf" || method == "lastIndexOf" {
            return Some(MirType::Integer);
        }
        if method == "is_digit_char" || method == "is_hex_digit_char" || method == "is_alpha_char" {
            return Some(MirType::Bool);
        }
        if method == "has" {
            // MapBox.has → Bool
            if let Some(MirType::Box(name)) = box_type {
                if name == "MapBox" {
                    return Some(MirType::Bool);
                }
            }
        }
        // push は void (Unit) を返す
        if method == "push" {
            return Some(MirType::Void);
        }

        None
    }

    /// Callee から戻り値型を推論
    fn infer_from_callee(callee: &crate::mir::Callee) -> Option<MirType> {
        use crate::mir::Callee;

        match callee {
            Callee::Method {
                box_name, method, ..
            } => {
                // Box 型を持っていないので、メソッド名だけで判定
                let box_type = Some(MirType::Box(box_name.clone()));
                Self::infer_from_boxcall(box_type.as_ref(), method)
            }
            Callee::Global(func_name) => {
                // TypeAnnotationBox::infer_return_type と同じロジック
                if func_name.ends_with(".str/0") {
                    return Some(MirType::String);
                }
                if func_name.ends_with(".length/0")
                    || func_name.ends_with(".size/0")
                    || func_name.ends_with(".len/0")
                {
                    return Some(MirType::Integer);
                }
                if func_name.ends_with(".substring/2") {
                    return Some(MirType::String);
                }
                None
            }
            _ => None,
        }
    }

    /// TypeOp から戻り値型を推論
    ///
    /// - TypeOpKind::Check (.is()) → Bool
    /// - TypeOpKind::Cast (.as()) → 対象型
    fn infer_from_typeop(op: TypeOpKind, ty: &MirType) -> MirType {
        match op {
            TypeOpKind::Check => MirType::Bool,
            TypeOpKind::Cast => ty.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_from_boxcall_length() {
        let box_type = Some(MirType::Box("StringBox".to_string()));
        assert_eq!(
            MethodReturnHintBox::infer_from_boxcall(box_type.as_ref(), "length"),
            Some(MirType::Integer)
        );
    }

    #[test]
    fn test_infer_from_boxcall_push() {
        let box_type = Some(MirType::Box("ArrayBox".to_string()));
        assert_eq!(
            MethodReturnHintBox::infer_from_boxcall(box_type.as_ref(), "push"),
            Some(MirType::Void)
        );
    }

    #[test]
    fn test_infer_from_boxcall_str() {
        let box_type = Some(MirType::Box("IntegerBox".to_string()));
        assert_eq!(
            MethodReturnHintBox::infer_from_boxcall(box_type.as_ref(), "str"),
            Some(MirType::String)
        );
    }

    #[test]
    fn test_infer_from_boxcall_mapbox_has() {
        let box_type = Some(MirType::Box("MapBox".to_string()));
        assert_eq!(
            MethodReturnHintBox::infer_from_boxcall(box_type.as_ref(), "has"),
            Some(MirType::Bool)
        );
    }

    #[test]
    fn test_infer_from_boxcall_unknown() {
        let box_type = Some(MirType::Box("UnknownBox".to_string()));
        assert_eq!(
            MethodReturnHintBox::infer_from_boxcall(box_type.as_ref(), "unknown_method"),
            None
        );
    }

    #[test]
    fn test_infer_from_typeop_check() {
        // TypeOpKind::Check (.is()) → Bool
        assert_eq!(
            MethodReturnHintBox::infer_from_typeop(TypeOpKind::Check, &MirType::String),
            MirType::Bool
        );
    }

    #[test]
    fn test_infer_from_typeop_cast() {
        // TypeOpKind::Cast (.as()) → 対象型
        assert_eq!(
            MethodReturnHintBox::infer_from_typeop(TypeOpKind::Cast, &MirType::Integer),
            MirType::Integer
        );
        assert_eq!(
            MethodReturnHintBox::infer_from_typeop(TypeOpKind::Cast, &MirType::String),
            MirType::String
        );
    }
}
