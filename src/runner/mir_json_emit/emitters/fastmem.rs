use crate::mir::instruction::{FastMemRegionId, MemOpAccess, MemOpKind};
use crate::mir::{EffectMask, ValueId};
use serde_json::{json, Value};

pub(crate) fn emit_memop(
    region: &FastMemRegionId,
    kind: &MemOpKind,
    dst: &Option<ValueId>,
    operands: &[ValueId],
    access: &Option<MemOpAccess>,
    effects: &EffectMask,
) -> Value {
    let mut value = json!({
        "op": "memop",
        "region": region.0,
        "kind": kind.as_json_name(),
        "dst": dst.map(|value| value.as_u32()),
        "operands": operands.iter().map(|value| value.as_u32()).collect::<Vec<_>>(),
        "effects": format!("{:?}", effects),
    });
    if let Some(access) = access {
        if let Value::Object(map) = &mut value {
            if let Some(layout_id) = &access.layout_id {
                map.insert("layout_id".to_string(), json!(layout_id));
            }
            if let Some(field_id) = &access.field_id {
                map.insert("field_id".to_string(), json!(field_id));
            }
            if let Some(table_id) = &access.table_id {
                map.insert("table_id".to_string(), json!(table_id));
            }
        }
    }
    value
}
