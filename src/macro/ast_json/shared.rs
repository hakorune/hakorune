use nyash_rust::ast::{BinaryOperator, LiteralValue, UnaryOperator};
use serde_json::{json, Value};

pub(crate) fn lit_to_json(v: &LiteralValue) -> Value {
    match v {
        LiteralValue::String(s) => json!({"type":"string","value":s}),
        LiteralValue::Integer(i) => json!({"type":"int","value":i}),
        LiteralValue::Float(f) => json!({"type":"float","value":f}),
        LiteralValue::Bool(b) => json!({"type":"bool","value":b}),
        LiteralValue::Null => json!({"type":"null"}),
        LiteralValue::Void => json!({"type":"void"}),
    }
}

pub(crate) fn json_to_lit(v: &Value) -> Option<LiteralValue> {
    let t = v.get("type")?.as_str()?;
    Some(match t {
        "string" => LiteralValue::String(v.get("value")?.as_str()?.to_string()),
        "int" => LiteralValue::Integer(v.get("value")?.as_i64()?),
        "float" => LiteralValue::Float(v.get("value")?.as_f64()?),
        "bool" => LiteralValue::Bool(v.get("value")?.as_bool()?),
        "null" => LiteralValue::Null,
        "void" => LiteralValue::Void,
        _ => return None,
    })
}

pub(crate) fn bin_to_str(op: &BinaryOperator) -> &'static str {
    match op {
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
    }
}

pub(crate) fn str_to_bin(s: &str) -> Option<BinaryOperator> {
    Some(match s {
        "+" => BinaryOperator::Add,
        "-" => BinaryOperator::Subtract,
        "*" => BinaryOperator::Multiply,
        "/" => BinaryOperator::Divide,
        "%" => BinaryOperator::Modulo,
        "&" => BinaryOperator::BitAnd,
        "|" => BinaryOperator::BitOr,
        "^" => BinaryOperator::BitXor,
        "<<" => BinaryOperator::Shl,
        ">>" => BinaryOperator::Shr,
        "==" => BinaryOperator::Equal,
        "!=" => BinaryOperator::NotEqual,
        "<" => BinaryOperator::Less,
        ">" => BinaryOperator::Greater,
        "<=" => BinaryOperator::LessEqual,
        ">=" => BinaryOperator::GreaterEqual,
        "&&" => BinaryOperator::And,
        "||" => BinaryOperator::Or,
        _ => return None,
    })
}

pub(crate) fn un_to_str(op: &UnaryOperator) -> &'static str {
    match op {
        UnaryOperator::Minus => "-",
        UnaryOperator::Not => "!",
        UnaryOperator::BitNot => "~",
        UnaryOperator::Weak => "weak",
    }
}

pub(crate) fn str_to_un(s: &str) -> Option<UnaryOperator> {
    Some(match s {
        "-" => UnaryOperator::Minus,
        "!" => UnaryOperator::Not,
        "~" => UnaryOperator::BitNot,
        "weak" => UnaryOperator::Weak,
        _ => return None,
    })
}

pub(crate) fn is_compare_op(op: &BinaryOperator) -> bool {
    matches!(
        op,
        BinaryOperator::Equal
            | BinaryOperator::NotEqual
            | BinaryOperator::Less
            | BinaryOperator::Greater
            | BinaryOperator::LessEqual
            | BinaryOperator::GreaterEqual
    )
}
