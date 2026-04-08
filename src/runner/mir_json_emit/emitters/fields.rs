use serde_json::json;

use crate::mir::{MirType, ValueId};

fn emit_declared_type_json(ty: &Option<MirType>) -> serde_json::Value {
    match ty {
        Some(MirType::Integer) => json!("i64"),
        Some(MirType::Float) => json!("f64"),
        Some(MirType::Bool) => json!("i1"),
        Some(MirType::String) => json!({"kind": "handle", "box_type": "StringBox"}),
        Some(MirType::Box(name)) => json!({"kind": "handle", "box_type": name}),
        Some(MirType::Void) => json!("void"),
        _ => json!(null),
    }
}

pub(crate) fn emit_field_get(
    dst: &ValueId,
    base: &ValueId,
    field: &str,
    declared_type: &Option<MirType>,
) -> serde_json::Value {
    let mut obj = json!({
        "op": "field_get",
        "dst": dst.as_u32(),
        "box": base.as_u32(),
        "field": field,
    });
    let declared_type_json = emit_declared_type_json(declared_type);
    if !declared_type_json.is_null() {
        obj["declared_type"] = declared_type_json;
    }
    obj
}

pub(crate) fn emit_field_set(
    base: &ValueId,
    field: &str,
    value: &ValueId,
    declared_type: &Option<MirType>,
) -> serde_json::Value {
    let mut obj = json!({
        "op": "field_set",
        "box": base.as_u32(),
        "field": field,
        "value": value.as_u32(),
    });
    let declared_type_json = emit_declared_type_json(declared_type);
    if !declared_type_json.is_null() {
        obj["declared_type"] = declared_type_json;
    }
    obj
}
