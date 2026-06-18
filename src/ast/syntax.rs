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
