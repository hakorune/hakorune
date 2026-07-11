use crate::mir::{
    ArrayElementWriteKind, ArrayWriteProducerKind, ArrayWriteSiteId, MirInstruction, ValueId,
};
use serde_json::Value;

use super::helpers::require_u64;

pub(super) fn parse_array_element_write(inst: &Value) -> Result<MirInstruction, String> {
    let site_id = require_u64(inst, "site_id", "array_element_write site_id")? as u32;
    let dst = inst
        .get("dst")
        .and_then(Value::as_u64)
        .map(|value| ValueId::new(value as u32));
    let kind = match inst.get("kind").and_then(Value::as_str) {
        Some("literal_append") => ArrayElementWriteKind::LiteralAppend,
        Some("push") => ArrayElementWriteKind::Push,
        Some("set") => ArrayElementWriteKind::Set,
        Some("insert") => ArrayElementWriteKind::Insert,
        other => return Err(unclassified("kind", other)),
    };
    let producer = match inst.get("producer").and_then(Value::as_str) {
        Some("literal") => ArrayWriteProducerKind::Literal,
        Some("method_call") => ArrayWriteProducerKind::MethodCall,
        Some("index_assignment") => ArrayWriteProducerKind::IndexAssignment,
        Some("compound_index_assignment") => ArrayWriteProducerKind::CompoundIndexAssignment,
        Some("legacy_canonicalized") => ArrayWriteProducerKind::LegacyCanonicalized,
        other => return Err(unclassified("producer", other)),
    };
    let receiver = value_id(inst, "receiver")?;
    let index = inst
        .get("index")
        .and_then(Value::as_u64)
        .map(|value| ValueId::new(value as u32));
    let value = value_id(inst, "value")?;
    crate::mir::array_element_write::instruction(
        ArrayWriteSiteId::new(site_id),
        dst,
        kind,
        producer,
        receiver,
        index,
        value,
    )
}

fn value_id(inst: &Value, field: &str) -> Result<ValueId, String> {
    Ok(ValueId::new(
        require_u64(inst, field, &format!("array_element_write {field}"))? as u32,
    ))
}

fn unclassified(field: &str, value: Option<&str>) -> String {
    format!("[mir/array_write/unclassified_surface] {field}={value:?}")
}
