use crate::mir::{MirType, ValueId};
use serde_json::json;

pub(crate) fn emit_sum_make(
    dst: &ValueId,
    enum_name: &str,
    variant: &str,
    tag: u32,
    payload: Option<&ValueId>,
    payload_type: Option<&MirType>,
) -> serde_json::Value {
    let mut obj = json!({
        "op": "sum_make",
        "dst": dst.as_u32(),
        "enum": enum_name,
        "variant": variant,
        "tag": tag,
    });
    if let Some(payload) = payload {
        obj["payload"] = json!(payload.as_u32());
    }
    if let Some(payload_type) = payload_type.and_then(type_hint_to_json) {
        obj["payload_type"] = payload_type;
    }
    obj
}

pub(crate) fn emit_sum_tag(dst: &ValueId, value: &ValueId, enum_name: &str) -> serde_json::Value {
    json!({
        "op": "sum_tag",
        "dst": dst.as_u32(),
        "value": value.as_u32(),
        "enum": enum_name,
    })
}

pub(crate) fn emit_sum_project(
    dst: &ValueId,
    value: &ValueId,
    enum_name: &str,
    variant: &str,
    tag: u32,
    payload_type: Option<&MirType>,
) -> serde_json::Value {
    let mut obj = json!({
        "op": "sum_project",
        "dst": dst.as_u32(),
        "value": value.as_u32(),
        "enum": enum_name,
        "variant": variant,
        "tag": tag,
    });
    if let Some(payload_type) = payload_type.and_then(type_hint_to_json) {
        obj["payload_type"] = payload_type;
    }
    obj
}

fn type_hint_to_json(ty: &MirType) -> Option<serde_json::Value> {
    match ty {
        MirType::Integer => Some(json!("Integer")),
        MirType::Float => Some(json!("Float")),
        MirType::Bool => Some(json!("Bool")),
        MirType::String => Some(json!("String")),
        MirType::Void => Some(json!("Void")),
        MirType::Box(name) => Some(json!(name)),
        MirType::Array(_) | MirType::Future(_) | MirType::WeakRef | MirType::Unknown => None,
    }
}
