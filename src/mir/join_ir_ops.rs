//! Phase 27.8: JoinIR 命令意味箱 (Ops Box)
//!
//! 目的: 比較演算の評価ロジックを一箇所に集約
//!
//! - `eval_compare()`: 比較演算の評価 (Lt, Le, Gt, Ge, Eq, Ne)
//!
//! Phase 27.8 の研究用 executor から切り出し、再利用可能な ops box として維持。

use crate::mir::join_ir::CompareOp;

/// JoinIR で扱う値型
///
/// Phase 27-shortterm S-5.1: VMValue との統合
/// - BoxRef variant 追加（S-5.2 で method_router 統合時に使用）
/// - from_vm_value()/into_vm_value() で VMValue と相互変換可能
#[derive(Debug, Clone)]
pub enum JoinValue {
    Int(i64),
    Bool(bool),
    Str(String),
    Unit,
    /// S-5.2: method_router 経由で Box/Plugin を実行するために使用
    BoxRef(std::sync::Arc<dyn crate::box_trait::NyashBox>),
}

// Manual PartialEq implementation (BoxRef は Arc::ptr_eq で比較)
impl PartialEq for JoinValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (JoinValue::Int(a), JoinValue::Int(b)) => a == b,
            (JoinValue::Bool(a), JoinValue::Bool(b)) => a == b,
            (JoinValue::Str(a), JoinValue::Str(b)) => a == b,
            (JoinValue::Unit, JoinValue::Unit) => true,
            (JoinValue::BoxRef(a), JoinValue::BoxRef(b)) => std::sync::Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

/// JoinIR ops box エラー型
#[derive(Debug, Clone)]
pub struct JoinIrOpError {
    pub message: String,
}

impl JoinIrOpError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

/// Phase 27.8: 比較演算の評価
///
/// ## 対応演算:
/// - **Int 比較**: Lt, Le, Gt, Ge, Eq, Ne
/// - **Bool 比較**: Eq, Ne のみ
/// - **String 比較**: Eq, Ne のみ
///
/// ## エラー:
/// - 型不一致 (例: Int と Bool の比較)
/// - Bool/String で Lt/Le/Gt/Ge を使用
pub fn eval_compare(
    op: CompareOp,
    lhs: &JoinValue,
    rhs: &JoinValue,
) -> Result<JoinValue, JoinIrOpError> {
    match (lhs, rhs) {
        (JoinValue::Int(a), JoinValue::Int(b)) => {
            let result = match op {
                CompareOp::Lt => a < b,
                CompareOp::Le => a <= b,
                CompareOp::Gt => a > b,
                CompareOp::Ge => a >= b,
                CompareOp::Eq => a == b,
                CompareOp::Ne => a != b,
            };
            Ok(JoinValue::Bool(result))
        }
        (JoinValue::Bool(a), JoinValue::Bool(b)) => match op {
            CompareOp::Eq => Ok(JoinValue::Bool(a == b)),
            CompareOp::Ne => Ok(JoinValue::Bool(a != b)),
            _ => Err(JoinIrOpError::new("Bool comparison only supports Eq/Ne")),
        },
        (JoinValue::Str(a), JoinValue::Str(b)) => match op {
            CompareOp::Eq => Ok(JoinValue::Bool(a == b)),
            CompareOp::Ne => Ok(JoinValue::Bool(a != b)),
            _ => Err(JoinIrOpError::new("String comparison only supports Eq/Ne")),
        },
        _ => Err(JoinIrOpError::new(
            "Type mismatch in Compare (expected homogeneous operands)",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_compare_int_gt() {
        let result = eval_compare(CompareOp::Gt, &JoinValue::Int(10), &JoinValue::Int(5));
        assert_eq!(result.unwrap(), JoinValue::Bool(true));
    }

    #[test]
    fn test_eval_compare_int_eq() {
        let result = eval_compare(CompareOp::Eq, &JoinValue::Int(5), &JoinValue::Int(5));
        assert_eq!(result.unwrap(), JoinValue::Bool(true));
    }

    #[test]
    fn test_eval_compare_str_eq() {
        let result = eval_compare(
            CompareOp::Eq,
            &JoinValue::Str("hello".to_string()),
            &JoinValue::Str("hello".to_string()),
        );
        assert_eq!(result.unwrap(), JoinValue::Bool(true));
    }

    #[test]
    fn test_eval_compare_str_ne() {
        let result = eval_compare(
            CompareOp::Ne,
            &JoinValue::Str("hello".to_string()),
            &JoinValue::Str("world".to_string()),
        );
        assert_eq!(result.unwrap(), JoinValue::Bool(true));
    }

    #[test]
    fn test_eval_compare_bool_eq() {
        let result = eval_compare(
            CompareOp::Eq,
            &JoinValue::Bool(true),
            &JoinValue::Bool(true),
        );
        assert_eq!(result.unwrap(), JoinValue::Bool(true));
    }

    #[test]
    fn test_eval_compare_bool_lt_error() {
        let result = eval_compare(
            CompareOp::Lt,
            &JoinValue::Bool(true),
            &JoinValue::Bool(false),
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message,
            "Bool comparison only supports Eq/Ne"
        );
    }

    #[test]
    fn test_eval_compare_type_mismatch() {
        let result = eval_compare(CompareOp::Eq, &JoinValue::Int(5), &JoinValue::Bool(true));
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Type mismatch"));
    }
}
