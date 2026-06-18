use std::fmt;

pub use hakorune_frontend_ast::{BinaryOperator, BuildPredicate, UnaryOperator};

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
