use serde_json::json;

use crate::mir::{ValueId, WeakRefOp};

pub(crate) fn emit_weak_ref(dst: &ValueId, op: &WeakRefOp, value: &ValueId) -> serde_json::Value {
    let op_name = match op {
        WeakRefOp::New => "weak_new",
        WeakRefOp::Load => "weak_load",
    };
    let value_field = match op {
        WeakRefOp::New => "box_val",
        WeakRefOp::Load => "weak_ref",
    };
    json!({"op": op_name, "dst": dst.as_u32(), value_field: value.as_u32()})
}

pub(crate) fn emit_keep_alive(values: &[ValueId]) -> serde_json::Value {
    let values_json: Vec<_> = values.iter().map(|v| json!(v.as_u32())).collect();
    json!({"op":"keepalive","values":values_json})
}

pub(crate) fn emit_release_strong(values: &[ValueId]) -> serde_json::Value {
    let values_json: Vec<_> = values.iter().map(|v| json!(v.as_u32())).collect();
    json!({"op":"release_strong","values":values_json})
}
