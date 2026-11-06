use nyash_rust::ast::{DeclarationAttrs, LiteralValue};
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

pub(super) fn literal_to_joinir_json(v: &LiteralValue) -> Value {
    match v {
        LiteralValue::Integer(i) => json!({
            "kind": "Literal",
            "type": "Int",
            "value": i
        }),
        LiteralValue::TypedInteger {
            value,
            declared_type_name,
        } => json!({
            "kind": "Literal",
            "type": "Int",
            "value": value,
            "declared_type": declared_type_name
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
