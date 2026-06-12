use crate::box_trait::NyashBox;
use std::fmt;

/// リテラル値の型 (Clone可能)
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    String(String),
    Integer(i64),
    TypedInteger {
        value: i64,
        declared_type_name: String,
    },
    Float(f64), // 浮動小数点数サポート追加
    Bool(bool),
    Null, // null値
    Void,
}

impl LiteralValue {
    /// LiteralValueをNyashBoxに変換
    pub fn to_nyash_box(&self) -> Box<dyn NyashBox> {
        use crate::box_trait::{BoolBox, IntegerBox, StringBox, VoidBox};
        use crate::boxes::FloatBox;

        match self {
            LiteralValue::String(s) => Box::new(StringBox::new(s)),
            LiteralValue::Integer(i) => Box::new(IntegerBox::new(*i)),
            LiteralValue::TypedInteger { value, .. } => Box::new(IntegerBox::new(*value)),
            LiteralValue::Float(f) => Box::new(FloatBox::new(*f)),
            LiteralValue::Bool(b) => Box::new(BoolBox::new(*b)),
            LiteralValue::Null => Box::new(crate::boxes::null_box::NullBox::new()),
            LiteralValue::Void => Box::new(VoidBox::new()),
        }
    }

    /// NyashBoxからLiteralValueに変換
    pub fn from_nyash_box(box_val: &dyn NyashBox) -> Option<LiteralValue> {
        use crate::box_trait::{BoolBox, IntegerBox, StringBox, VoidBox};
        use crate::boxes::FloatBox;
        #[allow(unused_imports)]
        use std::any::Any;

        if let Some(string_box) = box_val.as_any().downcast_ref::<StringBox>() {
            Some(LiteralValue::String(string_box.value.clone()))
        } else if let Some(int_box) = box_val.as_any().downcast_ref::<IntegerBox>() {
            Some(LiteralValue::Integer(int_box.value))
        } else if let Some(float_box) = box_val.as_any().downcast_ref::<FloatBox>() {
            Some(LiteralValue::Float(float_box.value))
        } else if let Some(bool_box) = box_val.as_any().downcast_ref::<BoolBox>() {
            Some(LiteralValue::Bool(bool_box.value))
        } else if box_val
            .as_any()
            .downcast_ref::<crate::boxes::null_box::NullBox>()
            .is_some()
        {
            Some(LiteralValue::Null)
        } else if box_val.as_any().downcast_ref::<VoidBox>().is_some() {
            Some(LiteralValue::Void)
        } else {
            None
        }
    }
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::String(s) => write!(f, "\"{}\"", s),
            LiteralValue::Integer(i) => write!(f, "{}", i),
            LiteralValue::TypedInteger {
                value,
                declared_type_name,
            } => write!(f, "{}{}", value, declared_type_name),
            LiteralValue::Float(fl) => write!(f, "{}", fl),
            LiteralValue::Bool(b) => write!(f, "{}", b),
            LiteralValue::Null => write!(f, "null"),
            LiteralValue::Void => write!(f, "void"),
        }
    }
}

/// 単項演算子の種類
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Minus,  // -x
    Not,    // not x / !x
    BitNot, // ~x
    Weak,   // weak x (Phase 285W-Syntax-0)
}

/// 二項演算子の種類
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitAnd,
    BitOr,
    BitXor,
    Shl, // << shift-left (Phase 1)
    Shr,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

/// Build-time conditional predicate carried by `when`.
///
/// This is intentionally separate from ordinary expression AST. `when` is
/// evaluated from build configuration before resolution/MIR, not at runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildPredicate {
    BuildFlag(String),
    Feature(String),
    TargetEq { key: String, value: String },
    BackendEq { key: String, value: String },
    Not(Box<BuildPredicate>),
    All(Vec<BuildPredicate>),
    Any(Vec<BuildPredicate>),
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            UnaryOperator::Minus => "-",
            UnaryOperator::Not => "not",
            UnaryOperator::BitNot => "~",
            UnaryOperator::Weak => "weak",
        };
        write!(f, "{}", symbol)
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulo => "%",
            BinaryOperator::BitAnd => "&",
            BinaryOperator::BitOr => "|",
            BinaryOperator::BitXor => "^",
            BinaryOperator::Shl => "<<",
            BinaryOperator::Shr => ">>",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::Less => "<",
            BinaryOperator::Greater => ">",
            BinaryOperator::LessEqual => "<=",
            BinaryOperator::GreaterEqual => ">=",
            BinaryOperator::And => "&&",
            BinaryOperator::Or => "||",
        };
        write!(f, "{}", symbol)
    }
}
