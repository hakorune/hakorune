use nyash_rust::ast::{
    BinaryOperator, DeclarationAttrs, LiteralValue, RuneAttr, UnaryOperator,
};
use serde_json::{json, Value};

pub(super) fn attrs_to_json(attrs: &DeclarationAttrs) -> Value {
    json!({
        "runes": attrs
            .runes
            .iter()
            .map(|rune| json!({"name": rune.name, "args": rune.args}))
            .collect::<Vec<_>>()
    })
}

pub(super) fn json_to_attrs(value: Option<&Value>) -> DeclarationAttrs {
    let runes = value
        .and_then(|attrs| attrs.get("runes"))
        .and_then(Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    Some(RuneAttr {
                        name: entry.get("name")?.as_str()?.to_string(),
                        args: entry
                            .get("args")
                            .and_then(Value::as_array)
                            .map(|args| {
                                args.iter()
                                    .filter_map(|arg| arg.as_str().map(|s| s.to_string()))
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default(),
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    DeclarationAttrs { runes }
}

pub(super) fn json_to_lit(v: &Value) -> Option<LiteralValue> {
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

pub(super) fn str_to_bin(s: &str) -> Option<BinaryOperator> {
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

pub(super) fn str_to_un(s: &str) -> Option<UnaryOperator> {
    Some(match s {
        "-" => UnaryOperator::Minus,
        "not" => UnaryOperator::Not,
        "~" => UnaryOperator::BitNot,
        "weak" => UnaryOperator::Weak,
        _ => return None,
    })
}

pub(super) fn literal_to_joinir_json(v: &LiteralValue) -> Value {
    match v {
        LiteralValue::Integer(i) => json!({
            "kind": "Literal",
            "type": "Int",
            "value": i
        }),
        LiteralValue::Bool(b) => json!({
            "kind": "Literal",
            "type": "Bool",
            "value": b
        }),
        LiteralValue::String(s) => json!({
            "kind": "Literal",
            "type": "String",
            "value": s
        }),
        LiteralValue::Float(f) => json!({
            "kind": "Literal",
            "type": "Float",
            "value": f
        }),
        LiteralValue::Null => json!({
            "kind": "Literal",
            "type": "Null"
        }),
        LiteralValue::Void => json!({
            "kind": "Literal",
            "type": "Void"
        }),
    }
}
