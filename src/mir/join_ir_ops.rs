//! Phase 27.8: JoinIR 命令意味箱 (Ops Box)
//!
//! 目的: BinOp / Compare の評価ロジックを一箇所に集約
//!
//! - `eval_binop()`: 二項演算の評価 (Add, Sub, Mul, Div, Or, And)
//! - `eval_compare()`: 比較演算の評価 (Lt, Le, Gt, Ge, Eq, Ne)
//!
//! Phase 27.8 以前は join_ir_runner.rs に直接記述されていたが、
//! 再利用性とテスタビリティ向上のため ops box として分離。

use crate::mir::join_ir::{BinOpKind, CompareOp};

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

/// Phase 27.8: 二項演算の評価
///
/// ## 対応演算:
/// - **Add**: Int+Int, Str+Str (文字列連結)
/// - **Sub**: Int-Int
/// - **Mul**: Int*Int
/// - **Div**: Int/Int (ゼロ除算チェック)
/// - **Or**: Bool||Bool
/// - **And**: Bool&&Bool
///
/// ## エラー:
/// - 型不一致 (例: Int + Bool)
/// - ゼロ除算 (Int / 0)
pub fn eval_binop(
    op: BinOpKind,
    lhs: &JoinValue,
    rhs: &JoinValue,
) -> Result<JoinValue, JoinIrOpError> {
    match op {
        BinOpKind::Add => match (lhs, rhs) {
            (JoinValue::Int(a), JoinValue::Int(b)) => Ok(JoinValue::Int(a + b)),
            (JoinValue::Str(a), JoinValue::Str(b)) => Ok(JoinValue::Str(format!("{}{}", a, b))),
            _ => Err(JoinIrOpError::new(
                "Add supported only for Int+Int or Str+Str",
            )),
        },
        BinOpKind::Sub => match (lhs, rhs) {
            (JoinValue::Int(a), JoinValue::Int(b)) => Ok(JoinValue::Int(a - b)),
            _ => Err(JoinIrOpError::new("Sub supported only for Int-Int")),
        },
        BinOpKind::Mul => match (lhs, rhs) {
            (JoinValue::Int(a), JoinValue::Int(b)) => Ok(JoinValue::Int(a * b)),
            _ => Err(JoinIrOpError::new("Mul supported only for Int*Int")),
        },
        BinOpKind::Div => match (lhs, rhs) {
            (JoinValue::Int(_), JoinValue::Int(0)) => Err(JoinIrOpError::new("Division by zero")),
            (JoinValue::Int(a), JoinValue::Int(b)) => Ok(JoinValue::Int(a / b)),
            _ => Err(JoinIrOpError::new("Div supported only for Int/Int")),
        },
        BinOpKind::Mod => match (lhs, rhs) {
            (JoinValue::Int(_), JoinValue::Int(0)) => Err(JoinIrOpError::new("Modulo by zero")),
            (JoinValue::Int(a), JoinValue::Int(b)) => Ok(JoinValue::Int(a % b)),
            _ => Err(JoinIrOpError::new("Mod supported only for Int%Int")),
        },
        BinOpKind::Or => match (lhs, rhs) {
            (JoinValue::Bool(a), JoinValue::Bool(b)) => Ok(JoinValue::Bool(*a || *b)),
            _ => Err(JoinIrOpError::new("Or supported only for Bool||Bool")),
        },
        BinOpKind::And => match (lhs, rhs) {
            (JoinValue::Bool(a), JoinValue::Bool(b)) => Ok(JoinValue::Bool(*a && *b)),
            _ => Err(JoinIrOpError::new("And supported only for Bool&&Bool")),
        },
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
    fn test_eval_binop_add_int() {
        let result = eval_binop(BinOpKind::Add, &JoinValue::Int(3), &JoinValue::Int(5));
        assert_eq!(result.unwrap(), JoinValue::Int(8));
    }

    #[test]
    fn test_eval_binop_add_str() {
        let result = eval_binop(
            BinOpKind::Add,
            &JoinValue::Str("hello".to_string()),
            &JoinValue::Str("world".to_string()),
        );
        assert_eq!(result.unwrap(), JoinValue::Str("helloworld".to_string()));
    }

    #[test]
    fn test_eval_binop_sub() {
        let result = eval_binop(BinOpKind::Sub, &JoinValue::Int(10), &JoinValue::Int(3));
        assert_eq!(result.unwrap(), JoinValue::Int(7));
    }

    #[test]
    fn test_eval_binop_div_by_zero() {
        let result = eval_binop(BinOpKind::Div, &JoinValue::Int(10), &JoinValue::Int(0));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Division by zero");
    }

    #[test]
    fn test_eval_binop_or() {
        let result = eval_binop(
            BinOpKind::Or,
            &JoinValue::Bool(true),
            &JoinValue::Bool(false),
        );
        assert_eq!(result.unwrap(), JoinValue::Bool(true));
    }

    #[test]
    fn test_eval_binop_and() {
        let result = eval_binop(
            BinOpKind::And,
            &JoinValue::Bool(true),
            &JoinValue::Bool(false),
        );
        assert_eq!(result.unwrap(), JoinValue::Bool(false));
    }

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

/// Phase 27-shortterm S-4.2: JoinValue ↔ VMValue conversion
///
/// Provides bidirectional conversion between JoinIR's lightweight JoinValue
/// and VM's runtime VMValue representation.
///
/// ## Conversion Strategy
/// - JoinValue::Int    ↔ VMValue::Integer
/// - JoinValue::Bool   ↔ VMValue::Bool
/// - JoinValue::Str    ↔ VMValue::String
/// - JoinValue::Unit   ↔ VMValue::Void
/// - VMValue::Float    → Error (not supported in JoinValue)
/// - VMValue::Future   → Error (not supported in JoinValue)
/// - VMValue::BoxRef   → Error (requires NyashBox downcast)
impl JoinValue {
    /// Convert JoinValue to VMValue for VM execution
    ///
    /// Phase 27-shortterm S-5.1: BoxRef サポート追加
    ///
    /// # Example
    /// ```ignore
    /// let join_val = JoinValue::Int(42);
    /// let vm_val = join_val.to_vm_value();
    /// assert!(matches!(vm_val, VMValue::Integer(42)));
    /// ```
    pub fn to_vm_value(&self) -> crate::backend::VMValue {
        match self {
            JoinValue::Int(i) => crate::backend::VMValue::Integer(*i),
            JoinValue::Bool(b) => crate::backend::VMValue::Bool(*b),
            JoinValue::Str(s) => crate::backend::VMValue::String(s.clone()),
            JoinValue::Unit => crate::backend::VMValue::Void,
            JoinValue::BoxRef(b) => crate::backend::VMValue::BoxRef(b.clone()),
        }
    }

    /// Convert JoinValue into VMValue (consuming self)
    ///
    /// Phase 27-shortterm S-5.1: Zero-cost conversion for owned values
    pub fn into_vm_value(self) -> crate::backend::VMValue {
        match self {
            JoinValue::Int(i) => crate::backend::VMValue::Integer(i),
            JoinValue::Bool(b) => crate::backend::VMValue::Bool(b),
            JoinValue::Str(s) => crate::backend::VMValue::String(s),
            JoinValue::Unit => crate::backend::VMValue::Void,
            JoinValue::BoxRef(b) => crate::backend::VMValue::BoxRef(b),
        }
    }

    /// Convert VMValue to JoinValue
    ///
    /// Phase 27-shortterm S-5.1: BoxRef サポート追加
    ///
    /// Returns error for VMValue types not representable in JoinValue (Float, Future).
    ///
    /// # Example
    /// ```ignore
    /// let vm_val = VMValue::Integer(42);
    /// let join_val = JoinValue::from_vm_value(&vm_val)?;
    /// assert_eq!(join_val, JoinValue::Int(42));
    /// ```
    pub fn from_vm_value(vm_val: &crate::backend::VMValue) -> Result<Self, JoinIrOpError> {
        match vm_val {
            crate::backend::VMValue::Integer(i) => Ok(JoinValue::Int(*i)),
            crate::backend::VMValue::Bool(b) => Ok(JoinValue::Bool(*b)),
            crate::backend::VMValue::String(s) => Ok(JoinValue::Str(s.clone())),
            crate::backend::VMValue::Void => Ok(JoinValue::Unit),
            crate::backend::VMValue::BoxRef(b) => Ok(JoinValue::BoxRef(b.clone())),
            crate::backend::VMValue::Float(_) => {
                Err(JoinIrOpError::new("Float not supported in JoinValue"))
            }
            crate::backend::VMValue::Future(_) => {
                Err(JoinIrOpError::new("Future not supported in JoinValue"))
            }
            // Phase 285A0: Upgrade WeakBox before conversion
            crate::backend::VMValue::WeakBox(w) => {
                if let Some(arc) = w.upgrade() {
                    Ok(JoinValue::BoxRef(arc))
                } else {
                    Ok(JoinValue::Unit) // Dropped weak → Unit (null)
                }
            }
        }
    }
}

#[cfg(test)]
mod vm_conversion_tests {
    use super::*;

    #[test]
    fn test_joinvalue_to_vmvalue() {
        // Int → Integer
        let join_int = JoinValue::Int(42);
        let vm_val = join_int.to_vm_value();
        assert!(matches!(vm_val, crate::backend::VMValue::Integer(42)));

        // Bool → Bool
        let join_bool = JoinValue::Bool(true);
        let vm_val = join_bool.to_vm_value();
        assert!(matches!(vm_val, crate::backend::VMValue::Bool(true)));

        // Str → String
        let join_str = JoinValue::Str("hello".to_string());
        let vm_val = join_str.to_vm_value();
        assert!(matches!(vm_val, crate::backend::VMValue::String(s) if s == "hello"));

        // Unit → Void
        let join_unit = JoinValue::Unit;
        let vm_val = join_unit.to_vm_value();
        assert!(matches!(vm_val, crate::backend::VMValue::Void));
    }

    #[test]
    fn test_vmvalue_to_joinvalue() {
        // Integer → Int
        let vm_int = crate::backend::VMValue::Integer(42);
        let join_val = JoinValue::from_vm_value(&vm_int).unwrap();
        assert_eq!(join_val, JoinValue::Int(42));

        // Bool → Bool
        let vm_bool = crate::backend::VMValue::Bool(false);
        let join_val = JoinValue::from_vm_value(&vm_bool).unwrap();
        assert_eq!(join_val, JoinValue::Bool(false));

        // String → Str
        let vm_str = crate::backend::VMValue::String("world".to_string());
        let join_val = JoinValue::from_vm_value(&vm_str).unwrap();
        assert_eq!(join_val, JoinValue::Str("world".to_string()));

        // Void → Unit
        let vm_void = crate::backend::VMValue::Void;
        let join_val = JoinValue::from_vm_value(&vm_void).unwrap();
        assert_eq!(join_val, JoinValue::Unit);
    }

    #[test]
    fn test_vmvalue_to_joinvalue_unsupported() {
        // Float → Error
        let vm_float = crate::backend::VMValue::Float(3.14);
        let result = JoinValue::from_vm_value(&vm_float);
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Float not supported"));
    }
}
