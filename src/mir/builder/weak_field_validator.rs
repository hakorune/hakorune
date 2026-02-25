// Phase 285A1: Weak Field Contract Validator
//
// Responsibility: weak field への代入時の型契約検証
// - 読み込み時の型注釈（WeakRef として扱う）
// - 書き込み時の型検証（3つの許可ケース）

use super::type_context::TypeContext;
use super::ValueId;
use crate::mir::types::MirType;

/// Phase 285A1: Weak Field Contract Validator Box
///
/// 単一責任: weak field の契約検証のみ
/// - 読み込み時: WeakRef 型注釈
/// - 書き込み時: 型契約検証（WeakRef/Void のみ許可）
pub(super) struct WeakFieldValidatorBox;

impl WeakFieldValidatorBox {
    /// Weak field 読み込み時の型注釈を追加
    ///
    /// Phase 285A1: weak field の読み込み結果は WeakRef 型として扱う（自動昇格なし）
    ///
    /// # Arguments
    /// - `type_ctx`: 型コンテキスト（value_types に WeakRef を記録）
    /// - `dst`: 読み込み結果の ValueId
    pub(super) fn annotate_read_result(
        type_ctx: &mut TypeContext,
        dst: ValueId,
    ) {
        // Phase 285A1: Mark the result as WeakRef type
        type_ctx.value_types.insert(dst, MirType::WeakRef);
    }

    /// Weak field への代入を検証（3つの許可ケース）
    ///
    /// Phase 285A1 Fail-Fast 契約:
    /// - **許可**: WeakRef (weak() または weak field 読み込み)
    /// - **許可**: Void (クリア操作)
    /// - **禁止**: BoxRef (weak() なしの Box)
    /// - **禁止**: プリミティブ型
    /// - **禁止**: 型追跡されていない値
    ///
    /// # Arguments
    /// - `value_type`: 代入する値の型情報
    /// - `box_name`: フィールドを持つ Box 名（エラーメッセージ用）
    /// - `field_name`: weak field 名（エラーメッセージ用）
    ///
    /// # Returns
    /// - `Ok(())`: 代入が許可される場合
    /// - `Err(String)`: 代入が禁止される場合（Fail-Fast）
    ///
    /// # Example Error Messages
    /// ```text
    /// Cannot assign Box (StringBox) to weak field 'Node.next'.
    /// Use weak(...) to create weak reference: me.next = weak(value)
    /// ```
    pub(super) fn validate_assignment(
        value_type: Option<&MirType>,
        box_name: &str,
        field_name: &str,
    ) -> Result<(), String> {
        match value_type {
            // Case 1: WeakRef allowed (from weak() or weak field read)
            Some(MirType::WeakRef) => Ok(()),

            // Case 2: Void allowed (clear operation)
            Some(MirType::Void) => Ok(()),

            // Forbidden: None/Unknown (型追跡漏れ防止 - Fail-Fast)
            None => Err(format!(
                "Cannot assign untracked value to weak field '{}.{}'. Use weak(...) or Void explicitly.",
                box_name, field_name
            )),

            // Forbidden: BoxRef (強参照を直接代入 - Fail-Fast)
            Some(MirType::Box(box_type)) => Err(format!(
                "Cannot assign Box ({}) to weak field '{}.{}'. Use weak(...) to create weak reference: me.{} = weak(value)",
                box_type, box_name, field_name, field_name
            )),

            // Forbidden: Primitives and others (Fail-Fast)
            Some(other_type) => Err(format!(
                "Cannot assign {:?} to weak field '{}.{}'. Weak fields require WeakRef type. Use weak(...) or Void.",
                other_type, box_name, field_name
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_weakref_allowed() {
        let result = WeakFieldValidatorBox::validate_assignment(
            Some(&MirType::WeakRef),
            "Node",
            "next",
        );
        assert!(result.is_ok(), "WeakRef should be allowed");
    }

    #[test]
    fn test_validate_void_allowed() {
        let result = WeakFieldValidatorBox::validate_assignment(
            Some(&MirType::Void),
            "Node",
            "next",
        );
        assert!(result.is_ok(), "Void should be allowed");
    }

    #[test]
    fn test_validate_boxref_forbidden() {
        let result = WeakFieldValidatorBox::validate_assignment(
            Some(&MirType::Box("StringBox".to_string())),
            "Node",
            "next",
        );
        assert!(result.is_err(), "BoxRef should be forbidden");
        assert!(result.unwrap_err().contains("Use weak(...)"));
    }

    #[test]
    fn test_validate_untracked_forbidden() {
        let result = WeakFieldValidatorBox::validate_assignment(
            None,
            "Node",
            "next",
        );
        assert!(result.is_err(), "Untracked value should be forbidden");
        assert!(result.unwrap_err().contains("untracked value"));
    }

    #[test]
    fn test_validate_primitive_forbidden() {
        let result = WeakFieldValidatorBox::validate_assignment(
            Some(&MirType::Integer),
            "Node",
            "count",
        );
        assert!(result.is_err(), "Primitive should be forbidden");
        assert!(result.unwrap_err().contains("Weak fields require WeakRef"));
    }
}
